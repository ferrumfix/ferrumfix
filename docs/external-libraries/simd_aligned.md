# simd_aligned - Complete Documentation

## Overview

`simd_aligned` is a Rust library that provides safe and fast SIMD-aligned data structures with easy and transparent 'flat' access. Built on top of the [`wide`](https://crates.io/crates/wide/) crate, it enables developers to work with SIMD vectors while maintaining simple, safe scalar access patterns.

**Version**: 0.6.1
**License**: MIT
**Repository**: [https://github.com/ralfbiedert/simd_aligned_rust](https://github.com/ralfbiedert/simd_aligned_rust)
**Rust Version**: 1.83+

## Key Features

- **Safe SIMD Operations**: Built on top of `wide` for easy data handling
- **Comprehensive Type Support**: Everything from `u8x16` to `f64x4`
- **Flat Slice Access**: Think in flat slices (`&[f32]`), but get performance of properly aligned SIMD vectors (`&[f32x4]`)
- **Data Structures**:
  - N-dimensional `VecSimd`
  - NxM-dimensional `MatSimd`
- **Zero Performance Penalty**: No overhead compared to direct SIMD operations
- **Memory Alignment**: Guaranteed proper alignment for all SIMD operations

## Quick Example

```rust
use simd_aligned::{VecSimd, arch::f64x4};

// Create vectors of 10 f64 elements with value 0.0
let mut v1 = VecSimd::<f64x4>::with(0.0, 10);
let mut v2 = VecSimd::<f64x4>::with(0.0, 10);

// Get "flat", mutable view of the vectors
let v1_m = v1.flat_mut();
let v2_m = v2.flat_mut();

// Set individual elements using scalar access
v1_m[0] = 0.0;
v1_m[4] = 4.0;
v1_m[8] = 8.0;

v2_m[1] = 0.0;
v2_m[5] = 5.0;
v2_m[9] = 9.0;

// Perform SIMD vector operations
let sum = v1[0] + v2[0];  // f64x4 + f64x4 in one operation
```

## Module Structure

### `arch` Module

Contains vector definitions with fixed bit widths, re-exported from the `wide` crate.

#### Available SIMD Types:

**Floating-point vectors:**
- `f32x4` - 4 × 32-bit floats (128-bit)
- `f32x8` - 8 × 32-bit floats (256-bit)
- `f64x2` - 2 × 64-bit floats (128-bit)
- `f64x4` - 4 × 64-bit floats (256-bit)

**Signed integer vectors:**
- `i8x16` - 16 × 8-bit signed integers (128-bit)
- `i8x32` - 32 × 8-bit signed integers (256-bit)
- `i16x8` - 8 × 16-bit signed integers (128-bit)
- `i16x16` - 16 × 16-bit signed integers (256-bit)
- `i32x4` - 4 × 32-bit signed integers (128-bit)
- `i32x8` - 8 × 32-bit signed integers (256-bit)
- `i64x2` - 2 × 64-bit signed integers (128-bit)
- `i64x4` - 4 × 64-bit signed integers (256-bit)

**Unsigned integer vectors:**
- `u8x16` - 16 × 8-bit unsigned integers (128-bit)
- `u16x8` - 8 × 16-bit unsigned integers (128-bit)
- `u16x16` - 16 × 16-bit unsigned integers (256-bit)
- `u32x4` - 4 × 32-bit unsigned integers (128-bit)
- `u32x8` - 8 × 32-bit unsigned integers (256-bit)
- `u64x2` - 2 × 64-bit unsigned integers (128-bit)
- `u64x4` - 4 × 64-bit unsigned integers (256-bit)

### `traits` Module

Provides unified views on SIMD types through the `Simd` trait.

## Core Structs

### `VecSimd<T>`

A dynamic (heap allocated) vector aligned for fast and safe SIMD access that also provides a flat view on its data.

```rust
pub struct VecSimd<T>
where
    T: Simd + Default + Clone,
```

#### Methods:

- **`with(t: T::Element, size: usize) -> Self`**
  Creates a new `VecSimd` with all elements initialized to the given value.

  ```rust
  let v = VecSimd::<f64x4>::with(0.0, 10);
  ```

- **`flat(&self) -> &[T::Element]`**
  Returns an immutable flat (non-SIMD) view of the vector's data.

  ```rust
  let v = VecSimd::<f32x4>::with(1.0, 8);
  let flat_view = v.flat();
  assert_eq!(flat_view.len(), 8);
  ```

- **`flat_mut(&mut self) -> &mut [T::Element]`**
  Returns a mutable flat view of the vector's data.

  ```rust
  let mut v = VecSimd::<f32x4>::with(0.0, 4);
  v.flat_mut()[0] = 1.0;
  ```

#### Trait Implementations:
- `Clone`
- `Debug`
- `Deref<Target = [T]>` - Provides slice methods
- `DerefMut`
- `Index<usize>` - Access SIMD vectors by index
- `IndexMut<usize>`

### `MatSimd<T, A>`

A dynamic (heap allocated) matrix with one axis aligned for fast and safe SIMD access.

```rust
pub struct MatSimd<T, A>
where
    T: Simd + Default + Clone,
    A: AccessStrategy,
```

#### Access Strategies:
- `Rows` - Row-optimized access
- `Columns` - Column-optimized access

#### Core Methods:

- **`with_dimension(width: usize, height: usize) -> Self`**
  Creates a new matrix with specified dimensions.

  ```rust
  let mat = MatSimd::<f32x4, Rows>::with_dimension(10, 5);
  ```

- **`dimension(&self) -> (usize, usize)`**
  Returns the matrix dimensions as (rows, columns).

- **`flat(&self) -> MatFlat<'_, T, A>`**
  Returns an immutable flat view of the matrix data.

- **`flat_mut(&mut self) -> MatFlatMut<'_, T, A>`**
  Returns a mutable flat view of the matrix data.

#### Row-Specific Methods (when A = Rows):

- **`row(&self, i: usize) -> &[T]`**
  Returns a row as a slice of SIMD vectors.

- **`row_mut(&mut self, i: usize) -> &mut [T]`**
  Returns a mutable row slice.

- **`row_as_flat(&self, i: usize) -> &[T::Element]`**
  Returns a row as a flat slice of elements.

#### Column-Specific Methods (when A = Columns):

- **`column(&self, i: usize) -> &[T]`**
  Returns a column as a slice of SIMD vectors.

- **`column_mut(&mut self, i: usize) -> &mut [T]`**
  Returns a mutable column slice.

- **`column_as_flat(&self, i: usize) -> &[T::Element]`**
  Returns a column as a flat slice of elements.

### Helper Structs

- **`MatFlat<'a, T, A>`** - Immutable flat view of matrix data
- **`MatFlatMut<'a, T, A>`** - Mutable flat view of matrix data

## Traits

### `Simd` Trait

The core trait that all SIMD vector types must implement.

```rust
pub trait Simd {
    type Element;
    type LanesType;

    const LANES: usize;

    fn splat(t: Self::Element) -> Self;
    fn as_array(&self) -> &[Self::Element];
    fn sum(&self) -> Self::Element;
}
```

#### Associated Types:
- **`Element`** - The scalar element type (e.g., `f32` for `f32x4`)
- **`LanesType`** - Array type representing the vector's elements

#### Associated Constant:
- **`LANES`** - Number of elements in the SIMD vector

#### Required Methods:
- **`splat(t: Self::Element) -> Self`** - Creates a vector with all lanes set to the same value
- **`as_array(&self) -> &[Self::Element]`** - Converts to a slice view
- **`sum(&self) -> Self::Element`** - Computes the sum of all elements

## Utility Functions

### `packed_as_flat<T>(data: &[T]) -> &[T::Element]`

Converts a slice of SIMD vectors into a flat slice of elements.

```rust
use simd_aligned::{packed_as_flat, arch::f32x4};

let packed = [f32x4::splat(1.0); 4];
let flat = packed_as_flat(&packed);
assert_eq!(flat.len(), 16);
assert_eq!(flat[0], 1.0);
```

### `packed_as_flat_mut<T>(data: &mut [T]) -> &mut [T::Element]`

Converts a mutable slice of SIMD vectors into a mutable flat slice of elements.

```rust
use simd_aligned::{packed_as_flat_mut, arch::f32x4};

let mut packed = [f32x4::splat(0.0); 4];
let flat = packed_as_flat_mut(&mut packed);
flat[0] = 1.0;
flat[15] = 2.0;
```

## Performance

Benchmarks show no performance penalty for using `simd_aligned` while retaining simplicity:

```
test vectors::packed       ... bench:          77 ns/iter (+/- 4)
test vectors::scalar       ... bench:       1,177 ns/iter (+/- 464)
test vectors::simd_aligned ... bench:          71 ns/iter (+/- 5)
```

## Use Cases

1. **High-Performance Computing**
   - Matrix operations
   - Vector calculations
   - Scientific computing

2. **Signal Processing**
   - Audio/video processing
   - FFT operations
   - Filter implementations

3. **Machine Learning**
   - Neural network computations
   - Batch processing
   - Feature extraction

4. **Financial Computing**
   - Risk calculations
   - Portfolio optimization
   - Market data processing

## Best Practices

1. **Alignment**: Always use `VecSimd` or `MatSimd` for SIMD data to ensure proper alignment
2. **Batch Operations**: Process multiple elements at once using SIMD vector operations
3. **Memory Access**: Use flat views for scalar operations when needed
4. **Type Selection**: Choose appropriate SIMD width based on your hardware capabilities

## Integration with HFT Systems

For high-frequency trading systems like rusty:

```rust
use simd_aligned::{VecSimd, arch::f64x4};

// Store aligned price data
let mut prices = VecSimd::<f64x4>::with(0.0, 1000);
let mut volumes = VecSimd::<f64x4>::with(0.0, 1000);

// Update via flat access
let prices_flat = prices.flat_mut();
prices_flat[0] = 100.5;

// SIMD operations for calculations
let vwap = (prices[0] * volumes[0]).sum() / volumes[0].sum();
```

## Notes

- The library is designed to be a safe wrapper around SIMD operations
- Future versions may integrate with `std::simd` when it stabilizes
- The current implementation is based on patterns from `packed_simd`
- All operations maintain memory safety while providing maximum performance