use crate::repo::{types as t, HasPk, HashMapPk, RepoV2010};
use crate::{Settings, Version};
use codegen::Scope;
use inflector::Inflector;
use std::collections::HashMap;

pub fn codegen(dict: Dictionary) -> String {
    codegen_with_settings(dict, Settings::default())
}

pub fn field_docstring(version: Version, field: &t::Field) -> String {
    let onixs_link = field.doc_url_onixs(version);
    format!(
        "{}\n\n# Field information\n\nTag Number: {}\nOnixS [reference]({}).",
        field.description.as_ref().unwrap_or(&String::new()),
        field.tag.to_string().as_str(),
        onixs_link.as_str()
    )
}

pub fn codegen_with_settings(dict: Dictionary, settings: Settings) -> String {
    let mut scope = Scope::new();
    scope.raw("#![allow(dead_code)]");
    scope.new_module("components").import("super", "*");
    scope.new_module("messages").import("super", "*");
    //dict.fields.values().for_each(|field| {
    //    let structure = scope
    //        .get_or_new_module("fields")
    //        .new_struct(field.name.as_str())
    //        .vis("pub");
    //    let data_type = dict.get_data_type(&field.data_type).unwrap().clone();
    //    let docstring = field_docstring(dict.version, &field);
    //    structure.doc(docstring.as_str());
    //    structure.field("data", (settings.typer)(data_type));
    //});
    dict.components.values().for_each(|component| {
        scope
            .get_or_new_module("components")
            .push_struct(dict.build_component_struct(component, &settings));
    });
    dict.messages.keys().for_each(|pk| {
        scope
            .get_or_new_module("messages")
            .push_struct(dict.build_message_struct(pk, &settings));
    });
    scope.to_string()
}

fn optionify_type<S: AsRef<str>>(required: bool, t: S) -> String {
    if required {
        t.as_ref().to_string()
    } else {
        format!("Option<{}>", t.as_ref())
    }
}

/// Allows lookup of FIX definitions based on `RepoV2010`.
#[derive(Clone, Debug, PartialEq)]
pub struct Dictionary {
    version: Version,
    data_types: HashMapPk<t::Datatype, t::Datatype>,
    fields: HashMapPk<t::Field, t::Field>,
    components: HashMap<String, t::Component>,
    messages: HashMapPk<t::Message, t::Message>,
    msg_contents: HashMapPk<t::Component, Vec<t::MsgContent>>,
}

impl Dictionary {
    /// Assembles a FIX dictionary by linking FIX definitions against each other.
    pub fn new(version: Version) -> Self {
        let mut data_types: HashMap<_, _> = RepoV2010::data_types(version)
            .map(|dt| (dt.pk(), dt))
            .collect();
        data_types.insert(
            "MultipleValueString".to_string(),
            t::Datatype {
                name: "MultipleValueString".to_string(),
                base_type: Some("String".to_string()),
                description: String::new(),
                examples: vec![],
            },
        );
        data_types.insert(
            "SeqNum".to_string(),
            t::Datatype {
                name: "SeqNum".to_string(),
                base_type: Some("int".to_string()),
                description: String::new(),
                examples: vec![],
            },
        );
        println!("ALL DATATYPES ARE {:?}", data_types);
        let fields = RepoV2010::fields(version).map(|f| (f.pk(), f)).collect();
        let components = RepoV2010::components(version)
            .map(|c| (c.name.clone(), c))
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

    pub fn get_component(&self, name: &str) -> Option<&t::Component> {
        self.components.get(name)
    }

    pub fn get_data_type(&self, pk: &<t::Datatype as HasPk>::Pk) -> Option<&t::Datatype> {
        self.data_types.get(pk)
    }

    pub fn get_field(&self, pk: &<t::Field as HasPk>::Pk) -> Option<&t::Field> {
        self.fields.get(pk)
    }

    pub fn get_message(&self, pk: &<t::Message as HasPk>::Pk) -> Option<&t::Message> {
        self.messages.get(pk)
    }

    pub fn get_msg_contents(
        &self,
        pk: &<t::Component as HasPk>::Pk,
    ) -> Option<&Vec<t::MsgContent>> {
        self.msg_contents.get(pk)
    }

    fn build_message_struct(
        &self,
        pk: &<t::Message as HasPk>::Pk,
        settings: &Settings,
    ) -> codegen::Struct {
        let message = self.get_message(pk).unwrap();
        let msg_contents = self.get_msg_contents(&message.component_id).unwrap();
        let mut structure = codegen::Struct::new(message.name.as_str());
        structure.vis("pub").doc(
            format!(
                "# Message information:\n\nMessage type: {}",
                message.msg_type
            )
            .as_str(),
        );
        for content in msg_contents.iter() {
            let (field_name, field_type) =
                self.translate_msg_content_to_struct_field(content, settings);
            (&mut structure).field(
                field_name.as_str(),
                optionify_type(content.reqd == '1', field_type.as_str()),
            );
        }
        structure
    }

    fn build_component_struct(
        &self,
        component: &t::Component,
        settings: &Settings,
    ) -> codegen::Struct {
        let msg_contents = self.get_msg_contents(&component.id).unwrap();
        let mut structure = codegen::Struct::new(component.name.as_str());
        structure.vis("pub");
        for content in msg_contents.iter() {
            let (field_name, field_type) =
                self.translate_msg_content_to_struct_field(content, settings);
            (&mut structure).field(
                field_name.as_str(),
                optionify_type(content.reqd == '1', field_type.as_str()),
            );
        }
        structure
    }

    fn translate_msg_content_to_struct_field(
        &self,
        content: &t::MsgContent,
        settings: &Settings,
    ) -> (String, String) {
        let tag_number_res = content.tag_text.parse::<usize>();
        if let Ok(tag_number) = tag_number_res {
            let field = self.get_field(&tag_number).unwrap();
            let data_type = self.get_data_type(&field.data_type).unwrap().clone();
            (
                format!("t_{}", &field.name.to_snake_case()),
                (settings.typer)(data_type).to_string(),
            )
        } else {
            let component = self.get_component(content.tag_text.as_str()).unwrap();
            (
                format!("c_{}", content.tag_text.to_snake_case()),
                format!("components::{}", component.name),
            )
        }
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
        let _code = codegen(dict);
        println!("{}", _code);
    }
}
