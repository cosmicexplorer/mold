use std::env;
use std::fs::File;

fn main() {
  let filename = env::args().collect().nth(1)?;
  let mut obj_file = File::open(filename)?;
  dump_segments(obj_file);
}

fn dump_segments(obj_file: File) {}
