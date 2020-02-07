/// This module offers pre-bundled tag dictionaries for several revisions of the
/// FIX protocol. All data comes from the official FIX Repository.
use rust_embed::RustEmbed;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Field {
    name: String,
    number: usize,
    field_type: String,
    values: Option<Vec<Value>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct FieldRef {
    name: String,
    required: char,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Value {
    value_enum: String,
    description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Message {
    name: String,
    msgtype: Option<String>,
    msgcat: Option<String>,
    #[serde(rename = "field", default)]
    fields: Vec<FieldRef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Header {
    #[serde(rename = "field", default)]
    fields: Vec<FieldRef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Trailer {
    fields: Vec<FieldRef>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
struct Component {
    name: String,
    fields: Vec<Field>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Abbreviation {
    term: String,
    usage_description: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Dictionary {
    header: Header,
    trailer: Trailer,
    messages: Vec<Message>,
    components: Vec<Component>,
    fields: HashMap<usize, Field>,
}

impl Dictionary {
    fn by_filename(filename: &str) -> Self {
        let xml = Assets::get(filename).unwrap();
        quick_xml::de::from_reader(xml.as_ref()).unwrap()
    }

    pub fn v40() -> Self {
        Dictionary::by_filename("FIX-4.0.xml")
    }

    pub fn v41() -> Self {
        Dictionary::by_filename("FIX-4.1.xml")
    }

    pub fn v42() -> Self {
        Dictionary::by_filename("FIX-4.2.xml")
    }

    pub fn v43() -> Self {
        Dictionary::by_filename("FIX-4.3.xml")
    }

    pub fn v44() -> Self {
        Dictionary::by_filename("FIX-4.4.xml")
    }

    pub fn v50() -> Self {
        Dictionary::by_filename("FIX-5.0.xml")
    }

    pub fn v50_sp1() -> Self {
        Dictionary::by_filename("FIX-5.0-SP1.xml")
    }

    pub fn v50_sp2() -> Self {
        Dictionary::by_filename("FIX-5.0-SP2.xml")
    }
}

#[derive(RustEmbed)]
#[folder = "resources/"]
struct Assets;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dictionary_v40_is_ok() {
        Dictionary::v40();
    }
}
