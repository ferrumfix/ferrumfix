use std::time::Duration;

/// Collection of configuration options related to FIX sessions.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`.
pub trait Configure: Clone + Default {
    // Asks the FIX connector to verify `TestMessageIndicator <464>` fields and
    // refuse messages if the value is invalid.
    fn verify_test_indicator(&self) -> bool {
        true
    }

    /// Returns the maximum allowed latency based on the sending time.
    fn max_allowed_latency(&self) -> Duration {
        Duration::from_secs(3)
    }
}

/// The canonical implementor of [`Configure`]. Every setting can be changed.
#[derive(Debug, Copy, Clone)]
pub struct Config {}

impl Config {}

impl Configure for Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod test {}
