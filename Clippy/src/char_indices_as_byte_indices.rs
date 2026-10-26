pub fn before_dash(s: &str) -> Option<&str> {
    for (i, c) in s.chars().enumerate() {
        if c == '-' {
            return Some(&s[..i]);
        }
    }
    None
}
