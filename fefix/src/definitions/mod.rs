//! Field and message definitions for all FIX application versions.

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

use crate::{dict, FixFieldValue, FixDataType, TagU16};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GeneratedFieldDef<'a, V>
where
    V: FixFieldValue<'a>,
{
    pub name: &'static str,
    pub tag: u16,
    pub is_group_leader: bool,
    pub data_type: FixDataType,
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

impl<'a, V> dict::IsTypedFieldDefinition<V> for GeneratedFieldDef<'a, V> where V: FixFieldValue<'a> {}
