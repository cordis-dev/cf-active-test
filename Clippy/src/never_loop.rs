pub fn first(v: &[i32]) -> Option<i32> {
    for x in v {
        return Some(*x);
    }
    None
}
