pub fn log(message: &str) {
    println!("{message}");
}

pub fn run() -> Option<()> {
    Some(log("done"))
}
