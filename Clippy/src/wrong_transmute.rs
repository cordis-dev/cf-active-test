pub fn as_ptr(bits: f64) -> *const u8 {
    unsafe { std::mem::transmute(bits) }
}
