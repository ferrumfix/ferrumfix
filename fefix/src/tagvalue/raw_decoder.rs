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
    /// Creates a new [`RawDecoder`] with default configuration options.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn buffered<B>(self, buffer: B) -> RawDecoderBuffered<B, C>
    where
        B: Buffer,
    {
        assert_eq!(buffer.as_slice().len(), 0);
        RawDecoderBuffered {
            buffer,
            decoder: self,
            error: None,
        }
    }

    /// Creates a new [`RawDecoder`] with `config` as a [`Configure`]
    /// implementor.
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

    /// Does minimal parsing on `data` and returns a [`RawFrame`] if it's valid.
    pub fn decode<'a>(&self, data: &'a [u8]) -> Result<RawFrame<'a>, DecodeError> {
        if data.len() < utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            return Err(DecodeError::Invalid);
        }
        let info = HeaderInfo::parse(data, self.config().separator())?;
        utils::verify_body_length(data, info.start_of_body(), info.body_range().len())?;
        if self.config().verify_checksum() {
            utils::verify_checksum(data)?;
        }
        let begin_string = &data[info.begin_string_range()];
        let contents = &data[info.body_range()];
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
    decoder: RawDecoder<C>,
    error: Option<DecodeError>,
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
        if self.buffer.as_slice().len() < utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            let current_len = self.buffer.as_slice().len();
            let missing_len = utils::MIN_FIX_MESSAGE_LEN_IN_BYTES - current_len;
            debug_assert!(missing_len > 0);
            self.buffer.resize(missing_len, 0);
            &mut self.buffer.as_mut_slice()[current_len..]
        } else {
            match HeaderInfo::parse(self.buffer.as_slice(), self.config().separator()) {
                Ok(info) => {
                    let start_of_body = info.start_of_body();
                    let body_len = info.body_range().len();
                    let total_len = start_of_body + body_len + utils::FIELD_CHECKSUM_LEN_IN_BYTES;
                    let current_len = self.buffer.as_slice().len();
                    self.buffer.resize(total_len, 0);
                    &mut self.buffer.as_mut_slice()[current_len..]
                }
                Err(e) => {
                    self.error = Some(e);
                    &mut []
                }
            }
        }
    }

    pub fn current_frame(&self) -> Result<Option<RawFrame>, DecodeError> {
        if let Some(err) = self.error.clone() {
            Err(err)
        } else {
            let data = &self.buffer.as_slice();
            if data.len() == 0 || data.len() == utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
                Ok(None)
            } else {
                self.decoder.decode(data).map(|message| Some(message))
            }
        }
    }
}

// Information regarding the indices of "important" parts of the FIX message.
struct HeaderInfo {
    i_equal_sign: [usize; 2],
    i_sep: [usize; 2],
    body_length: usize,
}

impl HeaderInfo {
    fn empty() -> Self {
        Self {
            i_equal_sign: [0, 0],
            i_sep: [0, 0],
            body_length: 0,
        }
    }

    pub fn start_of_body(&self) -> usize {
        // The body starts at the character immediately after the separator of
        // `BodyLength`.
        self.i_sep[1] + 1
    }

    pub fn begin_string_range(&self) -> Range<usize> {
        self.i_equal_sign[0] + 1..self.i_sep[0]
    }

    pub fn body_range(&self) -> Range<usize> {
        let start = self.start_of_body();
        start..start + self.body_length
    }

    fn parse(data: &[u8], separator: u8) -> Result<Self, DecodeError> {
        let mut info = HeaderInfo::empty();
        let mut field_i = 0;
        let mut i = 0;
        while field_i < 2 && i < data.len() {
            let byte = data[i];
            if byte == b'=' {
                info.i_equal_sign[field_i] = i;
                info.body_length = 0;
            } else if byte == separator {
                info.i_sep[field_i] = i;
                field_i += 1;
            } else {
                info.body_length = info.body_length * 10 + byte.wrapping_sub(b'0') as usize;
            }
            i += 1;
        }
        // Let's check that we got valid values for everything we need.
        if info.i_equal_sign[0] != 0
            && info.i_equal_sign[1] != 0
            && info.i_sep[0] != 0
            && info.i_sep[1] != 0
        {
            debug_assert!(info.i_sep[1] < data.len());
            Ok(info)
        } else {
            Err(DecodeError::Invalid)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_decoder() -> RawDecoder {
        let config = Config::default().with_separator(b'|');
        RawDecoder::with_config(config)
    }

    #[test]
    fn empty_message_is_invalid() {
        let decoder = new_decoder();
        assert!(matches!(decoder.decode(&[]), Err(DecodeError::Invalid)));
    }

    #[test]
    fn sample_message_is_valid() {
        let decoder = new_decoder();
        let msg = b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"FIX.4.2");
        assert_eq!(frame.payload(), b"35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|");
    }

    #[test]
    fn message_with_only_msg_type_tag_is_valid() {
        let decoder = new_decoder();
        let msg = b"8=?|9=5|35=?|10=183|";
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"?");
        assert_eq!(frame.payload(), b"35=?|");
    }

    #[test]
    fn message_with_empty_payload_is_invalid() {
        let decoder = new_decoder();
        let msg = b"8=?|9=5|10=082|";
        assert!(matches!(decoder.decode(msg), Err(DecodeError::Invalid)));
    }

    #[test]
    fn message_with_bad_checksum_is_invalid() {
        let decoder = new_decoder();
        let msg = b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=000|";
        assert!(matches!(decoder.decode(msg), Err(DecodeError::CheckSum)));
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

    fn new_decoder_buffered() -> RawDecoderBuffered {
        let config = Config::default().with_separator(b'|');
        RawDecoder::with_config(config).buffered(Vec::new())
    }

    #[test]
    #[should_panic]
    fn cannot_create_raw_decoder_buffered_with_nonempty_buffer() {
        let decoder = RawDecoder::with_config(Config::default());
        decoder.buffered(vec![0]);
    }

    #[test]
    fn new_buffered_decoder_has_no_current_frame() {
        let decoder = new_decoder_buffered();
        assert!(decoder.current_frame().unwrap().is_none());
    }

    #[test]
    fn new_buffered_decoder() {
        let stream = {
            let mut stream = Vec::new();
            for _ in 0..42 {
                stream.extend_from_slice(
                    b"8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|",
                );
            }
            stream
        };
        let mut i = 0;
        let decoder = &mut new_decoder_buffered();
        let mut frame = None;
        while frame.is_none() || i >= stream.len() {
            let buf = decoder.supply_buffer();
            buf.clone_from_slice(&stream[i..i + buf.len()]);
            i = buf.len();
            frame = decoder.current_frame().unwrap();
        }
        assert!(frame.is_some());
    }
}
