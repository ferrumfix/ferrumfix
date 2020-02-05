use crate::err::{Error, Result};
use crate::settings::Settings;
use codegen::Scope;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Element {
    name: String,
    content: ElementContent,
}

#[derive(Clone, Debug)]
pub enum ElementContent {
    Field(Field),
    Sequence(Vec<Element>),
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
    id: i32,
    elements: Vec<Element>,
    dictionary: HashMap<String, Element>,
}

impl Template {
    pub fn new() -> Self {
        Template {
            name: String::new(),
            id: 0,
            elements: vec![],
            dictionary: HashMap::new(),
        }
    }

    /// Custom tag mapping.
    pub fn new_dict(dictionary: HashMap<String, Element>) -> Self {
        Template {
            name: String::new(),
            id: -1,
            elements: vec![],
            dictionary,
        }
    }

    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_element(mut self, elem: Element) -> Self {
        self.elements.push(elem);
        self
    }

    pub fn from_xml<S: AsRef<str>>(xml: S) -> Result<Self> {
        //fixml::State::new(xml.as_ref()).consume()
        unimplemented!()
    }

    /// Parses a template definition using the traditional FIX `tag=value` syntax.
    pub fn from_compact<S: AsRef<str>>(formula: S) -> Result<Self> {
        let formula = formula.as_ref();
        unimplemented!()
    }

    pub fn codegen(&self, settings: Settings) -> String {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    /// Used for code generation.
    name: String,
    field_encoding_operator: Option<String>,
    data_type: String,
    details: HashMap<String, String>,
}

//impl Field {
//    fn from_string(s: String) -> Field {
//        let mut tag_number_string = String::new();
//        let mut chars = s.chars();
//        while let Some(c) = chars.next() {
//            if c.is_digit(10) {
//                tag_number_string.push(c);
//            } else {
//                break;
//            }
//        }
//        Field {
//            tag_number: tag_number_string.parse().unwrap(),
//            field_encoding_operator: chars.next().unwrap(),
//            data_type_descriptor: chars.next().unwrap(),
//            details: String::new(),
//        }
//    }
//}

struct Description {
    template_name: String,
    elements: Vec<Element>,
}

impl Description {
    fn codegen(self) -> String {
        let mut scope = Scope::new();

        let mut struct_definition = scope
            .new_struct(self.template_name.as_str())
            .derive("Debug")
            .derive("Clone");
        for element in self.elements {
            struct_definition = struct_definition.field("one", "usize");
        }
        scope.to_string()
    }
}
