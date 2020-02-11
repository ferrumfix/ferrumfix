use crate::repo::types;
use crate::repo::HasPk;
use crate::repo::RepoV2010;
use crate::Version;
use codegen::Scope;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
struct Message {
    def: types::Message,
    contents: Vec<String>,
}

pub fn codegen(dict: Dictionary) -> String {
    let mut scope = Scope::new();
    for (_, field) in dict.fields {
        let structure = scope
            .get_or_new_module("fields")
            .new_struct(field.name.as_str())
            .vis("pub");
        if let Some(description) = field.description {
            structure.doc(description.as_str());
        }
    }
    //for (name, message) in dict.messages {
    //    let structure = scope
    //        .get_or_new_module("messages")
    //        .new_struct(name.as_str())
    //        .vis("pub");
    //    for content in dict.contents[&message.pk()] {}
    //    structure.doc(message.description.as_str());
    //}
    scope.to_string()
}

#[derive(Clone, Debug, PartialEq)]
struct MessageDefinition {
    def: types::Message,
    fields: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    fields: HashMap<<types::Field as HasPk>::Pk, types::Field>,
    messages: HashMap<<types::Message as HasPk>::Pk, MessageDefinition>,
}

impl Dictionary {
    fn new(version: Version) -> Self {
        let fields = RepoV2010::fields(version).map(|f| (f.pk(), f)).collect();
        let mut message_definitions: HashMap<_, _> = RepoV2010::messages(version)
            .map(|m| {
                (
                    m.pk(),
                    MessageDefinition {
                        def: m,
                        fields: vec![],
                    },
                )
            })
            .collect();
        let msg_contents: HashMap<_, _> = RepoV2010::msg_contents(version)
            .map(|mc| (mc.component_id.clone(), mc))
            .collect();
        for (component_id, mc) in msg_contents.into_iter() {
            // TODO: also unpack components.
            // Not all components belong to a message.
            if let Some(def) = message_definitions.get_mut(&component_id) {
                def.fields.push(mc.tag_text);
            }
        }
        Self {
            fields,
            messages: message_definitions,
        }
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
