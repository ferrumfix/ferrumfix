use super::errors::StaticError;
use super::field_operators::FieldOperatorInstruction;
use super::Decimal;

#[derive(Clone, Debug)]
pub enum PrimitiveValue<'a> {
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    Decimal(Decimal),
    AsciiString(&'a [u8]),
    Utf8String(&'a str),
    Bytes(&'a [u8]),
}

#[derive(Clone, Debug, Copy)]
pub enum PrimitiveType {
    I32,
    U32,
    I64,
    U64,
    Decimal,
    AsciiString,
    Utf8String,
    Bytes,
}

#[derive(Clone, Debug)]
pub struct FieldInstruction {
    pub field_type: FieldType,
    pub name: String,
    pub id: u32,
    pub mandatory: bool,
    pub operator: FieldOperatorInstruction,
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
    Primitive(PrimitiveType),
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
            field_type: Template::xml_tag_to_instruction(type_name)?,
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
}

impl Template {
    pub fn new(xml_document: &str) -> Result<Template, StaticError> {
        let document = roxmltree::Document::parse(xml_document).unwrap();
        let container = document.root().first_element_child().unwrap();
        let root = container.first_element_child().unwrap();
        Template::from_xml(root)
    }

    fn from_xml(root: roxmltree::Node) -> Result<Self, StaticError> {
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

    fn xml_tag_to_instruction(tag: &str) -> Result<FieldType, StaticError> {
        Ok(match tag {
            "string" => FieldType::Primitive(PrimitiveType::AsciiString),
            "uInt32" => FieldType::Primitive(PrimitiveType::U32),
            "int32" => FieldType::Primitive(PrimitiveType::I32),
            "uInt64" => FieldType::Primitive(PrimitiveType::U64),
            "int64" => FieldType::Primitive(PrimitiveType::I64),
            "decimal" => FieldType::Primitive(PrimitiveType::Decimal),
            "byteVector" => FieldType::Primitive(PrimitiveType::Decimal),
            "length" => FieldType::Primitive(PrimitiveType::U32),
            _ => return Err(StaticError::S1),
        })
    }

    fn _xml_presence_attribute_to_bool(attribute: &str) -> bool {
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
