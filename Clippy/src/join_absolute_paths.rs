use std::path::{Path, PathBuf};

pub fn bin_dir() -> PathBuf {
    Path::new("/usr").join("/bin")
}
