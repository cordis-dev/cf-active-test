pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn width(&self) -> u32 {
        self.height
    }
}
