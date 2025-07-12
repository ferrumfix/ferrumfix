use super::{Config, EncodeError};
use crate::GetConfig;
use rustyfix_dictionary::Dictionary;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Encoder {
    dictionary: Arc<Dictionary>,
    config: Config,
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
