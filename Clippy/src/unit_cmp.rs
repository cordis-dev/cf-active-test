pub fn log(msg: &str) {
    println!("{msg}");
}

pub fn same() -> bool {
    log("a") == log("b")
}
