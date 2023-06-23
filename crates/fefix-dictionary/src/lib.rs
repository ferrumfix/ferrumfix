//! Access to FIX Dictionary reference and message specifications.

mod quickfix;

pub use datatype::FixDatatype;
use fnv::FnvHashMap;
use quickfix::{ParseDictionaryError, QuickFixReader};
use smartstring::alias::String as SmartString;
use std::{fmt, sync::Arc};

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
    /// assert_eq!(dict.get_version(), "FIX.4.4");
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
    /// versions (that have been enabled via feature flags). This is only
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
        self.abbreviation_definitions
            .get(term)
            .map(|data| Abbreviation(self, data))
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
            .map(|data| Component(self, data))
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
        self.data_types_by_name
            .get(name)
            .map(|data| Datatype(self, data))
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
        self.categories_by_name
            .get(name)
            .map(|data| Category(self, data))
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
        self.data_types_by_name
            .values()
            .map(|data| Datatype(self, data))
            .collect()
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
        self.categories_by_name
            .values()
            .map(|data| Category(self, data))
            .collect()
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
            .map(|data| Component(self, data))
            .collect()
    }
}

/// Builder utilities
impl Dictionary {
    fn add_field(&mut self, field: FieldData) {
        self.field_tags_by_name
            .insert(field.name.clone(), field.tag);
        self.fields_by_tags.insert(field.tag, field);
    }

    fn add_message(&mut self, message: MessageData) {
        self.message_msgtypes_by_name
            .insert(message.name.clone(), message.msg_type.clone());
        self.messages_by_msgtype
            .insert(message.msg_type.clone(), message);
    }

    fn add_component(&mut self, component: ComponentData) {
        self.components_by_name
            .insert(component.name.clone(), component);
    }

    fn add_datatype(&mut self, datatype: DatatypeData) {
        self.data_types_by_name
            .insert(datatype.datatype.name().into(), datatype);
    }

    fn add_category(&mut self, category: CategoryData) {
        self.categories_by_name
            .insert(category.name.clone().into(), category);
    }
}

#[derive(Clone, Debug)]
struct AbbreviationData {
    abbreviation: SmartString,
    is_last: bool,
}

