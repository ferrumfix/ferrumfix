//! Zero-copy SBE message decoder

use crate::buffer::{SBE_HEADER_SIZE, SbeReader};
use crate::error::{SbeError, SbeResult};

/// SBE message header structure
#[derive(Debug, Clone, Copy)]
pub struct SbeHeader {
    /// Message length including header
    pub message_length: u32,
    /// Template ID for message type
    pub template_id: u16,
    /// Schema version
    pub schema_version: u16,
}

impl SbeHeader {
    /// Parse header from bytes
    pub fn from_bytes(data: &[u8]) -> SbeResult<Self> {
        if data.len() < SBE_HEADER_SIZE {
            return Err(SbeError::BufferTooSmall {
                need: SBE_HEADER_SIZE,
                have: data.len(),
            });
        }

        let reader = SbeReader::new(data);
        let message_length = reader.read_u32(0)?;
        let template_id = reader.read_u16(4)?;
        let schema_version = reader.read_u16(6)?;

        Ok(Self {
            message_length,
            template_id,
            schema_version,
        })
    }

    /// Get the size of the message body (excluding header)
    pub fn body_length(&self) -> usize {
        self.message_length.saturating_sub(SBE_HEADER_SIZE as u32) as usize
    }
}

/// Zero-copy SBE message decoder
pub struct SbeDecoder<'a> {
    /// Reader for message data
    reader: SbeReader<'a>,
    /// Parsed message header
    header: SbeHeader,
    /// Offset to start of message body (after header)
    body_offset: usize,
}

impl<'a> SbeDecoder<'a> {
    /// Create a new decoder from message bytes
    pub fn new(data: &'a [u8]) -> SbeResult<Self> {
        let header = SbeHeader::from_bytes(data)?;

        // Validate message length
        if header.message_length as usize > data.len() {
            return Err(SbeError::InvalidMessageLength {
                length: header.message_length as u16,
            });
        }

        let reader = SbeReader::new(&data[..header.message_length as usize]);

        Ok(Self {
            reader,
            header,
            body_offset: SBE_HEADER_SIZE,
        })
    }

    /// Get the message header
    pub fn header(&self) -> &SbeHeader {
        &self.header
    }

    /// Get template ID
    pub fn template_id(&self) -> u16 {
        self.header.template_id
    }

    /// Get schema version
    pub fn schema_version(&self) -> u16 {
        self.header.schema_version
    }

    /// Verify template ID matches expected value
    pub fn verify_template_id(&self, expected: u16) -> SbeResult<()> {
        if self.header.template_id != expected {
            return Err(SbeError::InvalidTemplateId {
                expected,
                found: self.header.template_id,
            });
        }
        Ok(())
    }

    /// Verify schema version matches expected value
    pub fn verify_schema_version(&self, expected: u16) -> SbeResult<()> {
        if self.header.schema_version != expected {
            return Err(SbeError::InvalidSchemaVersion {
                expected,
                found: self.header.schema_version,
            });
        }
        Ok(())
    }

    /// Read u8 field at body offset
    pub fn read_u8(&self, offset: usize) -> SbeResult<u8> {
        self.reader.read_u8(self.body_offset + offset)
    }

    /// Read u16 field at body offset (little-endian)
    pub fn read_u16(&self, offset: usize) -> SbeResult<u16> {
        self.reader.read_u16(self.body_offset + offset)
    }

    /// Read u32 field at body offset (little-endian)
    pub fn read_u32(&self, offset: usize) -> SbeResult<u32> {
        self.reader.read_u32(self.body_offset + offset)
    }

    /// Read u64 field at body offset (little-endian)
    pub fn read_u64(&self, offset: usize) -> SbeResult<u64> {
        self.reader.read_u64(self.body_offset + offset)
    }

    /// Read f32 field at body offset (little-endian)
    pub fn read_f32(&self, offset: usize) -> SbeResult<f32> {
        self.reader.read_f32(self.body_offset + offset)
    }

    /// Read byte array field at body offset
    pub fn read_bytes(&self, offset: usize, length: usize) -> SbeResult<&'a [u8]> {
        self.reader.read_bytes_at(self.body_offset + offset, length)
    }

    /// Read fixed-length string field at body offset
    pub fn read_string(&self, offset: usize, length: usize) -> SbeResult<&'a str> {
        self.reader
            .read_string_at(self.body_offset + offset, length)
    }

    /// Read array of primitive values at body offset (returns bytes for manual casting)
    pub fn read_array_bytes(
        &self,
        offset: usize,
        element_size: usize,
        count: usize,
    ) -> SbeResult<&'a [u8]> {
        let total_size = element_size
            .checked_mul(count)
            .ok_or(SbeError::IntegerOverflow)?;
        self.read_bytes(offset, total_size)
    }

    /// Navigate to group at specified offset and get group iterator
    pub fn read_group(&self, offset: usize) -> SbeResult<SbeGroupIterator<'a>> {
        // We need to pass a reference with the same lifetime as the decoder
        let reader_ref: &'a SbeReader<'a> = unsafe {
            // SAFETY: The SbeDecoder lifetime 'a is tied to the data lifetime,
            // so extending the reader reference to 'a is safe
            std::mem::transmute(&self.reader)
        };
        SbeGroupIterator::new(reader_ref, self.body_offset + offset)
    }

    /// Get the underlying reader for advanced operations
    pub fn reader(&self) -> &SbeReader<'a> {
        &self.reader
    }

    /// Get body offset for manual calculations
    pub fn body_offset(&self) -> usize {
        self.body_offset
    }
}

