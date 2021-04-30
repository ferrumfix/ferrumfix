/// Collection of configuration options related to FIX sessions.
///
/// # Naming conventions
///
/// Implementors of this trait should start with `Config`.
pub trait Configure: Clone + Default {}

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
