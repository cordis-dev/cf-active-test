pub enum Shape {
    Circle(u32),
}

#[allow(function_casts_as_integer)]
pub fn shape_id() -> usize {
    Shape::Circle as usize
}
