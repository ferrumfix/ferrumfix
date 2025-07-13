# Comprehensive Guide to rustc-hash

## Overview

`rustc-hash` is a high-performance, non-cryptographic hashing library used by the Rust compiler (rustc). It provides a faster alternative to the standard library's SipHash for scenarios where DoS resistance isn't required.

### Key Characteristics

- **Speed-optimized**: Significantly faster than SipHash for common use cases
- **Non-cryptographic**: Not designed for security or DoS resistance
- **Compiler-focused**: Optimized specifically for rustc's performance needs
- **Simple API**: Drop-in replacement for standard HashMap/HashSet
- **Lightweight**: Minimal dependencies with optional features

### Design Philosophy

As stated by the maintainers: "Spending more CPU cycles on a higher quality hash does not reduce hash collisions enough to make the compiler faster on real-world benchmarks." The algorithm prioritizes raw speed over cryptographic properties.

## Algorithm Details

### Current Implementation

The current FxHash algorithm was designed by Orson Peters and consists of:

1. **Polynomial hash**: Fast computation for common data types
2. **Single bit rotation**: As a finishing step for better distribution
3. **Wyhash-inspired compression**: For efficient string/slice hashing

### Historical Context

- Originally derived from a Firefox hash algorithm
- Evolved through compiler performance benchmarking
- Current design optimized based on real-world rustc usage patterns

## Core Components

### 1. FxHasher

The main hasher implementation using the fast polynomial hash.

```rust
use rustc_hash::FxHasher;
use std::hash::{Hash, Hasher};

// Create with default seed (0)
let mut hasher = FxHasher::default();

// Create with custom seed
let mut hasher = FxHasher::with_seed(42);

// Hash a value
let value = 12345u64;
value.hash(&mut hasher);
let hash_result = hasher.finish();
```

### 2. FxBuildHasher

A `BuildHasher` implementation that produces `FxHasher` instances.

```rust
use std::hash::BuildHasher;
use rustc_hash::FxBuildHasher;

// Direct hashing using BuildHasher
let hash = FxBuildHasher.hash_one(&42);
assert_ne!(FxBuildHasher.hash_one(&1), FxBuildHasher.hash_one(&2));

// Use as hasher for collections
let mut map = std::collections::HashMap::with_hasher(FxBuildHasher);
```

### 3. FxSeededState

For deterministic hashing with custom seeds.

```rust
use rustc_hash::FxSeededState;
use std::collections::HashMap;

// Create a HashMap with a specific seed for deterministic behavior
let mut map = HashMap::with_hasher(FxSeededState::with_seed(12));
map.insert(15, 610);
assert_eq!(map[&15], 610);
```

## Primary Usage: FxHashMap and FxHashSet

### Type Aliases

```rust
// These are convenience type aliases
pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;
pub type FxHashSet<T> = HashSet<T, FxBuildHasher>;

// Seeded variants
pub type FxHashMapSeed<K, V> = HashMap<K, V, FxSeededState>;
pub type FxHashSetSeed<T> = HashSet<T, FxSeededState>;
```

### Basic Usage

```rust
use rustc_hash::{FxHashMap, FxHashSet};

// Creating maps and sets
let mut map: FxHashMap<u32, String> = FxHashMap::default();
let mut set: FxHashSet<i32> = FxHashSet::default();

// Standard operations work identically to HashMap/HashSet
map.insert(22, "hello".to_string());
set.insert(42);

// All standard methods are available
assert!(map.contains_key(&22));
assert!(set.contains(&42));
```

### Construction Methods

```rust
use rustc_hash::{FxHashMap, FxHashSet};

// With capacity
let map: FxHashMap<i32, i32> = FxHashMap::with_capacity_and_hasher(100, Default::default());

// From iterator
let map: FxHashMap<_, _> = vec![(1, "a"), (2, "b")].into_iter().collect();

// Using the From trait
let set = FxHashSet::from([1, 2, 3, 4]);

// From arrays (Rust 2021+)
let map = FxHashMap::from([(1, "one"), (2, "two"), (3, "three")]);
```

## Seeded Hashing for Determinism

### When to Use Seeded Hashing

