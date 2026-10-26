pub struct Id(pub u32);

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.0 == other.0
    }

    fn ne(&self, other: &Id) -> bool {
        self.0 != other.0
    }
}
