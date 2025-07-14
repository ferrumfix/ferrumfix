//! High-performance Protocol Buffers encoder for FIX messages.

use crate::{
    buffer::BufferUtils,
    error::{EncodeError, GpbError},
    FieldValue, FixMessage, GpbWriter,
};
use fastrace::prelude::*;
use smallvec::SmallVec;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Wire type indicators for Protocol Buffers.
#[derive(Debug, Clone, Copy)]
enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

/// Configuration for GPB encoding operations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EncodeConfig {
    /// Enable message validation before encoding
    pub validate_messages: bool,
    /// Include checksums for data integrity
    pub include_checksums: bool,
    /// Compress repeated fields
    pub compress_repeated: bool,
    /// Maximum message size in bytes
    pub max_message_size: usize,
    /// Buffer pre-allocation strategy
    pub buffer_strategy: BufferStrategy,
}

impl Default for EncodeConfig {
    fn default() -> Self {
        Self {
            validate_messages: true,
            include_checksums: true,
            compress_repeated: true,
            max_message_size: 1024 * 1024, // 1MB
            buffer_strategy: BufferStrategy::Adaptive,
        }
    }
}

/// Buffer allocation strategies for optimal performance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BufferStrategy {
    /// Fixed buffer size
    Fixed(usize),
    /// Adaptive based on message size estimation
    Adaptive,
    /// Large buffer for batch operations
    Batch,
}

/// High-performance Protocol Buffers encoder for FIX messages.
#[derive(Debug)]
pub struct GpbEncoder {
    config: EncodeConfig,
    writer: GpbWriter,
    /// Field ID mappings for optimization
    field_mappings: HashMap<u32, u32>,
}

impl GpbEncoder {
    /// Create a new encoder with default configuration.
    pub fn new() -> Self {
        Self::with_config(EncodeConfig::default())
    }

    /// Create encoder with custom configuration.
    pub fn with_config(config: EncodeConfig) -> Self {
        let writer_capacity = match config.buffer_strategy {
            BufferStrategy::Fixed(size) => size,
            BufferStrategy::Adaptive => 8192,   // 8KB default
            BufferStrategy::Batch => 64 * 1024, // 64KB for batch
        };

        Self {
            config,
            writer: GpbWriter::with_capacity(writer_capacity),
            field_mappings: Self::create_field_mappings(),
        }
    }

    /// Encode a FIX message to Protocol Buffers format.
    #[trace]
    pub fn encode(&mut self, message: &FixMessage) -> Result<&[u8], GpbError> {
        // Clear previous content
        self.writer.clear();

        // Validate message if configured
        if self.config.validate_messages {
            message.validate()?;
        }

        // Check message size limits
        let estimated_size = message.estimated_size();
        if estimated_size > self.config.max_message_size {
            return Err(GpbError::Encode(EncodeError::InvalidFieldValue {
                field_id: 0,
                reason: format!("Message too large: {} bytes", estimated_size),
            }));
        }

        // Ensure buffer capacity
        self.ensure_capacity(estimated_size)?;

        // Encode message header
        self.encode_message_header(message)?;

        // Encode fields
        self.encode_fields(message)?;

        // Add checksum if configured
        if self.config.include_checksums {
            self.encode_checksum()?;
        }

        Ok(self.writer.as_bytes())
    }

    /// Encode multiple messages in batch for better performance.
    #[trace]
    pub fn encode_batch(&mut self, messages: &[FixMessage]) -> Result<&[u8], GpbError> {
        self.writer.clear();

        // Calculate total estimated size
        let total_size: usize = messages.iter().map(|m| m.estimated_size()).sum();

        if total_size > self.config.max_message_size {
            return Err(GpbError::Encode(EncodeError::InvalidFieldValue {
                field_id: 0,
                reason: format!("Batch too large: {} bytes", total_size),
            }));
        }

        self.ensure_capacity(total_size + 1024)?; // Extra padding

        // Encode batch header
        self.encode_batch_header(messages.len())?;

        // Encode each message
        for message in messages {
            if self.config.validate_messages {
                message.validate()?;
            }

            // Encode message to a temporary buffer to get its length
            let mut temp_encoder = GpbEncoder::new();
            temp_encoder.encode_message_header(message)?;
            temp_encoder.encode_fields(message)?;
            let msg_data = temp_encoder.writer.as_bytes();
            let msg_len = msg_data.len();

            // Encode length prefix first
            self.encode_varint(msg_len as u64)?;

            // Then encode the message data
            self.write_bytes(msg_data)?;
        }

        if self.config.include_checksums {
            self.encode_checksum()?;
        }

        Ok(self.writer.as_bytes())
    }

