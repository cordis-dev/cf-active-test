use std::fs::{File, OpenOptions};
use std::io;

pub fn open_log(path: &str) -> io::Result<File> {
    OpenOptions::new().write(true).create(true).open(path)
}
