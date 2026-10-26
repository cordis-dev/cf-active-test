pub fn count(v: &[i32]) -> usize {
    v.iter().map(|x| x + 1).count()
}
