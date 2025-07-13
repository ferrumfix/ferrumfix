# Zerocopy Documentation

## Overview

Zerocopy is a Rust library that makes zero-cost memory manipulation effortless. We write `unsafe` so you don't have to.

**Version**: 0.8.26
**License**: BSD-2-Clause OR Apache-2.0 OR MIT
**Repository**: https://github.com/google/zerocopy

### Key Philosophy

**"Fast, safe, compile error. Pick two."**

Zerocopy enables efficient, safe transmutation between types with compile-time guarantees and minimal runtime overhead. It's expressly designed for use in security-critical contexts.

## Core Traits

### Conversion Traits

#### 1. `TryFromBytes`

```rust
pub unsafe trait TryFromBytes {
    // Methods for conditional conversion from bytes
}
```

- **Purpose**: Types for which *some* bit patterns are valid
- **Key Methods**:
  - `try_ref_from_bytes(source: &[u8]) -> Result<&Self, TryFromBytesError>`
  - `try_read_from_bytes(source: &[u8]) -> Result<Self, TryFromBytesError>`
  - `try_ref_from_prefix/suffix` variants
  - `_with_elems` variants for DSTs
- **Example**:
```rust
#[derive(TryFromBytes, KnownLayout, Immutable)]
#[repr(u8)]
enum Command {
    Read = 0x01,
    Write = 0x02,
}

let bytes = &[0x01];
let cmd = Command::try_ref_from_bytes(bytes).unwrap();
```

#### 2. `FromZeros`

```rust
pub unsafe trait FromZeros {
    // Zero bytes represent a valid instance
}
```

- **Purpose**: Types where a sequence of zero bytes is valid
- **Key Methods**:
  - `new_zeroed() -> Self` (for sized types)
  - `new_slice_zeroed(len: usize) -> Self` (for slices)
- **Implementations**: All numeric types, Option<NonZero*>, etc.

#### 3. `FromBytes`

```rust
pub unsafe trait FromBytes: FromZeros {
    // Any bit pattern is valid
}
```

- **Purpose**: Types where *any* bit pattern is valid
- **Key Methods**:
  - `ref_from_bytes(source: &[u8]) -> Result<&Self, CastError>`
  - `mut_from_bytes(source: &mut [u8]) -> Result<&mut Self, CastError>`
  - `read_from_bytes(source: &[u8]) -> Result<Self, CastError>`
- **Example**:
```rust
#[derive(FromBytes, KnownLayout, Immutable)]
#[repr(C)]
struct PacketHeader {
    src_port: [u8; 2],
    dst_port: [u8; 2],
    length: [u8; 2],
    checksum: [u8; 2],
}

let bytes = &[0, 1, 2, 3, 4, 5, 6, 7][..];
let header = PacketHeader::ref_from_bytes(bytes).unwrap();
```

#### 4. `IntoBytes`

```rust
pub unsafe trait IntoBytes {
    // Can be converted to bytes
}
```

- **Purpose**: Types that can be safely converted to byte slices
- **Key Methods**:
  - `as_bytes(&self) -> &[u8]`
  - `as_mut_bytes(&mut self) -> &mut [u8]`
  - `write_to(&self, dst: &mut [u8]) -> Result<(), SizeError>`
  - `write_to_io(&self, dst: &mut impl Write) -> io::Result<()>`
- **Example**:
```rust
#[derive(IntoBytes, Immutable)]
#[repr(C)]
struct Response {
    status: u16,
    data: [u8; 32],
}

let response = Response { status: 200, data: [0; 32] };
let bytes = response.as_bytes();
```

### Marker Traits

#### 1. `KnownLayout`

```rust
pub unsafe trait KnownLayout {
    // Zerocopy can reason about the type's layout
}
```

- **Purpose**: Enables zerocopy to understand type layout
- **Required for**: Most zerocopy operations
- **Auto-implemented for**: Sized types with `FromBytes`

#### 2. `Immutable`

```rust
pub unsafe trait Immutable {
    // No interior mutability
}
```

