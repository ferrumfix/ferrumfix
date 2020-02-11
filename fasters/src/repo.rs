//! Provides programmatic access to the FIX Repository.

use crate::Version;
use quick_xml::de::from_str;
use rust_embed::RustEmbed;
use serde::de::DeserializeOwned;
use std::borrow::Borrow;

/// FIX Repository 2010 Edition package for the latest published FIX 5.0 SP2
/// Extension Pack: EP254.
///
/// # Examples
///
/// ```
/// use fasters::repo::RepoV50SP2EP254;
///
/// let readme = RepoV50SP2EP254::get("Readme.html").unwrap();
/// println!("{:?}", readme);
/// ```
#[cfg(feature = "repo_v50sp2ep254")]
#[derive(RustEmbed)]
#[folder = "resources/repositories/FIXRepository_FIX.5.0SP2_EP254"]
pub struct RepoV50SP2EP254;
#[cfg(not(feature = "repo_v50sp2ep254"))]
#[derive(RustEmbed)]
#[folder = "resources/empty"]
pub struct RepoV50SP2EP254;

/// FIX Repository 2010 Edition.
///
/// # Examples
///
/// ```
/// use fasters::repo::RepoV2010;
/// use fasters::Version;
///
/// let schema = RepoV2010::fields(Version::Fix42);
/// println!("{:?}", schema);
/// ```
#[cfg(feature = "repo_v2010")]
#[derive(RustEmbed)]
#[folder = "resources/repositories/fix_repository_2010_edition_20140507"]
pub struct RepoV2010;
#[cfg(not(feature = "repo_v2010"))]
#[derive(RustEmbed)]
#[folder = "resources/empty"]
pub struct RepoV2010;

impl RepoV2010 {
    fn get_by_version<T: DeserializeOwned>(version: Version, filename: &str) -> T {
        let mut path = version.as_str().to_string();
        path.push_str("/Base/");
        path.push_str(filename);
        let bytes = RepoV2010::get(path.as_str()).unwrap();
        let xml = std::str::from_utf8(bytes.borrow()).unwrap();
        from_str(xml).unwrap()
    }

    pub fn components(version: Version) -> types::Components {
        Self::get_by_version(version, "Components.xml")
    }

    pub fn data_types(version: Version) -> types::Datatypes {
        Self::get_by_version(version, "Datatypes.xml")
    }

    pub fn enums(version: Version) -> types::Enums {
        Self::get_by_version(version, "Enums.xml")
    }

    pub fn fields(version: Version) -> types::Fields {
        Self::get_by_version(version, "Fields.xml")
    }

    pub fn messages(version: Version) -> types::Messages {
        Self::get_by_version(version, "Messages.xml")
    }

    pub fn msg_contents(version: Version) -> types::MsgContents {
        Self::get_by_version(version, "MsgContents.xml")
    }

    // ONLY FIX 4.4 AND UP
    // -------------------

    pub fn abbreviations(version: Version) -> types::Abbreviations {
        Self::get_by_version(version, "Abbreviations.xml")
    }

    pub fn categories(version: Version) -> types::Categories {
        Self::get_by_version(version, "Categories.xml")
    }

    pub fn sections(version: Version) -> types::Sections {
        Self::get_by_version(version, "Sections.xml")
    }
}

/// Fundamental data structures that map to resources within the FIX Repository via
/// Serde. They are intended to be exact, lossless representations of data
/// described by `repositorystructures.xsd` and `repositorytypes.xsd`.
pub mod types {
    pub use crate::Version;
    use serde::Deserialize;

    /// Signals the presence of non-schema attribute `ep:PK="1"`.
    pub trait HasPrimaryKey {
        type Pk;

        /// Get a copy of the primary key.
        fn pk(&self) -> Self::Pk;
    }

    /// Available versions of the FIX standard.
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct FixVersion(Version);

