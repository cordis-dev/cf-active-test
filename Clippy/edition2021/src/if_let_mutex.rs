use std::sync::Mutex;

pub fn bump(counter: &Mutex<Option<u32>>) {
    if let Some(value) = *counter.lock().unwrap() {
        println!("{value}");
    } else {
        *counter.lock().unwrap() = Some(1);
    }
}
