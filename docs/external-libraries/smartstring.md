# smartstring - Complete Documentation

## Overview

`smartstring` is a Rust crate that provides compact inlined strings as a drop-in replacement for `std::string::String`. It implements the Small String Optimization (SSO) to minimize heap allocations and improve cache locality, making it ideal for high-performance applications like crypto trading systems.

**Version**: 1.0.1
**License**: MPL-2.0

## Core Concept

`SmartString` is a wrapper around a `String` that automatically inlines small strings directly in the struct's memory space, avoiding heap allocation for strings up to 23 bytes in length.

### Key Benefits for Low-Latency Trading

1. **Heap Allocation Avoidance**: Eliminates non-deterministic latency from heap allocations for common short strings (trading symbols, order IDs, status messages)
2. **Cache Locality**: Stack-based storage keeps data in CPU L1/L2 cache, drastically improving access speed
3. **Zero-Cost Abstraction**: Same size as standard `String` (24 bytes on 64-bit architectures)

## Architecture

### Memory Layout

- **Size**: 24 bytes on 64-bit architectures (same as `String`)
- **Inline Capacity**: 23 bytes (`MAX_INLINE`)
- **Mechanism**: Uses 23 bytes for data + 7 bits for length encoding
- **Discriminant**: Uses pointer alignment to store a bit indicating inline vs heap storage

### String Modes

SmartString offers two modes via generic type parameter:

#### 1. LazyCompact (Recommended for Trading)

```rust
use smartstring::alias::String;  // Type alias for SmartString<LazyCompact>
```

**Characteristics**:
- Once expanded to heap, remains on heap even if string shrinks
- Avoids reallocation jitter when string length fluctuates
- **DEALLOC** constant: `false`
- **Best for**: Predictable performance in critical paths

**Behavior**:
```rust
let mut s = String::from("short");     // Inline
s.push_str("_very_long_suffix_text"); // Moves to heap
s.clear();                             // Remains on heap (no reallocation)
```

#### 2. Compact (Use with Caution)

```rust
use smartstring::alias::CompactString;  // Type alias for SmartString<Compact>
```

**Characteristics**:
- Aggressively re-inlines strings when they shrink below threshold
- Can cause allocation/deallocation churn
- **DEALLOC** constant: `true`
- **Best for**: Non-critical paths where memory usage is priority

**Behavior**:
```rust
let mut s = CompactString::from("short");     // Inline
s.push_str("_very_long_suffix_text");        // Moves to heap
s.truncate(5);                                // Re-inlines to stack
```

## API Reference

### Constants

```rust
pub const MAX_INLINE: usize = 23;  // Maximum bytes for inline storage
```

### Core Struct

```rust
pub struct SmartString<Mode: SmartStringMode> { /* private fields */ }
```

### Constructors

```rust
impl<Mode: SmartStringMode> SmartString<Mode> {
    // Create empty string
    pub fn new() -> Self;

    // Const constructor for compile-time creation
    pub const fn new_const() -> Self;
}
```

### String Manipulation

```rust
// Appending
pub fn push(&mut self, ch: char);
pub fn push_str(&mut self, string: &str);

// Removing
pub fn pop(&mut self) -> Option<char>;
pub fn remove(&mut self, index: usize) -> char;
pub fn clear(&mut self);
pub fn truncate(&mut self, new_len: usize);

// Inserting
pub fn insert(&mut self, index: usize, ch: char);
pub fn insert_str(&mut self, index: usize, string: &str);

// Replacing
pub fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where R: RangeBounds<usize>;
```

### Inspection Methods

```rust
pub fn len(&self) -> usize;
pub fn is_empty(&self) -> bool;
pub fn capacity(&self) -> usize;

// Unique to SmartString
pub fn is_inline(&self) -> bool;  // Check if currently stored inline
```

### Conversion Methods

```rust
pub fn as_str(&self) -> &str;
pub fn as_mut_str(&mut self) -> &mut str;
pub fn as_bytes(&self) -> &[u8];
```

### Iteration

```rust
pub fn drain<R>(&mut self, range: R) -> Drain<'_>
    where R: RangeBounds<usize>;

pub fn chars(&self) -> Chars<'_>;
pub fn char_indices(&self) -> CharIndices<'_>;
pub fn bytes(&self) -> Bytes<'_>;
```

