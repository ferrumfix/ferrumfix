//! A Financial Information eXchange
//! ([FIX](https://www.fixtrading.org/standards/)) protocol implementation in Rust.
//!
//! FerrumFIX is a collection of reusable components to produce and consume
//! FIX-compliant data. It is *not* a FIX engine, although you can very easily
//! build one with FerrumFIX. FerrumFIX is:
//!
//!  - **Unopinionated**. FerrumFIX takes care of every little detail of the FIX
//!  specification, but no configurations or decisions are mandated to the user
//!  (as much as practically feasible).
//!  - **Comprehensive**. Most standards adopted by the FIX Community are
//!  available, from [transport] and [session] layers to [encodings](encoders) and
//!  dictionary-related [application](backend) logic.
//!  - **Foundational**. FerrumFIX is foundational in the sense that it exposes a
//!  large amount of primitives in its public interface, so that users can
//!  easily build upon them to implement custom solutions tailored for their
//!  needs. Multiple FIX message data structures are available.
//!  - **Fast**. We favor configuration via trait interfaces directly in code rather
//!  than files. This results in much faster code at the cost of compilation speed
//!  and code size.
//!  
//! Please check out the [README](https://github.com/neysofu/fefix/) for more
//! general information regarding FerrumFIX.

#![deny(
    unused,
    missing_debug_implementations,
    clippy::useless_conversion,
    clippy::missing_panics_docs,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]

mod utils;

mod app_version;
pub mod backend;
pub mod buffering;
pub mod codec;
mod dictionary;
mod dt;
pub mod engines;
mod fix_codegen;
mod quickfix_specs;
pub mod session;
mod stream_iterator;
pub mod transport;

pub use crate::codec::{fast, json, sofh, tagvalue, Encoding, StreamingDecoder};
pub use app_version::AppVersion;
pub use dictionary::{Dictionary, MsgType};
pub use dt::DataType;
pub use fefix_derive::*;
pub use fix_codegen::codegen;
pub use quickfix_specs::quickfix_spec;
pub use stream_iterator::StreamIterator;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;
