use crate::dictionary::Datatype;
use crate::dictionary::{Component, Field, MsgContent};
use crate::Dictionary;
use codegen::Scope;
use inflector::Inflector;

pub fn field_docstring(version: String, field: &Field) -> String {
    let onixs_link = field.doc_url_onixs(version.as_str());
    String::new()
    //format!(
    //    "{}\n\n# Field information\n\nTag Number: {}\nOnixS [reference]({}).",
    //    (*field).description().as_ref().unwrap_or(&String::new()),
    //    field.tag.to_string().as_str(),
    //    onixs_link.as_str()
    //)
}

pub fn codegen(dict: &Dictionary) -> String {
    let mut scope = Scope::new();
    scope.raw("#![allow(dead_code)]");
    scope
        .new_module("components")
        .import("super", "*")
        .import("fasters_internals", "*")
        .import("std::io", "Write");
    scope
        .new_module("messages")
        .import("super", "*")
        .import("fasters_internals", "*")
        .import("std::io", "Write");
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
    //dict.components.values().for_each(|component| {
    //    scope
    //        .get_or_new_module("components")
    //        .push_struct(dict.build_component_struct(component));
    //});
    dict.messages().for_each(|msg| {
        scope
            .get_or_new_module("messages")
            .push_struct(dict.build_message_struct(msg.msg_type()));
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

impl Dictionary {
    fn build_message_struct(&self, msg_type: &str) -> codegen::Struct {
        let message = self.get_message_by_msg_type(msg_type).unwrap();
        let msg_contents = self.get_msg_content(message.msg_type()).unwrap();
        let mut structure = codegen::Struct::new(message.name());
        structure
            .vis("pub")
            .doc(format!("# Message information:\n\nMessage type: {}", message.name()).as_str());
        for content in msg_contents.iter() {
            let (field_name, field_type) = self.translate_msg_content_to_struct_field(content);
            (&mut structure).field(
                field_name.as_str(),
                optionify_type(content.reqd == '1', field_type.as_str()),
            );
        }
        structure
    }

    fn build_component_struct(&self, component: &Component, msg_type: &str) -> codegen::Struct {
        println!("FETCHING MSG CONTENTS OF C. WITH ID {}", component.id());
        let msg_contents = self.get_msg_content(msg_type).unwrap();
        let mut structure = codegen::Struct::new(component.name());
        structure.vis("pub");
        for content in msg_contents.iter() {
            let (field_name, field_type) = self.translate_msg_content_to_struct_field(content);
            (&mut structure).field(
                field_name.as_str(),
                optionify_type(content.reqd == '1', field_type.as_str()),
            );
        }
        structure
    }

    fn translate_msg_content_to_struct_field(&self, content: &MsgContent) -> (String, String) {
        let tag_number_res = content.tag_text.parse::<usize>();
        if let Ok(tag_number) = tag_number_res {
            let field = self.get_field(tag_number as u32).unwrap();
            println!("TRYING DATATYPE {}", field.data_type());
            let data_type = self.get_datatype_of_field(field.tag());
            (
                format!("t_{}", &field.name().to_snake_case()),
                type_to_str(data_type).to_string(),
            )
        } else {
            let component = self.get_component(content.tag_text.as_str()).unwrap();
            (
                format!("c_{}", content.tag_text.to_snake_case()),
                format!("components::{}", component.name()),
            )
        }
    }
}

fn type_to_str(datatype: &Datatype) -> &'static str {
    match datatype.name.as_str() {
        "int" => "u32",
        "string" => "char",
        "decimal" => "f32",
        "data" => "Vec<u8>",
        "string" => "String",
        _ => "String",
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
