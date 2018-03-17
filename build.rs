extern crate bindgen;

fn main() {
  let bindings = bindgen::Builder::default()
    .header("include/mach-o.h")
    .whitelist_var("MH_MAGIC_64")
    .generate()
    .unwrap();
  bindings
    .write_to_file("include/bindings.rs")
    .unwrap();
}
