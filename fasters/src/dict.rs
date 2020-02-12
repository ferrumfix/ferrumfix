use crate::repo::{types as t, HasPk, HashMapPk, RepoV2010};
use crate::Version;
use codegen::Scope;

pub fn codegen(dict: Dictionary) -> String {
    let mut scope = Scope::new();
    for (_, field) in &dict.fields {
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
    for (name, msg) in &dict.messages {
        let structure = scope
            .get_or_new_module("messages")
            .new_struct(name.as_str())
            .vis("pub");
        if let Some(component) = dict.get_component(msg.component_id) {
            //
        } else {
            println!("BAD COMPONENT IN MSG {}", name);
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
    msg_contents: HashMapPk<t::Component, t::MsgContent>,
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
        let msg_contents = RepoV2010::msg_contents(version)
            .map(|mc| (mc.component_id, mc))
            .collect();
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
    #[cfg(feature = "repo_v2010")]
    #[test]
    fn fix40_fields_has_at_least_100_fields() {
        use super::*;
        let dict = Dictionary::new(Version::Fix40);
        assert!(dict.fields.len() >= 100);
    }

    #[test]
    fn print_dict() {
        use super::*;
        let dict = Dictionary::new(Version::Fix40);
        println!("{}", codegen(dict));
    }
}
