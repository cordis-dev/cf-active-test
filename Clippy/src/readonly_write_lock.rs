use std::sync::RwLock;

pub fn current(value: &RwLock<u32>) -> u32 {
    let guard = value.write().unwrap();
    *guard
}
