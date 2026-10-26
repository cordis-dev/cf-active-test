use std::path::Path;

pub fn is_rust(p: &Path) -> bool {
    p.ends_with(".rs")
}
