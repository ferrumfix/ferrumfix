use crate::repo::{types as t, HasPk, HashMapPk, RepoV2010};
use crate::Version;
use codegen::Scope;
use std::collections::HashMap;
use std::rc::Rc;

pub fn codegen(dict: Dictionary) -> String {
    let mut scope = Scope::new();
    for (_, field) in dict.fields {
        let structure = scope
            .get_or_new_module("fields")
            .new_struct(field.name.as_str())
            .vis("pub");
        //if let Some(description) = field.description {
        //    structure.doc(description.as_str());
        //}
    }
    //for (name, definition) in dict.messages {
    //    let structure = scope
    //        .get_or_new_module("messages")
    //        .new_struct(name.as_str())
    //        .vis("pub");
    //    for tag in definition.fields {
    //        // TODO
    //        //structure.field(tag.as_str(), "foobar");
    //    }
    //    structure.doc(definition.def.description.as_str());
    //}
    scope.to_string()
}

#[derive(Clone, Debug, PartialEq)]
struct MessageDefinition {
    def: t::Message,
    fields: Vec<t::Component>,
}

/// Allows lookup of FIX definitions based on `RepoV2010`.
#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    data_types: HashMapPk<t::Datatype, Rc<t::Datatype>>,
    fields: HashMapPk<t::Field, Rc<t::Field>>,
    components: HashMapPk<t::Component, Rc<t::Component>>,
    msg_contents: HashMapPk<t::Component, Rc<t::MsgContent>>,
}

impl Dictionary {
    /// Assembles a FIX dictionary by linking FIX definitions against each other.
    pub fn new(version: Version) -> Self {
        let data_types = RepoV2010::data_types(version)
            .map(|f| (f.pk(), Rc::new(f)))
            .collect();
        let fields = RepoV2010::fields(version)
            .map(|f| (f.pk(), Rc::new(f)))
            .collect();
        let components = RepoV2010::components(version)
            .map(|c| (c.pk(), Rc::new(c)))
            .collect();
        let msg_contents = RepoV2010::msg_contents(version)
            .map(|mc| (mc.component_id, Rc::new(mc)))
            .collect();
        Self {
            data_types,
            fields,
            msg_contents,
            components,
        }
    }

    pub fn get_field(&self, pk: <t::Field as HasPk>::Pk) -> Option<Rc<t::Field>> {
        self.fields.get(&pk).cloned()
    }

    pub fn get_data_type(&self, pk: <t::Datatype as HasPk>::Pk) -> Option<Rc<t::Datatype>> {
        self.data_types.get(&pk).cloned()
    }

    pub fn get_component(&self, pk: <t::Component as HasPk>::Pk) -> Option<Rc<t::Component>> {
        self.components.get(&pk).cloned()
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
