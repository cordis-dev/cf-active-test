use std::io::{self, Write};

pub fn send(out: &mut impl Write, data: &[u8]) -> io::Result<()> {
    out.write(data)?;
    Ok(())
}
