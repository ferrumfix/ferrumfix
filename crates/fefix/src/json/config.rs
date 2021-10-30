use std::marker::PhantomData;

/// Configuration interface for the FIX JSON encoding format.
pub trait Configure: Clone + Default {
    /// This setting indicates that all encoded messages should be "prettified"
    /// if possible, i.e. the JSON code will not be compressed and instead it
    /// will have indentation and other whitespace that favors human
    /// readability. Some performance loss and increased payload size is
    /// expected.
    ///
    /// This is turned **off** be default.
    ///
    /// This setting has no effect when decoding messages.
    ///
    /// # Output examples
    ///
    /// With "pretty print":
    ///
    /// ```json
    /// {
    ///     "Header": {
    ///         "...": "..."
    ///     },
    ///     "Body": {
    ///         "...": "..."
    ///     },
    ///     "Trailer": {
    ///         "...": "..."
    ///     }
    /// }
    /// ```
    ///
    /// Without "pretty print":
    ///
    /// ```json
    /// {"Header":{"...":"..."},"Body":{"...":"..."},"Trailer":{"...":"..."}}
    /// ```
    #[inline]
    fn pretty_print(&self) -> bool {
        false
    }
}

/// The canonical implementor of [`Configure`]. It simply stores configuration
/// options as `struct` fields.
#[derive(Debug, Clone)]
pub struct Config {
    phantom: PhantomData<()>,

    /// Enables [`Configure::pretty_print`].
    pub pretty_print: bool,
}

impl Configure for Config {
    fn pretty_print(&self) -> bool {
        self.pretty_print
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            phantom: PhantomData::default(),
            pretty_print: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_doesnt_pretty_print_by_default() {
        assert_eq!(Config::default().pretty_print(), false);
    }

    #[test]
    fn config_pretty_print_behavior_can_be_changed() {
        let mut config = Config::default();
        config.pretty_print = true;
        assert_eq!(config.pretty_print(), true);
        config.pretty_print = false;
        assert_eq!(config.pretty_print(), false);
    }
}
