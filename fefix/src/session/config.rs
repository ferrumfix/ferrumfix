use std::time::Duration;

/// Collection of configuration options related to FIX sessions.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`. Implementors MUST
/// have [`Default`] behavior that is consistent with the provided default
/// return values of this trait.
pub trait Configure: Clone + Default {
    /// Asks the FIX connector to verify `TestMessageIndicator <464>` fields and
    /// refuse messages if the value is invalid. `true` by default.
    fn verify_test_indicator(&self) -> bool {
        true
    }

    /// Returns the maximum allowed latency based on the sending time. Three
    /// seconds by default.
    fn max_allowed_latency(&self) -> Duration {
        Duration::from_secs(3)
    }
}

/// The canonical implementor of [`Configure`]. Every setting can be changed.
#[derive(Debug, Copy, Clone)]
pub struct Config {
    verify_test_indicator: bool,
    max_allowed_latency: Duration,
}

// The reason why we're using setters rather than making `struct` fields `pub`
// is so that we can add more configuration options without breaking backwards
// compatibility.
impl Config {
    /// Changes the value of [`Configure::verify_test_indicator`].
    pub fn set_verify_test_indicator(&mut self, verify: bool) {
        self.verify_test_indicator = verify;
    }

    /// Changes the value of [`Configure::max_allowed_latency`].
    pub fn set_max_allowed_latency(&mut self, max_allowed_latency: Duration) {
        self.max_allowed_latency = max_allowed_latency;
    }
}

impl Configure for Config {
    fn verify_test_indicator(&self) -> bool {
        self.verify_test_indicator
    }

    fn max_allowed_latency(&self) -> Duration {
        self.max_allowed_latency
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verify_test_indicator: true,
            max_allowed_latency: Duration::from_secs(3),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[derive(Default, Clone)]
    struct ConfigDefault;

    impl Configure for ConfigDefault {}

    #[test]
    fn config_defaults() {
        let config = Config::default();
        assert_eq!(
            config.max_allowed_latency(),
            ConfigDefault.max_allowed_latency()
        );
        assert_eq!(
            config.verify_test_indicator(),
            ConfigDefault.verify_test_indicator()
        );
    }

    #[quickcheck]
    fn config_set_max_allowed_latency(latency: Duration) -> bool {
        let mut config = Config::default();
        config.set_max_allowed_latency(latency);
        config.max_allowed_latency() == latency
    }

    #[quickcheck]
    fn config_set_verify_test_indicator(verify: bool) -> bool {
        let mut config = Config::default();
        config.set_verify_test_indicator(verify);
        config.verify_test_indicator() == verify
    }
}
