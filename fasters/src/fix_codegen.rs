use crate::dictionary::{Component, Datatype, Dictionary, Field, LayoutItem, LayoutItemKind};
use codegen::Scope;
use inflector::Inflector;

pub fn codegen(dict: &Dictionary) -> String {
    let mut scope = Scope::new();
    scope.raw("#![allow(dead_code)]");
    let mod_components = scope
        .new_module("components")
        .import("super", "*")
        .import("fasters::prelude", "*")
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
    dict.components().for_each(|component| {
        let struct_component = dict.build_component_struct(&component);
        mod_components.push_struct(struct_component);
    });
    let mod_messages = scope
        .new_module("messages")
        .import("super", "*")
        .import("fasters::prelude", "*")
        .import("std::io", "Write");
    dict.messages().for_each(|msg| {
        let struct_msg = dict.build_message_struct(msg.msg_type());
        mod_messages.push_struct(struct_msg);
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
        let msg_contents = self.get_message_by_msg_type(message.msg_type()).unwrap();
        let mut structure = codegen::Struct::new(message.name());
        structure
            .vis("pub")
            .doc(format!("# Message information:\n\nMessage type: {}", message.name()).as_str());
        for layout_item in msg_contents.layout() {
            let field =
                self.translate_msg_content_to_struct_field(&layout_item, layout_item.required());
            (&mut structure).push_field(field);
        }
        structure
    }

    fn build_component_struct(&self, component: &Component) -> codegen::Struct {
        let mut structure = codegen::Struct::new(component.name());
        structure.vis("pub");
        for layout_item in component.items() {
            let field =
                self.translate_msg_content_to_struct_field(&layout_item, layout_item.required());
            (&mut structure).push_field(field);
        }
        structure
    }

    fn translate_msg_content_to_struct_field(
        &self,
        content: &LayoutItem,
        required: bool,
    ) -> codegen::Field {
        let field_name = match content.kind() {
            LayoutItemKind::Component(c) => c.name().to_snake_case(),
            LayoutItemKind::Group() => "a".to_snake_case(),
            LayoutItemKind::Field(f) => f.name().to_snake_case(),
        };
        let field_type = match content.kind() {
            LayoutItemKind::Component(c) => "()".to_string(),
            LayoutItemKind::Group() => "()".to_string(),
            LayoutItemKind::Field(f) => data_type_to_str(&f.data_type()).to_string(),
        };
        let field_doc = match content.kind() {
            LayoutItemKind::Component(c) => "///".to_string(),
            LayoutItemKind::Group() => "///".to_string(),
            LayoutItemKind::Field(f) => docs::gen_field(self.get_version().to_string(), &f),
        };
        let mut field =
            codegen::Field::new(field_name.as_str(), optionify_type(required, field_type));
        field.doc(vec![field_doc.as_str()]);
        match content.kind() {
            LayoutItemKind::Component(c) => (),
            LayoutItemKind::Group() => (),
            LayoutItemKind::Field(f) => {
                field.annotation(vec![format!("#[fasters(msg_type = {})]", f.tag()).as_str()]);
            }
        };
        field
    }
}

fn data_type_to_str(basetype: &Datatype) -> &'static str {
    match basetype.name() {
        "STRING" => "String",
        "INT" => "i64",
        "LENGTH" => "usize",
        "DATA" => "Vec<u8>",
        "BOOLEAN" => "bool",
        "CHAR" => "char",
        "MONTHYEAR" => "(u8, u16)",
        "DAYOFMONTH" => "u8",
        "PRICE" => "f64",
        "EXCHANGE" => "String",
        "CURRENCY" => "String",
        _ => "Vec<u8>",
    }
}

mod docs {
    use super::*;

    pub fn gen_field(version: String, field: &Field) -> String {
        let onixs_link = field.doc_url_onixs(version.as_str());
        format!(
            "/// {}\n///\n/// # Field information\n///\n/// Tag Number: {}\n/// OnixS [reference]({}).",
            field.name(),
            field.tag(),
            onixs_link.as_str()
        )
    }

    pub fn gen_message() -> String {
        "# Message information\n\n".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::Version;

    #[test]
    fn fix_v42_syntax() {
        let fix_v42 = Dictionary::from_version(Version::Fix42);
        let code = codegen(&fix_v42);
        assert!(syn::parse_file(code.as_str()).is_ok());
    }
}
