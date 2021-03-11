use crate::tagvalue::{TagLookup, TagLookupPredetermined};

const SOH: u8 = 0x1;

/// The [`Configure`] pattern allows deep customization of encoding
/// and decoding behavior without relying on runtime settings. By using this
/// trait and specializing the behavior of particular methods, users can change
/// the behavior of the FIX encoder without incurring in performance loss.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`.
pub trait Configure: Clone + Default {
    type TagLookup: TagLookup;

    /// The delimiter character, which terminates every tag-value pair including
    /// the last one.
    ///
    /// ASCII 0x1 (SOH) is the default separator character.
    #[inline]
    fn separator(&self) -> u8 {
        SOH
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

#[derive(Debug, Copy, Clone)]
pub struct Config {
    separator: u8,
    verify_checksum: bool,
}

impl Config {
    pub fn set_separator(&mut self, separator: u8) {
        self.separator = separator;
    }

    pub fn with_separator(mut self, separator: u8) -> Self {
        self.separator = separator;
        self
    }

    pub fn set_verify_checksum(&mut self, verify: bool) {
        self.verify_checksum = verify;
    }
}

impl Configure for Config {
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

impl Default for Config {
    fn default() -> Self {
        Self {
            separator: SOH,
            verify_checksum: true,
        }
    }
}

/// A [`Configure`] for [`Codec`] with default configuration
/// options.
///
/// This configurator uses [`ChecksumAlgoDefault`] as a checksum algorithm and
/// [`TagLookupPredetermined`] for its dynamic tag lookup logic.
#[derive(Debug, Default, Clone)]
pub struct ConfigFastDefault;

impl Configure for ConfigFastDefault {
    type TagLookup = TagLookupPredetermined;
}