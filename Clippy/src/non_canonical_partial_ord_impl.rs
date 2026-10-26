use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub struct Version(pub u32);

impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
