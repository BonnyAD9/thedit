use std::io::Read;

use crate::err::Result;

pub fn read_to_eof(mut r: impl Read, out: &mut [u8]) -> Result<usize> {
    let mut red = 0;
    while red < out.len() {
        let now = r.read(&mut out[red..])?;
        if now == 0 {
            break;
        }
        red += now;
    }
    Ok(red)
}
