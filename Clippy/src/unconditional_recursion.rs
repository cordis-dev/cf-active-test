pub struct Meters(pub f64);

impl PartialEq for Meters {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
