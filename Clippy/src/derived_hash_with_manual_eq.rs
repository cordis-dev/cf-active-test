#[derive(Hash)]
pub struct Id(pub u32);

impl PartialEq for Id {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