- Testing scenarios requiring deterministic behavior
- Reproducible builds
- Debugging hash-dependent algorithms
- Cross-run consistency requirements

### Examples

```rust
use rustc_hash::{FxHashMapSeed, FxHashSetSeed, FxSeededState};

// Create deterministic collections
let mut map: FxHashMapSeed<String, i32> =
    HashMap::with_hasher(FxSeededState::with_seed(42));

let mut set: FxHashSetSeed<i32> =
    HashSet::with_hasher(FxSeededState::with_seed(42));

// These will always hash the same way across runs
map.insert("key".to_string(), 100);
set.insert(42);
```

## Advanced Operations

### Entry API

```rust
use rustc_hash::FxHashMap;

let mut letters: FxHashMap<char, u32> = FxHashMap::default();

// Efficient upsert pattern
for ch in "a short treatise on fungi".chars() {
    letters.entry(ch)
        .and_modify(|counter| *counter += 1)
        .or_insert(1);
}

// Conditional insertion
letters.entry('z').or_insert_with(|| {
    println!("Computing default value");
    0
});
```

### Set Operations

```rust
use rustc_hash::FxHashSet;

let a = FxHashSet::from([1, 2, 3]);
let b = FxHashSet::from([2, 3, 4]);

// Set operations
let difference: FxHashSet<_> = a.difference(&b).copied().collect();
let intersection: FxHashSet<_> = a.intersection(&b).copied().collect();
let union: FxHashSet<_> = a.union(&b).copied().collect();
let symmetric_difference: FxHashSet<_> = a.symmetric_difference(&b).copied().collect();

// In-place operations
let mut c = a.clone();
c.extend(&b); // Union in-place
```

### Capacity Management

```rust
use rustc_hash::FxHashMap;

let mut map: FxHashMap<i32, i32> = FxHashMap::with_capacity(100);

// Check capacity
println!("Capacity: {}", map.capacity());

// Reserve additional capacity
map.reserve(50);

// Shrink to fit actual usage
map.shrink_to_fit();

// Try to reserve (can fail)
match map.try_reserve(1_000_000) {
    Ok(()) => println!("Reserved successfully"),
    Err(e) => println!("Failed to reserve: {}", e),
}
```

### Extract and Retain Operations

```rust
use rustc_hash::{FxHashSet, FxHashMap};

// Extract elements matching a predicate
let mut set: FxHashSet<i32> = (0..10).collect();
let evens: FxHashSet<i32> = set.extract_if(|&x| x % 2 == 0).collect();

// Retain only specific elements
let mut map: FxHashMap<i32, i32> = (0..8).map(|x| (x, x * 10)).collect();
map.retain(|&k, _| k % 2 == 0);

// Drain operations
let mut another_set: FxHashSet<i32> = (0..5).collect();
let drained: Vec<i32> = another_set.drain().collect();
assert!(another_set.is_empty());
```

## Performance Characteristics

### Benchmarking Results

FxHash typically shows:
- **2-4x faster** than SipHash for integer keys
- **1.5-3x faster** for short string keys
- Minimal performance gain for very long keys
- Excellent cache locality for small hash tables

### Best Performance Scenarios

1. **Integer keys**: Maximum performance benefit
2. **Short strings**: Significant speedup
3. **Compiler/tool use**: Where keys are trusted
4. **High-frequency operations**: Where hash computation dominates

### Memory Characteristics

- **No additional memory overhead** compared to standard HashMap
- **Better cache utilization** due to faster hash computation
- **Predictable memory layout** identical to std collections

## Implementation Details

### Hasher Trait Implementation

The FxHasher implements all standard trait methods:

```rust
// Primitive type methods
write_u8, write_u16, write_u32, write_u64, write_u128
write_usize, write_i8, write_i16, write_i32, write_i64, write_i128, write_isize

// Generic byte slice method
write(&mut self, bytes: &[u8])

// Finishing method
finish(&self) -> u64
```

### Internal Algorithm

The polynomial rolling hash with rotation:
1. Accumulates input using multiplication and XOR
2. Applies single-bit rotation for distribution
3. Uses platform-specific optimizations where available

## Feature Flags

### Available Features

