use std::fmt;

/// Which [`Dictionary`](fefix::Dictionary) version to use.
#[derive(Copy, Debug, Clone)]
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
    #[cfg(test)]
    pub(crate) fn all() -> impl Iterator<Item = Self> {
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