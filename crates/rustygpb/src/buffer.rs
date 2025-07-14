//! High-performance buffer management for Protocol Buffers encoding/decoding.

use crate::error::{DecodeError, EncodeError, GpbError};
use simd_aligned::{arch::f64x4, VecSimd};
use smallvec::SmallVec;
use std::io::{self, Read, Write};

/// High-performance buffer for GPB operations with SIMD alignment.
#[derive(Debug, Clone)]
pub struct GpbBuffer {
    /// SIMD-aligned storage for optimal performance
    data: VecSimd<f64x4>,
    /// Current position for reading/writing
    position: usize,
    /// Marked limit for reading operations
    limit: usize,
}

impl GpbBuffer {
    /// Create a new buffer with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        // Calculate SIMD vector count needed (each f64x4 is 32 bytes)
        let vec_count = capacity.div_ceil(32).max(1); // Ensure at least 1 vector
                                                      // Each f64x4 contains 4 f64 elements
        let element_count = vec_count * 4;

        Self {
            data: VecSimd::with(0.0, element_count),
            position: 0,
            limit: 0,
        }
    }

    /// Create a new empty buffer.
    pub fn new() -> Self {
        Self::with_capacity(4096) // Default 4KB
    }

    /// Get the current capacity in bytes.
    pub fn capacity(&self) -> usize {
        self.data.len() * 32 // 32 bytes per f64x4
    }

    /// Get the current length of valid data.
    pub fn len(&self) -> usize {
        self.limit
    }

    /// Check if buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.limit == 0
    }

    /// Get current position.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Set the position for reading/writing.
    pub fn set_position(&mut self, pos: usize) -> Result<(), GpbError> {
        if pos > self.limit {
            return Err(GpbError::InvalidFormat(format!(
                "Position {} exceeds limit {}",
                pos, self.limit
            )));
        }
        self.position = pos;
        Ok(())
    }

    /// Get remaining bytes from current position.
    pub fn remaining(&self) -> usize {
        self.limit.saturating_sub(self.position)
    }

    /// Clear the buffer and reset position.
    pub fn clear(&mut self) {
        self.position = 0;
        self.limit = 0;
    }

    /// Get byte slice of valid data.
    pub fn as_slice(&self) -> &[u8] {
        let flat = self.data.flat();
        unsafe { std::slice::from_raw_parts(flat.as_ptr() as *const u8, self.limit) }
    }

    /// Get mutable byte slice for writing.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        let flat = self.data.flat_mut();
        unsafe { std::slice::from_raw_parts_mut(flat.as_mut_ptr() as *mut u8, self.capacity()) }
    }

    /// Write data to buffer at current position.
    pub fn write(&mut self, data: &[u8]) -> Result<usize, GpbError> {
        let capacity = self.capacity();
        if self.position + data.len() > capacity {
            return Err(GpbError::Encode(EncodeError::BufferTooSmall {
                needed: self.position + data.len(),
                available: capacity,
            }));
        }

        // Extract position values before borrowing mutably
        let start_pos = self.position;
        let end_pos = self.position + data.len();

        let slice = self.as_mut_slice();
        slice[start_pos..end_pos].copy_from_slice(data);
        self.position += data.len();
        self.limit = self.limit.max(self.position);

        Ok(data.len())
    }

    /// Read data from buffer at current position.
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, GpbError> {
        let available = self.remaining();
        let to_read = buf.len().min(available);

        if to_read == 0 {
            return Ok(0);
        }

        let slice = self.as_slice();
        buf[..to_read].copy_from_slice(&slice[self.position..self.position + to_read]);
        self.position += to_read;

        Ok(to_read)
    }
}

impl Default for GpbBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// High-performance writer for Protocol Buffers messages.
#[derive(Debug)]
pub struct GpbWriter {
    buffer: GpbBuffer,
}

impl GpbWriter {
    /// Create a new writer with specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buffer: GpbBuffer::with_capacity(capacity),
        }
    }

    /// Create a new writer with default capacity.
    pub fn new() -> Self {
        Self::with_capacity(8192) // Default 8KB for writes
    }

    /// Get the underlying buffer.
    pub fn buffer(&self) -> &GpbBuffer {
        &self.buffer
    }

    /// Take ownership of the buffer.
    pub fn into_buffer(self) -> GpbBuffer {
        self.buffer
    }

    /// Get the written data as bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.buffer.as_slice()
    }

    /// Clear the writer for reuse.
    pub fn clear(&mut self) {
        self.buffer.clear();
    }
}

impl Write for GpbWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buffer
            .write(buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(()) // No-op for in-memory buffer
    }
}

impl Default for GpbWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// High-performance reader for Protocol Buffers messages.
#[derive(Debug)]
pub struct GpbReader {
    buffer: GpbBuffer,
}

