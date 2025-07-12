[Docs.rs](https://docs.rs/)

- [simd\_aligned-0.6.1](https://docs.rs/simd_aligned/0.6.1/simd_aligned/# "Safe and fast SIMD-aligned data structures with easy and transparent 'flat' access.")


- simd\_aligned 0.6.1

- [Docs.rs crate page](https://docs.rs/crate/simd_aligned/0.6.1 "See simd_aligned in docs.rs")
- [MIT](https://spdx.org/licenses/MIT)

- Links
- [Repository](https://github.com/ralfbiedert/simd_aligned_rust)
- [crates.io](https://crates.io/crates/simd_aligned "See simd_aligned in crates.io")
- [Source](https://docs.rs/crate/simd_aligned/0.6.1/source/ "Browse source of simd_aligned-0.6.1")

- Owners
- [ralfbiedert](https://crates.io/users/ralfbiedert)

- Dependencies
-
    - [wide ^0.7.30\\
      \\
      _normal_](https://docs.rs/wide/^0.7.30)


- Versions

- [**93.75%**\\
  of the crate is documented](https://docs.rs/crate/simd_aligned/0.6.1)

- [Platform](https://docs.rs/simd_aligned/0.6.1/simd_aligned/#)  - [i686-pc-windows-msvc](https://docs.rs/crate/simd_aligned/0.6.1/target-redirect/i686-pc-windows-msvc/simd_aligned/index.html)
    - [i686-unknown-linux-gnu](https://docs.rs/crate/simd_aligned/0.6.1/target-redirect/i686-unknown-linux-gnu/simd_aligned/index.html)
    - [x86\_64-apple-darwin](https://docs.rs/crate/simd_aligned/0.6.1/target-redirect/x86_64-apple-darwin/simd_aligned/index.html)
    - [x86\_64-pc-windows-msvc](https://docs.rs/crate/simd_aligned/0.6.1/target-redirect/x86_64-pc-windows-msvc/simd_aligned/index.html)
    - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/simd_aligned/0.6.1/target-redirect/x86_64-unknown-linux-gnu/simd_aligned/index.html)
- [Feature flags](https://docs.rs/crate/simd_aligned/0.6.1/features "Browse available feature flags of simd_aligned-0.6.1")

- [docs.rs](https://docs.rs/simd_aligned/0.6.1/simd_aligned/#)  - [About docs.rs](https://docs.rs/about)
    - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/simd_aligned/0.6.1/simd_aligned/#)  - [Rust website](https://www.rust-lang.org/)
    - [The Book](https://doc.rust-lang.org/book/)
    - [Standard Library API Reference](https://doc.rust-lang.org/std/)
    - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
    - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

# Crate simd\_alignedCopy item path

[Settings](https://docs.rs/simd_aligned/0.6.1/settings.html)

[Help](https://docs.rs/simd_aligned/0.6.1/help.html)

Summary[Source](https://docs.rs/simd_aligned/0.6.1/src/simd_aligned/lib.rs.html#1-108)

Expand description

[![crates.io-badge](https://img.shields.io/crates/v/simd_aligned.svg)](https://crates.io/crates/simd_aligned)[![docs.rs-badge](https://docs.rs/simd_aligned/badge.svg)](https://docs.rs/simd_aligned/)![license-badge](https://img.shields.io/badge/license-MIT-blue.svg)[![rust-version-badge](https://img.shields.io/badge/rust-1.83%2B-blue.svg?maxAge=3600)](https://github.com/ralfbiedert/simd_aligned)[![rust-build-badge](https://github.com/ralfbiedert/simd_aligned/actions/workflows/rust.yml/badge.svg)](https://github.com/ralfbiedert/simd_aligned/actions/workflows/rust.yml)

## [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#in-one-sentence) In One Sentence

You want to use safe SIMD datatypes from [`wide`](https://crates.io/crates/wide/) but realized there is no simple, safe
and fast way to align your `f32x4` (and friends) in memory _and_ treat them as regular `f32` slices for easy loading and
manipulation; `simd_aligned` to the rescue.

## [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#highlights) Highlights

- built on top of [`wide`](https://crates.io/crates/wide/) for easy data handling
- supports everything from `u8x16` to `f64x4`
- think in flat slices ( `&[f32]`), but get performance of properly aligned SIMD vectors ( `&[f32x4]`)
- provides N-dimensional [`VecSimd`](https://docs.rs/simd_aligned/latest/simd_aligned/struct.VecSimd.html) and
  NxM-dimensional [`MatSimd`](https://docs.rs/simd_aligned/latest/simd_aligned/struct.MatSimd.html).

## [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#examples) Examples

Produces a vector that can hold `10` elements of type `f64`. All elements are guaranteed to be properly aligned for fast
access.

```
use simd_aligned::{VecSimd, arch::f64x4};

// Create vectors of `10` f64 elements with value `0.0`.
let mut v1 = VecSimd::<f64x4>::with(0.0, 10);
let mut v2 = VecSimd::<f64x4>::with(0.0, 10);

// Get "flat", mutable view of the vector, and set individual elements:
let v1_m = v1.flat_mut();
let v2_m = v2.flat_mut();

// Set some elements on v1
v1_m[0] = 0.0;
v1_m[4] = 4.0;
v1_m[8] = 8.0;

// Set some others on v2
v2_m[1] = 0.0;
v2_m[5] = 5.0;
v2_m[9] = 9.0;

let mut sum = f64x4::splat(0.0);

// Eventually, do something with the actual SIMD types. Does
// `std::simd` vector math, e.g., f64x8 + f64x8 in one operation:
sum = v1[0] + v2[0];
```

## [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#benchmarks) Benchmarks

There is no performance penalty for using `simd_aligned`, while retaining all the
simplicity of handling flat arrays.

[ⓘ](https://docs.rs/simd_aligned/0.6.1/simd_aligned/# "This example is not tested")

```
test vectors::packed       ... bench:          77 ns/iter (+/- 4)
test vectors::scalar       ... bench:       1,177 ns/iter (+/- 464)
test vectors::simd_aligned ... bench:          71 ns/iter (+/- 5)
```

## [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#status) Status

- December 2024: Compiles on stable.
- March 2023: Compiles again on latest Rust nightly.
- August 2018: Initial version.

## [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#faq) FAQ

#### [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#how-does-it-relate-to-faster-and-stdsimd) How does it relate to [faster](https://github.com/AdamNiederer/faster) and [

`std::simd`](https://github.com/rust-lang-nursery/packed_simd/)?

- `simd_aligned` builds on top of `std::simd`. At aims to provide common, SIMD-aligned
  data structure that support simple and safe scalar access patterns.

- `faster` (as of today) is good if you already have exiting flat slices in your code
  and want to operate them “full SIMD ahead”. However, in particular when dealing with multiple
  slices at the same time (e.g., kernel computations) the performance impact of unaligned arrays can
  become a bit more noticeable (e.g., in the case of [ffsvm](https://github.com/ralfbiedert/ffsvm-rust/) up to 10% -
  20%).

## Modules [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#modules)

[arch](https://docs.rs/simd_aligned/0.6.1/simd_aligned/arch/index.html "mod simd_aligned::arch")Contains vector
definitions with a fixed bit width, reexported
from [wide](https://crates.io/crates/wide)[traits](https://docs.rs/simd_aligned/0.6.1/simd_aligned/traits/index.html "mod simd_aligned::traits")
Unified views on SIMD types.

## Structs [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#structs)

[MatFlat](https://docs.rs/simd_aligned/0.6.1/simd_aligned/struct.MatFlat.html "struct simd_aligned::MatFlat")Produced
by [
`MatSimd::flat`](https://docs.rs/simd_aligned/0.6.1/simd_aligned/struct.MatSimd.html#method.flat "method simd_aligned::MatSimd::flat"),
this allow for flat matrix
access.[MatFlatMut](https://docs.rs/simd_aligned/0.6.1/simd_aligned/struct.MatFlatMut.html "struct simd_aligned::MatFlatMut")
Provided by [
`MatSimd::flat_mut`](https://docs.rs/simd_aligned/0.6.1/simd_aligned/struct.MatSimd.html#method.flat_mut "method simd_aligned::MatSimd::flat_mut"),
this allow for flat, mutable matrix
access.[MatSimd](https://docs.rs/simd_aligned/0.6.1/simd_aligned/struct.MatSimd.html "struct simd_aligned::MatSimd")A
dynamic (heap allocated) matrix with one axis aligned for fast and safe SIMD access that
also provides a flat view on its
data.[VecSimd](https://docs.rs/simd_aligned/0.6.1/simd_aligned/struct.VecSimd.html "struct simd_aligned::VecSimd")A
dynamic (heap allocated) vector aligned for fast and safe SIMD access that also provides a
flat view on its data.

## Functions [§](https://docs.rs/simd_aligned/0.6.1/simd_aligned/\#functions)

[packed\_as\_flat](https://docs.rs/simd_aligned/0.6.1/simd_aligned/fn.packed_as_flat.html "fn simd_aligned::packed_as_flat")
Converts an slice of SIMD vectors into a flat slice of
elements.[packed\_as\_flat\_mut](https://docs.rs/simd_aligned/0.6.1/simd_aligned/fn.packed_as_flat_mut.html "fn simd_aligned::packed_as_flat_mut")
Converts a mutable slice of SIMD vectors into a flat slice of elements.