pub struct Pair {
    pub a: u32,
    pub b: u32,
}

pub fn take(p: &mut Pair) -> Pair {
    unsafe { std::mem::replace(p, std::mem::zeroed()) }
}
