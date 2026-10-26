pub fn header(bytes: &[u8; 4]) -> &[u8] {
    &bytes[1..8]
}
