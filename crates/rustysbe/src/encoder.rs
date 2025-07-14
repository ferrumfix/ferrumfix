//! High-performance SBE message encoder

use crate::buffer::{SBE_HEADER_SIZE, SbeBuffer, SbeSmallVec};
use crate::error::{SbeError, SbeResult};
use smallvec::SmallVec;
// zerocopy imports removed - using primitive byte conversion instead

/// SBE message encoder for building messages efficiently
pub struct SbeEncoder {
    /// Buffer for message data
    buffer: SbeBuffer,
    /// Template ID for this message type
    template_id: u16,
    /// Schema version
    schema_version: u16,
    /// Fixed fields block length
    block_length: u16,
    /// Current position for variable data
    variable_data_offset: usize,
    /// Stack of group encoders for nested groups
    group_stack: SbeSmallVec<GroupEncoder>,
}

impl SbeEncoder {
    /// Create a new encoder for a specific message template
    pub fn new(template_id: u16, schema_version: u16, block_length: u16) -> Self {
        let mut buffer = SbeBuffer::new();

        // Reserve space for header (will be written when message is finalized)
        buffer.write_bytes(&[0u8; SBE_HEADER_SIZE]).unwrap();

        // Reserve space for fixed fields block
        buffer
            .write_bytes(&vec![0u8; block_length as usize])
            .unwrap();

        Self {
            buffer,
            template_id,
            schema_version,
            block_length,
            variable_data_offset: SBE_HEADER_SIZE + block_length as usize,
            group_stack: SmallVec::new(),
        }
    }

    /// Create encoder with specific buffer capacity
    pub fn with_capacity(
        template_id: u16,
        schema_version: u16,
        block_length: u16,
        capacity: usize,
    ) -> Self {
        let mut buffer = SbeBuffer::with_capacity(capacity);

        // Reserve space for header
        buffer.write_bytes(&[0u8; SBE_HEADER_SIZE]).unwrap();

        // Reserve space for fixed fields block
        buffer
            .write_bytes(&vec![0u8; block_length as usize])
            .unwrap();

        Self {
            buffer,
            template_id,
            schema_version,
            block_length,
            variable_data_offset: SBE_HEADER_SIZE + block_length as usize,
            group_stack: SmallVec::new(),
        }
    }

    /// Write u8 field at specified offset in fixed fields block
    pub fn write_u8(&mut self, offset: usize, value: u8) -> SbeResult<()> {
        if offset >= self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        self.buffer.write_at_offset(SBE_HEADER_SIZE + offset, value)
    }

    /// Write u16 field at specified offset (little-endian)
    pub fn write_u16(&mut self, offset: usize, value: u16) -> SbeResult<()> {
        if offset + 2 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        let le_bytes = value.to_le_bytes();
        self.buffer
            .write_at_offset(SBE_HEADER_SIZE + offset, le_bytes)
    }

    /// Write u32 field at specified offset (little-endian)
    pub fn write_u32(&mut self, offset: usize, value: u32) -> SbeResult<()> {
        if offset + 4 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        let le_bytes = value.to_le_bytes();
        self.buffer
            .write_at_offset(SBE_HEADER_SIZE + offset, le_bytes)
    }

    /// Write u64 field at specified offset (little-endian)
    pub fn write_u64(&mut self, offset: usize, value: u64) -> SbeResult<()> {
        if offset + 8 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        let le_bytes = value.to_le_bytes();
        self.buffer
            .write_at_offset(SBE_HEADER_SIZE + offset, le_bytes)
    }

    /// Write f32 field at specified offset (little-endian)
    pub fn write_f32(&mut self, offset: usize, value: f32) -> SbeResult<()> {
        if offset + 4 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        let bytes = value.to_le_bytes();
        self.buffer.write_at_offset(SBE_HEADER_SIZE + offset, bytes)
    }

    /// Write byte array at specified offset
    pub fn write_bytes(&mut self, offset: usize, bytes: &[u8]) -> SbeResult<()> {
        if offset + bytes.len() > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        // Write bytes directly into the fixed fields section
        let start_offset = SBE_HEADER_SIZE + offset;
        self.buffer.as_mut_slice()[start_offset..start_offset + bytes.len()].copy_from_slice(bytes);

        Ok(())
    }

