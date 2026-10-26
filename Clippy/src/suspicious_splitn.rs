pub fn first_word(s: &str) -> Option<&str> {
    s.splitn(1, ' ').next()
}
