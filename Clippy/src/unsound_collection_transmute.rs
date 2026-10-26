pub fn to_bytes(v: Vec<u32>) -> Vec<u8> {
    unsafe { std::mem::transmute(v) }
}
