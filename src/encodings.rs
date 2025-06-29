use crate::errors::{BND4Error, Result};
use std::io::Read;

/// Read a null-terminated UTF16 encoded string from a buffer.
/// Uses lossy decoding when converting to UTF8.
pub fn read_utf16(f: &mut impl Read) -> Result<String> {
    let mut u16s = Vec::new();
    loop {
        let mut buf = [0u8; 2];
        f.read_exact(&mut buf).map_err(BND4Error::IoError)?;
        let v = u16::from_le_bytes(buf);
        if v == 0 {
            break;
        }
        u16s.push(v);
    }
    Ok(String::from_utf16_lossy(&u16s))
}

/// Read a null-terminated Shift JIS encoded string from a buffer.
/// Uses lossy decoding when converting to UTF8.
pub fn read_shift_jis(f: &mut impl Read) -> Result<String> {
    use encoding_rs::SHIFT_JIS;
    let mut bytes = Vec::new();
    loop {
        let mut buf = [0u8; 1];
        f.read_exact(&mut buf).map_err(BND4Error::IoError)?;
        if buf[0] == 0 {
            break;
        }
        bytes.push(buf[0]);
    }
    let (cow, _, _) = SHIFT_JIS.decode(&bytes);
    Ok(cow.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_shift_jis() {
        let mut cursor = Cursor::new("hello\0");
        let result = read_shift_jis(&mut cursor).unwrap();
        assert_eq!(result, "hello");

        let mut cursor = Cursor::new([
            0x8d, 0xac, 0x93, 0xd7, 0x82, 0xcc, 0x95, 0x63, 0x8f, 0xb0, 0x00,
        ]);
        let result = read_shift_jis(&mut cursor).unwrap();
        assert_eq!(result, "混沌の苗床");

        let empty_data = [0x00];
        let mut cursor = Cursor::new(&empty_data);
        let result = read_shift_jis(&mut cursor).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_read_utf16() {
        let test_data = [0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x00, 0x00];
        let mut cursor = Cursor::new(&test_data);
        let result = read_utf16(&mut cursor).unwrap();
        assert_eq!(result, "ABC");
    }
}