    /// Encode message header with metadata.
    #[trace]
    fn encode_message_header(&mut self, message: &FixMessage) -> Result<(), EncodeError> {
        // Field 1: Message Type
        self.encode_field_header(1, WireType::LengthDelimited)?;
        let msg_type_str = message.message_type.as_str();
        self.encode_string(msg_type_str)?;

        // Field 2: Sequence Number (if present)
        if let Some(seq_num) = message.seq_num {
            self.encode_field_header(2, WireType::Varint)?;
            self.encode_varint(seq_num as u64)?;
        }

        // Field 3: Sender CompID (if present)
        if let Some(ref sender) = message.sender_comp_id {
            self.encode_field_header(3, WireType::LengthDelimited)?;
            self.encode_string(sender)?;
        }

        // Field 4: Target CompID (if present)
        if let Some(ref target) = message.target_comp_id {
            self.encode_field_header(4, WireType::LengthDelimited)?;
            self.encode_string(target)?;
        }

        // Field 5: Sending Time (if present)
        if let Some(sending_time) = message.sending_time {
            self.encode_field_header(5, WireType::Varint)?;
            self.encode_varint(sending_time)?;
        }

        Ok(())
    }

    /// Encode all FIX fields.
    #[trace]
    fn encode_fields(&mut self, message: &FixMessage) -> Result<(), EncodeError> {
        // Sort fields by tag for deterministic output
        let mut sorted_fields: SmallVec<[_; 64]> = message.fields.iter().collect();
        sorted_fields.sort_by_key(|(tag, _)| *tag);

        for (&tag, value) in sorted_fields {
            // Map FIX tag to GPB field number (starting from 100 to avoid conflicts)
            let field_num = self.map_fix_tag_to_gpb_field(tag);

            self.encode_fix_field(field_num, value)?;
        }

        Ok(())
    }

    /// Encode a single FIX field.
    #[trace]
    fn encode_fix_field(&mut self, field_num: u32, value: &FieldValue) -> Result<(), EncodeError> {
        match value {
            FieldValue::String(s) => {
                self.encode_field_header(field_num, WireType::LengthDelimited)?;
                self.encode_string(s)?;
            }
            FieldValue::Int(i) => {
                self.encode_field_header(field_num, WireType::Varint)?;
                self.encode_varint_signed(*i)?;
            }
            FieldValue::UInt(u) => {
                self.encode_field_header(field_num, WireType::Varint)?;
                self.encode_varint(*u)?;
            }
            FieldValue::Float(f) => {
                self.encode_field_header(field_num, WireType::Fixed64)?;
                self.encode_double(*f)?;
            }
            FieldValue::Bool(b) => {
                self.encode_field_header(field_num, WireType::Varint)?;
                self.encode_varint(if *b { 1 } else { 0 })?;
            }
            FieldValue::Bytes(bytes) => {
                self.encode_field_header(field_num, WireType::LengthDelimited)?;
                self.encode_bytes(bytes)?;
            }
            FieldValue::Decimal { mantissa, scale } => {
                // Encode as embedded message with mantissa and scale
                self.encode_field_header(field_num, WireType::LengthDelimited)?;

                let decimal_data = self.encode_decimal(*mantissa, *scale)?;
                self.encode_bytes(&decimal_data)?;
            }
            FieldValue::Timestamp(ts) => {
                self.encode_field_header(field_num, WireType::Varint)?;
                self.encode_varint(*ts)?;
            }
            FieldValue::Optional(Some(inner_value)) => {
                // Encode the inner value directly
                self.encode_fix_field(field_num, inner_value)?;
            }
            FieldValue::Optional(None) => {
                // Skip optional fields that are None
            }
        }

        Ok(())
    }

    /// Encode batch header for multiple messages.
    fn encode_batch_header(&mut self, count: usize) -> Result<(), EncodeError> {
        // Field 0: Batch message count
        self.encode_field_header(0, WireType::Varint)?;
        self.encode_varint(count as u64)?;

        Ok(())
    }

    /// Encode checksum for data integrity.
    fn encode_checksum(&mut self) -> Result<(), EncodeError> {
        let data = self.writer.as_bytes();
        let checksum = BufferUtils::crc32(data);

        // Append checksum as final field
        self.encode_field_header(999, WireType::Fixed32)?;
        self.encode_fixed32(checksum)?;

        Ok(())
    }

