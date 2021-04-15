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
mod buffer;
pub mod dictionary;
mod dt;
mod dtf_date;
mod dtf_monthyear;
mod dtf_mulchar;
mod dtf_mulstr;
mod dtf_time;
mod dtf_timestamp;
pub mod fast;
mod fix_codegen;
pub mod fixs;
pub mod generated;
pub mod json;
pub mod models;
mod msgtypemap;
mod quickfix_specs;
pub mod session;
pub mod sofh;
mod tagmap;
pub mod tags;
pub mod tagvalue;

pub use app_version::AppVersion;
pub use buffer::Buffer;
pub use dictionary::Dictionary;
pub use dt::DataType;
pub use dtf_date::DtfDate;
pub use dtf_monthyear::DtfMonthYear;
pub use dtf_mulchar::DtfMulCharIter;
pub use dtf_mulstr::DtfMulStrIter;
pub use dtf_time::DtfTime;
pub use dtf_timestamp::DtfTimestamp;
pub use fefix_derive::*;
pub use fix_codegen::{codegen, codegen_tag_mnemonics, codegen_tag_numbers};
pub use models::{FieldsIter, FixFieldAccess, FixFieldsIter, FixMessage};
pub use quickfix_specs::quickfix_spec;
pub use tagmap::TagMap;

#[cfg(expose_openssl)]
pub extern crate openssl;

#[cfg(not(expose_openssl))]
pub(crate) extern crate openssl;
