pub unsafe fn swap_raw(a: *mut i32, b: *mut i32) {
    unsafe { std::mem::swap(&mut *a, &mut *b) }
}
