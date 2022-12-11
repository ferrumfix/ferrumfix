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
//! [`FieldType`] implementations for third-party crates and type conversions.
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
//!   [`fefix::tagvalue::DecoderStreaming`](crate::tagvalue::DecoderStreaming).
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
    clippy::missing_panics_doc,
    clippy::mixed_case_hex_literals,
    clippy::needless_bool,
    clippy::needless_lifetimes,
    rustdoc::invalid_rust_codeblocks
)]
// Only enables the `doc_cfg` feature when its feature is defined.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

mod buffer;
mod field_access;
mod utils;

pub mod definitions;
pub mod field_types;
pub mod prelude;
pub mod session;
pub use field_access::{FieldMap, GroupEntries, RepeatingGroup};
pub mod tagvalue;

#[cfg(feature = "json-encoding")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "json-encoding")))]
pub mod json;

pub use buffer::{Buffer, BufferWriter};
#[cfg(feature = "codegen")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "codegen")))]
pub use fefix_codegen as codegen;
// We don't show derive macros to pollute the docs.
#[doc(hidden)]
pub use fefix_derive::FieldType;
pub use fefix_dictionary::{self as dict, Dictionary, TagU32};

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

/// Provides (de)serialization logic for a Rust type as FIX field values.
///
/// See the [`field_types`](crate::field_types) module for more information.
pub trait FieldType<'a>
where
    Self: Sized,
{
    /// The error type that can arise during deserialization.
    type Error;
    /// A type with values that customize the serialization algorithm, e.g.
    /// padding information.
    type SerializeSettings: Default;

    /// Writes `self` to `buffer` using default settings.
    #[inline]
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.serialize_with(buffer, Self::SerializeSettings::default())
    }

    /// Writes `self` to `buffer` using custom serialization `settings`.
    fn serialize_with<B>(&self, buffer: &mut B, settings: Self::SerializeSettings) -> usize
    where
        B: Buffer;

    /// Parses and deserializes from `data`.
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error>;

    /// Like [`FieldType::deserialize`], but it's allowed to skip *some* amount of
    /// input checking. Invalid inputs might not trigger errors and instead be
    /// deserialized as random values.
    ///
    /// # Safety
    ///
    /// This method remains 100% safe even on malformed inputs.
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::deserialize(data)
    }

    /// Serializes `self` to a [`Vec`] of bytes, allocated on the fly.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        buffer
    }

    /// Allocates a [`String`] representation of `self`, using [`FieldType::to_bytes`].
    ///
    /// # Panics
    ///
    /// This function will panic if the underlying byte representation is not
    /// valid UTF-8. As such, you should only *ever* use this function for
    /// [`FieldType`] implementors that are guaranteed to be representable with
    /// valid UTF-8 (like numbers with ASCII digits).
    fn to_string(&self) -> String {
        String::from_utf8(self.to_bytes()).expect("Invalid UTF-8 representation of FIX field.")
    }
}

/// Common logic for interfacing with a streaming parser.
///
/// Streaming parsers store incoming bytes in a [`Buffer`] and try to parse
/// them into messages.
///
/// # Errors
///
/// As soon as a single message fails to parse, the whole decoder should be
/// assumed to be in an invalid state. Discard it and create another.
pub trait StreamingDecoder {
    /// The [`Buffer`] implementation used by this decoder.
    type Buffer: Buffer;
    /// The parsing error type.
    type Error;

    /// Returns a mutable reference to the whole internal [`Buffer`].
    fn buffer(&mut self) -> &mut Self::Buffer;

    /// Empties all contents of the internal buffer of `self`.
    fn clear(&mut self) {
        self.buffer().clear();
    }

    /// Provides a lower bound on the number of bytes that are required to reach the end of the
    /// current message.
    fn num_bytes_required(&self) -> usize;

    /// Provides a buffer that must be filled before re-attempting to deserialize
    /// the next message. The slice is *guaranteed* to be non-empty.
    fn fillable(&mut self) -> &mut [u8] {
        let len = self.buffer().len();
        let num_bytes_required = self.num_bytes_required();
        self.buffer().resize(num_bytes_required, 0);
        &mut self.buffer().as_mut_slice()[len..]
    }

    /// Attempts to parse the contents available in the internal [`Buffer`]. The return value gives
    /// you information about the state of the decoder:
    ///
    /// - [`Ok(None)`]: no errors found, but more bytes are required to finish parsing the message.
    /// - [`Ok(Some(()))`]: no errors found, and the message has been fully parsed.
    /// - [`Err`]: parsing failed.
    /// [`StreamingDecoder::Error`] upon failure.
    fn try_parse(&mut self) -> Result<Option<()>, Self::Error>;
}

/// Allows to write FIX fields.
pub trait SetField<F> {
    /// Writes a field with default [`FieldType::SerializeSettings`].
    fn set<'a, V>(&'a mut self, field: F, value: V)
    where
        V: FieldType<'a>,
    {
        self.set_with(field, value, <V::SerializeSettings as Default>::default())
    }

    /// Writes a field with custom [`FieldType::SerializeSettings`].
    fn set_with<'a, V>(&'a mut self, field: F, value: V, setting: V::SerializeSettings)
    where
        V: FieldType<'a>;
}

/// Either a field that is missing or has an invalid value.
#[derive(Debug, thiserror::Error)]
pub enum FieldValueError<E> {
    /// No such field was found.
    #[error("Missing field tag")]
    Missing,
    /// The field was found, but can't be parsed.
    #[error("Invalid field value: {0}")]
    Invalid(#[from] E),
}

impl<E> PartialEq<FieldValueError<E>> for FieldValueError<E> {
    fn eq(&self, other: &FieldValueError<E>) -> bool {
        match (self, other) {
            (FieldValueError::Missing, FieldValueError::Missing) => true,
            _ => false,
        }
    }
}

impl<E> From<Option<E>> for FieldValueError<E> {
    fn from(e: Option<E>) -> Self {
        match e {
            Some(e) => FieldValueError::Invalid(e),
            None => FieldValueError::Missing,
        }
    }
}
