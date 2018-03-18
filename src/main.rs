mod bfd;

use std::env;
use std::fs::File;

fn main() {
  unsafe {
    // Should probably be using std::sync::Once, but I can't get it to compile.
    bfd::init();
  }
  println!("{:?}", bfd::get_all_targets());
  if let Ok(result) = bfd::get_target("mach-o-x86-64") {
    println!("result: {}", result);
  } else {
    println!("?!");
  }
  let filename = env::args().nth(1).unwrap();
  let obj_file = File::open(&filename).unwrap();
  println!("filename: {}", &filename);
}