    /// Encode field header (tag + wire type).
    fn encode_field_header(
        &mut self,
        field_num: u32,
        wire_type: WireType,
    ) -> Result<(), EncodeError> {
        let tag = (field_num << 3) | (wire_type as u32);
        self.encode_varint(tag as u64)
    }

    /// Encode variable-length integer.
    fn encode_varint(&mut self, value: u64) -> Result<(), EncodeError> {
        let bytes = BufferUtils::encode_varint(value);
        self.write_bytes(&bytes)
    }

    /// Encode signed variable-length integer using zigzag encoding.
    fn encode_varint_signed(&mut self, value: i64) -> Result<(), EncodeError> {
        let zigzag = ((value << 1) ^ (value >> 63)) as u64;
        self.encode_varint(zigzag)
    }

    /// Encode string with length prefix.
    fn encode_string(&mut self, s: &str) -> Result<(), EncodeError> {
        let bytes = s.as_bytes();
        self.encode_varint(bytes.len() as u64)?;
        self.write_bytes(bytes)
    }

    /// Encode byte array with length prefix.
    fn encode_bytes(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        self.encode_varint(bytes.len() as u64)?;
        self.write_bytes(bytes)
    }

    /// Encode 64-bit double.
    fn encode_double(&mut self, value: f64) -> Result<(), EncodeError> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }

    /// Encode 32-bit fixed integer.
    fn encode_fixed32(&mut self, value: u32) -> Result<(), EncodeError> {
        let bytes = value.to_le_bytes();
        self.write_bytes(&bytes)
    }

    /// Encode decimal as embedded message.
    fn encode_decimal(&mut self, mantissa: i64, scale: i32) -> Result<Vec<u8>, EncodeError> {
        let mut decimal_data = Vec::new();

        // Field 1: Mantissa
        decimal_data.extend(BufferUtils::encode_varint((1 << 3) | 0)); // Field 1, Varint
        let zigzag_mantissa = ((mantissa << 1) ^ (mantissa >> 63)) as u64;
        decimal_data.extend(BufferUtils::encode_varint(zigzag_mantissa));

        // Field 2: Scale
        decimal_data.extend(BufferUtils::encode_varint((2 << 3) | 0)); // Field 2, Varint
        let zigzag_scale = (((scale as i64) << 1) ^ (scale as i64 >> 63)) as u64;
        decimal_data.extend(BufferUtils::encode_varint(zigzag_scale));

        Ok(decimal_data)
    }

    /// Write bytes to the buffer.
    fn write_bytes(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        use std::io::Write;
        self.writer
            .write_all(bytes)
            .map_err(|e| EncodeError::InvalidFieldValue {
                field_id: 0,
                reason: format!("Write error: {}", e),
            })
    }

    /// Ensure buffer has sufficient capacity.
    fn ensure_capacity(&mut self, needed: usize) -> Result<(), EncodeError> {
        let current_capacity = self.writer.buffer().capacity();
        let current_length = self.writer.buffer().len();

        if current_length + needed > current_capacity {
            // For simplicity, return error if capacity exceeded
            // In production, this would trigger buffer reallocation
            return Err(EncodeError::BufferTooSmall {
                needed: needed + current_length,
                available: current_capacity,
            });
        }

        Ok(())
    }

    /// Map FIX field tag to GPB field number.
    fn map_fix_tag_to_gpb_field(&self, fix_tag: u32) -> u32 {
        // Use pre-computed mapping for common fields, otherwise direct mapping
        self.field_mappings
            .get(&fix_tag)
            .copied()
            .unwrap_or(fix_tag + 100) // Offset to avoid conflicts with header fields
    }

    /// Create optimized field mappings for common FIX tags.
    fn create_field_mappings() -> HashMap<u32, u32> {
        let mut mappings = HashMap::new();

        // Map common FIX tags to compact GPB field numbers
        mappings.insert(8, 10); // BeginString -> 10
        mappings.insert(9, 11); // BodyLength -> 11
        mappings.insert(35, 12); // MsgType -> 12
        mappings.insert(34, 13); // MsgSeqNum -> 13
        mappings.insert(49, 14); // SenderCompID -> 14
        mappings.insert(56, 15); // TargetCompID -> 15
        mappings.insert(52, 16); // SendingTime -> 16
        mappings.insert(55, 20); // Symbol -> 20
        mappings.insert(44, 21); // Price -> 21
        mappings.insert(38, 22); // OrderQty -> 22
        mappings.insert(54, 23); // Side -> 23
        mappings.insert(40, 24); // OrdType -> 24
        mappings.insert(59, 25); // TimeInForce -> 25
        mappings.insert(37, 30); // OrderID -> 30
        mappings.insert(17, 31); // ExecID -> 31
        mappings.insert(150, 32); // ExecType -> 32
        mappings.insert(39, 33); // OrdStatus -> 33
        mappings.insert(32, 34); // LastQty -> 34
        mappings.insert(31, 35); // LastPx -> 35

        mappings
    }

    /// Get encoder statistics for monitoring.
    pub fn stats(&self) -> EncoderStats {
        EncoderStats {
            buffer_capacity: self.writer.buffer().capacity(),
            buffer_used: self.writer.buffer().len(),
            field_mappings_count: self.field_mappings.len(),
        }
    }
}

