use std::env;
use std::path::Path;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let in_path = Path::new(&argv[1]);
  eprintln!("in_path: {:?}", in_path);
  let out_path = Path::new(&argv[2]);
  eprintln!("out_path: {:?}", out_path);
}
