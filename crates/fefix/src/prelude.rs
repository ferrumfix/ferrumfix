//! Convenience re-exports of common traits and various items within `fefix`.

pub use crate::dict::Dictionary;
pub use crate::json::Configure as ConfigureJson;
pub use crate::tagvalue::Configure as ConfigureTagValue;
pub use crate::{
    Buffer, FieldType, GetConfig, RandomFieldAccess, RepeatingGroup, SetField, StreamingDecoder,
    TagU32,
};

#[cfg(feature = "fix40")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix40")))]
pub use crate::definitions::fix40;
#[cfg(feature = "fix41")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix41")))]
pub use crate::definitions::fix41;
#[cfg(feature = "fix42")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix42")))]
pub use crate::definitions::fix42;
#[cfg(feature = "fix43")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix43")))]
pub use crate::definitions::fix43;
#[cfg(feature = "fix44")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix44")))]
pub use crate::definitions::fix44;
#[cfg(feature = "fix50")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix50")))]
pub use crate::definitions::fix50;
#[cfg(feature = "fix50sp1")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix50sp1")))]
pub use crate::definitions::fix50sp1;
#[cfg(feature = "fix50sp2")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fix50sp2")))]
pub use crate::definitions::fix50sp2;
#[cfg(feature = "fixt11")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "fixt11")))]
pub use crate::definitions::fixt11;
