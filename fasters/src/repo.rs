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
/// println!("{}", readme);
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
/// let schema = RepoV2010::fields(Version::Fix42);
/// println!("{}", schema);
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

/// Basic data structures that can interface with the FIX repository via Serde.
/// They are mostly one-to-one mappings from `repositorystructures.xsd` and
/// `repositorytypes.xsd`.
pub mod types {
    pub use crate::Version;
    use serde::Deserialize;

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
        /// A positive integer representing the unique identifier for this field
        /// type.
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

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Abbreviation {}

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Section {}

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Category {}

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
        /// A human readable string representing the name of the field.
        #[serde(rename = "CategoryID")]
        pub category_id: String,
        #[serde(rename = "SectionID")]
        pub section_id: String,
        #[serde(rename = "NotReqXML")]
        pub required: String,
        pub description: String,
        pub elaboration: Option<String>,
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
        name: String,
        #[serde(rename = "field", default)]
        fields: Vec<Field>,
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