    /// Write fixed-length string field (padded with null bytes if necessary)
    pub fn write_string(&mut self, offset: usize, length: usize, value: &str) -> SbeResult<()> {
        if offset + length > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.block_length as usize,
            });
        }

        let value_bytes = value.as_bytes();
        if value_bytes.len() > length {
            return Err(SbeError::Custom {
                message: format!(
                    "String too long: {} bytes, field size is {}",
                    value_bytes.len(),
                    length
                ),
            });
        }

        // Write string data
        let start_offset = SBE_HEADER_SIZE + offset;
        let end_offset = start_offset + value_bytes.len();
        self.buffer.as_mut_slice()[start_offset..end_offset].copy_from_slice(value_bytes);

        // Pad remaining bytes with zeros
        if value_bytes.len() < length {
            let pad_start = end_offset;
            let pad_end = start_offset + length;
            self.buffer.as_mut_slice()[pad_start..pad_end].fill(0);
        }

        Ok(())
    }

    /// Begin encoding a repeating group
    pub fn begin_group(
        &mut self,
        offset: usize,
        block_length: u16,
    ) -> SbeResult<GroupEncoderBuilder> {
        if !self.group_stack.is_empty() {
            return Err(SbeError::Custom {
                message: "Cannot start group while another group is active".to_string(),
            });
        }

        // Ensure we have space at the current variable data offset
        self.buffer.reserve(6)?; // Group header size

        let group_encoder = GroupEncoder::new(self.variable_data_offset, block_length);

        Ok(GroupEncoderBuilder {
            encoder: self,
            group_encoder,
            offset,
        })
    }

    /// Write variable-length string data
    pub fn write_variable_string(&mut self, value: &str) -> SbeResult<()> {
        self.write_variable_bytes(value.as_bytes())
    }

    /// Write variable-length byte data
    pub fn write_variable_bytes(&mut self, bytes: &[u8]) -> SbeResult<()> {
        if bytes.len() > u16::MAX as usize {
            return Err(SbeError::Custom {
                message: format!("Variable data too large: {} bytes", bytes.len()),
            });
        }

        // Reserve space for length prefix + data
        self.buffer.reserve(2 + bytes.len())?;

        // Write length prefix (little-endian u16)
        let length = bytes.len() as u16;
        self.buffer.write_bytes(&length.to_le_bytes())?;

        // Write data
        self.buffer.write_bytes(bytes)?;

        Ok(())
    }

    /// Finalize the message and return the encoded bytes
    pub fn finalize(mut self) -> SbeResult<Vec<u8>> {
        // Write the message header
        let total_length = self.buffer.len() as u32;

        // Write header at the beginning
        self.buffer.write_at_offset(0, total_length.to_le_bytes())?;
        self.buffer
            .write_at_offset(4, self.template_id.to_le_bytes())?;
        self.buffer
            .write_at_offset(6, self.schema_version.to_le_bytes())?;

        Ok(self.buffer.as_slice().to_vec())
    }

    /// Get current message size
    pub fn current_size(&self) -> usize {
        self.buffer.len()
    }

    /// Get template ID
    pub fn template_id(&self) -> u16 {
        self.template_id
    }

    /// Get schema version
    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }
}

/// Builder for encoding repeating groups
#[allow(dead_code)]
pub struct GroupEncoderBuilder<'a> {
    encoder: &'a mut SbeEncoder,
    group_encoder: GroupEncoder,
    offset: usize,
}

impl<'a> GroupEncoderBuilder<'a> {
    /// Add an element to the group
    pub fn add_element(&mut self) -> SbeResult<GroupElementEncoder<'_>> {
        self.group_encoder.add_element()?;

        // Reserve space for the element
        self.encoder
            .buffer
            .reserve(self.group_encoder.block_length as usize)?;

        let element_offset = self.encoder.buffer.len();

        // Write zeros for the element block
        self.encoder
            .buffer
            .write_bytes(&vec![0u8; self.group_encoder.block_length as usize])?;

