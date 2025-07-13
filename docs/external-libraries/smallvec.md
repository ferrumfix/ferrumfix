# SmallVec Crate Documentation

## Overview

The `smallvec` crate provides a "Small vector" optimization: a `Vec`-like container called `SmallVec` that can store a small number of items directly on the stack before falling back to heap allocation. This optimization can lead to significant performance improvements due to:

- Better cache locality
- Reduced allocator traffic
- Fewer heap allocations
- Lower memory fragmentation

If the number of elements exceeds the inline capacity, `SmallVec` "spills" its data onto the heap, allocating a new buffer just like a standard `Vec`. The inline capacity is determined by the backing store type, which must implement the `Array` trait.

### Key Benefits

- **Stack-first allocation**: Small collections stay on the stack
- **Automatic heap fallback**: Seamlessly grows beyond inline capacity
- **API compatibility**: Works like a standard `Vec`
- **no_std support**: Works in embedded environments by default
- **Zero-cost abstraction**: No overhead compared to hand-rolled solutions

### Example Usage

```rust
use smallvec::SmallVec;

// Can store 4 u64 values inline before spilling to heap
let mut v = SmallVec::<[u64; 4]>::new();
for i in 0..4 {
    v.push(i); // Stored inline on stack
}
v.push(4); // Fifth element causes heap allocation
```

## Key Structs and Enums

### `SmallVec<A: Array>`

The primary struct representing the small vector. `A` is the backing array type that defines the inline storage capacity.

**Key Characteristics:**
- Generic over an `Array` type that defines inline storage capacity
- Behaves like `Vec<T>` but with optimized small-element storage
- Automatically manages the transition between stack and heap storage
- Provides O(1) amortized push/pop operations
- Implements most standard `Vec` operations

**Example:**

```rust
use smallvec::SmallVec;

// Empty vector that can hold 4 u8s inline
let mut v = SmallVec::<[u8; 4]>::new();

// Fill inline capacity
v.extend(0..4);
assert_eq!(v.len(), 4);
assert!(!v.spilled()); // Still on stack

// Exceed inline capacity
v.push(4); // Adding a 5th element
assert_eq!(v.len(), 5);
assert!(v.spilled()); // Now on heap

// Access like a normal Vec
let first = v[0];
let slice = v.as_slice();
```

### `Drain<'a, T: 'a + Array>`

An iterator that removes a specified range of items from a `SmallVec` and yields them by value. It's returned by the `SmallVec::drain()` method.

**Key Characteristics:**
- Removes elements from the vector while iterating
- The specified range is removed even if the iterator is not fully consumed
- Implements `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, and `FusedIterator`
- Efficient for bulk removal operations

**Example:**

```rust
let mut v = SmallVec::<[i32; 4]>::from_slice(&[1, 2, 3, 4, 5]);
let drained: Vec<i32> = v.drain(1..4).collect();
assert_eq!(drained, vec![2, 3, 4]);
assert_eq!(v.as_slice(), &[1, 5]);
```

### `IntoIter<A: Array>`

An iterator that consumes a `SmallVec` and yields its items by value. It's returned by `SmallVec::into_iter()`.

**Key Characteristics:**
- Takes ownership of the `SmallVec`
- Yields all elements by value
- Implements `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, and `FusedIterator`
- Provides `as_slice()` and `as_mut_slice()` methods for accessing remaining items
- Implements `Clone` if both `A` and `A::Item` are `Clone`

**Example:**

```rust
let v = SmallVec::<[i32; 4]>::from_slice(&[1, 2, 3, 4]);
let sum: i32 = v.into_iter().sum();
assert_eq!(sum, 10);
// v is now consumed and cannot be used
```

### `CollectionAllocErr`

An enum representing errors that can occur during heap allocation for `SmallVec`.

**Variants:**

1. **`CapacityOverflow`**
   - Occurs when attempting to exceed `usize::MAX`
   - Indicates size computation errors
   - Prevents integer overflow vulnerabilities

