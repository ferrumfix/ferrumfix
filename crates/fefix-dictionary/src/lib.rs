//! Access to FIX Dictionary reference and message specifications.
#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod builder;
mod fix_datatype;
mod quickfix;

use builder::{
    AbbreviationData, CategoryData, ComponentData, DatatypeData, FieldData, FieldEnumData,
    LayoutItemData, LayoutItemKindData, MessageData,
};
pub use fix_datatype::FixDatatype;
use fnv::FnvHashMap;
use quickfix::{ParseDictionaryError, QuickFixReader};
use smartstring::alias::String as SmartString;
use std::sync::Arc;

/// Type alias for FIX tags: 32-bit unsigned integers, strictly positive.
pub type TagU32 = std::num::NonZeroU32;

pub trait DataFieldLookup<F> {
    fn field_is_data(&self, field: F) -> bool;
}

pub trait NumInGroupLookup<F> {
    fn field_is_num_in_group(&self, field: F) -> bool;
}

impl DataFieldLookup<u32> for Dictionary {
    fn field_is_data(&self, tag: u32) -> bool {
        if let Some(field) = self.field_by_tag(tag) {
            field.data_type().basetype() == FixDatatype::Data
        } else {
            false
        }
    }
}

impl NumInGroupLookup<u32> for Dictionary {
    fn field_is_num_in_group(&self, tag: u32) -> bool {
        if let Some(field) = self.field_by_tag(tag) {
            field.data_type().basetype() == FixDatatype::NumInGroup
        } else {
            false
        }
    }
}

/// The expected location of a field within a FIX message (i.e. header, body, or
/// trailer).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldLocation {
    /// The field is located inside the "Standard Header".
    Header,
    /// This field is located inside the body of the FIX message.
    Body,
    /// This field is located inside the "Standard Trailer".
    Trailer,
}

/// A mapping from FIX version strings to [`Dictionary`] values.
pub type Dictionaries = FnvHashMap<String, Arc<Dictionary>>;

/// Specifies business semantics for application-level entities within the FIX
/// Protocol.
///
/// You can rely on [`Dictionary`] for accessing details about
/// fields, messages, and other abstract entities as defined in the FIX
/// specifications. Examples of such information include:
///
/// - The mapping of FIX field names to numeric tags (e.g. `BeginString` is 8).
/// - Which FIX fields are mandatory and which are optional.
/// - The data type of each and every FIX field.
/// - What fields to expect in FIX headers.
///
/// N.B. The FIX Protocol mandates separation of concerns between session and
/// application protocol only for FIX 5.0 and subsequent versions. All FIX
/// Dictionaries for older versions will also contain information about the
/// session layer.
#[derive(Debug, Clone)]
pub struct Dictionary {
    version: String,

    abbreviation_definitions: FnvHashMap<SmartString, AbbreviationData>,

    data_types_by_name: FnvHashMap<SmartString, DatatypeData>,

    fields_by_tags: FnvHashMap<u32, FieldData>,
    field_tags_by_name: FnvHashMap<SmartString, u32>,

    components_by_name: FnvHashMap<SmartString, ComponentData>,

    messages_by_msgtype: FnvHashMap<SmartString, MessageData>,
    message_msgtypes_by_name: FnvHashMap<SmartString, SmartString>,

    //layout_items: Vec<LayoutItemData>,
    categories_by_name: FnvHashMap<SmartString, CategoryData>,
    header: Vec<FieldData>,
}

impl Dictionary {
    /// Creates a new empty FIX Dictionary named `version`.
    fn new<S: ToString>(version: S) -> Self {
        Dictionary {
            header: Vec::new(), // FIXME
            version: version.to_string(),
            abbreviation_definitions: FnvHashMap::default(),
            data_types_by_name: FnvHashMap::default(),
            fields_by_tags: FnvHashMap::default(),
            field_tags_by_name: FnvHashMap::default(),
            components_by_name: FnvHashMap::default(),
            messages_by_msgtype: FnvHashMap::default(),
            message_msgtypes_by_name: FnvHashMap::default(),
            categories_by_name: FnvHashMap::default(),
        }
    }

