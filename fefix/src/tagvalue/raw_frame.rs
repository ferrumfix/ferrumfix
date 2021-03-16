/// An immutable view over the raw contents of a FIX message.
#[derive(Debug)]
pub struct RawFrame<'a> {
    begin_string: &'a [u8],
    payload: &'a [u8],
}

impl<'a> RawFrame<'a> {
    /// Returns a [`RawFrame`] with the specified contents.
    ///
    /// ```
    /// use fefix::tagvalue::RawFrame;
    ///
    /// let frame = RawFrame::new(b"FIX.4.2", b"");
    /// assert_eq!(frame.begin_string(), b"FIX.4.2");
    /// ```
    pub fn new(begin_string: &'a [u8], payload: &'a [u8]) -> Self {
        Self {
            begin_string,
            payload,
        }
    }

    /// Returns an immutable reference to the `BeginString <8>` field value of
    /// `self`.
    ///
    /// ```
    /// use fefix::tagvalue::{Config, RawDecoder};
    ///
    /// let mut decoder = RawDecoder::<Config>::new();
    /// decoder.config_mut().set_separator(b'|');
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
    /// use fefix::tagvalue::{Config, RawDecoder};
    ///
    /// let mut decoder = RawDecoder::<Config>::new();
    /// decoder.config_mut().set_separator(b'|');
    /// let data = b"8=FIX.4.2|9=42|35=0|49=A|56=B|34=12|52=20100304-07:59:30|10=022|";
    /// let message = decoder.decode(data).unwrap();
    ///
    /// assert_eq!(message.payload().len(), 42);
    /// ```
    pub fn payload(&self) -> &[u8] {
        self.payload
    }
}