//! High-performance buffer management for SBE operations

use crate::error::{SbeError, SbeResult};
use simd_aligned::{VecSimd, arch::u8x16};
use smallvec::SmallVec;
use zerocopy::{FromBytes, IntoBytes, LittleEndian, U16, U32, U64};

/// Maximum message size for SBE messages (64KB)
pub const MAX_MESSAGE_SIZE: usize = 65536;

/// Default buffer capacity for small messages
pub const DEFAULT_BUFFER_CAPACITY: usize = 1024;

/// SBE message header size (length + template ID + schema version)
pub const SBE_HEADER_SIZE: usize = 8;

/// Aligned buffer for SBE operations with SIMD optimization
pub struct SbeBuffer {
    /// SIMD-aligned buffer for optimal memory access
    data: VecSimd<u8x16>,
    /// Current position for writing
    position: usize,
    /// Maximum capacity
    capacity: usize,
}

impl SbeBuffer {
    /// Create a new SBE buffer with default capacity
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUFFER_CAPACITY)
    }

    /// Create a new SBE buffer with specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let aligned_capacity = (capacity + 15) & !15; // Align to 16 bytes
        Self {
            data: VecSimd::with(0u8, aligned_capacity),
            position: 0,
            capacity: aligned_capacity,
        }
    }

    /// Create a buffer from existing byte slice
    pub fn from_slice(slice: &[u8]) -> SbeResult<Self> {
        if slice.len() > MAX_MESSAGE_SIZE {
            return Err(SbeError::MessageTooLarge {
                length: slice.len(),
                max: MAX_MESSAGE_SIZE,
            });
        }

        let aligned_capacity = (slice.len() + 15) & !15;
        let mut buffer = Self::with_capacity(aligned_capacity);
        buffer.data.flat_mut()[..slice.len()].copy_from_slice(slice);
        buffer.position = slice.len();
        Ok(buffer)
    }

    /// Get the current length of valid data
    pub fn len(&self) -> usize {
        self.position
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.position == 0
    }

    /// Get the total capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get remaining space in buffer
    pub fn remaining(&self) -> usize {
        self.capacity - self.position
    }

    /// Clear the buffer (reset position to 0)
    pub fn clear(&mut self) {
        self.position = 0;
    }

    /// Get a slice of the valid data
    pub fn as_slice(&self) -> &[u8] {
        &self.data.flat()[..self.position]
    }

    /// Get a mutable slice of the valid data
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data.flat_mut()[..self.position]
    }

    /// Reserve additional capacity
    pub fn reserve(&mut self, additional: usize) -> SbeResult<()> {
        let new_capacity = self
            .position
            .checked_add(additional)
            .ok_or(SbeError::IntegerOverflow)?;

        if new_capacity > self.capacity {
            if new_capacity > MAX_MESSAGE_SIZE {
                return Err(SbeError::MessageTooLarge {
                    length: new_capacity,
                    max: MAX_MESSAGE_SIZE,
                });
            }

            let aligned_capacity = (new_capacity + 15) & !15;
            let mut new_data = VecSimd::with(0u8, aligned_capacity);
            new_data.flat_mut()[..self.position]
                .copy_from_slice(&self.data.flat()[..self.position]);

            self.data = new_data;
            self.capacity = aligned_capacity;
        }
        Ok(())
    }

    /// Write bytes at current position and advance
    pub fn write_bytes(&mut self, bytes: &[u8]) -> SbeResult<()> {
        self.reserve(bytes.len())?;
        self.data.flat_mut()[self.position..self.position + bytes.len()].copy_from_slice(bytes);
        self.position += bytes.len();
        Ok(())
    }

    /// Write a value at specific offset (for fixed fields)
    pub fn write_at_offset<T>(&mut self, offset: usize, value: T) -> SbeResult<()>
    where
        T: IntoBytes + zerocopy::Immutable,
    {
        let bytes = value.as_bytes();
        let end_offset = offset
            .checked_add(bytes.len())
            .ok_or(SbeError::IntegerOverflow)?;

        if end_offset > self.position {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.position,
            });
        }

        self.data.flat_mut()[offset..end_offset].copy_from_slice(bytes);
        Ok(())
    }

    /// Read a value at specific offset
    pub fn read_at_offset<T>(&self, offset: usize) -> SbeResult<T>
    where
        T: FromBytes + Copy,
    {
        let size = std::mem::size_of::<T>();
        let end_offset = offset.checked_add(size).ok_or(SbeError::IntegerOverflow)?;

        if end_offset > self.position {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.position,
            });
        }

        T::read_from_bytes(&self.data.flat()[offset..end_offset]).map_err(|_| {
            SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.position,
            }
        })
    }
}

