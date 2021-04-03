use super::{MessageAccumulator, SerializeField};
use crate::buffer::Buffer;
use crate::tagvalue::{field_value::TagNum, utils, Config, Configure, EncodeError, FixFieldValue};
use crate::{AppVersion, Dictionary, FixFieldsIter, FixMessage};
use std::fmt::Debug;

/// FIX message encoder and decoder.
#[derive(Debug, Clone)]
pub struct Encoder<C = Config>
where
    C: Configure,
{
    dict: Dictionary,
    config: C,
}

impl<C> Encoder<C>
where
    C: Configure,
{
    /// Builds a new [`Encoder`] encoding device with a FIX 4.4 dictionary.
    pub fn new(config: C) -> Self {
        Self::with_dict(Dictionary::from_version(AppVersion::Fix44), config)
    }

    /// Creates a new codec for the tag-value format. `dict` is used to parse
    /// messages.
    pub fn with_dict(dict: Dictionary, config: C) -> Self {
        Self { dict, config }
    }

    /// Returns an immutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Encoder};
    ///
    /// let encoder = &mut Encoder::new(Config::default());
    /// assert_eq!(encoder.config().separator(), 0x1);
    /// ```
    pub fn config(&self) -> &C {
        &self.config
    }

    /// Returns a mutable reference to the [`Configure`] used by `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure, Encoder};
    ///
    /// let encoder = &mut Encoder::new(Config::default());
    /// encoder.config_mut().set_separator(b'|');
    /// assert_eq!(encoder.config().separator(), b'|');
    /// ```
    pub fn config_mut(&mut self) -> &mut C {
        &mut self.config
    }

    pub fn encode<B>(&mut self, buffer: &mut B, message: &FixMessage) -> Result<usize, EncodeError>
    where
        B: Buffer,
    {
        let body_writer = |buffer: &mut B| {
            let start_i = buffer.as_slice().len();
            // Skips `BeginString`.
            for (tag, value) in message.iter_fields().skip(1) {
                encode_field(
                    TagNum::from(tag as u16),
                    value,
                    buffer,
                    self.config.separator(),
                );
            }
            buffer.as_slice().len() - start_i
        };
        let begin_string = message
            .iter_fields_in_std_header()
            .next()
            .unwrap()
            .1
            .as_str()
            .unwrap()
            .as_bytes();
        utils::encode_raw(begin_string, body_writer, buffer, self.config.separator())
    }
}

fn encode_field(tag: TagNum, value: &FixFieldValue, write: &mut impl Buffer, separator: u8) {
    write.extend_from_slice(tag.to_string().as_bytes());
    write.extend_from_slice(&[b'=']);
    match &value {
        FixFieldValue::Group(_) => panic!("Can't encode a group!"),
        FixFieldValue::Atom(field) => write.extend_from_slice(field.to_string().as_bytes()),
    };
    write.extend_from_slice(&[separator]);
}

/// FIX message encoder and decoder.
#[derive(Debug, Clone)]
pub struct EncoderNaive<B = Vec<u8>, C = Config>
where
    B: Buffer,
    C: Configure,
{
    buffer: B,
    config: C,
}

impl<B, C> MessageAccumulator for EncoderNaive<B, C>
where
    B: Buffer,
    C: Configure,
{
    fn set_field<T>(&mut self, tag: u32, value: T)
    where
        T: SerializeField,
    {
        tag.serialize(&mut self.buffer);
        self.buffer.extend_from_slice(b"=");
        value.serialize(&mut self.buffer);
        self.buffer.extend_from_slice(&[self.config.separator()]);
    }

    fn set_begin_string(&mut self, value: &[u8]) {
        self.set_field(crate::tags::BEGIN_STRING, value)
    }

    fn wrap_std_header(&mut self) {}

    fn wrap_body(&mut self) {}

    fn wrap_std_trailer(&mut self) {}
}