2. **`AllocErr { layout: Layout }`**
   - Indicates that the memory allocator returned an error
   - Contains the `Layout` that failed to allocate
   - Provides detailed allocation failure information

**Trait Implementations:**
- `Debug`, `Display` for error reporting
- `From<LayoutError>` for error conversion
- `Send`, `Sync`, `Unpin` auto traits

**Example:**

```rust
let mut v = SmallVec::<[u8; 4]>::new();
match v.try_reserve(usize::MAX) {
    Err(CollectionAllocErr::CapacityOverflow) => {
        println!("Capacity would overflow!");
    }
    Err(CollectionAllocErr::AllocErr { layout }) => {
        println!("Allocation failed for layout: {:?}", layout);
    }
    Ok(()) => unreachable!(),
}
```

## Key Traits

### `Array`

An `unsafe trait` implemented by types that can serve as the backing store for a `SmallVec`. This trait enables the stack-based storage optimization.

**Definition:**

```rust
pub unsafe trait Array {
    type Item;
    fn size() -> usize;
}
```

**Members:**

- **Associated Type `Item`**: The type of elements stored in the array
- **Required Method `size() -> usize`**: Returns the number of items the array can hold (inline capacity)

**Safety:**
- Marked `unsafe` because implementations must guarantee memory safety
- Must provide correct size information for safe memory operations

**Standard Implementation:**

```rust
impl<T, const N: usize> Array for [T; N] {
    type Item = T;
    fn size() -> usize { N }
}
```

**Notes:**
- Not "dyn compatible" (not object-safe)
- Enables compile-time optimization
- Core to the "small vector" optimization strategy

### `ToSmallVec<A: Array>`

A convenience trait for constructing a `SmallVec` from a slice.

**Definition:**

```rust
pub trait ToSmallVec<A: Array> {
    fn to_smallvec(&self) -> SmallVec<A>;
}
```

**Purpose:**
- Provides ergonomic conversion from slices to `SmallVec`
- Optimized for `Copy` types
- Enables method chaining and fluent APIs

**Implementation:**
- Implemented for slices `&[A::Item]` where `A::Item: Copy`
- Uses efficient copying for `Copy` types

**Example:**

```rust
use smallvec::{SmallVec, ToSmallVec};

let slice = &[1, 2, 3, 4];
let sv: SmallVec<[i32; 8]> = slice.to_smallvec();
assert_eq!(sv.as_slice(), slice);
```

## Macros

### `smallvec!`

A macro to create a `SmallVec` with a syntax similar to array expressions.

  * **List of elements**: `smallvec![1, 2, 3];` (Type of array `A` needs to be specified or inferred)

  * **Element and size**: `smallvec![1; 3];` (Creates a `SmallVec` with three copies of `1`)

    This macro uses `clone` for the second form, so be mindful of types with non-standard `Clone` implementations (e.g., `Rc<T>`).

### `smallvec_inline!`

(Enabled by the `const_new` feature)

Creates an *inline* `SmallVec` in a `const` context. The inline storage `A` will always be an array of the size specified by the arguments.

  * **List of elements**: `const V: SmallVec<[i32; 3]> = smallvec_inline![1, 2, 3];`

  * **Element and size**: `const V: SmallVec<[i32; 3]> = smallvec_inline![1; 3];`

    This behaves like array expressions, unlike the regular `smallvec!` macro.

## Core `SmallVec<A: Array>` Methods

