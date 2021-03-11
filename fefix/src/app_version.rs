use std::fmt;

/// Which [`Dictionary`](fefix::Dictionary) version to use.
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
    pub fn all() -> impl Iterator<Item = Self> {
        vec![
            AppVersion::Fix40,
            AppVersion::Fix41,
            AppVersion::Fix42,
            AppVersion::Fix43,
            AppVersion::Fix44,
            AppVersion::Fix50,
            AppVersion::Fix50SP1,
            AppVersion::Fix50SP2,
            AppVersion::Fixt11,
        ]
        .into_iter()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        Some(match s {
            "FIX-4.0" => Self::Fix40,
            "FIX-4.1" => Self::Fix41,
            "FIX-4.2" => Self::Fix42,
            "FIX-4.3" => Self::Fix43,
            "FIX-4.4" => Self::Fix44,
            "FIX-5.0" => Self::Fix50,
            "FIX-5.0-SP1" => Self::Fix50SP1,
            "FIX-5.0-SP2" => Self::Fix50SP2,
            "FIXT-1.1" => Self::Fixt11,
            _ => return None,
        })
    }
}

impl fmt::Display for AppVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str = match self {
            AppVersion::Fix40 => "FIX-4.0",
            AppVersion::Fix41 => "FIX-4.1",
            AppVersion::Fix42 => "FIX-4.2",
            AppVersion::Fix43 => "FIX-4.3",
            AppVersion::Fix44 => "FIX-4.4",
            AppVersion::Fix50 => "FIX-5.0",
            AppVersion::Fix50SP1 => "FIX-5.0-SP1",
            AppVersion::Fix50SP2 => "FIX-5.0-SP2",
            AppVersion::Fixt11 => "FIXT-1.1",
        };
        write!(f, "{}", as_str)
    }
}
