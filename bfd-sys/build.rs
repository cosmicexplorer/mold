extern crate autotools_dependency;
extern crate bindgen;
extern crate tempdir;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

use autotools_dependency::FetchedAutotoolsDep;
use tempdir::TempDir;

static BINUTILS_URL: &'static str =
  "https://ftpmirror.gnu.org/binutils/binutils-2.30.tar.gz";

// Make a literal array of strings into a Vec -- this might be nonidiomatic?
fn to_string_vec(args: &[&str]) -> Vec<String> {
  args.iter().map(|s| s.to_string()).collect()
}

fn build_binutils(src_dir: &Path, build_dir: &Path) {
  fs::DirBuilder::new()
    .recursive(true)
    .create(build_dir)
    .unwrap();

  let mut config_env: HashMap<String, String> = HashMap::new();
  config_env.insert("PATH".to_string(), "/bin:/usr/bin".to_string());

  let config_args = to_string_vec(&[
    "CFLAGS=-arch x86_64",
    "LDFLAGS=-arch x86_64",
    "CC=clang",
    "CXX=clang++",
    "--disable-werror",
    "--enable-64-bit-bfd",
    "--build=x86_64-apple-darwin",
    "--host=x86_64-apple-darwin",
    "--target=x86_64-apple-darwin",
  ]);

  // let tmp_build_dir = TempDir::new("autotools-build").unwrap();
  // let build_dir_abs: PathBuf = fs::canonicalize(tmp_build_dir.path()).unwrap();

  let FetchedAutotoolsDep {
    build_dir: complete_build_dir,
  } = autotools_dependency::build_local_autotools_dep(
    // BINUTILS_URL,
    src_dir,
    build_dir,
    config_args,
    config_env,
    // time::Duration::new(300, 0),
    4,
  ).unwrap();
}

fn file_exists(path: &Path) -> bool {
  if let Ok(metadata) = fs::metadata(path) {
    metadata.is_file()
  } else {
    false
  }
}

fn add_static_build_lib(build_dir: &Path, subdir: &str, lib_name: &str) {
  let lib_dir: PathBuf = [build_dir, Path::new(subdir)].iter().collect();
  println!(
    "cargo:rustc-link-search=native={}",
    lib_dir.to_str().unwrap()
  );
  println!("cargo:rustc-link-lib=static={}", lib_name);
}

fn main() {
  // Only re-run if our merged header file or anything in src has changed.
  let bfd_merged_headers = Path::new("include/bfd-headers.h");
  println!(
    "cargo:rerun-if-changed={}",
    bfd_merged_headers.to_str().unwrap()
  );

  let src_entries = fs::read_dir(Path::new("src")).unwrap();
  for entry in src_entries {
    println!(
      "cargo:rerun-if-changed={}",
      entry.unwrap().path().to_str().unwrap()
    );
  }

  let src_dir = Path::new("/Users/dmcclanahan/tools/binutils-2.30");
  let build_dir = Path::new("/Users/dmcclanahan/tools/binutils-osx-build");

  let bfd_archive_cached: PathBuf =
    [build_dir, Path::new("bfd/libbfd.a")].iter().collect();
  if !file_exists(bfd_archive_cached.as_path()) {
    build_binutils(src_dir, build_dir);
  }

  let bfd_inc_dir: PathBuf = [build_dir, Path::new("bfd")].iter().collect();
  // bfd.h requires PACKAGE or PACKAGE_VERSION to be defined, or it errors out
  // during preprocessing. This seems like a failure
  let bfd_bindings = bindgen::builder()
    .clang_arg("-DPACKAGE")
    .clang_arg(format!("-I{}", bfd_inc_dir.to_str().unwrap()))
    .header("include/bfd-headers.h")
    .raw_line("#[cfg_attr(rustfmt, rustfmt_skip)]")
    .derive_default(true)
    .generate()
    .unwrap();
  bfd_bindings
    .write_to_file("include/bfd-bindings.rs")
    .unwrap();

  add_static_build_lib(build_dir, "bfd", "bfd");
  add_static_build_lib(build_dir, "zlib", "z");
  add_static_build_lib(build_dir, "intl", "intl");
  add_static_build_lib(build_dir, "libiberty", "iberty");
  add_static_build_lib(build_dir, "opcodes", "opcodes");

  println!("cargo:rustc-link-lib=dylib={}", "iconv");
}
