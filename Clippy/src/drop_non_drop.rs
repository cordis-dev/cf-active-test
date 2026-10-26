pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn discard(point: Point) {
    std::mem::drop(point);
}
