use super::SerializeField;
use super::{CheckSum, Config, Configure};
use crate::buffer::Buffer;
use std::ops::Range;

/// A buffered, content-agnostic FIX encoder.
///
/// [`RawEncoder`] is the fundamental building block for building higher-level
/// FIX encoders. It allows for encoding of arbitrary payloads and takes care of
/// `BodyLength (9)` and `CheckSum (10)`.
///
/// # Examples
///
/// ```
/// use fefix::tagvalue::{Config, RawEncoder};
///
/// let encoder = &mut RawEncoder::<_, Config>::from_buffer(Vec::new());
/// encoder.config_mut().set_separator(b'|');
/// encoder.set_begin_string(b"FIX.4.4");
/// encoder.extend_from_slice(b"35=0|49=A|56=B|34=12|52=20100304-07:59:30|");
/// let data = encoder.finalize();
/// assert_eq!(data, b"8=FIX.4.4|9=000042|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=216|");
/// ```
#[derive(Debug, Clone)]
pub struct RawEncoder<B = Vec<u8>, C = Config>
where
    B: Buffer,
    C: Configure,
{
    buffer: B,
    config: C,
}

impl<B, C> RawEncoder<B, C>
where
    B: Buffer,
    C: Configure,
{
    pub fn from_buffer(buffer: B) -> Self {
        Self {
            buffer,
            config: C::default(),
        }
    }

    pub fn buffer(&self) -> &B {
        &self.buffer
    }

    pub fn buffer_mut(&self) -> &B {
        &self.buffer
    }

    /// Returns an immutable reference to the [`Configure`] implementor used by
    /// `self`.
    pub fn config(&self) -> &C {
        &self.config
    }

    /// Returns a mutable reference to the [`Configure`] implementor used by
    /// `self`.
    pub fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }

    pub fn new_message(&mut self, begin_string: &[u8]) -> RawEncoderState<B, C> {
        self.buffer.clear();
        let mut state = RawEncoderState {
            raw_encoder: self,
            body_start_i: 0,
        };
        // First, write `BeginString(8)`.
        state.add_field(8, begin_string);
        state.add_field(9, b"000000" as &[u8]);
        state.body_start_i = state.raw_encoder.buffer.len();
        state
    }
}

#[derive(Debug)]
pub struct RawEncoderState<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    raw_encoder: &'a mut RawEncoder<B, C>,
    body_start_i: usize,
}

impl<'a, B, C> RawEncoderState<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    pub fn add_field<T>(&mut self, tag: u32, value: T)
    where
        T: SerializeField,
    {
        tag.serialize(&mut self.raw_encoder.buffer);
        self.raw_encoder.buffer.extend_from_slice(b"=" as &[u8]);
        value.serialize(&mut self.raw_encoder.buffer);
        self.raw_encoder
            .buffer
            .extend_from_slice(&[self.raw_encoder.config().separator()]);
    }

    pub fn wrap(mut self) -> &'a [u8] {
        self.write_body_length();
        self.write_checksum();
        self.raw_encoder.buffer.as_slice()
    }

    fn body_length_writable_range(&self) -> Range<usize> {
        self.body_start_i - 7..self.body_start_i - 1
    }

    fn body_length(&self) -> usize {
        self.raw_encoder.buffer.as_slice().len() - self.body_start_i
    }

    fn write_body_length(&mut self) {
        let body_length = self.body_length();
        let body_length_range = self.body_length_writable_range();
        let slice = &mut self.raw_encoder.buffer.as_mut_slice()[body_length_range];
        slice[0] = to_digit((body_length / 100000) as u8 % 10);
        slice[1] = to_digit((body_length / 10000) as u8 % 10);
        slice[2] = to_digit((body_length / 1000) as u8 % 10);
        slice[3] = to_digit((body_length / 100) as u8 % 10);
        slice[4] = to_digit((body_length / 10) as u8 % 10);
        slice[5] = to_digit((body_length / 1) as u8 % 10);
    }

    fn write_checksum(&mut self) {
        let checksum = CheckSum::compute(self.raw_encoder.buffer.as_slice());
        self.add_field(10, checksum);
    }
}

fn to_digit(byte: u8) -> u8 {
    byte + b'0'
}
