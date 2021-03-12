use crate::buffering::Buffer;
use crate::tagvalue::{utils, DecodeError};
use std::ops::Range;

/// An immutable view over the raw contents of a FIX message.
#[derive(Debug)]
pub struct RawFrame<'a> {
    begin_string: &'a [u8],
    body: &'a [u8],
}

impl<'a> RawFrame<'a> {
    /// Returns an immutable reference to the `BeginString <8>` field value of
    /// `self`.
    ///
    /// ```
    /// use fefix::tagvalue::RawDecoder;
    ///
    /// let decoder = RawDecoder::new().with_separator(b'|');
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let message = decoder.decode(data).unwrap();
    ///
    /// assert_eq!(message.begin_string(), b"FIX.4.2");
    /// ```
    pub fn begin_string(&self) -> &[u8] {
        self.begin_string
    }

    /// Returns an immutable reference to the body contents of `self`. In this
    /// context, "body" means all fields besides
    ///
    /// - `BeginString <8>`;
    /// - `BodyLength <9>`;
    /// - `CheckSum <10>`.
    ///
    /// According to this definition, the body may also contain fields that are
    /// technically part of `StandardHeader` and `StandardTrailer`.
    ///
    /// ```
    /// use fefix::tagvalue::RawDecoder;
    ///
    /// let decoder = RawDecoder::new().with_separator(b'|');
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let message = decoder.decode(data).unwrap();
    ///
    /// assert_eq!(message.payload().len(), 42);
    /// ```
    pub fn payload(&self) -> &[u8] {
        self.body
    }
}

/// A buffered, content-agnostic FIX encoder.
///
/// [`RawEncoder`] is the fundamental building block for building higher-level
/// FIX encoders. It allows for encoding of arbitrary payloads and takes care of
/// `BodyLength (9)` and `CheckSum (10)`.
///
/// # Examples
///
/// ```
/// use fefix::tagvalue::RawEncoder;
///
/// let encoder = &mut RawEncoder::from_buffer(Vec::new());
/// encoder.set_separator(b'|');
/// encoder.set_begin_string(b"FIX.4.4");
/// encoder.extend_from_slice(b"35=0|49=A|56=B|34=12|52=20100304-07:59:30|");
/// let data = encoder.finalize();
/// assert_eq!(data, b"8=FIX.4.4|9=000042|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=216|");
/// ```
#[derive(Debug, Clone)]
pub struct RawEncoder<B = Vec<u8>>
where
    B: Buffer,
{
    buffer: B,
    body_start_i: usize,
    separator: u8,
}

impl<B> RawEncoder<B>
where
    B: Buffer,
{
    pub fn from_buffer(buffer: B) -> Self {
        Self {
            buffer,
            body_start_i: 0,
            separator: 0x1,
        }
    }

    pub fn set_separator(&mut self, separator: u8) {
        self.separator = separator;
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
            self.separator,
        ]);
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
    /// use fefix::tagvalue::RawEncoder;
    ///
    /// let encoder = &mut RawEncoder::from_buffer(Vec::new());
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
            self.separator,
            b'9',
            b'=',
            b'0',
            b'0',
            b'0',
            b'0',
            b'0',
            b'0',
            self.separator,
        ]);
        self.body_start_i = self.buffer.len();
    }

    /// Adds `data` to the payload part of the FIX message.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::RawEncoder;
    ///
    /// let encoder = &mut RawEncoder::from_buffer(Vec::new());
    /// encoder.set_separator(b'|');
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
}

/// An [`Encoding`] that does as minimal parsing as possible and operates over
/// [`AgnosticMessage`] values.
///
/// Content-agnostic operations are only useful for complex and/or unusual needs,
/// e.g.:
///
/// - Non `Latin-1` -compatible encoding.
/// - Custom application-level encryption mechanism.
#[derive(Debug, Clone)]
pub struct RawDecoder {
    separator: u8,
    verify_checksum: bool,
}

