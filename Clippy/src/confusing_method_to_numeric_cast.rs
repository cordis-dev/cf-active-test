#[allow(function_casts_as_integer)]
pub fn port_limit() -> usize {
    u16::max as usize
}
