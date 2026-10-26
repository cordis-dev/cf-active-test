use std::sync::Mutex;

pub fn touch(m: &Mutex<i32>) {
    let _ = m.lock();
}
