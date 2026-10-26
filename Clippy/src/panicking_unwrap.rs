pub fn value_or_zero(value: Option<i32>) -> i32 {
    if value.is_none() {
        value.unwrap()
    } else {
        0
    }
}