impl Default for GpbEncoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Encoder performance and usage statistics.
#[derive(Debug, Clone)]
pub struct EncoderStats {
    /// Current buffer capacity
    pub buffer_capacity: usize,
    /// Current buffer usage
    pub buffer_used: usize,
    /// Number of field mappings
    pub field_mappings_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MessageType;

    #[test]
    fn test_encoder_creation() {
        let encoder = GpbEncoder::new();
        let stats = encoder.stats();
        assert!(stats.buffer_capacity > 0);
        assert_eq!(stats.buffer_used, 0);
        assert!(stats.field_mappings_count > 0);
    }

    #[test]
    fn test_encode_new_order_single() {
        let mut encoder = GpbEncoder::new();
        let message =
            FixMessage::new_order_single("BTCUSD".to_string(), 50000.0, 1.5, "1".to_string());

        let encoded = encoder.encode(&message).unwrap();
        assert!(!encoded.is_empty());

        // Verify it starts with proper GPB header
        assert!(encoded.len() > 10);
    }

    #[test]
    fn test_encode_with_validation() {
        let mut encoder = GpbEncoder::with_config(EncodeConfig {
            validate_messages: true,
            ..Default::default()
        });

        // Invalid message (missing required fields)
        let invalid_message = FixMessage::new(MessageType::NewOrderSingle);
        let result = encoder.encode(&invalid_message);
        assert!(result.is_err());

        // Valid message
        let valid_message =
            FixMessage::new_order_single("ETHUSD".to_string(), 3000.0, 2.0, "2".to_string());
        let result = encoder.encode(&valid_message);
        assert!(result.is_ok());
    }

    #[test]
    fn test_encode_batch() {
        let mut encoder = GpbEncoder::with_config(EncodeConfig {
            buffer_strategy: BufferStrategy::Batch,
            ..Default::default()
        });

        let messages = vec![
            FixMessage::new_order_single("BTC".to_string(), 50000.0, 1.0, "1".to_string()),
            FixMessage::new_order_single("ETH".to_string(), 3000.0, 2.0, "2".to_string()),
        ];

        let encoded = encoder.encode_batch(&messages).unwrap();
        assert!(!encoded.is_empty());
        assert!(encoded.len() > 20); // Should be larger than single message
    }

    #[test]
    fn test_field_value_encoding() {
        let mut encoder = GpbEncoder::new();

        // Test different field types
        let mut message = FixMessage::new(MessageType::Heartbeat);
        message.set_field(1, FieldValue::String("test".to_string()));
        message.set_field(2, FieldValue::Int(-123));
        message.set_field(3, FieldValue::UInt(456));
        message.set_field(4, FieldValue::Float(123.45));
        message.set_field(5, FieldValue::Bool(true));
        message.set_field(
            6,
            FieldValue::Decimal {
                mantissa: 12345,
                scale: 2,
            },
        );

        let encoded = encoder.encode(&message).unwrap();
        assert!(!encoded.is_empty());
    }

    #[test]
    fn test_buffer_strategies() {
        // Fixed buffer
        let encoder_fixed = GpbEncoder::with_config(EncodeConfig {
            buffer_strategy: BufferStrategy::Fixed(1024),
            ..Default::default()
        });
        assert!(encoder_fixed.writer.buffer().capacity() >= 1024);

        // Adaptive buffer
        let encoder_adaptive = GpbEncoder::with_config(EncodeConfig {
            buffer_strategy: BufferStrategy::Adaptive,
            ..Default::default()
        });
        assert!(encoder_adaptive.writer.buffer().capacity() > 0);

        // Batch buffer
        let encoder_batch = GpbEncoder::with_config(EncodeConfig {
            buffer_strategy: BufferStrategy::Batch,
            ..Default::default()
        });
        assert!(encoder_batch.writer.buffer().capacity() >= 64 * 1024);
    }
}
