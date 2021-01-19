//! Access to FIX Dictionary reference and message specifications.

use crate::app::Version;
use quickfix::{ParseDictionaryError, QuickFixReader};
use std::collections::HashMap;
use std::ops::Range;

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
struct MsgType(u32);

impl From<&[u8]> for MsgType {
    fn from(bytes: &[u8]) -> Self {
        let mut arr_bytes: [u8; 4] = [0, 0, 0, 0];
        for (i, byte) in bytes.iter().take(4).enumerate() {
            arr_bytes[i] = *byte;
        }
        MsgType(u32::from_be_bytes(arr_bytes))
    }
}

type InternalId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum PKey {
    Abbreviation(String),
    CategoryByName(String),
    ComponentByName(String),
    DatatypeByName(String),
    FieldByTag(u32),
    FieldByName(String),
    MessageByName(String),
    MessageByMsgType(MsgType),
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
enum PKeyRef<'a> {
    Abbreviation(&'a str),
    CategoryByName(&'a str),
    ComponentByName(&'a str),
    DatatypeByName(&'a str),
    FieldByTag(u32),
    FieldByName(&'a str),
    MessageByName(&'a str),
    MessageByMsgType(MsgType),
}

impl PKey {
    fn as_ref<'a>(&'a self) -> PKeyRef<'a> {
        match self {
            PKey::Abbreviation(s) => PKeyRef::Abbreviation(s.as_str()),
            PKey::CategoryByName(s) => PKeyRef::CategoryByName(s.as_str()),
            PKey::ComponentByName(s) => PKeyRef::ComponentByName(s.as_str()),
            PKey::DatatypeByName(s) => PKeyRef::DatatypeByName(s.as_str()),
            PKey::FieldByTag(t) => PKeyRef::FieldByTag(*t),
            PKey::FieldByName(s) => PKeyRef::FieldByName(s.as_str()),
            PKey::MessageByName(s) => PKeyRef::MessageByName(s.as_str()),
            PKey::MessageByMsgType(t) => PKeyRef::MessageByMsgType(*t),
        }
    }
}

trait SymbolTableIndex {
    fn to_key(&self) -> PKeyRef;
}

impl SymbolTableIndex for PKey {
    fn to_key(&self) -> PKeyRef {
        self.as_ref()
    }
}

impl<'a> SymbolTableIndex for PKeyRef<'a> {
    fn to_key(&self) -> PKeyRef {
        *self
    }
}

impl<'a> std::hash::Hash for dyn SymbolTableIndex + 'a {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_key().hash(state);
    }
}

impl<'a> std::borrow::Borrow<dyn SymbolTableIndex + 'a> for PKey {
    fn borrow(&self) -> &(dyn SymbolTableIndex + 'a) {
        self
    }
}

impl<'a> Eq for dyn SymbolTableIndex + 'a {}

impl<'a> PartialEq for dyn SymbolTableIndex + 'a {
    fn eq(&self, other: &dyn SymbolTableIndex) -> bool {
        self.to_key() == other.to_key()
    }
}

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
    symbol_table: HashMap<PKey, InternalId>,
    abbreviations: Vec<AbbreviatonData>,
    data_types: Vec<DatatypeData>,
    fields: Vec<FieldData>,
    components: Vec<ComponentData>,
    messages: Vec<MessageData>,
    layout_items: Vec<LayoutItemData>,
    categories: Vec<CategoryData>,
    header: Vec<FieldData>,
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
            symbol_table: HashMap::new(),
            abbreviations: Vec::new(),
            data_types: Vec::new(),
            fields: Vec::new(),
            components: Vec::new(),
            messages: Vec::new(),
            layout_items: Vec::new(),
            categories: vec![],
            header: Vec::new(),
        }
    }

    pub fn from_version(version: Version) -> Self {
        Dictionary::save_definition_spec(version.get_quickfix_spec()).unwrap()
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

    fn symbol(&self, pkey: PKeyRef) -> Option<&u32> {
        self.symbol_table.get(&pkey as &dyn SymbolTableIndex)
    }

    /// Return the known abbreviation for `term` -if any- according to the
    /// documentation of this FIX Dictionary.
    pub fn abbreviation_for<S: AsRef<str>>(&self, term: S) -> Option<Abbreviation> {
        self.symbol(PKeyRef::Abbreviation(term.as_ref()))
            .map(|iid| self.abbreviations.get(*iid as usize).unwrap())
            .map(move |data| Abbreviation(self, data))
    }

    pub fn get_message_by_name<S: AsRef<str>>(&self, key: S) -> Option<Message> {
        self.symbol(PKeyRef::MessageByName(key.as_ref()))
            .map(|iid| self.messages.get(*iid as usize).unwrap())
            .map(|data| Message(self, data))
    }

    pub fn get_message_by_msg_type<S: AsRef<str>>(&self, key: S) -> Option<Message> {
        self.symbol(PKeyRef::MessageByMsgType(MsgType::from(
            key.as_ref().as_bytes(),
        )))
        .map(|iid| self.messages.get(*iid as usize).unwrap())
        .map(|data| Message(self, data))
    }

    pub fn get_component<S: AsRef<str>>(&self, key: S) -> Option<Component> {
        self.symbol(PKeyRef::ComponentByName(key.as_ref()))
            .map(|iid| self.components.get(*iid as usize).unwrap())
            .map(|data| Component(self, data))
    }

    pub fn components(&self) -> impl Iterator<Item = Component> {
        self.components
            .iter()
            .map(move |data| Component(&self, data))
    }

    pub fn messages(&self) -> impl Iterator<Item = Message> {
        self.messages.iter().map(move |data| Message(&self, data))
    }

    pub fn categories(&self) -> impl Iterator<Item = Category> {
        self.categories
            .iter()
            .map(move |data| Category(&self, data))
    }

    pub fn get_field(&self, key: u32) -> Option<Field> {
        self.symbol(PKeyRef::FieldByTag(key))
            .map(|iid| self.fields.get(*iid as usize).unwrap())
            .map(|data| Field(self, data))
    }

    pub fn get_field_by_name<S: AsRef<str>>(&self, name: S) -> Option<Field> {
        self.symbol(PKeyRef::FieldByName(name.as_ref()))
            .map(|iid| self.fields.get(*iid as usize).unwrap())
            .map(|data| Field(self, data))
    }

    pub fn save_definition_spec<S: AsRef<str>>(input: S) -> Result<Self, ParseDictionaryError> {
        let xml_document = roxmltree::Document::parse(input.as_ref()).unwrap();
        QuickFixReader::new(&xml_document)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BaseType {
    Int,
    Float,
    Char,
    String,
    Data,
}

#[derive(Clone, Debug)]
pub struct CategoryData {
    /// **Primary key**. A string uniquely identifying this category.
    name: String,
    /// The FIXML file name for a Category.
    fixml_filename: String,
}

#[derive(Clone, Debug)]
pub struct Category<'a>(&'a Dictionary, &'a CategoryData);

#[derive(Clone, Debug)]
struct AbbreviatonData {
    abbreviation: String,
    is_last: bool,
}

pub struct Abbreviation<'a>(&'a Dictionary, &'a AbbreviatonData);

impl<'a> Abbreviation<'a> {
    pub fn term(&self) -> &str {
        self.1.abbreviation.as_str()
    }
}

#[derive(Clone, Debug)]
pub struct ComponentData {
    /// **Primary key.** The unique integer identifier of this component
    /// type.
    id: usize,
    component_type: ComponentType,
    layout_items_iid_range: Range<u32>,
    category_iid: InternalId,
    /// The human readable name of the component.
    name: String,
    /// The name for this component when used in an XML context.
    abbr_name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Component<'a>(&'a Dictionary, &'a ComponentData);

impl<'a> Component<'a> {
    pub fn id(&self) -> u32 {
        self.1.id as u32
    }

    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    pub fn category(&self) -> Category {
        let data = self.0.categories.get(self.1.category_iid as usize).unwrap();
        Category(self.0, data)
    }

    pub fn items(&self) -> impl Iterator<Item = LayoutItem> {
        let start = self.1.layout_items_iid_range.start as usize;
        let end = self.1.layout_items_iid_range.end as usize;
        self.0.layout_items[start..end]
            .iter()
            .map(move |data| LayoutItem(self.0, data))
    }

    pub fn contains_field(&self, field: &Field) -> bool {
        self.items().any(|layout_item| {
            if let LayoutItemKind::Field(f) = layout_item.kind() {
                f.tag() == field.tag()
            } else {
                false
            }
        })
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
struct DatatypeData {
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

pub struct Datatype<'a>(&'a Dictionary, &'a DatatypeData);

impl<'a> Datatype<'a> {
    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    pub fn basetype(&self) -> BaseType {
        str_to_basetype(self.1.base_type.as_ref().unwrap().as_str())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {}

/// A field is identified by a unique tag number and a name. Each field in a
/// message is associated with a value.
#[derive(Clone, Debug)]
pub struct FieldData {
    /// A human readable string representing the name of the field.
    name: String,
    /// **Primary key.** A positive integer representing the unique
    /// identifier for this field type.
    tag: u32,
    /// The datatype of the field.
    data_type_iid: InternalId,
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

pub struct Field<'a>(&'a Dictionary, &'a FieldData);

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

impl<'a> Field<'a> {
    pub fn doc_url_onixs(&self, version: &str) -> String {
        let v = match version {
            "FIX.4.0" => "4.0",
            "FIX.4.1" => "4.1",
            "FIX.4.2" => "4.2",
            "FIX.4.3" => "4.3",
            "FIX.4.4" => "4.4",
            "FIX.5.0" => "5.0",
            "FIX.5.0SP1" => "5.0.SP1",
            "FIX.5.0SP2" => "5.0.SP2",
            "FIXT.1.1" => "FIXT.1.1",
            s => s,
        };
        let mut url = "https://www.onixs.biz/fix-dictionary/".to_string();
        url.push_str(v);
        url.push_str("/tagNum_");
        url.push_str(self.1.tag.to_string().as_str());
        url.push_str(".html");
        url
    }

    pub fn basetype(&self) -> BaseType {
        str_to_basetype(
            (&self.data_type().1.base_type)
                .as_ref()
                .unwrap_or(&"char".to_string())
                .as_str(),
        )
    }

    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    pub fn tag(&self) -> u32 {
        self.1.tag
    }

    pub fn data_type(&self) -> Datatype {
        let data = self
            .0
            .data_types
            .get(self.1.data_type_iid as usize)
            .unwrap();
        Datatype(self.0, data)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldRef {
    pub name: String,
    pub required: char,
}

#[derive(Clone, Debug)]
enum LayoutItemKindData {
    Component(u32),
    Group(Range<u32>),
    Field(u32),
}

#[derive(Clone, Debug)]
struct LayoutItemData {
    required: bool,
    kind: LayoutItemKindData,
}

#[derive(Clone, Debug)]
pub struct LayoutItem<'a>(&'a Dictionary, &'a LayoutItemData);

pub enum LayoutItemKind<'a> {
    Component(Component<'a>),
    Group(),
    Field(Field<'a>),
}

impl<'a> LayoutItem<'a> {
    pub fn required(&self) -> bool {
        self.1.required
    }

    pub fn kind(&self) -> LayoutItemKind {
        match &self.1.kind {
            LayoutItemKindData::Component(n) => LayoutItemKind::Component(Component(
                self.0,
                self.0.components.get(*n as usize).unwrap(),
            )),
            LayoutItemKindData::Group(_range) => {
                LayoutItemKind::Group() // FIXME
            }
            LayoutItemKindData::Field(n) => {
                LayoutItemKind::Field(Field(self.0, self.0.fields.get(*n as usize).unwrap()))
            }
        }
    }

    pub fn tag_text(&self) -> &str {
        match &self.1.kind {
            LayoutItemKindData::Component(n) => {
                self.0.components.get(*n as usize).unwrap().name.as_str()
            }
            LayoutItemKindData::Group(_range) => "",
            LayoutItemKindData::Field(n) => self.0.fields.get(*n as usize).unwrap().name.as_str(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MessageData {
    /// The unique integer identifier of this message type.
    component_id: u32,
    /// **Primary key**. The unique character identifier of this message
    /// type; used literally in FIX messages.
    msg_type: String,
    /// The name of this message type.
    name: String,
    /// Identifier of the category to which this message belongs.
    category_iid: InternalId,
    /// Identifier of the section to which this message belongs.
    section_id: String,
    layout_items: Range<InternalId>,
    /// The abbreviated name of this message, when used in an XML context.
    abbr_name: Option<String>,
    /// A boolean used to indicate if the message is to be generated as part
    /// of FIXML.
    required: bool,
    description: String,
    elaboration: Option<String>,
}

pub struct Message<'a>(&'a Dictionary, &'a MessageData);

impl<'a> Message<'a> {
    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    pub fn msg_type(&self) -> &str {
        self.1.msg_type.as_str()
    }

    pub fn description(&self) -> &str {
        &self.1.description
    }

    pub fn component_id(&self) -> u32 {
        self.1.component_id
    }

    pub fn layout(&self) -> impl Iterator<Item = LayoutItem> {
        let start = self.1.layout_items.start as usize;
        let end = self.1.layout_items.end as usize;
        self.0.layout_items[start..end]
            .iter()
            .map(move |data| LayoutItem(self.0, data))
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

mod quickfix {
    use super::*;

    pub(crate) struct QuickFixReader<'a> {
        node_with_header: roxmltree::Node<'a, 'a>,
        node_with_trailer: roxmltree::Node<'a, 'a>,
        node_with_components: roxmltree::Node<'a, 'a>,
        node_with_messages: roxmltree::Node<'a, 'a>,
        node_with_fields: roxmltree::Node<'a, 'a>,
        dict: Dictionary,
    }

    impl<'a> QuickFixReader<'a> {
        pub fn new(
            xml_document: &'a roxmltree::Document<'a>,
        ) -> Result<Dictionary, ParseDictionaryError> {
            let mut reader = Self::empty(&xml_document)?;
            for child in reader.node_with_fields.children() {
                if child.is_element() {
                    reader.add_field(child);
                }
            }
            for child in reader.node_with_components.children() {
                if child.is_element() {
                    reader.add_component(child);
                }
            }
            for child in reader.node_with_messages.children() {
                if child.is_element() {
                    reader.add_message(child);
                }
            }
            reader.add_component_with_name(reader.node_with_header, "StandardHeader");
            reader.add_component_with_name(reader.node_with_trailer, "StandardTrailer");
            Ok(reader.dict)
        }

        fn empty(xml_document: &'a roxmltree::Document<'a>) -> Result<Self, ParseDictionaryError> {
            let root = xml_document.root_element();
            let find_tagged_child = |tag: &str| {
                root.children()
                    .find(|n| n.has_tag_name(tag))
                    .ok_or_else(|| {
                        ParseDictionaryError::InvalidData(format!("<{}> tag not found", tag))
                    })
            };
            let version_type = root
                .attribute("type")
                .ok_or(ParseDictionaryError::InvalidData(
                    "No version attribute.".to_string(),
                ))?;
            let version_major =
                root.attribute("major")
                    .ok_or(ParseDictionaryError::InvalidData(
                        "No majorr version attribute.".to_string(),
                    ))?;
            let version_minor =
                root.attribute("minor")
                    .ok_or(ParseDictionaryError::InvalidData(
                        "No minor version attribute.".to_string(),
                    ))?;
            let version = format!("{}.{}.{}", version_type, version_major, version_minor);
            Ok(QuickFixReader {
                node_with_header: find_tagged_child("header")?,
                node_with_trailer: find_tagged_child("trailer")?,
                node_with_messages: find_tagged_child("messages")?,
                node_with_components: find_tagged_child("components")?,
                node_with_fields: find_tagged_child("fields")?,
                dict: Dictionary::new(version),
            })
        }

        fn add_field(&mut self, node: roxmltree::Node) {
            let iid = self.dict.fields.len() as u32;
            let field = FieldData::definition_from_node(&mut self.dict, node);
            self.dict
                .symbol_table
                .insert(PKey::FieldByName(field.name.clone()).into(), iid);
            self.dict
                .symbol_table
                .insert(PKey::FieldByTag(field.tag as u32).into(), iid);
            self.dict.fields.push(field);
        }

        fn add_component_with_name<S: AsRef<str>>(&mut self, node: roxmltree::Node, name: S) {
            let iid = self.dict.components.len();
            let component =
                ComponentData::definition_from_node_with_name(&mut self.dict, node, name.as_ref());
            self.dict
                .symbol_table
                .insert(PKey::ComponentByName(name.as_ref().to_string()), iid as u32);
            self.dict.components.push(component);
        }

        fn add_component(&mut self, node: roxmltree::Node) {
            let iid = self.dict.components.len();
            let component = ComponentData::definition_from_node(&mut self.dict, node);
            self.dict
                .symbol_table
                .insert(PKey::ComponentByName(component.name.clone()), iid as u32);
            self.dict.components.push(component);
        }

        fn add_message(&mut self, node: roxmltree::Node) {
            let iid = self.dict.messages.len() as u32;
            let message = MessageData::definition_from_node(&mut self.dict, node);
            self.dict
                .symbol_table
                .insert(PKey::MessageByName(message.name.clone()), iid);
            self.dict.symbol_table.insert(
                PKey::MessageByMsgType(MsgType::from(message.msg_type.as_bytes())),
                iid,
            );
            self.dict.messages.push(message);
        }
    }

    impl ComponentData {
        fn definition_from_node(dict: &mut Dictionary, node: roxmltree::Node) -> Self {
            debug_assert_eq!(node.tag_name().name(), "component");
            let name = node.attribute("name").unwrap().to_string();
            Self::definition_from_node_with_name(dict, node, name)
        }

        fn definition_from_node_with_name<S: AsRef<str>>(
            dict: &mut Dictionary,
            node: roxmltree::Node,
            name: S,
        ) -> Self {
            let layout_start = dict.layout_items.len() as u32;
            for child in node.children() {
                if child.is_element() {
                    // We don't need IID's because we're dealing with ranges.
                    let item = LayoutItemData::save_definition(dict, child);
                    dict.layout_items.push(item);
                }
            }
            let layout_end = dict.layout_items.len() as u32;
            ComponentData {
                id: 0,
                component_type: ComponentType::Block,
                layout_items_iid_range: layout_start..layout_end,
                category_iid: 0, // FIXME
                name: name.as_ref().to_string(),
                abbr_name: None,
            }
        }

        fn get_or_create_iid_from_ref(dict: &mut Dictionary, node: roxmltree::Node) -> InternalId {
            debug_assert_eq!(node.tag_name().name(), "component");
            let name = node.attribute("name").unwrap();
            match dict.symbol(PKeyRef::ComponentByName(name)) {
                Some(x) => *x,
                None => {
                    let iid = dict.data_types.len() as u32;
                    let data = ComponentData {
                        id: 0,
                        component_type: ComponentType::Block,
                        layout_items_iid_range: 0..0,
                        name: name.to_string(),
                        category_iid: 0, // FIXME
                        abbr_name: None,
                    };
                    dict.components.push(data);
                    dict.symbol_table
                        .insert(PKey::ComponentByName(name.to_string()), iid);
                    iid
                }
            }
        }
    }

    impl FieldData {
        fn definition_from_node(dict: &mut Dictionary, node: roxmltree::Node) -> Self {
            debug_assert_eq!(node.tag_name().name(), "field");
            let data_type_iid = DatatypeData::get_or_create_iid_from_ref(dict, node);
            FieldData {
                name: node.attribute("name").unwrap().to_string(),
                tag: node.attribute("number").unwrap().parse().unwrap(),
                data_type_iid: data_type_iid,
                associated_data_tag: None,
                required: true,
                abbr_name: None,
                base_category_abbr_name: None,
                base_category_id: None,
                description: None,
            }
        }
    }

    impl DatatypeData {
        fn get_or_create_iid_from_ref(dict: &mut Dictionary, node: roxmltree::Node) -> InternalId {
            // References should only happen at <field> tags.
            debug_assert_eq!(node.tag_name().name(), "field");
            let name = node.attribute("type").unwrap();
            match dict.symbol(PKeyRef::DatatypeByName(name)) {
                Some(x) => *x,
                None => {
                    let iid = dict.data_types.len() as u32;
                    let data = DatatypeData {
                        name: name.to_string(),
                        description: String::new(),
                        examples: Vec::new(),
                        base_type: None,
                    };
                    dict.data_types.push(data);
                    dict.symbol_table
                        .insert(PKey::DatatypeByName(name.to_string()), iid);
                    iid
                }
            }
        }
    }

    impl LayoutItemData {
        fn save_definition(dict: &mut Dictionary, node: roxmltree::Node) -> Self {
            // This processing step requires on fields being already present in
            // the dictionary.
            debug_assert_ne!(dict.fields.len(), 0);
            let name = node.attribute("name").unwrap();
            let required = node.attribute("required").unwrap() == "Y";
            let tag = node.tag_name().name();
            let kind = match tag {
                "field" => {
                    let field_iid = dict.symbol(PKeyRef::FieldByName(name)).unwrap();
                    LayoutItemKindData::Field(*field_iid)
                }
                "component" => {
                    // Components may *not* be already present.
                    let component_iid = ComponentData::get_or_create_iid_from_ref(dict, node);
                    LayoutItemKindData::Component(component_iid)
                }
                "group" => {
                    let start_range = dict.layout_items.len() as u32;
                    let items = node
                        .children()
                        .filter(|n| n.is_element())
                        .map(|child| LayoutItemData::save_definition(dict, child))
                        .count();
                    LayoutItemKindData::Group(start_range..(start_range + items as u32))
                }
                _ => {
                    panic!("Invalid tag!")
                }
            };
            LayoutItemData { required, kind }
        }
    }

    impl MessageData {
        fn definition_from_node(dict: &mut Dictionary, node: roxmltree::Node) -> Self {
            debug_assert_eq!(node.tag_name().name(), "message");
            let category_iid = CategoryData::get_or_create_iid_from_ref(dict, node);
            let layout_start = dict.layout_items.len() as u32;
            for child in node.children() {
                if child.is_element() {
                    // We don't need IID's because we're dealing with ranges.
                    let data = LayoutItemData::save_definition(dict, child);
                    dict.layout_items.push(data);
                }
            }
            let layout_end = dict.layout_items.len() as u32;
            MessageData {
                name: node.attribute("name").unwrap().to_string(),
                msg_type: node.attribute("msgtype").unwrap().to_string(),
                component_id: 0,
                category_iid,
                section_id: String::new(),
                layout_items: layout_start..layout_end,
                abbr_name: None,
                required: true,
                elaboration: None,
                description: String::new(),
            }
        }
    }

    impl CategoryData {
        fn get_or_create_iid_from_ref(dict: &mut Dictionary, node: roxmltree::Node) -> InternalId {
            debug_assert_eq!(node.tag_name().name(), "message");
            let name = node.attribute("msgcat").unwrap();
            match dict.symbol(PKeyRef::CategoryByName(name)) {
                Some(x) => *x,
                None => {
                    let iid = dict.categories.len() as u32;
                    dict.categories.push(CategoryData {
                        name: name.to_string(),
                        fixml_filename: String::new(),
                    });
                    dict.symbol_table
                        .insert(PKey::CategoryByName(name.to_string()), iid);
                    iid
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum ParseDictionaryError {
        InvalidFormat,
        InvalidData(String),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::Version;

    #[test]
    pub fn fixt11_quickfix_is_ok() {
        let dict = Dictionary::from_version(Version::Fixt11);
        let msg_heartbeat = dict.get_message_by_name("Heartbeat").unwrap();
        assert_eq!(msg_heartbeat.msg_type(), "0");
        assert_eq!(msg_heartbeat.name(), "Heartbeat".to_string());
        assert!(msg_heartbeat.layout().any(|c| {
            if let LayoutItemKind::Field(f) = c.kind() {
                f.name() == "TestReqID"
            } else {
                false
            }
        }));
    }

    #[test]
    fn dictionary_save_definition_spec_is_ok() {
        for version in Version::all() {
            Dictionary::from_version(version);
        }
    }
}
