# SmallVec Crate Documentation

## Overview

The `smallvec` crate provides a `Vec`-like container called `SmallVec`. It's designed to store a small number of elements inline, directly within the `SmallVec` struct itself, rather than on the heap. This can lead to performance improvements due to better cache locality and reduced allocator traffic, especially for workloads where the number of elements frequently fits within this inline buffer. If the number of elements exceeds the inline capacity, `SmallVec` "spills" its data onto the heap, allocating a new buffer just like a standard `Vec`.

The inline capacity is determined by the backing store type, which must implement the `Array` trait. Typically, this is a fixed-size array. For example, `SmallVec<[u64; 8]>` can store up to eight `u64` values inline.

## Key Structs and Enums

### `SmallVec<A: Array>`

The primary struct representing the small vector. `A` is the backing array type.

**Example:**

```rust
use smallvec::SmallVec;
let mut v = SmallVec::<[u8; 4]>::new(); // Empty vector, can hold 4 u8s inline

v.extend(0..4);
assert_eq!(v.len(), 4);
assert!(!v.spilled()); // Not spilled to heap

v.push(4); // Adding a 5th element
assert_eq!(v.len(), 5);
assert!(v.spilled()); // Now spilled to heap
````

### `Drain<'a, T: 'a + Array>`

An iterator that removes a specified range of items from a `SmallVec` and yields them by value. It's returned by the `SmallVec::drain()` method. The specified range is removed even if the iterator is not fully consumed.

### `IntoIter<A: Array>`

An iterator that consumes a `SmallVec` and yields its items by value. It's returned by `SmallVec::into_iter()`.

### `CollectionAllocErr`

An enum representing errors that can occur during heap allocation for `SmallVec`.

  * **`CapacityOverflow`**: Indicates that a capacity calculation overflowed `usize` or another size computation error occurred.
  * **`AllocErr { layout: Layout }`**: Indicates that the memory allocator returned an error for the given `Layout`.

## Key Traits

### `Array`

A trait implemented by types that can serve as the backing store for a `SmallVec`.

  * **Associated Type `Item`**: The type of elements stored in the array.

  * **Required Method `size() -> usize`**: Returns the number of items the array can hold (i.e., the inline capacity).

    Currently implemented for fixed-size arrays like `[T; N]`.

### `ToSmallVec<A: Array>`

A convenience trait for constructing a `SmallVec` from a slice.

  * **Required Method `to_smallvec(&self) -> SmallVec<A>`**: Constructs a new `SmallVec` from a slice.

    Implemented for slices `&[A::Item]` where `A::Item` is `Copy`.

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

  * **`serde`**: Implements `Serialize` and `Deserialize` for `SmallVec`.
  * **`write`**: (Requires `std`) Implements `std::io::Write` for `SmallVec<[u8; _]>`.
  * **`union`**: (Rust 1.49+) Reduces `SmallVec` size by removing the enum tag for inline/spilled state.
  * **`const_generics`**: (Rust 1.51+) Allows `SmallVec` with any array size `[T; N]`.
  * **`const_new`**: (Rust 1.51+) Enables `const` initialization functions like `SmallVec::new_const()` and the `smallvec_inline!` macro.
  * **`drain_filter`**: (Unstable) Enables `drain_filter` method.
  * **`drain_keep_rest`**: (Unstable) Enables `DrainFilter::keep_rest` method.
  * **`specialization`**: (Nightly) Improves `SmallVec::from(slice)` for `Copy` types.
  * **`may_dangle`**: (Nightly) Relaxes compiler checks for vectors with borrowed references.

## `no_std` Support

`smallvec` supports `no_std` by default. The `write` feature enables `std` dependency.
