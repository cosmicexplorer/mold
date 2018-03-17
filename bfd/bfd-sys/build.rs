extern crate autotools_dependency;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

static BINUTILS_URL: &'static str =
  "https://ftpmirror.gnu.org/binutils/binutils-2.30.tar.gz";

fn main() {
  let bfd_binutils_dirname = PathBuf::from("bfd-binutils");
  let binutils_src_dirname = PathBuf::from("binutils-2.30");

  fs::DirBuilder::new()
    .recursive(true)
    .create(bfd_binutils_dirname.clone())
    .unwrap();
  let bfd_binutils_dir = autotools_dependency::fetch_build_autotools_dep(
    BINUTILS_URL,
    bfd_binutils_dirname.clone(),
    binutils_src_dirname,
    vec![],
    HashMap::new(),
    1,
  ).unwrap();

  eprintln!("bfd_binutils_dir: {:?}", bfd_binutils_dir);

  let bfd_inc_dir: PathBuf =
    [bfd_binutils_dir.clone(), PathBuf::from("include")]
      .iter()
      .collect();
  println!("cargo:include={:?}", bfd_inc_dir);

  let bfd_lib_dir: PathBuf = [bfd_binutils_dir.clone(), PathBuf::from("lib")]
    .iter()
    .collect();
  println!("cargo:rustc-link-search=native={:?}", bfd_lib_dir);

  println!("cargo:rustc-link-lib=static=bfd");
}
