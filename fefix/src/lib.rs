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
//! # Tongue-in-cheek FAQ
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
//! - **Q.** Is this `async`?  
//!   **A.** No, because it doesn't need to. By design, you'll be the done doing
//!   the plumbing together with `tokio` or whatever library you use. FerrumFIX
//!   itself doesn't care about I/O.
//!
//! - **Q.** Why isn't X supported?  
//!   **A.** Time, mostly. Drop me an email or open an issue and let's see what I
//!   can do.

#![deny(
    unused,
    missing_debug_implementations,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_docs,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]

mod utils;

mod app_version;
mod buffer;
pub mod codegen;
mod data_type;
pub mod dictionary;
pub mod dtf;
pub mod fast;
pub mod fixs;
pub mod json;
pub mod models;
pub mod msgs;
mod msgtypemap;
mod quickfix_specs;
pub mod session;
pub mod sofh;
mod tagmap;
pub mod tagvalue;

#[rustfmt::skip]
pub mod fix40;
#[rustfmt::skip]
pub mod fix41;
#[rustfmt::skip]
pub mod fix42;
#[rustfmt::skip]
pub mod fix43;
#[rustfmt::skip]
pub mod fix44;
#[rustfmt::skip]
pub mod fix50;
#[rustfmt::skip]
pub mod fix50sp1;
#[rustfmt::skip]
pub mod fix50sp2;
#[rustfmt::skip]
pub mod fixt11;

pub use app_version::AppVersion;
pub use buffer::Buffer;
pub use data_type::DataType;
pub use dictionary::Dictionary;
pub use fefix_derive::*;
pub use models::{FieldsIter, FixFieldAccess, FixFieldsIter, FixMessage};
pub use quickfix_specs::quickfix_spec;
pub use tagmap::TagMap;

pub use dtf::DataField;
pub use fefix_derive::DataField;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct FieldDef<'a, V>
where
    V: DataField<'a>,
{
    pub name: &'a str,
    pub tag: u32,
    pub is_group_leader: bool,
    pub data_type: DataType,
    pub location: FieldLocation,
    pub phantom: PhantomData<V>,
}

/// The expected location of a field within a FIX message (i.e. header, body, or
/// trailer).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldLocation {
    StdHeader,
    Body,
    Trailer,
}

impl<'a, V> FieldDef<'a, V>
where
    V: DataField<'a>,
{
    /// Returns the numeric tag associated with `self`.
    pub fn tag(&self) -> u32 {
        self.tag
    }

    /// Returns the human-readable name given to `self`.
    pub fn name(&self) -> &'a str {
        self.name
    }

    /// Returns the [`DataType`] of `self`.
    pub fn data_type(&self) -> DataType {
        self.data_type
    }
}
