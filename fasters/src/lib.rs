//! A Financial Information eXchange
//! ([FIX](https://www.fixtrading.org/standards/)) protocol implementation in Rust.
//!
//! Fasters is a collection of reusable components to produce and consume
//! FIX-compliant data. It is *not* a FIX engine, although you can very easily
//! build one with Fasters. Fasters is:
//!
//!  - **Unopinionated**. Fasters takes care of every little detail of the FIX
//!  specification, but no configurations or decisions are mandated to the user
//!  (as much as practically feasible).
//!  - **Comprehensive**. Most standards adopted by the FIX Community are
//!  available, from [transport] and [session] layers to [encodings](encoders) and
//!  dictionary-related [application](app) logic.
//!  - **Foundational**. Fasters is foundational in the sense that it exposes a
//!  large amount of primitives in its public interface, so that users can
//!  easily build upon them to implement custom solutions tailored for their
//!  needs. Multiple FIX message data structures are available.
//!  - **Fast**. We favor configuration via trait specialization in code rather
//!  than files. This results in much faster code at the cost of compilation speed
//!  and code size.
//!  
//! Please check out the [README](https://github.com/neysofu/fasters/) for more
//! general information regarding Fasters.

pub mod app;
mod dictionary;
pub mod encoders;
pub mod engines;
pub mod fix42;
mod fix_codegen;
pub mod prelude;
pub mod session;
mod stream_iterator;
pub mod transport;
pub(crate) mod utils;

pub use dictionary::Dictionary;
pub use fasters_derive::*;
pub use fix_codegen::codegen;
pub use stream_iterator::StreamIterator;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;
