pub fn reset(v: &mut Vec<i32>) {
    std::mem::swap(v, &mut Vec::new());
}
