# Comprehensive Guide to rustc-hash

## Overview

`rustc-hash` is a high-performance, non-cryptographic hashing library used by the Rust compiler (rustc). It provides a
faster alternative to the standard library's SipHash for scenarios where DoS resistance isn't required.

### Key Features

- **Speed-optimized**: Significantly faster than SipHash
- **Non-cryptographic**: Not designed for security or DoS resistance
- **Simple API**: Drop-in replacement for standard HashMap/HashSet
- **Seeded variants**: Support for deterministic hashing with custom seeds

## Core Components

### 1. FxHasher

The main hasher implementation using a fast polynomial hash with a single bit rotation as a finishing step (designed by
Orson Peters).

```rust
use rustc_hash::FxHasher;

// Create with default seed (0)
let hasher = FxHasher::default ();

// Create with custom seed
let hasher = FxHasher::with_seed(42);
```

### 2. FxBuildHasher

A `BuildHasher` implementation that produces `FxHasher` instances.

```rust
use std::hash::BuildHasher;
use rustc_hash::FxBuildHasher;

// Can be used to hash values directly
let hash = FxBuildHasher.hash_one(42);
assert_ne!(FxBuildHasher.hash_one(1), FxBuildHasher.hash_one(2));
```

## Primary Usage: FxHashMap and FxHashSet

### Basic Usage

```rust
use rustc_hash::{FxHashMap, FxHashSet};

// Creating maps and sets
let mut map: FxHashMap<u32, String> = FxHashMap::default ();
let mut set: FxHashSet<i32> = FxHashSet::default ();

// Standard operations work identically to HashMap/HashSet
map.insert(22, "hello".to_string());
set.insert(42);
```

### Alternative Construction Methods

```rust
// With capacity
let map: FxHashMap<i32, i32> = FxHashMap::with_capacity_and_hasher(100, FxBuildHasher);

// From iterator
let map: FxHashMap<_, _ > = vec![(1, "a"), (2, "b")].into_iter().collect();

// Using the From trait
let set = FxHashSet::from([1, 2, 3, 4]);
```

## Seeded Hashing

For deterministic hashing across runs, use the seeded variants:

### FxSeededState

```rust
use rustc_hash::FxSeededState;
use std::collections::HashMap;

// Create a HashMap with a specific seed
let mut map = HashMap::with_hasher(FxSeededState::with_seed(12));
map.insert(15, 610);
assert_eq!(map[&15], 610);
```

### FxHashMapSeed and FxHashSetSeed

```rust
use rustc_hash::{FxHashMapSeed, FxHashSetSeed, FxSeededState};

// Type aliases for convenience
let mut map: FxHashMapSeed<String, i32> =
HashMap::with_hasher(FxSeededState::with_seed(42));

let mut set: FxHashSetSeed<i32> =
HashSet::with_hasher(FxSeededState::with_seed(42));
```

## Advanced Operations

### Iteration

```rust
let map = FxHashMap::from([("a", 1), ("b", 2), ("c", 3)]);

// Iterate over key-value pairs
for (key, value) in & map {
println!("{}: {}", key, value);
}

// Iterate over keys only
for key in map.keys() {
println ! ("Key: {}", key);
}

// Iterate over mutable values
for value in map.values_mut() {
* value += 10;
}
```

### Entry API

```rust
let mut letters: FxHashMap<char, u32> = FxHashMap::default ();

for ch in "a short treatise on fungi".chars() {
letters.entry(ch)
.and_modify( |counter | * counter += 1)
.or_insert(1);
}
```

### Set Operations

```rust
let a = FxHashSet::from([1, 2, 3]);
let b = FxHashSet::from([2, 3, 4]);

// Difference
let diff: FxHashSet<_ > = a.difference( & b).copied().collect();

// Intersection
let inter: FxHashSet<_ > = a.intersection( & b).copied().collect();

// Union
let union: FxHashSet<_ > = a.union (&b).copied().collect();
```

### Capacity Management

```rust
let mut map: FxHashMap<i32, i32> = FxHashMap::with_capacity(100);

// Reserve additional capacity
map.reserve(50);

// Shrink to fit
map.shrink_to_fit();

// Try to reserve (can fail)
map.try_reserve(1000).expect("Failed to reserve");
```

### Extract and Retain Operations

```rust
let mut set: FxHashSet<i32> = (0..10).collect();

// Extract elements matching a predicate
let evens: FxHashSet<i32> = set.extract_if( | & x| x % 2 == 0).collect();

// Retain only specific elements
let mut map: FxHashMap<i32, i32> = (0..8).map( | x| (x, x * 10)).collect();
map.retain( | & k, _ | k % 2 == 0);
```

## Performance Considerations

1. **When to use FxHash**:
   - Internal data structures where keys are trusted
   - Compiler/tool implementations
   - Performance-critical code where DoS isn't a concern
   - When keys are integers or simple types

2. **When NOT to use FxHash**:
   - Public-facing APIs accepting untrusted input
   - Security-sensitive applications
   - When cryptographic properties are needed
   - Hash tables exposed to potential DoS attacks

## Implementation Details

The hasher implements all standard trait methods:

- `write_u8`, `write_u16`, `write_u32`, `write_u64`, `write_u128`
- `write_usize`, `write_i8`, `write_i16`, `write_i32`, `write_i64`, `write_i128`
- Generic `write` for byte slices

The algorithm uses a polynomial rolling hash with a rotation-based finalizer for good distribution properties while
maintaining high speed.

## Feature Flags

The crate supports an optional `rand` dependency (version 0.8) for additional functionality when the feature is enabled.