/// Iterator for SBE repeating groups
pub struct SbeGroupIterator<'a> {
    /// Reader for group data
    reader: &'a SbeReader<'a>,
    /// Starting offset of group
    start_offset: usize,
    /// Number of elements in group
    element_count: u32,
    /// Size of each element block
    block_length: u16,
    /// Current element index
    current_index: u32,
    /// Current position in data
    current_offset: usize,
}

impl<'a> SbeGroupIterator<'a> {
    /// Create a new group iterator
    pub fn new(reader: &'a SbeReader<'a>, offset: usize) -> SbeResult<Self> {
        // Read group header: element count (u32) + block length (u16)
        let element_count = reader.read_u32(offset)?;
        let block_length = reader.read_u16(offset + 4)?;

        // Validate group parameters
        if element_count > 10_000_000 {
            return Err(SbeError::GroupCountTooLarge {
                count: element_count,
            });
        }

        if block_length == 0 {
            return Err(SbeError::InvalidGroupBlockLength {
                length: block_length,
            });
        }

        let group_header_size = 6; // 4 bytes count + 2 bytes block length

        Ok(Self {
            reader,
            start_offset: offset,
            element_count,
            block_length,
            current_index: 0,
            current_offset: offset + group_header_size,
        })
    }

    /// Get the number of elements in the group
    pub fn count(&self) -> u32 {
        self.element_count
    }

    /// Get the block length for each element
    pub fn block_length(&self) -> u16 {
        self.block_length
    }

    /// Get the current element index (0-based)
    pub fn current_index(&self) -> u32 {
        self.current_index
    }

    /// Check if there are more elements
    pub fn has_next(&self) -> bool {
        self.current_index < self.element_count
    }

    /// Move to the next element and return a decoder for it
    pub fn next_element(&mut self) -> SbeResult<Option<SbeGroupElement<'a>>> {
        if !self.has_next() {
            return Ok(None);
        }

        let element = SbeGroupElement::new(self.reader, self.current_offset, self.block_length)?;

        // Move to next element
        self.current_index += 1;
        self.current_offset += self.block_length as usize;

        Ok(Some(element))
    }

    /// Skip to a specific element index
    pub fn seek_to(&mut self, index: u32) -> SbeResult<()> {
        if index >= self.element_count {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: index as usize,
                length: self.element_count as usize,
            });
        }

        self.current_index = index;
        self.current_offset = self.start_offset + 6 + (index as usize * self.block_length as usize);
        Ok(())
    }

    /// Get the total size of the group including header
    pub fn total_size(&self) -> usize {
        6 + (self.element_count as usize * self.block_length as usize)
    }
}

/// Decoder for a single group element
#[allow(dead_code)]
pub struct SbeGroupElement<'a> {
    /// Reader for element data
    reader: &'a SbeReader<'a>,
    /// Offset to start of this element
    offset: usize,
    /// Size of this element block
    block_length: u16,
}

impl<'a> SbeGroupElement<'a> {
    /// Create a new group element decoder
    pub fn new(reader: &'a SbeReader<'a>, offset: usize, block_length: u16) -> SbeResult<Self> {
        // Validate that we have enough data for this element
        let end_offset = offset
            .checked_add(block_length as usize)
            .ok_or(SbeError::IntegerOverflow)?;

        if end_offset > reader.len() {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: reader.len(),
            });
        }

        Ok(Self {
            reader,
            offset,
            block_length,
        })
    }

    /// Read u8 field at element offset
    pub fn read_u8(&self, field_offset: usize) -> SbeResult<u8> {
        self.reader.read_u8(self.offset + field_offset)
    }

    /// Read u16 field at element offset
    pub fn read_u16(&self, field_offset: usize) -> SbeResult<u16> {
        self.reader.read_u16(self.offset + field_offset)
    }

    /// Read u32 field at element offset
    pub fn read_u32(&self, field_offset: usize) -> SbeResult<u32> {
        self.reader.read_u32(self.offset + field_offset)
    }

    /// Read u64 field at element offset
    pub fn read_u64(&self, field_offset: usize) -> SbeResult<u64> {
        self.reader.read_u64(self.offset + field_offset)
    }

    /// Read f32 field at element offset
    pub fn read_f32(&self, field_offset: usize) -> SbeResult<f32> {
        self.reader.read_f32(self.offset + field_offset)
    }

    /// Read string field at element offset
    pub fn read_string(&self, field_offset: usize, length: usize) -> SbeResult<&'a str> {
        self.reader
            .read_string_at(self.offset + field_offset, length)
    }

    /// Read bytes at element offset
    pub fn read_bytes(&self, field_offset: usize, length: usize) -> SbeResult<&'a [u8]> {
        self.reader
            .read_bytes_at(self.offset + field_offset, length)
    }

    /// Get nested group within this element
    pub fn read_nested_group(&self, field_offset: usize) -> SbeResult<SbeGroupIterator<'a>> {
        SbeGroupIterator::new(self.reader, self.offset + field_offset)
    }
}

