use crate::tagvalue::{utils, Config, Configure, DecodeError};
use crate::{Buffer, GetConfig, StreamingDecoder};
use std::ops::Range;

/// An immutable view over the contents of a FIX message by a [`RawDecoder`].
#[derive(Debug)]
pub struct RawFrame<T> {
    /// Raw, untouched contents of the message. Includes everything from `BeginString <8>` up to
    /// `CheckSum <8>`.
    pub data: T,
    /// The range of bytes that address the value of `BeginString <8>`.
    pub begin_string: Range<usize>,
    /// The range of bytes that address all contents after `MsgType <35>` and before `CheckSum
    /// <10>`.
    pub payload: Range<usize>,
}

impl<T> RawFrame<T>
where
    T: AsRef<[u8]>,
{
    /// Returns an immutable reference to the raw contents of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, RawDecoder};
    /// use fefix::prelude::*;
    ///
    /// let mut decoder = RawDecoder::<Config>::new();
    /// decoder.config_mut().set_separator(b'|');
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let message = decoder.decode(data).unwrap();
    ///
    /// assert_eq!(message.as_bytes(), data);
    /// ```
    pub fn as_bytes(&self) -> &[u8] {
        self.data.as_ref()
    }

    /// Returns an immutable reference to the `BeginString <8>` field value of
    /// `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, RawDecoder};
    /// use fefix::prelude::*;
    ///
    /// let mut decoder = RawDecoder::<Config>::new();
    /// decoder.config_mut().set_separator(b'|');
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let message = decoder.decode(data).unwrap();
    ///
    /// assert_eq!(message.begin_string(), b"FIX.4.2");
    /// ```
    pub fn begin_string(&self) -> &[u8] {
        &self.as_bytes()[self.begin_string.clone()]
    }

    /// Returns an immutable reference to the payload of `self`. In this
    /// context, "payload" means all fields besides
    ///
    /// - `BeginString <8>`;
    /// - `BodyLength <9>`;
    /// - `CheckSum <10>`.
    ///
    /// According to this definition, the payload may also contain fields that are
    /// technically part of `StandardHeader` and `StandardTrailer`, i.e. payload
    /// and body and *not* synonyms.
    ///
    /// ```
    /// use fefix::tagvalue::{Config, RawDecoder};
    /// use fefix::prelude::*;
    ///
    /// let mut decoder = RawDecoder::<Config>::new();
    /// decoder.config_mut().set_separator(b'|');
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let message = decoder.decode(data).unwrap();
    ///
    /// assert_eq!(message.payload().len(), 42);
    /// ```
    pub fn payload(&self) -> &[u8] {
        &self.as_bytes()[self.payload.clone()]
    }
}

/// A bare-bones FIX decoder for low-level message handling.
///
/// [`RawDecoder`] is the fundamental building block for building higher-level
/// FIX decoder. It allows for decoding of arbitrary payloads and only "hides"
/// `BodyLength (9)` and `CheckSum (10)` to the final user. Everything else is
/// left to the user to deal with.
#[derive(Debug, Clone, Default)]
pub struct RawDecoder<C = Config> {
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

    /// Adds a [`Buffer`] to `self`, turning it into a [`RawDecoderStreaming`].
    pub fn streaming<B>(self, buffer: B) -> RawDecoderStreaming<B, C>
    where
        B: Buffer,
    {
        RawDecoderStreaming {
            config: self.config,
            buffer,
            state: ParserState::Empty,
            bytes_read: 0,
        }
    }

    /// Does minimal parsing on `data` and returns a [`RawFrame`] if it's valid.
    pub fn decode<T>(&self, src: T) -> Result<RawFrame<T>, DecodeError>
    where
        T: AsRef<[u8]>,
    {
        let data = src.as_ref();
        let len = data.len();
        if len < utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            return Err(DecodeError::Invalid);
        }

        let header_info =
            HeaderInfo::parse(data, self.config().separator()).ok_or(DecodeError::Invalid)?;

