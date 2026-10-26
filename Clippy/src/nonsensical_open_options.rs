use std::fs::{File, OpenOptions};
use std::io;

pub fn open_log(path: &str) -> io::Result<File> {
    OpenOptions::new().read(true).truncate(true).open(path)
}
