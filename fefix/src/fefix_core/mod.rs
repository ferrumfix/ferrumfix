//! FerrumFIX code generation utilities.

use std::collections::HashMap;
use std::hash::Hasher;

#[cfg(feature = "codegen")]
pub mod codegen;
pub mod dict;

/// Type alias for FIX tags: 16-bit unsigned integers, strictly positive.
pub type TagU16 = std::num::NonZeroU16;

/// A "no-hasher" designed to be perform well with [`TagU16`] and [`TagU16`]
/// alone. Vulnerable to
/// [HashDOS](https://en.wikipedia.org/wiki/Collision_attack#Usage_in_DoS_attacks)
/// when used on untrusted inputs.
#[derive(Debug, Default)]
pub struct TagU16NoHasher {
    hash: u64,
}

impl Hasher for TagU16NoHasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash << 8) + *byte as u64;
        }
    }
}

/// A [`HashMap`] with [`TagU16`] keys and [`TagU16NoHasher`].
pub type TagU16Map<T> = HashMap<TagU16, T, TagU16NoHasher>;
