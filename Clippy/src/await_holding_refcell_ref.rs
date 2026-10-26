use std::cell::RefCell;

async fn save() {}

pub async fn flush(count: &RefCell<u32>) -> u32 {
    let value = count.borrow_mut();
    save().await;
    *value
}
