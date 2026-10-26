pub fn count(n: u32, step: u32) -> u32 {
    if n == 0 { 0 } else { 1 + count(n - 1, step) }
}