- **Purpose**: Indicates freedom from interior mutability
- **Required for**: Creating immutable references from bytes
- **NOT implemented for**: `Cell`, `RefCell`, atomics, etc.

#### 3. `Unaligned`

```rust
pub unsafe trait Unaligned {
    // align_of::<Self>() == 1
}
```

- **Purpose**: Types with no alignment requirement
- **Use Cases**: Network protocols, packed structs
- **Derive Requirements**: `#[repr(C)]`, `#[repr(packed)]`, or `#[repr(u8)]`

## Transmutation Macros

### Basic Transmutation

#### `transmute!`
```rust
let one_dimensional: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
let two_dimensional: [[u8; 4]; 2] = transmute!(one_dimensional);
```

- Compile-time size checks
- Zero runtime cost
- Works in const contexts

#### `transmute_ref!`
```rust
let array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
let bytes: &[[u8; 2]; 4] = transmute_ref!(&array);
```

#### `transmute_mut!`
```rust
let mut array: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
let bytes: &mut [[u8; 2]; 4] = transmute_mut!(&mut array);
```

### Conditional Transmutation

#### `try_transmute!`
```rust
let value: Result<NonZeroU32, _> = try_transmute!(some_u32);
```

- Runtime validation
- Returns `Result`
- For types with validity requirements

Similar variants: `try_transmute_ref!`, `try_transmute_mut!`

## Core Types

### `Ref<B, T>`

A typed reference derived from a byte slice.

```rust
pub struct Ref<B, T: ?Sized> { ... }
```

**Key Methods**:
- `from_bytes(source: B) -> Result<Ref<B, T>, CastError>`
- `into_ref(self) -> &T`
- `into_mut(self) -> &mut T` (if B: ByteSliceMut)

**Example**:
```rust
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct UdpPacket {
    header: UdpHeader,
    body: [u8],  // DST field
}

impl UdpPacket {
    pub fn parse<B: ByteSlice>(bytes: B) -> Option<Ref<B, UdpPacket>> {
        Ref::from_bytes(bytes).ok()
    }
}
```

### `Unalign<T>`

A wrapper that removes alignment requirements.

```rust
#[repr(packed)]
pub struct Unalign<T>(T);
```

**Use Case**: Reading unaligned data from byte streams
```rust
let bytes = &[0xFF, 0x00, 0x00, 0x00];
let unaligned = Unalign::<u32>::ref_from_bytes(bytes).unwrap();
let value: u32 = unaligned.get();
```

## Byteorder Module

Provides byte order-aware numeric primitives for network protocols and file formats.

### Types

- **Integers**: `U16`, `U32`, `U64`, `U128`, `I16`, `I32`, `I64`, `I128`
- **Floating Point**: `F32`, `F64`
- **Platform Types**: `Usize`, `Isize`

### Endianness

```rust
use zerocopy::byteorder::{U32, NetworkEndian, LittleEndian};

// Type aliases
type NetworkU32 = U32<NetworkEndian>;
type LEU32 = U32<LittleEndian>;
```

### Example: Network Protocol

```rust
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[repr(C)]
struct TcpHeader {
    src_port: U16<NetworkEndian>,
    dst_port: U16<NetworkEndian>,
    sequence: U32<NetworkEndian>,
    ack_number: U32<NetworkEndian>,
    // ... other fields
}

impl TcpHeader {
    fn get_src_port(&self) -> u16 {
        self.src_port.get()
    }
}
```

## Error Types

### `CastError<Src, Dst>`
Errors from casting operations, containing:
- `SizeError`: Size mismatch
- `AlignmentError`: Alignment requirements not met
- `ValidityError`: Invalid bit pattern for target type

### Error Handling Example
```rust
match PacketHeader::ref_from_bytes(bytes) {
    Ok(header) => process_header(header),
    Err(CastError::Size(e)) => println!("Wrong size: {:?}", e),
    Err(CastError::Alignment(e)) => println!("Misaligned: {:?}", e),
    Err(CastError::Validity(e)) => println!("Invalid data: {:?}", e),
}
```

