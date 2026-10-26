use std::cmp::{max, min};

pub fn clamp(x: i32) -> i32 {
    min(0, max(100, x))
}
