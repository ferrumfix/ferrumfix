const SOH: u8 = 0x1;
const DEFAULT_MAX_MESSAGE_SIZE: usize = 0xffff;

/// Configuration options for [`Encoder`](super::Encoder) and
/// [`Decoder`](super::Decoder).
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub struct Config {
    /// The delimiter character, which terminates every tag-value pair including
    /// the last one. This setting is relevant for both encoding and decoding
    /// operations.
    ///
    /// ASCII 0x1 (SOH) is the default separator character.
    pub separator: u8,
    /// The maximum allowed size for any single FIX message. No restrictions are
    /// imposed when it is [`None`].
    pub max_message_size: Option<usize>,
    /// Determines whether or not `CheckSum(10)` should be verified.
    ///
    /// This setting has no effect when encoding FIX messages.
    pub verify_checksum: bool,
    /// Determines whether or not the decoder needs to have access to
    /// associative FIX fields. If turned off, only linear access is possible.
    pub should_decode_associative: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            separator: SOH,
            max_message_size: Some(DEFAULT_MAX_MESSAGE_SIZE),
            verify_checksum: true,
            should_decode_associative: true,
        }
    }
}
