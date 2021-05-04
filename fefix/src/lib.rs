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
//!  available, from and session layers to [encodings] and
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

#![deny(
    unused,
    missing_debug_implementations,
    //missing_docs,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_docs,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]

mod buffer;
mod fefix_core;
mod utils;
pub use fefix_core::codegen;
pub use fefix_core::dict;
pub use fefix_core::TagU16;
pub mod definitions;
pub mod fast;
pub mod fixp;
pub mod fixs;
pub mod json;
pub mod session;
pub mod sofh;
pub mod tagvalue;

pub use buffer::Buffer;
pub use dict::Dictionary;
pub use fefix_derive::FixFieldValue;
pub use tagvalue::datatypes::FixFieldValue;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;

/// Wrapper type for dealing with `Result<None, ...>` as errors.
pub type OptResult<T, E> = Result<T, OptError<E>>;

/// The error variant of [`OptResult`].
#[derive(Clone, Debug, PartialEq)]
pub enum OptError<E> {
    None,
    Other(E),
}
