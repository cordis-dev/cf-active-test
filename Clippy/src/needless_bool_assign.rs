pub fn update(ready: &mut bool, count: u32) {
    if count > 0 {
        *ready = true;
    } else {
        *ready = false;
    }
}