impl RawDecoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_separator(&mut self, separator: u8) -> Self {
        self.separator = separator;
        self.clone()
    }

    pub fn with_checksum_verification(&mut self, verify: bool) -> Self {
        self.verify_checksum = verify;
        self.clone()
    }

    pub fn decode<'a>(&self, data: &'a [u8]) -> Result<RawFrame<'a>, DecodeError> {
        if data.len() < utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            return Err(DecodeError::Syntax);
        }
        let header_indices = utils::parse_header_indices(data, self.separator)?;
        utils::verify_body_length(
            data,
            header_indices.start_of_body(),
            header_indices.body().len(),
        )?;
        if self.verify_checksum {
            utils::verify_checksum(data)?;
        }
        let begin_string = &data[header_indices.begin_string_value()];
        let contents = &data[header_indices.body()];
        Ok(RawFrame {
            begin_string,
            body: contents,
        })
    }
}

impl Default for RawDecoder {
    fn default() -> Self {
        Self {
            separator: 0x1, // SOH
            verify_checksum: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawDecoderBuffered<B = Vec<u8>>
where
    B: Buffer,
{
    buffer: B,
    buffer_actual_len: usize,
    _buffer_max_len: usize,
    decoder: RawDecoder,
}

impl<B> RawDecoderBuffered<B>
where
    B: Buffer,
{
    /// Provides a buffer that must be filled before re-attempting to deserialize
    /// the next [`RawFrame`].
    pub fn supply_buffer(&mut self) -> &mut [u8] {
        let data = &self.buffer.as_slice()[..self.buffer_actual_len];
        let decode_result = self.decoder.decode(data).map(|_| ());
        match decode_result {
            Ok(_) => &mut [],
            Err(_) => todo!("TODO"),
        }
    }

    pub fn current_frame(&self) -> Option<RawFrame> {
        let data = &self.buffer.as_slice()[..self.buffer_actual_len];
        self.decoder.decode(data).ok()
    }
}

//impl StreamingRawDecoder<Frame> for RawDecoderBuffered {
//    type Error = DecodeError;
//
//    fn supply_buffer(&mut self) -> (&mut usize, &mut [u8]) {
//        debug_assert!(self.buffer_len <= self.buffer.len());
//        match self.progress {
//            None => {
//
//            },
//            Some(header_indices) => {
//                (self.buffer_len, &self.buffer[..])
//            },
//        }
//    }
//
//    fn attempt_decoding(&mut self) -> Result<Option<&Frame>, Self::Error> {
//
//    }
//}

#[cfg(test)]
mod test {
    use super::*;

    fn new_decoder() -> RawDecoder {
        RawDecoder::new().with_separator(b'|')
    }

    #[test]
    fn agnostic_simple_message() {
        let decoder = new_decoder();
        let msg = b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"FIX.4.2");
        assert_eq!(frame.payload(), b"35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|");
    }

    #[test]
    fn empty_body() {
        let decoder = new_decoder();
        let msg = b"8=FIX.FOOBAR|9=0|10=225|";
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"FIX.FOOBAR");
        assert_eq!(frame.payload(), b"");
    }

    #[test]
    fn edge_cases_dont_cause_panic() {
        let decoder = new_decoder();
        decoder.decode(b"8=FIX.FOOBAR|9=0|10=225|").ok();
        decoder.decode(b"8=|9=0|10=225|").ok();
        decoder.decode(b"8=|9=0|10=|").ok();
        decoder.decode(b"8====|9=0|10=|").ok();
        decoder.decode(b"|||9=0|10=|").ok();
        decoder.decode(b"9999999999999").ok();
        decoder.decode(b"-9999999999999").ok();
        decoder.decode(b"==============").ok();
        decoder.decode(b"9999999999999|").ok();
        decoder.decode(b"|999999999999=|").ok();
        decoder.decode(b"|999=999999999999999999|=").ok();
    }
}
