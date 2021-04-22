//! Field definition ergonomics.

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

use crate::dtf::DataField;
use crate::DataType;
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
