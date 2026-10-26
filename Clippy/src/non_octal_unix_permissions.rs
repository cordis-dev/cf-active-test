use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

pub fn default_mode() -> Permissions {
    Permissions::from_mode(644)
}
