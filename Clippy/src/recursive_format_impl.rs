use std::fmt;

pub struct Name(pub String);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}
