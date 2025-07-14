//! High-performance Protocol Buffers decoder for FIX messages.

use crate::{
    buffer::BufferUtils,
    error::{DecodeError, GpbError},
    FieldValue, FixMessage, GpbReader, MessageType,
};
use fastrace::prelude::*;
use std::collections::HashMap;
use std::io::Read;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Wire type indicators for Protocol Buffers.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum WireType {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

/// Configuration for GPB decoding operations.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DecodeConfig {
    /// Validate checksums if present
    pub validate_checksums: bool,
    /// Validate message structure
    pub validate_structure: bool,
    /// Maximum message size to accept
    pub max_message_size: usize,
    /// Strict field validation
    pub strict_field_validation: bool,
    /// Zero-copy string optimization
    pub zero_copy_strings: bool,
}

impl Default for DecodeConfig {
    fn default() -> Self {
        Self {
            validate_checksums: true,
            validate_structure: true,
            max_message_size: 1024 * 1024, // 1MB
            strict_field_validation: true,
            zero_copy_strings: false, // Disabled by default for safety
        }
    }
}

/// High-performance Protocol Buffers decoder for FIX messages.
#[derive(Debug)]
pub struct GpbDecoder {
    config: DecodeConfig,
    /// Reverse field mappings (GPB field -> FIX tag)
    reverse_field_mappings: HashMap<u32, u32>,
}

impl GpbDecoder {
    /// Create a new decoder with default configuration.
    pub fn new() -> Self {
        Self::with_config(DecodeConfig::default())
    }

    /// Create decoder with custom configuration.
    pub fn with_config(config: DecodeConfig) -> Self {
        Self {
            config,
            reverse_field_mappings: Self::create_reverse_field_mappings(),
        }
    }