/// An [`Abbreviation`] is a standardized abbreviated form for a specific word,
/// pattern, or name. Abbreviation data is mostly meant for documentation
/// purposes, but in general it can have other uses as well, e.g. FIXML field
/// naming.
#[derive(Debug)]
pub struct Abbreviation<'a>(&'a Dictionary, &'a AbbreviationData);

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
    component_type: FixmlComponentAttributes,
    layout_items: Vec<LayoutItemData>,
    category_name: SmartString,
    /// The human readable name of the component.
    name: SmartString,
    /// The name for this component when used in an XML context.
    abbr_name: Option<SmartString>,
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
            FixmlComponentAttributes::Block { is_repeating, .. } => is_repeating,
            _ => false,
        }
    }

    /// Returns the [`Category`] to which `self` belongs.
    pub fn category(&self) -> Category {
        self.0
            .category_by_name(self.1.category_name.as_str())
            .unwrap()
    }

    /// Returns an [`Iterator`] over all items that are part of `self`.
    pub fn items(&self) -> impl Iterator<Item = LayoutItem> {
        self.1
            .layout_items
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

#[derive(Clone, Debug, PartialEq)]
struct DatatypeData {
    /// **Primary key.** Identifier of the datatype.
    datatype: FixDatatype,
    /// Human readable description of this Datatype.
    description: String,
    /// A string that contains examples values for a datatype
    examples: Vec<String>,
    // TODO: 'XML'.
}

/// A FIX data type defined as part of a [`Dictionary`].
#[derive(Debug)]
pub struct Datatype<'a>(&'a Dictionary, &'a DatatypeData);

impl<'a> Datatype<'a> {
    /// Returns the name of `self`.  This is also guaranteed to be a valid Rust
    /// identifier.
    pub fn name(&self) -> &str {
        self.1.datatype.name()
    }

    /// Returns `self` as an `enum`.
    pub fn basetype(&self) -> FixDatatype {
        self.1.datatype
    }
}

mod datatype {
    use strum::IntoEnumIterator;
    use strum_macros::{EnumIter, IntoStaticStr};

    /// Sum type for all possible FIX data types ever defined across all FIX
    /// application versions.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
    #[repr(u8)]
    #[non_exhaustive]
    pub enum FixDatatype {
        /// Single character value, can include any alphanumeric character or
        /// punctuation except the delimiter. All char fields are case sensitive
        /// (i.e. m != M). The following fields are based on char.
        Char,
        /// char field containing one of two values: 'Y' = True/Yes 'N' = False/No.
        Boolean,
        /// Sequence of digits with optional decimal point and sign character (ASCII
        /// characters "-", "0" - "9" and "."); the absence of the decimal point
        /// within the string will be interpreted as the float representation of an
        /// integer value. All float fields must accommodate up to fifteen
        /// significant digits. The number of decimal places used should be a factor
        /// of business/market needs and mutual agreement between counterparties.
        /// Note that float values may contain leading zeros (e.g. "00023.23" =
        /// "23.23") and may contain or omit trailing zeros after the decimal point
        /// (e.g. "23.0" = "23.0000" = "23" = "23."). Note that fields which are
        /// derived from float may contain negative values unless explicitly
        /// specified otherwise. The following data types are based on float.
        Float,
        /// float field typically representing a Price times a Qty.
        Amt,
        /// float field representing a price. Note the number of decimal places may
        /// vary. For certain asset classes prices may be negative values. For
        /// example, prices for options strategies can be negative under certain
        /// market conditions. Refer to Volume 7: FIX Usage by Product for asset
        /// classes that support negative price values.
        Price,
        /// float field representing a price offset, which can be mathematically
        /// added to a "Price". Note the number of decimal places may vary and some
        /// fields such as LastForwardPoints may be negative.
        PriceOffset,
        /// float field capable of storing either a whole number (no decimal places)
        /// of "shares" (securities denominated in whole units) or a decimal value
        /// containing decimal places for non-share quantity asset classes
        /// (securities denominated in fractional units).
        Qty,
        /// float field representing a percentage (e.g. 0.05 represents 5% and 0.9525
        /// represents 95.25%). Note the number of decimal places may vary.
        Percentage,
        /// Sequence of digits without commas or decimals and optional sign character
        /// (ASCII characters "-" and "0" - "9" ). The sign character utilizes one
        /// byte (i.e. positive int is "99999" while negative int is "-99999"). Note
        /// that int values may contain leading zeros (e.g. "00023" = "23").
        /// Examples: 723 in field 21 would be mapped int as |21=723|. -723 in field
        /// 12 would be mapped int as |12=-723| The following data types are based on
        /// int.
        Int,
        /// int field representing a day during a particular monthy (values 1 to 31).
        DayOfMonth,
        /// int field representing the length in bytes. Value must be positive.
        Length,
        /// int field representing the number of entries in a repeating group. Value
        /// must be positive.
        NumInGroup,
        /// int field representing a message sequence number. Value must be positive.
        SeqNum,
        /// `int` field representing a field's tag number when using FIX "Tag=Value"
        /// syntax. Value must be positive and may not contain leading zeros.
        TagNum,
        /// Alpha-numeric free format strings, can include any character or
        /// punctuation except the delimiter. All String fields are case sensitive
        /// (i.e. morstatt != Morstatt).
        String,
        /// string field containing raw data with no format or content restrictions.
        /// Data fields are always immediately preceded by a length field. The length
        /// field should specify the number of bytes of the value of the data field
        /// (up to but not including the terminating SOH). Caution: the value of one
        /// of these fields may contain the delimiter (SOH) character. Note that the
        /// value specified for this field should be followed by the delimiter (SOH)
        /// character as all fields are terminated with an "SOH".
        Data,
        /// string field representing month of a year. An optional day of the month
        /// can be appended or an optional week code. Valid formats: YYYYMM YYYYMMDD
        /// YYYYMMWW Valid values: YYYY = 0000-9999; MM = 01-12; DD = 01-31; WW = w1,
        /// w2, w3, w4, w5.
        MonthYear,
        /// string field containing one or more space delimited single character
        /// values (e.g. |18=2 A F| ).
        MultipleCharValue,
        /// string field representing a currency type using ISO 4217 Currency code (3
        /// character) values (see Appendix 6-A).
        Currency,
        /// string field representing a market or exchange using ISO 10383 Market
        /// Identifier Code (MIC) values (see"Appendix 6-C).
        Exchange,
        /// Identifier for a national language - uses ISO 639-1 standard.
        Language,
        /// string field represening a Date of Local Market (as oppose to UTC) in
        /// YYYYMMDD format. This is the "normal" date field used by the FIX
        /// Protocol. Valid values: YYYY = 0000-9999, MM = 01-12, DD = 01-31.
        LocalMktDate,
        /// string field containing one or more space delimited multiple character
        /// values (e.g. |277=AV AN A| ).
        MultipleStringValue,
        /// string field representing Date represented in UTC (Universal Time
        /// Coordinated, also known as "GMT") in YYYYMMDD format. This
        /// special-purpose field is paired with UTCTimeOnly to form a proper
        /// UTCTimestamp for bandwidth-sensitive messages. Valid values: YYYY =
        /// 0000-9999, MM = 01-12, DD = 01-31.
        UtcDateOnly,
        /// string field representing Time-only represented in UTC (Universal Time
        /// Coordinated, also known as "GMT") in either HH:MM:SS (whole seconds) or
        /// HH:MM:SS.sss (milliseconds) format, colons, and period required. This
        /// special-purpose field is paired with UTCDateOnly to form a proper
        /// UTCTimestamp for bandwidth-sensitive messages. Valid values: HH = 00-23,
        /// MM = 00-60 (60 only if UTC leap second), SS = 00-59. (without
        /// milliseconds) HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap
        /// second), sss=000-999 (indicating milliseconds).
        UtcTimeOnly,
        /// string field representing Time/date combination represented in UTC
        /// (Universal Time Coordinated, also known as "GMT") in either
        /// YYYYMMDD-HH:MM:SS (whole seconds) or YYYYMMDD-HH:MM:SS.sss (milliseconds)
        /// format, colons, dash, and period required. Valid values: * YYYY =
        /// 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60
        /// only if UTC leap second) (without milliseconds). * YYYY = 0000-9999, MM =
        /// 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC
        /// leap second), sss=000-999 (indicating milliseconds). Leap Seconds: Note
        /// that UTC includes corrections for leap seconds, which are inserted to
        /// account for slowing of the rotation of the earth. Leap second insertion
        /// is declared by the International Earth Rotation Service (IERS) and has,
        /// since 1972, only occurred on the night of Dec. 31 or Jun 30. The IERS
        /// considers March 31 and September 30 as secondary dates for leap second
        /// insertion, but has never utilized these dates. During a leap second
        /// insertion, a UTCTimestamp field may read "19981231-23:59:59",
        /// "19981231-23:59:60", "19990101-00:00:00". (see
        /// <http://tycho.usno.navy.mil/leapsec.html>)
        UtcTimestamp,
        /// Contains an XML document raw data with no format or content restrictions.
        /// XMLData fields are always immediately preceded by a length field. The
        /// length field should specify the number of bytes of the value of the data
        /// field (up to but not including the terminating SOH).
        XmlData,
        /// string field representing a country using ISO 3166 Country code (2
        /// character) values (see Appendix 6-B).
        Country,
    }

    impl FixDatatype {
        /// Compares `name` to the set of strings commonly used by QuickFIX's custom
        /// specification format and returns its associated
        /// [`Datatype`](super::Datatype) if a match
        /// was found. The query is case-insensitive.
        ///
        /// # Examples
        ///
        /// ```
        /// use fefix_dictionary::FixDatatype;
        ///
        /// assert_eq!(FixDatatype::from_quickfix_name("AMT"), Some(FixDatatype::Amt));
        /// assert_eq!(FixDatatype::from_quickfix_name("Amt"), Some(FixDatatype::Amt));
        /// assert_eq!(FixDatatype::from_quickfix_name("MONTHYEAR"), Some(FixDatatype::MonthYear));
        /// assert_eq!(FixDatatype::from_quickfix_name(""), None);
        /// ```
        pub fn from_quickfix_name(name: &str) -> Option<Self> {
            // https://github.com/quickfix/quickfix/blob/b6760f55ac6a46306b4e081bb13b65e6220ab02d/src/C%2B%2B/DataDictionary.cpp#L646-L680
            Some(match name.to_ascii_uppercase().as_str() {
                "AMT" => FixDatatype::Amt,
                "BOOLEAN" => FixDatatype::Boolean,
                "CHAR" => FixDatatype::Char,
                "COUNTRY" => FixDatatype::Country,
                "CURRENCY" => FixDatatype::Currency,
                "DATA" => FixDatatype::Data,
                "DATE" => FixDatatype::UtcDateOnly, // FIXME?
                "DAYOFMONTH" => FixDatatype::DayOfMonth,
                "EXCHANGE" => FixDatatype::Exchange,
                "FLOAT" => FixDatatype::Float,
                "INT" => FixDatatype::Int,
                "LANGUAGE" => FixDatatype::Language,
                "LENGTH" => FixDatatype::Length,
                "LOCALMKTDATE" => FixDatatype::LocalMktDate,
                "MONTHYEAR" => FixDatatype::MonthYear,
                "MULTIPLECHARVALUE" | "MULTIPLEVALUESTRING" => FixDatatype::MultipleCharValue,
                "MULTIPLESTRINGVALUE" => FixDatatype::MultipleStringValue,
                "NUMINGROUP" => FixDatatype::NumInGroup,
                "PERCENTAGE" => FixDatatype::Percentage,
                "PRICE" => FixDatatype::Price,
                "PRICEOFFSET" => FixDatatype::PriceOffset,
                "QTY" => FixDatatype::Qty,
                "STRING" => FixDatatype::String,
                "TZTIMEONLY" => FixDatatype::UtcTimeOnly, // FIXME
                "TZTIMESTAMP" => FixDatatype::UtcTimestamp, // FIXME
                "UTCDATE" => FixDatatype::UtcDateOnly,
                "UTCDATEONLY" => FixDatatype::UtcDateOnly,
                "UTCTIMEONLY" => FixDatatype::UtcTimeOnly,
                "UTCTIMESTAMP" => FixDatatype::UtcTimestamp,
                "SEQNUM" => FixDatatype::SeqNum,
                "TIME" => FixDatatype::UtcTimestamp,
                "XMLDATA" => FixDatatype::XmlData,
                _ => {
                    return None;
                }
            })
        }

        /// Returns the name adopted by QuickFIX for `self`.
        pub fn to_quickfix_name(&self) -> &str {
            match self {
                FixDatatype::Int => "INT",
                FixDatatype::Length => "LENGTH",
                FixDatatype::Char => "CHAR",
                FixDatatype::Boolean => "BOOLEAN",
                FixDatatype::Float => "FLOAT",
                FixDatatype::Amt => "AMT",
                FixDatatype::Price => "PRICE",
                FixDatatype::PriceOffset => "PRICEOFFSET",
                FixDatatype::Qty => "QTY",
                FixDatatype::Percentage => "PERCENTAGE",
                FixDatatype::DayOfMonth => "DAYOFMONTH",
                FixDatatype::NumInGroup => "NUMINGROUP",
                FixDatatype::Language => "LANGUAGE",
                FixDatatype::SeqNum => "SEQNUM",
                FixDatatype::TagNum => "TAGNUM",
                FixDatatype::String => "STRING",
                FixDatatype::Data => "DATA",
                FixDatatype::MonthYear => "MONTHYEAR",
                FixDatatype::Currency => "CURRENCY",
                FixDatatype::Exchange => "EXCHANGE",
                FixDatatype::LocalMktDate => "LOCALMKTDATE",
                FixDatatype::MultipleStringValue => "MULTIPLESTRINGVALUE",
                FixDatatype::UtcTimeOnly => "UTCTIMEONLY",
                FixDatatype::UtcTimestamp => "UTCTIMESTAMP",
                FixDatatype::UtcDateOnly => "UTCDATEONLY",
                FixDatatype::Country => "COUNTRY",
                FixDatatype::MultipleCharValue => "MULTIPLECHARVALUE",
                FixDatatype::XmlData => "XMLDATA",
            }
        }

        /// Returns the name of `self`, character by character identical to the name
        /// that appears in the official guidelines. **Generally** primitive datatypes
        /// will use `snake_case` and non-primitive ones will have `PascalCase`, but
        /// that's not true for every [`Datatype`](super::Datatype).
        ///
        /// # Examples
        ///
        /// ```
        /// use fefix_dictionary::FixDatatype;
        ///
        /// assert_eq!(FixDatatype::Qty.name(), "Qty");
        /// assert_eq!(FixDatatype::Float.name(), "float");
        /// assert_eq!(FixDatatype::String.name(), "String");
        /// ```
        pub fn name(&self) -> &'static str {
            // 1. Most primitive data types have `snake_case` names.
            // 2. Most derivative data types have `PascalCase` names.
            // 3. `data` and `String` ruin the party and mess it up.
            //    Why, you ask? Oh, you sweet summer child. You'll learn soon enough
            //    that nothing makes sense in FIX land.
            match self {
                FixDatatype::Int => "int",
                FixDatatype::Length => "Length",
                FixDatatype::Char => "char",
                FixDatatype::Boolean => "Boolean",
                FixDatatype::Float => "float",
                FixDatatype::Amt => "Amt",
                FixDatatype::Price => "Price",
                FixDatatype::PriceOffset => "PriceOffset",
                FixDatatype::Qty => "Qty",
                FixDatatype::Percentage => "Percentage",
                FixDatatype::DayOfMonth => "DayOfMonth",
                FixDatatype::NumInGroup => "NumInGroup",
                FixDatatype::Language => "Language",
                FixDatatype::SeqNum => "SeqNum",
                FixDatatype::TagNum => "TagNum",
                FixDatatype::String => "String",
                FixDatatype::Data => "data",
                FixDatatype::MonthYear => "MonthYear",
                FixDatatype::Currency => "Currency",
                FixDatatype::Exchange => "Exchange",
                FixDatatype::LocalMktDate => "LocalMktDate",
                FixDatatype::MultipleStringValue => "MultipleStringValue",
                FixDatatype::UtcTimeOnly => "UTCTimeOnly",
                FixDatatype::UtcTimestamp => "UTCTimestamp",
                FixDatatype::UtcDateOnly => "UTCDateOnly",
                FixDatatype::Country => "Country",
                FixDatatype::MultipleCharValue => "MultipleCharValue",
                FixDatatype::XmlData => "XMLData",
            }
        }

        /// Returns `true` if and only if `self` is a "base type", i.e. a primitive;
        /// returns `false` otherwise.
        ///
        /// # Examples
        ///
        /// ```
        /// use fefix_dictionary::FixDatatype;
        ///
        /// assert_eq!(FixDatatype::Float.is_base_type(), true);
        /// assert_eq!(FixDatatype::Price.is_base_type(), false);
        /// ```
        pub fn is_base_type(&self) -> bool {
            match self {
                Self::Char | Self::Float | Self::Int | Self::String => true,
                _ => false,
            }
        }

        /// Returns the primitive [`Datatype`](super::Datatype) from which `self` is derived. If
        /// `self` is primitive already, returns `self` unchanged.
        ///
        /// # Examples
        ///
        /// ```
        /// use fefix_dictionary::FixDatatype;
        ///
        /// assert_eq!(FixDatatype::Float.base_type(), FixDatatype::Float);
        /// assert_eq!(FixDatatype::Price.base_type(), FixDatatype::Float);
        /// ```
        pub fn base_type(&self) -> Self {
            let dt = match self {
                Self::Char | Self::Boolean => Self::Char,
                Self::Float
                | Self::Amt
                | Self::Price
                | Self::PriceOffset
                | Self::Qty
                | Self::Percentage => Self::Float,
                Self::Int
                | Self::DayOfMonth
                | Self::Length
                | Self::NumInGroup
                | Self::SeqNum
                | Self::TagNum => Self::Int,
                _ => Self::String,
            };
            debug_assert!(dt.is_base_type());
            dt
        }

        /// Returns an [`Iterator`] over all variants of
        /// [`Datatype`](super::Datatype).
        pub fn iter_all() -> impl Iterator<Item = Self> {
            <Self as IntoEnumIterator>::iter()
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        use std::collections::HashSet;

        #[test]
        fn iter_all_unique() {
            let as_vec = FixDatatype::iter_all().collect::<Vec<FixDatatype>>();
            let as_set = FixDatatype::iter_all().collect::<HashSet<FixDatatype>>();
            assert_eq!(as_vec.len(), as_set.len());
        }

        #[test]
        fn more_than_20_datatypes() {
            // According to the official documentation, FIX has "about 20 data
            // types". Including recent revisions, we should well exceed that
            // number.
            assert!(FixDatatype::iter_all().count() > 20);
        }

        #[test]
        fn names_are_unique() {
            let as_vec = FixDatatype::iter_all()
                .map(|dt| dt.name())
                .collect::<Vec<&str>>();
            let as_set = FixDatatype::iter_all()
                .map(|dt| dt.name())
                .collect::<HashSet<&str>>();
            assert_eq!(as_vec.len(), as_set.len());
        }

        #[test]
        fn base_type_is_itself() {
            for dt in FixDatatype::iter_all() {
                if dt.is_base_type() {
                    assert_eq!(dt.base_type(), dt);
                } else {
                    assert_ne!(dt.base_type(), dt);
                }
            }
        }

        #[test]
        fn base_type_is_actually_base_type() {
            for dt in FixDatatype::iter_all() {
                assert!(dt.base_type().is_base_type());
            }
        }
    }
}

/// A field is identified by a unique tag number and a name. Each field in a
/// message is associated with a value.
#[derive(Clone, Debug)]
struct FieldData {
    /// A human readable string representing the name of the field.
    name: SmartString,
    /// **Primary key.** A positive integer representing the unique
    /// identifier for this field type.
    tag: u32,
    /// The datatype of the field.
    data_type_name: SmartString,
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

/// A limitation imposed on the value of a specific FIX [`Field`].  Also known as
/// "code set".
#[derive(Debug)]
pub struct FieldEnum<'a>(&'a Dictionary, &'a FieldEnumData);

impl<'a> FieldEnum<'a> {
    /// Returns the string representation of this field variant.
    pub fn value(&self) -> &str {
        &self.1.value[..]
    }

    /// Returns the documentation description for `self`.
    pub fn description(&self) -> &str {
        &self.1.description[..]
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
            .map(move |v| v.iter().map(move |f| FieldEnum(self.0, f)))
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
        self.1.description.as_ref().map(|s| s.as_str())
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

#[derive(Clone, Debug)]
#[allow(dead_code)]
enum LayoutItemKindData {
    Component {
        name: SmartString,
    },
    Group {
        len_field_tag: u32,
        items: Vec<LayoutItemData>,
    },
    Field {
        tag: u32,
    },
}

#[derive(Clone, Debug)]
struct LayoutItemData {
    required: bool,
    kind: LayoutItemKindData,
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

type LayoutItems = Vec<LayoutItemData>;

#[derive(Clone, Debug)]
struct MessageData {
    /// The unique integer identifier of this message type.
    component_id: u32,
    /// **Primary key**. The unique character identifier of this message
    /// type; used literally in FIX messages.
    msg_type: SmartString,
    /// The name of this message type.
    name: SmartString,
    /// Identifier of the category to which this message belongs.
    category_name: SmartString,
    /// Identifier of the section to which this message belongs.
    section_id: String,
    layout_items: LayoutItems,
    /// The abbreviated name of this message, when used in an XML context.
    abbr_name: Option<SmartString>,
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
            assert!(dict.datatypes().len() >= 1);
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
