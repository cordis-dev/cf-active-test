pub fn next_id() -> u32 {
    7
}

pub fn empty_ids() -> usize {
    let ids: Vec<u32> = vec![next_id(); 0];
    ids.len()
}
