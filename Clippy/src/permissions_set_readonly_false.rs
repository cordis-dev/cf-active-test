use std::fs;
use std::io;
use std::path::Path;

pub fn unlock(path: &Path) -> io::Result<()> {
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_readonly(false);
    fs::set_permissions(path, perms)
}
