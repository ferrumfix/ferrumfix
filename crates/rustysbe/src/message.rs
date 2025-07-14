//! Common traits and utilities for SBE messages

use crate::decoder::SbeDecoder;
use crate::encoder::SbeEncoder;
use crate::error::{SbeError, SbeResult};

/// Trait for SBE message types with compile-time metadata
pub trait SbeMessage: Sized {
    /// Template ID for this message type
    const TEMPLATE_ID: u16;

    /// Schema version for this message type
    const SCHEMA_VERSION: u16;

    /// Block length for fixed fields
    const BLOCK_LENGTH: u16;

    /// Message name for debugging
    const MESSAGE_NAME: &'static str;

    /// Create a decoder wrapper for this message type
    fn decode(data: &[u8]) -> SbeResult<SbeMessageDecoder<'_, Self>> {
        let decoder = SbeDecoder::new(data)?;
        decoder.verify_template_id(Self::TEMPLATE_ID)?;
        decoder.verify_schema_version(Self::SCHEMA_VERSION)?;

        Ok(SbeMessageDecoder {
            decoder,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Create an encoder for this message type
    fn encode() -> SbeMessageEncoder<Self> {
        let encoder = SbeEncoder::new(Self::TEMPLATE_ID, Self::SCHEMA_VERSION, Self::BLOCK_LENGTH);

        SbeMessageEncoder {
            encoder,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Create an encoder with specific capacity
    fn encode_with_capacity(capacity: usize) -> SbeMessageEncoder<Self> {
        let encoder = SbeEncoder::with_capacity(
            Self::TEMPLATE_ID,
            Self::SCHEMA_VERSION,
            Self::BLOCK_LENGTH,
            capacity,
        );

        SbeMessageEncoder {
            encoder,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Validate that a buffer contains a message of this type
    fn validate_header(data: &[u8]) -> SbeResult<()> {
        let decoder = SbeDecoder::new(data)?;
        decoder.verify_template_id(Self::TEMPLATE_ID)?;
        decoder.verify_schema_version(Self::SCHEMA_VERSION)?;
        Ok(())
    }

    /// Get message metadata
    fn metadata() -> SbeMessageMetadata {
        SbeMessageMetadata {
            template_id: Self::TEMPLATE_ID,
            schema_version: Self::SCHEMA_VERSION,
            block_length: Self::BLOCK_LENGTH,
            message_name: Self::MESSAGE_NAME,
        }
    }
}

/// Typed wrapper around SbeDecoder for specific message types
pub struct SbeMessageDecoder<'a, T: SbeMessage> {
    decoder: SbeDecoder<'a>,
    _phantom: std::marker::PhantomData<T>,
}

impl<'a, T: SbeMessage> SbeMessageDecoder<'a, T> {
    /// Get the underlying decoder
    pub fn decoder(&self) -> &SbeDecoder<'a> {
        &self.decoder
    }

    /// Get message template ID
    pub fn template_id(&self) -> u16 {
        self.decoder.template_id()
    }

    /// Get schema version
    pub fn schema_version(&self) -> u16 {
        self.decoder.schema_version()
    }

    /// Read u8 field at offset
    pub fn read_u8(&self, offset: usize) -> SbeResult<u8> {
        self.decoder.read_u8(offset)
    }

    /// Read u16 field at offset
    pub fn read_u16(&self, offset: usize) -> SbeResult<u16> {
        self.decoder.read_u16(offset)
    }

    /// Read u32 field at offset
    pub fn read_u32(&self, offset: usize) -> SbeResult<u32> {
        self.decoder.read_u32(offset)
    }

    /// Read u64 field at offset
    pub fn read_u64(&self, offset: usize) -> SbeResult<u64> {
        self.decoder.read_u64(offset)
    }

    /// Read f32 field at offset
    pub fn read_f32(&self, offset: usize) -> SbeResult<f32> {
        self.decoder.read_f32(offset)
    }

    /// Read string field at offset
    pub fn read_string(&self, offset: usize, length: usize) -> SbeResult<&'a str> {
        self.decoder.read_string(offset, length)
    }

    /// Read bytes at offset
    pub fn read_bytes(&self, offset: usize, length: usize) -> SbeResult<&'a [u8]> {
        self.decoder.read_bytes(offset, length)
    }
}

/// Typed wrapper around SbeEncoder for specific message types
pub struct SbeMessageEncoder<T: SbeMessage> {
    encoder: SbeEncoder,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: SbeMessage> SbeMessageEncoder<T> {
    /// Get the underlying encoder
    pub fn encoder(&mut self) -> &mut SbeEncoder {
        &mut self.encoder
    }

    /// Write u8 field at offset
    pub fn write_u8(&mut self, offset: usize, value: u8) -> SbeResult<()> {
        self.encoder.write_u8(offset, value)
    }

    /// Write u16 field at offset
    pub fn write_u16(&mut self, offset: usize, value: u16) -> SbeResult<()> {
        self.encoder.write_u16(offset, value)
    }

    /// Write u32 field at offset
    pub fn write_u32(&mut self, offset: usize, value: u32) -> SbeResult<()> {
        self.encoder.write_u32(offset, value)
    }

    /// Write u64 field at offset
    pub fn write_u64(&mut self, offset: usize, value: u64) -> SbeResult<()> {
        self.encoder.write_u64(offset, value)
    }

    /// Write f32 field at offset
    pub fn write_f32(&mut self, offset: usize, value: f32) -> SbeResult<()> {
        self.encoder.write_f32(offset, value)
    }

    /// Write string field at offset
    pub fn write_string(&mut self, offset: usize, length: usize, value: &str) -> SbeResult<()> {
        self.encoder.write_string(offset, length, value)
    }

    /// Write bytes at offset
    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> SbeResult<()> {
        self.encoder.write_bytes(offset, bytes)
    }

    /// Write variable-length string
    pub fn write_variable_string(&mut self, value: &str) -> SbeResult<()> {
        self.encoder.write_variable_string(value)
    }

    /// Write variable-length bytes
    pub fn write_variable_bytes(&mut self, bytes: &[u8]) -> SbeResult<()> {
        self.encoder.write_variable_bytes(bytes)
    }

    /// Finalize and return encoded message
    pub fn finalize(self) -> SbeResult<Vec<u8>> {
        self.encoder.finalize()
    }

    /// Get current message size
    pub fn current_size(&self) -> usize {
        self.encoder.current_size()
    }
}

/// Metadata about an SBE message type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SbeMessageMetadata {
    /// Template ID
    pub template_id: u16,
    /// Schema version
    pub schema_version: u16,
    /// Fixed fields block length
    pub block_length: u16,
    /// Message name
    pub message_name: &'static str,
}

impl SbeMessageMetadata {
    /// Check if this metadata matches a template ID
    pub fn matches_template(&self, template_id: u16) -> bool {
        self.template_id == template_id
    }

    /// Check if this metadata is compatible with a schema version
    pub fn is_compatible_version(&self, schema_version: u16) -> bool {
        self.schema_version == schema_version
    }
}

/// Registry for SBE message types
pub struct SbeMessageRegistry {
    /// Map from template ID to metadata
    messages: std::collections::HashMap<u16, SbeMessageMetadata>,
}

impl SbeMessageRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            messages: std::collections::HashMap::new(),
        }
    }

    /// Register a message type
    pub fn register<T: SbeMessage>(&mut self) {
        let metadata = T::metadata();
        self.messages.insert(metadata.template_id, metadata);
    }

    /// Look up metadata by template ID
    pub fn get_metadata(&self, template_id: u16) -> Option<&SbeMessageMetadata> {
        self.messages.get(&template_id)
    }

    /// Check if a template ID is registered
    pub fn is_registered(&self, template_id: u16) -> bool {
        self.messages.contains_key(&template_id)
    }

    /// Get all registered template IDs
    pub fn template_ids(&self) -> Vec<u16> {
        self.messages.keys().copied().collect()
    }

    /// Get all registered metadata
    pub fn all_metadata(&self) -> Vec<&SbeMessageMetadata> {
        self.messages.values().collect()
    }
}

