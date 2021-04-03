use crate::Buffer;

/// A trait for serializing data directly into a [`Buffer`].
pub trait SerializeField {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer;
}

impl SerializeField for bool {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let byte;
        if *self {
            byte = b'Y';
        } else {
            byte = b'N';
        }
        buffer.extend_from_slice(&[byte]);
        1
    }
}

impl SerializeField for &[u8] {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self);
        self.len()
    }
}

impl<const N: usize> SerializeField for [u8; N] {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        buffer.extend_from_slice(self);
        self.len()
    }
}

impl SerializeField for u32 {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }
}

impl SerializeField for i32 {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }
}

impl SerializeField for u64 {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }
}

impl SerializeField for i64 {
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        let s = self.to_string();
        buffer.extend_from_slice(s.as_bytes());
        s.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_bools() {
        let mut buffer = Vec::new();
        assert_eq!(true.serialize(&mut buffer), 1);
        assert_eq!(false.serialize(&mut buffer), 1);
        assert_eq!(&buffer[..], b"YN" as &[u8]);
    }

    #[test]
    fn serialize_bytes() {
        let data: &[&[u8]] = &[b"hello", b"", b" ", b"foo"];
        let mut buffer = Vec::new();
        for slice in data {
            assert_eq!(slice.serialize(&mut buffer), slice.len());
        }
        assert_eq!(&buffer[..], b"hello foo" as &[u8]);
    }

    #[test]
    fn serialize_country() {
        let mut buffer = Vec::new();
        assert_eq!(b"IT".serialize(&mut buffer), 2);
        assert_eq!(&buffer[..], b"IT" as &[u8]);
    }

    #[test]
    fn serialize_currency() {
        let mut buffer = Vec::new();
        assert_eq!(b"USD".serialize(&mut buffer), 3);
        assert_eq!(&buffer[..], b"USD" as &[u8]);
    }
}
