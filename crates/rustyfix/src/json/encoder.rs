//! FIX JSON encoding.

use super::Config;
use crate::Dictionary;
use crate::GetConfig;
use std::fmt::Debug;
use std::sync::Arc;

// TODO: `serde_json` is not a very high-performance library.
/// JSON encoder for FIX messages.
///
/// This encoder converts FIX messages to JSON format using the provided dictionary
/// for message and field definitions.
pub struct Encoder {
    #[allow(dead_code)] // TODO: Will be used for encoding implementation
    dictionary: Arc<Dictionary>,
    config: Config,
}

impl Debug for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Encoder")
            .field("config", &self.config)
            .finish()
    }
}

impl Encoder {
    /// Creates a new JSON encoder with the provided dictionary.
    ///
    /// # Arguments
    /// * `dict` - The FIX dictionary containing message and field definitions
    ///
    /// # Returns
    /// A new `Encoder` instance with default configuration
    pub fn new(dict: Dictionary) -> Self {
        Self {
            dictionary: Arc::new(dict),
            config: Config::default(),
        }
    }
}

impl GetConfig for Encoder {
    type Config = Config;

    fn config(&self) -> &Self::Config {
        &self.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        &mut self.config
    }
}
