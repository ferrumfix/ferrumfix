# `arrayvec` Crate Summary for LLMs

## Overview

The `arrayvec` crate provides two primary data structures: `ArrayVec` and `ArrayString`. These are array-backed, fixed-capacity alternatives to Rust's standard `Vec` and `String` types. Their main advantage is that they store their contents inline, on the stack (if declared as such), which can lead to performance benefits by avoiding heap allocations. This makes them suitable for performance-sensitive applications or environments where heap allocation is undesirable or unavailable (like `no_std` environments).

## Core Types

### 1. `ArrayVec<T, const CAP: usize>`

* **Description**: A vector with a fixed capacity, meaning it can hold up to `CAP` elements of type `T`.
* **Storage**: Elements are stored inline within the `ArrayVec` structure itself.
* **Capacity Limit**: `CAP` is a `usize` but is effectively limited to `u32::MAX`.
* **Key Features**:
    * `new()`: Creates a new, empty `ArrayVec`.
    * `new_const()`: A `const fn` to create a new, empty `ArrayVec` in a const context.
    * `len()`: Returns the current number of elements.
    * `capacity()`: Returns the maximum number of elements the `ArrayVec` can hold (i.e., `CAP`).
    * `is_empty()`: Checks if the `ArrayVec` contains no elements.
    * `is_full()`: Checks if the `ArrayVec` has reached its capacity.
    * `remaining_capacity()`: Returns how many more elements can be added.
    * `push(element: T)`: Adds an element to the end. Panics if full.
    * `try_push(element: T)`: Adds an element to the end. Returns `Err(CapacityError)` if full.
    * `push_unchecked(element: T)`: (unsafe) Adds an element without a capacity check.
    * `pop()`: Removes and returns the last element, if any.
    * `insert(index: usize, element: T)`: Inserts an element at a given index. Panics on error.
    * `try_insert(index: usize, element: T)`: Inserts an element. Returns `Err(CapacityError)` if full. Panics if index is out of bounds.
    * `remove(index: usize)`: Removes and returns the element at `index`, shifting subsequent elements. Panics if index is out of bounds.
    * `swap_remove(index: usize)`: Removes an element at `index` by swapping it with the last element. Faster than `remove`. Panics if index is out of bounds.
    * `clear()`: Removes all elements.
    * `truncate(new_len: usize)`: Shortens the vector to `new_len`.
    * `drain(range: R)`: Creates a draining iterator that removes elements in the specified range.
    * `retain(f: F)`: Retains only elements for which the predicate `f` returns `true`.
    * `as_slice()`: Returns a shared slice `&[T]` of the elements.
    * `as_mut_slice()`: Returns a mutable slice `&mut [T]` of the elements.
    * `into_inner()`: Consumes the `ArrayVec` and returns the inner array `[T; CAP]` if full, otherwise returns `Err(Self)`.
    * `from([T; CAP])`: Creates an `ArrayVec` from a given array, making it full.
* **Dereferencing**: `ArrayVec` dereferences to `&[T]` (immutable) and `&mut [T]` (mutable), allowing usage of all standard slice methods.
* **Iteration**:
    * `&ArrayVec`: Iterates over `&T`.
    * `&mut ArrayVec`: Iterates over `&mut T`.
    * `ArrayVec` (by value): Consumes the `ArrayVec` and iterates over `T` using `IntoIter`.

### 2. `ArrayString<const CAP: usize>`

