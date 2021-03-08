use crate::codec::tagvalue::{TagLookup, TagLookupPredetermined};

/// The [`Config`](Config) pattern allows deep customization of encoding
/// and decoding behavior without relying on runtime settings. By using this
/// trait and specializing the behavior of particular methods, users can change
/// the behavior of the FIX encoder without incurring in performance loss.
///
/// # Naming conventions
/// Implementors of this trait should start with `Config`.
pub trait Config: Clone + Default {
    type TagLookup: TagLookup;

    /// The delimiter character, which terminates every tag-value pair including
    /// the last one.
    ///
    /// ASCII 0x1 (SOH) is the default separator character.
    #[inline]
    fn separator(&self) -> u8 {
        0x1
    }

    #[inline]
    fn max_message_size(&self) -> Option<usize> {
        Some(65536)
    }

    #[inline]
    #[deprecated(note = "BodyLength is mandatory. This method is ignored.")]
    fn verify_body_length(&self) -> bool {
        true
    }

    #[inline]
    fn verify_checksum(&self) -> bool {
        true
    }
}

/// A [`Config`] for [`Codec`] with default configuration
/// options.
///
/// This configurator uses [`ChecksumAlgoDefault`] as a checksum algorithm and
/// [`TagLookupPredetermined`] for its dynamic tag lookup logic.
#[derive(Debug, Default, Clone)]
pub struct ConfigFastDefault;

impl Config for ConfigFastDefault {
    type TagLookup = TagLookupPredetermined;
}

#[derive(Debug, Clone)]
pub struct Configurable {
    separator: u8,
    verify_checksum: bool,
}

impl Configurable {
    pub fn with_separator(mut self, separator: u8) -> Self {
        self.separator = separator;
        self
    }

    pub fn with_verify_checksum(mut self, verify: bool) -> Self {
        self.verify_checksum = verify;
        self
    }
}

impl Config for Configurable {
    type TagLookup = TagLookupPredetermined;

    #[inline]
    fn separator(&self) -> u8 {
        self.separator
    }

    #[inline]
    fn verify_checksum(&self) -> bool {
        self.verify_checksum
    }
}

impl Default for Configurable {
    fn default() -> Self {
        Self {
            separator: b'|',
            verify_checksum: true,
        }
    }
}