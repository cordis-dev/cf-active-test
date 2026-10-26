pub fn is_admin(x: Option<&str>) -> bool {
    x.as_deref() == Some("admin")
}