* **Description**: A string with a fixed capacity, analogous to `ArrayVec` but for `char` data, stored as UTF-8.
* **Storage**: UTF-8 encoded string data is stored inline.
* **Capacity Limit**: `CAP` represents the maximum number of bytes (not characters). Effectively limited to `u32::MAX`.
* **Key Features**:
    * `new()`: Creates a new, empty `ArrayString`.
    * `new_const()`: A `const fn` for creating an empty `ArrayString`.
    * `len()`: Returns the current length in bytes.
    * `capacity()`: Returns the maximum capacity in bytes.
    * `is_empty()`: Checks if the string is empty.
    * `is_full()`: Checks if the string's byte length has reached its capacity.
    * `remaining_capacity()`: Returns how many more bytes can be added.
    * `push(c: char)`: Appends a character. Panics if not enough capacity.
    * `try_push(c: char)`: Appends a character. Returns `Err(CapacityError)` if not enough capacity.
    * `push_str(s: &str)`: Appends a string slice. Panics if not enough capacity.
    * `try_push_str(s: &str)`: Appends a string slice. Returns `Err(CapacityError)` if not enough capacity.
    * `pop()`: Removes and returns the last character, if any.
    * `remove(idx: usize)`: Removes and returns the character at byte position `idx`. Panics if `idx` is not a char boundary or out of bounds.
    * `clear()`: Makes the string empty.
    * `truncate(new_len: usize)`: Shortens the string to `new_len` bytes. Panics if `new_len` is not a char boundary.
    * `as_str()`: Returns a shared string slice `&str`.
    * `as_mut_str()`: Returns a mutable string slice `&mut str`.
    * `from(s: &str)`: Creates an `ArrayString` from a string slice. Returns `Err(CapacityError)` if `s` is too large.
* **Dereferencing**: `ArrayString` dereferences to `&str` (immutable) and `&mut str` (mutable), allowing usage of all standard string slice methods.
* **Traits**: Implements `Write` (from `std::fmt::Write`) for appending formatted text.

## Error Types

### `CapacityError<T = ()>`

* **Description**: An error type indicating that an operation failed due to insufficient capacity in an `ArrayVec` or `ArrayString`.
* **Generic Type `T`**:
    * For operations like `try_push` on `ArrayVec` or `try_push` (char) on `ArrayString`, `T` is the element that could not be inserted.
    * For operations like `try_push_str` on `ArrayString` or `from_str`, `T` is the string slice (`&str`) or `fmt::Error` that failed.
    * It defaults to `()` if the element itself is not important or if it's a general capacity issue.
* **Methods**:
    * `new(element: T)`: Creates a `CapacityError` with the given element.
    * `element(self) -> T`: Consumes the error and returns the overflowing element.
    * `simplify(self) -> CapacityError<()>`: Converts the error to a version without the element.
* **Traits**: Implements `Debug`, `Display`, `Error` (if `std` feature is enabled).

## Iterators

### 1. `Drain<'a, T, const CAP: usize>`

* **Description**: A draining iterator for `ArrayVec`. Created by `ArrayVec::drain()`.
* **Functionality**: Removes elements from a specified range within an `ArrayVec` and yields the removed items. The `ArrayVec` is modified even if the iterator is not fully consumed.
* **Traits**: Implements `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `Drop`. `Send` and `Sync` if `T` is `Send` and `Sync`.

### 2. `IntoIter<T, const CAP: usize>`

* **Description**: A by-value iterator for `ArrayVec`. Created when an `ArrayVec` is consumed by `into_iter()` or a for-loop.
* **Functionality**: Yields elements of the `ArrayVec` by value.
* **Methods**:
    * `as_slice()`: Returns the remaining items as a slice.
    * `as_mut_slice()`: Returns the remaining items as a mutable slice.
* **Traits**: Implements `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `Clone` (if `T` is `Clone`), `Debug` (if `T` is `Debug`), `Drop`. `Send` and `Sync` if `T` is `Send` and `Sync`.

## Cargo Features

* **`std`** (enabled by default):
    * Enables functionality that depends on the Rust standard library (libstd).
    * If disabled, the crate becomes `no_std` compatible.
    * The `Error` trait implementation for `CapacityError` depends on this feature.
* **`serde`**:
    * Enables serialization and deserialization support for `ArrayVec` and `ArrayString` using the Serde library (version 1.x).
* **`zeroize`**:
    * Implements the `Zeroize` trait (from the `zeroize` crate) for `ArrayVec` and `ArrayString`, allowing their contents to be securely overwritten with zeros.

## Rust Version Compatibility
* Requires Rust 1.51 or later.

This summary should provide a good starting point for an LLM to understand and use the `arrayvec` crate.
