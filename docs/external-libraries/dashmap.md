# dashmap - Concurrent HashMap for Rust

## Overview

DashMap is a blazingly fast concurrent HashMap implementation for Rust, designed as a drop-in replacement for `RwLock<HashMap<K, V>>`. It provides thread-safe operations without requiring explicit locking, making it ideal for high-performance concurrent applications.

**Key Features:**
- Thread-safe concurrent HashMap
- Methods take `&self` instead of `&mut self` for easy sharing
- Can be wrapped in `Arc<T>` for multi-thread access
- Sharded internal design for improved concurrent performance
- API similar to `std::collections::HashMap`
- Minimum Supported Rust Version (MSRV): 1.70

## Installation

```toml
[dependencies]
dashmap = "5.5"  # Check for latest version
```

### Feature Flags

- `serde`: Enables serialization/deserialization support
- `raw-api`: Provides access to unstable raw-shard API
- `rayon`: Enables parallel iteration support
- `inline-more`: Enables aggressive inlining for potential performance gains
- `arbitrary`: Adds support for the `arbitrary` crate

## Core Data Structures

### DashMap

The primary concurrent HashMap implementation.

```rust
use dashmap::DashMap;

// Create a new empty DashMap
let map = DashMap::new();

// Create with initial capacity
let map = DashMap::with_capacity(100);

// Create with custom hasher
let map = DashMap::with_hasher(MyHasher::default());

// Create with specific shard count (for tuning)
let map = DashMap::with_shard_amount(16);
```

### DashSet

A concurrent set implementation, essentially a thin wrapper around `DashMap<K, ()>`.

```rust
use dashmap::DashSet;

let set = DashSet::new();
set.insert("value");
assert!(set.contains("value"));
```

### ReadOnlyView

Provides immutable, read-only access to a DashMap's contents.

```rust
let map = DashMap::new();
// ... populate map
let read_view = map.into_read_only();
// Can only read, not modify
```

## Basic Operations

### Insertion

```rust
let map = DashMap::new();

// Basic insert - returns previous value if exists
let previous = map.insert("key", "value");

// Using entry API for more control
map.entry("key").or_insert("default_value");

// Insert only if key doesn't exist
map.entry("key").or_insert_with(|| expensive_computation());
```

### Retrieval

```rust
// Get immutable reference
if let Some(value) = map.get("key") {
    println!("Value: {}", *value);
}

// Get mutable reference
if let Some(mut value) = map.get_mut("key") {
    *value += 1;
}

// Try get without blocking (non-blocking attempt)
if let Some(value) = map.try_get("key") {
    println!("Got value: {}", *value);
}

// Check if key exists
if map.contains_key("key") {
    println!("Key exists");
}
```

### Removal

```rust
// Remove and return value
if let Some((_key, value)) = map.remove("key") {
    println!("Removed: {}", value);
}

// Conditional removal
map.remove_if("key", |_k, v| v == &"remove_me");

// Clear all entries
map.clear();
```

### Iteration

```rust
// Iterate over immutable references
for ref_multi in map.iter() {
    let (key, value) = ref_multi.pair();
    println!("{}: {}", key, value);
}

// Iterate over mutable references
map.iter_mut().for_each(|mut ref_multi| {
    *ref_multi.value_mut() += 1;
});

// Parallel iteration with Rayon (requires `rayon` feature)
#[cfg(feature = "rayon")]
{
    use rayon::prelude::*;
    map.par_iter().for_each(|ref_multi| {
        // Process in parallel
    });
}
```

## Advanced Usage

### Entry API

The entry API provides fine-grained control over map operations:

```rust
use dashmap::mapref::entry::Entry;

match map.entry("key") {
    Entry::Occupied(mut entry) => {
        // Key exists, modify value
        *entry.get_mut() += 1;
    }
    Entry::Vacant(entry) => {
        // Key doesn't exist, insert
        entry.insert(42);
    }
}

// Or more concisely:
map.entry("key")
   .and_modify(|v| *v += 1)
   .or_insert(0);
```

### Altering Values

```rust
// Apply a function to modify a value
map.alter("key", |_k, v| v * 2);

// Apply a function to all values
map.alter_all(|_k, v| *v += 1);
```

### Capacity Management

```rust
// Get current capacity
let capacity = map.capacity();

// Check if empty
if map.is_empty() {
    println!("Map is empty");
}

// Get number of elements
let len = map.len();

// Shrink to fit
map.shrink_to_fit();
```

## Concurrent Usage Patterns

### Thread Sharing with Arc

```rust
use std::sync::Arc;
use std::thread;

let map = Arc::new(DashMap::new());

// Spawn multiple threads
let handles: Vec<_> = (0..4)
    .map(|i| {
        let map_clone = Arc::clone(&map);
        thread::spawn(move || {
            map_clone.insert(i, i * 2);
        })
    })
    .collect();

// Wait for all threads
for handle in handles {
    handle.join().unwrap();
}
```

### Producer-Consumer Pattern