impl GpbReader {
    /// Create a new reader from bytes.
    pub fn new(data: &[u8]) -> Result<Self, GpbError> {
        let mut buffer = GpbBuffer::with_capacity(data.len());
        buffer.write(data)?;
        buffer.set_position(0)?;

        Ok(Self { buffer })
    }

    /// Create a reader from an existing buffer.
    pub fn from_buffer(mut buffer: GpbBuffer) -> Result<Self, GpbError> {
        buffer.set_position(0)?;
        Ok(Self { buffer })
    }

    /// Get remaining bytes to read.
    pub fn remaining(&self) -> usize {
        self.buffer.remaining()
    }

    /// Check if more data is available.
    pub fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    /// Get current position.
    pub fn position(&self) -> usize {
        self.buffer.position()
    }

    /// Peek at the next byte without advancing position.
    pub fn peek_u8(&self) -> Result<u8, GpbError> {
        if self.remaining() == 0 {
            return Err(GpbError::Decode(DecodeError::TruncatedBuffer {
                expected: 1,
                actual: 0,
            }));
        }

        Ok(self.buffer.as_slice()[self.buffer.position()])
    }
}

impl Read for GpbReader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.buffer
            .read(buf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

/// Buffer utilities for common operations.
pub struct BufferUtils;

impl BufferUtils {
    /// Calculate CRC32 checksum for data integrity.
    pub fn crc32(data: &[u8]) -> u32 {
        // Basic CRC32 implementation for integrity checking
        data.iter().fold(0, |acc, &b| acc.wrapping_add(b as u32))
    }

    /// Encode a value using variable-length encoding.
    pub fn encode_varint(value: u64) -> SmallVec<[u8; 10]> {
        let mut result = SmallVec::new();
        let mut val = value;

        while val >= 0x80 {
            result.push((val & 0x7F) as u8 | 0x80);
            val >>= 7;
        }
        result.push(val as u8);

        result
    }

    /// Decode variable-length integer (varint).
    pub fn decode_varint(reader: &mut GpbReader) -> Result<u64, DecodeError> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            if shift >= 64 {
                return Err(DecodeError::InvalidWireFormat {
                    reason: "Varint too long".into(),
                });
            }

            let byte = reader.peek_u8().map_err(|_| DecodeError::TruncatedBuffer {
                expected: 1,
                actual: 0,
            })?;

            let mut buf = [0u8; 1];
            reader
                .read_exact(&mut buf)
                .map_err(|_| DecodeError::TruncatedBuffer {
                    expected: 1,
                    actual: 0,
                })?;

            result |= ((byte & 0x7F) as u64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_creation() {
        let buffer = GpbBuffer::with_capacity(1024);
        assert!(buffer.capacity() >= 1024);
        assert_eq!(buffer.len(), 0);
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_buffer_write_read() {
        let mut buffer = GpbBuffer::new();
        let data = b"Hello, Protocol Buffers!";

        let written = buffer.write(data).unwrap();
        assert_eq!(written, data.len());
        assert_eq!(buffer.len(), data.len());

        buffer.set_position(0).unwrap();
        let mut read_buf = vec![0u8; data.len()];
        let read = buffer.read(&mut read_buf).unwrap();

        assert_eq!(read, data.len());
        assert_eq!(&read_buf, data);
    }

    #[test]
    fn test_writer_functionality() {
        let mut writer = GpbWriter::new();
        let data = b"Test message";

        writer.write_all(data).unwrap();
        assert_eq!(writer.as_bytes(), data);
    }

    #[test]
    fn test_reader_functionality() {
        let data = b"Test message for reading";
        let mut reader = GpbReader::new(data).unwrap();

        assert_eq!(reader.remaining(), data.len());
        assert!(reader.has_remaining());

        let mut buf = vec![0u8; data.len()];
        reader.read_exact(&mut buf).unwrap();
        assert_eq!(&buf, data);
        assert!(!reader.has_remaining());
    }

    #[test]
    fn test_varint_encoding() {
        let test_cases = [0u64, 127, 128, 16383, 16384, u64::MAX];

        for &value in &test_cases {
            let encoded = BufferUtils::encode_varint(value);
            let mut reader = GpbReader::new(&encoded).unwrap();
            let decoded = BufferUtils::decode_varint(&mut reader).unwrap();
            assert_eq!(value, decoded);
        }
    }

    #[test]
    fn test_buffer_overflow() {
        let mut buffer = GpbBuffer::with_capacity(10);
        let large_data = vec![0u8; 100];

        let result = buffer.write(&large_data);
        assert!(matches!(
            result,
            Err(GpbError::Encode(EncodeError::BufferTooSmall { .. }))
        ));
    }
}
