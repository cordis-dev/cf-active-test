use std::process::Command;

pub fn format_disk() -> Command {
    let mut cmd = Command::new("mkfs");
    cmd.arg("-t ext2");
    cmd
}
