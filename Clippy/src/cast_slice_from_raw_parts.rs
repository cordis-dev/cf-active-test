pub unsafe fn as_slice_ptr(ptr: *const u8, len: usize) -> *const [u8] {
    unsafe { std::slice::from_raw_parts(ptr, len) as *const [u8] }
}
