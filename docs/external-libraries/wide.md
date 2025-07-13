# Wide Crate Documentation

## Overview

The `wide` crate is a Rust library that provides SIMD-compatible data types for high-performance vectorized operations. It aims to leverage explicit SIMD instructions when possible, with automatic fallback to LLVM optimizations for platforms without native SIMD support.

**Current Version**: 0.7.32

## Key Features

- **Portable SIMD**: Works across multiple platforms (x86_64, i686, ARM)
- **Safe Abstractions**: Provides safe Rust interfaces to SIMD operations
- **Automatic Optimization**: Falls back to scalar operations that LLVM can often vectorize
- **Zero-Overhead**: Designed for performance-critical applications
- **NaN-Safe**: Proper handling of NaN values in floating-point operations

## Crate Features

- `std`: Links to the standard library
  - Improves performance of `sqrt` when explicit SIMD isn't available
  - Enables additional optimizations

## Dependencies

- `bytemuck ^1` - For safe transmutation between types
- `safe_arch ^0.7` - Safe wrappers around architecture-specific intrinsics
- `serde ^1` (optional) - Serialization support

## SIMD Vector Types

### Floating-Point Vectors

#### f32x4
```rust
#[repr(C, align(16))]
pub struct f32x4 { /* private fields */ }
```
- 4-lane 32-bit floating-point vector
- 16-byte aligned
- Ideal for single-precision calculations

#### f32x8
```rust
#[repr(C, align(32))]
pub struct f32x8 { /* private fields */ }
```
- 8-lane 32-bit floating-point vector
- 32-byte aligned
- AVX/AVX2 optimized on x86_64

#### f64x2
```rust
#[repr(C, align(16))]
pub struct f64x2 { /* private fields */ }
```
- 2-lane 64-bit floating-point vector
- 16-byte aligned
- SSE2 optimized

#### f64x4
```rust
#[repr(C, align(32))]
pub struct f64x4 { /* private fields */ }
```
- 4-lane 64-bit floating-point vector
- 32-byte aligned
- AVX/AVX2 optimized
- **Recommended for HFT applications requiring double precision**

### Integer Vectors

#### Signed Integers
- `i8x16` - 16-lane 8-bit signed integers
- `i8x32` - 32-lane 8-bit signed integers
- `i16x8` - 8-lane 16-bit signed integers
- `i16x16` - 16-lane 16-bit signed integers
- `i32x4` - 4-lane 32-bit signed integers
- `i32x8` - 8-lane 32-bit signed integers
- `i64x2` - 2-lane 64-bit signed integers
- `i64x4` - 4-lane 64-bit signed integers

#### Unsigned Integers
- `u8x16` - 16-lane 8-bit unsigned integers
- `u8x32` - 32-lane 8-bit unsigned integers (AVX2)
- `u16x8` - 8-lane 16-bit unsigned integers
- `u16x16` - 16-lane 16-bit unsigned integers
- `u32x4` - 4-lane 32-bit unsigned integers
- `u32x8` - 8-lane 32-bit unsigned integers
- `u64x2` - 2-lane 64-bit unsigned integers
- `u64x4` - 4-lane 64-bit unsigned integers

## Common Operations

### Constructors

```rust
// Create from array
let v = f64x4::new([1.0, 2.0, 3.0, 4.0]);

// Create with same value in all lanes
let v = f64x4::splat(100.0);

// Convert from arrays
let v = f64x4::from([1.0, 2.0, 3.0, 4.0]);

// Convert from integer vectors
let ints = i32x4::new([1, 2, 3, 4]);
let floats = f32x4::from_i32x4(ints);
```

### Arithmetic Operations

All vector types support standard arithmetic operations:

```rust
let a = f64x4::splat(10.0);
let b = f64x4::splat(20.0);

let sum = a + b;        // Addition
let diff = a - b;       // Subtraction
let prod = a * b;       // Multiplication
let quot = a / b;       // Division
let neg = -a;           // Negation
```

### Mathematical Functions

Floating-point vectors provide extensive mathematical operations:

```rust
let v = f64x4::splat(2.0);

// Basic functions
let abs_v = v.abs();
let sqrt_v = v.sqrt();
let recip = v.recip();      // 1/v

// Trigonometric
let sin_v = v.sin();
let cos_v = v.cos();
let tan_v = v.tan();
let asin_v = v.asin();
let acos_v = v.acos();
let atan_v = v.atan();

// Exponential and logarithmic
let exp_v = v.exp();
let ln_v = v.ln();
let log2_v = v.log2();
let log10_v = v.log10();

// Rounding
let floor_v = v.floor();
let ceil_v = v.ceil();
let round_v = v.round();

// Power
let squared = v.powf(2.0);
```

### Comparison Operations

The wide crate provides comparison traits that return mask vectors:

