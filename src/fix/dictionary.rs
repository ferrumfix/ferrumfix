use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Field {
    name: String,
    number: usize,
    #[serde(rename = "type")]
    field_type: String,
    // https://github.com/tafia/quick-xml#parsing-the-value-of-a-tag
    #[serde(rename = "$value")]
    values: Option<Vec<Value>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct FieldRef {
    name: String,
    required: char,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Value {
    value_enum: String,
    description: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Message {
    name: String,
    msgtype: Option<String>,
    msgcat: Option<String>,
    #[serde(rename = "field", default)]
    fields: Vec<FieldRef>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Header {
    #[serde(rename = "field", default)]
    fields: Vec<FieldRef>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Trailer {
    #[serde(rename = "field", default)]
    fields: Vec<FieldRef>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Component {
    name: String,
    #[serde(rename = "field", default)]
    fields: Vec<Field>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "fix")]
pub struct Dictionary {
    // Meta.
    major: usize,
    minor: usize,
    servicepack: usize,
    // Tag definitions.
    header: Header,
    trailer: Trailer,
    #[serde(rename = "message", default)]
    messages: Vec<Message>,
    #[serde(rename = "component", default)]
    components: Vec<Component>,
    #[serde(rename = "field", default)]
    fields: Vec<Field>,
}

impl Dictionary {
    fn by_filename(filename: &str) -> Self {
        let xml = Assets::get(filename).unwrap();
        quick_xml::de::from_reader(xml.as_ref()).unwrap()
    }

    pub fn fix_40() -> Self {
        Dictionary::by_filename("FIX-4.0.xml")
    }

    pub fn fix_41() -> Self {
        Dictionary::by_filename("FIX-4.1.xml")
    }

    pub fn fix_42() -> Self {
        Dictionary::by_filename("FIX-4.2.xml")
    }

    pub fn fix_43() -> Self {
        Dictionary::by_filename("FIX-4.3.xml")
    }

    pub fn fix_44() -> Self {
        Dictionary::by_filename("FIX-4.4.xml")
    }
}

#[derive(RustEmbed)]
#[folder = "resources/fix/"]
struct Assets;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dictionary_fix_40_is_ok() {
        Dictionary::fix_40();
    }
}
