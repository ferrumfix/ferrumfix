use crate::TagU32;

/// Univocally locates a tag within a FIX message, even with nested groups.
///
/// Typically, every FIX tag is guaranteed to be unique within a single FIX
/// message. Repeating groups, however, break this promise and allow *multiple*
/// values with the same tag, each in a different *group entry*. This means that
/// a FIX message is a tree rather than an associative array. [`FieldLocator`]
/// generates unique identifiers for tags both outside and within groups, which
/// allows for random (i.e. non-sequential) reads on a FIX message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct FieldLocator {
    pub tag: TagU32,
    pub context: FieldLocatorContext,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum FieldLocatorContext {
    TopLevel,
    WithinGroup {
        index_of_group_tag: u32,
        entry_index: u32,
    },
}