        utils::verify_body_length(
            data,
            header_info.field_1.end + 1,
            header_info.nominal_body_len as usize,
        )?;

        if self.config.verify_checksum() {
            utils::verify_checksum(data)?;
        }

        Ok(RawFrame {
            data: src,
            begin_string: header_info.field_0,
            payload: header_info.field_1.end + 1..len - utils::FIELD_CHECKSUM_LEN_IN_BYTES,
        })
    }
}

impl<C> GetConfig for RawDecoder<C> {
    type Config = C;

    fn config(&self) -> &C {
        &self.config
    }

    fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }
}

#[derive(Debug)]
enum ParserState {
    Empty,
    Header(HeaderInfo, usize),
    Failed,
}

/// A [`RawDecoder`] that can buffer incoming data and read a stream of messages.
#[derive(Debug)]
pub struct RawDecoderStreaming<B, C = Config> {
    buffer: B,
    config: C,
    state: ParserState,
    bytes_read: usize,
}

impl<B, C> StreamingDecoder for RawDecoderStreaming<B, C>
where
    B: Buffer,
    C: Configure,
{
    type Buffer = B;
    type Error = DecodeError;

    fn num_bytes_read(&self) -> usize {
        self.bytes_read
    }

    fn buffer(&mut self) -> &mut B {
        &mut self.buffer
    }

    fn clear(&mut self) {
        self.buffer().clear();
        self.state = ParserState::Empty;
        self.bytes_read = 0;
    }

    fn num_bytes_required(&self) -> usize {
        match self.state {
            ParserState::Empty => utils::MIN_FIX_MESSAGE_LEN_IN_BYTES,
            ParserState::Header(_, expected_len) => expected_len,
            ParserState::Failed => 0,
        }
    }

    fn try_parse(&mut self) -> Result<Option<()>, Self::Error> {
        match self.state {
            ParserState::Empty => {
                let header_info =
                    HeaderInfo::parse(self.buffer.as_slice(), self.config().separator());
                if let Some(header_info) = header_info {
                    let expected_len_of_frame = header_info.field_1.end
                        + 1
                        + header_info.nominal_body_len
                        + utils::FIELD_CHECKSUM_LEN_IN_BYTES;

                    self.state = ParserState::Header(header_info, expected_len_of_frame);
                    Ok(None)
                } else {
                    Err(DecodeError::Invalid)
                }
            }
            ParserState::Header(_, _) => Ok(Some(())),
            ParserState::Failed => panic!("Failed state"),
        }
    }
}

impl<B, C> RawDecoderStreaming<B, C>
where
    B: Buffer,
    C: Configure,
{
    /// Tries to deserialize the next [`RawFrame`] from the internal buffer. If
    /// the internal buffer does not contain a complete message, returns an
    /// [`Ok(None)`].
    pub fn raw_frame(&self) -> RawFrame<&[u8]> {
        if let ParserState::Header(header_info, _len) = &self.state {
            let data = &self.buffer.as_slice();

            RawFrame {
                data,
                begin_string: header_info.field_0.clone(),
                payload: header_info.field_1.end + 1
                    ..data.len() - utils::FIELD_CHECKSUM_LEN_IN_BYTES,
            }
        } else {
            panic!("The message is not fully decoded. Check `try_parse` return value.");
        }
    }

    pub fn add_bytes_read(&mut self, bytes_read: usize) {
        self.bytes_read += bytes_read;
    }
}

impl<B, C> GetConfig for RawDecoderStreaming<B, C> {
    type Config = C;

    fn config(&self) -> &C {
        &self.config
    }

    fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }
}

#[derive(Debug, Clone)]
struct HeaderInfo {
    field_0: Range<usize>,
    field_1: Range<usize>,
    nominal_body_len: usize,
}

