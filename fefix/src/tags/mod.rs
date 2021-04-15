//! Tag mnemonics to avoid using magic numbers in code.
//!
//! This module contains tag mnemonics for "FIX Latest". Tag mnemonics for
//! specific protocol versions are available in sub-module.

use crate::DataType;

pub mod fix40;
pub mod fix41;
pub mod fix42;
pub mod fix43;
pub mod fix44;
pub mod fix50;
pub mod fix50sp1;
pub mod fix50sp2;
pub mod fixt11;

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name: &'static str,
    pub tag: u32,
    pub is_group_leader: bool,
    pub data_type: DataType,
}

pub use fix44::*;
