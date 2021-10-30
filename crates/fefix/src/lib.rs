//! A Financial Information eXchange
//! ([FIX](https://www.fixtrading.org/standards/)) protocol implementation in Rust.
//!
//! FerrumFIX is a collection of reusable components to produce and consume
//! FIX-compliant data. It is *not* a FIX engine, although you can very easily
//! build one with FerrumFIX. FerrumFIX is intended to be:
//!
//!  - **Comprehensive**. Most standards adopted by the FIX Community are
//!  either available or planned, from session layers to encodings and
//!  dictionary-related logic.
//!  - **Foundational**. FerrumFIX is foundational in the sense that it exposes a
//!  large amount of primitives in its public interface, so that users can
//!  easily build upon them to implement custom solutions tailored for their
//!  needs. You'll often find that there are many ways of doing the same thing.
//!  It's up to you to choose whatever works best.
//!  - **Unopinionated**. We try to provide good defaults, but you can discard
//!  them entirely and provide your own should you choose to do so.
//!  Customization is mostly done via Rust traits, which allows for inlining and
//!  maintains great performance.
//!  - **Fast**. Everything is planned around zero-copy and zero-allocations in
//!  the hot paths.
//!  
//! Please check out the [README](https://github.com/neysofu/fefix/) for more
//! general information regarding FerrumFIX.
//!
//! # Cargo features
//!
//! FerrumFIX puts a lot of functionality behind optional features in order to
//! optimize compilation times. The following features are available:
//!
//! ### `fix40`, `fix41`, `fix42`, `fix43`, `fix44`, `fix50`, `fix50sp1`, `fix50sp2`, `fixt11`
//!
//! Version-specific FIX utilities. See the modules within [`definitions`] and
//! the similarly named [`Dictionary`] methods.
//!
//! ### `utils-chrono`, `utils-decimal`, `utils-rust-decimal`
//!
//! [`FixValue`] implementations for third-party crates and type conversions.
//!
//! ### `utils-slog`
//!
//! Logging of [`tagvalue::Message`]s.
//!
//! ### `utils-bytes`, `utils-tokio`
//!
//! FIX decoders and encoders that integrate nicely with the Tokio ecosystem.
//!
//! ### `json-encoding`
//!
//! Decode and encode FIX messages with JSON.
//!
//! ### `codegen`
//!
//! This feature it intended to be used within Cargo's `[build-dependencies]`, like this:
//!
//! ```toml
//! fefix = { version = "0.1", features = ["codegen"] }
//! ```
//!
//! `codegen` allows you to generate Rust files with lots of FIX version
//! -specific constants, types, and documentation. This is not mandatory by any
//! means and you can decide to simply use [`Dictionary`], which provides
//! dynamic access to mostly the same information, but you'll lose on
//! performance, type safety, and quality of life. There's no reason not to use
//! `codegen` if your use case allows it (it likely does!).
//!
//! # FAQ
//!
//! - **Q.** I simply want to read FIX messages. Where do I start?  
//!   **A.** Use [`fefix::tagvalue::Decoder`](crate::tagvalue::Decoder) and
//!   [`fefix::tagvalue::DecoderBuffered`](crate::tagvalue::DecoderBuffered).
//!   The former is for individual messages, the latter is for streams.
//!
//! - **Q.** What about `serde` integration?  
//!   **A.** FIX semantics don't map well to `serde` and there are subtle
//!   performance implications. It's not a good idea.
//!
//! # External resources
//!
//! - [`https://fixtrading.org/standards`](https://fixtrading.org/standards).
//! - [`https://fiximate.fixtrading.org`](https://fiximate.fixtrading.org).
//! - [`https://github.com/FIXTradingCommunity`](https://github.com/FIXTradingCommunity).
//! - [`https://forum.fixtrading.org`](https://forum.fixtrading.org).

#![doc(html_root_url = "https://docs.rs/fefix/")]
#![warn(missing_docs)]
#![deny(
    //unused FIXME,
    missing_debug_implementations,
    unsafe_op_in_unsafe_fn,
    rustdoc::broken_intra_doc_links,
    //missing_docs FIXME,
    unconditional_recursion,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_docs,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes,
    rustdoc::invalid_rust_codeblocks
)]
// Only enables the `doc_cfg` feature when its feature is defined.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod buffer;
mod fefix_core;
mod random_field_access;
mod set_field;
mod utils;

pub mod definitions;
pub mod fix_value;
pub mod prelude;
pub mod session;
pub use random_field_access::{GroupEntries, RandomFieldAccess, RepeatingGroup};
pub use set_field::SetField;
pub mod tagvalue;

#[cfg(feature = "json-encoding")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "json-encoding")))]
pub mod json;
#[cfg(feature = "codegen")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "codegen")))]
pub use fefix_core::codegen;

pub use buffer::{Buffer, BufferWriter};
pub use dict::Dictionary;
pub use fefix_core::dict;
pub use fefix_core::TagU16;
pub use fix_value::FixValue;

// We don't show derive macros to pollute the docs.
#[doc(hidden)]
pub use fefix_derive::FixValue;

/// Allows to get mutable and immutable references to configuration options.
pub trait GetConfig {
    /// The configuration options type.
    type Config;

    /// Returns an immutable reference to the configuration options used by
    /// `self`.
    fn config(&self) -> &Self::Config;

    /// Returns a mutable reference to the configuration options used by
    /// `self`.
    fn config_mut(&mut self) -> &mut Self::Config;
}
