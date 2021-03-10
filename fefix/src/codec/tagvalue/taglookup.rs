use crate::{DataType, Dictionary};
use std::fmt::Debug;
use std::rc::Rc;

/// This trait describes dynamic tag lookup logic.
///
/// In this context, "tag lookup"
/// means to search in the dictionary the data type associated with a specific
/// tag number. This may seem trivial at best, but it can actually be quite
/// convoluted and require internal state (thus it is "dynamic" tag lookup). In
/// particular, several fields affect the internal state of a
/// [`TagLookup`](TagLookup):
///
///  - `ApplVerID <1128>`
///  - `ApplExtID <1156>`
///  - `CstmApplVerID <1129>`
///  - `DefaultApplVerID <1137>`
///  - `DefaultApplExtID <1407>`
///  - `DefaultCstmApplVerID <1408>`
///
/// Each of these fields affects the internal state and thus changes how
/// subsequent fields (and messages) are interpreted.
///
/// # Naming conventions
/// Implementors of this trait should start with `TagLookup`.
pub trait TagLookup {
    type Error: Debug;

    fn from_dict(dict: &Dictionary) -> Self;

    /// Returns the [`BaseType`] of the tag number `tag`.
    fn lookup(&mut self, tag: u32) -> Result<DataType, Self::Error>;
}

/// A [`TagLookup`](TagLookup) that only allows a specific revision of the
/// standard, like most venues do.
#[derive(Debug)]
pub struct TagLookupPredetermined {
    current_dict: Rc<Dictionary>,
}

impl TagLookup for TagLookupPredetermined {
    type Error = TagLookupPredeterminedError;

    fn from_dict(dict: &Dictionary) -> Self {
        Self {
            current_dict: Rc::new(dict.clone()),
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
        Ok(self
            .current_dict
            .field_by_tag(tag)
            .map(|f| f.basetype())
            .unwrap_or(DataType::String))
    }
}

#[derive(Debug)]
pub enum TagLookupPredeterminedError {
    InvalidApplVerID,
    InvalidApplExtID,
    InvalidCstmApplVerID,
}
