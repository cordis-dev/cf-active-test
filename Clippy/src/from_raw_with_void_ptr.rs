use std::ffi::c_void;

pub unsafe fn take_ownership(ptr: *mut c_void) -> Box<c_void> {
    unsafe { Box::from_raw(ptr) }
}
