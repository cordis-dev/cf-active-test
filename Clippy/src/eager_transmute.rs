#[repr(u8)]
pub enum Level {
    Low = 0,
    High = 1,
}

pub fn to_level(x: u8) -> Option<Level> {
    (x < 2).then_some(unsafe { std::mem::transmute::<u8, Level>(x) })
}
