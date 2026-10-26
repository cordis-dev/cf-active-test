pub fn same(values: Vec<i32>) -> Vec<i32> {
    unsafe { std::mem::transmute::<Vec<i32>, Vec<i32>>(values) }
}
