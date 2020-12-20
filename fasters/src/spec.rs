//! FIX dictionary and message specification in a Rust-friendly format.

use roxmltree;
use std::collections::HashMap;
use std::error;

/// The `HasPk` trait provides access to primary keys.
pub trait HasPk {
    /// The type of `Self`'s primary key.
    type Pk;

    /// Get a copy of the primary key.
    fn pk(&self) -> Self::Pk;
}

pub type HashMapPk<K, T> = HashMap<<K as HasPk>::Pk, T>;

/// Allows lookup of FIX definitions based on `RepoV2010`.
#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    pub version: String,
    pub data_types: HashMapPk<Datatype, Datatype>,
    pub fields: HashMapPk<Field, Field>,
    pub components: HashMap<String, Component>,
    pub messages: HashMapPk<Message, Message>,
    pub msg_contents: HashMapPk<Component, Vec<MsgContent>>,
    pub header: Vec<Field>,
}

impl Dictionary {
    pub fn empty() -> Self {
        Dictionary {
            version: "foobar".to_string(),
            data_types: HashMap::new(),
            fields: HashMap::new(),
            components: HashMap::new(),
            messages: HashMap::new(),
            msg_contents: HashMap::new(),
            header: Vec::new(),
        }
    }

    pub fn from_quickfix_spec(input: &str) -> Result<Self, Box<dyn error::Error>> {
        let xml_document = roxmltree::Document::parse(input).unwrap();
        let mut dictionary = Dictionary::empty();
        let root = xml_document.root();
        let node_header = root
            .descendants()
            .find(|n| n.has_tag_name("header"))
            .unwrap();
        let node_trailer = root
            .descendants()
            .find(|n| n.has_tag_name("trailer"))
            .unwrap();
        let node_messages = root
            .descendants()
            .find(|n| n.has_tag_name("messages"))
            .unwrap();
        let node_fields = root
            .descendants()
            .find(|n| n.has_tag_name("fields"))
            .unwrap();
        let mut fields_by_name = HashMap::new();
        for node in node_fields.descendants() {
            let field = quickfix_node_to_field(node);
            fields_by_name.insert(field.name.clone(), field.clone());
            dictionary.fields.insert(field.tag, field);
        }
        for node in node_messages.descendants() {
            let message = quickfix_node_to_message(node);
        }
        Ok(dictionary)
    }
}

fn quickfix_node_to_field<'a, 'b>(node: roxmltree::Node<'a, 'b>) -> Field {
    Field {
        name: node.attribute("name").unwrap().to_string(),
        tag: node.attribute("number").unwrap().parse().unwrap(),
        data_type: node.attribute("type").unwrap().to_string(),
        associated_data_tag: None,
        required: true,
        abbr_name: None,
        base_category_abbr_name: None,
        base_category_id: None,
        description: None,
    }
}

