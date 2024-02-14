use bitvec::prelude::*;
use std::io;

const STOP_BYTE: u8 = 0x80;
const SIGNIFICANT_BYTE: u8 = !STOP_BYTE;
const NEGATIVE_SIGN_MASK: u8 = 0x40;

/// A trait to (de)serialize on-the-wire representations of entities.
pub trait Codec {
    fn deserialize(&mut self, input: &mut impl io::Read) -> io::Result<usize>;
    fn serialize(&self, output: &mut impl io::Write) -> io::Result<usize>;
}

impl Codec for u32 {
    fn serialize(&self, output: &mut impl io::Write) -> io::Result<usize> {
        let num_ignored_bytes = self.leading_zeros() / 7;
        let bytes = [
            (self >> 28) as u8 & SIGNIFICANT_BYTE,
            (self >> 21) as u8 & SIGNIFICANT_BYTE,
            (self >> 14) as u8 & SIGNIFICANT_BYTE,
            (self >> 7) as u8 & SIGNIFICANT_BYTE,
            *self as u8 | STOP_BYTE,
        ];

        output.write_all(&bytes[num_ignored_bytes as usize..])?;
        Ok(bytes.len() - num_ignored_bytes as usize)
    }

    fn deserialize(&mut self, input: &mut impl io::Read) -> io::Result<usize> {
        *self = 0;
        let bytes = decode_stop_bit_entity(input)?;
        for byte in &bytes {
            *self = (*self << 7) | u32::from(*byte);
        }
        Ok(bytes.len())
    }
}

impl Codec for i32 {
    fn serialize(&self, output: &mut impl io::Write) -> io::Result<usize> {
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
        output.write_all(&bytes[..i])?;
        Ok(i)
    }

    fn deserialize(&mut self, input: &mut impl io::Read) -> io::Result<usize> {
        let bytes = decode_stop_bit_entity(input)?;
        let is_negative = (bytes[0] & NEGATIVE_SIGN_MASK) != 0;
        *self = -(is_negative as i32);
        for byte in &bytes {
            *self = (*self << 7) | i32::from(*byte);
        }
        Ok(bytes.len())
    }
}

impl Codec for u64 {
    fn deserialize(&mut self, _input: &mut impl io::Read) -> io::Result<usize> {
        todo!();
    }

    fn serialize(&self, _output: &mut impl io::Write) -> io::Result<usize> {
        todo!();
    }
}

impl Codec for i64 {
    fn deserialize(&mut self, _input: &mut impl io::Read) -> io::Result<usize> {
        todo!();
    }

    fn serialize(&self, _output: &mut impl io::Write) -> io::Result<usize> {
        todo!();
    }
}

impl Codec for Vec<u8> {
    fn serialize(&self, output: &mut impl io::Write) -> io::Result<usize> {
        let len = self.len() as u32;
        len.serialize(output)?;
        output.write_all(self)?;
        Ok(len as usize)
    }

    fn deserialize(&mut self, input: &mut impl io::Read) -> io::Result<usize> {
        let mut len = 0u32;
        len.deserialize(input)?;
        *self = vec![0u8; len as usize];
        input.read_exact(&mut self[..])?;
        Ok(len as usize)
    }
}

impl Codec for String {
    fn serialize(&self, output: &mut impl io::Write) -> io::Result<usize> {
        let len = self.len() as u32;
        let bytes = self.as_bytes();
        len.serialize(output)?;
        output.write_all(bytes)?;
        Ok(len as usize)
    }

    fn deserialize(&mut self, input: &mut impl io::Read) -> io::Result<usize> {
        let mut len = 0u32;
        len.deserialize(input)?;
        let mut bytes = vec![0u8; len as usize];
        input.read_exact(&mut bytes[..])?;
        *self = String::from_utf8_lossy(&bytes[..]).to_string();
        Ok(len as usize)
    }
}

fn _serialize_bitvec(bits: &BitSlice<Msb0, u8>, output: &mut impl io::Write) -> io::Result<usize> {
    let significant_data_bits_per_byte = bits.chunks_exact(7);
    let mut i = 0;
    let remaineder = significant_data_bits_per_byte.remainder().load::<u8>();
    for significant_data_bits in significant_data_bits_per_byte {
        let byte = significant_data_bits.load::<u8>();
        if byte != 0 {
            output.write_all(&[byte])?;
            i += 1;
        }
    }
    if remaineder != 0 {
        output.write_all(&[STOP_BYTE | remaineder])?;
    }
    Ok(i)
}

#[derive(Debug, Clone)]
pub struct PresenceMap {
    bits: BitVec,
}

