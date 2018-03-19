extern crate bfd_sys;
extern crate libc;

use self::bfd_sys::{bfd, bfd_hash_table, bfd_link_info, bfd_target};

use std::ffi::CStr;
use std::fmt;
use std::io;
use std::mem::size_of;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ptr;
use std::slice;

#[derive(Debug)]
pub enum BFDError {
  // TODO: add error string from bfd's perror-like c api
  FormatCheckError,
  NullPtrError,
  IoError(io::Error),
  LinkError,
}

impl From<io::Error> for BFDError {
  fn from(error: io::Error) -> Self {
    BFDError::IoError(error)
  }
}

pub type Error = BFDError;

pub type Result<T> = ::std::result::Result<T, Error>;

fn str_to_c_char_vec(s: &str) -> Vec<c_char> {
  s.as_bytes().iter().map(|b| *b as c_char).collect()
}

fn path_to_c_char_vec(path: &Path) -> Vec<c_char> {
  str_to_c_char_vec(path.to_str().unwrap())
}

fn ptr_opt<'a, T>(ptr: *const T) -> Option<&'a T> {
  unsafe {
    if ptr.is_null() {
      None
    } else {
      Some(&*ptr)
    }
  }
}

fn ptr_mut_opt<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
  unsafe {
    if ptr.is_null() {
      None
    } else {
      Some(&mut *ptr)
    }
  }
}

pub fn openr<'a>(path: &'a Path, target: &'a str) -> Result<&'a mut bfd> {
  let in_obj_path_c_char_vec: Vec<c_char> = path_to_c_char_vec(path);
  let in_obj_path_c_str: *const c_char =
    in_obj_path_c_char_vec.as_slice().as_ptr();
  let target_c_str: *const c_char =
    str_to_c_char_vec(target).as_slice().as_ptr();
  let bfd_h: &mut bfd;
  unsafe {
    bfd_h = &mut *bfd_sys::bfd_openr(in_obj_path_c_str, target_c_str);
  }
  if let Some(x) = ptr_mut_opt(bfd_h) {
    Ok(x)
  } else {
    Err(BFDError::NullPtrError)
  }
}

pub fn openw<'a>(path: &'a Path, target: &'a str) -> Result<&'a mut bfd> {
  let out_obj_path_c_char_vec: Vec<c_char> = path_to_c_char_vec(path);
  let out_obj_path_c_str: *const c_char =
    out_obj_path_c_char_vec.as_slice().as_ptr();
  let target_c_str: *const c_char =
    str_to_c_char_vec(target).as_slice().as_ptr();
  let bfd_h: &mut bfd;
  unsafe {
    bfd_h = &mut *bfd_sys::bfd_openw(out_obj_path_c_str, target_c_str);
  }
  if let Some(x) = ptr_mut_opt(bfd_h) {
    Ok(x)
  } else {
    Err(BFDError::NullPtrError)
  }
}

#[derive(Debug)]
pub struct BFDHandle<'a> {
  pub bfd: &'a mut bfd,
}

impl<'a> Drop for BFDHandle<'a> {
  fn drop(&mut self) {
    unsafe {
      bfd_sys::bfd_close(self.bfd);
    }
  }
}

static only_target: &str = "mach-o-x86-64";

