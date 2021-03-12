use crate::tagvalue::{TagLookup, TagLookupSingleAppVersion};

const SOH: u8 = 0x1;

/// Collection of configuration options related to FIX encoding and decoding.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`.
pub trait Configure: Clone + Default {
    /// The [`TagLookup`] implementor that will be used during decoding.
    type TagLookup: TagLookup;

    /// The delimiter character, which terminates every tag-value pair including
    /// the last one.
    ///
    /// ASCII 0x1 (SOH) is the default separator character.
    fn separator(&self) -> u8 {
        SOH
    }

    /// The maximum allowed size for any single FIX message. No restrictions are
    /// imposed when it is `None`.
    fn max_message_size(&self) -> Option<usize> {
        Some(65536)
    }

    #[deprecated(note = "BodyLength is mandatory. This method is ignored.")]
    fn verify_body_length(&self) -> bool {
        true
    }

    /// Determines wheather or not `CheckSum(10)` should be verified.
    fn verify_checksum(&self) -> bool {
        true
    }
}

/// The canonical implementor of [`Configure`]. Every setting can be changed.
#[derive(Debug, Copy, Clone)]
pub struct Config {
    separator: u8,
    verify_checksum: bool,
}

impl Config {
    /// Changes the field separator character. It is SOH (ASCII 0x1) by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::Config;
    ///
    /// let config = &mut Config::default();
    /// assert_eq!(config.separator(), 0x1);
    /// config.set_separator(b'|');
    /// assert_eq!(config.separator(), b'|');
    /// ```
    pub fn set_separator(&mut self, separator: u8) {
        self.separator = separator;
    }

    pub fn with_separator(mut self, separator: u8) -> Self {
        self.separator = separator;
        self
    }

    /// Turns on or off `ChekSum(10)` verification. On by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::Config;
    ///
    /// let config = &mut Config::default();
    /// assert_eq!(config.verify_checksum(), true);
    /// config.set_verify_checksum(false);
    /// assert_eq!(config.verify_checksum(), false);
    /// ```
    pub fn set_verify_checksum(&mut self, verify: bool) {
        self.verify_checksum = verify;
    }
}

impl Configure for Config {
    type TagLookup = TagLookupSingleAppVersion;

    fn separator(&self) -> u8 {
        self.separator
    }

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