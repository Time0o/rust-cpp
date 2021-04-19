use std::ffi::c_void;
use std::os::raw::c_int;

#[link(name="integer")]
extern "C" {
  pub fn integer_new(value: c_int) -> *mut c_void;
  pub fn integer_delete(__self: *const c_void);
  pub fn integer_get(__self: *const c_void) -> c_int;
  pub fn integer_set(__self: *mut c_void, value: c_int);
}

pub struct Integer {
  ptr: *mut c_void
}

impl Integer {
  pub fn new(value: c_int) -> Integer {
    unsafe {
      Integer {
        ptr: integer_new(value)
      }
    }
  }

  pub fn get(&self) -> c_int {
    unsafe {
      integer_get(self.ptr)
    }
  }

  pub fn set(&mut self, value: c_int) {
    unsafe {
      integer_set(self.ptr, value)
    }
  }
}

impl Drop for Integer {
  fn drop(&mut self) {
    unsafe {
      integer_delete(self.ptr)
    }
  }
}
