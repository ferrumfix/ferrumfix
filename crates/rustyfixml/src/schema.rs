//! FIXML schema definitions and validation.

use serde::{Deserialize, Serialize};

/// FIXML schema versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SchemaVersion {
    /// FIXML 4.4
    V44,
    /// FIXML 5.0 SP2
    V50SP2,
}

/// FIXML schema manager.
#[derive(Debug)]
pub struct FixmlSchema {
    version: SchemaVersion,
}

impl FixmlSchema {
    /// Create new schema with specified version.
    pub fn new(version: SchemaVersion) -> Self {
        Self { version }
    }

    /// Get schema namespace URI.
    pub fn namespace_uri(&self) -> &'static str {
        match self.version {
            SchemaVersion::V44 => "http://www.fixprotocol.org/FIXML-4-4",
            SchemaVersion::V50SP2 => "http://www.fixprotocol.org/FIXML-5-0-SP2",
        }
    }

    /// Get schema version.
    pub fn version(&self) -> SchemaVersion {
        self.version
    }
}

impl Default for FixmlSchema {
    fn default() -> Self {
        Self::new(SchemaVersion::V44)
    }
}
