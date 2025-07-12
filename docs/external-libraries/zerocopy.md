[Docs.rs](https://docs.rs/)

- [zerocopy-0.8.25](https://docs.rs/zerocopy/0.8.25/zerocopy/# "Zerocopy makes zero-cost memory manipulation effortless. We write \"unsafe\" so you don't have to.")


- zerocopy 0.8.25

- [Docs.rs crate page](https://docs.rs/crate/zerocopy/0.8.25 "See zerocopy in docs.rs")
- [BSD-2-Clause](https://spdx.org/licenses/BSD-2-Clause) OR [Apache-2.0](https://spdx.org/licenses/Apache-2.0)
  OR [MIT](https://spdx.org/licenses/MIT)

- Links
- [Repository](https://github.com/google/zerocopy)
- [crates.io](https://crates.io/crates/zerocopy "See zerocopy in crates.io")
- [Source](https://docs.rs/crate/zerocopy/0.8.25/source/ "Browse source of zerocopy-0.8.25")

- Owners
- [jswrenn](https://crates.io/users/jswrenn)
- [joshlf](https://crates.io/users/joshlf)

- Dependencies
-
    - [zerocopy-derive =0.8.25\\
      \\
      _normal_ _optional_](https://docs.rs/zerocopy-derive/=0.8.25)
- [either =1.13.0\\
  \\
  _dev_](https://docs.rs/either/=1.13.0)
- [elain ^0.3.0\\
  \\
  _dev_](https://docs.rs/elain/^0.3.0)
- [itertools ^0.11\\
  \\
  _dev_](https://docs.rs/itertools/^0.11)
- [rand ^0.8.5\\
  \\
  _dev_](https://docs.rs/rand/^0.8.5)
- [rustversion ^1.0\\
  \\
  _dev_](https://docs.rs/rustversion/^1.0)
- [static\_assertions ^1.1\\
  \\
  _dev_](https://docs.rs/static_assertions/^1.1)
- [trybuild =1.0.89\\
  \\
  _dev_](https://docs.rs/trybuild/=1.0.89)
- [zerocopy-derive =0.8.25\\
  \\
  _dev_](https://docs.rs/zerocopy-derive/=0.8.25)
- [zerocopy-derive =0.8.25\\
  \\
  _normal_](https://docs.rs/zerocopy-derive/=0.8.25)

- Versions

- [**100%**\\
  of the crate is documented](https://docs.rs/crate/zerocopy/0.8.25)

- [Platform](https://docs.rs/zerocopy/0.8.25/zerocopy/#)  - [i686-pc-windows-msvc](https://docs.rs/crate/zerocopy/0.8.25/target-redirect/i686-pc-windows-msvc/zerocopy/index.html)
    - [i686-unknown-linux-gnu](https://docs.rs/crate/zerocopy/0.8.25/target-redirect/i686-unknown-linux-gnu/zerocopy/index.html)
    - [x86\_64-apple-darwin](https://docs.rs/crate/zerocopy/0.8.25/target-redirect/x86_64-apple-darwin/zerocopy/index.html)
    - [x86\_64-pc-windows-msvc](https://docs.rs/crate/zerocopy/0.8.25/target-redirect/x86_64-pc-windows-msvc/zerocopy/index.html)
    - [x86\_64-unknown-linux-gnu](https://docs.rs/crate/zerocopy/0.8.25/target-redirect/x86_64-unknown-linux-gnu/zerocopy/index.html)
- [Feature flags](https://docs.rs/crate/zerocopy/0.8.25/features "Browse available feature flags of zerocopy-0.8.25")

- [docs.rs](https://docs.rs/zerocopy/0.8.25/zerocopy/#)  - [About docs.rs](https://docs.rs/about)
    - [Privacy policy](https://foundation.rust-lang.org/policies/privacy-policy/#docs.rs)

- [Rust](https://docs.rs/zerocopy/0.8.25/zerocopy/#)  - [Rust website](https://www.rust-lang.org/)
    - [The Book](https://doc.rust-lang.org/book/)
    - [Standard Library API Reference](https://doc.rust-lang.org/std/)
    - [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
    - [The Cargo Guide](https://doc.rust-lang.org/cargo/guide/)
    - [Clippy Documentation](https://doc.rust-lang.org/nightly/clippy)

[iframe](/-/storage-change-detection.html)

# Crate zerocopyCopy item path

[Settings](https://docs.rs/zerocopy/0.8.25/settings.html)

[Help](https://docs.rs/zerocopy/0.8.25/help.html)

Summary[Source](https://docs.rs/zerocopy/0.8.25/src/zerocopy/lib.rs.html#15-6733)

Expand description

_Need more out of zerocopy?_
_Submit a [customer request issue](https://github.com/google/zerocopy/issues/new/choose)!_

_**Fast, safe, compile error. Pick two.**_

Zerocopy makes zero-cost memory manipulation effortless. We write `unsafe`
so you don’t have to.

_Thanks for using zerocopy 0.8! For an overview of what changes from 0.7,_
_check out our [release notes](https://github.com/google/zerocopy/discussions/1680), which include a step-by-step_
_guide for upgrading from 0.7._

_Have questions? Need help? Ask the maintainers
on [GitHub](https://github.com/google/zerocopy/discussions/categories/q-a) or_
_on [Discord](https://discord.gg/MAvWH2R6zk)!_

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#overview) Overview

###### [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#conversion-traits) Conversion Traits

Zerocopy provides four derivable traits for zero-cost conversions:

- [`TryFromBytes`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.TryFromBytes.html "trait zerocopy::TryFromBytes")
  indicates that a type may safely be converted from
  certain byte sequences (conditional on runtime checks)
- [`FromZeros`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.FromZeros.html "trait zerocopy::FromZeros") indicates
  that a sequence of zero bytes represents a valid
  instance of a type
- [`FromBytes`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.FromBytes.html "trait zerocopy::FromBytes") indicates
  that a type may safely be converted from an
  arbitrary byte sequence
- [`IntoBytes`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.IntoBytes.html "trait zerocopy::IntoBytes") indicates
  that a type may safely be converted _to_ a byte
  sequence

These traits support sized types, slices,
and [slice DSTs](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.KnownLayout.html#dynamically-sized-types "trait zerocopy::KnownLayout").

###### [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#marker-traits) Marker Traits

Zerocopy provides three derivable marker traits that do not provide any
functionality themselves, but are required to call certain methods provided
by the conversion traits:

- [`KnownLayout`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.KnownLayout.html "trait zerocopy::KnownLayout")
  indicates that zerocopy can reason about certain layout
  qualities of a type
- [`Immutable`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.Immutable.html "trait zerocopy::Immutable") indicates
  that a type is free from interior mutability,
  except by ownership or an exclusive ( `&mut`) borrow
- [`Unaligned`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.Unaligned.html "trait zerocopy::Unaligned") indicates
  that a type’s alignment requirement is 1

You should generally derive these marker traits whenever possible.

###### [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#conversion-macros) Conversion Macros

Zerocopy provides six macros for safe casting between types:

- ( [`try_`](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.try_transmute.html "macro zerocopy::try_transmute")) [
  `transmute`](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.transmute.html "macro zerocopy::transmute") (
  conditionally) converts a value of
  one type to a value of another type of the same size
- ( [
  `try_`](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.try_transmute_mut.html "macro zerocopy::try_transmute_mut")) [
  `transmute_mut`](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.transmute_mut.html "macro zerocopy::transmute_mut") (
  conditionally) converts a
  mutable reference of one type to a mutable reference of another type of
  the same size
- ( [
  `try_`](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.try_transmute_ref.html "macro zerocopy::try_transmute_ref")) [
  `transmute_ref`](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.transmute_ref.html "macro zerocopy::transmute_ref") (
  conditionally) converts a
  mutable or immutable reference of one type to an immutable reference of
  another type of the same size

These macros perform _compile-time_ size and alignment checks, meaning that
unconditional casts have zero cost at runtime. Conditional casts do not need
to validate size or alignment runtime, but do need to validate contents.

These macros cannot be used in generic contexts. For generic conversions,
use the methods defined by the [conversion traits](https://docs.rs/zerocopy/0.8.25/zerocopy/#conversion-traits).

###### [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#byteorder-aware-numerics) Byteorder-Aware Numerics

Zerocopy provides byte-order aware integer types that support these
conversions; see the [
`byteorder`](https://docs.rs/zerocopy/0.8.25/zerocopy/byteorder/index.html "mod zerocopy::byteorder") module. These
types are especially useful
for network parsing.

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#cargo-features) Cargo Features

- **`alloc`**
  By default, `zerocopy` is `no_std`. When the `alloc` feature is enabled,
  the `alloc` crate is added as a dependency, and some allocation-related
  functionality is added.

- **`std`**
  By default, `zerocopy` is `no_std`. When the `std` feature is enabled, the
  `std` crate is added as a dependency (ie, `no_std` is disabled), and
  support for some `std` types is added. `std` implies `alloc`.

- **`derive`**
  Provides derives for the core marker traits via the `zerocopy-derive`
  crate. These derives are re-exported from `zerocopy`, so it is not
  necessary to depend on `zerocopy-derive` directly.

However, you may experience better compile times if you instead directly
depend on both `zerocopy` and `zerocopy-derive` in your `Cargo.toml`,
since doing so will allow Rust to compile these crates in parallel. To do
so, do _not_ enable the `derive` feature, and list both dependencies in
your `Cargo.toml` with the same leading non-zero version number; e.g:

```
[dependencies]
zerocopy = "0.X"
zerocopy-derive = "0.X"
```

To avoid the risk of [duplicate import errors](https://github.com/google/zerocopy/issues/1587) if
one of your dependencies enables zerocopy’s `derive` feature, import
derives as `use zerocopy_derive::*` rather than by name (e.g., `use zerocopy_derive::FromBytes`).

- **`simd`**
  When the `simd` feature is enabled, `FromZeros`, `FromBytes`, and
  `IntoBytes` impls are emitted for all stable SIMD types which exist on the
  target platform. Note that the layout of SIMD types is not yet stabilized,
  so these impls may be removed in the future if layout changes make them
  invalid. For more information, see the Unsafe Code Guidelines Reference
  page on
  the [layout of packed SIMD vectors](https://rust-lang.github.io/unsafe-code-guidelines/layout/packed-simd-vectors.html).

- **`simd-nightly`**
  Enables the `simd` feature and adds support for SIMD types which are only
  available on nightly. Since these types are unstable, support for any type
  may be removed at any point in the future.

- **`float-nightly`**
  Adds support for the unstable `f16` and `f128` types. These types are
  not yet fully implemented and may not be supported on all platforms.

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#security-ethos) Security Ethos

Zerocopy is expressly designed for use in security-critical contexts. We
strive to ensure that that zerocopy code is sound under Rust’s current
memory model, and _any future memory model_. We ensure this by:

- **…not ‘guessing’ about Rust’s semantics.**
  We annotate `unsafe` code with a precise rationale for its soundness that
  cites a relevant section of Rust’s official documentation. When Rust’s
  documented semantics are unclear, we work with the Rust Operational
  Semantics Team to clarify Rust’s documentation.
- **…rigorously testing our implementation.**
  We run tests using [Miri](https://github.com/rust-lang/miri), ensuring that zerocopy is sound across a wide
  array of supported target platforms of varying endianness and pointer
  width, and across both current and experimental memory models of Rust.
- **…formally proving the correctness of our implementation.**
  We apply formal verification tools like [Kani](https://github.com/model-checking/kani) to prove zerocopy’s
  correctness.

For more information, see our
full [soundness policy](https://github.com/google/zerocopy/blob/main/POLICIES.md#soundness).

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#relationship-to-project-safe-transmute) Relationship to Project Safe Transmute

[Project Safe Transmute](https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html) is an official initiative of
the Rust Project to
develop language-level support for safer transmutation. The Project consults
with crates like zerocopy to identify aspects of safer transmutation that
would benefit from compiler support, and has developed an [experimental,\\
compiler-supported analysis](https://github.com/rust-lang/compiler-team/issues/411) which determines whether,
for a given type, any value of that type may be soundly transmuted into
another type. Once this functionality is sufficiently mature, zerocopy
intends to replace its internal transmutability analysis (implemented by our
custom derives) with the compiler-supported one. This change will likely be
an implementation detail that is invisible to zerocopy’s users.

Project Safe Transmute will not replace the need for most of zerocopy’s
higher-level abstractions. The experimental compiler analysis is a tool for
checking the soundness of `unsafe` code, not a tool to avoid writing
`unsafe` code altogether. For the foreseeable future, crates like zerocopy
will still be required in order to provide higher-level abstractions on top
of the building block provided by Project Safe Transmute.

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#msrv) MSRV

See our [MSRV policy](https://github.com/google/zerocopy/blob/main/POLICIES.md#msrv).

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#changelog) Changelog

Zerocopy uses [GitHub Releases](https://github.com/google/zerocopy/releases).

## [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#thanks) Thanks

Zerocopy is maintained by engineers at Google and Amazon with help from
[many wonderful contributors](https://github.com/google/zerocopy/graphs/contributors). Thank you to everyone who has
lent a hand in making Rust a little more secure!

## Re-exports [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#reexports)

`pub use crate::byte_slice::*;``pub use crate::byteorder::*;``pub use crate::error::*;`

## Modules [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#modules)

[byte\_slice](https://docs.rs/zerocopy/0.8.25/zerocopy/byte_slice/index.html "mod zerocopy::byte_slice")Traits for types
that encapsulate a
`[u8]`.[byteorder](https://docs.rs/zerocopy/0.8.25/zerocopy/byteorder/index.html "mod zerocopy::byteorder")Byte
order-aware numeric primitives.[error](https://docs.rs/zerocopy/0.8.25/zerocopy/error/index.html "mod zerocopy::error")
Types related to error reporting.

## Macros [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#macros)

[include\_value](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.include_value.html "macro zerocopy::include_value")
Includes a file and safely transmutes it to a value of an arbitrary
type.[transmute](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.transmute.html "macro zerocopy::transmute")Safely
transmutes a value of one type to a value of another type of the same
size.[transmute\_mut](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.transmute_mut.html "macro zerocopy::transmute_mut")
Safely transmutes a mutable reference of one type to a mutable reference of
another type of the same size and compatible
alignment.[transmute\_ref](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.transmute_ref.html "macro zerocopy::transmute_ref")
Safely transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.[try\_transmute](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.try_transmute.html "macro zerocopy::try_transmute")
Conditionally transmutes a value of one type to a value of another type of
the same
size.[try\_transmute\_mut](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.try_transmute_mut.html "macro zerocopy::try_transmute_mut")
Conditionally transmutes a mutable reference of one type to a mutable
reference of another type of the same size and compatible
alignment.[try\_transmute\_ref](https://docs.rs/zerocopy/0.8.25/zerocopy/macro.try_transmute_ref.html "macro zerocopy::try_transmute_ref")
Conditionally transmutes a mutable or immutable reference of one type to an
immutable reference of another type of the same size and compatible
alignment.

## Structs [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#structs)

[Ref](https://docs.rs/zerocopy/0.8.25/zerocopy/struct.Ref.html "struct zerocopy::Ref")A typed reference derived from a
byte slice.[Split](https://docs.rs/zerocopy/0.8.25/zerocopy/struct.Split.html "struct zerocopy::Split")A `T` that has
been split into two possibly-overlapping
parts.[Unalign](https://docs.rs/zerocopy/0.8.25/zerocopy/struct.Unalign.html "struct zerocopy::Unalign")A type with no
alignment requirement.

## Traits [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#traits)

[FromBytes](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.FromBytes.html "trait zerocopy::FromBytes")Types for which
any bit pattern is
valid.[FromZeros](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.FromZeros.html "trait zerocopy::FromZeros")Types for
which a sequence of `0` bytes is a valid
instance.[Immutable](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.Immutable.html "trait zerocopy::Immutable")Types
which are free from interior
mutability.[IntoBytes](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.IntoBytes.html "trait zerocopy::IntoBytes")Types
that can be converted to an immutable slice of initialized
bytes.[KnownLayout](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.KnownLayout.html "trait zerocopy::KnownLayout")
Indicates that zerocopy can reason about certain aspects of a type’s
layout.[SplitAt](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.SplitAt.html "trait zerocopy::SplitAt")Types that can be
split in
two.[TryFromBytes](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.TryFromBytes.html "trait zerocopy::TryFromBytes")Types
for which some bit patterns are
valid.[Unaligned](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.Unaligned.html "trait zerocopy::Unaligned")Types with
no alignment requirement.

## Derive Macros [§](https://docs.rs/zerocopy/0.8.25/zerocopy/\#derives)

[ByteEq](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.ByteEq.html "derive zerocopy::ByteEq") `derive`Derives
optimized [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")
and [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html "trait core::cmp::Eq")
implementations.[ByteHash](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.ByteHash.html "derive zerocopy::ByteHash")
`derive`Derives an optimized [
`Hash`](https://doc.rust-lang.org/nightly/core/hash/trait.Hash.html "trait core::hash::Hash")
implementation.[FromBytes](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.FromBytes.html "derive zerocopy::FromBytes")
`derive`Analyzes whether a type is [
`FromBytes`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.FromBytes.html "trait zerocopy::FromBytes").[FromZeros](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.FromZeros.html "derive zerocopy::FromZeros")
`derive`Analyzes whether a type is [
`FromZeros`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.FromZeros.html "trait zerocopy::FromZeros").[Immutable](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.Immutable.html "derive zerocopy::Immutable")
`derive`Analyzes whether a type is [
`Immutable`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.Immutable.html "trait zerocopy::Immutable").[IntoBytes](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.IntoBytes.html "derive zerocopy::IntoBytes")
`derive`Analyzes whether a type is [
`IntoBytes`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.IntoBytes.html "trait zerocopy::IntoBytes").[KnownLayout](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.KnownLayout.html "derive zerocopy::KnownLayout")
`derive`Implements [
`KnownLayout`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.KnownLayout.html "trait zerocopy::KnownLayout").[SplitAt](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.SplitAt.html "derive zerocopy::SplitAt")
`derive`Implements [
`SplitAt`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.SplitAt.html "trait zerocopy::SplitAt").[TryFromBytes](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.TryFromBytes.html "derive zerocopy::TryFromBytes")
`derive`Implements [
`TryFromBytes`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.TryFromBytes.html "trait zerocopy::TryFromBytes").[Unaligned](https://docs.rs/zerocopy/0.8.25/zerocopy/derive.Unaligned.html "derive zerocopy::Unaligned")
`derive`Analyzes whether a type is [
`Unaligned`](https://docs.rs/zerocopy/0.8.25/zerocopy/trait.Unaligned.html "trait zerocopy::Unaligned").