use std::fs::File;
use std::io::Read;

pub fn size(file: File) -> usize {
    file.bytes().count()
}