### Construction

  * `new() -> SmallVec<A>`: Creates a new, empty `SmallVec`.
  * `with_capacity(n: usize) -> Self`: Creates an empty `SmallVec` with at least `n` capacity (heap allocation if `n` \> inline capacity).
  * `from_vec(vec: Vec<A::Item>) -> SmallVec<A>`: Constructs from a `Vec`. Copies to inline buffer if `vec.capacity()` \<= inline capacity.
  * `from_buf(buf: A) -> SmallVec<A>`: Constructs from a backing array `A` directly (no copy).
  * `from_buf_and_len(buf: A, len: usize) -> SmallVec<A>`: From backing array `A` with a specified length.
  * `unsafe from_buf_and_len_unchecked(buf: MaybeUninit<A>, len: usize) -> SmallVec<A>`: Unsafe version of `from_buf_and_len`. Caller ensures `len <= A::size()`.
  * `new_const() -> Self` (requires `const_new` feature, for `SmallVec<[T; N]>`): `const` version of `new()`.
  * `from_const(items: [T; N]) -> Self` (requires `const_new` feature, for `SmallVec<[T; N]>`): `const` version of `from_buf()`.
  * `unsafe from_const_with_len_unchecked(items: [T; N], len: usize) -> Self` (requires `const_new` feature, for `SmallVec<[T; N]>`): `const` version of `from_buf_and_len_unchecked()`.

### Capacity and Size

  * `len() -> usize`: Returns the number of elements.
  * `is_empty() -> bool`: Returns `true` if `len()` is 0.
  * `capacity() -> usize`: Returns the current total capacity (inline or heap).
  * `inline_size() -> usize`: Returns the maximum number of elements that can be stored inline.
  * `spilled() -> bool`: Returns `true` if data is stored on the heap.
  * `reserve(&mut self, additional: usize)`: Reserves capacity for at least `additional` more elements.
  * `reserve_exact(&mut self, additional: usize)`: Reserves the minimum capacity for `additional` more elements.
  * `try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr>`: Fallible version of `reserve`.
  * `try_reserve_exact(&mut self, additional: usize) -> Result<(), CollectionAllocErr>`: Fallible version of `reserve_exact`.
  * `shrink_to_fit(&mut self)`: Shrinks capacity as much as possible, potentially moving data from heap to inline.
  * `grow(&mut self, new_cap: usize)`: Re-allocates to set capacity to `max(new_cap, inline_size())`. Panics on error.
  * `try_grow(&mut self, new_cap: usize) -> Result<(), CollectionAllocErr>`: Fallible version of `grow`.
  * `unsafe set_len(&mut self, new_len: usize)`: Sets the length directly. Caller must ensure invariants.

### Element Access & Modification

  * `push(&mut self, value: A::Item)`: Appends an element.
  * `pop(&mut self) -> Option<A::Item>`: Removes and returns the last element.
  * `insert(&mut self, index: usize, element: A::Item)`: Inserts an element at `index`.
  * `remove(&mut self, index: usize) -> A::Item`: Removes and returns element at `index`, shifting subsequent elements.
  * `swap_remove(&mut self, index: usize) -> A::Item`: Removes element at `index` by replacing it with the last element (O(1), does not preserve order).
  * `clear(&mut self)`: Removes all elements.
  * `truncate(&mut self, len: usize)`: Shortens the vector to `len` elements.
  * `as_slice() -> &[A::Item]`: Returns a slice of all elements.
  * `as_mut_slice() -> &mut [A::Item]`: Returns a mutable slice of all elements.
  * `as_ptr() -> *const A::Item`: Returns a raw pointer to the buffer.
  * `as_mut_ptr() -> *mut A::Item`: Returns a raw mutable pointer to the buffer.
  * `drain<R>(&mut self, range: R) -> Drain<'_, A>` (where `R: RangeBounds<usize>`): Creates a draining iterator.
  * `retain<F>(&mut self, f: F)` (where `F: FnMut(&A::Item) -> bool`): Retains only elements for which `f` returns `true`.
  * `retain_mut<F>(&mut self, f: F)` (identical to `retain` for API compatibility with `std::Vec`).
  * `dedup(&mut self)` (where `A::Item: PartialEq`): Removes consecutive duplicate elements.
  * `dedup_by<F>(&mut self, same_bucket: F)` (where `F: FnMut(&mut A::Item, &mut A::Item) -> bool`): Removes consecutive duplicates using a custom equality function.
  * `dedup_by_key<F, K>(&mut self, key: F)` (where `F: FnMut(&mut A::Item) -> K, K: PartialEq`): Removes consecutive elements that map to the same key.
  * `resize(&mut self, len: usize, value: A::Item)` (where `A::Item: Clone`): Resizes to `len`, filling new spots with clones of `value`.
  * `resize_with<F>(&mut self, new_len: usize, f: F)` (where `F: FnMut() -> A::Item`): Resizes, filling new spots by calling `f`.

