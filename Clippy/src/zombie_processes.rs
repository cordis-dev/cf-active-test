use std::process::Command;

pub fn launch() -> std::io::Result<()> {
    let mut child = Command::new("ls").spawn().unwrap();
    child.kill()
}
