use crate::buffering::Buffer;
use crate::tagvalue::{utils, Config, Configure, DecodeError, RawFrame};
use std::ops::Range;

/// A bare-bones FIX decoder for low-level message handling.
///
/// [`RawDecoder`] is the fundamental building block for building higher-level
/// FIX decoder. It allows for decoding of arbitrary payloads and only "hides"
/// `BodyLength (9)` and `CheckSum (10)` to the final user. Everything else is
/// left to the user to deal with.
#[derive(Debug, Clone, Default)]
pub struct RawDecoder<C = Config>
where
    C: Configure,
{
    config: C,
}

impl<C> RawDecoder<C>
where
    C: Configure,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: C) -> Self {
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

    pub fn decode<'a>(&self, data: &'a [u8]) -> Result<RawFrame<'a>, DecodeError> {
        if data.len() < utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            return Err(DecodeError::Syntax);
        }
        let header_indices = parse_header_indices(data, self.config().separator())?;
        utils::verify_body_length(
            data,
            header_indices.start_of_body(),
            header_indices.body().len(),
        )?;
        if self.config().verify_checksum() {
            utils::verify_checksum(data)?;
        }
        let begin_string = &data[header_indices.begin_string_range()];
        let contents = &data[header_indices.body()];
        Ok(RawFrame::new(begin_string, contents))
    }
}

/// A bare-bones FIX decoder for byte streams.
#[derive(Debug, Clone)]
pub struct RawDecoderBuffered<B = Vec<u8>, C = Config>
where
    B: Buffer,
    C: Configure,
{
    buffer: B,
    buffer_actual_len: usize,
    decoder: RawDecoder<C>,
}

impl<B, C> RawDecoderBuffered<B, C>
where
    B: Buffer,
    C: Configure,
{
    /// Returns an immutable reference to the [`Configure`] implementor used by
    /// `self`.
    pub fn config(&self) -> &C {
        self.decoder.config()
    }

    /// Returns a mutable reference to the [`Configure`] implementor used by
    /// `self`.
    pub fn config_mut(&mut self) -> &mut C {
        self.decoder.config_mut()
    }

    /// Provides a buffer that must be filled before re-attempting to deserialize
    /// the next [`RawFrame`].
    pub fn supply_buffer(&mut self) -> &mut [u8] {
        let data = self.buffer.as_slice();
        let header_indices = parse_header_indices(data, self.config().separator());
        match header_indices {
            Ok(_indices) => {
                let len = self.buffer.as_slice().len();
                //let required_len = indices.body().end;
                //self.buffer.extend_from_slice(&[0; required_len]);
                &mut self.buffer.as_mut_slice()[len..]
            }
            Err(DecodeError::Incomplete) => {
                let start = self.buffer.as_slice().len();
                self.buffer.extend_from_slice(&[0; 7]);
                &mut self.buffer.as_mut_slice()[start..]
            }
            Err(_) => &mut [],
        }
    }

    pub fn current_frame(&self) -> Option<RawFrame> {
        let data = &self.buffer.as_slice()[..self.buffer_actual_len];
        self.decoder.decode(data).ok()
    }
}

// Information regarding the indices of "important" parts of the FIX message.
struct HeaderIndices {
    indexes_of_equal_sign: [usize; 2],
    indexes_of_separator: [usize; 2],
    body_length: usize,
}

impl HeaderIndices {
    fn empty() -> Self {
        Self {
            indexes_of_equal_sign: [0, 0],
            indexes_of_separator: [0, 0],
            body_length: 0,
        }
    }

    fn is_valid(&self) -> bool {
        // Let's check that we got valid values for everything we need.
        self.indexes_of_equal_sign[0] != 0
            || self.indexes_of_equal_sign[1] != 0
            || self.indexes_of_separator[0] != 0
            || self.indexes_of_separator[1] != 0
    }

    pub fn start_of_body(&self) -> usize {
        // The body starts at the character immediately after the separator of
        // `BodyLength`.
        self.indexes_of_separator[1] + 1
    }

    pub fn begin_string_range(&self) -> Range<usize> {
        self.indexes_of_equal_sign[0] + 1..self.indexes_of_separator[0]
    }

    pub fn body(&self) -> Range<usize> {
        let start = self.start_of_body();
        start..start + self.body_length
    }
}

fn parse_header_indices(data: &[u8], separator: u8) -> Result<HeaderIndices, DecodeError> {
    // Branchless decoding. It feels weird if you're not used to it. Note
    // that we only care about the first two fields, after which we jump
    // right to `BodyLength` and `CheckSum` verification. In this specific
    // context, everything in the middle is considered part of the body
    // (even though e.g. `MsgType` is the third field and is part of
    // `StandardHeader`): we simply don't care about that distinction. The
    // only fields that matter here are `BeginString`, `BodyLength`, and
    // `CheckSum`.
    let mut header_indices = HeaderIndices::empty();
    let mut field_i = 0;
    let mut i = 0;
    while field_i < 2 && i < data.len() {
        let byte = data[i];
        let is_equal_sign = byte == b'=';
        let is_separator = byte == separator;
        header_indices.indexes_of_equal_sign[field_i] =
            [header_indices.indexes_of_equal_sign[field_i], i][is_equal_sign as usize];
        header_indices.indexes_of_separator[field_i] =
            [header_indices.indexes_of_separator[field_i], i][is_separator as usize];
        i += 1;
        field_i += is_separator as usize;
        // We should reset the value in case it's the equal sign.
        header_indices.body_length = [
            (header_indices.body_length * 10 + byte.wrapping_sub(b'0') as usize)
                * !is_equal_sign as usize,
            header_indices.body_length,
        ][is_separator as usize];
    }
    if !header_indices.is_valid() {
        Err(DecodeError::Syntax)
    } else {
        debug_assert!(header_indices.indexes_of_separator[1] < data.len());
        Ok(header_indices)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_decoder() -> RawDecoder {
        RawDecoder::with_config(Config::default().with_separator(b'|'))
    }

    #[test]
    fn decode_simple_message() {
        let decoder = new_decoder();
        let msg = b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"FIX.4.2");
        assert_eq!(frame.payload(), b"35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|");
    }

    #[test]
    fn decode_without_checksum_verification() {
        let decoder = new_decoder();
        let msg = b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=000|";
        assert!(decoder.decode(msg).is_err());
    }

    #[test]
    fn decode_empty_body() {
        let decoder = new_decoder();
        let msg = b"8=FIX.FOOBAR|9=0|10=225|";
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"FIX.FOOBAR");
        assert_eq!(frame.payload(), b"");
    }

    #[test]
    fn edge_cases_dont_cause_panic() {
        let decoder = new_decoder();
        assert!(decoder.decode(b"8=|9=0|10=225|").is_err());
        assert!(decoder.decode(b"8=|9=0|10=|").is_err());
        assert!(decoder.decode(b"8====|9=0|10=|").is_err());
        assert!(decoder.decode(b"|||9=0|10=|").is_err());
        assert!(decoder.decode(b"9999999999999").is_err());
        assert!(decoder.decode(b"-9999999999999").is_err());
        assert!(decoder.decode(b"==============").is_err());
        assert!(decoder.decode(b"9999999999999|").is_err());
        assert!(decoder.decode(b"|999999999999=|").is_err());
        assert!(decoder.decode(b"|999=999999999999999999|=").is_err());
    }
}
