use crate::repo::types::Message;
use serde::Deserialize;

/// Available versions of the FIX standard.
#[derive(Copy, Clone, Debug, PartialEq, Deserialize, strum_macros::EnumIter)]
pub enum Version {
    /// Fix 2.7.
    #[serde(rename = "FIX.2.7")]
    Fix27,
    /// Fix 3.0.
    #[serde(rename = "FIX.3.0")]
    Fix30,
    /// Fix 4.0.
    #[serde(rename = "FIX.4.0")]
    Fix40,
    /// Fix 4.1.
    #[serde(rename = "FIX.4.1")]
    Fix41,
    /// Fix 4.2.
    #[serde(rename = "FIX.4.2")]
    Fix42,
    /// Fix 4.3.
    #[serde(rename = "FIX.4.3")]
    Fix43,
    /// Fix 4.4.
    #[serde(rename = "FIX.4.4")]
    Fix44,
    /// Fix 5.0.
    #[serde(rename = "FIX.5.0")]
    Fix50,
    /// Fix 5.0 SP1.
    #[serde(rename = "FIX.5.0SP1")]
    Fix50SP1,
    /// Fix 5.0 SP2.
    #[serde(rename = "FIX.5.0SP2")]
    Fix50SP2,
    /// FIXT 1.1.
    #[serde(rename = "FIXT.1.1")]
    Fixt11,
}

impl Version {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as strum::IntoEnumIterator>::iter()
    }

    pub fn iter_supported() -> impl Iterator<Item = Self> {
        Self::iter().filter(|v| v.is_supported())
    }

    pub fn is_supported(self) -> bool {
        // From <https://www.fixtrading.org/standards/unsupported/>
        match self {
            Self::Fix27 => false,
            Self::Fix30 => false,
            Self::Fix40 => false,
            Self::Fix41 => false,
            Self::Fix42 => false,//true
            Self::Fix43 => false,
            Self::Fix44 => false,//true
            Self::Fix50 => false,
            Self::Fix50SP1 => false,
            Self::Fix50SP2 => false,//true
            Self::Fixt11 => false,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fix27 => "FIX.2.7",
            Self::Fix30 => "FIX.3.0",
            Self::Fix40 => "FIX.4.0",
            Self::Fix41 => "FIX.4.1",
            Self::Fix42 => "FIX.4.2",
            Self::Fix43 => "FIX.4.3",
            Self::Fix44 => "FIX.4.4",
            Self::Fix50 => "FIX.5.0",
            Self::Fix50SP1 => "FIX.5.0SP1",
            Self::Fix50SP2 => "FIX.5.0SP2",
            Self::Fixt11 => "FIXT.1.1",
        }
    }

    pub fn module_name(self) -> &'static str {
        match self {
            Self::Fix27 => "fix27",
            Self::Fix30 => "fix30",
            Self::Fix40 => "fix40",
            Self::Fix41 => "fix41",
            Self::Fix42 => "fix42",
            Self::Fix43 => "fix43",
            Self::Fix44 => "fix44",
            Self::Fix50 => "fix50",
            Self::Fix50SP1 => "fix50sp1",
            Self::Fix50SP2 => "fix50sp2",
            Self::Fixt11 => "fixt11",
        }
    }

    pub fn onixs_str(self) -> &'static str {
        match self {
            Self::Fix27 => unimplemented!(),
            Self::Fix30 => "3.0",
            Self::Fix40 => "4.0",
            Self::Fix41 => "4.1",
            Self::Fix42 => "4.2",
            Self::Fix43 => "4.3",
            Self::Fix44 => "4.4",
            Self::Fix50 => "5.0",
            Self::Fix50SP1 => "5.0SP1",
            Self::Fix50SP2 => "5.0SP2",
            Self::Fixt11 => "FIXT1.1",
        }
    }

    pub fn header(self) -> Message {
        unimplemented!()
    }

    pub fn trailer(self) -> Message {
        unimplemented!()
    }
}
