use crate::Result;
use std::io;

const STOP_BYTE: u8 = 0x80;
const SIGNIFICANT_BYTE: u8 = !STOP_BYTE;
const NEGATIVE_SIGN_MASK: u8 = 0x40;

pub trait Codec: Sized {
    fn decode_from_reader<R: io::Read>(&mut self, input: &mut R) -> Result<()>;
    fn encode_to_writer<W: io::Write>(&self, output: &mut W) -> io::Result<()>;
}

impl Codec for u32 {
    fn encode_to_writer<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        let mut bytes = [0u8; 5];
        let i;
        if *self >= 0x1000_0000 {
            bytes[0] = (self >> 28) as u8;
            bytes[1] = (self >> 21) as u8 & SIGNIFICANT_BYTE;
            bytes[2] = (self >> 14) as u8 & SIGNIFICANT_BYTE;
            bytes[3] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[4] = *self as u8 | STOP_BYTE;
            i = 5;
        } else if *self >= 0x20_0000 {
            bytes[0] = (self >> 21) as u8 & SIGNIFICANT_BYTE;
            bytes[1] = (self >> 14) as u8 & SIGNIFICANT_BYTE;
            bytes[2] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[3] = *self as u8 | STOP_BYTE;
            i = 4;
        } else if *self >= 0x4000 {
            bytes[0] = (self >> 14) as u8 & SIGNIFICANT_BYTE;
            bytes[1] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[2] = *self as u8 | STOP_BYTE;
            i = 3;
        } else if *self >= 0x80 {
            bytes[0] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[1] = *self as u8 | STOP_BYTE;
            i = 2;
        } else {
            bytes[0] = *self as u8 | STOP_BYTE;
            i = 1;
        }
        output.write_all(&bytes[..i])
    }

    fn decode_from_reader<R: io::Read>(&mut self, input: &mut R) -> Result<()> {
        *self = 0;
        for byte in decode_stop_bit_entity(input)? {
            *self = (*self << 7) | u32::from(byte);
        }
        Ok(())
    }
}

impl Codec for i32 {
    fn encode_to_writer<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        let mut bytes = [0u8; 5];
        let i;
        let abs = self.abs();
        if abs >= 0x800_0000 {
            bytes[0] = (self >> 28) as u8;
            bytes[1] = (self >> 21) as u8 & SIGNIFICANT_BYTE;
            bytes[2] = (self >> 14) as u8 & SIGNIFICANT_BYTE;
            bytes[3] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[4] = *self as u8 | STOP_BYTE;
            i = 5;
        } else if abs >= 0x10_0000 {
            bytes[0] = (self >> 21) as u8 & SIGNIFICANT_BYTE;
            bytes[1] = (self >> 14) as u8 & SIGNIFICANT_BYTE;
            bytes[2] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[3] = *self as u8 | STOP_BYTE;
            i = 4;
        } else if abs >= 0x2000 {
            bytes[0] = (self >> 14) as u8 & SIGNIFICANT_BYTE;
            bytes[1] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[2] = *self as u8 | STOP_BYTE;
            i = 3;
        } else if abs >= 0x40 {
            bytes[0] = (self >> 7) as u8 & SIGNIFICANT_BYTE;
            bytes[1] = *self as u8 | STOP_BYTE;
            i = 2;
        } else {
            bytes[0] = *self as u8 | STOP_BYTE;
            i = 1;
        }
        output.write_all(&bytes[..i])
    }

    fn decode_from_reader<R: io::Read>(&mut self, input: &mut R) -> Result<()> {
        let bytes = decode_stop_bit_entity(input)?;
        let is_negative = (bytes[0] & NEGATIVE_SIGN_MASK) != 0;
        *self = -(is_negative as i32);
        for byte in bytes {
            *self = (*self << 7) | i32::from(byte);
        }
        Ok(())
    }
}

impl Codec for Vec<u8> {
    fn encode_to_writer<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        let len = self.len() as u32;
        len.encode_to_writer(output)?;
        output.write_all(self)
    }

    fn decode_from_reader<R: io::Read>(&mut self, input: &mut R) -> Result<()> {
        let len = &mut 0u32;
        len.decode_from_reader(input)?;
        *self = vec![0u8; *len as usize];
        input.read_exact(&mut self[..])?;
        Ok(())
    }
}

impl Codec for String {
    fn encode_to_writer<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        let len = self.len() as u32;
        let bytes = self.as_bytes();
        len.encode_to_writer(output)?;
        output.write_all(bytes)
    }

    fn decode_from_reader<R: io::Read>(&mut self, input: &mut R) -> Result<()> {
        let len = &mut 0u32;
        len.decode_from_reader(input)?;
        let mut bytes = vec![0u8; *len as usize];
        input.read_exact(&mut bytes[..])?;
        *self = String::from_utf8_lossy(&bytes[..]).to_string();
        Ok(())
    }
}

fn decode_stop_bit_entity<R: io::Read>(input: &mut R) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    loop {
        let mut byte = [0u8; 1];
        input.read_exact(&mut byte[..])?;
        if byte[0] >= STOP_BYTE {
            byte[0] ^= STOP_BYTE;
            bytes.push(byte[0]);
            break;
        } else {
            bytes.push(byte[0]);
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn encode_then_decode_u32(expected_value: u32) -> bool {
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.encode_to_writer(&mut bytes).unwrap();
        let value = &mut 0u32;
        value.decode_from_reader(&mut &bytes[..]).unwrap();
        *value == expected_value
    }

    #[quickcheck]
    fn encode_then_decode_i32(expected_value: i32) -> bool {
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.encode_to_writer(&mut bytes).unwrap();
        let value = &mut 0i32;
        value.decode_from_reader(&mut &bytes[..]).unwrap();
        *value == expected_value
    }

    #[test]
    fn encode_i32_example() {
        let mut bytes: Vec<u8> = Vec::new();
        (-794_2755_i32).encode_to_writer(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0x7c, 0x1b, 0x1b, 0x9d]);
    }

    #[test]
    fn decode_i32_fast_doc_example() {
        let bytes: Vec<u8> = vec![0x7c, 0x1b, 0x1b, 0x9d];
        let mut value = 0i32;
        value.decode_from_reader(&mut &bytes[..]).unwrap();
        assert_eq!(value, -794_2755);
    }

    #[test]
    fn encode_64i32_regression() {
        let mut bytes: Vec<u8> = Vec::new();
        (64i32).encode_to_writer(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0x00, 0xc0]);
    }

    #[test]
    fn encode_then_decode_99i32_regression() {
        let expected_value = 99i32;
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.encode_to_writer(&mut bytes).unwrap();
        let value = &mut 0i32;
        value.decode_from_reader(&mut &bytes[..]).unwrap();
        assert_eq!(*value, expected_value);
    }

    #[quickcheck]
    fn encode_then_decode_string(expected_value: String) -> bool {
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.encode_to_writer(&mut bytes).unwrap();
        let value = &mut String::default();
        value.decode_from_reader(&mut &bytes[..]).unwrap();
        *value == expected_value
    }

    #[quickcheck]
    fn encode_then_decode_bytes(expected_value: Vec<u8>) -> bool {
        let mut bytes: Vec<u8> = Vec::default();
        expected_value.encode_to_writer(&mut bytes).unwrap();
        let value = &mut Vec::default();
        value.decode_from_reader(&mut &bytes[..]).unwrap();
        *value == expected_value
    }
}
