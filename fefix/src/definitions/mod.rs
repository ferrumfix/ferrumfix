//! Field and message definitions for all FIX application versions.

#[cfg(feature = "fix40")]
#[rustfmt::skip]
pub mod fix40;
#[cfg(feature = "fix41")]
#[rustfmt::skip]
pub mod fix41;
#[cfg(feature = "fix42")]
#[rustfmt::skip]
pub mod fix42;
#[cfg(feature = "fix43")]
#[rustfmt::skip]
pub mod fix43;
#[cfg(feature = "fix44")]
#[rustfmt::skip]
pub mod fix44;
#[cfg(feature = "fix50")]
#[rustfmt::skip]
pub mod fix50;
#[cfg(feature = "fix50sp1")]
#[rustfmt::skip]
pub mod fix50sp1;
#[cfg(feature = "fix50sp2")]
#[rustfmt::skip]
pub mod fix50sp2;
#[cfg(feature = "fixt11")]
#[rustfmt::skip]
pub mod fixt11;

use crate::{dict, dict::FixDataType, FixFieldValue, TagU16};
use std::marker::PhantomData;

/// Metadata about a specific FIX tag. Designed for code generation.
///
/// # Type signature
///
/// `V` is the suggested [`FixFieldValue`] for this FIX field.
#[derive(Debug, Clone)]
pub struct GeneratedFieldDef<'a, V>
where
    V: FixFieldValue<'a>,
{
    /// Human-readable ASCII name of this FIX field, e.g. `MsgSeqNum`.
    pub name: &'static str,
    /// Numeric tag associated with this FIX field.
    pub tag: u16,
    pub is_group_leader: bool,
    pub data_type: FixDataType,
    /// Expected location of this FIX field within messages - header, body, or
    /// trailer. Used for JSON-encoded FIX messages.
    pub location: dict::FieldLocation,
    pub phantom: PhantomData<&'a V>,
}

impl<'a, V> dict::IsFieldDefinition for GeneratedFieldDef<'a, V>
where
    V: FixFieldValue<'a>,
{
    #[inline(always)]
    fn tag(&self) -> TagU16 {
        TagU16::new(self.tag).unwrap()
    }

    #[inline(always)]
    fn name(&self) -> &str {
        self.name
    }

    #[inline(always)]
    fn location(&self) -> dict::FieldLocation {
        self.location
    }
}

impl<'a, V> dict::IsTypedFieldDefinition<V> for GeneratedFieldDef<'a, V> where
    V: FixFieldValue<'a>
{
}