### Conversions

  * `into_vec(self) -> Vec<A::Item>`: Converts to `Vec<A::Item>`. Avoids reallocation if already spilled.
  * `into_boxed_slice(self) -> Box<[A::Item]>`: Converts to `Box<[A::Item]>`. Avoids reallocation if already spilled; drops excess capacity.
  * `into_inner(self) -> Result<A, Self>`: Converts to the backing array `A` if possible (not spilled, full).

### Slice Operations (often via `Deref` and `DerefMut` to `&[T]`)

`SmallVec` dereferences to `&[A::Item]` (and `&mut [A::Item]`), providing a vast array of slice methods, including but not limited to:

  * Sorting: `sort_unstable`, `sort_unstable_by`, `sort_unstable_by_key`, (if `std` is available) `sort`, `sort_by`, `sort_by_key`, `sort_by_cached_key`.
  * Searching: `binary_search`, `binary_search_by`, `binary_search_by_key`, `contains`.
  * Splitting: `split_at`, `split`, `chunks`, `chunks_exact`, `windows`, `rchunks`, `rchunks_exact`.
  * Iterators: `iter`, `iter_mut`.
  * And many more like `first`, `last`, `get`, `get_mut`, `starts_with`, `ends_with`, `reverse`, `rotate_left`, `rotate_right`, etc.

### Methods for `Copy` types (`A::Item: Copy`)

  * `from_slice(slice: &[A::Item]) -> Self`: More efficient construction from a slice for `Copy` types.
  * `insert_from_slice(&mut self, index: usize, slice: &[A::Item])`: More efficient insertion from a slice for `Copy` types.
  * `extend_from_slice(&mut self, slice: &[A::Item])`: More efficient extension from a slice for `Copy` types.

### Unsafe Methods

  * `unsafe from_raw_parts(ptr: *mut A::Item, length: usize, capacity: usize) -> SmallVec<A>`: Creates a `SmallVec` from raw parts. Highly unsafe. Requires `capacity` \> inline capacity.

## `Drain<'a, T: Array>` Methods

(Primarily an iterator)

  * Implements `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`.
  * Implements `Debug`, `Drop`.

## `IntoIter<A: Array>` Methods

(Primarily an iterator)

  * `as_slice(&self) -> &[A::Item]`: Returns remaining items as a slice.
  * `as_mut_slice(&mut self) -> &mut [A::Item]`: Returns remaining items as a mutable slice.
  * Implements `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`.
  * Implements `Clone` (if `A` and `A::Item` are `Clone`), `Debug`, `Drop`.

## Optional Features

### Stable Features

- **`serde`**:
  - Implements `Serialize` and `Deserialize` for `SmallVec`
  - Enables integration with serde ecosystem
  - Use: `smallvec = { version = "1.13", features = ["serde"] }`

- **`write`**:
  - Implements `std::io::Write` for `SmallVec<[u8; _]>`
  - **Requires `std`** (not compatible with `no_std`)
  - Useful for building byte buffers
  - Use: `smallvec = { version = "1.13", features = ["write"] }`

- **`union`**:
  - **Requires Rust 1.49+**
  - Uses union to track inline/heap state without enum tag
  - Reduces `SmallVec` memory footprint
  - Improves memory efficiency for performance-critical code
  - Use: `smallvec = { version = "1.13", features = ["union"] }`

