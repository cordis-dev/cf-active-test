use std::fs::{File, OpenOptions};

pub fn open_log() -> std::io::Result<File> {
    OpenOptions::new().write(true).append(true).open("app.log")
}
