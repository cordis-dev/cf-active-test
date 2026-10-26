use std::mem::MaybeUninit;

pub fn make<T>() -> T {
    unsafe { MaybeUninit::uninit().assume_init() }
}