- **`const_generics`**:
  - **Requires Rust 1.51+**
  - Allows `SmallVec` with arrays of any size `[T; N]`
  - More flexible than pre-defined array sizes
  - Use: `smallvec = { version = "1.13", features = ["const_generics"] }`

- **`const_new`**:
  - **Requires Rust 1.51+**
  - Enables const context initialization
  - Adds `new_const()`, `from_const()`, `from_const_with_len_unchecked()`
  - Enables `smallvec_inline!` macro
  - Use: `smallvec = { version = "1.13", features = ["const_new"] }`

### Unstable Features

- **`drain_filter`**:
  - **Unstable API** (may change)
  - Provides `drain_filter` method
  - Iterator that removes elements based on a predicate
  - Use: `smallvec = { version = "1.13", features = ["drain_filter"] }`

- **`drain_keep_rest`**:
  - **Unstable API** (may change)
  - Adds `DrainFilter::keep_rest` method
  - Experimental feature for advanced drain operations
  - Use: `smallvec = { version = "1.13", features = ["drain_keep_rest"] }`

### Nightly-Only Features

- **`specialization`**:
  - **Requires nightly Rust**
  - Improves performance of `SmallVec::from(slice)` for `Copy` types
  - Uses Rust's specialization feature for optimization
  - Use: `smallvec = { version = "1.13", features = ["specialization"] }`

- **`may_dangle`**:
  - **Requires nightly Rust**
  - Relaxes compiler strictness for vectors with borrowed references
  - Advanced lifetime management
  - For expert use only
  - Use: `smallvec = { version = "1.13", features = ["may_dangle"] }`

### Feature Compatibility Notes

- Most features are additive and can be combined
- `write` feature requires `std` and is incompatible with `no_std`
- Unstable features may have breaking changes in future versions
- Nightly features require a nightly Rust toolchain
- Consider your MSRV (Minimum Supported Rust Version) when selecting features

## `no_std` Support

`smallvec` has first-class `no_std` support:

- **Default**: Works in `no_std` environments without configuration
- **Core-only**: Uses only `core` library features by default
- **Allocation**: Still supports heap allocation via `alloc` crate
- **`write` feature**: Only feature that requires `std`

**Example for embedded/no_std:**

```rust
#![no_std]
extern crate alloc;
use smallvec::SmallVec;

// Works perfectly in no_std environment
let mut v = SmallVec::<[u8; 32]>::new();
v.extend_from_slice(b"Hello, embedded world!");
```

## Performance Characteristics

### Memory Layout

- **Inline storage**: Elements stored directly in the struct
- **Size overhead**: Minimal metadata (length, capacity, discriminant)
- **Alignment**: Respects alignment of both array and heap allocations
- **Cache efficiency**: Inline elements typically fit in L1 cache

### Performance Benefits

1. **Allocation Performance**:
   - Zero allocations for small collections
   - Reduced allocator pressure
   - Faster construction/destruction

2. **Access Performance**:
   - Better cache locality for small collections
   - No pointer indirection for inline elements
   - Predictable memory access patterns

3. **When to Use SmallVec**:
   - Collections that are usually small (< 32 elements)
   - Temporary collections in hot paths
   - Avoiding allocations in real-time systems
   - Reducing memory fragmentation

### Trade-offs

- **Stack usage**: Larger stack footprint than `Vec`
- **Move cost**: Moving `SmallVec` is more expensive than `Vec`
- **Size variability**: Performance degrades if size frequently exceeds inline capacity

## Common Patterns and Best Practices

### Choosing Inline Capacity

```rust
// For small, fixed-size collections
type SmallString = SmallVec<[u8; 24]>; // SSO for strings

// For typical use cases
type NodeChildren = SmallVec<[NodeId; 4]>; // Tree with few children

// For performance-critical paths
type TempBuffer = SmallVec<[f64; 16]>; // SIMD-friendly size
```

### Integration with Standard Collections

