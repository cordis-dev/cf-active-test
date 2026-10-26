use std::cmp::Ordering;

#[derive(PartialEq, Eq, Ord)]
pub struct Version(pub u32);

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
