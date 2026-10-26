use std::borrow::Borrow;

#[derive(Hash, PartialEq, Eq)]
pub struct Name(pub String);

impl Borrow<str> for Name {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl Borrow<[u8]> for Name {
    fn borrow(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
