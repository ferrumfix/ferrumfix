# simd-json Complete Documentation

## Overview

simd-json is a high-performance Rust JSON parser that leverages SIMD (Single Instruction, Multiple Data) instructions for parsing speeds up to several GB/s. It's a port of the C++ simdjson library, designed to provide a fast, Rust-ecosystem-friendly JSON parsing solution critical for parsing exchange WebSocket feeds and REST responses with minimal latency.

## Core Features

- **Ultra-fast JSON parsing** using SIMD instructions (AVX2/SSE4.2 on x86, NEON on ARM)
- **Runtime CPU feature detection** for optimal instruction set selection
- **Multiple parsing strategies** for different use cases
- **Serde compatibility** for drop-in replacement
- **Zero-copy parsing** capabilities with borrowed values
- **Extensive safety testing** including fuzzing and property-based tests

## Module Structure

### 1. `borrowed` Module
- **Purpose**: DOM implementation with borrowed values for maximum efficiency
- **Key Types**:
  - `Value`: Enum representing JSON values with borrowed references
  - `Object`: JSON object representation (hashmap-like)
  - `Array`: JSON array representation (vector-like)
- **Use Cases**: When you need efficient parsing with minimal allocations and data lifetime is tied to the buffer

### 2. `owned` Module
- **Purpose**: Lifetime-less value representation for data that needs to outlive the buffer
- **Key Types**: `OwnedValue` - fully owned JSON DOM
- **Use Cases**: When parsed data needs to be stored or sent across threads

### 3. `tape` Module
- **Purpose**: Low-level, high-performance tape-based parsing
- **Key Types**:
  - `Tape`: Core parsing structure
  - `Array`/`Object`/`Value`: Wrappers for tape interaction
- **Use Cases**: Known schemas with direct field access patterns

### 4. `value` Module
- **Purpose**: JSON-DOM value implementations and type conversions
- **Key Types**:
  - `ValueType`: Standard JSON types
  - `ExtendedValueType`: Extended types beyond JSON spec
  - `StaticNode`: Static tape node representation

### 5. `serde` Module
- **Purpose**: Drop-in serde compatibility layer
- **Key Functions**:
  - `from_slice()`: Parse bytes into Rust structs
  - `to_string()`: Serialize to JSON string
  - `from_borrowed_value()`: Convert from simd-json values

## Performance Optimization Guide

### 1. Memory Allocators (CRITICAL)

```toml
# In Cargo.toml
[dependencies]
mimalloc = { version = "0.1", default-features = false }

# In main.rs
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```

**Allocator Rankings**:
- **Best**: `mimalloc` - Consistent low latency
- **Good**: `jemalloc` - Proven in production
- **Experimental**: `snmalloc` - Newer, untested
- **Avoid**: System allocator - High p99 latency

### 2. CPU Architecture & SIMD Support

#### Compile-Time Optimization (Recommended for Production)
```toml
# Cargo.toml
[dependencies]
simd-json = { version = "0.15", default-features = false, features = ["serde_impl", "known-key"] }
```

```bash
# Build command
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat" cargo build --release
```

#### Runtime Detection (Default)
- Auto-selects: AVX2 > SSE4.2 > Fallback (x86_64)
- Auto-selects: NEON > Fallback (ARM)
- **Note**: Runtime detection has slight overhead

### 3. Feature Flags for Trading

#### Essential Features

```toml
[dependencies]
simd-json = {
  version = "0.15",
  features = [
    "serde_impl",           # Serde compatibility
    "known-key",            # Faster lookups for known fields
    "swar-number-parsing"   # 8-digit parallel float parsing (default)
  ]
}
```

#### Feature Details

- **`swar-number-parsing`** (default):
  - Parses 8 digits simultaneously for floats
  - Critical for price/volume parsing
  - Slight accuracy trade-off for massive speed gain

- **`known-key`**:
  - Switches from ahash to fxhash
  - Enables hash memoization for repeated keys
  - Perfect for exchange feeds with fixed fields ("price", "quantity", "side")
  - Trade-off: Less DoS protection (not relevant for internal systems)

- **`runtime-detection`** (default):
  - Selects optimal SIMD at runtime
  - Disable for production with `default-features = false`

- **`128bit`**:
  - Support for u128/i128
  - Performance penalty - disable unless required

- **`ordered-float`**:
  - Makes floats Eq-comparable
  - Small performance cost

- **`big-int-as-float`**:
  - Handle very large integers as f64
  - Some exchanges use large integer order IDs

## API Selection Guide

### 1. Borrowed Values (Hot Path Recommended)