impl PresenceMap {
    pub fn bits(&self) -> impl Iterator<Item = &bool> {
        self.bits.iter()
    }
}

impl Codec for PresenceMap {
    fn serialize(&self, _output: &mut impl io::Write) -> io::Result<usize> {
        todo!();
    }

    fn deserialize(&mut self, input: &mut impl io::Read) -> io::Result<usize> {
        self.bits = BitVec::new();
        let mut stop_bit = false;
        while !stop_bit {
            let mut buffer = [0u8; 1];
            input.read_exact(&mut buffer[..])?;
            let byte = buffer[0];
            stop_bit = byte >= STOP_BYTE;
            if !stop_bit {
                self.bits.push(byte >> 7 == 1);
            }
            self.bits.push((byte >> 6) & 1 == 1);
            self.bits.push((byte >> 5) & 1 == 1);
            self.bits.push((byte >> 4) & 1 == 1);
            self.bits.push((byte >> 4) & 1 == 1);
            self.bits.push((byte >> 3) & 1 == 1);
            self.bits.push((byte >> 2) & 1 == 1);
            self.bits.push((byte >> 1) & 1 == 1);
            self.bits.push(byte & 1 == 1);
        }
        Ok(self.bits.len())
    }
}

//pub fn encode_stop_bit_entity(target: &mut impl io::Write, buffer: &[u8]) -> io::Result<usize> {
//    let bits = BitVec::from(buffer);
//    for bit in bits {
//        target:w
//    }
//}

pub fn decode_stop_bit_entity(input: &mut impl io::Read) -> io::Result<Vec<u8>> {
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

#[allow(dead_code)]
pub fn decode_stop_bit_bitvec(input: &mut impl io::Read) -> io::Result<BitVec> {
    let mut bits = BitVec::new();
    let mut stop_bit = false;
    while !stop_bit {
        let mut buffer = [0u8; 1];
        input.read_exact(&mut buffer[..])?;
        let byte = buffer[0];
        stop_bit = byte >= STOP_BYTE;
        if !stop_bit {
            bits.push(byte >> 7 == 1);
        }
        bits.push((byte >> 6) & 1 == 1);
        bits.push((byte >> 5) & 1 == 1);
        bits.push((byte >> 4) & 1 == 1);
        bits.push((byte >> 4) & 1 == 1);
        bits.push((byte >> 3) & 1 == 1);
        bits.push((byte >> 2) & 1 == 1);
        bits.push((byte >> 1) & 1 == 1);
        bits.push(byte & 1 == 1);
    }
    Ok(bits)
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn encode_then_decode_u32(expected_value: u32) -> bool {
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.serialize(&mut bytes).unwrap();
        let mut value = 0u32;
        value.deserialize(&mut &bytes[..]).unwrap();
        value == expected_value
    }

    #[test]
    fn encode_i32_example() {
        let mut bytes: Vec<u8> = Vec::new();
        (-794_2755_i32).serialize(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0x7c, 0x1b, 0x1b, 0x9d]);
    }

    #[test]
    fn decode_i32_fast_doc_example() {
        let bytes: Vec<u8> = vec![0x7c, 0x1b, 0x1b, 0x9d];
        let mut value = 0i32;
        value.deserialize(&mut &bytes[..]).unwrap();
        assert_eq!(value, -794_2755);
    }

    #[test]
    fn encode_64i32_regression() {
        let mut bytes: Vec<u8> = Vec::new();
        (64i32).serialize(&mut bytes).unwrap();
        assert_eq!(bytes, vec![0x00, 0xc0]);
    }

    #[test]
    fn encode_then_decode_99i32_regression() {
        let expected_value = 99i32;
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.serialize(&mut bytes).unwrap();
        let mut value = 0i32;
        value.deserialize(&mut &bytes[..]).unwrap();
        assert_eq!(value, expected_value);
    }

    #[quickcheck]
    fn encode_then_decode_string(expected_value: String) -> bool {
        let mut bytes: Vec<u8> = Vec::new();
        expected_value.serialize(&mut bytes).unwrap();
        let mut value = String::default();
        value.deserialize(&mut &bytes[..]).unwrap();
        *value == expected_value
    }

    #[quickcheck]
    fn encode_then_decode_bytes(expected_value: Vec<u8>) -> bool {
        let mut bytes: Vec<u8> = Vec::default();
        expected_value.serialize(&mut bytes).unwrap();
        let mut value = Vec::default();
        value.deserialize(&mut &bytes[..]).unwrap();
        *value == expected_value
    }
}