## Trait Implementations

### Conversion Traits

```rust
impl<Mode> From<&str> for SmartString<Mode>
impl<Mode> From<String> for SmartString<Mode>
impl<Mode> From<&String> for SmartString<Mode>
impl<Mode> From<Box<str>> for SmartString<Mode>
impl<Mode> From<Cow<'_, str>> for SmartString<Mode>
impl<Mode> From<char> for SmartString<Mode>
```

### Comparison Traits

```rust
impl<Mode> PartialEq<str> for SmartString<Mode>
impl<Mode> PartialEq<&str> for SmartString<Mode>
impl<Mode> PartialEq<String> for SmartString<Mode>
impl<Mode> Eq for SmartString<Mode>
impl<Mode> PartialOrd for SmartString<Mode>
impl<Mode> Ord for SmartString<Mode>
```

### Collection Traits

```rust
impl<Mode> Extend<char> for SmartString<Mode>
impl<Mode> Extend<&char> for SmartString<Mode>
impl<Mode> Extend<&str> for SmartString<Mode>
impl<Mode> FromIterator<char> for SmartString<Mode>
impl<Mode> FromIterator<&str> for SmartString<Mode>
```

### String Operations

```rust
impl<Mode> Add<&str> for SmartString<Mode>
impl<Mode> AddAssign<&str> for SmartString<Mode>
```

### Indexing

```rust
impl<Mode> Index<Range<usize>> for SmartString<Mode>
impl<Mode> Index<RangeFrom<usize>> for SmartString<Mode>
impl<Mode> Index<RangeTo<usize>> for SmartString<Mode>
impl<Mode> Index<RangeFull> for SmartString<Mode>
impl<Mode> IndexMut<Range<usize>> for SmartString<Mode>
// ... and other range types
```

### Display and Debug

```rust
impl<Mode> Display for SmartString<Mode>
impl<Mode> Debug for SmartString<Mode>
```

### I/O Traits

```rust
impl<Mode> Write for SmartString<Mode>  // fmt::Write
```

### Serialization (with `serde` feature)

```rust
impl<Mode> Serialize for SmartString<Mode>
impl<'de, Mode> Deserialize<'de> for SmartString<Mode>
```

## API Differences from Standard String

### Missing Methods

SmartString deliberately omits:
- `with_capacity(capacity: usize)` - No pre-allocation control
- `reserve(additional: usize)` - No capacity management
- `reserve_exact(additional: usize)` - No capacity management
- `shrink_to_fit()` - Automatic memory management
- `shrink_to(min_capacity: usize)` - Automatic memory management

### Additional Methods

SmartString adds:
- `is_inline() -> bool` - Check if string is currently inlined
- `new_const() -> Self` - Const constructor for compile-time creation

## Feature Flags

```toml
[dependencies.smartstring]
version = "1.0"
features = ["serde", "arbitrary", "proptest"]
```

- **`serde`**: Enable serialization/deserialization support
- **`arbitrary`**: Support for arbitrary data generation
- **`proptest`**: Property-based testing strategies

## Type Aliases

For convenience, the crate provides type aliases:

```rust
// Recommended for most use cases
use smartstring::alias::String;        // SmartString<LazyCompact>

// For aggressive memory optimization
use smartstring::alias::CompactString; // SmartString<Compact>
```

## Usage Examples

### Basic Usage

```rust
use smartstring::alias::String;

// Creation
let mut s = String::new();
let s = String::from("BTC-USDT");

// String operations
s.push_str("SPOT");
s.push('_');
s.insert_str(0, "PREFIX_");

// Inspection
assert!(s.len() < 24);
assert!(s.is_inline());  // Still fits inline
```

### Trading System Example

```rust
use smartstring::alias::String;
use std::collections::HashMap;

// Order book using SmartString for symbols
struct OrderBook {
    symbol: String,  // e.g., "BTC-USDT"
    orders: HashMap<String, Order>,  // Order ID as key
}

struct Order {
    id: String,          // e.g., "ORD123456"
    status: String,      // e.g., "FILLED"
    side: String,        // e.g., "BUY"
    order_type: String,  // e.g., "LIMIT"
}

// All these common trading strings fit within 23 bytes
// and avoid heap allocation
```

### Performance Pattern