impl<'a> BFDHandle<'a> {
  pub fn for_input_file(path: &'a Path) -> Result<BFDHandle<'a>> {
    let abfd = openr(path, only_target)?;
    let format_check_result: bfd_sys::bfd_boolean;
    // FIXME: check errors here!
    unsafe {
      format_check_result =
        bfd_sys::bfd_check_format(abfd, bfd_sys::bfd_format_bfd_object);
    }
    if format_check_result != 0 {
      Ok(BFDHandle { bfd: abfd })
    } else {
      Err(BFDError::FormatCheckError)
    }
  }

  pub fn for_output_obj_file(path: &'a Path) -> Result<BFDHandle<'a>> {
    let abfd = openw(path, only_target)?;
    let format_set_result: bfd_sys::bfd_boolean;
    // FIXME: check errors here!
    unsafe {
      format_set_result =
        bfd_sys::bfd_set_format(abfd, bfd_sys::bfd_format_bfd_object);
    }
    if format_set_result != 0 {
      Ok(BFDHandle { bfd: abfd })
    } else {
      Err(BFDError::FormatCheckError)
    }
  }

  unsafe fn link_hash_table_create(
    &mut self,
  ) -> *mut bfd_sys::bfd_link_hash_table {
    let fun = (*self.bfd.xvec)._bfd_link_hash_table_create.unwrap();
    fun(self.bfd)
  }

  unsafe fn final_link(&mut self, link_info: &mut bfd_link_info) -> bool {
    eprintln!("final_link: 1");
    let fun = (*self.bfd.xvec)._bfd_final_link.unwrap();
    eprintln!("final_link: 2");
    fun(self.bfd, link_info) != 0
  }

  fn target(&self) -> Option<&bfd_target> {
    ptr_opt(self.bfd.xvec)
  }
}

pub struct LinkProcess {
  link_info: bfd_link_info,
}

fn unix_link_info() -> bfd_link_info {
  bfd_link_info {
    path_separator: ':' as c_char,
    ..Default::default()
  }
}

impl LinkProcess {
  pub fn new() -> Self {
    LinkProcess {
      link_info: unix_link_info(),
    }
  }

  pub fn add_symbols<'b>(&mut self, other: BFDHandle<'b>) -> bool {
    let add_sym_fun = other.target().unwrap()._bfd_link_add_symbols.unwrap();
    unsafe {
      // TODO: is this conversion checked? should we check whether the return
      // value (a bfd_boolean type) has any higher bits set than what bool
      // allows? how wide is bool?
      add_sym_fun(other.bfd, &mut self.link_info) != 0
    }
  }
}

// Produce a mach-o main executable that has file type MH_EXECUTE.
pub fn make_executable<'a>(
  object_path: &Path,
  // clang_rt: &Path,
  out_path: &'a Path,
) -> Result<&'a Path> {
  eprintln!("1");
  let mut tbl = bfd_hash_table {
    ..Default::default()
  };
  unsafe {
    bfd_sys::bfd_hash_table_init(
      &mut tbl,
      Some(bfd_sys::bfd_hash_newfunc),
      size_of::<bfd_sys::bfd_hash_entry>() as u32,
    );
  }
  eprintln!("2");

  // Create the output object file.
  let mut obj_out = BFDHandle::for_output_obj_file(out_path)?;

  eprintln!("3");

  let mut link_info: bfd_link_info;
  unsafe {
    link_info = bfd_link_info {
      output_bfd: obj_out.bfd,
      hash: obj_out.link_hash_table_create(),
      ..Default::default()
    };
    link_info.input_bfds_tail = &mut link_info.input_bfds;
  };
  eprintln!("4");
  // Read in the input object file.
  let obj_in = BFDHandle::for_input_file(object_path)?;
  eprintln!("5");
  // Add symbols from the input object file.
  unsafe {
    eprintln!("5.1");
    (*link_info.input_bfds_tail) = obj_in.bfd;
    eprintln!("5.2");
    link_info.input_bfds_tail = &mut obj_in.bfd.link.next;
    eprintln!("5.3");
    if !obj_out.final_link(&mut link_info) {
      return Err(BFDError::LinkError);
    }
  }
  eprintln!("6");

  // Add symbols from the -lSystem library.
  // Add symbols from the clang runtime archive libclang_rt.osx.a.
  // Produce a binary for the x86_64 architecture.
  Ok(out_path)
}

pub fn get_target(target_name: &str) -> Result<String> {
  let target_c_str: *const c_char =
    str_to_c_char_vec(target_name).as_slice().as_ptr();
  let tgt: *const bfd_target;
  unsafe {
    tgt = bfd_sys::bfd_find_target(target_c_str, ptr::null_mut());
  }
  if let Some(x) = ptr_opt(tgt) {
    let c_str: &CStr;
    unsafe {
      c_str = CStr::from_ptr(x.name);
    }
    Ok(c_str.to_str().unwrap().to_owned())
  } else {
    Err(BFDError::NullPtrError)
  }
}

unsafe fn null_term_array_of_c_string_to_vec(
  arr: *const *const c_char,
) -> Vec<String> {
  let mut str_vec: Vec<String> = Vec::new();

  if arr.is_null() {
    return str_vec;
  }

  for i in 0.. {
    let cur_c_str: &*const c_char = &*arr.offset(i);
    if cur_c_str.is_null() {
      break;
    } else {
      let cur_str: &str = CStr::from_ptr(*cur_c_str).to_str().unwrap();
      str_vec.push(String::from(cur_str));
    }
  }

  str_vec
}

unsafe fn free<T>(arg: *mut T) {
  libc::free(arg as *mut libc::c_void);
}

pub fn get_all_targets() -> Vec<String> {
  let tgt_inits: Vec<String>;
  unsafe {
    let target_listing: *mut *const c_char = bfd_sys::bfd_target_list();
    tgt_inits = null_term_array_of_c_string_to_vec(target_listing);
    free(target_listing);
  }
  tgt_inits
}
