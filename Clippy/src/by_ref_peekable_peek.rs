pub fn peek_first(numbers: &mut std::vec::IntoIter<u32>) -> Option<u32> {
    numbers.by_ref().peekable().peek().copied()
}
