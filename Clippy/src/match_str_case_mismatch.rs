pub fn is_yes(s: &str) -> bool {
    match s.to_lowercase().as_str() {
        "Yes" => true,
        "y" => true,
        _ => false,
    }
}
