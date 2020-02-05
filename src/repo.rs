//! Provides programmatic access to the FIX Repository.

use rust_embed::RustEmbed;
use serde::Deserialize;

#[derive(RustEmbed)]
#[folder = "resources/"]
pub struct Repository;

impl Repository {
    #[cfg(repo_v50SP2EP254)]
    pub fn v50_sp2_ep254() -> impl RustEmbed {
        #[derive(RustEmbed)]
        #[folder = "resources/repositories/FIXRepository_FIX.5.0SP2_EP254"]
        pub struct Repo;
        Repo
    }

    #[cfg(repo_v2010)]
    pub fn v2010_edition() -> impl RustEmbed {
        #[derive(RustEmbed)]
        #[folder = "resources/repositories/fix_repository_2010_edition_20140507"]
        pub struct Repo;
        Repo
    }
}

/// Basic data structures that can interface with the FIX repository via Serde.
/// They are mostly one-to-one mappings from `repositorystructures.xsd` and
/// `repositorytypes.xsd`.
mod types {

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    enum FixVersion {
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

    /// A field is identified by a unique tag number and a name. Each field in a
    /// message is associated with a value.
    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all("PascalCase"))]
    struct Field {
        /// A human readable string representing the name of the field.
        name: String,
        /// A positive integer representing the unique identifier for this field
        /// type.
        number: usize,
        /// The datatype of the field.
        data_type: String,
        /// The associated data field. If given, this field represents the length of
        /// the referenced data field
        associated_data_tag: usize,
        /// bbreviated form of the Name, typically to specify the element name when
        /// the field is used in an XML message. Can be overridden by BaseCategory /
        /// BaseCategoryAbbrName.
        abbr_name: String,
        /// Specifies the base message category when field is used in an XML message.
        base_category_id: usize,
        /// If BaseCategory is specified, this is the XML element identifier to use
        /// for this field, overriding AbbrName.
        base_category_abbr_name: String,
        /// Indicates whether the field is required in an XML message.
        #[serde(rename = "NotReqXML")]
        required: bool,
        #[serde(rename = "type")]
        field_type: String,
        description: String,
        // https://github.com/tafia/quick-xml#parsing-the-value-of-a-tag
        #[serde(rename = "$value")]
        values: Option<Vec<Value>>,
    }

    #[derive(Clone, Debug, Deserialize, PartialEq)]
    #[serde(rename_all("PascalCase"))]
    struct Message {
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
    struct Fields {
        name: String,
        required: char,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct FieldRef {
        name: String,
        required: char,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Value {
        value_enum: String,
        description: Option<String>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Message {
        name: String,
        msgtype: Option<String>,
        msgcat: Option<String>,
        #[serde(rename = "field", default)]
        fields: Vec<FieldRef>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Header {
        #[serde(rename = "field", default)]
        fields: Vec<FieldRef>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Trailer {
        #[serde(rename = "field", default)]
        fields: Vec<FieldRef>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct Component {
        name: String,
        #[serde(rename = "field", default)]
        fields: Vec<Field>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    #[serde(rename = "fix")]
    pub struct Dictionary {
        // Meta.
        major: usize,
        minor: usize,
        servicepack: usize,
        // Tag definitions.
        header: Header,
        trailer: Trailer,
        #[serde(rename = "message", default)]
        messages: Vec<Message>,
        #[serde(rename = "component", default)]
        components: Vec<Component>,
        #[serde(rename = "field", default)]
        fields: Vec<Field>,
    }
}
