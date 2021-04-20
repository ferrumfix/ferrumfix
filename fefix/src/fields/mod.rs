//! Field definition ergonomics.

pub mod fix40;
pub mod fix41;
pub mod fix42;
pub mod fix43;
pub mod fix44;
pub mod fix50;
pub mod fix50sp1;
pub mod fix50sp2;
pub mod fixt11;

use std::marker::PhantomData;

use crate::dtf::DataField;
use crate::DataType;

#[derive(Debug, Clone)]
pub struct FieldDef<'a, V>
where
    V: DataField<'a>,
{
    pub(crate) name: &'a str,
    pub(crate) tag: u32,
    pub(crate) is_group_leader: bool,
    pub(crate) data_type: DataType,
    pub(crate) location: FieldLocation,
    pub(crate) phantom: PhantomData<V>,
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