    /// Decode a Protocol Buffers message to FIX format.
    #[trace]
    pub fn decode(&self, data: &[u8]) -> Result<FixMessage, GpbError> {
        // Validate input size
        if data.len() > self.config.max_message_size {
            return Err(GpbError::Decode(DecodeError::TruncatedBuffer {
                expected: self.config.max_message_size,
                actual: data.len(),
            }));
        }

        let mut reader = GpbReader::new(data)?;

        // Start with default message
        let mut message_type = MessageType::Heartbeat;
        let mut seq_num = None;
        let mut sender_comp_id = None;
        let mut target_comp_id = None;
        let mut sending_time = None;
        let mut fields = HashMap::new();
        let mut checksum_validated = false;

        // Decode all fields
        while reader.has_remaining() {
            let tag_and_wire = self.decode_varint(&mut reader)?;
            let field_num = (tag_and_wire >> 3) as u32;
            let wire_type = (tag_and_wire & 0x07) as u8;

            match field_num {
                1 => {
                    // Message Type
                    if wire_type != WireType::LengthDelimited as u8 {
                        return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                            reason: format!("Invalid wire type for message type: {}", wire_type),
                        }));
                    }
                    let msg_type_str = self.decode_string(&mut reader)?;
                    message_type = MessageType::from_str(&msg_type_str);
                }
                2 => {
                    // Sequence Number
                    if wire_type != WireType::Varint as u8 {
                        return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                            reason: format!("Invalid wire type for sequence number: {}", wire_type),
                        }));
                    }
                    seq_num = Some(self.decode_varint(&mut reader)? as u32);
                }
                3 => {
                    // Sender CompID
                    if wire_type != WireType::LengthDelimited as u8 {
                        return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                            reason: format!("Invalid wire type for sender CompID: {}", wire_type),
                        }));
                    }
                    sender_comp_id = Some(self.decode_string(&mut reader)?);
                }
                4 => {
                    // Target CompID
                    if wire_type != WireType::LengthDelimited as u8 {
                        return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                            reason: format!("Invalid wire type for target CompID: {}", wire_type),
                        }));
                    }
                    target_comp_id = Some(self.decode_string(&mut reader)?);
                }
                5 => {
                    // Sending Time
                    if wire_type != WireType::Varint as u8 {
                        return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                            reason: format!("Invalid wire type for sending time: {}", wire_type),
                        }));
                    }
                    sending_time = Some(self.decode_varint(&mut reader)?);
                }
                999 => {
                    // Checksum field
                    if wire_type != WireType::Fixed32 as u8 {
                        return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                            reason: format!("Invalid wire type for checksum: {}", wire_type),
                        }));
                    }
                    let checksum = self.decode_fixed32(&mut reader)?;

                    if self.config.validate_checksums {
                        // Calculate checksum of data excluding the checksum field
                        let data_without_checksum = &data[..data.len() - 6]; // Assume 6 bytes for checksum field
                        let calculated_checksum = BufferUtils::crc32(data_without_checksum);

                        if checksum != calculated_checksum {
                            return Err(GpbError::Decode(DecodeError::ChecksumMismatch {
                                expected: checksum,
                                actual: calculated_checksum,
                            }));
                        }
                        checksum_validated = true;
                    }
                }
                0 => {
                    // Batch header - skip for now
                    self.skip_field(&mut reader, wire_type)?;
                }
                _ => {
                    // Regular FIX field
                    let fix_tag = self.map_gpb_field_to_fix_tag(field_num);
                    let field_value = self.decode_field_value(&mut reader, wire_type)?;
                    fields.insert(fix_tag, field_value);
                }
            }
        }

        // Validate checksum was present if required
        if self.config.validate_checksums && !checksum_validated {
            return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                reason: "Missing required checksum".to_string(),
            }));
        }

        // Construct the message
        let mut message = FixMessage::new(message_type);
        message.seq_num = seq_num;
        message.sender_comp_id = sender_comp_id;
        message.target_comp_id = target_comp_id;
        message.sending_time = sending_time;
        message.fields = fields;

        // Validate message structure if configured
        if self.config.validate_structure {
            message.validate().map_err(GpbError::Encode)?;
        }

        Ok(message)
    }

    /// Decode a batch of messages.
    #[trace]
    pub fn decode_batch(&self, data: &[u8]) -> Result<Vec<FixMessage>, GpbError> {
        let mut reader = GpbReader::new(data)?;
        let mut messages = Vec::new();

        // First field should be batch count
        let tag_and_wire = self.decode_varint(&mut reader)?;
        let field_num = (tag_and_wire >> 3) as u32;
        let wire_type = (tag_and_wire & 0x07) as u8;

        if field_num != 0 || wire_type != WireType::Varint as u8 {
            return Err(GpbError::Decode(DecodeError::InvalidWireFormat {
                reason: "Invalid batch header".to_string(),
            }));
        }

        let message_count = self.decode_varint(&mut reader)? as usize;
        messages.reserve(message_count);

        // Decode each message
        for _ in 0..message_count {
            if !reader.has_remaining() {
                break;
            }

            // Read message length
            let msg_length = self.decode_varint(&mut reader)? as usize;

            // Extract message data
            let mut msg_data = vec![0u8; msg_length];
            reader.read_exact(&mut msg_data).map_err(|_| {
                GpbError::Decode(DecodeError::TruncatedBuffer {
                    expected: msg_length,
                    actual: reader.remaining(),
                })
            })?;

            // Decode the individual message with relaxed checksum validation
            // (individual messages in batch don't have checksums)
            let mut batch_config = self.config.clone();
            batch_config.validate_checksums = false;
            let batch_decoder = GpbDecoder::with_config(batch_config);
            let message = batch_decoder.decode(&msg_data)?;
            messages.push(message);
        }

        Ok(messages)
    }

    /// Decode field value based on wire type.
    #[trace]
    fn decode_field_value(
        &self,
        reader: &mut GpbReader,
        wire_type: u8,
    ) -> Result<FieldValue, DecodeError> {
        match wire_type {
            t if t == WireType::Varint as u8 => {
                let value = self.decode_varint(reader)?;
                // Try to determine if this is signed (zigzag encoded)
                // Check if the zigzag decode produces a negative number that makes sense
                let zigzag_decoded = self.decode_zigzag(value);

                // If the zigzag decoding produces a negative number or the value is odd
                // (which indicates a negative number in zigzag encoding), treat as signed
                if zigzag_decoded < 0 || (value & 1) == 1 {
                    Ok(FieldValue::Int(zigzag_decoded))
                } else {
                    Ok(FieldValue::UInt(value))
                }
            }
            t if t == WireType::Fixed64 as u8 => {
                let value = self.decode_double(reader)?;
                Ok(FieldValue::Float(value))
            }
            t if t == WireType::LengthDelimited as u8 => {
                let bytes = self.decode_bytes(reader)?;

                // Try to decode as UTF-8 string first
                match String::from_utf8(bytes.clone()) {
                    Ok(string) => Ok(FieldValue::String(string)),
                    Err(_) => {
                        // Check if it might be a decimal (embedded message)
                        if let Ok(decimal) = self.try_decode_decimal(&bytes) {
                            Ok(decimal)
                        } else {
                            Ok(FieldValue::Bytes(bytes))
                        }
                    }
                }
            }
            t if t == WireType::Fixed32 as u8 => {
                let value = self.decode_fixed32(reader)?;
                Ok(FieldValue::UInt(value as u64))
            }
            _ => Err(DecodeError::InvalidWireFormat {
                reason: format!("Unknown wire type: {}", wire_type),
            }),
        }
    }

    /// Decode variable-length integer.
    fn decode_varint(&self, reader: &mut GpbReader) -> Result<u64, DecodeError> {
        BufferUtils::decode_varint(reader)
    }

    /// Decode zigzag-encoded signed integer.
    fn decode_zigzag(&self, value: u64) -> i64 {
        ((value >> 1) as i64) ^ (-((value & 1) as i64))
    }

    /// Decode string with length prefix.
    fn decode_string(&self, reader: &mut GpbReader) -> Result<String, DecodeError> {
        let bytes = self.decode_bytes(reader)?;
        String::from_utf8(bytes).map_err(|e| DecodeError::InvalidWireFormat {
            reason: format!("Invalid UTF-8: {}", e),
        })
    }

    /// Decode byte array with length prefix.
    fn decode_bytes(&self, reader: &mut GpbReader) -> Result<Vec<u8>, DecodeError> {
        let length = self.decode_varint(reader)? as usize;

        if length > reader.remaining() {
            return Err(DecodeError::TruncatedBuffer {
                expected: length,
                actual: reader.remaining(),
            });
        }

        let mut bytes = vec![0u8; length];
        reader
            .read_exact(&mut bytes)
            .map_err(|_| DecodeError::TruncatedBuffer {
                expected: length,
                actual: reader.remaining(),
            })?;

        Ok(bytes)
    }

    /// Decode 64-bit double.
    fn decode_double(&self, reader: &mut GpbReader) -> Result<f64, DecodeError> {
        let mut bytes = [0u8; 8];
        reader
            .read_exact(&mut bytes)
            .map_err(|_| DecodeError::TruncatedBuffer {
                expected: 8,
                actual: reader.remaining(),
            })?;

        Ok(f64::from_le_bytes(bytes))
    }

    /// Decode 32-bit fixed integer.
    fn decode_fixed32(&self, reader: &mut GpbReader) -> Result<u32, DecodeError> {
        let mut bytes = [0u8; 4];
        reader
            .read_exact(&mut bytes)
            .map_err(|_| DecodeError::TruncatedBuffer {
                expected: 4,
                actual: reader.remaining(),
            })?;

        Ok(u32::from_le_bytes(bytes))
    }

    /// Try to decode bytes as a decimal embedded message.
    fn try_decode_decimal(&self, bytes: &[u8]) -> Result<FieldValue, DecodeError> {
        let mut reader = GpbReader::new(bytes).map_err(|e| match e {
            GpbError::Io(_io_err) => DecodeError::TruncatedBuffer {
                expected: bytes.len(),
                actual: 0,
            },
            _ => DecodeError::InvalidWireFormat {
                reason: format!("Failed to create reader: {}", e),
            },
        })?;
        let mut mantissa = None;
        let mut scale = None;

        while reader.has_remaining() {
            let tag_and_wire = self.decode_varint(&mut reader)?;
            let field_num = (tag_and_wire >> 3) as u32;
            let wire_type = (tag_and_wire & 0x07) as u8;

            if wire_type != WireType::Varint as u8 {
                return Err(DecodeError::InvalidWireFormat {
                    reason: "Invalid decimal field wire type".to_string(),
                });
            }

            match field_num {
                1 => {
                    // Mantissa
                    let value = self.decode_varint(&mut reader)?;
                    mantissa = Some(self.decode_zigzag(value));
                }
                2 => {
                    // Scale
                    let value = self.decode_varint(&mut reader)?;
                    scale = Some(self.decode_zigzag(value) as i32);
                }
                _ => {
                    // Unknown field in decimal - skip
                    self.skip_field(&mut reader, wire_type)?;
                }
            }
        }

        match (mantissa, scale) {
            (Some(m), Some(s)) => Ok(FieldValue::Decimal {
                mantissa: m,
                scale: s,
            }),
            _ => Err(DecodeError::InvalidWireFormat {
                reason: "Incomplete decimal fields".to_string(),
            }),
        }
    }

    /// Skip a field based on wire type.
    fn skip_field(&self, reader: &mut GpbReader, wire_type: u8) -> Result<(), DecodeError> {
        match wire_type {
            t if t == WireType::Varint as u8 => {
                self.decode_varint(reader)?;
            }
            t if t == WireType::Fixed64 as u8 => {
                let mut bytes = [0u8; 8];
                reader
                    .read_exact(&mut bytes)
                    .map_err(|_| DecodeError::TruncatedBuffer {
                        expected: 8,
                        actual: reader.remaining(),
                    })?;
            }
            t if t == WireType::LengthDelimited as u8 => {
                let length = self.decode_varint(reader)? as usize;
                let mut bytes = vec![0u8; length];
                reader
                    .read_exact(&mut bytes)
                    .map_err(|_| DecodeError::TruncatedBuffer {
                        expected: length,
                        actual: reader.remaining(),
                    })?;
            }
            t if t == WireType::Fixed32 as u8 => {
                let mut bytes = [0u8; 4];
                reader
                    .read_exact(&mut bytes)
                    .map_err(|_| DecodeError::TruncatedBuffer {
                        expected: 4,
                        actual: reader.remaining(),
                    })?;
            }
            _ => {
                return Err(DecodeError::InvalidWireFormat {
                    reason: format!("Unknown wire type to skip: {}", wire_type),
                });
            }
        }

        Ok(())
    }

    /// Map GPB field number back to FIX tag.
    fn map_gpb_field_to_fix_tag(&self, gpb_field: u32) -> u32 {
        self.reverse_field_mappings
            .get(&gpb_field)
            .copied()
            .unwrap_or(gpb_field.saturating_sub(100)) // Reverse the offset
    }

    /// Create reverse field mappings (GPB field -> FIX tag).
    fn create_reverse_field_mappings() -> HashMap<u32, u32> {
        let mut mappings = HashMap::new();

        // Reverse the encoder mappings
        mappings.insert(10, 8); // 10 -> BeginString
        mappings.insert(11, 9); // 11 -> BodyLength
        mappings.insert(12, 35); // 12 -> MsgType
        mappings.insert(13, 34); // 13 -> MsgSeqNum
        mappings.insert(14, 49); // 14 -> SenderCompID
        mappings.insert(15, 56); // 15 -> TargetCompID
        mappings.insert(16, 52); // 16 -> SendingTime
        mappings.insert(20, 55); // 20 -> Symbol
        mappings.insert(21, 44); // 21 -> Price
        mappings.insert(22, 38); // 22 -> OrderQty
        mappings.insert(23, 54); // 23 -> Side
        mappings.insert(24, 40); // 24 -> OrdType
        mappings.insert(25, 59); // 25 -> TimeInForce
        mappings.insert(30, 37); // 30 -> OrderID
        mappings.insert(31, 17); // 31 -> ExecID
        mappings.insert(32, 150); // 32 -> ExecType
        mappings.insert(33, 39); // 33 -> OrdStatus
        mappings.insert(34, 32); // 34 -> LastQty
        mappings.insert(35, 31); // 35 -> LastPx

        mappings
    }

    /// Get decoder statistics for monitoring.
    pub fn stats(&self) -> DecoderStats {
        DecoderStats {
            reverse_mappings_count: self.reverse_field_mappings.len(),
            config: self.config.clone(),
        }
    }
}

