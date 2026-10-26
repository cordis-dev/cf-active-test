use std::sync::{Arc, Mutex};

pub fn slots() -> Vec<Arc<Mutex<u32>>> {
    vec![Arc::new(Mutex::new(0)); 8]
}
