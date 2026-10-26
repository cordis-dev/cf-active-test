use std::io;

pub fn read_number() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.parse().unwrap()
}