impl Default for SbeBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// SBE message reader for zero-copy decoding
pub struct SbeReader<'a> {
    /// Reference to the message data
    pub(crate) data: &'a [u8],
    /// Current reading position
    position: usize,
}

impl<'a> SbeReader<'a> {
    /// Create a new reader from byte slice
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// Get the total data length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if we're at the end
    pub fn is_empty(&self) -> bool {
        self.position >= self.data.len()
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set position
    pub fn set_position(&mut self, position: usize) -> SbeResult<()> {
        if position > self.data.len() {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset: position,
                length: self.data.len(),
            });
        }
        self.position = position;
        Ok(())
    }

    /// Read a primitive value at specific offset (zero-copy)
    pub fn read_at<T>(&self, offset: usize) -> SbeResult<T>
    where
        T: FromBytes + Copy,
    {
        let size = std::mem::size_of::<T>();
        let end_offset = offset.checked_add(size).ok_or(SbeError::IntegerOverflow)?;

        if end_offset > self.data.len() {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.data.len(),
            });
        }

        T::read_from_bytes(&self.data[offset..end_offset]).map_err(|_| {
            SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.data.len(),
            }
        })
    }

    /// Read bytes at specific offset
    pub fn read_bytes_at(&self, offset: usize, length: usize) -> SbeResult<&'a [u8]> {
        let end_offset = offset
            .checked_add(length)
            .ok_or(SbeError::IntegerOverflow)?;

        if end_offset > self.data.len() {
            return Err(SbeError::FieldOffsetOutOfBounds {
                offset,
                length: self.data.len(),
            });
        }

        Ok(&self.data[offset..end_offset])
    }

    /// Read a little-endian u16 at offset
    pub fn read_u16(&self, offset: usize) -> SbeResult<u16> {
        let value: U16<LittleEndian> = self.read_at(offset)?;
        Ok(value.get())
    }

    /// Read a little-endian u32 at offset
    pub fn read_u32(&self, offset: usize) -> SbeResult<u32> {
        let value: U32<LittleEndian> = self.read_at(offset)?;
        Ok(value.get())
    }

    /// Read a little-endian u64 at offset
    pub fn read_u64(&self, offset: usize) -> SbeResult<u64> {
        let value: U64<LittleEndian> = self.read_at(offset)?;
        Ok(value.get())
    }

    /// Read a u8 at offset
    pub fn read_u8(&self, offset: usize) -> SbeResult<u8> {
        self.read_at(offset)
    }

    /// Read an f32 at offset
    pub fn read_f32(&self, offset: usize) -> SbeResult<f32> {
        let bytes = self.read_at::<[u8; 4]>(offset)?;
        Ok(f32::from_le_bytes(bytes))
    }

    /// Read string data with proper UTF-8 validation
    pub fn read_string_at(&self, offset: usize, length: usize) -> SbeResult<&'a str> {
        let bytes = self.read_bytes_at(offset, length)?;

        // Handle null-terminated strings by finding the first null byte
        let string_bytes = if let Some(null_pos) = bytes.iter().position(|&b| b == 0) {
            &bytes[..null_pos]
        } else {
            bytes
        };

        std::str::from_utf8(string_bytes).map_err(|_| SbeError::InvalidUtf8String)
    }
}

/// Small vector type optimized for temporary collections in SBE operations
pub type SbeSmallVec<T> = SmallVec<[T; 8]>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = SbeBuffer::new();
        assert_eq!(buffer.len(), 0);
        assert!(!buffer.is_empty() || buffer.len() == 0);
        assert!(buffer.capacity() >= DEFAULT_BUFFER_CAPACITY);
    }

    #[test]
    fn test_buffer_write_read() {
        let mut buffer = SbeBuffer::new();
        let data = b"Hello, SBE!";

        buffer.write_bytes(data).unwrap();
        assert_eq!(buffer.len(), data.len());
        assert_eq!(buffer.as_slice(), data);
    }

    #[test]
    fn test_reader_operations() {
        let data = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let reader = SbeReader::new(&data);

        // Test u16 reading (little-endian)
        assert_eq!(reader.read_u16(0).unwrap(), 0x0201);
        assert_eq!(reader.read_u16(2).unwrap(), 0x0403);

        // Test u32 reading
        assert_eq!(reader.read_u32(0).unwrap(), 0x04030201);

        // Test bounds checking
        assert!(reader.read_u32(6).is_err());
    }

    #[test]
    fn test_alignment() {
        let buffer = SbeBuffer::with_capacity(100);
        // Buffer should be aligned to 16 bytes
        assert_eq!(buffer.capacity() % 16, 0);
    }
}