    /// Attempts to read a QuickFIX-style specification file and convert it into
    /// a [`Dictionary`].
    pub fn from_quickfix_spec(input: &str) -> Result<Self, ParseDictionaryError> {
        let xml_document =
            roxmltree::Document::parse(input).map_err(|_| ParseDictionaryError::InvalidFormat)?;
        QuickFixReader::new(&xml_document)
    }

    /// Returns the version string associated with this [`Dictionary`] (e.g.
    /// `FIXT.1.1`, `FIX.4.2`).
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    /// assert_eq!(dict.version(), "FIX.4.4");
    /// ```
    pub fn version(&self) -> &str {
        self.version.as_str()
    }

    /// Creates a new [`Dictionary`] for FIX 4.0.
    #[cfg(feature = "fix40")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix40")))]
    pub fn fix40() -> Self {
        let spec = include_str!("resources/quickfix/FIX-4.0.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 4.1.
    #[cfg(feature = "fix41")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix41")))]
    pub fn fix41() -> Self {
        let spec = include_str!("resources/quickfix/FIX-4.1.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 4.2.
    #[cfg(feature = "fix42")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix42")))]
    pub fn fix42() -> Self {
        let spec = include_str!("resources/quickfix/FIX-4.2.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 4.3.
    #[cfg(feature = "fix43")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix43")))]
    pub fn fix43() -> Self {
        let spec = include_str!("resources/quickfix/FIX-4.3.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 4.4.
    pub fn fix44() -> Self {
        let spec = include_str!("resources/quickfix/FIX-4.4.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 5.0.
    #[cfg(feature = "fix50")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix50")))]
    pub fn fix50() -> Self {
        let spec = include_str!("resources/quickfix/FIX-5.0.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 5.0 SP1.
    #[cfg(feature = "fix50sp1")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix50sp1")))]
    pub fn fix50sp1() -> Self {
        let spec = include_str!("resources/quickfix/FIX-5.0-SP1.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIX 5.0 SP2.
    #[cfg(feature = "fix50sp2")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fix50sp1")))]
    pub fn fix50sp2() -> Self {
        let spec = include_str!("resources/quickfix/FIX-5.0-SP2.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Creates a new [`Dictionary`] for FIXT 1.1.
    #[cfg(feature = "fixt11")]
    #[cfg_attr(doc_cfg, doc(cfg(feature = "fixt11")))]
    pub fn fixt11() -> Self {
        let spec = include_str!("resources/quickfix/FIXT-1.1.xml");
        Dictionary::from_quickfix_spec(spec).unwrap()
    }

    /// Returns a [`Vec`] of FIX [`Dictionary`]'s for the most common FIX
    /// versions (all that have been enabled via feature flags). This is only
    /// intended for testing purposes.
    pub fn common_dictionaries() -> Vec<Dictionary> {
        vec![
            #[cfg(feature = "fix40")]
            Self::fix40(),
            #[cfg(feature = "fix41")]
            Self::fix41(),
            #[cfg(feature = "fix42")]
            Self::fix42(),
            #[cfg(feature = "fix43")]
            Self::fix43(),
            #[cfg(feature = "fix44")]
            Self::fix44(),
            #[cfg(feature = "fix50")]
            Self::fix50(),
            #[cfg(feature = "fix50sp1")]
            Self::fix50sp1(),
            #[cfg(feature = "fix50sp2")]
            Self::fix50sp2(),
            #[cfg(feature = "fixt11")]
            Self::fixt11(),
        ]
    }

    /// Return the known abbreviation for `term` -if any- according to the
    /// documentation of this FIX Dictionary.
    pub fn abbreviation_for(&self, term: &str) -> Option<Abbreviation> {
        self.abbreviation_definitions.get(term).map(Abbreviation)
    }

    /// Returns the [`Message`](Message) associated with `name`, if any.
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    ///
    /// let msg1 = dict.message_by_name("Heartbeat").unwrap();
    /// let msg2 = dict.message_by_msgtype("0").unwrap();
    /// assert_eq!(msg1.name(), msg2.name());
    /// ```
    pub fn message_by_name(&self, name: &str) -> Option<Message> {
        let msg_type = self.message_msgtypes_by_name.get(name)?;
        self.message_by_msgtype(msg_type)
    }

    /// Returns the [`Message`](Message) that has the given `msgtype`, if any.
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    ///
    /// let msg1 = dict.message_by_msgtype("0").unwrap();
    /// let msg2 = dict.message_by_name("Heartbeat").unwrap();
    /// assert_eq!(msg1.name(), msg2.name());
    /// ```
    pub fn message_by_msgtype(&self, msgtype: &str) -> Option<Message> {
        self.messages_by_msgtype
            .get(msgtype)
            .map(|data| Message(self, data))
    }

    /// Returns the [`Component`] named `name`, if any.
    pub fn component_by_name(&self, name: &str) -> Option<Component> {
        self.components_by_name
            .get(name)
            .map(|data| Component(data, self))
    }

    /// Returns the [`Datatype`] named `name`, if any.
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    /// let dt = dict.datatype_by_name("String").unwrap();
    /// assert_eq!(dt.name(), "String");
    /// ```
    pub fn datatype_by_name(&self, name: &str) -> Option<Datatype> {
        self.data_types_by_name.get(name).map(Datatype)
    }

    /// Returns the [`Field`] associated with `tag`, if any.
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    /// let field1 = dict.field_by_tag(112).unwrap();
    /// let field2 = dict.field_by_name("TestReqID").unwrap();
    /// assert_eq!(field1.name(), field2.name());
    /// ```
    pub fn field_by_tag(&self, tag: u32) -> Option<Field> {
        self.fields_by_tags.get(&tag).map(|data| Field(self, data))
    }

    /// Returns the [`Field`] named `name`, if any.
    pub fn field_by_name(&self, name: &str) -> Option<Field> {
        let tag = self.field_tags_by_name.get(name)?;
        self.field_by_tag(*tag)
    }

    /// Returns the [`Category`] named `name`, if any.
    fn category_by_name(&self, name: &str) -> Option<Category> {
        self.categories_by_name.get(name).map(Category)
    }

    /// Returns a [`Vec`] of all [`Datatype`]'s in this [`Dictionary`]. The ordering
    /// of items is not specified.
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    /// // FIX 4.4 defines 23 (FIXME) datatypes.
    /// assert_eq!(dict.datatypes().len(), 23);
    /// ```
    pub fn datatypes(&self) -> Vec<Datatype> {
        self.data_types_by_name.values().map(Datatype).collect()
    }

    /// Returns a [`Vec`] of all [`Message`]'s in this [`Dictionary`]. The ordering
    /// of items is not specified.
    ///
    /// ```
    /// use fefix_dictionary::Dictionary;
    ///
    /// let dict = Dictionary::fix44();
    /// let msgs = dict.messages();
    /// let msg = msgs.iter().find(|m| m.name() == "MarketDataRequest");
    /// assert_eq!(msg.unwrap().msg_type(), "V");
    /// ```
    pub fn messages(&self) -> Vec<Message> {
        self.messages_by_msgtype
            .values()
            .map(|data| Message(self, data))
            .collect()
    }

    /// Returns a [`Vec`] of all [`Category`]'s in this [`Dictionary`]. The ordering
    /// of items is not specified.
    pub fn categories(&self) -> Vec<Category> {
        self.categories_by_name.values().map(Category).collect()
    }

    /// Returns a [`Vec`] of all [`Field`]'s in this [`Dictionary`]. The ordering
    /// of items is not specified.
    pub fn fields(&self) -> Vec<Field> {
        self.fields_by_tags
            .values()
            .map(|data| Field(self, data))
            .collect()
    }

    /// Returns a [`Vec`] of all [`Component`]'s in this [`Dictionary`]. The ordering
    /// of items is not specified.
    pub fn components(&self) -> Vec<Component> {
        self.components_by_name
            .values()
            .map(|data| Component(data, self))
            .collect()
    }
}

/// An [`Abbreviation`] is a standardized abbreviated form for a specific word,
/// pattern, or name. Abbreviation data is mostly meant for documentation
/// purposes, but in general it can have other uses as well, e.g. FIXML field
/// naming.
#[derive(Debug)]
pub struct Abbreviation<'a>(&'a AbbreviationData);

impl<'a> Abbreviation<'a> {
    /// Returns the full term (non-abbreviated) associated with `self`.
    pub fn term(&self) -> &str {
        self.0.abbreviation.as_str()
    }
}

/// A [`Category`] is a collection of loosely related FIX messages or components
/// all belonging to the same [`Section`].
#[derive(Clone, Debug)]
pub struct Category<'a>(&'a CategoryData);

impl<'a> Category<'a> {
    /// Returns the name of `self`. The name of every [`Category`] is unique
    /// across a [`Dictionary`].
    pub fn name(&self) -> &str {
        self.0.name.as_str()
    }
}

/// A [`Component`] is an ordered collection of fields and/or other components.
/// There are two kinds of components: (1) common blocks and (2) repeating
/// groups. Common blocks are merely commonly reused sequences of the same
/// fields/components
/// which are given names for simplicity, i.e. they serve as "macros". Repeating
/// groups, on the other hand, are components which can appear zero or more times
/// inside FIX messages (or other components, for that matter).
#[derive(Clone, Debug)]
pub struct Component<'a>(&'a ComponentData, &'a Dictionary);

impl<'a> Component<'a> {
    /// Returns the unique numberic ID of `self`.
    pub fn id(&self) -> u32 {
        self.0.id as u32
    }

    /// Returns the name of `self`. The name of every [`Component`] is unique
    /// across a [`Dictionary`].
    pub fn name(&self) -> &str {
        self.0.name.as_str()
    }

    /// Returns `true` if and only if `self` is a "group" component; `false`
    /// otherwise.
    pub fn is_group(&self) -> bool {
        match self.0.component_type {
            FixmlComponentAttributes::Block { is_repeating, .. } => is_repeating,
            _ => false,
        }
    }

    /// Returns the [`Category`] to which `self` belongs.
    pub fn category(&self) -> Category {
        self.1
            .category_by_name(self.0.category_name.as_str())
            .unwrap()
    }

    /// Returns an [`Iterator`] over all items that are part of `self`.
    pub fn items(&self) -> impl Iterator<Item = LayoutItem> {
        self.0
            .layout_items
            .iter()
            .map(move |data| LayoutItem(self.1, data))
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

/// Component type (FIXML-specific information).
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum FixmlComponentAttributes {
    Xml,
    Block {
        is_repeating: bool,
        is_implicit: bool,
        is_optimized: bool,
    },
    Message,
}

/// A FIX data type defined as part of a [`Dictionary`].
#[derive(Debug)]
pub struct Datatype<'a>(&'a DatatypeData);

impl<'a> Datatype<'a> {
    /// Returns the name of `self`.  This is also guaranteed to be a valid Rust
    /// identifier.
    pub fn name(&self) -> &str {
        self.0.datatype.name()
    }

    /// Returns `self` as an `enum`.
    pub fn basetype(&self) -> FixDatatype {
        self.0.datatype
    }
}

/// A limitation imposed on the value of a specific FIX [`Field`].  Also known as
/// "code set".
#[derive(Debug)]
pub struct FieldEnum<'a>(&'a FieldEnumData);

impl<'a> FieldEnum<'a> {
    /// Returns the string representation of this field variant.
    pub fn value(&self) -> &str {
        &self.0.value[..]
    }

    /// Returns the documentation description for `self`.
    pub fn description(&self) -> &str {
        &self.0.description[..]
    }
}

/// A field is the most granular message structure abstraction. It carries a
/// specific business meaning as described by the FIX specifications. The data
/// domain of a [`Field`] is either a [`Datatype`] or a "code set", i.e.
/// enumeration.
#[derive(Debug, Copy, Clone)]
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
        format!(
            "https://www.onixs.biz/fix-dictionary/{}/tagNum_{}.html",
            v,
            self.1.tag.to_string().as_str()
        )
    }

    pub fn is_num_in_group(&self) -> bool {
        fn nth_char_is_uppercase(s: &str, i: usize) -> bool {
            s.chars().nth(i).map(|c| c.is_ascii_uppercase()) == Some(true)
        }

        self.fix_datatype().base_type() == FixDatatype::NumInGroup
            || self.name().ends_with("Len")
            || (self.name().starts_with("No") && nth_char_is_uppercase(self.name(), 2))
    }

    /// Returns the [`FixDatatype`] of `self`.
    pub fn fix_datatype(&self) -> FixDatatype {
        self.data_type().basetype()
    }

    /// Returns the name of `self`. Field names are unique across each FIX
    /// [`Dictionary`].
    pub fn name(&self) -> &str {
        self.1.name.as_str()
    }

    /// Returns the numeric tag of `self`. Field tags are unique across each FIX
    /// [`Dictionary`].
    pub fn tag(&self) -> TagU32 {
        TagU32::new(self.1.tag).unwrap()
    }

    /// In case this field allows any value, it returns `None`; otherwise; it
    /// returns an [`Iterator`] of all allowed values.
    pub fn enums(&self) -> Option<impl Iterator<Item = FieldEnum>> {
        self.1
            .value_restrictions
            .as_ref()
            .map(move |v| v.iter().map(FieldEnum))
    }

    /// Returns the [`Datatype`] of `self`.
    pub fn data_type(&self) -> Datatype {
        self.0
            .datatype_by_name(self.1.data_type_name.as_str())
            .unwrap()
    }

    pub fn data_tag(&self) -> Option<TagU32> {
        self.1
            .associated_data_tag
            .map(|tag| TagU32::new(tag as u32).unwrap())
    }

    pub fn required_in_xml_messages(&self) -> bool {
        self.1.required
    }

    pub fn description(&self) -> Option<&str> {
        self.1.description.as_deref()
    }
}

impl<'a> IsFieldDefinition for Field<'a> {
    fn name(&self) -> &str {
        self.1.name.as_str()
    }

    fn tag(&self) -> TagU32 {
        TagU32::new(self.1.tag).expect("Invalid FIX tag (0)")
    }

    fn location(&self) -> FieldLocation {
        FieldLocation::Body // FIXME
    }
}

pub trait IsFieldDefinition {
    /// Returns the FIX tag associated with `self`.
    fn tag(&self) -> TagU32;

    /// Returns the official, ASCII, human-readable name associated with `self`.
    fn name(&self) -> &str;

    /// Returns the field location of `self`.
    fn location(&self) -> FieldLocation;
}

fn layout_item_kind<'a>(item: &'a LayoutItemKindData, dict: &'a Dictionary) -> LayoutItemKind<'a> {
    match item {
        LayoutItemKindData::Component { name } => {
            LayoutItemKind::Component(dict.component_by_name(name).unwrap())
        }
        LayoutItemKindData::Group {
            len_field_tag,
            items: items_data,
        } => {
            let items = items_data
                .iter()
                .map(|item_data| LayoutItem(dict, item_data))
                .collect::<Vec<_>>();
            let len_field = dict.field_by_tag(*len_field_tag).unwrap();
            LayoutItemKind::Group(len_field, items)
        }
        LayoutItemKindData::Field { tag } => {
            LayoutItemKind::Field(dict.field_by_tag(*tag).unwrap())
        }
    }
}

/// An entry in a sequence of FIX field definitions.
#[derive(Clone, Debug)]
pub struct LayoutItem<'a>(&'a Dictionary, &'a LayoutItemData);

/// The kind of element contained in a [`Message`].
#[derive(Debug)]
pub enum LayoutItemKind<'a> {
    /// This component item is another component.
    Component(Component<'a>),
    /// This component item is a FIX repeating group.
    Group(Field<'a>, Vec<LayoutItem<'a>>),
    /// This component item is a FIX field.
    Field(Field<'a>),
}

impl<'a> LayoutItem<'a> {
    /// Returns `true` if `self` is required in order to have a valid definition
    /// of its parent container, `false` otherwise.
    pub fn required(&self) -> bool {
        self.1.required
    }

    /// Returns the [`LayoutItemKind`] of `self`.
    pub fn kind(&self) -> LayoutItemKind {
        layout_item_kind(&self.1.kind, self.0)
    }

    /// Returns the human-readable name of `self`.
    pub fn tag_text(&self) -> String {
        match &self.1.kind {
            LayoutItemKindData::Component { name } => {
                self.0.component_by_name(name).unwrap().name().to_string()
            }
            LayoutItemKindData::Group {
                len_field_tag,
                items: _items,
            } => self
                .0
                .field_by_tag(*len_field_tag)
                .unwrap()
                .name()
                .to_string(),
            LayoutItemKindData::Field { tag } => {
                self.0.field_by_tag(*tag).unwrap().name().to_string()
            }
        }
    }
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

    pub fn group_info(&self, num_in_group_tag: TagU32) -> Option<TagU32> {
        self.layout().find_map(|layout_item| {
            if let LayoutItemKind::Group(field, items) = layout_item.kind() {
                if field.tag() == num_in_group_tag {
                    if let LayoutItemKind::Field(f) = items[0].kind() {
                        Some(f.tag())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else if let LayoutItemKind::Component(_component) = layout_item.kind() {
                None
            } else {
                None
            }
        })
    }

    /// Returns the component ID of `self`.
    pub fn component_id(&self) -> u32 {
        self.1.component_id
    }

    pub fn layout(&self) -> impl Iterator<Item = LayoutItem> {
        self.1
            .layout_items
            .iter()
            .map(move |data| LayoutItem(self.0, data))
    }

    pub fn fixml_required(&self) -> bool {
        self.1.required
    }
}

/// A [`Section`] is a collection of many [`Component`]-s. It has no practical
/// effect on encoding and decoding of FIX data and it's only used for
/// documentation and human readability.
#[derive(Clone, Debug, PartialEq)]
pub struct Section {}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn fix44_quickfix_is_ok() {
        let dict = Dictionary::fix44();
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
    fn all_datatypes_are_used_at_least_once() {
        for dict in Dictionary::common_dictionaries().iter() {
            let datatypes_count = dict.datatypes().len();
            let mut datatypes = HashSet::new();
            for field in dict.fields() {
                datatypes.insert(field.data_type().name().to_string());
            }
            assert_eq!(datatypes_count, datatypes.len());
        }
    }

    #[test]
    fn at_least_one_datatype() {
        for dict in Dictionary::common_dictionaries().iter() {
            assert!(!dict.datatypes().is_empty());
        }
    }

    #[test]
    fn std_header_and_trailer_always_present() {
        for dict in Dictionary::common_dictionaries().iter() {
            let std_header = dict.component_by_name("StandardHeader");
            let std_trailer = dict.component_by_name("StandardTrailer");
            assert!(std_header.is_some() && std_trailer.is_some());
        }
    }

    #[test]
    fn fix44_field_28_has_three_variants() {
        let dict = Dictionary::fix44();
        let field_28 = dict.field_by_tag(28).unwrap();
        assert_eq!(field_28.name(), "IOITransType");
        assert_eq!(field_28.enums().unwrap().count(), 3);
    }

    #[test]
    fn fix44_field_36_has_no_variants() {
        let dict = Dictionary::fix44();
        let field_36 = dict.field_by_tag(36).unwrap();
        assert_eq!(field_36.name(), "NewSeqNo");
        assert!(field_36.enums().is_none());
    }

    #[test]
    fn fix44_field_167_has_eucorp_variant() {
        let dict = Dictionary::fix44();
        let field_167 = dict.field_by_tag(167).unwrap();
        assert_eq!(field_167.name(), "SecurityType");
        assert!(field_167.enums().unwrap().any(|e| e.value() == "EUCORP"));
    }

    const INVALID_QUICKFIX_SPECS: &[&str] = &[
        include_str!("test_data/quickfix_specs/empty_file.xml"),
        include_str!("test_data/quickfix_specs/missing_components.xml"),
        include_str!("test_data/quickfix_specs/missing_fields.xml"),
        include_str!("test_data/quickfix_specs/missing_header.xml"),
        include_str!("test_data/quickfix_specs/missing_messages.xml"),
        include_str!("test_data/quickfix_specs/missing_trailer.xml"),
        include_str!("test_data/quickfix_specs/root_has_no_type_attr.xml"),
        include_str!("test_data/quickfix_specs/root_has_no_version_attrs.xml"),
        include_str!("test_data/quickfix_specs/root_is_not_fix.xml"),
    ];

    #[test]
    fn invalid_quickfix_specs() {
        for spec in INVALID_QUICKFIX_SPECS.iter() {
            let dict = Dictionary::from_quickfix_spec(spec);
            assert!(dict.is_err(), "{}", spec);
        }
    }
}
