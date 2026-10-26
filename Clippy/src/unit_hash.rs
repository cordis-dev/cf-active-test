use std::hash::{Hash, Hasher};

pub fn hash_empty<H: Hasher>(state: &mut H) {
    ().hash(state);
}
