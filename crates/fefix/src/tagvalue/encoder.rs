use super::{Config, Configure, FvWrite};
use crate::buffer::Buffer;
use crate::definitions::fix44;
use crate::dict;
use crate::dict::IsFieldDefinition;
use crate::fix_values::CheckSum;
use crate::FixValue;
use crate::TagU16;
use std::ops::Range;

/// A buffered, content-agnostic FIX encoder.
///
/// [`Encoder`] is the fundamental building block for building higher-level
/// FIX encoders. It allows for encoding of arbitrary payloads and takes care of
/// `BodyLength (9)` and `CheckSum (10)`.
///
/// # Examples
///
/// ```
/// use fefix::tagvalue::{Config, Encoder};
///
/// let mut buffer = Vec::new();
/// let mut encoder = Encoder::<Config>::default();
/// encoder.config_mut().set_separator(b'|');
/// let msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"A");
/// let data = msg.wrap();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Encoder<C = Config>
where
    C: Configure,
{
    config: C,
}

impl<C> Encoder<C>
where
    C: Configure,
{
    /// Creates a new [`Encoder`] from the given `config` options.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Encoder};
    ///
    /// let mut config = Config::default();
    /// config.set_separator(b'|');
    /// let encoder = Encoder::new(config);
    /// assert_eq!(encoder.config().separator(), b'|');
    /// ```
    pub fn new(config: C) -> Self {
        Self { config }
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

    pub fn start_message<'a>(
        &'a mut self,
        begin_string: &[u8],
        buffer: &'a mut Vec<u8>,
        msg_type: &[u8],
    ) -> EncoderHandle<'a, Vec<u8>, C> {
        let mut state = EncoderHandle {
            raw_encoder: self,
            buffer,
            body_start_i: 0,
        };
        state.set(fix44::BEGIN_STRING, begin_string);
        // The second field is supposed to be `BodyLength(9)`, but obviously
        // the length of the message is unknow until later in the
        // serialization phase. This alone would usually require to
        //
        //  1. Serialize the rest of the message into an external buffer.
        //  2. Calculate the length of the message.
        //  3. Serialize `BodyLength(9)` to `buffer`.
        //  4. Copy the contents of the external buffer into `buffer`.
        //  5. ... go on with the serialization process.
        //
        // Luckily, FIX allows for zero-padded integer values and we can
        // leverage this to reserve some space for the value. We waste
        // some bytes but the benefits largely outweight the costs.
        //
        // Six digits (~1MB) ought to be enough for every message.
        state.set_any(fix44::BODY_LENGTH.tag(), b"000000" as &[u8]);
        state.body_start_i = state.buffer.len();
        state.set_any(fix44::MSG_TYPE.tag(), msg_type);
        state
    }
}

/// A type returned by [`Encoder::start_message`](Encoder::start_message) to
/// actually encode data fields.
#[derive(Debug)]
pub struct EncoderHandle<'a, B, C = Config>
where
    B: Buffer,
    C: Configure,
{
    raw_encoder: &'a mut Encoder<C>,
    buffer: &'a mut B,
    body_start_i: usize,
}

impl<'a, B, C> EncoderHandle<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    /// Adds a `field` with a `value` to the current message.
    pub fn set<'b, F, T>(&mut self, field: &F, value: T)
    where
        F: dict::IsFieldDefinition,
        T: FixValue<'b>,
    {
        self.set_any(field.tag(), value)
    }

    pub fn set_any<'b, T>(&mut self, tag: TagU16, value: T)
    where
        T: FixValue<'b>,
    {
        tag.serialize(self.buffer);
        self.buffer.extend_from_slice(b"=" as &[u8]);
        value.serialize(self.buffer);
        self.buffer
            .extend_from_slice(&[self.raw_encoder.config().separator()]);
    }

    pub fn raw(&mut self, raw: &[u8]) {
        self.buffer.extend_from_slice(raw);
    }

    /// Closes the current message writing operation and returns its byte
    /// representation.
    pub fn wrap(mut self) -> &'a [u8] {
        self.write_body_length();
        self.write_checksum();
        self.buffer.as_slice()
    }

    fn body_length_writable_range(&self) -> Range<usize> {
        self.body_start_i - 7..self.body_start_i - 1
    }

    fn body_length(&self) -> usize {
        self.buffer.as_slice().len() - self.body_start_i
    }

    fn write_body_length(&mut self) {
        let body_length = self.body_length();
        let body_length_range = self.body_length_writable_range();
        let slice = &mut self.buffer.as_mut_slice()[body_length_range];
        slice[0] = to_digit((body_length / 100000) as u8 % 10);
        slice[1] = to_digit((body_length / 10000) as u8 % 10);
        slice[2] = to_digit((body_length / 1000) as u8 % 10);
        slice[3] = to_digit((body_length / 100) as u8 % 10);
        slice[4] = to_digit((body_length / 10) as u8 % 10);
        slice[5] = to_digit((body_length / 1) as u8 % 10);
    }

    fn write_checksum(&mut self) {
        let checksum = CheckSum::compute(self.buffer.as_slice());
        self.set(fix44::CHECK_SUM, checksum);
    }
}

impl<'a, B, C> FvWrite<'a> for EncoderHandle<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    type Key = TagU16;

    fn set_fv_with_key<'b, T>(&'b mut self, key: &Self::Key, value: T)
    where
        T: FixValue<'b>,
    {
        self.set_any(*key, value);
    }

    fn set_fv<'b, V, F>(&'b mut self, field: &F, value: V)
    where
        V: FixValue<'b>,
        F: IsFieldDefinition,
    {
        self.set_fv_with_key(&field.tag(), value);
    }
}

fn to_digit(byte: u8) -> u8 {
    byte + b'0'
}
