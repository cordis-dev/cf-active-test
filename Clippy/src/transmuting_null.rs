#[allow(clippy::transmute_ptr_to_ref)]
pub fn null_ref() -> &'static u64 {
    unsafe { std::mem::transmute(std::ptr::null::<u64>()) }
}
