pub fn name(x: &Option<String>) -> Option<&String> {
    x.as_ref().take()
}
