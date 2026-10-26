use std::cell::UnsafeCell;

pub struct Cache {
    data: UnsafeCell<Vec<u8>>,
}

impl Cache {
    pub fn data_mut(&self) -> &mut Vec<u8> {
        unsafe { &mut *self.data.get() }
    }
}
