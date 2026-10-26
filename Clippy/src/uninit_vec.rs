pub fn buffer(len: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(len);
    unsafe { v.set_len(len) };
    v
}
