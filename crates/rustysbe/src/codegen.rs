//! SBE schema representation and code generation.

#[derive(Debug, Clone, Default)]
pub struct MessageSchema {
    pub package: String,
    pub version: u16,
    pub semantic_version: String,
    pub description: String,
    pub messages: Vec<SbeMessage>,
    pub types: Vec<SbeType>,
}

#[derive(Debug, Clone)]
pub struct SbeMessage {
    pub name: String,
    pub id: u16,
    pub description: String,
    pub fields: Vec<SbeField>,
    pub groups: Vec<SbeGroup>,
    pub block_length: u16,
}

#[derive(Debug, Clone)]
pub struct SbeField {
    pub name: String,
    pub id: u16,
    pub field_type: String,
    pub description: String,
    pub presence: String,
    pub offset: u16,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct SbeGroup {
    pub name: String,
    pub id: u16,
    pub dimension_type: String,
    pub fields: Vec<SbeField>,
    pub groups: Vec<SbeGroup>,
}

#[derive(Debug, Clone)]
pub struct SbeEnum {
    pub name: String,
    pub encoding_type: String,
    pub valid_values: Vec<SbeValidValue>,
}

#[derive(Debug, Clone)]
pub struct SbeValidValue {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub enum SbeType {
    Composite(SbeComposite),
    Enum(SbeEnum),
}

#[derive(Debug, Clone)]
pub struct SbeComposite {
    pub name: String,
    pub types: Vec<SbeField>,
}
