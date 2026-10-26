pub fn null_callback() -> fn() {
    unsafe { std::mem::transmute(std::ptr::null::<()>()) }
}