fn quickfix_node_to_message<'a, 'b>(node: roxmltree::Node<'a, 'b>) -> Message {
    Message {
        name: node.attribute("name").unwrap().to_string(),
        msg_type: node.attribute("msg_type").unwrap().parse().unwrap(),
        component_id: 0,
        category_id: String::new(),
        section_id: String::new(),
        abbr_name: None,
        required: true,
        elaboration: None,
        description: String::new(),
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Abbreviation {}

#[derive(Clone, Debug, PartialEq)]
pub struct Abbreviations {
    pub data: Vec<Abbreviation>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    Int,
    Float,
    Char,
    String,
    Data,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    /// **Primary key**. A string uniquely identifying this category.
    pub id: String,
    /// The FIXML file name for a Category.
    pub fixml_filename: String,
}

impl HasPk for Category {
    type Pk = String;

    fn pk(&self) -> Self::Pk {
        self.id.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Categories {
    pub data: Vec<Category>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Component {
    /// **Primary key.** The unique integer identifier of this component
    /// type.
    pub id: usize,
    pub component_type: ComponentType,
    /// **Primary key.** The unique integer identifier of this component
    /// type.
    pub category_id: <Category as HasPk>::Pk,
    /// The human readable name of the component.
    pub name: String,
    /// The name for this component when used in an XML context.
    pub abbr_name: Option<String>,
}

impl HasPk for Component {
    type Pk = usize;

    fn pk(&self) -> Self::Pk {
        self.id
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Components {
    pub data: Vec<Component>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ComponentType {
    BlockRepeating,
    Block,
    ImplicitBlockRepeating,
    ImplicitBlock,
    OptimisedBlockRepeating,
    OptimisedImplicitBlockRepeating,
    XMLDataBlock,
    Message,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Datatype {
    /// **Primary key.** Identifier of the datatype.
    pub name: String,
    /// Base type from which this type is derived.
    pub base_type: Option<String>,
    /// Human readable description of this Datatype.
    pub description: String,
    /// A string that contains examples values for a datatype
    pub examples: Vec<String>,
    // TODO: 'XML'.
}

impl HasPk for Datatype {
    type Pk = String;

    fn pk(&self) -> Self::Pk {
        self.name.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Datatypes {
    pub data: Vec<Datatype>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {}

#[derive(Clone, Debug, PartialEq)]
pub struct Enums {
    pub data: Vec<Enum>,
}

/// A field is identified by a unique tag number and a name. Each field in a
/// message is associated with a value.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Field {
    /// A human readable string representing the name of the field.
    pub name: String,
    /// **Primary key.** A positive integer representing the unique
    /// identifier for this field type.
    pub tag: usize,
    /// The datatype of the field.
    pub data_type: String,
    /// The associated data field. If given, this field represents the length of
    /// the referenced data field
    pub associated_data_tag: Option<usize>,
    /// Abbreviated form of the Name, typically to specify the element name when
    /// the field is used in an XML message. Can be overridden by BaseCategory /
    /// BaseCategoryAbbrName.
    pub abbr_name: Option<String>,
    /// Specifies the base message category when field is used in an XML message.
    pub base_category_id: Option<usize>,
    /// If BaseCategory is specified, this is the XML element identifier to use
    /// for this field, overriding AbbrName.
    pub base_category_abbr_name: Option<String>,
    /// Indicates whether the field is required in an XML message.
    pub required: bool,
    pub description: Option<String>,
}

impl Field {
    pub fn doc_url_onixs(&self, version: &str) -> String {
        let mut url = "https://www.onixs.biz/fix-dictionary/".to_string();
        url.push_str(version);
        url.push_str("/tagNum_");
        url.push_str(self.tag.to_string().as_str());
        url.push_str(".html");
        url
    }
}

impl HasPk for Field {
    type Pk = usize;

    fn pk(&self) -> Self::Pk {
        self.tag
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldRef {
    pub name: String,
    pub required: char,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Fields {
    pub data: Vec<Field>,
}

/// Available versions of the FIX standard.
#[derive(Clone, Debug, PartialEq)]
pub struct FixVersion(String);

#[derive(Clone, Debug, PartialEq)]
pub struct Message {
    /// The unique integer identifier of this message type.
    pub component_id: usize,
    /// **Primary key**. The unique character identifier of this message
    /// type; used literally in FIX messages.
    pub msg_type: String,
    /// The name of this message type.
    pub name: String,
    /// Identifier of the category to which this message belongs.
    pub category_id: <Category as HasPk>::Pk,
    /// Identifier of the section to which this message belongs.
    pub section_id: String,
    /// The abbreviated name of this message, when used in an XML context.
    pub abbr_name: Option<String>,
    /// A boolean used to indicate if the message is to be generated as part
    /// of FIXML.
    pub required: bool,
    pub description: String,
    pub elaboration: Option<String>,
}

impl HasPk for Message {
    type Pk = String;

    fn pk(&self) -> Self::Pk {
        self.msg_type.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Messages {
    pub data: Vec<Message>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgContent {
    pub component_id: usize,
    pub tag_text: String,
    pub reqd: char,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgContents {
    pub data: Vec<MsgContent>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Section {}

#[derive(Clone, Debug, PartialEq)]
pub struct Sections {
    pub data: Vec<Section>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    pub value_enum: String,
    pub description: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
}
