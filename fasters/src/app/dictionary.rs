//! Access to FIX Dictionary reference and message specifications.

use std::collections::HashMap;
use std::rc::Rc;

/// Provides access to embedded primary keys.
pub trait HasPk {
    /// The type of `Self`'s primary key.
    type Pk: Clone;

    /// Get a copy of the primary key.
    fn pk(&self) -> Self::Pk;
}

type HashMapPk<K, T> = HashMap<<K as HasPk>::Pk, T>;

/// Specification of the application layer of FIX Protocol.
///
/// All FIX Dictionaries have a version string which MUST be unique and
/// established out-of-band between involved parties.
///
/// N.B. The FIX Protocol mandates separation of concerns between session and
/// application protocol only for FIX 5.0 and subsequent versions. All FIX
/// Dictionaries with older versions will also contain information about session
/// layer.
#[derive(Clone, Debug)]
pub struct Dictionary {
    version: String,
    abbreviations: HashMap<String, Vec<String>>,
    data_types: HashMap<String, Rc<Datatype>>,
    fields: HashMap<u32, Rc<Field>>,
    components: HashMap<String, Rc<Component>>,
    messages: HashMap<String, Message>,
    msg_contents: HashMapPk<Component, Vec<MsgContent>>,
    categories: Vec<Rc<Category>>,
    header: Vec<Field>,
}

impl Dictionary {
    /// Creates a new empty FIX Dictionary named `version`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fasters::Dictionary;
    /// let dict = Dictionary::new("FIX.foobar");
    /// ```
    pub fn new<S: ToString>(version: S) -> Self {
        Dictionary {
            version: version.to_string(),
            abbreviations: HashMap::new(),
            data_types: HashMap::new(),
            fields: HashMap::new(),
            components: HashMap::new(),
            messages: HashMap::new(),
            msg_contents: HashMap::new(),
            categories: vec![],
            header: Vec::new(),
        }
    }

    /// Create a new empty FIX Dictionary with `FIX.???` as its version string.
    pub fn empty() -> Self {
        Self::new("FIX.???")
    }

    /// Return the version string associated with this FIX Dictionary (e.g.
    /// `FIXT.1.1`, `FIX.4.2`).
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }

    /// Return the known abbreviation for `term` -if any- according to the
    /// documentation of this FIX Dictionary.
    pub fn abbreviation_for<S: AsRef<str>>(&self, term: S) -> Option<&str> {
        self.abbreviations
            .get(term.as_ref())
            .and_then(|v: &Vec<String>| v.get(0))
            .map(|s| s.as_str())
    }

    pub fn get_message_by_name<S: AsRef<str>>(&self, key: S) -> Option<&Message> {
        self.messages.values().find(|m| m.name() == key.as_ref())
    }

    pub fn get_msg_content<S: AsRef<str>>(&self, msg_type: S) -> Option<&Vec<MsgContent>> {
        self.get_message_by_msg_type(msg_type)
            .map(|msg| self.msg_contents.get(&(msg.component_id as usize)).unwrap())
    }

    pub fn get_message_by_msg_type<S: AsRef<str>>(&self, key: S) -> Option<&Message> {
        self.messages.get(key.as_ref())
    }

    pub fn get_component<S: AsRef<str>>(&self, key: S) -> Option<&Component> {
        self.components.get(key.as_ref()).map(|rc| &**rc)
    }

    pub fn get_datatype_of_field(&self, tag: u32) -> &Datatype {
        &**self
            .data_types
            .get(&self.fields.get(&tag).unwrap().data_type.name)
            .unwrap()
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.values()
    }

    pub fn get_categories(&self) -> impl Iterator<Item = &Category> {
        self.categories.iter().map(|rc_cat| &**rc_cat)
    }

    pub fn get_field(&self, key: u32) -> Option<&Field> {
        self.fields.get(&key).map(|rc| &**rc)
    }

    pub fn from_quickfix_spec(input: &str) -> Result<Self, ParseDictionaryError> {
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
        let node_components = root
            .descendants()
            .find(|n| n.has_tag_name("components"))
            .unwrap();
        let node_fields = root
            .descendants()
            .find(|n| n.has_tag_name("fields"))
            .unwrap();
        let mut fields_by_name = HashMap::new();
        for node in node_fields.children() {
            if node.is_element() {
                let field = Field::from_quickfix_node(&mut dictionary, node);
                fields_by_name.insert(field.name.clone(), field.clone());
                dictionary.fields.insert(field.tag, Rc::new(field));
            }
        }
        for node in node_messages.children() {
            if node.is_element() {
                let category_name = node.attribute("msgcat").unwrap().to_string();
                let category = Category {
                    name: category_name,
                    fixml_filename: String::new(),
                };
                dictionary.categories.push(Rc::new(category));
            }
        }
        for node in node_components.children() {
            if node.is_element() {
                let component = Component::from_quickfix_node(&mut dictionary, node);
                dictionary
                    .components
                    .insert(component.name.clone(), Rc::new(component));
            }
        }
        for node in node_messages.children() {
            if node.is_element() {
                let message = Message::from_quickfix_node(&mut dictionary, node);
                dictionary
                    .messages
                    .insert(message.msg_type().to_string(), message);
            }
        }
        Ok(dictionary)
    }
}

