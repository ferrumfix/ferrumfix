use crate::buffering::Buffer;
use crate::tagvalue::{utils, Config, Configure};
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
    body_start_i: usize,
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
            body_start_i: 0,
            config: C::default(),
        }
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

    /// Sets the `BeginString (8)` field in the FIX message. This method must be
    /// called first during the encoding phase. Any of the following will result
    /// in invalid FIX messages:
    ///
    /// - Not calling [`RawEncoder::set_begin_string`].
    /// - Calling [`RawEncoder::set_begin_string`] multiple times.
    /// - Calling [`RawEncoder::set_begin_string`] before any other
    /// [`RawEncoder`] methods.
    ///
    /// # Examples
    /// ```
    /// use fefix::tagvalue::{Config, RawEncoder};
    ///
    /// let encoder = &mut RawEncoder::<_, Config>::from_buffer(Vec::new());
    /// encoder.set_begin_string(b"FIX.4.4");
    /// encoder.extend_from_slice(b"...");
    /// let data = encoder.finalize();
    /// assert!(data.starts_with(b"8=FIX.4.4"));
    /// ```
    pub fn set_begin_string(&mut self, begin_string: &[u8]) {
        self.buffer.clear();
        // First, write `BeginString(8)`.
        self.buffer.extend_from_slice(b"8=");
        self.buffer.extend_from_slice(begin_string);
        self.buffer.extend_from_slice(&[
            self.config.separator(),
            b'9',
            b'=',
            b'0',
            b'0',
            b'0',
            b'0',
            b'0',
            b'0',
            self.config.separator(),
        ]);
        self.body_start_i = self.buffer.len();
    }

    /// Adds `data` to the payload part of the FIX message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, RawEncoder};
    ///
    /// let encoder = &mut RawEncoder::<_, Config>::from_buffer(Vec::new());
    /// encoder.config_mut();
    /// encoder.set_begin_string(b"FIX.4.2");
    /// encoder.extend_from_slice(b"1=fake-body|2=foobar|");
    /// let data = encoder.finalize();
    /// assert!(data.starts_with(b"8=FIX.4.2"));
    /// ```
    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    /// Writes `CheckSum (10)` and `BodyLength (9)` and then returns an immutable
    /// reference over the raw FIX message.
    pub fn finalize(&mut self) -> &[u8] {
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
        slice[0] = (body_length / 100000) as u8 + b'0';
        slice[1] = ((body_length / 10000) % 10) as u8 + b'0';
        slice[2] = ((body_length / 1000) % 10) as u8 + b'0';
        slice[3] = ((body_length / 100) % 10) as u8 + b'0';
        slice[4] = ((body_length / 10) % 10) as u8 + b'0';
        slice[5] = (body_length % 10) as u8 + b'0';
    }

    fn write_checksum(&mut self) {
        let checksum = utils::checksum_10(self.buffer.as_slice());
        self.buffer.extend_from_slice(&[
            b'1',
            b'0',
            b'=',
            (checksum / 100) + b'0',
            ((checksum / 10) % 10) + b'0',
            (checksum % 10) + b'0',
            self.config.separator(),
        ]);
    }
}
