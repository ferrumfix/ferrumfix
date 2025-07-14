//! *FIX Adapted for Streaming*
//! ([FAST](https://www.fixtrading.org/standards/fast/)) support.

#![doc(html_root_url = "https://docs.rs/rustyfast/")]
#![deny(
    unused,
    missing_debug_implementations,
    unsafe_op_in_unsafe_fn,
    rustdoc::broken_intra_doc_links,
    //missing_docs,
    unconditional_recursion,
    unstable_name_collisions,
    clippy::useless_conversion,
    clippy::missing_panics_doc,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes
)]
#![allow(unexpected_cfgs)]
// Only enables the `doc_cfg` feature when its feature is defined.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod codec;
mod codegen;
mod data_type_field;
mod decimal;
mod errors;
mod field_operators;
mod template;

pub use self::decimal::Decimal;
pub use codec::{Codec, PresenceMap};
pub use codegen::template_struct as codegen_template_struct;
pub use errors::{DynamicError, Error, ReportableError, StaticError};
pub use field_operators::*;
pub use template::*;
