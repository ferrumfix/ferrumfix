use super::errors::StaticError;
use super::field_operators::FieldOperatorInstruction;
use crate::spec::Dictionary;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Element {
    name: String,
    pub content: ElementContent,
}

#[derive(Clone, Debug)]
pub enum ElementContent {
    Field(Field),
    Sequence(Vec<Element>),
}

#[derive(Clone, Debug)]
pub struct Field {
    id: i64,
    pub kind: FieldType,
    pub presence: bool,
    operator: FieldOperatorInstruction,
}

#[derive(Clone, Debug)]
pub enum FieldValue {
    SInt32(i32),
    UInt32(u32),
    SInt64(i64),
    UInt64(u64),
    Decimal(f32), // FIXME
    AsciiString(Vec<u8>),
    UnicodeString(String),
    ByteVector(Vec<u8>),
}

#[derive(Clone, Debug)]
pub enum FieldType {
    SInt32,
    UInt32,
    SInt64,
    UInt64,
    Decimal,
    AsciiString,
    UnicodeString,
    ByteVector,
}

/// Templates are used to represent the structure of the data that is to be
/// encoded. A template represents a logical unit of data as it is transmitted
/// from sender to receiver. In other words, a template is used to represent a
/// specific message type. When planning to encode a data feed a user should
/// begin by converting standard message formats to templates.
///
/// Templates need to be communicated from sender to receiver. The originator of
/// the data is responsible for distributing the templates once they have been
/// defined.
#[derive(Clone, Debug)]
pub struct Template {
    /// Used for code generation.
    name: String,
    /// Each template is assigned a Template ID that can be used to uniquely
    /// describe the format of an encoded message. A Template ID will be carried
    /// in every encoded message which will provide a link to the correct
    /// template for decoding.
    pub id: i64,
    pub elements: Vec<Element>,
    dictionary: Dictionary,
}

impl Template {
    fn new(xml_document: &str) -> Result<Template, StaticError> {
        let document = roxmltree::Document::parse(xml_document).unwrap();
        let container = document.root().first_element_child().unwrap();
        let root = container.first_element_child().unwrap();
        let mut template = Template {
            name: String::new(),
            id: 0,
            elements: Vec::new(),
            dictionary: Dictionary::empty(),
        };
        template.name = root.attribute("name").unwrap().to_string();
        for node in root.children() {
            if node.is_element() {
                let instruction = node.tag_name().name();
                let field = Field {
                    id: 0,
                    kind: Template::xml_tag_to_instruction(instruction),
                    presence: Template::xml_presence_attribute_to_bool(
                        node.attribute("presence").unwrap_or("true"),
                    ),
                    operator: FieldOperatorInstruction::None,
                };
                let element = Element {
                    name: instruction.to_string(),
                    content: ElementContent::Field(field),
                };
                template.elements.push(element);
            }
        }
        Ok(template)
    }

    fn xml_tag_to_instruction(tag: &str) -> FieldType {
        match tag {
            "string" => FieldType::AsciiString,
            "uInt32" => FieldType::UInt32,
            "int32" => FieldType::SInt32,
            "uInt64" => FieldType::UInt64,
            "int64" => FieldType::SInt64,
            "decimal" => FieldType::Decimal,
            "byteVector" => FieldType::ByteVector,
            _ => FieldType::SInt32,
        }
    }

    fn xml_presence_attribute_to_bool(attribute: &str) -> bool {
        match attribute {
            "true" => true,
            "false" => false,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SIMPLE_TEMPLATE: &str = std::include_str!("templates/example.xml");

    #[test]
    fn test() {
        let template = Template::new(SIMPLE_TEMPLATE).unwrap();
        let first_element = template.elements.get(1).unwrap();
        assert_eq!(first_element.name, "BeginString");
    }
}
