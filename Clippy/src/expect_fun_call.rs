pub fn get(value: Option<u32>, key: &str) -> u32 {
    value.expect(&format!("missing {key}"))
}
