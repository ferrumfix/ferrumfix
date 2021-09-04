use std::time::Duration;

/// Collection of configuration options related to FIX sessions.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`.
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

impl Config {
    /// Changes the value of [`Configure::verify_test_indicator`].
    pub fn set_verify_test_indicator(&mut self, verify: bool) {
        self.verify_test_indicator = verify;
    }
}

impl Configure for Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            verify_test_indicator: true,
            max_allowed_latency: Duration::from_secs(3),
        }
    }
}

#[cfg(test)]
mod test {}
