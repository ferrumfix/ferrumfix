//! Field and message definitions for all FIX application versions.
#![cfg_attr(docsrs, feature(doc_cfg))]
//!
//! # What is this and why is this necessary?
//!
//! RustyFix internals rely on [`Dictionary`](crate::Dictionary) for accessing
//! details about fields, messages and other abstract entities defined in the
//! FIX Dictionary specifications. Although this approach works quite well, it
//! can become daunting to query a [`Dictionary`](crate::Dictionary) for even
//! the most basic operation.

use crate::dict::FixDatatype;
use crate::{TagU32, dict};

#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct HardCodedFixFieldDefinition {
    pub name: &'static str,
    pub tag: u32,
    pub data_type: FixDatatype,
    pub location: dict::FieldLocation,
}

impl dict::IsFieldDefinition for HardCodedFixFieldDefinition {
    fn tag(&self) -> TagU32 {
        TagU32::new(self.tag).expect("Invalid tag number 0.")
    }

    fn name(&self) -> &str {
        self.name
    }

    fn location(&self) -> dict::FieldLocation {
        self.location
    }
}

#[cfg(feature = "fix40")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.4.0.
pub mod fix40 {
    include!(concat!(env!("OUT_DIR"), "/fix40.rs"));
}

#[cfg(feature = "fix41")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.4.1.
pub mod fix41 {
    include!(concat!(env!("OUT_DIR"), "/fix41.rs"));
}

#[cfg(feature = "fix42")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.4.2.
pub mod fix42 {
    include!(concat!(env!("OUT_DIR"), "/fix42.rs"));
}

#[cfg(feature = "fix43")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.4.3.
pub mod fix43 {
    include!(concat!(env!("OUT_DIR"), "/fix43.rs"));
}

#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.4.4.
pub mod fix44 {
    include!(concat!(env!("OUT_DIR"), "/fix44.rs"));
}

#[cfg(feature = "fix50")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.5.0.
pub mod fix50 {
    include!(concat!(env!("OUT_DIR"), "/fix50.rs"));
}

#[cfg(feature = "fix50sp1")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.5.0 SP1.
pub mod fix50sp1 {
    include!(concat!(env!("OUT_DIR"), "/fix50sp1.rs"));
}

#[cfg(feature = "fix50sp2")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIX.5.0 SP2.
pub mod fix50sp2 {
    include!(concat!(env!("OUT_DIR"), "/fix50sp2.rs"));
}

#[cfg(feature = "fixt11")]
#[allow(dead_code, unused, warnings)]
#[rustfmt::skip]
/// Field and message definitions for FIXT.1.1.
pub mod fixt11 {
    include!(concat!(env!("OUT_DIR"), "/fixt11.rs"));
}
