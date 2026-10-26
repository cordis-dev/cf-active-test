pub fn read_value(ptr: *const i32) -> i32 {
    unsafe { *ptr }
}
