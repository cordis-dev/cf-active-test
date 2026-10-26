fn count<T>(items: &[i32]) -> usize {
    items.len()
}

pub fn total(items: &[i32]) -> usize {
    count::<u8>(items)
}
