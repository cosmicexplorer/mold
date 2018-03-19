extern crate bfd_sys;

mod bfd;

use std::env;
use std::ffi::CString;
use std::fs::File;
use std::path::Path;
use std::os::raw::c_char;

fn main() {
  // eprintln!("{:?}", bfd::get_all_targets());
  // let argv: Vec<String> = env::args().collect();
  // let in_path = Path::new(&argv[1]);
  // eprintln!("in_path: {:?}", in_path);
  // let out_path = Path::new(&argv[2]);
  // eprintln!("out_path: {:?}", out_path);
  // let res = bfd::make_executable(in_path, out_path).unwrap();
  // eprintln!("res: {:?}", res);

  let mut abfd: &mut bfd_sys::bfd;
  unsafe {
    abfd = &mut *bfd_sys::bfd_openw(
      CString::new("foo").unwrap().as_ptr(),
      CString::new("mach-o-x86-64").unwrap().as_ptr(),
    );
    bfd_sys::bfd_set_format(&mut *abfd, bfd_sys::bfd_format_bfd_object);
  }

  let mut new: &mut bfd_sys::asymbol;
  unsafe {
    new = &mut *bfd_sys::_bfd_generic_make_empty_symbol(&mut *abfd);
  }
  new.name = CString::new("dummy symbol").unwrap().as_ptr();
  unsafe {
    new.section = bfd_sys::bfd_make_section_old_way(
      &mut *abfd,
      CString::new(".text").unwrap().as_ptr(),
    );
  }
  new.flags = bfd_sys::BSF_GLOBAL;
  new.value = 0x12345;

  let new_ptr: *mut bfd_sys::asymbol = &mut *new;
  let nptr: *mut bfd_sys::asymbol = std::ptr::null_mut();
  let mut ptrs: &mut [*mut bfd_sys::asymbol] = &mut [new_ptr, nptr];

  unsafe {
    bfd_sys::bfd_set_symtab(&mut *abfd, ptrs.as_mut_ptr(), 1);
    bfd_sys::bfd_close(&mut *abfd);
  }
}
