//! A Financial Information eXchange
//! ([FIX](https://www.fixtrading.org/standards/)) protocol implementation in Rust.
//!
//! FerrumFIX is a collection of reusable components to produce and consume
//! FIX-compliant data. It is *not* a FIX engine, although you can very easily
//! build one with FerrumFIX. FerrumFIX is:
//!
//!  - **Unopinionated**. No configurations or divisive decisions are mandated to
//!  the user
//!  (as much as practically feasible). Good defaults are provided but they are
//!  zero-cost and you can swap them out if necessary.
//!  - **Comprehensive**. Most standards adopted by the FIX Community are
//!  available, from and session layers to encodings and
//!  dictionary-related logic.
//!  - **Foundational**. FerrumFIX is foundational in the sense that it exposes a
//!  large amount of primitives in its public interface, so that users can
//!  easily build upon them to implement custom solutions tailored for their
//!  needs. You'll often find that there are many ways of doing the same thing.
//!  It's up to you to choose whatever works best.
//!  - **Fast**. Everything is planned around zero-copy and zero-allocations in
//!  the hot paths.
//!  
//! Please check out the [README](https://github.com/neysofu/fefix/) for more
//! general information regarding FerrumFIX.
//!
//! # Optional features
//!
//! FerrumFIX puts a lot of functionality behind optional features in order to
//! optimize compilation times for the most common use cases. The following
//! features are available:
//!
//! - `fix40`, `fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1`,
//! `fix50sp2`, `fixt11` – Ergonomic utilities for the respective FIX versions.
//! - `fixs` – FIX-over-TLS support.
//!
//! # FAQ
//!
//! - **Q.** I simply want to read FIX messages. Where do I start?  
//!   **A.** Use [`fefix::tagvalue::Decoder`](crate::tagvalue::Decoder) and
//!   [`fefix::tagvalue::DecoderBuffered`](crate::tagvalue::DecoderBuffered).
//!   The former is for individual messages, the latter is for streams of messages.
//!
//! - **Q.** What FIX message representation does FerrumFIX use?  
//!   **A.** None, at least in the classical sense. Encoding operations work
//!   directly on buffers, which is simple and fast. Decoding operations do use
//!   some internal representation but it's invisible to the user.
//!
//! - **Q.** What about `serde` integration?  
//!   **A.** FIX semantics don't map well to `serde` and there are subtle
//!   performance implications. It's not a good idea.
//!
//! - **Q.** Is this production-ready?  
//!   **A.** Not at the moment, but Bitwyre and other companies are looking to
//!   adopt FerrumFIX within their tech stacks in production, i.e. soon it will be.
//!
//! - **Q.** Why isn't X supported?  
//!   **A.** Time, mostly. Drop me an email or open an issue and let's see what I
//!   can do.
//!
//! # About code generation
//!
//! FerrumFIX internals rely on [`Dictionary`] for accessing details about
//! fields, messages, and other abstract entities as defined in the FIX
//! specifications. Examples of such information include:
//!
//! - The mapping of FIX field names to numeric tags (e.g. `BeginString` is 8).
//! - Which FIX fields are mandatory and which are optional.
//! - The data type of each and every FIX field.
//! - What fields to expect in FIX headers.
//!
//! Sometimes, things become easier by querying [`Dictionary`] at compile time
//! and generating Rust code with optimal performance and better ergonomics and
//! type safety. This is where the [`codegen`] module comes in: Rust code
//! generation utilities that you call in your `build.rs`. The [`definitions`]
//! module includes the code generation artifacts for common FIX versions.
//!
//! # External resources
//!
//! - [`https://fixtrading.org/standards`](https://fixtrading.org/standards).
//! - [`https://fiximate.fixtrading.org`](https://fiximate.fixtrading.org).
//! - [`https://github.com/FIXTradingCommunity`](https://github.com/FIXTradingCommunity).
//! - [`https://forum.fixtrading.org`](https://forum.fixtrading.org).

#![doc(html_root_url = "https://docs.rs/fefix/")]
#![warn(missing_docs, missing_doc_code_examples)]
#![deny(
    unused,
    missing_debug_implementations,
    unsafe_op_in_unsafe_fn,
    rustdoc::broken_intra_doc_links,
    //missing_docs,
    unconditional_recursion,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_docs,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]
// Only enables the `doc_cfg` feature when its feature is defined.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod buffer;
mod fefix_core;
mod fix_value;
pub mod fix_values;
mod utils;
#[cfg(feature = "codegen")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "codegen")))]
pub use fefix_core::codegen;
pub use fefix_core::dict;
pub use fefix_core::TagU16;
pub mod definitions;
#[cfg(feature = "fast-encoding")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fast-encoding")))]
pub mod fast;
#[cfg(feature = "fixp")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fixp")))]
pub mod fixp;
#[cfg(feature = "fixs")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fixs")))]
pub mod fixs;
#[cfg(feature = "json-encoding")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "json-encoding")))]
pub mod json;
pub mod prelude;
pub mod session;
pub mod tagvalue;

pub use buffer::Buffer;
pub use dict::Dictionary;
// We don't derive macros to pollute the docs.
#[doc(hidden)]
pub use fefix_derive::FixValue;
pub use fix_value::FixValue;

/// Wrapper type for dealing with `Ok<None>` as errors.
///
/// Intuitively, this type only makes sense when the *absence* of data is *in
/// itself* an error, e.g. undefined FIX fields which are mandatory.
pub type OptResult<T, E> = Result<T, OptError<E>>;

/// The error variant of [`OptResult`].
#[derive(Clone, Debug, PartialEq)]
pub enum OptError<E> {
    /// Missing data.
    None,
    /// Data is present but there's some other error.
    Other(E),
}