impl Default for SbeMessageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility for working with message headers without full decoding
pub struct SbeMessageHeader;

impl SbeMessageHeader {
    /// Extract just the template ID from a message buffer
    pub fn extract_template_id(data: &[u8]) -> SbeResult<u16> {
        if data.len() < 8 {
            return Err(SbeError::BufferTooSmall {
                need: 8,
                have: data.len(),
            });
        }

        // Template ID is at offset 4 (after 4-byte length)
        Ok(u16::from_le_bytes([data[4], data[5]]))
    }

    /// Extract schema version from a message buffer
    pub fn extract_schema_version(data: &[u8]) -> SbeResult<u16> {
        if data.len() < 8 {
            return Err(SbeError::BufferTooSmall {
                need: 8,
                have: data.len(),
            });
        }

        // Schema version is at offset 6 (after 4-byte length + 2-byte template ID)
        Ok(u16::from_le_bytes([data[6], data[7]]))
    }

    /// Extract message length from a message buffer
    pub fn extract_message_length(data: &[u8]) -> SbeResult<u32> {
        if data.len() < 4 {
            return Err(SbeError::BufferTooSmall {
                need: 4,
                have: data.len(),
            });
        }

        // Message length is at offset 0
        Ok(u32::from_le_bytes([data[0], data[1], data[2], data[3]]))
    }