```rust
use std::sync::Arc;
use std::thread;
use std::time::Duration;

let map = Arc::new(DashMap::new());

// Producer thread
let producer_map = Arc::clone(&map);
let producer = thread::spawn(move || {
    for i in 0..10 {
        producer_map.insert(i, i * i);
        thread::sleep(Duration::from_millis(10));
    }
});

// Consumer thread
let consumer_map = Arc::clone(&map);
let consumer = thread::spawn(move || {
    loop {
        if consumer_map.len() >= 10 {
            break;
        }

        // Process available items
        consumer_map.iter().for_each(|entry| {
            println!("Processing: {} -> {}", entry.key(), entry.value());
        });

        thread::sleep(Duration::from_millis(5));
    }
});

producer.join().unwrap();
consumer.join().unwrap();
```

## Performance Characteristics

### Sharding

DashMap uses internal sharding to reduce contention:
- Default shard count is based on available parallelism
- Each shard has its own lock
- Operations typically only lock one shard
- Can be tuned with `with_shard_amount()`

### Lock Contention

- Read operations can proceed concurrently on the same shard
- Write operations require exclusive access to a shard
- Different shards can be accessed concurrently
- Iteration locks shards one at a time

### Best Practices for Performance

1. **Use appropriate shard count**: More shards reduce contention but increase memory overhead
2. **Prefer batch operations**: Use `extend()` for bulk inserts
3. **Use entry API**: More efficient than separate get/insert operations
4. **Consider read-only views**: For read-heavy workloads
5. **Enable `inline-more`**: For hot paths requiring maximum performance

## Comparison with Alternatives

### vs `RwLock<HashMap>`

**Advantages:**
- No explicit lock management
- Better concurrent performance
- Prevents common deadlock scenarios
- More ergonomic API

**Disadvantages:**
- Slightly higher memory overhead
- Not as flexible for complex locking patterns

### vs `parking_lot::RwLock<HashMap>`

**Advantages:**
- Sharded design reduces contention
- Built-in concurrent iteration
- No manual lock management

**Disadvantages:**
- Less control over locking behavior
- Higher memory usage due to sharding

## Common Pitfalls and Solutions

### 1. Holding References Too Long

```rust
// BAD: Holds lock on shard
let value = map.get("key").unwrap();
expensive_operation(); // Lock still held!

// GOOD: Clone or drop reference
let value = map.get("key").unwrap().clone();
expensive_operation(); // No lock held
```

### 2. Excessive Iteration

```rust
// BAD: Iterating to find one item
for entry in map.iter() {
    if entry.key() == &target_key {
        return Some(entry.value().clone());
    }
}

// GOOD: Direct lookup
map.get(&target_key).map(|v| v.clone())
```

### 3. Not Using Entry API

```rust
// BAD: Multiple lookups
if !map.contains_key(&key) {
    map.insert(key, compute_value());
}

// GOOD: Single lookup with entry
map.entry(key).or_insert_with(compute_value);
```

## Examples

### Word Counter

```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::thread;

fn count_words(text: &str) -> Arc<DashMap<String, usize>> {
    let word_counts = Arc::new(DashMap::new());
    let words: Vec<_> = text.split_whitespace().collect();

    let chunk_size = words.len() / 4;
    let mut handles = vec![];

    for chunk in words.chunks(chunk_size) {
        let counts = Arc::clone(&word_counts);
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            for word in chunk {
                counts.entry(word.to_string())
                     .and_modify(|count| *count += 1)
                     .or_insert(1);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    word_counts
}
```

### Cache Implementation

```rust
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Cache<K, V> {
    data: Arc<DashMap<K, (V, Instant)>>,
    ttl: Duration,
}

impl<K: Eq + std::hash::Hash, V: Clone> Cache<K, V> {
    fn new(ttl: Duration) -> Self {
        Self {
            data: Arc::new(DashMap::new()),
            ttl,
        }
    }

    fn get(&self, key: &K) -> Option<V> {
        self.data.get(key).and_then(|entry| {
            let (value, timestamp) = entry.pair();
            if timestamp.elapsed() < self.ttl {
                Some(value.clone())
            } else {
                drop(entry);
                self.data.remove(key);
                None
            }
        })
    }

    fn insert(&self, key: K, value: V) {
        self.data.insert(key, (value, Instant::now()));
    }

    fn cleanup(&self) {
        self.data.retain(|_, (_, timestamp)| {
            timestamp.elapsed() < self.ttl
        });
    }
}
```

## Safety and Thread Safety

DashMap is designed with safety as a primary concern:

- All operations are thread-safe by design
- No unsafe code in the public API
- Internal unsafe usage is carefully audited
- Implements `Send` and `Sync` for thread sharing
- Prevents data races at compile time

## Dependencies

Core dependencies include:
- `hashbrown`: High-performance hash table implementation
- `parking_lot_core`: Efficient synchronization primitives
- `lock_api`: Generic locking traits
- `crossbeam-utils`: Concurrent programming utilities

## Resources

- [Documentation](https://docs.rs/dashmap/)
- [GitHub Repository](https://github.com/xacrimon/dashmap)
- [Benchmarks](https://github.com/xacrimon/conc-map-bench)
- License: MIT

## Summary

DashMap provides a powerful, efficient solution for concurrent HashMap needs in Rust. Its sharded design, intuitive API, and strong safety guarantees make it an excellent choice for multi-threaded applications requiring shared mutable state. While it has slightly higher memory overhead than a simple `RwLock<HashMap>`, the performance benefits and ease of use often outweigh this cost in concurrent scenarios.