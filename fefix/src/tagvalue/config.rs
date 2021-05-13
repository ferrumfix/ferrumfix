const SOH: u8 = 0x1;
const DEFAULT_MAX_MESSAGE_SIZE: usize = 0xffff;

/// Collection of configuration options related to FIX encoding and decoding.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`.
pub trait Configure: Clone + Default {
    /// The delimiter character, which terminates every tag-value pair including
    /// the last one.
    ///
    /// ASCII 0x1 (SOH) is the default separator character.
    ///
    /// This setting is relevant for both encoding and decoding operations.
    #[inline(always)]
    fn separator(&self) -> u8 {
        SOH
    }

    /// The maximum allowed size for any single FIX message. No restrictions are
    /// imposed when it is `None`.
    #[inline(always)]
    fn max_message_size(&self) -> Option<usize> {
        Some(DEFAULT_MAX_MESSAGE_SIZE)
    }

    /// Determines wheather or not `CheckSum(10)` should be verified.
    ///
    /// This setting has no effect when encoding FIX messages.
    #[inline(always)]
    fn verify_checksum(&self) -> bool {
        true
    }

    /// Determines wheather or not the decoder needs to have access to
    /// associative FIX fields.
    #[inline(always)]
    fn should_decode_associative(&self) -> bool {
        true
    }

    /// Determines wheather or not the decoder needs to have access to
    /// sequential FIX fields.
    #[inline(always)]
    fn should_decode_sequential(&self) -> bool {
        true
    }
}

/// The canonical implementor of [`Configure`]. Every setting can be changed.
#[derive(Debug, Copy, Clone)]
pub struct Config {
    separator: u8,
    max_message_size: Option<usize>,
    verify_checksum: bool,
    should_decode_associative: bool,
    should_decode_sequential: bool,
}

impl Config {
    /// Changes the field separator character. It is SOH (ASCII 0x1) by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure};
    ///
    /// let config = &mut Config::default();
    /// assert_eq!(config.separator(), 0x1);
    /// config.set_separator(b'|');
    /// assert_eq!(config.separator(), b'|');
    /// ```
    pub fn set_separator(&mut self, separator: u8) {
        self.separator = separator;
    }

    /// Changes the value of [`Configure::max_message_size`].
    pub fn set_max_message_size(&mut self, max_message_size: Option<usize>) {
        self.max_message_size = max_message_size;
    }

    /// Turns on or off `CheckSum <10>` verification. On by default.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::tagvalue::{Config, Configure};
    ///
    /// let config = &mut Config::default();
    /// assert_eq!(config.verify_checksum(), true);
    /// config.set_verify_checksum(false);
    /// assert_eq!(config.verify_checksum(), false);
    /// ```
    pub fn set_verify_checksum(&mut self, verify: bool) {
        self.verify_checksum = verify;
    }

    /// Enables or disables random access of fields within a
    /// [`Message`](super::Message). When this setting is turned off fields can
    /// only be accessed iteratively.
    ///
    /// Enabled by default.
    pub fn set_decode_assoc(&mut self, should: bool) {
        self.should_decode_associative = should;
    }

    /// Enables or disables iterative access of fields within a
    /// [`Message`](super::Message).
    ///
    /// Enabled by default.
    pub fn set_decode_seq(&mut self, should: bool) {
        self.should_decode_sequential = should;
    }
}

impl Configure for Config {
    #[inline(always)]
    fn separator(&self) -> u8 {
        self.separator
    }

    #[inline(always)]
    fn verify_checksum(&self) -> bool {
        self.verify_checksum
    }

    #[inline(always)]
    fn max_message_size(&self) -> Option<usize> {
        self.max_message_size
    }

    #[inline(always)]
    fn should_decode_associative(&self) -> bool {
        self.should_decode_associative
    }

    #[inline(always)]
    fn should_decode_sequential(&self) -> bool {
        self.should_decode_sequential
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_message_size: Some(DEFAULT_MAX_MESSAGE_SIZE),
            separator: SOH,
            verify_checksum: true,
            should_decode_associative: true,
            should_decode_sequential: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_separator_is_soh_by_default() {
        assert_eq!(Config::default().separator(), 0x1);
    }

    #[test]
    fn config_separator_can_be_changed() {
        let config = &mut Config::default();
        config.set_separator(b'|');
        assert_eq!(config.separator(), b'|');
        config.set_separator(b'^');
        assert_eq!(config.separator(), b'^');
        config.set_separator(0x1);
        assert_eq!(config.separator(), 0x1);
    }

    #[test]
    fn config_verifies_checksum_by_default() {
        assert_eq!(Config::default().verify_checksum(), true);
    }

    #[test]
    fn config_checksum_verification_can_be_changed() {
        let config = &mut Config::default();
        config.set_verify_checksum(false);
        assert_eq!(config.verify_checksum(), false);
        config.set_verify_checksum(true);
        assert_eq!(config.verify_checksum(), true);
    }
}
