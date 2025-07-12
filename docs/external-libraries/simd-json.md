# simd-json for Crypto Exchange Low-Latency Trading

## Overview

simd-json is a high-performance JSON parser that leverages SIMD instructions for parsing speeds up to several GB/s.
Critical for parsing exchange WebSocket feeds and REST responses with minimal latency.

## Performance Optimization

### 1. Memory Allocators (CRITICAL)

- **Recommended**: Use `mimalloc` or `jemalloc` instead of system allocator
- **Alternative**: `snmalloc` (newer, untested in production)
- Allocator choice can significantly impact latency percentiles

### 2. CPU Architecture & SIMD Support

- **x86_64**: Auto-selects AVX2 or SSE4.2 (with `runtime-detection`)
- **ARM**: Uses NEON instructions
- **Disable runtime detection** for production:
  ```toml
  default-features = false
  ```

Then compile with: `RUSTFLAGS="-C target-cpu=native"`

- Yields better performance but binary is CPU-specific

### 3. Feature Flags for Trading

#### Essential Features:

- **`swar-number-parsing`** (default): Parses 8 digits at once for floats
    - Keep enabled for price/volume parsing

- **`known-key`**: Switches from ahash to fxhash
    - Enable if parsing known fields repeatedly (e.g., "price", "amount", "side")
    - Allows hash memoization for hot paths
    - Trade-off: Less DOS protection but faster lookups

#### Optional Features:

- **`big-int-as-float`**: Handle very large integers as f64
    - Consider for some exchanges that use large integer IDs

- **`128bit`**: Support for u128/i128
    - Disable unless needed (performance penalty)

- **`ordered-float`**: Makes floats Eq-comparable
    - Small performance cost, enable if needed for comparisons

## API Selection for Trading

### 1. **Borrowed Values** (Recommended for hot path)

```rust
let mut buffer = exchange_json_data.to_vec();
let order_book: BorrowedValue = simd_json::to_borrowed_value( & mut buffer) ?;
```

- Zero-copy parsing
- Minimal allocations
- Best for transient data (order book updates, trades)

### 2. **Tape API** (For known schemas)

```rust
let mut buffer = br#"{"bid": 50000.5, "ask": 50001.0, "ts": 1234567890}"#.to_vec();
let tape = simd_json::to_tape( & mut buffer) ?;
// Direct access without DOM construction
```

- Lowest overhead
- Best when schema is known
- Ideal for tick data with fixed structure

### 3. **Owned Values** (Avoid in hot path)

- Only use when data needs to outlive the buffer
- Higher allocation overhead

## Performance Characteristics

### From Benchmark Code:

- Measures cycles/byte, instructions, cache misses
- Warmup phase important for stable measurements
- Multiple iterations (2000 rounds) for statistical significance

### Key Metrics for Trading:

- **Cycles per byte**: Lower is better
- **Cache misses**: Critical for consistent latency
- **Branch mispredictions**: Can cause latency spikes

## Best Practices for Trading

1. **Buffer Reuse**
   ```rust
   let mut buffer = Vec::with_capacity(65536); // Pre-allocate
   // Reuse buffer for each message
   ```

2. **Feature Configuration**
   ```toml
   [dependencies]
   simd-json = {
     version = "0.15",
     features = ["serde_impl", "known-key", "swar-number-parsing"] # Available features
   }
   ```

3. **Compile Flags**
   ```bash
   RUSTFLAGS="-C target-cpu=native -C opt-level=3 -C lto=fat"
   ```

4. **Schema-Specific Parsing**
    - Use Tape API for known message types
    - Pre-compute field positions when possible
    - Avoid dynamic lookups in hot path

## Safety Considerations

- Extensive use of unsafe code for SIMD operations
- Well-tested with fuzzing and property-based testing
- Production-ready despite unsafe usage

## Platform Notes

- Linux recommended for best performance tools
- Performance counter access needed for profiling
- SIMD availability crucial (falls back to slower implementation without)

## More Guides

```text
TITLE: Deserialize JSON with Serde using simd-json
DESCRIPTION: Shows how to use `simd-json`'s Serde-compatible API to deserialize JSON into a `serde_json::Value` type. This feature allows `simd-json` to act as a drop-in replacement for `serde_json` in many deserialization contexts, leveraging `simd-json`'s performance benefits.
SOURCE: https://github.com/simd-lite/simd-json/blob/main/README.md#_snippet_2

LANGUAGE: Rust
CODE:
```

use simd_json;
use serde_json::Value;

let mut d = br#"{"some": ["key", "value", 2]}"# .to_vec();
let v: Value = simd_json::serde::from_slice(&mut d).unwrap();

```

----------------------------------------

TITLE: Parse JSON into OwnedValue using simd-json
DESCRIPTION: Illustrates parsing JSON data from a byte slice into a `simd_json::OwnedValue`. Unlike `BorrowedValue`, `OwnedValue` takes ownership of the parsed data, making it suitable for scenarios where the data needs to outlive the original buffer.
SOURCE: https://github.com/simd-lite/simd-json/blob/main/README.md#_snippet_1

LANGUAGE: Rust
CODE:
```

use simd_json;
let mut d = br#"{"some": ["key", "value", 2]}"# .to_vec();
let v: simd_json::OwnedValue = simd_json::to_owned_value(&mut d).unwrap();

```

----------------------------------------

TITLE: Parse JSON into BorrowedValue using simd-json
DESCRIPTION: Demonstrates how to parse a byte slice containing JSON data into a `simd_json::BorrowedValue` object. This API is optimized for scenarios where the JSON structure is unknown and provides a DOM-like representation without owning the underlying data.
SOURCE: https://github.com/simd-lite/simd-json/blob/main/README.md#_snippet_0

LANGUAGE: Rust
CODE:
```

use simd_json;
let mut d = br#"{"some": ["key", "value", 2]}"# .to_vec();
let v: simd_json::BorrowedValue = simd_json::to_borrowed_value(&mut d).unwrap();

```

----------------------------------------

TITLE: Access JSON data using simd-json Tape API
DESCRIPTION: Demonstrates the `simd-json` Tape API, which provides a low-level, efficient way to navigate and query parsed JSON data. It allows for direct access to values by key or index and includes methods for checking data types and existence.
SOURCE: https://github.com/simd-lite/simd-json/blob/main/README.md#_snippet_3

LANGUAGE: Rust
CODE:
```

use simd_json;

let mut d = br#"{"the_answer": 42}"# .to_vec();
let tape = simd_json::to_tape(&mut d).unwrap();
let value = tape.as_value();
// try_get treats value like an object, returns Ok(Some(_)) because the key is found
assert!(value.try_get("the_answer").unwrap().unwrap() == 42);
// returns Ok(None) because the key is not found but value is an object
assert!(value.try_get("does_not_exist").unwrap() == None);
// try_get_idx treats value like an array, returns Err(_) because value is not an array
assert!(value.try_get_idx(0).is_err());

```
```