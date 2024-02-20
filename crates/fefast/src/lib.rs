//! *FIX Adapted for Streaming*
//! ([FAST](https://www.fixtrading.org/standards/fast/)) support.

#![doc(html_root_url = "https://docs.rs/fefast/")]
#![warn(rustdoc::missing_doc_code_examples)]
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
#![cfg_attr(doc_cfg, feature(doc_cfg))]

use template::Template;

mod codec;
mod codegen;
mod decimal;
mod dtf;
mod errors;
mod field_operators;
mod template;

pub use self::decimal::Decimal;
pub use codec::{Codec, PresenceMap};
pub use codegen::template_struct as codegen_template_struct;
pub use errors::{DynamicError, Error, ReportableError, StaticError};
pub use field_operators::*;
pub use template::*;
