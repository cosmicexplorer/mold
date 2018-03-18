mod bfd;

use std::env;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;
use std::sync::{Once, ONCE_INIT};

fn main() {
  unsafe {
    bfd::init();
  }
  let filename = env::args().nth(1).unwrap();
  let obj_file = File::open(&filename).unwrap();
  println!("filename: {}", &filename);
}
