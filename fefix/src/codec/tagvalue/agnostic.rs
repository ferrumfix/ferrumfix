use crate::buffering::Buffer;
use crate::codec::Encoding;
use crate::codec::tagvalue::{Config, DecodeError, utils};
use crate::dbglog;

/// An immutable view over the raw contents of a FIX message.
///
/// See [`AgnosticCodec`] for more information.
#[derive(Debug)]
pub struct AgnosticMessage {
    begin_string: (*const u8, usize),
    body: (*const u8, usize),
}

impl AgnosticMessage {
    /// Creates an [`AgnosticMessage`] that has an empty `BeginString <8>` and empty
    /// body.
    fn empty() -> Self {
        Self {
            begin_string: ([].as_ptr(), 0),
            body: ([].as_ptr(), 0),
        }
    }

    /// Returns an immutable reference to the `BeginString <8>` field value of
    /// `self`.
    ///
    /// ```
    /// use fefix::codec::Encoding;
    /// use fefix::codec::tagvalue::{CodecAgnostic, Configurable};
    ///
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let codec = &mut CodecAgnostic::<Configurable>::default();
    /// let message = codec.decode(data).unwrap();
    ///
    /// assert_eq!(message.begin_string(), b"FIX.4.2");
    /// ```
    pub fn begin_string(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.begin_string.0, self.begin_string.1) }
    }

    /// Returns an immutable reference to the body contents of `self`. In this context, "body" means all fields besides
    ///
    /// - `BeginString <8>`;
    /// - `BodyLength <9>`;
    /// - `CheckSum <10>`.
    ///
    /// According to this definition, the body may also contain fields that are
    /// technically part of `StandardHeader` and `StandardTrailer`.
    ///
    /// ```
    /// use fefix::codec::Encoding;
    /// use fefix::codec::tagvalue::{CodecAgnostic, Configurable};
    ///
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let codec = &mut CodecAgnostic::<Configurable>::default();
    /// let message = codec.decode(data).unwrap();
    ///
    /// assert_eq!(message.body().len(), 42);
    /// ```
    pub fn body(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.body.0, self.body.1) }
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
#[derive(Debug)]
pub struct CodecAgnostic<Z>
where
    Z: Config,
{
    message: AgnosticMessage,
    config: Z,
}

impl<Z> CodecAgnostic<Z>
where
    Z: Config,
{
    /// Returns an immutable reference to the [`Config`] used by `self`.
    ///
    /// ```
    /// use fefix::codec::tagvalue::CodecAgnostic;
    ///
    /// let mut codec = CodecAgnostic::default();
    /// assert_eq!(codec.config().separator(), 0x1); // SOH
    /// ```
    pub fn config(&self) -> &Z {
        &self.config
    }

    /// Returns a mutable reference to the [`Config`] used by `self`.
    ///
    /// ```
    /// use fefix::codec::tagvalue::CodecAgnostic;
    ///
    /// let mut codec = CodecAgnostic::default();
    /// *codec.config_mut() = Configurable::default().with_separator(b'|');
    /// assert_eq!(codec.config().separator(), b'|');
    /// ```
    pub fn config_mut(&mut self) -> &mut Z {
        &mut self.config
    }
}

impl<Z> Default for CodecAgnostic<Z>
where
    Z: Config,
{
    fn default() -> Self {
        Self {
            message: AgnosticMessage::empty(),
            config: Z::default(),
        }
    }
}

impl<Z> Encoding<AgnosticMessage> for CodecAgnostic<Z>
where
    Z: Config,
{
    type DecodeError = DecodeError;
    type EncodeError = ();

    fn decode(&mut self, data: &[u8]) -> Result<&AgnosticMessage, Self::DecodeError> {
        if data.len() < utils::MIN_FIX_MESSAGE_LEN_IN_BYTES {
            dbglog!("The input data is too short to contain a well-defined FIX message.");
            return Err(Self::DecodeError::Syntax);
        }
        dbglog!(
            "Content-agnostic decoding: UTF-8 lossy is '{}'.",
            String::from_utf8_lossy(data)
        );
        let header_indices = utils::parse_header_indices(data, self.config().separator())?;
        utils::verify_body_length(
            data,
            header_indices.start_of_body(),
            header_indices.body().len(),
        )?;
        if self.config.verify_checksum() {
            utils::verify_checksum(data)?;
        } else {
            dbglog!("Skipping checksum verification.");
        }
        self.message.begin_string = {
            let range = &data[header_indices.begin_string_value()];
            (range.as_ptr(), range.len())
        };
        self.message.body = {
            let range = &data[header_indices.body()];
            (range.as_ptr(), range.len())
        };
        Ok(&self.message)
    }

    fn encode<B>(
        &mut self,
        mut buffer: &mut B,
        message: &AgnosticMessage,
    ) -> Result<usize, Self::EncodeError>
    where
        B: Buffer,
    {
        let body_writer = |buffer: &mut B| {
            buffer.extend_from_slice(message.body());
            message.body().len()
        };
        utils::encode(
            &self.config,
            message.begin_string(),
            body_writer,
            &mut buffer,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::codec::tagvalue::Configurable;

    fn config_vertical_bar() -> Configurable {
        Configurable::default().with_separator(b'|')
    }

    #[test]
    fn agnostic_simple_message() {
        let msg = "8=FIX.4.2|9=40|35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|10=091|";
        let mut decoder = CodecAgnostic::default();
        *decoder.config_mut() = config_vertical_bar();
        let message = decoder.decode(&mut msg.as_bytes()).unwrap();
        assert_eq!(message.begin_string(), b"FIX.4.2");
        assert_eq!(message.body(), b"35=D|49=AFUNDMGR|56=ABROKER|15=USD|59=0|");
    }

    #[test]
    fn empty_body() {
        let msg = "8=FIX.FOOBAR|9=0|10=225|";
        let codec = &mut CodecAgnostic::default();
        *codec.config_mut() = config_vertical_bar();
        let message = codec.decode(&mut msg.as_bytes()).unwrap();
        assert_eq!(message.begin_string(), b"FIX.FOOBAR");
        assert_eq!(message.body(), b"");
    }

    #[test]
    fn edge_cases_dont_cause_panic() {
        let mut decoder = CodecAgnostic::default();
        *decoder.config_mut() = config_vertical_bar();
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