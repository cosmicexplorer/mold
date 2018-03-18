mod bfd;
mod lib;

use std::env;
use std::fs::File;

fn main() {
  unsafe {
    // Should probably be using std::sync::Once, but I can't get it to compile.
    bfd::init();
  }
  let filename = env::args().nth(1).unwrap();
  let obj_file = File::open(&filename).unwrap();
  println!("filename: {}", &filename);
}
