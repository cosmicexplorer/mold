mod system_headers;

extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::Seek;
use std::io::SeekFrom;

use byteorder::{NativeEndian, ReadBytesExt};

fn main() {
  let filename = env::args().nth(1).unwrap();
  let obj_file = File::open(filename).unwrap();
  dump_segments(obj_file);
}

fn read_magic(mut obj_file: File) -> u32 {
  obj_file.seek(SeekFrom::Start(0)).unwrap();
  obj_file.read_u32::<NativeEndian>().unwrap()
}

fn dump_segments(obj_file: File) {
  let magic: u32 = read_magic(obj_file);
  if magic != system_headers::MH_MAGIC_64 {
    panic!("Magic number is: {}. Mach-O files should have {}",
           magic,
           system_headers::MH_MAGIC_64);
  }
  println!("made it! magic number is: {}.", magic);
}