    /// A field is identified by a unique tag number and a name. Each field in a
    /// message is associated with a value.
    #[derive(Clone, Debug, Default, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Field {
        /// A human readable string representing the name of the field.
        pub name: String,
        /// **Primary key.** A positive integer representing the unique
        /// identifier for this field type.
        pub tag: usize,
        /// The datatype of the field.
        #[serde(rename = "Type")]
        pub data_type: String,
        /// The associated data field. If given, this field represents the length of
        /// the referenced data field
        pub associated_data_tag: Option<usize>,
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
        #[serde(rename = "NotReqXML")]
        pub required: bool,
        pub description: Option<String>,
    }

    impl HasPrimaryKey for Field {
        type Pk = usize;

        fn pk(&self) -> Self::Pk {
            self.tag
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Abbreviation {}

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Section {}

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Category {
        /// **Primary key**. A string uniquely identifying this category.
        #[serde(rename = "CategoryID")]
        pub id: String,
        /// The FIXML file name for a Category.
        #[serde(rename = "FIXMLFileName")]
        pub fixml_filename: String,
    }

    impl HasPrimaryKey for Category {
        type Pk = String;

        fn pk(&self) -> Self::Pk {
            self.id.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Datatype {
        /// A human readable string representing the name of the field.
        name: String,
        base_type: Option<String>,
        description: String,
        examples: Vec<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Message {
        /// The unique integer identifier of this message type.
        #[serde(rename = "ComponentID")]
        pub component_id: usize,
        /// **Primary key**. The unique character identifier of this message
        /// type; used literally in FIX messages.
        pub msg_type: String,
        /// The name of this message type
        pub name: String,
        /// Identifier of the category to which this message belongs.
        #[serde(rename = "CategoryID")]
        pub category_id: <Category as HasPrimaryKey>::Pk,
        /// Identifier of the section to which this message belongs.
        #[serde(rename = "SectionID")]
        pub section_id: String,
        /// The abbreviated name of this message, when used in an XML context.
        pub abbr_name: Option<String>,
        /// A boolean used to indicate if the message is to be generated as part
        /// of FIXML.
        #[serde(rename = "NotReqXML")]
        pub required: String,
        pub description: String,
        pub elaboration: Option<String>,
    }

    impl HasPrimaryKey for Message {
        type Pk = String;

        fn pk(&self) -> Self::Pk {
            self.msg_type.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct MsgContent {
        #[serde(rename = "ComponentId")]
        pub component_id: String,
        pub tag_text: String,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct FieldRef {
        name: String,
        required: char,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Value {
        value_enum: String,
        description: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Component {
        /// **Primary key.** The unique integer identifier of this component
        /// type.
        #[serde(rename = "ComponentID")]
        id: usize,
        /// **Primary key.** The unique integer identifier of this component
        /// type.
        #[serde(rename = "CategoryID")]
        category_id: usize,
        /// The human readable name of the component.
        name: String,
        #[serde(rename = "field", default)]
        fields: Vec<Field>,
    }

    impl HasPrimaryKey for Component {
        type Pk = usize;

        fn pk(&self) -> Self::Pk {
            self.id
        }
    }

    // CONTAINERS (plural format)
    // --------------------------

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Fields {
        #[serde(rename = "Field", default)]
        pub fields: Vec<Field>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Enums {
        #[serde(rename = "Enum", default)]
        pub enums: Vec<Enums>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Abbreviations {
        #[serde(rename = "Abbreviation", default)]
        data: Vec<Abbreviation>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Categories {
        #[serde(rename = "Category", default)]
        data: Vec<Category>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Components {
        name: String,
        #[serde(rename = "Component", default)]
        data: Vec<Component>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Datatypes {
        name: String,
        #[serde(rename = "Component", default)]
        data: Vec<Component>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Messages {
        #[serde(rename = "Message", default)]
        pub data: Vec<Message>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct MsgContents {
        #[serde(rename = "MsgContent", default)]
        pub data: Vec<MsgContent>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Sections {
        #[serde(rename = "Section", default)]
        pub data: Vec<Section>,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Version;

    #[test]
    fn repo_v50sp2ep254_readme() {
        let readme = RepoV50SP2EP254::get("Readme.html");
        assert!(readme.is_some());
        assert!(readme.unwrap().len() > 42);
    }

    #[test]
    fn repo_v2010_deserialize_fields() {
        for v in Version::iter_supported() {
            RepoV2010::fields(v);
        }
    }

    #[test]
    fn repo_v2010_deserialize_enums() {
        for v in Version::iter_supported() {
            RepoV2010::enums(v);
        }
    }

    #[test]
    fn repo_v2010_deserialize_messages() {
        for v in Version::iter_supported() {
            RepoV2010::messages(v);
        }
    }
}
