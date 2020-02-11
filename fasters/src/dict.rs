use crate::repo::types;
use crate::repo::types::HasPk;
use crate::repo::RepoV2010;
use crate::Version;
use codegen::Scope;
use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone, Debug, PartialEq)]
struct Message {
    def: types::Message,
    contents: Vec<String>,
}

pub fn codegen(dict: Dictionary) -> String {
    let mut scope = Scope::new();
    for (_, field) in dict.fields {
        let structure = scope.new_struct(field.name.as_str()).vis("pub");
        if let Some(description) = field.description {
            structure.doc(description.as_str());
        }
    }
    for (name, message) in dict.messages {
        let structure = scope.new_struct(name.as_str()).vis("pub");
        structure.doc(message.description.as_str());
    }
    scope.to_string()
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    fields: HashMap<<types::Field as HasPk>::Pk, types::Field>,
    messages: HashMap<<types::Message as HasPk>::Pk, types::Message>,
    components: HashMap<<types::Component as HasPk>::Pk, types::Component>,
}

impl Dictionary {
    fn new(version: Version) -> Self {
        let fields = HashMap::from_iter(
            RepoV2010::fields(version)
                .data
                .into_iter()
                .map(|f| (f.pk(), f)),
        );
        let messages = HashMap::from_iter(
            RepoV2010::messages(version)
                .data
                .into_iter()
                .map(|m| (m.pk(), m)),
        );
        let components = HashMap::from_iter(
            RepoV2010::components(version)
                .data
                .into_iter()
                .map(|m| (m.pk(), m)),
        );
        Self {
            messages,
            fields,
            components,
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
