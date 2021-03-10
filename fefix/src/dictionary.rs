//! Access to FIX Dictionary reference and message specifications.

use crate::backend::Version;
use crate::DataType;
use quickfix::{ParseDictionaryError, QuickFixReader};
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Range;

/// Value for the field `MsgType (35)`.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MsgType(u16);

impl MsgType {
    pub fn write(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let bytes = self.0.to_be_bytes();
        for byte in bytes.iter() {
            writer.write(&[*byte])?;
        }
        Ok(())
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.is_empty() || bytes.len() >= 3 {
            None
        } else {
            let mut value: u16 = 0;
            for byte in bytes {
                value = (value << 8) + (*byte as u16);
            }
            Some(Self(value))
        }
    }
}

type InternalId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Key {
    #[allow(dead_code)]
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
enum KeyRef<'a> {
    Abbreviation(&'a str),
    CategoryByName(&'a str),
    ComponentByName(&'a str),
    DatatypeByName(&'a str),
    FieldByTag(u32),
    FieldByName(&'a str),
    MessageByName(&'a str),
    MessageByMsgType(MsgType),
}

impl Key {
    fn as_ref(&self) -> KeyRef {
        match self {
            Key::Abbreviation(s) => KeyRef::Abbreviation(s.as_str()),
            Key::CategoryByName(s) => KeyRef::CategoryByName(s.as_str()),
            Key::ComponentByName(s) => KeyRef::ComponentByName(s.as_str()),
            Key::DatatypeByName(s) => KeyRef::DatatypeByName(s.as_str()),
            Key::FieldByTag(t) => KeyRef::FieldByTag(*t),
            Key::FieldByName(s) => KeyRef::FieldByName(s.as_str()),
            Key::MessageByName(s) => KeyRef::MessageByName(s.as_str()),
            Key::MessageByMsgType(t) => KeyRef::MessageByMsgType(*t),
        }
    }
}

trait SymbolTableIndex {
    fn to_key(&self) -> KeyRef;
}

impl SymbolTableIndex for Key {
    fn to_key(&self) -> KeyRef {
        self.as_ref()
    }
}

impl<'a> SymbolTableIndex for KeyRef<'a> {
    fn to_key(&self) -> KeyRef {
        *self
    }
}

impl<'a> std::hash::Hash for dyn SymbolTableIndex + 'a {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.to_key().hash(state);
    }
}

impl<'a> std::borrow::Borrow<dyn SymbolTableIndex + 'a> for Key {
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

/// Specifies business semantics for application-level entities within the FIX
/// Protocol.
///
/// [`Dictionary`] doesn't provide information regarding
/// the detailed layouts of messages and components; such intricacies are handled
/// by the
/// presentation layer ([`fefix::codec`]).
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
    symbol_table: HashMap<Key, InternalId>,
    abbreviations: Vec<AbbreviatonData>,
    data_types: Vec<DatatypeData>,
    fields: Vec<FieldData>,
    components: Vec<ComponentData>,
    messages: Vec<MessageData>,
    layout_items: Vec<LayoutItemData>,
    categories: Vec<CategoryData>,
    header: Vec<FieldData>,
}

fn display_layout_item(indent: u32, item: LayoutItem, f: &mut fmt::Formatter) -> fmt::Result {
    for _ in 0..indent {
        write!(f, " ")?;
    }
    match item.kind() {
        LayoutItemKind::Field(_) => {
            writeln!(
                f,
                "<field name='{}' required='{}' />",
                item.tag_text(),
                item.required(),
            )?;
        }
        LayoutItemKind::Group => {
            writeln!(
                f,
                "<group name='{}' required='{}' />",
                item.tag_text(),
                item.required(),
            )?;
            writeln!(f, "</group>")?;
        }
        LayoutItemKind::Component(_c) => {
            writeln!(
                f,
                "<component name='{}' required='{}' />",
                item.tag_text(),
                item.required(),
            )?;
            writeln!(f, "</component>")?;
        }
    }
    Ok(())
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<fix type='FIX' version='{}'>", self.version)?;
        {
            writeln!(f, " <header>")?;
            let std_header = self.component_by_name("StandardHeader").unwrap();
            for item in std_header.items() {
                display_layout_item(2, item, f)?;
            }
            writeln!(f, " </header>")?;
        }
        {
            writeln!(f, " <messages>")?;
            for message in self.iter_messages() {
                writeln!(
                    f,
                    "  <message name='{}' msgtype='{}' msgcat='{}'>",
                    message.name(),
                    message.msg_type(),
                    "TODO"
                )?;
                for item in message.layout() {
                    display_layout_item(2, item, f)?;
                }
                writeln!(f, "  </message>")?;
            }
            writeln!(f, " </messages>")?;
        }
        {
            writeln!(f, " <header>")?;
            let std_header = self.component_by_name("StandardTrailer").unwrap();
            for item in std_header.items() {
                display_layout_item(2, item, f)?;
            }
            writeln!(f, " </header>")?;
        }
        Ok(())
    }
}

impl Dictionary {
    /// Creates a new empty FIX Dictionary named `version`.
    fn new<S: ToString>(version: S) -> Self {
        Dictionary {
            version: version.to_string(),
            symbol_table: HashMap::new(),
            abbreviations: Vec::new(),
            data_types: Vec::new(),
            fields: Vec::new(),
            components: Vec::new(),
            messages: Vec::new(),
            layout_items: Vec::new(),
            categories: Vec::new(),
            header: Vec::new(),
        }
    }

    /// Creates a new [`Dictionary`](Dictionary) according to the specification of
    /// `version`.
    pub fn from_version(version: Version) -> Self {
        Dictionary::save_definition_spec(version.get_quickfix_spec()).unwrap()
    }

    /// Creates a new empty FIX Dictionary with `FIX.???` as its version string.
    pub fn empty() -> Self {
        Self::new("FIX.???")
    }

    /// Returns the version string associated with this [`Dictionary`] (e.g.
    /// `FIXT.1.1`, `FIX.4.2`).
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix44);
    /// assert_eq!(dict.get_version(), "FIX.4.4");
    /// ```
    pub fn get_version(&self) -> &str {
        self.version.as_str()
    }

    fn symbol(&self, pkey: KeyRef) -> Option<&u32> {
        self.symbol_table.get(&pkey as &dyn SymbolTableIndex)
    }

    /// Return the known abbreviation for `term` -if any- according to the
    /// documentation of this FIX Dictionary.
    pub fn abbreviation_for<S: AsRef<str>>(&self, term: S) -> Option<Abbreviation> {
        self.symbol(KeyRef::Abbreviation(term.as_ref()))
            .map(|iid| self.abbreviations.get(*iid as usize).unwrap())
            .map(move |data| Abbreviation(self, data))
    }

    /// Returns the [`Message`](Message) associated with `name`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix44);
    ///
    /// let msg1 = dict.message_by_name("Heartbeat").unwrap();
    /// let msg2 = dict.message_by_msgtype("0").unwrap();
    /// assert_eq!(msg1.name(), msg2.name());
    /// ```
    pub fn message_by_name<S: AsRef<str>>(&self, name: S) -> Option<Message> {
        self.symbol(KeyRef::MessageByName(name.as_ref()))
            .map(|iid| self.messages.get(*iid as usize).unwrap())
            .map(|data| Message(self, data))
    }

    /// Returns the [`Message`](Message) that has the given `msgtype`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix44);
    ///
    /// let msg1 = dict.message_by_msgtype("0").unwrap();
    /// let msg2 = dict.message_by_name("Heartbeat").unwrap();
    /// assert_eq!(msg1.name(), msg2.name());
    /// ```
    pub fn message_by_msgtype<S: AsRef<str>>(&self, msgtype: S) -> Option<Message> {
        self.symbol(KeyRef::MessageByMsgType(MsgType::from_bytes(
            msgtype.as_ref().as_bytes(),
        )?))
        .map(|iid| self.messages.get(*iid as usize).unwrap())
        .map(|data| Message(self, data))
    }

    /// Returns the [`Component`] named `name`, if any.
    pub fn component_by_name<S: AsRef<str>>(&self, name: S) -> Option<Component> {
        self.symbol(KeyRef::ComponentByName(name.as_ref()))
            .map(|iid| self.components.get(*iid as usize).unwrap())
            .map(|data| Component(self, data))
    }

    /// Attempts to read a QuickFIX-style specification file and convert it into
    /// a [`Dictionary`].
    pub fn save_definition_spec<S: AsRef<str>>(input: S) -> Result<Self, ParseDictionaryError> {
        let xml_document = roxmltree::Document::parse(input.as_ref()).unwrap();
        QuickFixReader::new(&xml_document)
    }

    /// Returns the [`DataType`](DataType) named `name`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix44);
    ///
    /// let dt = dict.datatype_by_name("String").unwrap();
    /// assert_eq!(dt.name(), "String");
    /// ```
    pub fn datatype_by_name<S: AsRef<str>>(&self, name: S) -> Option<Datatype> {
        self.symbol(KeyRef::DatatypeByName(name.as_ref()))
            .map(|iid| self.data_types.get(*iid as usize).unwrap())
            .map(|data| Datatype(self, data))
    }

    /// Returns the [`Field`](Field) associated with `tag`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix44);
    ///
    /// let field1 = dict.field_by_tag(112).unwrap();
    /// let field2 = dict.field_by_name("TestReqID").unwrap();
    /// assert_eq!(field1.name(), field2.name());
    /// ```
    pub fn field_by_tag(&self, tag: u32) -> Option<Field> {
        self.symbol(KeyRef::FieldByTag(tag))
            .map(|iid| self.fields.get(*iid as usize).unwrap())
            .map(|data| Field(self, data))
    }

    /// Returns the [`Field`] named `name`, if any.
    pub fn field_by_name<S: AsRef<str>>(&self, name: S) -> Option<Field> {
        self.symbol(KeyRef::FieldByName(name.as_ref()))
            .map(|iid| self.fields.get(*iid as usize).unwrap())
            .map(|data| Field(self, data))
    }

    /// Returns an [`Iterator`](Iterator) over all [`DataType`](DataType) defined
    /// in `self`. Items are in no particular order.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix42);
    /// // FIX 4.2 defines 19 datatypes.
    /// assert_eq!(dict.iter_datatypes().count(), 19);
    /// ```
    pub fn iter_datatypes(&self) -> impl Iterator<Item = Datatype> {
        self.data_types.iter().map(move |data| Datatype(self, data))
    }

    /// Returns an [`Iterator`](Iterator) over this [`Dictionary`]'s messages. Items are in
    /// no particular order.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::backend::Version;
    ///
    /// let dict = Dictionary::from_version(Version::Fix44);
    /// let msg = dict.iter_messages().find(|m| m.name() == "MarketDataRequest");
    /// assert_eq!(msg.unwrap().msg_type(), "V");
    /// ```
    pub fn iter_messages(&self) -> impl Iterator<Item = Message> {
        self.messages.iter().map(move |data| Message(&self, data))
    }

    /// Returns an [`Iterator`] over this [`Dictionary`]'s categories. Items are
    /// in no particular order.
    pub fn iter_categories(&self) -> impl Iterator<Item = Category> {
        self.categories
            .iter()
            .map(move |data| Category(&self, data))
    }

    /// Returns an [`Iterator`] over this [`Dictionary`]'s fields. Items are
    /// in no particular order.
    pub fn iter_fields(&self) -> impl Iterator<Item = Field> {
        self.fields.iter().map(move |data| Field(&self, data))
    }

    /// Returns an [`Iterator`] over this [`Dictionary`]'s components. Items are in
    /// no particular order.
    pub fn iter_components(&self) -> impl Iterator<Item = Component> {
        self.components
            .iter()
            .map(move |data| Component(&self, data))
    }
}

#[derive(Clone, Debug)]
struct CategoryData {
    /// **Primary key**. A string uniquely identifying this category.
    name: String,
    /// The FIXML file name for a Category.
    fixml_filename: String,
}

/// A [`Category`] is a collection of loosely related FIX messages or components
/// all belonging to the same [`Section`].
#[derive(Clone, Debug)]
pub struct Category<'a>(&'a Dictionary, &'a CategoryData);

#[derive(Clone, Debug)]
struct AbbreviatonData {
    abbreviation: String,
    is_last: bool,
}

/// An [`Abbreviation`] is a standardized abbreviated form for a specific word,
/// pattern, or name. Abbreviation data is mostly meant for documentation
/// purposes, but in general it can have other uses as well, e.g. FIXML field
/// naming.
#[derive(Debug)]
pub struct Abbreviation<'a>(&'a Dictionary, &'a AbbreviatonData);

impl<'a> Abbreviation<'a> {
    /// Returns the full term (non-abbreviated) associated with `self`.
    pub fn term(&self) -> &str {
        self.1.abbreviation.as_str()
    }
}

#[derive(Clone, Debug)]
struct ComponentData {
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

/// A [`Component`] is an ordered collection of fields and/or other components.
/// There are two kinds of components: (1) common blocks and (2) repeating
/// groups. Common blocks are merely commonly reused sequences of the same
/// fields/components
/// which are given names for simplicity, i.e. they serve as "macros". Repeating
/// groups, on the other hand, are components which can appear zero or more times
/// inside FIX messages (or other components, for that matter).
#[derive(Clone, Debug)]
pub struct Component<'a>(&'a Dictionary, &'a ComponentData);

impl<'a> Component<'a> {
    /// Returns the unique numberic ID of `self`.
    pub fn id(&self) -> u32 {
        self.1.id as u32
    }

    /// Returns the name of `self`. The name of every [`Component`] is unique
    /// across a [`Dictionary`].
    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    /// Returns the [`Category`] to which `self` belongs.
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

    /// Checks whether `field` appears in the definition of `self` and returns
    /// `true` if it does, `false` otherwise.
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

// FIXME: this is FIXML-specific stuff.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
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
    datatype: DataType,
    /// Human readable description of this Datatype.
    description: String,
    /// A string that contains examples values for a datatype
    examples: Vec<String>,
    // TODO: 'XML'.
}

#[derive(Debug)]
pub struct Datatype<'a>(&'a Dictionary, &'a DatatypeData);

impl<'a> Datatype<'a> {
    pub fn name(&self) -> &str {
        self.1.datatype.name()
    }

    pub fn basetype(&self) -> DataType {
        self.1.datatype
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enum {}

/// A field is identified by a unique tag number and a name. Each field in a
/// message is associated with a value.
#[derive(Clone, Debug)]
struct FieldData {
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
    value_restrictions: Option<Vec<FieldEnumData>>,
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

#[derive(Clone, Debug)]
struct FieldEnumData {
    value: String,
    description: String,
}

#[derive(Debug)]
pub struct FieldEnum<'a>(&'a Dictionary, &'a FieldEnumData);

impl<'a> FieldEnum<'a> {
    pub fn value(&self) -> &str {
        &self.1.value[..]
    }

    pub fn description(&self) -> &str {
        &self.1.description[..]
    }
}

/// A field is the most granular message structure abstraction. It carries a
/// specific business meaning as described by the FIX specifications. The data
/// domain of a [`Field`] is either a [`Datatype`] or a "code set", i.e.
/// enumeration.
#[derive(Debug)]
pub struct Field<'a>(&'a Dictionary, &'a FieldData);

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

    /// Returns the [`BaseType`] of `self`.
    pub fn basetype(&self) -> DataType {
        self.data_type().basetype()
    }

    /// Returns the name of `self`. Field names are unique across each FIX
    /// [`Dictionary`].
    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    /// Returns the numeric tag of `self`. Field tags are unique across each FIX
    /// [`Dictionary`].
    pub fn tag(&self) -> u32 {
        self.1.tag
    }

    pub fn enums(&self) -> Option<impl Iterator<Item = FieldEnum>> {
        self.1
            .value_restrictions
            .as_ref()
            .map(move |v| v.iter().map(move |f| FieldEnum(self.0, f)))
    }

    /// Returns the [`Datatype`] of `self`.
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

/// The kind of element contained in a [`Message`].
#[derive(Debug)]
pub enum LayoutItemKind<'a> {
    Component(Component<'a>),
    Group,
    Field(Field<'a>),
}

impl<'a> LayoutItem<'a> {
    /// Returns `true` if `self` is required in order to have a valid definition
    /// of its parent container, `false` otherwise.
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
                LayoutItemKind::Group // FIXME
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
struct MessageData {
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

/// A [`Message`] is a unit of information sent on the wire between
/// counterparties. Every [`Message`] is composed of fields and/or components.
#[derive(Debug)]
pub struct Message<'a>(&'a Dictionary, &'a MessageData);

impl<'a> Message<'a> {
    /// Returns the human-readable name of `self`.
    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    /// Returns the message type of `self`.
    pub fn msg_type(&self) -> &str {
        self.1.msg_type.as_str()
    }

    /// Returns the description associated with `self`.
    pub fn description(&self) -> &str {
        &self.1.description
    }

    /// Returns the component ID of `self`.
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

/// A [`Section`] is a collection of many [`Components`]-s. It has no practical
/// effect on encoding and decoding of FIX data and it's only used for
/// documentation and human readability.
#[derive(Clone, Debug, PartialEq)]
pub struct Section {}

#[derive(Clone, Debug, PartialEq)]
pub struct Value {
    value_enum: String,
    description: Option<String>,
}

mod quickfix {
    use super::*;

    pub struct QuickFixReader<'a> {
        node_with_header: roxmltree::Node<'a, 'a>,
        node_with_trailer: roxmltree::Node<'a, 'a>,
        node_with_components: roxmltree::Node<'a, 'a>,
        node_with_messages: roxmltree::Node<'a, 'a>,
        node_with_fields: roxmltree::Node<'a, 'a>,
        dict: Dictionary,
    }

    impl<'a> QuickFixReader<'a> {
        pub fn new(xml_document: &'a roxmltree::Document<'a>) -> ParseResult<Dictionary> {
            let mut reader = Self::empty(&xml_document)?;
            for child in reader.node_with_fields.children() {
                if child.is_element() {
                    reader.add_field(child)?;
                }
            }
            for child in reader.node_with_components.children() {
                if child.is_element() {
                    reader.add_component(child)?;
                }
            }
            for child in reader.node_with_messages.children() {
                if child.is_element() {
                    reader.add_message(child)?;
                }
            }
            // `StandardHeader` and `StandardTrailer` are defined in ad-hoc
            // sections of the XML files. They're always there, even if
            // potentially empty (FIX 5.0+).
            reader.add_component_with_name(reader.node_with_header, "StandardHeader")?;
            reader.add_component_with_name(reader.node_with_trailer, "StandardTrailer")?;
            Ok(reader.dict)
        }

        fn empty(xml_document: &'a roxmltree::Document<'a>) -> ParseResult<Self> {
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

        /// Reads a [`FieldData`](FieldData) definition from `node` and
        /// updates `self`.
        fn add_field(&mut self, node: roxmltree::Node) -> ParseResult<()> {
            let iid = self.dict.fields.len() as u32;
            let field = self.import_field(node)?;
            self.dict
                .symbol_table
                .insert(Key::FieldByName(field.name.clone()), iid);
            self.dict
                .symbol_table
                .insert(Key::FieldByTag(field.tag as u32), iid);
            self.dict.fields.push(field);
            Ok(())
        }

        /// Reads a [`MessageData`](MessageData) definition from `node` and
        /// updates `self`.
        fn add_message(&mut self, node: roxmltree::Node) -> ParseResult<()> {
            let iid = self.dict.messages.len() as u32;
            let message = self.import_message(node)?;
            self.dict
                .symbol_table
                .insert(Key::MessageByName(message.name.clone()), iid);
            self.dict.symbol_table.insert(
                Key::MessageByMsgType(MsgType::from_bytes(message.msg_type.as_bytes()).unwrap()),
                iid,
            );
            self.dict.messages.push(message);
            Ok(())
        }

        /// Reads a [`ComponentData`](ComponentData) definition from `node` and
        /// updates `self`.
        fn add_component(&mut self, node: roxmltree::Node) -> ParseResult<()> {
            let name = node
                .attribute("name")
                .ok_or(ParseDictionaryError::InvalidFormat)?
                .to_string();
            self.add_component_with_name(node, name)?;
            Ok(())
        }

        fn add_component_with_name<S: AsRef<str>>(
            &mut self,
            node: roxmltree::Node,
            name: S,
        ) -> ParseResult<()> {
            let iid = self.dict.components.len();
            let component =
                ComponentData::definition_from_node_with_name(&mut self.dict, node, name.as_ref())?;
            self.dict
                .symbol_table
                .insert(Key::ComponentByName(name.as_ref().to_string()), iid as u32);
            self.dict.components.push(component);
            Ok(())
        }

        fn import_message(&mut self, node: roxmltree::Node) -> ParseResult<MessageData> {
            debug_assert_eq!(node.tag_name().name(), "message");
            let category_iid = CategoryData::get_or_create_iid_from_ref(&mut self.dict, node)?;
            let layout_start = self.dict.layout_items.len() as u32;
            for child in node.children() {
                if child.is_element() {
                    // We don't need to generate new IID's because we're dealing
                    // with ranges.
                    let data = LayoutItemData::save_definition(&mut self.dict, child)?;
                    self.dict.layout_items.push(data);
                }
            }
            let layout_end = self.dict.layout_items.len() as u32;
            Ok(MessageData {
                name: node
                    .attribute("name")
                    .ok_or(ParseDictionaryError::InvalidFormat)?
                    .to_string(),
                msg_type: node
                    .attribute("msgtype")
                    .ok_or(ParseDictionaryError::InvalidFormat)?
                    .to_string(),
                component_id: 0,
                category_iid,
                section_id: String::new(),
                layout_items: layout_start..layout_end,
                abbr_name: None,
                required: true,
                elaboration: None,
                description: String::new(),
            })
        }

        fn import_field(&mut self, node: roxmltree::Node) -> ParseResult<FieldData> {
            debug_assert_eq!(node.tag_name().name(), "field");
            let data_type_iid = DatatypeData::get_or_create_iid_from_ref(&mut self.dict, node);
            let value_restrictions = value_restrictions_from_node(node, data_type_iid);
            Ok(FieldData {
                name: node
                    .attribute("name")
                    .ok_or(ParseDictionaryError::InvalidFormat)?
                    .to_string(),
                tag: node
                    .attribute("number")
                    .ok_or(ParseDictionaryError::InvalidFormat)?
                    .parse()
                    .map_err(|_| ParseDictionaryError::InvalidFormat)?,
                data_type_iid: data_type_iid,
                associated_data_tag: None,
                value_restrictions,
                required: true,
                abbr_name: None,
                base_category_abbr_name: None,
                base_category_id: None,
                description: None,
            })
        }
    }

    impl ComponentData {
        fn definition_from_node_with_name<S: AsRef<str>>(
            dict: &mut Dictionary,
            node: roxmltree::Node,
            name: S,
        ) -> ParseResult<Self> {
            let layout_start = dict.layout_items.len() as u32;
            for child in node.children() {
                if child.is_element() {
                    // We don't need IID's because we're dealing with ranges.
                    let item = LayoutItemData::save_definition(dict, child)?;
                    dict.layout_items.push(item);
                }
            }
            let layout_end = dict.layout_items.len() as u32;
            Ok(ComponentData {
                id: 0,
                component_type: ComponentType::Block,
                layout_items_iid_range: layout_start..layout_end,
                category_iid: 0, // FIXME
                name: name.as_ref().to_string(),
                abbr_name: None,
            })
        }

        fn get_or_create_iid_from_ref(
            dict: &mut Dictionary,
            node: roxmltree::Node,
        ) -> ParseResult<InternalId> {
            debug_assert_eq!(node.tag_name().name(), "component");
            let name = node
                .attribute("name")
                .ok_or(ParseDictionaryError::InvalidFormat)?;
            Ok(match dict.symbol(KeyRef::ComponentByName(name)) {
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
                        .insert(Key::ComponentByName(name.to_string()), iid);
                    iid
                }
            })
        }
    }

    fn value_restrictions_from_node(
        node: roxmltree::Node,
        _datatype: InternalId,
    ) -> Option<Vec<FieldEnumData>> {
        let mut values = Vec::new();
        for child in node.children() {
            if child.is_element() {
                let variant = child.attribute("enum").unwrap().to_string();
                let description = child.attribute("description").unwrap().to_string();
                let enum_value = FieldEnumData {
                    value: variant,
                    description,
                };
                values.push(enum_value);
            }
        }
        if values.len() == 0 {
            None
        } else {
            Some(values)
        }
    }

    impl DatatypeData {
        fn get_or_create_iid_from_ref(dict: &mut Dictionary, node: roxmltree::Node) -> InternalId {
            // References should only happen at <field> tags.
            debug_assert_eq!(node.tag_name().name(), "field");
            let datatype = {
                // The idenfier that QuickFIX uses for this type.
                let quickfix_name = node.attribute("type").unwrap();
                // Translate that into a real datatype.
                DataType::from_quickfix_name(quickfix_name).unwrap()
            };
            // Get the official (not QuickFIX's) name of `datatype`.
            let name = datatype.name();
            match dict.symbol(KeyRef::DatatypeByName(name)) {
                Some(x) => *x,
                None => {
                    let iid = dict.data_types.len() as u32;
                    let data = DatatypeData {
                        datatype,
                        description: String::new(),
                        examples: Vec::new(),
                    };
                    dict.data_types.push(data);
                    dict.symbol_table
                        .insert(Key::DatatypeByName(name.to_string()), iid);
                    iid
                }
            }
        }
    }

    impl LayoutItemData {
        fn save_definition(dict: &mut Dictionary, node: roxmltree::Node) -> ParseResult<Self> {
            // This processing step requires on fields being already present in
            // the dictionary.
            debug_assert_ne!(dict.fields.len(), 0);
            let name = node.attribute("name").unwrap();
            let required = node.attribute("required").unwrap() == "Y";
            let tag = node.tag_name().name();
            let kind = match tag {
                "field" => {
                    let field_iid = dict.symbol(KeyRef::FieldByName(name)).unwrap();
                    LayoutItemKindData::Field(*field_iid)
                }
                "component" => {
                    // Components may *not* be already present.
                    let component_iid = ComponentData::get_or_create_iid_from_ref(dict, node)?;
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
            Ok(LayoutItemData { required, kind })
        }
    }

    impl CategoryData {
        fn get_or_create_iid_from_ref(
            dict: &mut Dictionary,
            node: roxmltree::Node,
        ) -> ParseResult<InternalId> {
            debug_assert_eq!(node.tag_name().name(), "message");
            let name = node.attribute("msgcat").ok_or(ParseError::InvalidFormat)?;
            Ok(match dict.symbol(KeyRef::CategoryByName(name)) {
                Some(x) => *x,
                None => {
                    let iid = dict.categories.len() as u32;
                    dict.categories.push(CategoryData {
                        name: name.to_string(),
                        fixml_filename: String::new(),
                    });
                    dict.symbol_table
                        .insert(Key::CategoryByName(name.to_string()), iid);
                    iid
                }
            })
        }
    }

    type ParseError = ParseDictionaryError;
    type ParseResult<T> = Result<T, ParseError>;

    /// The error type that can arise when decoding a QuickFIX Dictionary.
    #[derive(Clone, Debug)]
    pub enum ParseDictionaryError {
        InvalidFormat,
        InvalidData(String),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::backend::Version;
    use quickcheck::QuickCheck;
    use std::collections::HashSet;
    use std::convert::TryInto;

    #[test]
    fn msg_type_conversion() {
        fn prop(val: u16) -> bool {
            let bytes = val.to_le_bytes();
            let msg_type = MsgType::from_bytes(&bytes[..]).unwrap();
            let mut buffer = vec![0, 0];
            msg_type.write(&mut &mut buffer[..]).unwrap();
            val == u16::from_le_bytes((&buffer[..]).try_into().unwrap())
        }
        QuickCheck::new()
            .tests(1000)
            .quickcheck(prop as fn(u16) -> bool)
    }

    #[test]
    fn fixt11_quickfix_is_ok() {
        let dict = Dictionary::from_version(Version::Fixt11);
        let msg_heartbeat = dict.message_by_name("Heartbeat").unwrap();
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

    #[test]
    fn all_datatypes_are_used_at_least_once() {
        for version in Version::all() {
            let dict = Dictionary::from_version(version);
            let datatypes_count = dict.iter_datatypes().count();
            let mut datatypes = HashSet::new();
            for field in dict.iter_fields() {
                datatypes.insert(field.data_type().name().to_string());
            }
            assert_eq!(datatypes_count, datatypes.len());
        }
    }

    #[test]
    fn at_least_one_datatype() {
        for version in Version::all() {
            let dict = Dictionary::from_version(version);
            assert!(dict.iter_datatypes().count() >= 1);
        }
    }

    #[test]
    fn std_header_and_trailer_always_present() {
        for version in Version::all() {
            let dict = Dictionary::from_version(version);
            let std_header = dict.component_by_name("StandardHeader");
            let std_trailer = dict.component_by_name("StandardTrailer");
            assert!(std_header.is_some() && std_trailer.is_some());
        }
    }

    #[test]
    fn fix44_field_28_has_three_variants() {
        let dict = Dictionary::from_version(Version::Fix44);
        let field_28 = dict.field_by_tag(28).unwrap();
        assert_eq!(field_28.name(), "IOITransType");
        assert_eq!(field_28.enums().unwrap().count(), 3);
    }

    #[test]
    fn fix44_field_36_has_no_variants() {
        let dict = Dictionary::from_version(Version::Fix44);
        let field_36 = dict.field_by_tag(36).unwrap();
        assert_eq!(field_36.name(), "NewSeqNo");
        assert!(field_36.enums().is_none());
    }

    #[test]
    fn fix44_field_167_has_eucorp_variant() {
        let dict = Dictionary::from_version(Version::Fix44);
        let field_167 = dict.field_by_tag(167).unwrap();
        assert_eq!(field_167.name(), "SecurityType");
        assert!(field_167.enums().unwrap().any(|e| e.value() == "EUCORP"));
    }
}
