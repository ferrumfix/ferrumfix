use crate::repo::{types as t, HasPk, HashMapPk, RepoV2010};
use crate::Version;
use codegen::Scope;
use inflector::Inflector;
use std::collections::HashMap;

pub fn codegen(dict: Dictionary) -> String {
    let mut scope = Scope::new();
    for field in dict.fields.values() {
        let structure = scope
            .get_or_new_module("fields")
            .new_struct(field.name.as_str())
            .vis("pub");
        if let Some(description) = &field.description {
            let onixs_link = field.doc_url_onixs(dict.version);
            let docstring = format!(
                "{}\n\n# Field information\n\nTag Number: {}\nOnixS [reference]({}).",
                description,
                field.tag.to_string().as_str(),
                onixs_link.as_str()
            );
            structure.doc(docstring.as_str());
        }
    }
    for (msg_type, msg) in &dict.messages {
        let mut structure = scope
            .get_or_new_module("messages")
            .new_struct(msg.name.as_str())
            .vis("pub")
            .doc(format!("# Message information:\n\nMessage type: {}", msg_type).as_str());
        if let Some(msg_contents) = dict.msg_contents.get(&msg.component_id) {
            let mut i = 1;
            for content in msg_contents {
                structure = structure.field(
                    format!("tag_{}_{}", i, &content.tag_text.to_snake_case()).as_str(),
                    "foobar",
                );
                i += 1;
            }
        }
    }
    scope.to_string()
}

/// Allows lookup of FIX definitions based on `RepoV2010`.
#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    version: Version,
    data_types: HashMapPk<t::Datatype, t::Datatype>,
    fields: HashMapPk<t::Field, t::Field>,
    components: HashMapPk<t::Component, t::Component>,
    messages: HashMapPk<t::Message, t::Message>,
    msg_contents: HashMapPk<t::Component, Vec<t::MsgContent>>,
}

impl Dictionary {
    /// Assembles a FIX dictionary by linking FIX definitions against each other.
    pub fn new(version: Version) -> Self {
        let data_types = RepoV2010::data_types(version)
            .map(|dt| (dt.pk(), dt))
            .collect();
        let fields = RepoV2010::fields(version).map(|f| (f.pk(), f)).collect();
        let components = RepoV2010::components(version)
            .map(|c| (c.pk(), c))
            .collect();
        let messages = RepoV2010::messages(version).map(|m| (m.pk(), m)).collect();
        let mut msg_contents: HashMapPk<t::Component, Vec<t::MsgContent>> = HashMap::new();
        for msg_content in RepoV2010::msg_contents(version) {
            let key = msg_content.component_id;
            if let Some(components) = msg_contents.get_mut(&key) {
                components.push(msg_content)
            } else {
                msg_contents.insert(key, vec![msg_content]);
            }
        }
        Self {
            version,
            data_types,
            fields,
            messages,
            msg_contents,
            components,
        }
    }

    pub fn get_field(&self, pk: <t::Field as HasPk>::Pk) -> Option<&t::Field> {
        self.fields.get(&pk)
    }

    pub fn get_data_type(&self, pk: <t::Datatype as HasPk>::Pk) -> Option<&t::Datatype> {
        self.data_types.get(&pk)
    }

    pub fn get_component(&self, pk: <t::Component as HasPk>::Pk) -> Option<&t::Component> {
        self.components.get(&pk)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[cfg(feature = "repo_v2010")]
    #[test]
    fn fix40_fields_has_at_least_100_fields() {
        use super::*;
        let dict = Dictionary::new(Version::Fix40);
        assert!(dict.fields.len() >= 100);
    }

    #[test]
    fn codegen_44_doesnt_panic() {
        let dict = Dictionary::new(Version::Fix44);
        codegen(dict);
    }
}
