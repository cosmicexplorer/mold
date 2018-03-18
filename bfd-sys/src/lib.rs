#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("../include/bfd-bindings.rs");

use std::fmt;
use std::os::raw::c_char;

impl fmt::Debug for bfd {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("bfd")
      .field("filename", &self.filename)
      .finish()
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
