use std::fmt;

const NAME_FIX40: &str = "FIX-4.0";
const NAME_FIX41: &str = "FIX-4.1";
const NAME_FIX42: &str = "FIX-4.2";
const NAME_FIX43: &str = "FIX-4.3";
const NAME_FIX44: &str = "FIX-4.4";
const NAME_FIX50: &str = "FIX-5.0";
const NAME_FIX50SP1: &str = "FIX-5.0-SP1";
const NAME_FIX50SP2: &str = "FIX-5.0-SP2";
const NAME_FIXT11: &str = "FIXT-1.1";

/// Indicates any of the officially supported FIX Application Layer versions.
///
/// FerrumFIX doesn't restrict support to only these versions; these are simply
/// the ones that are shipped by default with the library.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AppVersion {
    Fix40,
    Fix41,
    Fix42,
    Fix43,
    Fix44,
    Fix50,
    Fix50SP1,
    Fix50SP2,
    Fixt11,
}

impl AppVersion {
    /// An array of all [`AppVersion`] variants.
    pub const ALL: &'static [Self] = &[
        Self::Fix40,
        Self::Fix41,
        Self::Fix42,
        Self::Fix43,
        Self::Fix44,
        Self::Fix50,
        Self::Fix50SP1,
        Self::Fix50SP2,
        Self::Fixt11,
    ];

    /// Returns the signature name of `self`, e.g. "FIX-4.4" or "FIX-5.0-SP1".
    pub const fn name(&self) -> &str {
        match self {
            Self::Fix40 => NAME_FIX40,
            Self::Fix41 => NAME_FIX41,
            Self::Fix42 => NAME_FIX42,
            Self::Fix43 => NAME_FIX43,
            Self::Fix44 => NAME_FIX44,
            Self::Fix50 => NAME_FIX50,
            Self::Fix50SP1 => NAME_FIX50SP1,
            Self::Fix50SP2 => NAME_FIX50SP2,
            Self::Fixt11 => NAME_FIXT11,
        }
    }

    /// Returns the [`AppVersion`] associated with `name`, if it exists. Returns
    /// `None` if no match was found.
    pub fn from_str(name: &str) -> Option<Self> {
        Some(match name {
            NAME_FIX40 => Self::Fix40,
            NAME_FIX41 => Self::Fix41,
            NAME_FIX42 => Self::Fix42,
            NAME_FIX43 => Self::Fix43,
            NAME_FIX44 => Self::Fix44,
            NAME_FIX50 => Self::Fix50,
            NAME_FIX50SP1 => Self::Fix50SP1,
            NAME_FIX50SP2 => Self::Fix50SP2,
            NAME_FIXT11 => Self::Fixt11,
            _ => return None,
        })
    }

    pub fn standard(&self) -> &str {
        if let Self::Fixt11 = self {
            "FIXT"
        } else {
            "FIX"
        }
    }

    pub fn major(&self) -> u32 {
        match self {
            Self::Fix40 => 4,
            Self::Fix41 => 4,
            Self::Fix42 => 4,
            Self::Fix43 => 4,
            Self::Fix44 => 4,
            Self::Fix50 => 5,
            Self::Fix50SP1 => 5,
            Self::Fix50SP2 => 5,
            Self::Fixt11 => 1,
        }
    }

    pub fn minor(&self) -> u32 {
        match self {
            Self::Fix40 => 0,
            Self::Fix41 => 1,
            Self::Fix42 => 2,
            Self::Fix43 => 3,
            Self::Fix44 => 4,
            Self::Fix50 => 0,
            Self::Fix50SP1 => 0,
            Self::Fix50SP2 => 0,
            Self::Fixt11 => 1,
        }
    }
}

impl fmt::Display for AppVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
