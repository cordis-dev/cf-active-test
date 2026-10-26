pub fn nearly_equal(a: f64, b: f64) -> bool {
    (a - b) < f64::EPSILON
}
