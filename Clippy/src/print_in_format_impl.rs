use std::fmt;

pub struct Point(pub i32, pub i32);

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!("formatting point");
        write!(f, "({}, {})", self.0, self.1)
    }
}
