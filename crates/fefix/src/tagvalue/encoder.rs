use super::{Config, Configure};
use crate::dict::IsFieldDefinition;
use crate::field_types::CheckSum;
use crate::{Buffer, BufferWriter, FieldType, GetConfig, SetField, TagU16};
use std::fmt::Write;
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
/// use fefix::prelude::*;
///
/// let mut buffer = Vec::new();
/// let mut encoder = Encoder::<Config>::default();
/// encoder.config_mut().set_separator(b'|');
/// let msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"A");
/// let data = msg.done();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Encoder<C = Config> {
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
    /// use fefix::prelude::*;
    ///
    /// let mut config = Config::default();
    /// config.set_separator(b'|');
    /// let encoder = Encoder::new(config);
    /// assert_eq!(encoder.config().separator(), b'|');
    /// ```
    pub fn new(config: C) -> Self {
        Self { config }
    }

    /// Creates a new [`EncoderHandle`] that allows to set the field values of a
    /// new FIX message. The raw byte contents of the newly created FIX messages
    /// are appended directly at the end of `buffer`.
    pub fn start_message<'a, B>(
        &'a mut self,
        begin_string: &[u8],
        buffer: &'a mut B,
        msg_type: &[u8],
    ) -> EncoderHandle<'a, B, C>
    where
        B: Buffer,
    {
        let initial_buffer_len = buffer.len();
        let mut state = EncoderHandle {
            encoder: self,
            buffer,
            initial_buffer_len,
            body_start_i: 0,
        };
        state.set(8, begin_string);
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
        // Eight digits (~100MB) are enough for every message.
        state.set(9, b"00000000" as &[u8]);
        state.body_start_i = state.buffer.len();
        state.set(35, msg_type);
        state
    }
}

impl<C> GetConfig for Encoder<C> {
    type Config = C;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        &mut self.config
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
    encoder: &'a mut Encoder<C>,
    buffer: &'a mut B,
    initial_buffer_len: usize,
    body_start_i: usize,
}

impl<'a, B, C> EncoderHandle<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    /// Closes the current message writing operation and returns its byte
    /// representation, as well as its offset within the whole contents of the
    /// [`Buffer`].
    pub fn done(mut self) -> (&'a [u8], usize) {
        self.write_body_length();
        self.write_checksum();
        (self.buffer.as_slice(), self.initial_buffer_len)
    }

    fn body_length_writable_range(&self) -> Range<usize> {
        self.body_start_i - 9..self.body_start_i - 1
    }

    fn body_length(&self) -> usize {
        self.buffer.as_slice().len() - self.body_start_i
    }

    fn write_body_length(&mut self) {
        let body_length = self.body_length() as u32;
        let body_length_range = self.body_length_writable_range();
        let slice = &mut self.buffer.as_mut_slice()[body_length_range];
        slice[0] = to_digit((body_length / 10000000) as u8 % 10);
        slice[1] = to_digit((body_length / 1000000) as u8 % 10);
        slice[2] = to_digit((body_length / 100000) as u8 % 10);
        slice[3] = to_digit((body_length / 10000) as u8 % 10);
        slice[4] = to_digit((body_length / 1000) as u8 % 10);
        slice[5] = to_digit((body_length / 100) as u8 % 10);
        slice[6] = to_digit((body_length / 10) as u8 % 10);
        slice[7] = to_digit((body_length / 1) as u8 % 10);
    }

    fn write_checksum(&mut self) {
        let checksum = CheckSum::compute(self.buffer.as_slice());
        self.set(10, checksum);
    }
}

fn to_digit(byte: u8) -> u8 {
    byte + b'0'
}

impl<'a, B, C> SetField<u32> for EncoderHandle<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    fn set_with<'s, V>(&'s mut self, tag: u32, value: V, settings: V::SerializeSettings)
    where
        V: FieldType<'s>,
    {
        write!(BufferWriter(self.buffer), "{}=", tag).unwrap();
        value.serialize_with(self.buffer, settings);
        self.buffer
            .extend_from_slice(&[self.encoder.config().separator()]);
    }
}

impl<'a, B, C> SetField<TagU16> for EncoderHandle<'a, B, C>
where
    B: Buffer,
    C: Configure,
{
    fn set_with<'s, V>(&'s mut self, tag: TagU16, value: V, settings: V::SerializeSettings)
    where
        V: FieldType<'s>,
    {
        self.set_with(tag.get() as u32, value, settings)
    }
}

impl<'a, B, C, F> SetField<&F> for EncoderHandle<'a, B, C>
where
    B: Buffer,
    C: Configure,
    F: IsFieldDefinition,
{
    fn set_with<'s, V>(&'s mut self, field: &F, value: V, settings: V::SerializeSettings)
    where
        V: FieldType<'s>,
    {
        self.set_with(field.tag(), value, settings)
    }
}