impl HeaderInfo {
    fn parse(data: &[u8], separator: u8) -> Option<Self> {
        let mut info = Self {
            field_0: 0..1,
            field_1: 0..1,
            nominal_body_len: 0,
        };

        let mut iterator = data.iter();
        let mut find_byte = |byte| iterator.position(|b| *b == byte);
        let mut i = 0;

        i += find_byte(b'=')? + 1;
        info.field_0.start = i;
        i += find_byte(separator)?;
        info.field_0.end = i;
        i += 1;

        i += find_byte(b'=')? + 1;
        info.field_1.start = i;
        i += find_byte(separator)?;
        info.field_1.end = i;

        for byte in &data[info.field_1.clone()] {
            info.nominal_body_len = info
                .nominal_body_len
                .wrapping_mul(10)
                .wrapping_add(byte.wrapping_sub(b'0') as usize);
        }

        Some(info)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_decoder() -> RawDecoder {
        let mut config = Config::default();
        config.set_separator(b'|');

        let mut decoder = RawDecoder::new();
        *decoder.config_mut() = config;
        decoder
    }

    #[test]
    fn empty_message_is_invalid() {
        let decoder = new_decoder();
        assert!(matches!(
            decoder.decode(&[] as &[u8]),
            Err(DecodeError::Invalid)
        ));
    }

    #[test]
    fn sample_message_is_valid() {
        let decoder = new_decoder();
        let msg = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|".as_bytes();
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"FIX.4.2");
        assert_eq!(frame.payload(), b"35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|");
    }

    #[test]
    fn message_with_only_msg_type_tag_is_valid() {
        let decoder = new_decoder();
        let msg = "8=??????|9=5|35=?|10=183|".as_bytes();
        let frame = decoder.decode(msg).unwrap();
        assert_eq!(frame.begin_string(), b"??????");
        assert_eq!(frame.payload(), b"35=?|");
    }

    #[test]
    fn message_with_empty_payload_is_invalid() {
        let decoder = new_decoder();
        let msg = "8=?|9=5|10=082|".as_bytes();
        assert!(matches!(decoder.decode(msg), Err(DecodeError::Invalid)));
    }

    #[test]
    fn message_with_bad_checksum_is_invalid() {
        let mut decoder = new_decoder();
        decoder.config_mut().set_verify_checksum(true);
        let msg = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=000|".as_bytes();
        assert!(matches!(decoder.decode(msg), Err(DecodeError::CheckSum)));
    }

    #[test]
    fn edge_cases_dont_cause_panic() {
        let decoder = new_decoder();
        decoder.decode("8=|9=0|10=225|".as_bytes()).ok();
        decoder.decode("8=|9=0|10=|".as_bytes()).ok();
        decoder.decode("8====|9=0|10=|".as_bytes()).ok();
        decoder.decode("|||9=0|10=|".as_bytes()).ok();
        decoder.decode("9999999999999".as_bytes()).ok();
        decoder.decode("-9999999999999".as_bytes()).ok();
        decoder.decode("==============".as_bytes()).ok();
        decoder.decode("9999999999999|".as_bytes()).ok();
        decoder.decode("|999999999999=|".as_bytes()).ok();
        decoder.decode("|999=999999999999999999|=".as_bytes()).ok();
    }

    #[test]
    fn new_streaming_decoder_has_no_current_frame() {
        let decoder = new_decoder().streaming(vec![]);
        assert!(decoder.num_bytes_required() > 0);
    }

    #[test]
    fn new_streaming_decoder() {
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
        let mut decoder = new_decoder().streaming(vec![]);
        let mut ready = false;
        while !ready || i >= stream.len() {
            let buf = decoder.fillable();
            let buf_len = buf.len();
            buf.clone_from_slice(&stream[i..i + buf.len()]);
            i += buf_len;
            ready = decoder.try_parse().unwrap().is_some();
            decoder.add_bytes_read(buf_len);
        }
        assert_eq!(decoder.raw_frame().begin_string(), b"FIX.4.2");
        assert_eq!(
            decoder.raw_frame().payload(),
            b"35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|"
        );
    }
}
