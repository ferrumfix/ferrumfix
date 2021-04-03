//! Access to FIX Dictionary reference and message specifications.

use self::symbol_table::{Key, KeyRef, SymbolTable, SymbolTableIndex};
use crate::AppVersion;
use crate::{quickfix_spec, DataType};
use quickfix::{ParseDictionaryError, QuickFixReader};
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::ops::Range;
use std::sync::Arc;

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
    inner: Arc<DictionaryData>,
}

#[derive(Clone, Debug)]
struct DictionaryData {
    version: String,
    symbol_table: SymbolTable,
    abbreviations: Vec<AbbreviatonData>,
    data_types: Vec<DatatypeData>,
    fields: Vec<FieldData>,
    components: Vec<ComponentData>,
    messages: Vec<MessageData>,
    layout_items: Vec<LayoutItemData>,
    categories: Vec<CategoryData>,
    header: Vec<FieldData>,
}

impl fmt::Display for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<fix type='FIX' version='{}'>", self.inner.version)?;
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
        LayoutItemKind::Group(_, _fields) => {
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

impl DictionaryData {
    fn symbol(&self, pkey: KeyRef) -> Option<&u32> {
        self.symbol_table.get(&pkey as &dyn SymbolTableIndex)
    }
}

impl Dictionary {
    /// Creates a new empty FIX Dictionary named `version`.
    fn new<S: ToString>(version: S) -> Self {
        Dictionary {
            inner: Arc::new(DictionaryData {
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
            }),
        }
    }

    /// Creates a new [`Dictionary`](Dictionary) according to the specification of
    /// `version`.
    pub fn from_version(version: AppVersion) -> Self {
        let spec = quickfix_spec(version);
        Dictionary::save_definition_spec(spec).unwrap()
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
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// assert_eq!(dict.get_version(), "FIX.4.4");
    /// ```
    pub fn get_version(&self) -> &str {
        self.inner.version.as_str()
    }

    fn symbol(&self, pkey: KeyRef) -> Option<&u32> {
        self.inner.symbol(pkey)
    }

    /// Return the known abbreviation for `term` -if any- according to the
    /// documentation of this FIX Dictionary.
    pub fn abbreviation_for<S: AsRef<str>>(&self, term: S) -> Option<Abbreviation> {
        self.symbol(KeyRef::Abbreviation(term.as_ref()))
            .map(|iid| self.inner.abbreviations.get(*iid as usize).unwrap())
            .map(move |data| Abbreviation(self, data))
    }

    /// Returns the [`Message`](Message) associated with `name`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    ///
    /// let msg1 = dict.message_by_name("Heartbeat").unwrap();
    /// let msg2 = dict.message_by_msgtype("0").unwrap();
    /// assert_eq!(msg1.name(), msg2.name());
    /// ```
    pub fn message_by_name<S: AsRef<str>>(&self, name: S) -> Option<Message> {
        self.symbol(KeyRef::MessageByName(name.as_ref()))
            .map(|iid| self.inner.messages.get(*iid as usize).unwrap())
            .map(|data| Message(self, data))
    }

    /// Returns the [`Message`](Message) that has the given `msgtype`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    ///
    /// let msg1 = dict.message_by_msgtype("0").unwrap();
    /// let msg2 = dict.message_by_name("Heartbeat").unwrap();
    /// assert_eq!(msg1.name(), msg2.name());
    /// ```
    pub fn message_by_msgtype<S: AsRef<str>>(&self, msgtype: S) -> Option<Message> {
        self.symbol(KeyRef::MessageByMsgType(MsgType::from_bytes(
            msgtype.as_ref().as_bytes(),
        )?))
        .map(|iid| self.inner.messages.get(*iid as usize).unwrap())
        .map(|data| Message(self, data))
    }

    /// Returns the [`Component`] named `name`, if any.
    pub fn component_by_name<S: AsRef<str>>(&self, name: S) -> Option<Component> {
        self.symbol(KeyRef::ComponentByName(name.as_ref()))
            .map(|iid| self.inner.components.get(*iid as usize).unwrap())
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
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    ///
    /// let dt = dict.datatype_by_name("String").unwrap();
    /// assert_eq!(dt.name(), "String");
    /// ```
    pub fn datatype_by_name<S: AsRef<str>>(&self, name: S) -> Option<Datatype> {
        self.symbol(KeyRef::DatatypeByName(name.as_ref()))
            .map(|iid| self.inner.data_types.get(*iid as usize).unwrap())
            .map(|data| Datatype(self, data))
    }

    /// Returns the [`Field`](Field) associated with `tag`, if any.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    ///
    /// let field1 = dict.field_by_tag(112).unwrap();
    /// let field2 = dict.field_by_name("TestReqID").unwrap();
    /// assert_eq!(field1.name(), field2.name());
    /// ```
    pub fn field_by_tag(&self, tag: u32) -> Option<Field> {
        self.symbol(KeyRef::FieldByTag(tag))
            .map(|iid| self.inner.fields.get(*iid as usize).unwrap())
            .map(|data| Field(self, data))
    }

    /// Returns the [`Field`] named `name`, if any.
    pub fn field_by_name<S: AsRef<str>>(&self, name: S) -> Option<Field> {
        self.symbol(KeyRef::FieldByName(name.as_ref()))
            .map(|iid| self.inner.fields.get(*iid as usize).unwrap())
            .map(|data| Field(self, data))
    }

    /// Returns an [`Iterator`](Iterator) over all [`DataType`](DataType) defined
    /// in `self`. Items are in no particular order.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix42);
    /// // FIX 4.2 defines 19 datatypes.
    /// assert_eq!(dict.iter_datatypes().count(), 19);
    /// ```
    pub fn iter_datatypes(&self) -> impl Iterator<Item = Datatype> {
        self.inner
            .data_types
            .iter()
            .map(move |data| Datatype(self, data))
    }

    /// Returns an [`Iterator`](Iterator) over this [`Dictionary`]'s messages. Items are in
    /// no particular order.
    ///
    /// ```
    /// use fefix::Dictionary;
    /// use fefix::AppVersion;
    ///
    /// let dict = Dictionary::from_version(AppVersion::Fix44);
    /// let msg = dict.iter_messages().find(|m| m.name() == "MarketDataRequest");
    /// assert_eq!(msg.unwrap().msg_type(), "V");
    /// ```
    pub fn iter_messages(&self) -> impl Iterator<Item = Message> {
        self.inner
            .messages
            .iter()
            .map(move |data| Message(&self, data))
    }

    /// Returns an [`Iterator`] over this [`Dictionary`]'s categories. Items are
    /// in no particular order.
    pub fn iter_categories(&self) -> impl Iterator<Item = Category> {
        self.inner
            .categories
            .iter()
            .map(move |data| Category(&self, data))
    }

    /// Returns an [`Iterator`] over this [`Dictionary`]'s fields. Items are
    /// in no particular order.
    pub fn iter_fields(&self) -> impl Iterator<Item = Field> {
        self.inner.fields.iter().map(move |data| Field(&self, data))
    }

    /// Returns an [`Iterator`] over this [`Dictionary`]'s components. Items are in
    /// no particular order.
    pub fn iter_components(&self) -> impl Iterator<Item = Component> {
        self.inner
            .components
            .iter()
            .map(move |data| Component(&self, data))
    }

    pub fn to_quickfix_xml(&self) -> String {
        quickfix::to_quickfix_xml(self)
    }
}

struct DictionaryBuilder {
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

impl DictionaryBuilder {
    pub fn new(version: String) -> Self {
        Self {
            version,
            symbol_table: HashMap::default(),
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

    pub fn symbol(&self, pkey: KeyRef) -> Option<&InternalId> {
        self.symbol_table.get(&pkey as &dyn SymbolTableIndex)
    }

    pub fn add_field(&mut self, field: FieldData) -> InternalId {
        let iid = self.fields.len() as InternalId;
        self.symbol_table
            .insert(Key::FieldByName(field.name.clone()), iid);
        self.symbol_table
            .insert(Key::FieldByTag(field.tag as u32), iid);
        self.fields.push(field);
        iid
    }

    pub fn add_message(&mut self, message: MessageData) -> InternalId {
        let iid = self.messages.len() as InternalId;
        self.symbol_table
            .insert(Key::MessageByName(message.name.clone()), iid);
        self.symbol_table.insert(
            Key::MessageByMsgType(MsgType::from_bytes(message.msg_type.as_bytes()).unwrap()),
            iid,
        );
        self.messages.push(message);
        iid
    }

    pub fn add_component(&mut self, component: ComponentData) -> InternalId {
        let iid = self.components.len() as InternalId;
        self.symbol_table
            .insert(Key::ComponentByName(component.name.to_string()), iid);
        self.components.push(component);
        iid
    }

    pub fn build(self) -> Dictionary {
        Dictionary {
            inner: Arc::new(DictionaryData {
                version: self.version,
                symbol_table: self.symbol_table,
                abbreviations: self.abbreviations,
                data_types: self.data_types,
                fields: self.fields,
                components: self.components,
                messages: self.messages,
                layout_items: self.layout_items,
                categories: self.categories,
                header: self.header,
            }),
        }
    }
}

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

    /// Returns `true` if and only if `self` is a "group" component; `false`
    /// otherwise.
    pub fn is_group(&self) -> bool {
        match self.1.component_type {
            ComponentType::BlockRepeating => true,
            ComponentType::ImplicitBlockRepeating => true,
            ComponentType::OptimisedBlockRepeating => true,
            ComponentType::OptimisedImplicitBlockRepeating => true,
            _ => false,
        }
    }

    /// Returns the [`Category`] to which `self` belongs.
    pub fn category(&self) -> Category {
        let data = self
            .0
            .inner
            .categories
            .get(self.1.category_iid as usize)
            .unwrap();
        Category(self.0, data)
    }

    pub fn items(&self) -> impl Iterator<Item = LayoutItem> {
        let start = self.1.layout_items_iid_range.start as usize;
        let end = self.1.layout_items_iid_range.end as usize;
        self.0.inner.layout_items[start..end]
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
            .inner
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
#[allow(dead_code)]
enum LayoutItemKindData {
    Component {
        iid: InternalId,
    },
    Group {
        len_field_iid: InternalId,
        items: Vec<LayoutItemData>,
    },
    Field {
        iid: InternalId,
    },
}

#[derive(Clone, Debug)]
struct LayoutItemData {
    required: bool,
    kind: LayoutItemKindData,
}

fn layout_item_kind<'a>(item: &'a LayoutItemKindData, dict: &'a Dictionary) -> LayoutItemKind<'a> {
    match item {
        LayoutItemKindData::Component { iid } => LayoutItemKind::Component(Component(
            dict,
            dict.inner.components.get(*iid as usize).unwrap(),
        )),
        LayoutItemKindData::Group {
            len_field_iid,
            items: items_data,
        } => {
            let items = items_data
                .iter()
                .map(|item_data| LayoutItem(dict, item_data))
                .collect::<Vec<_>>();
            let len_field_data = &dict.inner.fields[*len_field_iid as usize];
            let len_field = Field(dict, len_field_data);
            LayoutItemKind::Group(len_field, items)
        }
        LayoutItemKindData::Field { iid } => {
            LayoutItemKind::Field(Field(dict, dict.inner.fields.get(*iid as usize).unwrap()))
        }
    }
}

#[derive(Clone, Debug)]
pub struct LayoutItem<'a>(&'a Dictionary, &'a LayoutItemData);

/// The kind of element contained in a [`Message`].
#[derive(Debug)]
pub enum LayoutItemKind<'a> {
    Component(Component<'a>),
    Group(Field<'a>, Vec<LayoutItem<'a>>),
    Field(Field<'a>),
}

impl<'a> LayoutItem<'a> {
    /// Returns `true` if `self` is required in order to have a valid definition
    /// of its parent container, `false` otherwise.
    pub fn required(&self) -> bool {
        self.1.required
    }

    pub fn kind(&self) -> LayoutItemKind {
        layout_item_kind(&self.1.kind, self.0)
    }

    pub fn tag_text(&self) -> &str {
        match &self.1.kind {
            LayoutItemKindData::Component { iid } => self
                .0
                .inner
                .components
                .get(*iid as usize)
                .unwrap()
                .name
                .as_str(),
            LayoutItemKindData::Group {
                len_field_iid,
                items: _items,
            } => self
                .0
                .inner
                .fields
                .get(*len_field_iid as usize)
                .unwrap()
                .name
                .as_str(),
            LayoutItemKindData::Field { iid } => self
                .0
                .inner
                .fields
                .get(*iid as usize)
                .unwrap()
                .name
                .as_str(),
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
        self.0.inner.layout_items[start..end]
            .iter()
            .map(move |data| LayoutItem(self.0, data))
    }
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

mod symbol_table {
    use super::InternalId;
    use super::MsgType;
    use std::borrow::Borrow;
    use std::collections::HashMap;
    use std::hash::Hash;

    pub type SymbolTable = HashMap<Key, InternalId>;

    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub enum Key {
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
    pub enum KeyRef<'a> {
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

    pub trait SymbolTableIndex {
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

    impl<'a> Hash for dyn SymbolTableIndex + 'a {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.to_key().hash(state);
        }
    }

    impl<'a> Borrow<dyn SymbolTableIndex + 'a> for Key {
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
}

mod quickfix {
    use super::*;

    fn write_layout_item<T>(writer: &mut quick_xml::Writer<T>, item: LayoutItem)
    where
        T: io::Write,
    {
        use quick_xml::events::*;
        let required = if item.required() { 'Y' } else { 'N' };
        match item.kind() {
            LayoutItemKind::Field(_) => {
                writer
                    .write_event(Event::Empty(BytesStart::borrowed(
                        format!("field name='{}' required='{}' ", item.tag_text(), required)
                            .as_bytes(),
                        b"field".len(),
                    )))
                    .unwrap();
            }
            LayoutItemKind::Group(_, fields) => {
                writer
                    .write_event(Event::Start(BytesStart::borrowed(
                        format!("group name='{}' required='{}' ", item.tag_text(), required)
                            .as_bytes(),
                        b"group".len(),
                    )))
                    .unwrap();
                for item in fields {
                    write_layout_item(writer, item);
                }
                writer
                    .write_event(Event::End(BytesEnd::borrowed(b"group")))
                    .unwrap();
            }
            LayoutItemKind::Component(_) => {
                writer
                    .write_event(Event::Empty(BytesStart::borrowed(
                        format!(
                            "component name='{}' required='{}' ",
                            item.tag_text(),
                            required
                        )
                        .as_bytes(),
                        b"component".len(),
                    )))
                    .unwrap();
            }
        }
    }

    pub fn to_quickfix_xml(dict: &Dictionary) -> String {
        use quick_xml::events::*;
        use quick_xml::Writer;
        use std::io::Cursor;
        let mut writer = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
        writer
            .write_event(Event::Start(BytesStart::borrowed_name(b"fix")))
            .unwrap();
        writer
            .write_event(Event::Start(BytesStart::borrowed_name(b"header")))
            .unwrap();
        for item in dict.component_by_name("StandardHeader").unwrap().items() {
            write_layout_item(&mut writer, item);
        }
        writer
            .write_event(Event::End(BytesEnd::borrowed(b"header")))
            .unwrap();
        writer
            .write_event(Event::Start(BytesStart::borrowed_name(b"trailer")))
            .unwrap();
        for item in dict.component_by_name("StandardTrailer").unwrap().items() {
            write_layout_item(&mut writer, item);
        }
        writer
            .write_event(Event::End(BytesEnd::borrowed(b"trailer")))
            .unwrap();
        writer
            .write_event(Event::Start(BytesStart::borrowed_name(b"messages")))
            .unwrap();
        for message in dict.iter_messages() {
            writer
                .write_event(Event::Start(BytesStart::borrowed(
                    format!(
                        "message msgcat='{}' msgtype='{}' name='{}' ",
                        "FIXME",
                        message.msg_type(),
                        message.name()
                    )
                    .as_bytes(),
                    b"message".len(),
                )))
                .unwrap();
            for item in message.layout() {
                write_layout_item(&mut writer, item);
            }
            writer
                .write_event(Event::End(BytesEnd::borrowed(b"message")))
                .unwrap();
        }
        writer
            .write_event(Event::End(BytesEnd::borrowed(b"messages")))
            .unwrap();
        writer
            .write_event(Event::Start(BytesStart::borrowed_name(b"components")))
            .unwrap();
        for component in dict
            .iter_components()
            .filter(|c| c.name() != "StandardHeader" && c.name() != "StandardTrailer")
        {
            writer
                .write_event(Event::Start(BytesStart::borrowed(
                    format!("component name='{}' ", component.name()).as_bytes(),
                    "component".len(),
                )))
                .unwrap();
            for item in component.items() {
                write_layout_item(&mut writer, item);
            }
            writer
                .write_event(Event::End(BytesEnd::borrowed(b"component")))
                .unwrap();
        }
        writer
            .write_event(Event::End(BytesEnd::borrowed(b"components")))
            .unwrap();
        writer
            .write_event(Event::Start(BytesStart::borrowed_name(b"fields")))
            .unwrap();
        for field in dict.iter_fields() {
            writer
                .write_event(Event::Empty(BytesStart::borrowed(
                    format!(
                        "field name='{}' number='{}' type='{}' ",
                        field.name(),
                        field.tag(),
                        field.data_type().basetype().to_quickfix_name()
                    )
                    .as_bytes(),
                    b"field".len(),
                )))
                .unwrap();
        }
        writer
            .write_event(Event::End(BytesEnd::borrowed(b"fields")))
            .unwrap();
        writer
            .write_event(Event::End(BytesEnd::borrowed(b"fix")))
            .unwrap();
        String::from_utf8(writer.into_inner().into_inner()).unwrap()
    }

    pub struct QuickFixReader<'a> {
        node_with_header: roxmltree::Node<'a, 'a>,
        node_with_trailer: roxmltree::Node<'a, 'a>,
        node_with_components: roxmltree::Node<'a, 'a>,
        node_with_messages: roxmltree::Node<'a, 'a>,
        node_with_fields: roxmltree::Node<'a, 'a>,
        builder: DictionaryBuilder,
    }

    impl<'a> QuickFixReader<'a> {
        pub fn new(xml_document: &'a roxmltree::Document<'a>) -> ParseResult<Dictionary> {
            let mut reader = Self::empty(&xml_document)?;
            for child in reader.node_with_fields.children() {
                if child.is_element() {
                    import_field(&mut reader.builder, child)?;
                }
            }
            for child in reader.node_with_components.children() {
                if child.is_element() {
                    let name = child
                        .attribute("name")
                        .ok_or(ParseDictionaryError::InvalidFormat)?
                        .to_string();
                    import_component(&mut reader.builder, child, name)?;
                }
            }
            for child in reader.node_with_messages.children() {
                if child.is_element() {
                    import_message(&mut reader.builder, child)?;
                }
            }
            // `StandardHeader` and `StandardTrailer` are defined in ad-hoc
            // sections of the XML files. They're always there, even if
            // potentially empty (FIX 5.0+).
            import_component(
                &mut reader.builder,
                reader.node_with_header,
                "StandardHeader",
            )?;
            import_component(
                &mut reader.builder,
                reader.node_with_trailer,
                "StandardTrailer",
            )?;
            Ok(reader.builder.build())
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
                        "No major version attribute.".to_string(),
                    ))?;
            let version_minor =
                root.attribute("minor")
                    .ok_or(ParseDictionaryError::InvalidData(
                        "No minor version attribute.".to_string(),
                    ))?;
            let version = format!("{}.{}.{}", version_type, version_major, version_minor);
            Ok(QuickFixReader {
                builder: DictionaryBuilder::new(version),
                node_with_header: find_tagged_child("header")?,
                node_with_trailer: find_tagged_child("trailer")?,
                node_with_messages: find_tagged_child("messages")?,
                node_with_components: find_tagged_child("components")?,
                node_with_fields: find_tagged_child("fields")?,
            })
        }
    }

    fn import_field(
        builder: &mut DictionaryBuilder,
        node: roxmltree::Node,
    ) -> ParseResult<InternalId> {
        if node.tag_name().name() != "field" {
            return Err(ParseDictionaryError::InvalidFormat);
        }
        let data_type_iid = import_datatype(builder, node);
        let value_restrictions = value_restrictions_from_node(node, data_type_iid);
        let name = node
            .attribute("name")
            .ok_or(ParseDictionaryError::InvalidFormat)?
            .to_string();
        let tag = node
            .attribute("number")
            .ok_or(ParseDictionaryError::InvalidFormat)?
            .parse()
            .map_err(|_| ParseDictionaryError::InvalidFormat)?;
        let field = FieldData {
            name,
            tag,
            data_type_iid,
            associated_data_tag: None,
            value_restrictions,
            required: true,
            abbr_name: None,
            base_category_abbr_name: None,
            base_category_id: None,
            description: None,
        };
        Ok(builder.add_field(field))
    }

    fn import_message(
        builder: &mut DictionaryBuilder,
        node: roxmltree::Node,
    ) -> ParseResult<InternalId> {
        debug_assert_eq!(node.tag_name().name(), "message");
        let category_iid = import_category(builder, node)?;
        let layout_start = builder.layout_items.len() as u32;
        for child in node.children() {
            if child.is_element() {
                // We don't need to generate new IID's because we're dealing
                // with ranges.
                import_layout_item(builder, child)?;
            }
        }
        let layout_end = builder.layout_items.len() as u32;
        let message = MessageData {
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
        };
        Ok(builder.add_message(message))
    }

    fn import_component<S: AsRef<str>>(
        builder: &mut DictionaryBuilder,
        node: roxmltree::Node,
        name: S,
    ) -> ParseResult<InternalId> {
        let layout_start = builder.layout_items.len() as u32;
        for child in node.children() {
            if child.is_element() {
                import_layout_item(builder, child)?;
            }
        }
        let layout_end = builder.layout_items.len() as u32;
        let component = ComponentData {
            id: 0,
            component_type: ComponentType::Block,
            layout_items_iid_range: layout_start..layout_end,
            category_iid: 0, // FIXME
            name: name.as_ref().to_string(),
            abbr_name: None,
        };
        let iid = builder.add_component(component);
        match builder.symbol(KeyRef::ComponentByName(name.as_ref())) {
            Some(x) => Ok(*x),
            None => {
                builder
                    .symbol_table
                    .insert(Key::ComponentByName(name.as_ref().to_string()), iid);
                Ok(iid)
            }
        }
    }

    fn import_datatype(builder: &mut DictionaryBuilder, node: roxmltree::Node) -> InternalId {
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
        match builder.symbol(KeyRef::DatatypeByName(name)) {
            Some(x) => *x,
            None => {
                let iid = builder.data_types.len() as u32;
                let data = DatatypeData {
                    datatype,
                    description: String::new(),
                    examples: Vec::new(),
                };
                builder.data_types.push(data);
                builder
                    .symbol_table
                    .insert(Key::DatatypeByName(name.to_string()), iid);
                iid
            }
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

    fn import_layout_item(
        builder: &mut DictionaryBuilder,
        node: roxmltree::Node,
    ) -> ParseResult<InternalId> {
        // This processing step requires on fields being already present in
        // the dictionary.
        debug_assert_ne!(builder.fields.len(), 0);
        let name = node.attribute("name").unwrap();
        let required = node.attribute("required").unwrap() == "Y";
        let tag = node.tag_name().name();
        let kind = match tag {
            "field" => {
                let field_iid = builder.symbol(KeyRef::FieldByName(name)).unwrap();
                LayoutItemKindData::Field { iid: *field_iid }
            }
            "component" => {
                // Components may *not* be already present.
                let component_iid = import_component(builder, node, name)?;
                LayoutItemKindData::Component { iid: component_iid }
            }
            "group" => {
                let len_field_iid = *builder.symbol(KeyRef::FieldByName(name)).unwrap();
                let mut iids = Vec::new();
                for child in node.children().filter(|n| n.is_element()) {
                    iids.push(import_layout_item(builder, child)?);
                }
                LayoutItemKindData::Group {
                    len_field_iid,
                    items: Vec::new(),
                }
            }
            _ => {
                return Err(ParseDictionaryError::InvalidFormat);
            }
        };
        let item = LayoutItemData { required, kind };
        let iid = builder.layout_items.len() as InternalId;
        builder.layout_items.push(item);
        Ok(iid)
    }

    fn import_category(
        builder: &mut DictionaryBuilder,
        node: roxmltree::Node,
    ) -> ParseResult<InternalId> {
        debug_assert_eq!(node.tag_name().name(), "message");
        let name = node.attribute("msgcat").ok_or(ParseError::InvalidFormat)?;
        Ok(match builder.symbol(KeyRef::CategoryByName(name)) {
            Some(x) => *x,
            None => {
                let iid = builder.categories.len() as u32;
                builder.categories.push(CategoryData {
                    name: name.to_string(),
                    fixml_filename: String::new(),
                });
                builder
                    .symbol_table
                    .insert(Key::CategoryByName(name.to_string()), iid);
                iid
            }
        })
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
    use crate::AppVersion;
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
        let dict = Dictionary::from_version(AppVersion::Fixt11);
        println!("{}", dict.to_quickfix_xml());
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
        for version in AppVersion::ALL.iter().copied() {
            Dictionary::from_version(version);
        }
    }

    #[test]
    fn all_datatypes_are_used_at_least_once() {
        for version in AppVersion::ALL.iter().copied() {
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
        for version in AppVersion::ALL.iter().copied() {
            let dict = Dictionary::from_version(version);
            assert!(dict.iter_datatypes().count() >= 1);
        }
    }

    #[test]
    fn std_header_and_trailer_always_present() {
        for version in AppVersion::ALL.iter().copied() {
            let dict = Dictionary::from_version(version);
            let std_header = dict.component_by_name("StandardHeader");
            let std_trailer = dict.component_by_name("StandardTrailer");
            assert!(std_header.is_some() && std_trailer.is_some());
        }
    }

    #[test]
    fn fix44_field_28_has_three_variants() {
        let dict = Dictionary::from_version(AppVersion::Fix44);
        let field_28 = dict.field_by_tag(28).unwrap();
        assert_eq!(field_28.name(), "IOITransType");
        assert_eq!(field_28.enums().unwrap().count(), 3);
    }

    #[test]
    fn fix44_field_36_has_no_variants() {
        let dict = Dictionary::from_version(AppVersion::Fix44);
        let field_36 = dict.field_by_tag(36).unwrap();
        assert_eq!(field_36.name(), "NewSeqNo");
        assert!(field_36.enums().is_none());
    }

    #[test]
    fn fix44_field_167_has_eucorp_variant() {
        let dict = Dictionary::from_version(AppVersion::Fix44);
        let field_167 = dict.field_by_tag(167).unwrap();
        assert_eq!(field_167.name(), "SecurityType");
        assert!(field_167.enums().unwrap().any(|e| e.value() == "EUCORP"));
    }
}
