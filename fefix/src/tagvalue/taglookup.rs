use crate::{DataType, Dictionary};
use std::fmt::Debug;

/// This trait describes dynamic tag lookup logic.
///
/// In this context, "tag lookup"
/// means to search in the dictionary the data type associated with a specific
/// tag number. This may seem trivial at best, but it can actually be quite
/// convoluted and require internal state (thus it is "dynamic" tag lookup). In
/// particular, several fields affect the internal state of a
/// [`TagLookup`]:
///
///  - `ApplVerID (1128)`
///  - `ApplExtID (1156)`
///  - `CstmApplVerID (1129)`
///  - `DefaultApplVerID (1137)`
///  - `DefaultApplExtID (1407)`
///  - `DefaultCstmApplVerID (1408)`
///
/// Each of these fields affects the internal state and thus changes how
/// subsequent fields (and messages) are interpreted.
///
/// Most applications don't use such advanced features and are fine with adopting
/// a single standard without any extensions, e.g. "vanilla" FIX 4.4. You should
/// only implement this trait if your needs go way beyond that and you want to
/// employ more exotic rules. e.g. multi-version FIX messages, custom FIX
/// extensions on a per-message basis, etc..
///
/// # Naming conventions
///
/// Implementors of this trait should start with `TagLookup`.
pub trait TagLookup {
    /// The type returned when decoding an invalid or misplaced tag.
    type Error: Debug;

    /// Creates a new instance of the implementor `Self` from `dict`.
    fn from_dict(dict: &Dictionary) -> Self;

    /// Returns the [`DataType`] of the tag number `tag`.
    fn lookup(&mut self, tag: u32) -> Result<DataType, Self::Error>;
}

/// A [`TagLookup`] that only allows a specific version of the FIX protocol, as
/// most FIX applications do.
#[derive(Debug)]
pub struct TagLookupSingleAppVersion {
    current_dict: Dictionary,
    data_types_by_tag: Vec<DataType>,
}

impl TagLookup for TagLookupSingleAppVersion {
    type Error = TagLookupError;

    fn from_dict(dict: &Dictionary) -> Self {
        let mut max_tag = 0;
        let mut datatypes = Vec::new();
        for field in dict.iter_fields() {
            max_tag = field.tag().max(max_tag);
            datatypes.resize(max_tag as usize + 1, DataType::TagNum);
            datatypes[field.tag() as usize] = field.data_type().basetype();
        }
        datatypes.shrink_to_fit();
        Self {
            current_dict: dict.clone(),
            data_types_by_tag: datatypes,
        }
    }

    fn lookup(&mut self, tag: u32) -> Result<DataType, Self::Error> {
        // TODO
        match tag {
            // `ApplVerID <1128>`
            1128 => {}
            // `ApplExtID <1156>`
            1156 => {
                return Err(Self::Error::InvalidApplExtID);
            }
            // `CstmApplVerID <1129>`
            1129 => {
                return Err(Self::Error::InvalidCstmApplVerID);
            }
            // `DefaultApplVerID <1137>`
            1137 => {
                return Err(Self::Error::InvalidApplExtID);
            }
            // `DefaultApplExtID <1407>`
            1407 => {
                return Err(Self::Error::InvalidApplExtID);
            }
            // `DefaultCstmApplVerID <1408>`
            1408 => {
                return Err(Self::Error::InvalidCstmApplVerID);
            }
            _ => (),
        };
        Ok(*self
            .data_types_by_tag
            .get(tag as usize)
            .unwrap_or(&DataType::String))
    }
}

/// Error type for the most common errors found by [`TagLookup`] implementors.
#[derive(Debug)]
pub enum TagLookupError {
    InvalidApplVerID,
    InvalidApplExtID,
    InvalidCstmApplVerID,
}