## Feature Flags

### Core Features

- **`alloc`**: Enables `alloc` crate dependency
- **`std`**: Enables `std` crate (implies `alloc`)
- **`derive`**: Enables derive macros (re-exported from `zerocopy-derive`)

### Platform Features

- **`simd`**: Stable SIMD type support
- **`simd-nightly`**: Unstable SIMD types
- **`float-nightly`**: `f16` and `f128` support

### Recommended Setup

For better compile times, use both crates directly:
```toml
[dependencies]
zerocopy = "0.8"
zerocopy-derive = "0.8"
```

Then import derives as:
```rust
use zerocopy::*;
use zerocopy_derive::*;
```

## Best Practices

### 1. Derive Usage

Always use derive macros instead of manual implementation:
```rust
#[derive(TryFromBytes, FromZeros, FromBytes, IntoBytes, KnownLayout, Immutable)]
#[repr(C)]
struct MyStruct {
    // fields
}
```

### 2. Representation Attributes

Use appropriate `#[repr]` attributes:
- `#[repr(C)]`: For structs with predictable layout
- `#[repr(packed)]`: For unaligned access
- `#[repr(u8)]`/`#[repr(i32)]`: For enums

### 3. Network Parsing Pattern

```rust
use zerocopy::{byteorder::NetworkEndian, FromBytes, Ref, Unaligned};

#[derive(FromBytes, Unaligned)]
#[repr(C)]
struct NetworkPacket {
    magic: U32<NetworkEndian>,
    version: u8,
    flags: u8,
    payload_len: U16<NetworkEndian>,
    payload: [u8],  // DST
}

fn parse_packet(bytes: &[u8]) -> Option<Ref<&[u8], NetworkPacket>> {
    Ref::from_bytes(bytes).ok()
}
```

### 4. Zero-Copy Deserialization

```rust
// Instead of:
let data: MyStruct = bincode::deserialize(&bytes)?;  // Allocates

// Use:
let data = MyStruct::ref_from_bytes(&bytes)?;  // Zero allocation
```

## Safety Guarantees

Zerocopy provides strong safety guarantees through:

1. **Compile-time validation**: Derives check type layouts at compile time
2. **Conservative approach**: Only allows operations proven safe
3. **Formal verification**: Uses tools like Kani for correctness proofs
4. **Extensive testing**: Miri testing across platforms and memory models

## Common Patterns

### File Memory Mapping
```rust
use memmap2::Mmap;
use zerocopy::FromBytes;

let file = File::open("data.bin")?;
let mmap = unsafe { Mmap::map(&file)? };
let header = FileHeader::ref_from_bytes(&mmap[..]).unwrap();
```

### Efficient Serialization
```rust
#[derive(IntoBytes)]
#[repr(C)]
struct Record {
    id: u64,
    timestamp: i64,
    data: [u8; 256],
}

let record = Record { /* ... */ };
file.write_all(record.as_bytes())?;
```

### Protocol Implementation
```rust
#[derive(FromBytes, IntoBytes, Unaligned)]
#[repr(C, packed)]
struct DnsHeader {
    id: U16<NetworkEndian>,
    flags: U16<NetworkEndian>,
    qdcount: U16<NetworkEndian>,
    ancount: U16<NetworkEndian>,
    nscount: U16<NetworkEndian>,
    arcount: U16<NetworkEndian>,
}
```

## Performance Considerations

1. **Zero allocations**: All operations work on existing memory
2. **No runtime overhead**: Compile-time checks eliminate runtime validation
3. **Cache-friendly**: Direct memory access patterns
4. **SIMD-compatible**: Works with aligned data for vectorization

## Migration from v0.7 to v0.8

Major changes include:
- New trait hierarchy with `TryFromBytes`
- Improved error types
- Better DST support
- Enhanced derive capabilities

See [release notes](https://github.com/google/zerocopy/discussions/1680) for detailed migration guide.