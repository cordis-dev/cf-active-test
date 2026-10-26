pub fn describe(x: i32) -> &'static str {
    match x {
        1 => "one",
        2 | _ => "other",
    }
}
