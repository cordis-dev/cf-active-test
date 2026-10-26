pub unsafe fn address(ptr: *const usize) -> usize {
    unsafe { std::mem::transmute::<*const usize, usize>(ptr) }
}
