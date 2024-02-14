use crate::{Dictionary, FixDatatype, FixmlComponentAttributes};
use smartstring::alias::String as SmartString;

/// A public API for building a [`Dictionary`] from scratch.
///
/// It's better to normally rely on safe wrappers around this builder, such as
/// [`Dictionary::from_quickfix_spec`]. This builder is nonetheless useful if
/// you're building a [`Dictionary`] from some custom format.
///
/// # Warning
/// [`DictionaryBuilder`] tries its best to enforce data consistency and
/// correctness rules for the dictionaries it builds, but you shouldn't rely on
/// it to catch all possible errors. Make sure the dictionaries you build are
/// correct and consistent.
pub struct DictionaryBuilder {
    dict: Dictionary,
}

/// Builder utilities
impl DictionaryBuilder {
    pub fn new(dict: Dictionary) -> Self {
        DictionaryBuilder { dict }
    }

    /// Returns the underlying [`Dictionary`] currently being built.
    pub fn dict(&self) -> &Dictionary {
        &self.dict
    }

    /// Returns the finished [`Dictionary`].
    pub fn build(self) -> Dictionary {
        self.dict
    }

    pub fn add_field(&mut self, field: FieldData) {
        self.dict
            .field_tags_by_name
            .insert(field.name.clone(), field.tag);
        self.dict.fields_by_tags.insert(field.tag, field);
    }

    pub fn add_message(&mut self, message: MessageData) {
        self.dict
            .message_msgtypes_by_name
            .insert(message.name.clone(), message.msg_type.clone());
        self.dict
            .messages_by_msgtype
            .insert(message.msg_type.clone(), message);
    }

    pub fn add_component(&mut self, component: ComponentData) {
        self.dict
            .components_by_name
            .insert(component.name.clone(), component);
    }

    pub fn add_datatype(&mut self, datatype: DatatypeData) {
        self.dict
            .data_types_by_name
            .insert(datatype.datatype.name().into(), datatype);
    }

    pub fn add_category(&mut self, category: CategoryData) {
        self.dict
            .categories_by_name
            .insert(category.name.clone().into(), category);
    }
}

/// A field is identified by a unique tag number and a name. Each field in a
/// message is associated with a value.
#[derive(Clone, Debug)]
pub struct FieldData {
    /// A human readable string representing the name of the field.
    pub name: SmartString,
    /// **Primary key.** A positive integer representing the unique
    /// identifier for this field type.
    pub tag: u32,
    /// The datatype of the field.
    pub data_type_name: SmartString,
    /// The associated data field. If given, this field represents the length of
    /// the referenced data field
    pub associated_data_tag: Option<usize>,
    pub value_restrictions: Option<Vec<FieldEnumData>>,
    /// Abbreviated form of the Name, typically to specify the element name when
    /// the field is used in an XML message. Can be overridden by BaseCategory /
    /// BaseCategoryAbbrName.
    pub abbr_name: Option<String>,
    /// Specifies the base message category when field is used in an XML message.
    pub base_category_id: Option<usize>,
    /// If BaseCategory is specified, this is the XML element identifier to use
    /// for this field, overriding AbbrName.
    pub base_category_abbr_name: Option<String>,
    /// Indicates whether the field is required in an XML message.
    pub required: bool,
    pub description: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CategoryData {
    /// **Primary key**. A string uniquely identifying this category.
    pub name: String,
    /// The FIXML file name for a Category.
    pub fixml_filename: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DatatypeData {
    /// **Primary key.** Identifier of the datatype.
    pub datatype: FixDatatype,
    /// Human readable description of this Datatype.
    pub description: String,
    /// A string that contains examples values for a datatype
    pub examples: Vec<String>,
    // TODO: 'XML'.
}

#[derive(Clone, Debug)]
pub struct AbbreviationData {
    pub abbreviation: SmartString,
    pub is_last: bool,
}

#[derive(Clone, Debug)]
pub struct ComponentData {
    /// **Primary key.** The unique integer identifier of this component
    /// type.
    pub id: usize,
    pub component_type: FixmlComponentAttributes,
    pub layout_items: Vec<LayoutItemData>,
    pub category_name: SmartString,
    /// The human readable name of the component.
    pub name: SmartString,
    /// The name for this component when used in an XML context.
    pub abbr_name: Option<SmartString>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum LayoutItemKindData {
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
pub struct LayoutItemData {
    pub required: bool,
    pub kind: LayoutItemKindData,
}

#[derive(Clone, Debug)]
pub struct MessageData {
    /// The unique integer identifier of this message type.
    pub component_id: u32,
    /// **Primary key**. The unique character identifier of this message
    /// type; used literally in FIX messages.
    pub msg_type: SmartString,
    /// The name of this message type.
    pub name: SmartString,
    /// Identifier of the category to which this message belongs.
    pub category_name: SmartString,
    /// Identifier of the section to which this message belongs.
    pub section_id: String,
    pub layout_items: Vec<LayoutItemData>,
    /// The abbreviated name of this message, when used in an XML context.
    pub abbr_name: Option<SmartString>,
    /// A boolean used to indicate if the message is to be generated as part
    /// of FIXML.
    pub required: bool,
    pub description: String,
    pub elaboration: Option<String>,
}

#[derive(Clone, Debug)]
pub struct FieldEnumData {
    pub value: String,
    pub description: String,
}
