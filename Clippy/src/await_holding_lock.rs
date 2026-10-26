use std::sync::Mutex;

async fn save() {}

pub async fn flush(count: &Mutex<u32>) -> u32 {
    let guard = count.lock().unwrap();
    save().await;
    *guard
}
