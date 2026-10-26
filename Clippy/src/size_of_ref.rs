pub fn byte_len(data: &[u8]) -> usize {
    std::mem::size_of_val(&data)
}
