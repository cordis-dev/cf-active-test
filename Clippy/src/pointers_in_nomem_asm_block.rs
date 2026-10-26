use std::arch::asm;

pub fn clear(p: *mut u8) {
    unsafe {
        asm!("mov byte ptr [{}], 0", in(reg) p, options(nomem, nostack));
    }
}
