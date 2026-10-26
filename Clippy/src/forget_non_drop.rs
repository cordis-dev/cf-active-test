pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn leak(point: Point) {
    std::mem::forget(point);
}