        Ok(GroupElementEncoder {
            encoder: &mut self.encoder.buffer,
            offset: element_offset,
            block_length: self.group_encoder.block_length,
        })
    }

    /// Finish encoding the group
    pub fn finish(self) -> SbeResult<()> {
        // Write group header at the beginning of variable data
        let group_start = self.group_encoder.start_offset;

        // Write element count
        self.encoder
            .buffer
            .write_at_offset(group_start, self.group_encoder.element_count.to_le_bytes())?;

        // Write block length
        self.encoder.buffer.write_at_offset(
            group_start + 4,
            self.group_encoder.block_length.to_le_bytes(),
        )?;

        // Update variable data offset for next variable data
        self.encoder.variable_data_offset = self.encoder.buffer.len();

        Ok(())
    }
}

/// Internal group encoder state
struct GroupEncoder {
    start_offset: usize,
    block_length: u16,
    element_count: u32,
}

impl GroupEncoder {
    fn new(start_offset: usize, block_length: u16) -> Self {
        Self {
            start_offset,
            block_length,
            element_count: 0,
        }
    }

    fn add_element(&mut self) -> SbeResult<()> {
        if self.element_count >= 10_000_000 {
            return Err(SbeError::GroupCountTooLarge {
                count: self.element_count + 1,
            });
        }
        self.element_count += 1;
        Ok(())
    }
}

/// Encoder for individual group elements
pub struct GroupElementEncoder<'a> {
    encoder: &'a mut SbeBuffer,
    offset: usize,
    block_length: u16,
}

