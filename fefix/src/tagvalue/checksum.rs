use super::SerializeField;
use crate::Buffer;

#[derive(Debug)]
pub struct CheckSum(pub u8);

impl CheckSum {
    pub fn compute(data: &[u8]) -> Self {
        let mut value = 0u8;
        for byte in data {
            value = value.wrapping_add(*byte);
        }
        Self(value)
    }
}

impl SerializeField for CheckSum {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        [
            to_digit(self.0 / 100),
            to_digit((self.0 / 10) % 10),
            to_digit(self.0 % 10),
        ]
        .serialize(buffer)
    }
}

fn to_digit(byte: u8) -> u8 {
    byte + b'0'
}
