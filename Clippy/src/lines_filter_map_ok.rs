use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_lines(file: File) -> Vec<String> {
    BufReader::new(file).lines().filter_map(Result::ok).collect()
}