```rust
use smartstring::alias::String;

// Good: Predictable performance
fn process_order_update(order_id: &str, status: &str) {
    let mut key = String::from(order_id);
    key.push('_');
    key.push_str(status);  // May allocate once if > 23 bytes

    // Further operations won't cause reallocations
    // even if string shrinks (LazyCompact mode)
}

// Avoid: CompactString in hot paths
use smartstring::alias::CompactString;

fn unstable_performance(mut s: CompactString) {
    s.push_str("long_suffix_exceeding_inline");  // Heap allocation
    s.truncate(5);  // Re-inlines (allocation!)
    s.push_str("another_long_suffix");  // Heap allocation again!
}
```

## Implementation Details

### Memory Safety

- All operations maintain Rust's memory safety guarantees
- No unsafe code exposed in public API
- Thread-safe (`Send` + `Sync`)

### Performance Characteristics

1. **Inline Strings (≤ 23 bytes)**
   - Zero heap allocations
   - All operations on stack
   - Optimal cache locality

2. **Heap Strings (> 23 bytes)**
   - Single allocation (LazyCompact)
   - Slight overhead vs standard String
   - Transparent conversion

3. **Mode Selection Impact**
   - LazyCompact: Predictable, stable performance
   - Compact: Variable performance, better memory usage

## Best Practices for Trading Systems

### Do's

1. **Use LazyCompact mode** (`smartstring::alias::String`)
   - Predictable latency characteristics
   - No allocation churn in critical paths

2. **Apply to short identifiers**
   - Trading symbols: "BTC-USDT", "ETH-PERP"
   - Order IDs: "ORD123456", "TRADE789"
   - Status codes: "NEW", "FILLED", "CANCELED"
   - Exchange names: "BINANCE", "COINBASE"

3. **Use in collections**
   ```rust
   type SymbolMap = HashMap<String, OrderBook>;  // SmartString keys
   type OrderCache = BTreeMap<String, Order>;    // Sorted by ID
   ```

4. **Leverage in parsing**
   ```rust
   // FIX protocol tags are perfect candidates
   let tag = String::from("35");   // MsgType
   let value = String::from("D");   // NewOrderSingle
   ```

### Don'ts

1. **Avoid Compact mode in hot paths**
   - Allocation/deallocation churn
   - Unpredictable latency spikes

2. **Don't use for large strings**
   - Log messages
   - JSON payloads
   - Error descriptions

3. **Don't rely on capacity control**
   - No `reserve()` or `with_capacity()`
   - Let SmartString manage memory

## Benchmarking Considerations

When benchmarking SmartString:

1. **Test realistic string sizes**
   - Most trading identifiers < 23 bytes
   - Include edge cases around threshold

2. **Measure allocation patterns**
   - Use allocation profilers
   - Count heap allocations

3. **Consider cache effects**
   - Test with realistic data structures
   - Measure cache misses

4. **Compare modes appropriately**
   - LazyCompact for stability
   - Compact for memory efficiency

## Integration with Trading Stack

### With Serde

```toml
[dependencies]
smartstring = { version = "1.0", features = ["serde"] }
```

```rust
use smartstring::alias::String;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MarketData {
    symbol: String,
    exchange: String,
    #[serde(rename = "type")]
    data_type: String,
}
```

### With FxHashMap (rustc-hash)

```rust
use smartstring::alias::String;
use rustc_hash::FxHashMap;

// Optimal for trading systems
type OrderMap = FxHashMap<String, Order>;
```

### With Other HFT Libraries

```rust
use smartstring::alias::String;
use rust_decimal::Decimal;
use flume;

struct OrderUpdate {
    symbol: String,      // SmartString
    price: Decimal,      // Precise decimal
    quantity: Decimal,
    order_id: String,    // SmartString
}

// Send via flume channel
let (tx, rx) = flume::unbounded::<OrderUpdate>();
```

## Summary

`smartstring` is an essential optimization for Rust-based trading systems:

- **Eliminates heap allocations** for strings ≤ 23 bytes
- **Improves cache locality** for better performance
- **Drop-in replacement** for standard String
- **LazyCompact mode** provides predictable performance
- **Zero-cost abstraction** with same size as String

For trading applications, always prefer `smartstring::alias::String` (LazyCompact) to maximize performance predictability while minimizing allocation overhead.