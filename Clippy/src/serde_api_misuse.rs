use serde::de::{self, Visitor};
use std::fmt;

pub struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = String;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a name")
    }

    fn visit_string<E: de::Error>(self, v: String) -> Result<String, E> {
        Ok(v)
    }
}
