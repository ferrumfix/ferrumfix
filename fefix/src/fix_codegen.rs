//!

use crate::dictionary::{Component, Dictionary, Field, LayoutItem, LayoutItemKind};
use inflector::Inflector;

/// Generates Rust code for a module that contains field tag mnemonics. The
/// module contains `pub const` definitions for all fields, and allows access to
/// tag numbers via names (to reduce magic numbers in code).
///
/// # Examples
///
/// ```
/// use fefix::{AppVersion, Dictionary, codegen_tag_mnemonics};
///
/// let dict = Dictionary::from_version(AppVersion::Fixt11);
/// let code = codegen_tag_mnemonics(&dict);
///
/// println!("{}", code);
/// // pub const BEGIN_STRING: u32 = 8;
/// // pub const CHECK_SUM: u32 = 10;
/// // ...
/// ```
pub fn codegen_tag_mnemonics(dict: &Dictionary) -> String {
    let field_tags: Vec<String> = dict
        .iter_fields()
        .map(|field| {
            let name = field.name().to_screaming_snake_case();
            let tag = field.tag().to_string();
            format!("pub const {}: u32 = {};", name, tag)
        })
        .collect();
    let code = format!(
        r#"#![allow(dead_code)]

{field_tags}
"#,
        field_tags = field_tags.join("\n"),
    );
    code
}

pub fn codegen(dict: &Dictionary) -> String {
    let component_defs: Vec<String> = dict
        .iter_components()
        .map(|comp| dict.build_component_struct(&comp))
        .collect();
    let message_defs: Vec<String> = dict
        .iter_messages()
        .map(|msg| dict.build_message_struct(msg.msg_type()))
        .collect();
    let code = format!(
        "#![allow(dead_code)]

        use fefix_derive::*;
        use fefix::backend::value::*;
    
        {components}
        
        {messages}
        ",
        components = component_defs.join("\n\n"),
        messages = message_defs.join("\n\n")
    );
    code
}

fn make_type_optional(required: bool, typ: String) -> String {
    if required {
        typ
    } else {
        format!("::std::option::Option<{}>", typ)
    }
}

impl Dictionary {
    fn build_message_struct(&self, msg_type: &str) -> String {
        let message = self.message_by_msgtype(msg_type).unwrap();
        let fields: Vec<String> = message
            .layout()
            .map(|layout_item| {
                self.translate_layout_item_to_struct_field(&layout_item, layout_item.required())
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        format!(
            r#"
            /// Message information: {msg_name}
            #[derive(Debug, Clone, ReadFields)]
            #[fefix(msg_type = "{msg_type}")]
            pub struct {msg_name} {{
                {fields}
            }}
            "#,
            msg_type = message.msg_type(),
            msg_name = message.name(),
            fields = fields.join(", ")
        )
    }

    fn build_component_struct(&self, component: &Component) -> String {
        let fields: Vec<String> = component
            .items()
            .map(|layout_item| {
                self.translate_layout_item_to_struct_field(&layout_item, layout_item.required())
            })
            .filter(|opt| opt.is_some())
            .map(|opt| opt.unwrap())
            .collect();
        format!(
            r#"
            /// Component information: {msg_name}
            #[fefix(msg_type = "TODO")]
            #[derive(Debug, Clone, ReadFields)]
            pub struct {msg_name} {{
                {fields}
            }}
            "#,
            msg_name = component.name(),
            fields = fields.join(", ")
        )
    }

    fn translate_layout_item_to_struct_field(
        &self,
        item: &LayoutItem,
        required: bool,
    ) -> Option<String> {
        let field_name = match item.kind() {
            LayoutItemKind::Component(c) => c.name().to_snake_case(),
            LayoutItemKind::Group => return None,
            LayoutItemKind::Field(f) => f.name().to_snake_case(),
        };
        let field_type = match item.kind() {
            LayoutItemKind::Component(_c) => "()".to_string(),
            LayoutItemKind::Group => "()".to_string(),
            LayoutItemKind::Field(f) => f.data_type().basetype().name().to_string(),
        };
        let field_tag = match item.kind() {
            LayoutItemKind::Component(_c) => 1337,
            LayoutItemKind::Group => 42,
            LayoutItemKind::Field(f) => f.tag(),
        };
        let _field_doc = match item.kind() {
            LayoutItemKind::Component(_c) => "///".to_string(),
            LayoutItemKind::Group => "///".to_string(),
            LayoutItemKind::Field(f) => docs::gen_field(self.get_version().to_string(), &f),
        };
        Some(format!(
            r#"
            #[fefix(tag = {field_tag}, rust_type = "{rust_type}", opt = {opt})]
            pub {field_name}: {field_type}
            "#,
            opt = !required,
            rust_type = "",
            field_tag = field_tag,
            field_name = field_name,
            field_type = make_type_optional(required, field_type)
        ))
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

    pub fn _gen_message() -> String {
        "# Message information\n\n".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::AppVersion;

    #[test]
    fn fix_v42_syntax() {
        let fix_v42 = Dictionary::from_version(AppVersion::Fix42);
        let code = codegen(&fix_v42);
        assert!(syn::parse_file(code.as_str()).is_ok());
    }

    #[test]
    fn syntax_of_field_tags_is_ok() {
        for version in AppVersion::iter_all() {
            println!("{}", version);
            let dict = Dictionary::from_version(version);
            let code = codegen_tag_mnemonics(&dict);
            syn::parse_file(code.as_str()).unwrap();
        }
    }
}