#[derive(Clone, Debug)]
pub enum ParseDictionaryError {
    InvalidFormat,
    InvalidData(String),
}

trait FromQuickFixNode {
    fn from_quickfix_node(dict: &mut Dictionary, node: roxmltree::Node<'_, '_>) -> Self;
}

//impl<'a> Abbreviations<'a> {
//    pub fn new() -> Self {
//        Self::default()
//    }
//
//    pub fn query(&self, term: &str) -> Option<&str> {
//        self.data.get(term).map(|a| a.get(0).unwrap().as_str())
//    }
//
//    pub fn query_all(&self, term: &str) -> Option<impl Iterator<Item = &str>> {
//        self.data.get(term).map(|v| v.iter().map(|s| s.as_str()))
//    }
//
//    pub fn insert(&self, term: &str, abbreviation: String) {
//        self.data.entry(*term).or_default().push(abbreviation);
//    }
//}

//impl Default for Abbreviations {
//    fn default() -> Self {
//        Abbreviations {
//            data: HashMap::default()
//        }
//    }
//}

#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    Int,
    Float,
    Char,
    String,
    Data,
}

#[derive(Clone, Debug)]
pub struct Category {
    /// **Primary key**. A string uniquely identifying this category.
    name: String,
    /// The FIXML file name for a Category.
    fixml_filename: String,
}

#[derive(Clone, Debug)]
pub struct Component {
    /// **Primary key.** The unique integer identifier of this component
    /// type.
    id: usize,
    component_type: ComponentType,
    items: Vec<LayoutItem>,
    category: Rc<Category>,
    /// The human readable name of the component.
    name: String,
    /// The name for this component when used in an XML context.
    abbr_name: Option<String>,
}

