use std::ptr;

pub fn copy_values(src: &[u32], dst: &mut [u32], count: usize) {
    unsafe {
        ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), count * size_of::<u32>());
    }
}