```rust
// Convert to/from Vec seamlessly
let vec = vec![1, 2, 3];
let small: SmallVec<[i32; 4]> = SmallVec::from_vec(vec);
let vec_back: Vec<i32> = small.into_vec();

// Collect from iterators
let small: SmallVec<[i32; 4]> = (0..3).collect();
```

### Error Handling

```rust
// Use try_* methods for fallible operations
let mut v = SmallVec::<[u8; 4]>::new();
match v.try_reserve(1000) {
    Ok(()) => println!("Capacity reserved"),
    Err(e) => eprintln!("Allocation failed: {:?}", e),
}
```

## Version Compatibility

- **Minimum Rust Version**: 1.36.0 (without features)
- **With `union` feature**: Rust 1.49+
- **With `const_generics`**: Rust 1.51+
- **With `const_new`**: Rust 1.51+
- **Latest stable version**: 1.13.2 (as of documentation)

## Trait Implementations

### Standard Traits

`SmallVec` implements a comprehensive set of standard traits:

- **Collection Traits**: `Default`, `Clone`, `Eq`, `PartialEq`, `Ord`, `PartialOrd`, `Hash`
- **Iterator Traits**: `IntoIterator`, `FromIterator`, `Extend`
- **Conversion Traits**: `From<Vec<T>>`, `From<A>`, `From<&[T]>` (for `T: Copy`)
- **Deref Traits**: `Deref<Target = [T]>`, `DerefMut`
- **Index Traits**: `Index<usize>`, `IndexMut<usize>`, `Index<Range<usize>>`, etc.
- **Debug/Display**: `Debug` (no `Display` implementation)
- **Send/Sync**: Automatically derived when `T` is `Send`/`Sync`
- **AsRef/AsMut**: `AsRef<[T]>`, `AsMut<[T]>`
- **Borrow**: `Borrow<[T]>`, `BorrowMut<[T]>`

### With Features

- **`serde` feature**: `Serialize`, `Deserialize`
- **`write` feature**: `std::io::Write` for `SmallVec<[u8; N]>`

## Advanced Usage Examples

### Custom Inline Sizes for Different Use Cases

```rust
use smallvec::SmallVec;

// Optimize for specific data structures
struct TreeNode {
    value: i32,
    // Most nodes have 2-3 children
    children: SmallVec<[Box<TreeNode>; 3]>,
}

// Network packet processing
struct Packet {
    header: [u8; 20],
    // Most packets are small
    payload: SmallVec<[u8; 512]>,
}

// String interning
struct InternedString {
    // Short strings stored inline
    data: SmallVec<[u8; 23]>,
}
```

### Performance-Critical Loops

```rust
use smallvec::SmallVec;

// Avoid allocations in hot loops
fn process_data(items: &[Item]) -> Vec<Result> {
    let mut results = Vec::with_capacity(items.len());
    // Reusable buffer, stays on stack for small batches
    let mut temp: SmallVec<[f64; 16]> = SmallVec::new();

    for item in items {
        temp.clear();
        temp.extend_from_slice(&item.values);
        // Process without allocation for small items
        let result = compute(&temp);
        results.push(result);
    }
    results
}
```

### Zero-Copy Parsing

```rust
use smallvec::SmallVec;

#[derive(Clone)]
struct ParsedMessage<'a> {
    // Most messages have few fields
    fields: SmallVec<[&'a str; 8]>,
}

impl<'a> ParsedMessage<'a> {
    fn parse(input: &'a str) -> Self {
        let fields: SmallVec<[&str; 8]> = input
            .split(',')
            .collect();
        ParsedMessage { fields }
    }
}
```

## Memory Layout and Overhead

### Size Comparison

