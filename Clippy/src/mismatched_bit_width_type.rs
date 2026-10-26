pub fn bit_width(value: u64) -> u32 {
    u32::BITS - value.leading_zeros()
}