impl Component {
    pub fn id(&self) -> u32 {
        self.id as u32
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl FromQuickFixNode for Component {
    fn from_quickfix_node(dict: &mut Dictionary, node: roxmltree::Node<'_, '_>) -> Self {
        let name = node.attribute("name").unwrap().to_string();
        let layout = node
            .children()
            .filter(|n| n.is_element())
            .map(|n| LayoutItem::from_quickfix_node(dict, n))
            .collect();
        Component {
            id: 0,
            component_type: ComponentType::Block,
            items: layout,
            category: dict.categories.get(0).unwrap().clone(),
            name: name,
            abbr_name: None,
        }
    }
}

impl HasPk for Component {
    type Pk = usize;

    fn pk(&self) -> Self::Pk {
        self.id
    }
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
    base_type: Option<String>,
    /// Human readable description of this Datatype.
    description: String,
    /// A string that contains examples values for a datatype
    examples: Vec<String>,
    // TODO: 'XML'.
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {}

/// A field is identified by a unique tag number and a name. Each field in a
/// message is associated with a value.
#[derive(Clone, Debug)]
pub struct Field {
    /// A human readable string representing the name of the field.
    name: String,
    /// **Primary key.** A positive integer representing the unique
    /// identifier for this field type.
    tag: u32,
    /// The datatype of the field.
    data_type: Rc<Datatype>,
    /// The associated data field. If given, this field represents the length of
    /// the referenced data field
    associated_data_tag: Option<usize>,
    /// Abbreviated form of the Name, typically to specify the element name when
    /// the field is used in an XML message. Can be overridden by BaseCategory /
    /// BaseCategoryAbbrName.
    abbr_name: Option<String>,
    /// Specifies the base message category when field is used in an XML message.
    base_category_id: Option<usize>,
    /// If BaseCategory is specified, this is the XML element identifier to use
    /// for this field, overriding AbbrName.
    base_category_abbr_name: Option<String>,
    /// Indicates whether the field is required in an XML message.
    required: bool,
    description: Option<String>,
}

fn str_to_basetype(s: &str) -> BaseType {
    match s {
        "string" => BaseType::String,
        "char" => BaseType::Char,
        "int" => BaseType::Int,
        "float" => BaseType::Float,
        "data" => BaseType::Data,
        _ => BaseType::Char, // FIXME
    }
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

    pub fn basetype(&self) -> BaseType {
        str_to_basetype((&self.data_type.base_type).as_ref().unwrap().as_str())
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn tag(&self) -> u32 {
        self.tag
    }

    pub fn data_type(&self) -> &str {
        self.data_type.name.as_str()
    }
}

impl FromQuickFixNode for Field {
    fn from_quickfix_node(dict: &mut Dictionary, node: roxmltree::Node<'_, '_>) -> Self {
        let data_type_name = node.attribute("type").unwrap().to_string();
        let data_type_opt = dict
            .data_types
            .values()
            .find(|dt| dt.name == data_type_name);
        let data_type = match data_type_opt {
            Some(dt) => dt.clone(),
            None => {
                let dt = Rc::new(Datatype {
                    name: data_type_name,
                    base_type: Some(String::new()),
                    description: String::new(),
                    examples: vec![],
                });
                dict.data_types.insert(dt.name.clone(), dt.clone());
                dt.clone()
            }
        };
        Field {
            name: node.attribute("name").unwrap().to_string(),
            tag: node.attribute("number").unwrap().parse().unwrap(),
            data_type,
            associated_data_tag: None,
            required: true,
            abbr_name: None,
            base_category_abbr_name: None,
            base_category_id: None,
            description: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldRef {
    pub name: String,
    pub required: char,
}

#[derive(Clone, Debug)]
pub enum LayoutItem {
    Component(Rc<Component>, bool),
    Group(Vec<LayoutItem>, bool),
    Field(Rc<Field>, bool),
}

impl FromQuickFixNode for LayoutItem {
    fn from_quickfix_node(dict: &mut Dictionary, node: roxmltree::Node<'_, '_>) -> Self {
        let name = node.attribute("name").unwrap();
        let required = node.attribute("required").unwrap() == "Y";
        let tag = node.tag_name().name();
        match tag {
            "field" => {
                let field = dict.fields.values().find(|f| f.name() == name).unwrap();
                LayoutItem::Field(field.clone(), required)
            }
            "component" => {
                let component = dict.components.get(name).unwrap();
                LayoutItem::Component(component.clone(), required)
            }
            "group" => {
                let items = node
                    .children()
                    .filter(|n| n.is_element())
                    .map(|child| LayoutItem::from_quickfix_node(dict, child))
                    .collect();
                LayoutItem::Group(items, required)
            }
            _ => {
                panic!()
            }
        }
    }
}

/// Available versions of the FIX standard.
#[derive(Clone, Debug, PartialEq)]
pub struct FixVersion(String);

#[derive(Clone, Debug)]
pub struct Message {
    /// The unique integer identifier of this message type.
    component_id: u32,
    /// **Primary key**. The unique character identifier of this message
    /// type; used literally in FIX messages.
    msg_type: String,
    /// The name of this message type.
    name: String,
    /// Identifier of the category to which this message belongs.
    category: Rc<Category>,
    /// Identifier of the section to which this message belongs.
    section_id: String,
    layout: Vec<LayoutItem>,
    /// The abbreviated name of this message, when used in an XML context.
    abbr_name: Option<String>,
    /// A boolean used to indicate if the message is to be generated as part
    /// of FIXML.
    required: bool,
    description: String,
    elaboration: Option<String>,
}

impl Message {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn msg_type(&self) -> &str {
        self.msg_type.as_str()
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn component_id(&self) -> u32 {
        self.component_id
    }

    pub fn layout(&self) -> impl Iterator<Item = &LayoutItem> {
        self.layout.iter()
    }
}

impl FromQuickFixNode for Message {
    fn from_quickfix_node(dict: &mut Dictionary, node: roxmltree::Node<'_, '_>) -> Self {
        let category_name = node.attribute("msgcat").unwrap().to_string();
        let category_opt = dict.categories.iter().find(|c| c.name == category_name);
        let category = match category_opt {
            Some(cat) => cat.clone(),
            None => {
                let cat = Rc::new(Category {
                    name: category_name,
                    fixml_filename: String::new(),
                });
                dict.categories.push(cat.clone());
                cat.clone()
            }
        };
        let mut layout = Vec::new();
        for child in node.children() {
            if child.is_element() {
                layout.push(LayoutItem::from_quickfix_node(dict, child));
            }
        }
        Message {
            name: node.attribute("name").unwrap().to_string(),
            msg_type: node.attribute("msgtype").unwrap().to_string(),
            component_id: 0,
            category: category.clone(),
            section_id: String::new(),
            layout,
            abbr_name: None,
            required: true,
            elaboration: None,
            description: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MsgContent {
    component_id: usize,
    pub tag_text: String,
    pub reqd: char,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Section {}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    value_enum: String,
    description: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;
    use rust_embed::RustEmbed;

    #[derive(RustEmbed)]
    #[folder = "resources/quickfix/"]
    struct QuickFixDicts;

    #[test]
    pub fn fixt11_quickfix_is_ok() {
        let xml_spec = QuickFixDicts::get("FIXT-1.1.xml").unwrap();
        let dict = Dictionary::from_quickfix_spec(std::str::from_utf8(xml_spec.as_ref()).unwrap())
            .unwrap();
        let msg_heartbeat = dict.get_message_by_name("Heartbeat").unwrap();
        assert_eq!(msg_heartbeat.msg_type(), "0");
        assert_eq!(msg_heartbeat.name(), "Heartbeat".to_string());
        assert!(msg_heartbeat.layout().any(|c| {
            if let LayoutItem::Field(f, false) = c {
                f.name() == "TestReqID"
            } else {
                false
            }
        }));
    }
}
