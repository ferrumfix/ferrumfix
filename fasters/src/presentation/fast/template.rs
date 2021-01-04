use super::errors::StaticError;
use super::field_operators::FieldOperatorInstruction;
use crate::app::dictionary::Dictionary;

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
pub enum FieldPrimitiveType {
    SInt32,
    UInt32,
    SInt64,
    UInt64,
    Decimal,
    AsciiString,
    UnicodeString,
    ByteVector,
}

#[derive(Clone, Debug)]
pub struct FieldInstruction {
    field_type: FieldType,
    name: String,
    id: u32,
    mandatory: bool,
    operator: FieldOperatorInstruction,
}

impl FieldInstruction {
    pub fn kind(&self) -> &FieldType {
        &self.field_type
    }

    pub fn is_mandatory(&self) -> bool {
        self.mandatory
    }
}

#[derive(Clone, Debug)]
pub enum FieldType {
    Primitive(FieldPrimitiveType),
    Group(u32),
}

impl FieldInstruction {
    fn from_template(node: roxmltree::Node) -> Result<Self, StaticError> {
        let name = node.attribute("name").ok_or(StaticError::S1)?;
        let id = node.attribute("id").unwrap().parse().unwrap();
        let mandatory = {
            let attr = node.attribute("presence").unwrap_or("true");
            attr == "true"
        };
        let type_name = node.tag_name().name();
        let instruction = FieldInstruction {
            field_type: FieldType::Primitive(Template::xml_tag_to_instruction(type_name)?),
            name: name.to_string(),
            id,
            mandatory,
            operator: FieldOperatorInstruction::Constant,
        };
        Ok(instruction)
    }
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
    /// Each template is assigned a Template ID that can be used to uniquely
    /// describe the format of an encoded message. A Template ID will be carried
    /// in every encoded message which will provide a link to the correct
    /// template for decoding.
    id: Option<u32>,
    /// Used for code generation.
    name: String,
    instructions: Vec<FieldInstruction>,
    dictionary: Dictionary,
}

impl Template {
    fn new(xml_document: &str) -> Result<Template, StaticError> {
        let document = roxmltree::Document::parse(xml_document).unwrap();
        let container = document.root().first_element_child().unwrap();
        let root = container.first_element_child().unwrap();
        Template::from_xml(Dictionary::empty(), root)
    }

    fn from_xml(dict: Dictionary, root: roxmltree::Node) -> Result<Self, StaticError> {
        debug_assert_eq!(root.tag_name().name(), "template");
        let name = root.attribute("name").unwrap();
        let id = {
            let id = root.attribute("id");
            match id {
                Some(num) => Some(num.parse().map_err(|_| StaticError::S1)?),
                None => None,
            }
        };
        let mut instructions = Vec::new();
        for node in root.children() {
            if node.is_element() {
                match node.tag_name().name() {
                    "sequence" => {
                        for child in node.children() {
                            if child.is_element() {
                                let instruction = FieldInstruction::from_template(child)?;
                                instructions.push(instruction);
                            }
                        }
                    }
                    "typeRef" => (),
                    _ => {
                        let instruction = FieldInstruction::from_template(node)?;
                        instructions.push(instruction);
                    }
                }
            }
        }
        let template = Template {
            id,
            name: name.to_string(),
            instructions,
            dictionary: dict,
        };
        Ok(template)
    }

    pub fn id(&self) -> Option<u32> {
        self.id
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn iter_items(&self) -> impl Iterator<Item = &FieldInstruction> {
        self.instructions.iter()
    }

    fn xml_tag_to_instruction(tag: &str) -> Result<FieldPrimitiveType, StaticError> {
        Ok(match tag {
            "string" => FieldPrimitiveType::AsciiString,
            "uInt32" => FieldPrimitiveType::UInt32,
            "int32" => FieldPrimitiveType::SInt32,
            "uInt64" => FieldPrimitiveType::UInt64,
            "int64" => FieldPrimitiveType::SInt64,
            "decimal" => FieldPrimitiveType::Decimal,
            "byteVector" => FieldPrimitiveType::ByteVector,
            "length" => FieldPrimitiveType::UInt32,
            _ => return Err(StaticError::S1),
        })
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
    fn first_field_instruction() {
        let template = Template::new(SIMPLE_TEMPLATE).unwrap();
        let first_field_instruction = template.instructions.get(0).unwrap();
        assert_eq!(first_field_instruction.name, "BeginString");
    }
}
