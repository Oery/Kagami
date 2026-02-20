use std::io;

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub fn temp_convert(mut value: i32) -> io::Result<Vec<u8>> {
    let mut val = Vec::new();

    loop {
        let mut byte = (value & SEGMENT_BITS) as u8;
        value >>= 7;

        if value == 0 && (byte & CONTINUE_BIT) == 0 {
            val.push(byte);
            break;
        }

        byte |= CONTINUE_BIT;
        val.push(byte);

        if value == 0 {
            break;
        }
    }

    Ok(val)
}
