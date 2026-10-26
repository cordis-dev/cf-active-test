pub fn buffer(len: usize) -> Vec<u8> {
    let mut buf = Vec::with_capacity(len);
    buf.resize(len, 0);
    buf
}
