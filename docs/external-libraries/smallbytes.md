# SmallBytes 0.1.0 Documentation

## Overview

SmallBytes is a Rust library that combines the features of SmallVec and the bytes crate, providing a buffer that can store a small number of bytes inline before moving to heap allocation. It offers efficient memory management for small byte collections by avoiding heap allocation when the data fits within the inline capacity.

**Key Features:**
- Inline storage of bytes on the stack for small collections
- Automatic transition to heap allocation when inline capacity is exceeded
- Implements `BufMut` trait for easy buffer manipulation
- Generic over inline capacity via const generic parameter
- Wraps `SmallVec<[u8; N]>` for underlying storage

## Core Characteristics

### Memory Layout
- **Minimum object size**: 24 bytes
- **Optimal inline storage**: Up to 16 bytes on the stack efficiently
- **Scalable**: Object size increases with inline capacity beyond 16 bytes
- **Automatic growth**: Seamlessly switches to heap storage when needed

### Performance Benefits
- Zero heap allocation for small byte collections
- Predictable memory layout
- Cache-friendly for small data
- Low overhead compared to `Vec<u8>` for small sizes

## Main Struct: SmallBytes<N>

### Declaration
```rust
pub struct SmallBytes<const N: usize>(SmallVec<[u8; N]>);
```

A Vec-like container that can store a small number of bytes inline, where `N` is the inline capacity.

## Constructors

### `new() -> Self`
Creates an empty SmallBytes with zero length.

```rust
let buf = SmallBytes::<4>::new();
assert!(buf.is_empty());
assert_eq!(buf.len(), 0);
```

### `from(small_vec: SmallVec<[u8; N]>) -> Self`
Wraps an existing SmallVec into a SmallBytes container.

```rust
let vec = SmallVec::<[u8; 4]>::new();
let bytes = SmallBytes::from(vec);
```

## Core Methods

### Capacity and Size Information

#### `len() -> usize`
Returns the number of bytes currently stored.

#### `is_empty() -> bool`
Returns true if the container holds no bytes.

#### `capacity() -> usize`
Returns the total storage capacity (inline + heap).

#### `inline_size() -> usize`
Returns the maximum number of bytes that can be stored inline.

#### `spilled() -> bool`
Returns true if the data has overflowed the inline storage and moved to heap.

### Data Manipulation

#### `push(byte: u8)`
Appends a single byte to the end.

```rust
let mut buf = SmallBytes::<4>::new();
buf.push(b'H');
buf.push(b'i');
```

#### `pop() -> Option<u8>`
Removes and returns the last byte.

#### `clear()`
Removes all bytes, keeping the allocated capacity.

#### `truncate(len: usize)`
Shortens the buffer to the specified length.

#### `extend_from_slice(slice: &[u8])`
Appends all bytes from a slice.

```rust
let mut buf = SmallBytes::<8>::new();
buf.extend_from_slice(b"hello");
```

## Trait Implementations

### Deref and DerefMut
Provides direct access to the underlying SmallVec and slice operations.

```rust
let buf = SmallBytes::<4>::from(b"test".to_vec());
assert_eq!(&buf[..], b"test");
```

### AsRef<[u8]>
Enables slice-like references for reading.

```rust
let buf = SmallBytes::<4>::from(b"data".to_vec());
let slice: &[u8] = buf.as_ref();
```

### BufMut Implementation
Implements the `bytes` crate's `BufMut` trait for buffer manipulation.

#### `remaining_mut() -> usize`
Returns the number of bytes that can be written without reallocating.

#### `chunk_mut() -> &mut UninitMem`
Provides a mutable uninitialized slice for writing.

#### `advance_mut(cnt: usize)`
Advances the internal cursor after writing bytes.

#### Buffer Writing Methods:
- `put_slice(src: &[u8])` - Writes a slice
- `put(src: impl Buf)` - Writes from a Buf
- `put_bytes(val: u8, cnt: usize)` - Writes repeated bytes
- `put_u8(n: u8)`, `put_u16(n: u16)`, etc. - Writes primitive types

