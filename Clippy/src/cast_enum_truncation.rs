#[repr(u16)]
pub enum Status {
    Ok = 200,
    Large = 300,
}

pub fn large_byte() -> u8 {
    Status::Large as u8
}
