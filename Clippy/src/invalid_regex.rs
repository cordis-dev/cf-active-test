use regex::Regex;

pub fn pattern() -> Regex {
    Regex::new("(abc").unwrap()
}