/// Variable-length data accessor for SBE messages
pub struct SbeVariableData<'a> {
    /// Reference to message data
    data: &'a [u8],
    /// Current offset in data
    offset: usize,
}

impl<'a> SbeVariableData<'a> {
    /// Create a new variable data accessor
    pub fn new(decoder: &SbeDecoder<'a>, start_offset: usize) -> Self {
        Self {
            data: decoder.reader.data,
            offset: decoder.body_offset + start_offset,
        }
    }

    /// Read next variable-length field
    pub fn read_next_string(&mut self) -> SbeResult<&'a str> {
        // Variable data format: 2-byte length + data
        if self.offset + 2 > self.data.len() {
            return Err(SbeError::InvalidVariableDataOffset {
                offset: self.offset,
            });
        }

        let length =
            u16::from_le_bytes([self.data[self.offset], self.data[self.offset + 1]]) as usize;

        let data_start = self.offset + 2;
        let data_end = data_start + length;

        if data_end > self.data.len() {
            return Err(SbeError::InvalidVariableDataOffset { offset: data_end });
        }

        let string_data = &self.data[data_start..data_end];
        let result = std::str::from_utf8(string_data).map_err(|_| SbeError::InvalidUtf8String)?;

        self.offset = data_end;
        Ok(result)
    }

    /// Read next variable-length bytes
    pub fn read_next_bytes(&mut self) -> SbeResult<&'a [u8]> {
        // Variable data format: 2-byte length + data
        if self.offset + 2 > self.data.len() {
            return Err(SbeError::InvalidVariableDataOffset {
                offset: self.offset,
            });
        }

        let length =
            u16::from_le_bytes([self.data[self.offset], self.data[self.offset + 1]]) as usize;

        let data_start = self.offset + 2;
        let data_end = data_start + length;

        if data_end > self.data.len() {
            return Err(SbeError::InvalidVariableDataOffset { offset: data_end });
        }

        let result = &self.data[data_start..data_end];
        self.offset = data_end;
        Ok(result)
    }

    /// Check if there's more variable data
    pub fn has_more(&self) -> bool {
        self.offset < self.data.len()
    }

    /// Get current offset
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::SbeBuffer;

    #[test]
    fn test_header_parsing() {
        let mut buffer = SbeBuffer::new();
        // Write header: length=100, template_id=1, schema_version=0
        buffer.write_bytes(&100u32.to_le_bytes()).unwrap();
        buffer.write_bytes(&1u16.to_le_bytes()).unwrap();
        buffer.write_bytes(&0u16.to_le_bytes()).unwrap();

        let header = SbeHeader::from_bytes(buffer.as_slice()).unwrap();
        assert_eq!(header.message_length, 100);
        assert_eq!(header.template_id, 1);
        assert_eq!(header.schema_version, 0);
    }

    #[test]
    fn test_decoder_creation() {
        let mut buffer = SbeBuffer::new();
        // Write minimal valid message
        buffer.write_bytes(&16u32.to_le_bytes()).unwrap(); // length
        buffer.write_bytes(&1u16.to_le_bytes()).unwrap(); // template_id
        buffer.write_bytes(&0u16.to_le_bytes()).unwrap(); // schema_version
        buffer.write_bytes(&[0u8; 8]).unwrap(); // body data

        let decoder = SbeDecoder::new(buffer.as_slice()).unwrap();
        assert_eq!(decoder.template_id(), 1);
        assert_eq!(decoder.schema_version(), 0);
    }

    #[test]
    fn test_field_reading() {
        let mut buffer = SbeBuffer::new();
        // Write header
        buffer.write_bytes(&20u32.to_le_bytes()).unwrap();
        buffer.write_bytes(&1u16.to_le_bytes()).unwrap();
        buffer.write_bytes(&0u16.to_le_bytes()).unwrap();
        // Write test data
        buffer.write_bytes(&42u32.to_le_bytes()).unwrap();
        buffer.write_bytes(&1234u64.to_le_bytes()).unwrap();

        let decoder = SbeDecoder::new(buffer.as_slice()).unwrap();
        assert_eq!(decoder.read_u32(0).unwrap(), 42);
        assert_eq!(decoder.read_u64(4).unwrap(), 1234);
    }
}