### Clone
Supports cloning the entire buffer and its contents.

### Debug
Provides debug formatting for development.

### Default
Creates an empty SmallBytes instance.

### Comparison Traits
Implements `Eq`, `PartialEq`, `Ord`, `PartialOrd`, and `Hash` for comparisons.

### Extend Traits
- `Extend<u8>` - Extends from an iterator of bytes
- `Extend<&u8>` - Extends from an iterator of byte references

```rust
let mut buf = SmallBytes::<8>::new();
buf.extend([1, 2, 3, 4].iter());
```

## Usage Examples

### Basic Usage
```rust
use smallbytes::SmallBytes;

let mut buf = SmallBytes::<6>::new();
buf.put(&b"hello"[..]);
buf.put(&b" world"[..]);
buf.put_u16(1234);

assert_eq!(buf.len(), 13); // "hello world" + 2 bytes for u16
```

### Working with BufMut
```rust
use bytes::BufMut;
use smallbytes::SmallBytes;

let mut buf = SmallBytes::<16>::new();

// Write various data types
buf.put_u32(0x12345678);
buf.put_slice(b"data");
buf.put_u8(255);

// Check remaining capacity
println!("Remaining: {}", buf.remaining_mut());
```

### Inline vs Heap Storage
```rust
let mut small_buf = SmallBytes::<4>::new();
small_buf.extend_from_slice(b"hi");  // Stored inline
assert!(!small_buf.spilled());

small_buf.extend_from_slice(b"hello"); // Exceeds inline capacity
assert!(small_buf.spilled()); // Now on heap
```

### Performance-Oriented Usage
```rust
// Pre-allocate for known size to avoid reallocations
let mut buf = SmallBytes::<32>::new();

// Efficient byte manipulation
buf.put_slice(&data);
buf.put_u64_le(timestamp);

// Zero-copy slice access
let slice: &[u8] = buf.as_ref();
```

## Performance Characteristics

### Memory Efficiency
- **Stack allocation**: Up to 16 bytes stored efficiently on stack
- **Zero overhead**: No heap allocation for small collections
- **Predictable size**: Base object is 24 bytes

### Time Complexity
- **Access**: O(1) for element access
- **Append**: Amortized O(1) for push operations
- **Extend**: O(n) where n is the number of elements being added

### Best Use Cases
- Small message buffers
- Network packet headers
- Temporary byte collections
- Protocol parsing buffers
- Cache-friendly byte manipulation

### Comparison with Alternatives

| Operation | SmallBytes | Vec<u8> | bytes::BytesMut |
|-----------|------------|---------|-----------------|
| Small collections (≤16 bytes) | Inline storage | Heap allocation | Heap allocation |
| Memory overhead | 24 bytes minimum | 24 bytes + heap | Variable |
| BufMut support | ✅ Native | ❌ No | ✅ Native |
| Zero-copy slicing | ✅ Yes | ❌ No | ✅ Yes |

## Integration Patterns

### With Tokio and Async Code
```rust
use tokio::io::AsyncWriteExt;
use smallbytes::SmallBytes;

async fn send_small_message(writer: &mut impl AsyncWriteExt) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = SmallBytes::<64>::new();
    buf.put_slice(b"header");
    buf.put_u32(message_id);
    buf.put_slice(&payload);
    
    writer.write_all(buf.as_ref()).await?;
    Ok(())
}
```

### With Protocol Parsing
```rust
fn parse_header(data: &[u8]) -> SmallBytes<32> {
    let mut header = SmallBytes::new();
    header.put_u8(VERSION);
    header.put_u16(data.len() as u16);
    header.put_slice(&data[..std::cmp::min(data.len(), 29)]);
    header
}
```

## Dependencies

- **bytes**: ^1.6.0 - Provides BufMut trait and utilities
- **smallvec**: ^1.13.2 - Underlying storage implementation

## License

MIT License

## Version

0.1.0

---

*This documentation is based on smallbytes version 0.1.0 from docs.rs*