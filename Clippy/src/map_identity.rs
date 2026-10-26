pub fn doubled(items: &[i32]) -> Vec<i32> {
    items.iter().map(|x| x * 2).map(|x| x).collect()
}