impl Default for GpbDecoder {
    fn default() -> Self {
        Self::new()
    }
}

/// Decoder performance and usage statistics.
#[derive(Debug, Clone)]
pub struct DecoderStats {
    /// Number of reverse field mappings
    pub reverse_mappings_count: usize,
    /// Current decoder configuration
    pub config: DecodeConfig,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GpbEncoder, MessageType};

    #[test]
    fn test_decoder_creation() {
        let decoder = GpbDecoder::new();
        let stats = decoder.stats();
        assert!(stats.reverse_mappings_count > 0);
    }

    #[test]
    fn test_encode_decode_round_trip() {
        let mut encoder = GpbEncoder::new();
        let decoder = GpbDecoder::new();

        let original_message =
            FixMessage::new_order_single("BTCUSD".to_string(), 50000.0, 1.5, "1".to_string());

        // Encode
        let encoded = encoder.encode(&original_message).unwrap();

        // Decode
        let decoded_message = decoder.decode(encoded).unwrap();

        // Verify round trip
        assert_eq!(original_message.message_type, decoded_message.message_type);
        assert_eq!(original_message.fields.len(), decoded_message.fields.len());

        // Check specific fields
        assert_eq!(
            original_message.get_field(55).unwrap().as_string(),
            decoded_message.get_field(55).unwrap().as_string()
        );
        assert_eq!(
            original_message.get_field(44).unwrap().as_float(),
            decoded_message.get_field(44).unwrap().as_float()
        );
    }

    #[test]
    fn test_decode_with_checksum_validation() {
        let decoder = GpbDecoder::with_config(DecodeConfig {
            validate_checksums: true,
            ..Default::default()
        });

        let mut encoder = GpbEncoder::new();
        let message =
            FixMessage::new_order_single("ETHUSD".to_string(), 3000.0, 2.0, "2".to_string());

        let encoded = encoder.encode(&message).unwrap();
        let decoded = decoder.decode(encoded).unwrap();

        assert_eq!(message.message_type, decoded.message_type);
    }

    #[test]
    fn test_decode_different_field_types() {
        let mut encoder = GpbEncoder::new();
        let decoder = GpbDecoder::new();

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
        let decoded = decoder.decode(encoded).unwrap();

        assert_eq!(message.fields.len(), decoded.fields.len());

        // Verify specific field types
        assert!(matches!(
            decoded.get_field(1).unwrap(),
            FieldValue::String(_)
        ));
        assert!(matches!(decoded.get_field(2).unwrap(), FieldValue::Int(_)));
        assert!(matches!(decoded.get_field(3).unwrap(), FieldValue::UInt(_)));
        assert!(matches!(
            decoded.get_field(4).unwrap(),
            FieldValue::Float(_)
        ));
        assert!(matches!(
            decoded.get_field(6).unwrap(),
            FieldValue::Decimal { .. }
        ));
    }

    #[test]
    fn test_invalid_data_handling() {
        let decoder = GpbDecoder::new();

        // Test with invalid data
        let invalid_data = b"this is not protobuf data";
        let result = decoder.decode(invalid_data);
        assert!(result.is_err());

        // Test with truncated data
        let truncated_data = b"\x08\x96\x01"; // Incomplete varint
        let _result = decoder.decode(truncated_data);
        // This might succeed or fail depending on the specific bytes
        // The important thing is it doesn't panic
    }

    #[test]
    fn test_batch_decode() {
        let mut encoder = GpbEncoder::new();
        let decoder = GpbDecoder::new();

        let messages = vec![
            FixMessage::new_order_single("BTC".to_string(), 50000.0, 1.0, "1".to_string()),
            FixMessage::new_order_single("ETH".to_string(), 3000.0, 2.0, "2".to_string()),
        ];

        let encoded = encoder.encode_batch(&messages).unwrap();
        let decoded_messages = decoder.decode_batch(encoded).unwrap();

        assert_eq!(messages.len(), decoded_messages.len());

        for (original, decoded) in messages.iter().zip(decoded_messages.iter()) {
            assert_eq!(original.message_type, decoded.message_type);
        }
    }
}
