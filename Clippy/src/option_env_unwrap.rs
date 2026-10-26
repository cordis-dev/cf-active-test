pub fn api_key() -> &'static str {
    option_env!("API_KEY").unwrap()
}
