pub fn same(x: Option<i32>) -> Option<i32> {
    match x {
        Some(v) => Some(v),
        None => None,
    }
}
