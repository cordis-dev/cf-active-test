pub fn will_overflow(a: u32, b: u32) -> bool {
    a + b < a
}
