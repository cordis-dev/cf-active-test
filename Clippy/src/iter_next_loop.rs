#[allow(for_loops_over_fallibles)]
pub fn sum(v: &[i32]) -> i32 {
    let mut total = 0;
    for x in v.iter().next() {
        total += x;
    }
    total
}