```rust
use std::mem::size_of;
use smallvec::SmallVec;

// Standard Vec overhead
assert_eq!(size_of::<Vec<u8>>(), 24); // 3 pointers

// SmallVec overhead (without union feature)
type SV8 = SmallVec<[u8; 8]>;
assert_eq!(size_of::<SV8>(), 24); // 8 bytes inline + metadata

// SmallVec with larger inline capacity
type SV32 = SmallVec<[u8; 32]>;
assert_eq!(size_of::<SV32>(), 48); // 32 bytes inline + metadata

// With union feature enabled (reduced overhead)
// Size is approximately: max(inline_array_size, heap_vec_size) + discriminant
```

### Memory Layout Details

```rust
// Conceptual layout (simplified)
struct SmallVec<A: Array> {
    // Without union feature:
    capacity: usize,
    data: union {
        inline: MaybeUninit<A>,
        heap: (*mut A::Item, usize), // ptr, capacity
    },
    // Discriminant to track inline vs heap
}
```

## Performance Benchmarks

Typical performance characteristics compared to `Vec`:

- **Small collections (< inline capacity)**:
  - Construction: 2-5x faster (no allocation)
  - Push operations: 1.5-2x faster
  - Iteration: Similar performance
  - Drop: 2-3x faster (no deallocation)

- **Large collections (> inline capacity)**:
  - Performance similar to `Vec`
  - Slightly higher memory usage
  - Move operations are slower

## Common Pitfalls and Solutions

### 1. Choosing Too Large Inline Capacity

```rust
// Bad: Wastes stack space
type BadBuffer = SmallVec<[u8; 4096]>;

// Good: Reasonable inline size
type GoodBuffer = SmallVec<[u8; 256]>;
```

### 2. Forgetting About Move Costs

```rust
// Bad: Expensive moves in hot path
fn bad_pattern(mut v: SmallVec<[u64; 32]>) -> SmallVec<[u64; 32]> {
    v.push(42);
    v // Moves 256+ bytes
}

// Good: Use references when possible
fn good_pattern(v: &mut SmallVec<[u64; 32]>) {
    v.push(42);
}
```

### 3. Not Checking Spilled State

```rust
use smallvec::SmallVec;

// Monitor spill behavior in performance-critical code
let mut v = SmallVec::<[u8; 16]>::new();
for i in 0..n {
    v.push(i as u8);
    if v.spilled() {
        // Consider increasing inline capacity
        log::warn!("SmallVec spilled at size {}", v.len());
    }
}
```

## Integration Tips

### With Serde

```rust
use serde::{Serialize, Deserialize};
use smallvec::SmallVec;

#[derive(Serialize, Deserialize)]
struct Config {
    // Serializes just like a Vec
    #[serde(with = "serde_bytes")]
    data: SmallVec<[u8; 32]>,
}
```

### With Rayon

```rust
use rayon::prelude::*;
use smallvec::SmallVec;

// SmallVec works with parallel iterators
let mut data: SmallVec<[i32; 8]> = (0..8).collect();
let sum: i32 = data.par_iter().sum();
```

## HFT-Specific Usage Patterns

### Order Book Updates

```rust
use smallvec::SmallVec;
use rust_decimal::Decimal;

#[derive(Clone)]
struct OrderBookUpdate {
    // Most updates affect < 10 levels
    bids: SmallVec<[(Decimal, Decimal); 10]>,
    asks: SmallVec<[(Decimal, Decimal); 10]>,
}

// Efficient update processing
fn apply_update(book: &mut OrderBook, update: OrderBookUpdate) {
    // No allocation for typical updates
    for (price, size) in update.bids {
        book.update_bid(price, size);
    }
    for (price, size) in update.asks {
        book.update_ask(price, size);
    }
}
```

### Trade Batch Processing

```rust
use smallvec::SmallVec;

// Process trades in small batches
struct TradeBatch {
    // Most batches are small
    trades: SmallVec<[Trade; 16]>,
}

impl TradeBatch {
    fn process(&self) -> SmallVec<[OrderUpdate; 32]> {
        let mut updates = SmallVec::new();
        for trade in &self.trades {
            // Generate updates without allocation
            updates.extend(trade.generate_updates());
        }
        updates
    }
}
```

