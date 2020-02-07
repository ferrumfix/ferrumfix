//! Provides programmatic access to the FIX Repository.

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
/// let schema = RepoV2010::get("schema/Fields.xsd").unwrap();
/// println!("{}", schema);
/// ```
#[cfg(feature = "repo_v2010")]
#[derive(RustEmbed)]
#[folder = "resources/repositories/fix_repository_2010_edition_20140507"]
pub struct RepoV2010;

impl RepoV2010 {
    fn get_by_version<T: DeserializeOwned>(version: types::FixVersion, filename: &str) -> T {
        let mut path = version.to_str().to_string();
        path.push_str("/Base/");
        path.push_str(filename);
        let bytes = RepoV2010::get(path.as_str()).unwrap();
        let xml = std::str::from_utf8(bytes.borrow()).unwrap();
        from_str(xml).unwrap()
    }

    pub fn components(version: types::FixVersion) -> types::Components {
        Self::get_by_version(version, "Components.xml")
    }

    pub fn data_types(version: types::FixVersion) -> types::Datatypes {
        Self::get_by_version(version, "Datatypes.xml")
    }

    pub fn enums(version: types::FixVersion) -> types::Enums {
        Self::get_by_version(version, "Enums.xml")
    }

    pub fn fields(version: types::FixVersion) -> types::Fields {
        Self::get_by_version(version, "Fields.xml")
    }

    pub fn messages(version: types::FixVersion) -> types::Messages {
        Self::get_by_version(version, "Messages.xml")
    }

    // ONLY FIX 4.4 AND UP
    // -------------------

    pub fn abbreviations(version: types::FixVersion) -> types::Abbreviations {
        Self::get_by_version(version, "Abbreviations.xml")
    }

    pub fn categories(version: types::FixVersion) -> types::Categories {
        Self::get_by_version(version, "Categories.xml")
    }

    pub fn sections(version: types::FixVersion) -> types::Sections {
        Self::get_by_version(version, "Sections.xml")
    }
}

#[cfg(not(feature = "repo_v2010"))]
#[derive(RustEmbed)]
#[folder = "resources/empty"]
pub struct RepoV2010;

/// Basic data structures that can interface with the FIX repository via Serde.
/// They are mostly one-to-one mappings from `repositorystructures.xsd` and
/// `repositorytypes.xsd`.
mod types {
    use serde::Deserialize;

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub enum FixVersion {
        #[serde(rename = "FIX.2.7")]
        Fix27,
        #[serde(rename = "FIX.3.0")]
        Fix30,
        #[serde(rename = "FIX.4.0")]
        Fix40,
        #[serde(rename = "FIX.4.1")]
        Fix41,
        #[serde(rename = "FIX.4.2")]
        Fix42,
        #[serde(rename = "FIX.4.3")]
        Fix43,
        #[serde(rename = "FIX.4.4")]
        Fix44,
        #[serde(rename = "FIX.5.0")]
        Fix50,
        #[serde(rename = "FIX.5.0SP1")]
        Fix50SP1,
        #[serde(rename = "FIX.5.0SP2")]
        Fix50SP2,
        #[serde(rename = "FIXT.1.1")]
        Fixt11,
    }

    impl FixVersion {
        pub fn all() -> impl Iterator<Item = &'static &'static str> {
            [
                "FIX.4.0",
                "FIX.4.1",
                "FIX.4.2",
                "FIX.4.3",
                "FIX.4.4",
                "FIX.5.0",
                "FIX.5.0SP1",
                "FIX.5.0SP2",
                "FIXT.1.1",
            ]
            .iter()
        }

        pub fn to_str(&self) -> &'static str {
            match self {
                Self::Fix27 => "FIX.2.7",
                Self::Fix30 => "FIX.3.0",
                Self::Fix40 => "FIX.4.0",
                Self::Fix41 => "FIX.4.1",
                Self::Fix42 => "FIX.4.2",
                Self::Fix43 => "FIX.4.3",
                Self::Fix44 => "FIX.4.4",
                Self::Fix50 => "FIX.5.0",
                Self::Fix50SP1 => "FIX.5.0SP1",
                Self::Fix50SP2 => "FIX.5.0SP2",
                Self::Fixt11 => "FIXT.1.1",
            }
        }
    }

    /// A field is identified by a unique tag number and a name. Each field in a
    /// message is associated with a value.
    #[derive(Clone, Debug, Default, Deserialize, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    pub struct Field {
        /// A human readable string representing the name of the field.
        name: String,
        /// A positive integer representing the unique identifier for this field
        /// type.
        tag: usize,
        /// The datatype of the field.
        #[serde(rename = "Type")]
        data_type: String,
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
        #[serde(rename = "NotReqXML")]
        required: bool,
        description: Option<String>,
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
        category_id: String,
        #[serde(rename = "SectionID")]
        section_id: String,
        #[serde(rename = "NotReqXML")]
        required: String,
        description: String,
        elaboration: String,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Fields {
        #[serde(rename = "Field", default)]
        fields: Vec<Field>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Enums {
        #[serde(rename = "Enum", default)]
        enums: Vec<Enums>,
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
        data: Vec<Message>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    pub struct Sections {
        #[serde(rename = "Section", default)]
        data: Vec<Section>,
    }
}

#[cfg(test)]
mod test {
    use super::types::*;
    use super::*;
    use quick_xml::de::from_str;
    use serde::de::DeserializeOwned;
    use std::borrow::Borrow;

    fn deserialize_v2010<T: DeserializeOwned>(filename: &'static str) {
        for version in FixVersion::all() {
            let mut path = (*version).to_string();
            path.push_str("/Base/");
            path.push_str(filename);
            let bytes = RepoV2010::get(path.as_str()).unwrap();
            let xml = std::str::from_utf8(bytes.borrow()).unwrap();
            from_str::<T>(xml).unwrap();
        }
    }

    #[test]
    fn repo_v2010_schema_enums() {
        let schema = RepoV2010::get("schema/Enums.xsd");
        assert!(schema.is_some());
    }

    #[test]
    fn repo_v50sp2ep254_readme() {
        let readme = RepoV50SP2EP254::get("Readme.html");
        assert!(readme.is_some());
        assert!(readme.unwrap().len() > 42);
    }

    #[test]
    fn repo_v2010_deserialize_fields() {
        deserialize_v2010::<Fields>("Fields.xml");
    }

    #[test]
    fn repo_v2010_deserialize_enums() {
        deserialize_v2010::<Enums>("Enums.xml");
    }

    #[test]
    fn repo_v2010_deserialize_messages() {
        deserialize_v2010::<Messages>("Messages.xml");
    }
}