impl<'a> GroupElementEncoder<'a> {
    /// Write u8 field in this group element
    pub fn write_u8(&mut self, field_offset: usize, value: u8) -> SbeResult<()> {
        if field_offset >= self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: field_offset,
                length: self.block_length as usize,
            });
        }

        self.encoder
            .write_at_offset(self.offset + field_offset, value)
    }

    /// Write u16 field in this group element
    pub fn write_u16(&mut self, field_offset: usize, value: u16) -> SbeResult<()> {
        if field_offset + 2 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: field_offset,
                length: self.block_length as usize,
            });
        }

        let le_bytes = value.to_le_bytes();
        self.encoder
            .write_at_offset(self.offset + field_offset, le_bytes)
    }

    /// Write u32 field in this group element
    pub fn write_u32(&mut self, field_offset: usize, value: u32) -> SbeResult<()> {
        if field_offset + 4 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: field_offset,
                length: self.block_length as usize,
            });
        }

        let le_bytes = value.to_le_bytes();
        self.encoder
            .write_at_offset(self.offset + field_offset, le_bytes)
    }

    /// Write u64 field in this group element
    pub fn write_u64(&mut self, field_offset: usize, value: u64) -> SbeResult<()> {
        if field_offset + 8 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: field_offset,
                length: self.block_length as usize,
            });
        }

        let le_bytes = value.to_le_bytes();
        self.encoder
            .write_at_offset(self.offset + field_offset, le_bytes)
    }

    /// Write f32 field in this group element
    pub fn write_f32(&mut self, field_offset: usize, value: f32) -> SbeResult<()> {
        if field_offset + 4 > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: field_offset,
                length: self.block_length as usize,
            });
        }

        let bytes = value.to_le_bytes();
        self.encoder
            .write_at_offset(self.offset + field_offset, bytes)
    }

    /// Write string field in this group element
    pub fn write_string(
        &mut self,
        field_offset: usize,
        length: usize,
        value: &str,
    ) -> SbeResult<()> {
        if field_offset + length > self.block_length as usize {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: field_offset,
                length: self.block_length as usize,
            });
        }

        let value_bytes = value.as_bytes();
        if value_bytes.len() > length {
            return Err(SbeError::Custom {
                message: format!(
                    "String too long: {} bytes, field size is {}",
                    value_bytes.len(),
                    length
                ),
            });
        }

        // Write string data
        let start_offset = self.offset + field_offset;
        let end_offset = start_offset + value_bytes.len();
        self.encoder.as_mut_slice()[start_offset..end_offset].copy_from_slice(value_bytes);

        // Pad remaining bytes with zeros
        if value_bytes.len() < length {
            let pad_start = end_offset;
            let pad_end = start_offset + length;
            self.encoder.as_mut_slice()[pad_start..pad_end].fill(0);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decoder::SbeDecoder;

    #[test]
    fn test_encoder_creation() {
        let encoder = SbeEncoder::new(1, 0, 32);
        assert_eq!(encoder.template_id(), 1);
        assert_eq!(encoder.schema_version(), 0);
        assert!(encoder.current_size() >= SBE_HEADER_SIZE + 32);
    }

    #[test]
    fn test_field_encoding() {
        let mut encoder = SbeEncoder::new(1, 0, 16);

        encoder.write_u32(0, 42).unwrap();
        encoder.write_u64(4, 1234567890).unwrap();
        encoder.write_u16(12, 999).unwrap();

        let message = encoder.finalize().unwrap();

        // Verify by decoding
        let decoder = SbeDecoder::new(&message).unwrap();
        assert_eq!(decoder.template_id(), 1);
        assert_eq!(decoder.read_u32(0).unwrap(), 42);
        assert_eq!(decoder.read_u64(4).unwrap(), 1234567890);
        assert_eq!(decoder.read_u16(12).unwrap(), 999);
    }

    #[test]
    fn test_string_encoding() {
        let mut encoder = SbeEncoder::new(1, 0, 16);

        encoder.write_string(0, 8, "BTCUSDT").unwrap();
        encoder.write_string(8, 8, "BUY").unwrap();

        let message = encoder.finalize().unwrap();

        // Verify by decoding
        let decoder = SbeDecoder::new(&message).unwrap();
        assert_eq!(
            decoder.read_string(0, 8).unwrap().trim_end_matches('\0'),
            "BTCUSDT"
        );
        assert_eq!(
            decoder.read_string(8, 8).unwrap().trim_end_matches('\0'),
            "BUY"
        );
    }

    #[test]
    fn test_variable_data_encoding() {
        let mut encoder = SbeEncoder::new(1, 0, 8);

        encoder.write_u64(0, 12345).unwrap();
        encoder.write_variable_string("Hello SBE").unwrap();
        encoder.write_variable_string("World").unwrap();

        let message = encoder.finalize().unwrap();

        // Calculate expected size: header (8) + fixed fields (8) + var data
        // Variable data: "Hello SBE" (9 chars + 2 byte length) + "World" (5 chars + 2 byte length)
        let expected_min_size = SBE_HEADER_SIZE + 8 + (2 + 9) + (2 + 5);
        assert!(
            message.len() >= expected_min_size,
            "Message length {} should be at least {}",
            message.len(),
            expected_min_size
        );

        // Verify fixed field
        let decoder = SbeDecoder::new(&message).unwrap();
        assert_eq!(decoder.read_u64(0).unwrap(), 12345);
    }

    #[test]
    fn test_round_trip_encoding_decoding() {
        // Test a complete encode/decode cycle
        let mut encoder = SbeEncoder::new(123, 1, 24);

        // Write various field types
        encoder.write_u8(0, 255).unwrap();
        encoder.write_u16(1, 65535).unwrap();
        encoder.write_u32(3, 4294967295).unwrap();
        encoder.write_u64(7, 18446744073709551615).unwrap();
        encoder.write_f32(15, 3.14159).unwrap();
        encoder.write_string(19, 5, "TEST").unwrap();

        let message = encoder.finalize().unwrap();

        // Decode and verify
        let decoder = SbeDecoder::new(&message).unwrap();
        assert_eq!(decoder.template_id(), 123);
        assert_eq!(decoder.schema_version(), 1);
        assert_eq!(decoder.read_u8(0).unwrap(), 255);
        assert_eq!(decoder.read_u16(1).unwrap(), 65535);
        assert_eq!(decoder.read_u32(3).unwrap(), 4294967295);
        assert_eq!(decoder.read_u64(7).unwrap(), 18446744073709551615);
        assert!((decoder.read_f32(15).unwrap() - 3.14159).abs() < 0.001);
        assert_eq!(
            decoder.read_string(19, 5).unwrap().trim_end_matches('\0'),
            "TEST"
        );
    }
}
