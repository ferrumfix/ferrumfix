[Docs.rs](https://docs.rs/)

- [wide-0.7.32](https://docs.rs/wide/0.7.32/wide/# "A crate to help you go wide.")


- wide 0.7.32

- [Docs.rs crate page](https://docs.rs/crate/wide/0.7.32 "See wide in docs.rs")
- [Zlib](https://spdx.org/licenses/Zlib) OR [Apache-2.0](https://spdx.org/licenses/Apache-2.0)
  OR [MIT](https://spdx.org/licenses/MIT)

- Links
- [Repository](https://github.com/Lokathor/wide)
- [crates.io](https://crates.io/crates/wide "See wide in crates.io")
- [Source](https://docs.rs/crate/wide/0.7.32/source/ "Browse source of wide-0.7.32")

- Owners
- [Lokathor](https://crates.io/users/Lokathor)

- Dependencies
-
    - [bytemuck ^1\\
      \\
      _normal_](https://docs.rs/bytemuck/^1)
- [safe\_arch ^0.7\\
  \\
  _normal_](https://docs.rs/safe_arch/^0.7)
- [serde ^1\\
  \\
  _normal_ _optional_](https://docs.rs/serde/^1)
- [bincode ^1.3.3\\
  \\
  _dev_](https://docs.rs/bincode/^1.3.3)

- Versions

- [**1.75%**\\
  of the crate is documented](https://docs.rs/crate/wide/0.7.32)

- [Platform](https://docs.rs/wide/0.7.32/wide/#)  - [i686-pc-windows-msvc](https://docs.rs/crate/wide/0.7.32/target-redirect/i686-pc-windows-msvc/wide/index.html)
    - [i686-unknown-linux-gnu](https://docs.rs/crate/wide/0.7.32/target-redirect/i686-unknown-linux-gnu/wide/index.html)
    - [x86\_64-apple-darwin](https://docs.rs/crate/wide/0.7.32/target-redirect/x86_64-apple-darwin/wide/index.html)
    - [x86\_64-pc-windows-msvc](https://docs.rs/crate/wide/0.7.32/target-redirect/x86_64-pc-windows-msvc/wide/index.html)
    - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/wide/0.7.32/target-redirect/x86_64-unknown-linux-gnu/wide/index.html)
- [Feature flags](https://docs.rs/crate/wide/0.7.32/features "Browse available feature flags of wide-0.7.32")

- [docs.rs](https://docs.rs/wide/0.7.32/wide/#)  - [About docs.rs](https://docs.rs/about)
    - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/wide/0.7.32/wide/#)  - [Rust website](https://www.rust-lang.org/)
    - [The Book](https://doc.rust-lang.org/book/)
    - [Standard Library API Reference](https://doc.rust-lang.org/std/)
    - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
    - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

# Crate wideCopy item path

[Settings](https://docs.rs/wide/0.7.32/settings.html)

[Help](https://docs.rs/wide/0.7.32/help.html)

Summary[Source](https://docs.rs/wide/0.7.32/src/wide/lib.rs.html#1-977)

Expand description

A crate to help you go wide.

This crate provides SIMD-compatible data types.

When possible, explicit SIMD is used with all the math operations here. As a
fallback, the fact that all the lengths of a fixed length array are doing
the same thing will often make LLVM notice that it should use SIMD
instructions to complete the task. In the worst case, the code just becomes
totally scalar (though the math is still correct, at least).

### [§](https://docs.rs/wide/0.7.32/wide/\#crate-features) Crate Features

- `std`: This causes the feature to link to `std`.

    - Currently this just improves the performance of `sqrt` when an explicit
      SIMD `sqrt` isn’t available.

## Structs [§](https://docs.rs/wide/0.7.32/wide/\#structs)

[f32x4](https://docs.rs/wide/0.7.32/wide/struct.f32x4.html "struct wide::f32x4")[f32x8](https://docs.rs/wide/0.7.32/wide/struct.f32x8.html "struct wide::f32x8")[f64x2](https://docs.rs/wide/0.7.32/wide/struct.f64x2.html "struct wide::f64x2")[f64x4](https://docs.rs/wide/0.7.32/wide/struct.f64x4.html "struct wide::f64x4")[i8x16](https://docs.rs/wide/0.7.32/wide/struct.i8x16.html "struct wide::i8x16")[i8x32](https://docs.rs/wide/0.7.32/wide/struct.i8x32.html "struct wide::i8x32")[i16x8](https://docs.rs/wide/0.7.32/wide/struct.i16x8.html "struct wide::i16x8")[i16x16](https://docs.rs/wide/0.7.32/wide/struct.i16x16.html "struct wide::i16x16")[i32x4](https://docs.rs/wide/0.7.32/wide/struct.i32x4.html "struct wide::i32x4")[i32x8](https://docs.rs/wide/0.7.32/wide/struct.i32x8.html "struct wide::i32x8")[i64x2](https://docs.rs/wide/0.7.32/wide/struct.i64x2.html "struct wide::i64x2")[i64x4](https://docs.rs/wide/0.7.32/wide/struct.i64x4.html "struct wide::i64x4")[u8x16](https://docs.rs/wide/0.7.32/wide/struct.u8x16.html "struct wide::u8x16")[u16x8](https://docs.rs/wide/0.7.32/wide/struct.u16x8.html "struct wide::u16x8")[u16x16](https://docs.rs/wide/0.7.32/wide/struct.u16x16.html "struct wide::u16x16")[u32x4](https://docs.rs/wide/0.7.32/wide/struct.u32x4.html "struct wide::u32x4")[u32x8](https://docs.rs/wide/0.7.32/wide/struct.u32x8.html "struct wide::u32x8")[u64x2](https://docs.rs/wide/0.7.32/wide/struct.u64x2.html "struct wide::u64x2")[u64x4](https://docs.rs/wide/0.7.32/wide/struct.u64x4.html "struct wide::u64x4")

## Traits [§](https://docs.rs/wide/0.7.32/wide/\#traits)

[CmpEq](https://docs.rs/wide/0.7.32/wide/trait.CmpEq.html "trait wide::CmpEq")[CmpGe](https://docs.rs/wide/0.7.32/wide/trait.CmpGe.html "trait wide::CmpGe")[CmpGt](https://docs.rs/wide/0.7.32/wide/trait.CmpGt.html "trait wide::CmpGt")[CmpLe](https://docs.rs/wide/0.7.32/wide/trait.CmpLe.html "trait wide::CmpLe")[CmpLt](https://docs.rs/wide/0.7.32/wide/trait.CmpLt.html "trait wide::CmpLt")[CmpNe](https://docs.rs/wide/0.7.32/wide/trait.CmpNe.html "trait wide::CmpNe")