### Message Queue Optimization

```rust
use smallvec::SmallVec;

// Lock-free message passing with small buffers
type MessageBuffer = SmallVec<[Message; 64]>;

struct MessageQueue {
    // Pre-allocated buffers
    buffers: Vec<MessageBuffer>,
}

impl MessageQueue {
    fn get_buffer(&mut self) -> MessageBuffer {
        self.buffers.pop()
            .unwrap_or_else(|| SmallVec::with_capacity(64))
    }

    fn return_buffer(&mut self, mut buffer: MessageBuffer) {
        buffer.clear();
        if !buffer.spilled() {
            self.buffers.push(buffer);
        }
    }
}
```

## Safety and Soundness

### Unsafe Operations

SmallVec provides several unsafe methods for performance-critical scenarios:

```rust
use smallvec::SmallVec;

// SAFETY: Caller must ensure:
// 1. length <= capacity
// 2. All elements in 0..length are initialized
unsafe {
    let mut v = SmallVec::<[u8; 16]>::new();
    let ptr = v.as_mut_ptr();
    // Direct memory manipulation
    ptr.write(42);
    v.set_len(1);
}

// SAFETY: from_raw_parts requirements:
// 1. ptr must be valid for reads/writes for `capacity` elements
// 2. ptr must be obtained from SmallVec/Vec
// 3. length <= capacity
// 4. capacity > inline_size()
unsafe {
    let ptr = /* ... */;
    let v = SmallVec::<[u8; 4]>::from_raw_parts(ptr, length, capacity);
}
```

### Memory Safety Guarantees

1. **No uninitialized reads**: SmallVec ensures all accessible elements are initialized
2. **Bounds checking**: Index operations panic on out-of-bounds access
3. **Drop safety**: Elements are properly dropped when SmallVec is dropped
4. **Exception safety**: Strong exception safety for most operations

## Debugging and Profiling

### Monitoring Spill Behavior

```rust
#[cfg(debug_assertions)]
macro_rules! track_spills {
    ($v:expr, $context:expr) => {
        if $v.spilled() {
            eprintln!("SmallVec spilled in {}: len={}, cap={}",
                     $context, $v.len(), $v.capacity());
        }
    };
}

// Usage in development
let mut v = SmallVec::<[u8; 32]>::new();
for data in incoming_data {
    v.push(data);
    track_spills!(v, "data processing loop");
}
```

### Performance Profiling

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

// Track allocations from SmallVec
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

struct TrackingAllocator;

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static GLOBAL: TrackingAllocator = TrackingAllocator;
```

## API Quick Reference

### Most Important Methods for HFT

| Method | Description | Complexity |
|--------|-------------|------------|
| `new()` | Create empty SmallVec | O(1) |
| `with_capacity(n)` | Pre-allocate capacity | O(1) or O(n) |
| `from_slice(&[T])` | Create from slice (Copy types) | O(n) |
| `push(value)` | Append element | O(1) amortized |
| `extend_from_slice(&[T])` | Bulk append (Copy types) | O(n) |
| `clear()` | Remove all elements | O(n) |
| `truncate(len)` | Shrink to length | O(removed) |
| `spilled()` | Check if using heap | O(1) |
| `as_slice()` | Get slice view | O(1) |
| `as_mut_slice()` | Get mutable slice | O(1) |

## See Also

- [`tinyvec`](https://docs.rs/tinyvec): 100% safe alternative with similar optimization
- [`arrayvec`](https://docs.rs/arrayvec): Stack-only vectors (no heap fallback)
- [`heapless`](https://docs.rs/heapless): Collections for `no_std` without allocator
- [`thin-vec`](https://docs.rs/thin-vec): Reduces `Vec` size to single pointer
- [SmallVec GitHub](https://github.com/servo/rust-smallvec): Source code and issues
- [Servo Project](https://servo.org/): Original creators of SmallVec