- **`std`** (default): Standard library support
- **`rand`**: Enables rand 0.8 integration for randomized testing

### No-std Support

```toml
[dependencies]
rustc-hash = { version = "2.1", default-features = false }
```

When using no-std:
- Core hashing functionality remains available
- HashMap/HashSet type aliases are not available
- Manual construction with FxBuildHasher required

## Security Considerations

### When to Use FxHash

✅ **Appropriate for:**
- Compiler internals
- Build tools and dev tools
- Internal data structures with trusted input
- Performance-critical paths without security requirements
- Scenarios where DoS attacks are not a concern

### When NOT to Use FxHash

❌ **Avoid for:**
- Web servers accepting untrusted input
- Public APIs exposed to user data
- Security-sensitive applications
- Any scenario requiring DoS resistance
- Cryptographic use cases

### Security Warning

FxHash is **not cryptographically secure** and provides **no protection against HashDoS attacks**. An attacker who can control the input keys can potentially cause O(n²) behavior in hash table operations.

## Migration Guide

### From Standard HashMap/HashSet

```rust
// Before
use std::collections::{HashMap, HashSet};
let mut map = HashMap::new();
let mut set = HashSet::new();

// After
use rustc_hash::{FxHashMap, FxHashSet};
let mut map = FxHashMap::default();
let mut set = FxHashSet::default();
```

### Custom Hasher Migration

```rust
// Before - using default hasher
let mut map = HashMap::with_capacity(100);

// After - using FxHash
let mut map = FxHashMap::with_capacity_and_hasher(100, Default::default());
```

## Best Practices

### 1. Type Imports

```rust
// Prefer explicit imports for clarity
use rustc_hash::{FxHashMap, FxHashSet, FxBuildHasher};

// Avoid wildcard imports
// use rustc_hash::*;  // Don't do this
```

### 2. Capacity Planning

```rust
// Pre-allocate when size is known
let mut map = FxHashMap::with_capacity_and_hasher(
    expected_size,
    Default::default()
);

// Use shrink_to_fit after bulk operations
map.shrink_to_fit();
```

### 3. Error Handling

```rust
// Handle allocation failures in constrained environments
match map.try_reserve(large_size) {
    Ok(()) => { /* proceed */ },
    Err(e) => { /* handle allocation failure */ },
}
```

### 4. Generic Code

```rust
use std::hash::{BuildHasher, Hash};
use rustc_hash::FxBuildHasher;

// Write generic code that works with any hasher
fn process_map<K, V, S>(map: &HashMap<K, V, S>)
where
    K: Hash + Eq,
    S: BuildHasher,
{
    // Implementation
}
```

## Common Patterns

### Configuration Storage

```rust
use rustc_hash::FxHashMap;

struct Config {
    settings: FxHashMap<String, String>,
}

impl Config {
    fn new() -> Self {
        Self {
            settings: FxHashMap::default(),
        }
    }
}
```

### Symbol Tables

```rust
use rustc_hash::FxHashMap;

struct SymbolTable {
    symbols: FxHashMap<String, SymbolInfo>,
    reverse_lookup: FxHashMap<usize, String>,
}
```

### Cache Implementation

```rust
use rustc_hash::FxHashMap;

struct Cache<K, V> {
    entries: FxHashMap<K, V>,
    max_size: usize,
}

impl<K: Hash + Eq, V> Cache<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.entries.get(key)
    }

    fn insert(&mut self, key: K, value: V) {
        if self.entries.len() >= self.max_size {
            // Implement eviction policy
        }
        self.entries.insert(key, value);
    }
}
```

## Version Compatibility

- **Current Version**: 2.1
- **Minimum Rust Version**: Check Cargo.toml for current MSRV
- **Stability**: API is stable and used in production by rustc

## License

Dual-licensed under:
- Apache License, Version 2.0
- MIT License

Users may choose either license for their use case.

## Summary

`rustc-hash` provides a performance-focused alternative to Rust's default hashing algorithm. It excels in scenarios where:
- Performance is critical
- Input is trusted
- DoS resistance is not required

The library offers a simple migration path with type aliases that match the standard library's API, making it an excellent choice for performance-critical internal data structures.