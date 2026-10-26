pub fn label(name: &str) -> String {
    let suffix = "";
    if suffix.is_empty() { name.to_string() } else { format!("{name}{suffix}") }
}
