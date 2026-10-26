use std::fmt;

pub struct Point;

impl Point {
    pub fn to_string(&self) -> String {
        "point".to_owned()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "p")
    }
}
