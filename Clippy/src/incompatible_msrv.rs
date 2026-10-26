#[clippy::msrv = "1.70"]
pub fn dashes(count: usize) -> String {
    std::iter::repeat_n('-', count).collect()
}
