pub fn advance(items: &[()], n: usize) -> *const () {
    unsafe { items.as_ptr().add(n) }
}
