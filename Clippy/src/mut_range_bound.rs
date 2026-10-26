pub fn sum(mut end: usize) -> usize {
    let mut total = 0;
    for i in 0..end {
        total += i;
        end = 5;
    }
    total + end
}
