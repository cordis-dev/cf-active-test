#[derive(Copy)]
pub struct Id(pub u32);

impl Clone for Id {
    fn clone(&self) -> Self {
        Id(self.0)
    }
}