```rust
use wide::{CmpEq, CmpLt, CmpGt};

let a = f64x4::new([1.0, 2.0, 3.0, 4.0]);
let b = f64x4::new([1.0, 5.0, 3.0, 2.0]);

// Returns mask vector with all-1s for true, all-0s for false
let eq_mask = a.cmp_eq(b);  // [true, false, true, false]
let lt_mask = a.cmp_lt(b);  // [false, true, false, false]
let gt_mask = a.cmp_gt(b);  // [false, false, false, true]

// Check for special values
let is_nan = a.is_nan();
let is_finite = a.is_finite();
let is_inf = a.is_inf();
```

### Bitwise Operations

Integer and mask types support bitwise operations:

```rust
let a = u32x4::splat(0xFF00FF00);
let b = u32x4::splat(0x00FF00FF);

let and_result = a & b;
let or_result = a | b;
let xor_result = a ^ b;
let not_result = !a;
```

### Lane Access and Manipulation

```rust
let v = f64x4::new([1.0, 2.0, 3.0, 4.0]);

// Convert to array
let arr: [f64; 4] = v.to_array();

// Access individual lanes (through array conversion)
let first = arr[0];
let last = arr[3];
```

### Reduction Operations

While not explicitly documented, typical SIMD reduction operations may include:
- `reduce_sum()` - Sum all lanes
- `reduce_min()` - Minimum across lanes
- `reduce_max()` - Maximum across lanes

## Mathematical Constants

Floating-point types define useful constants:

```rust
f64x4::ZERO;           // All zeros
f64x4::ONE;            // All ones
f64x4::HALF;           // All 0.5
f64x4::E;              // Euler's number
f64x4::PI;             // Pi
f64x4::TAU;            // 2*Pi
f64x4::SQRT_2;         // Square root of 2
f64x4::FRAC_PI_2;      // Pi/2
f64x4::LN_2;           // Natural log of 2
```

## Performance Considerations

### Memory Alignment

- All types are properly aligned for their respective SIMD instructions
- `f32x4`, `f64x2`: 16-byte alignment (SSE)
- `f32x8`, `f64x4`: 32-byte alignment (AVX)
- Proper alignment ensures optimal performance

### NaN Handling

- Floating-point operations properly propagate NaN values
- Use `is_nan()` to check for NaN values
- NaN-safe comparisons available through comparison traits

### Platform-Specific Optimizations

The crate automatically selects the best available instructions:
- **x86_64**: SSE2, SSE4.1, AVX, AVX2, FMA
- **ARM**: NEON (where available)
- **Fallback**: Scalar operations that LLVM can often vectorize

## Usage Examples

### Basic Vector Math
```rust
use wide::f64x4;

fn weighted_average(prices: f64x4, volumes: f64x4) -> f64 {
    let weighted = prices * volumes;
    let total_weighted = weighted.reduce_sum();
    let total_volume = volumes.reduce_sum();
    total_weighted / total_volume
}
```

### SIMD Order Book Processing
```rust
use wide::{f64x4, CmpGt};

fn find_arbitrage_opportunities(bid_prices: f64x4, ask_prices: f64x4) -> f64x4 {
    // Find where bid > ask (arbitrage opportunity)
    let opportunities = bid_prices.cmp_gt(ask_prices);
    let spread = bid_prices - ask_prices;

    // Mask out non-opportunities
    spread & opportunities
}
```

### NaN-Safe Division
```rust
use wide::f64x4;

fn safe_divide(numerator: f64x4, denominator: f64x4) -> f64x4 {
    // The division operation handles NaN/Inf cases automatically
    numerator / denominator
}
```

## Integration with simd_aligned

When using `wide` with `simd_aligned` for HFT applications:

```rust
use simd_aligned::VecSimd;
use wide::f64x4;

// Create SIMD-aligned buffer
let mut prices = VecSimd::<f64x4>::with(0.0, 1000);

// Perform SIMD operations
for i in 0..prices.len() {
    prices[i] = prices[i] * f64x4::splat(1.01);  // 1% increase
}
```

## Best Practices

1. **Use appropriate vector width**: Match vector width to your data size and CPU capabilities
2. **Maintain alignment**: Use proper alignment for optimal performance
3. **Batch operations**: Process multiple elements together to maximize SIMD utilization
4. **Handle NaN carefully**: Use NaN-checking methods when needed
5. **Profile your code**: Verify that SIMD operations provide expected speedup

## Platform Support

- **Tier 1**: x86_64 (Windows, Linux, macOS)
- **Tier 2**: i686, ARM64
- **Fallback**: Any platform with Rust support (scalar operations)

## License

Available under any of:
- Zlib License
- Apache License 2.0
- MIT License

## Resources

- [Repository](https://github.com/Lokathor/wide)
- [Crates.io](https://crates.io/crates/wide)
- [Documentation](https://docs.rs/wide)