//! FIX JSON encoding.

use super::Config;
use crate::{GetConfig, SetField};
use std::fmt::Debug;

// TODO: `serde_json` is not a very high-performance library.
pub struct Encoder {
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
