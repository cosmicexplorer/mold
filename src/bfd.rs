extern crate bfd_sys;

pub unsafe fn init() {
  bfd_sys::bfd_init();
}
