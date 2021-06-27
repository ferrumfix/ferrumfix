const SOH: u8 = 0x1;
const DEFAULT_MAX_MESSAGE_SIZE: usize = 0xffff;

/// A provider of configuration options related to FIX encoding and decoding.
///
/// # Implementing this trait
///
/// Before implementing this trait, you should look into [`Config`], which is
/// adequate for most uses. The only benefit of writing your own [`Configure`]
/// implementor rather than using [`Config`] is the possibility of relying on
/// constants in code rather than accessing `struct` members, which results in
/// better inlining by LLVM. E.g.
///
/// ```
/// use fefix::tagvalue::Configure;
///
/// #[derive(Default, Copy, Clone)]
/// struct FixInlineConfig {}
///
/// impl Configure for FixInlineConfig {
///     #[inline]
///     fn max_message_size(&self) -> Option<usize> {
///         None
///     }
///
///     #[inline]
///     fn verify_checksum(&self) -> bool {
///         true
///     }
/// }
/// ```
///
/// Needless to say, **think twice before polluting your codebase with such
/// micro-optimizations.
pub trait Configure: Clone + Default {
    /// The delimiter character, which terminates every tag-value pair including
    /// the last one.
    ///
    /// ASCII 0x1 (SOH) is the default separator character.
    ///
    /// This setting is relevant for both encoding and decoding operations.
    #[inline]
    fn separator(&self) -> u8 {
        SOH
    }

    /// The maximum allowed size for any single FIX message. No restrictions are
    /// imposed when it is `None`.
    #[inline]
    fn max_message_size(&self) -> Option<usize> {
        Some(DEFAULT_MAX_MESSAGE_SIZE)
    }

    /// Determines wheather or not `CheckSum(10)` should be verified.
    ///
    /// This setting has no effect when encoding FIX messages.
    #[inline]
    fn verify_checksum(&self) -> bool {
        true
    }

    /// Determines wheather or not the decoder needs to have access to
    /// associative FIX fields.
    #[inline]
    fn should_decode_associative(&self) -> bool {
        true
    }
}

/// A `struct` that has settable fields and implements [`Configure`].
///
/// When using [`Config`], you have full control over all FIX configuration
/// options offered by `fefix`. The documentation of [`Configure`] goes into
/// detail about the reasons why you might want to use something other than
/// [`Config`].
#[derive(Debug, Copy, Clone)]
pub struct Config {
    separator: u8,
    max_message_size: Option<usize>,
    verify_checksum: bool,
    should_decode_associative: bool,
}

impl Config {
    /// Changes the field separator character. It is SOH (ASCII 0x1) by default.
    /// This also disables checksum verification for decode operations to avoid
    /// checksum issues if not SOH.
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
        self.verify_checksum = separator == SOH;
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
}

impl Configure for Config {
    #[inline]
    fn separator(&self) -> u8 {
        self.separator
    }

    #[inline]
    fn verify_checksum(&self) -> bool {
        self.verify_checksum
    }

    #[inline]
    fn max_message_size(&self) -> Option<usize> {
        self.max_message_size
    }

    #[inline]
    fn should_decode_associative(&self) -> bool {
        self.should_decode_associative
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_message_size: Some(DEFAULT_MAX_MESSAGE_SIZE),
            separator: SOH,
            verify_checksum: true,
            should_decode_associative: true,
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