    /// Validate a message header without full parsing
    pub fn validate_basic(data: &[u8]) -> SbeResult<(u32, u16, u16)> {
        let length = Self::extract_message_length(data)?;
        let template_id = Self::extract_template_id(data)?;
        let schema_version = Self::extract_schema_version(data)?;

        // Basic sanity checks
        if length < 8 {
            return Err(SbeError::InvalidMessageLength {
                length: length as u16,
            });
        }

        if length as usize > data.len() {
            return Err(SbeError::InvalidMessageLength {
                length: length as u16,
            });
        }

        Ok((length, template_id, schema_version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock message type for testing
    struct TestMessage;

    impl SbeMessage for TestMessage {
        const TEMPLATE_ID: u16 = 1;
        const SCHEMA_VERSION: u16 = 0;
        const BLOCK_LENGTH: u16 = 16;
        const MESSAGE_NAME: &'static str = "TestMessage";
    }

    #[test]
    fn test_message_metadata() {
        let metadata = TestMessage::metadata();
        assert_eq!(metadata.template_id, 1);
        assert_eq!(metadata.schema_version, 0);
        assert_eq!(metadata.block_length, 16);
        assert_eq!(metadata.message_name, "TestMessage");
    }

    #[test]
    fn test_message_registry() {
        let mut registry = SbeMessageRegistry::new();
        registry.register::<TestMessage>();

        assert!(registry.is_registered(1));
        assert!(!registry.is_registered(2));

        let metadata = registry.get_metadata(1).unwrap();
        assert_eq!(metadata.message_name, "TestMessage");
    }

    #[test]
    fn test_header_extraction() {
        let mut encoder = TestMessage::encode();
        encoder.write_u64(0, 12345).unwrap();
        let message = encoder.finalize().unwrap();

        let template_id = SbeMessageHeader::extract_template_id(&message).unwrap();
        let schema_version = SbeMessageHeader::extract_schema_version(&message).unwrap();
        let length = SbeMessageHeader::extract_message_length(&message).unwrap();

        assert_eq!(template_id, 1);
        assert_eq!(schema_version, 0);
        assert_eq!(length, message.len() as u32);
    }

    #[test]
    fn test_typed_encoding_decoding() {
        // Test typed message interface
        let mut encoder = TestMessage::encode();
        encoder.write_u32(0, 42).unwrap();
        encoder.write_u64(4, 1234567890).unwrap();
        encoder.write_u16(12, 999).unwrap();

        let message = encoder.finalize().unwrap();

        // Decode with type safety
        let decoder = TestMessage::decode(&message).unwrap();
        assert_eq!(decoder.read_u32(0).unwrap(), 42);
        assert_eq!(decoder.read_u64(4).unwrap(), 1234567890);
        assert_eq!(decoder.read_u16(12).unwrap(), 999);
    }
}