```rust
use simd_json::{BorrowedValue, to_borrowed_value};

// For order book updates, trades, real-time data
pub fn parse_orderbook_update(data: &[u8]) -> Result<OrderBookUpdate> {
    let mut buffer = data.to_vec();
    let value: BorrowedValue = to_borrowed_value(&mut buffer)?;

    // Zero-copy access to fields
    let bids = value.get("bids").unwrap();
    let asks = value.get("asks").unwrap();

    // Process without additional allocations
    Ok(OrderBookUpdate { /* ... */ })
}
```

**Characteristics**:
- Zero-copy for strings
- Minimal allocations
- Buffer must outlive the value
- Best for transient data

### 2. Tape API (Known Schemas)

```rust
use simd_json::{to_tape, Tape};

// For fixed-structure messages like trades
pub fn parse_trade_tape(data: &[u8]) -> Result<Trade> {
    let mut buffer = data.to_vec();
    let tape = to_tape(&mut buffer)?;
    let value = tape.as_value();

    // Direct field access without DOM construction
    let price = value.try_get("price")?.unwrap();
    let quantity = value.try_get("quantity")?.unwrap();
    let side = value.try_get("side")?.unwrap();

    Ok(Trade { price, quantity, side })
}
```

**Characteristics**:
- Lowest overhead
- No DOM construction
- Direct navigation to known fields
- Ideal for high-frequency tick data

### 3. Serde Integration (Convenience)

```rust
use serde::Deserialize;
use simd_json;

#[derive(Deserialize)]
struct MarketData {
    symbol: String,
    price: f64,
    quantity: f64,
    timestamp: u64,
}

pub fn parse_market_data(data: &[u8]) -> Result<MarketData> {
    let mut buffer = data.to_vec();
    Ok(simd_json::serde::from_slice(&mut buffer)?)
}
```

**Characteristics**:
- Drop-in replacement for serde_json
- Type-safe deserialization
- Higher overhead than manual parsing
- Good for complex structures

### 4. Owned Values (Avoid in Hot Path)

```rust
use simd_json::{OwnedValue, to_owned_value};

// Only when data needs to outlive buffer
pub fn store_historical_data(data: &[u8]) -> Result<OwnedValue> {
    let mut buffer = data.to_vec();
    to_owned_value(&mut buffer)
}
```

**Characteristics**:
- Full ownership of data
- Can outlive original buffer
- Higher allocation overhead
- Use only when necessary

## Performance Characteristics

### Benchmark Insights

From the codebase analysis:
- Measures: cycles/byte, instructions, cache misses
- Warmup phase critical for stable measurements
- Multiple iterations (2000+) for statistical significance

### Key Metrics for Trading

1. **Cycles per byte**: Target < 1.0 for hot path
2. **Cache misses**: Critical for p99 latency
3. **Branch mispredictions**: Cause latency spikes
4. **Allocation count**: Minimize in hot path

### Performance Tips

1. **Buffer Reuse Pattern**
```rust
struct Parser {
    buffer: Vec<u8>,
}

impl Parser {
    fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(65536), // Pre-allocate
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<BorrowedValue> {
        self.buffer.clear();
        self.buffer.extend_from_slice(data);
        to_borrowed_value(&mut self.buffer)
    }
}
```

2. **Known-Key Optimization**
```rust
// With known-key feature, these lookups are memoized
static PRICE_KEY: &str = "price";
static QTY_KEY: &str = "quantity";

let price = value.get(PRICE_KEY);  // Hash computed once
```

3. **SIMD Alignment**
```rust
// Ensure buffers are aligned for SIMD
#[repr(align(64))]
struct AlignedBuffer([u8; 65536]);
```

## Safety Considerations

### Unsafe Code Usage
- Extensive unsafe code for SIMD operations
- Safety verified through:
  - Comprehensive unit tests
  - Property-based testing with proptest
  - Fuzzing with cargo-fuzz
  - Production usage in high-stakes environments

### Input Mutation Warning
⚠️ **Important**: Some parsing functions modify the input buffer in-place for performance. This is safe but requires exclusive access to the buffer during parsing.

```rust
// Buffer is modified during parsing
let mut buffer = data.to_vec();
let value = to_borrowed_value(&mut buffer)?;
// Don't use buffer contents after parsing
```

## Trading-Specific Patterns

