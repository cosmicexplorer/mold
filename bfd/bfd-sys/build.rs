extern crate autotools_dependency;
extern crate bindgen;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time;

static BINUTILS_URL: &'static str =
  "https://ftpmirror.gnu.org/binutils/binutils-2.30.tar.gz";

// Make a literal array of strings into a Vec -- this might be nonidiomatic?
fn to_string_vec(args: &[&str]) -> Vec<String> {
  args.iter().map(|s| s.to_string()).collect()
}

fn main() {
  let bfd_binutils_dirname = Path::new("bfd-binutils");
  let binutils_src_dirname = Path::new("binutils-2.30");

  let bfd_lib_loc: PathBuf = [bfd_binutils_dirname, Path::new("lib/libbfd.a")]
    .iter()
    .collect();
  let bfd_binutils_dir = if fs::metadata(bfd_lib_loc).unwrap().is_file() {
    fs::canonicalize(bfd_binutils_dirname).unwrap()
  } else {
    fs::DirBuilder::new()
      .recursive(true)
      .create(bfd_binutils_dirname)
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

    let install_dir = autotools_dependency::fetch_build_autotools_dep(
      BINUTILS_URL,
      bfd_binutils_dirname,
      binutils_src_dirname,
      config_args,
      config_env,
      time::Duration::new(300, 0),
      1,
    ).unwrap();

    eprintln!("install_dir: {:?}", install_dir);

    install_dir
  };

  let bfd_inc_dir: PathBuf = [bfd_binutils_dir.as_path(), Path::new("include")]
    .iter()
    .collect();
  // bfd.h requires PACKAGE or PACKAGE_VERSION to be defined, or it errors out
  // during preprocessing. This seems like a failure
  let bfd_bindings = bindgen::builder()
    .clang_arg("-DPACKAGE")
    .clang_arg(format!("-I{}", bfd_inc_dir.to_str().unwrap()))
    .header("include/bfd-headers.h")
    .generate()
    .unwrap();
  bfd_bindings
    .write_to_file("include/bfd-bindings.rs")
    .unwrap();

  let bfd_lib_dir: PathBuf = [bfd_binutils_dir.as_path(), Path::new("lib")]
    .iter()
    .collect();
  println!(
    "cargo:rustc-link-search=native={}",
    bfd_lib_dir.to_str().unwrap()
  );

  println!("cargo:rustc-link-lib=static=bfd");
}