### 1. Order Book Parsing
```rust
pub struct OrderBookParser {
    buffer: Vec<u8>,
}

impl OrderBookParser {
    pub fn parse_l2_update(&mut self, data: &[u8]) -> Result<L2Update> {
        self.buffer.clear();
        self.buffer.extend_from_slice(data);

        let value = to_borrowed_value(&mut self.buffer)?;

        // Use known-key feature for these repeated lookups
        let bids = value.get("bids").unwrap().as_array()?;
        let asks = value.get("asks").unwrap().as_array()?;

        // Process levels without allocation
        let bid_levels: SmallVec<[Level; 10]> = bids
            .iter()
            .map(|v| Level {
                price: v[0].as_f64().unwrap(),
                quantity: v[1].as_f64().unwrap(),
            })
            .collect();

        Ok(L2Update { bid_levels, /* ... */ })
    }
}
```

### 2. Trade Stream Processing
```rust
pub struct TradeParser {
    buffer: Vec<u8>,
}

impl TradeParser {
    pub fn parse_trade(&mut self, data: &[u8]) -> Result<Trade> {
        self.buffer.clear();
        self.buffer.extend_from_slice(data);

        // Use tape API for known structure
        let tape = to_tape(&mut self.buffer)?;
        let v = tape.as_value();

        Ok(Trade {
            price: v.try_get("p")?.unwrap().as_f64()?,
            quantity: v.try_get("q")?.unwrap().as_f64()?,
            timestamp: v.try_get("t")?.unwrap().as_u64()?,
            is_buyer_maker: v.try_get("m")?.unwrap().as_bool()?,
        })
    }
}
```

### 3. Multi-Message Parsing
```rust
// For exchanges that batch messages
pub fn parse_message_batch(data: &[u8]) -> Result<Vec<Message>> {
    let mut buffer = data.to_vec();
    let array = to_borrowed_value(&mut buffer)?.as_array()?;

    array.iter()
        .map(|msg| {
            match msg.get("type").and_then(|t| t.as_str()) {
                Some("trade") => parse_trade(msg).map(Message::Trade),
                Some("depth") => parse_depth(msg).map(Message::Depth),
                _ => Err(Error::UnknownMessageType),
            }
        })
        .collect()
}
```

## Platform-Specific Notes

### Linux (Recommended)
- Best performance profiling tools
- Access to performance counters
- Huge pages support for large buffers

### Compilation Flags
```bash
# Maximum performance build
RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat -C codegen-units=1" \
    cargo build --release

# With specific CPU features
RUSTFLAGS="-C target-cpu=x86-64-v3 -C target-feature=+avx2,+fma" \
    cargo build --release
```

### CPU Feature Requirements
- **x86_64**: SSE4.2 minimum, AVX2 recommended
- **ARM**: NEON required
- Falls back to scalar implementation without SIMD (10x slower)

## Common Pitfalls

1. **Using system allocator**: 10-50% performance loss
2. **Not reusing buffers**: Allocation overhead
3. **Using OwnedValue in hot path**: Unnecessary allocations
4. **Parsing without known-key**: Repeated hash computations
5. **Not warming up parser**: First parse is slower

## Integration Example

```rust
use simd_json::{BorrowedValue, to_borrowed_value};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct ExchangeParser {
    // Thread-local buffer pools
    buffers: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl ExchangeParser {
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(Mutex::new(
                (0..10).map(|_| Vec::with_capacity(65536)).collect()
            )),
        }
    }

    pub fn parse(&self, data: &[u8]) -> Result<BorrowedValue> {
        // Get buffer from pool
        let mut buffer = self.buffers.lock().pop()
            .unwrap_or_else(|| Vec::with_capacity(65536));

        buffer.clear();
        buffer.extend_from_slice(data);

        let result = to_borrowed_value(&mut buffer);

        // Return buffer to pool
        self.buffers.lock().push(buffer);

        result
    }
}
```

## Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_parsing(c: &mut Criterion) {
    let data = br#"{"symbol":"BTC-USDT","price":50000.5,"quantity":1.2345,"side":"buy","timestamp":1234567890}"#;

    c.bench_function("simd_json_borrowed", |b| {
        b.iter(|| {
            let mut buffer = data.to_vec();
            let value = to_borrowed_value(&mut buffer).unwrap();
            black_box(value);
        });
    });

    c.bench_function("simd_json_tape", |b| {
        b.iter(|| {
            let mut buffer = data.to_vec();
            let tape = to_tape(&mut buffer).unwrap();
            black_box(tape);
        });
    });
}
```

## Summary

simd-json provides multiple APIs optimized for different use cases:
- Use **Borrowed Values** for real-time market data
- Use **Tape API** for known message structures
- Use **Serde** for complex type-safe parsing
- Avoid **Owned Values** in performance-critical paths

Combined with proper allocator selection, CPU optimization, and buffer reuse patterns, simd-json can achieve parsing speeds of several GB/s, making it ideal for high-frequency trading applications.