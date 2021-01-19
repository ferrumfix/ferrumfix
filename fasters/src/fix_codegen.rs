use crate::dictionary::{Component, Datatype, Dictionary, Field, LayoutItem, LayoutItemKind};
use inflector::Inflector;

fn crate_name() -> &'static str {
    "crate"
}

pub fn codegen(dict: &Dictionary) -> String {
    let component_defs: Vec<String> = dict
        .components()
        .map(|comp| dict.build_component_struct(&comp))
        .collect();
    let message_defs: Vec<String> = dict
        .messages()
        .map(|msg| dict.build_message_struct(msg.msg_type()))
        .collect();
    let code = format!(
        "#![allow(dead_code)]

        use fasters_derive::*;
        
        pub mod components {{
            use super::*;
        
            {components}
        }}
        
        pub mod messages {{
            use super::*;
        
            {messages}
        }}
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
        let message = self.get_message_by_msg_type(msg_type).unwrap();
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
            #[derive(TsrMessage)]
            #[fasters(msg_type = "{msg_type}")]
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
            #[fasters(msg_type = "TODO")]
            #[derive(TsrMessage)]
            pub struct {msg_name} {{
                {fields}
            }}
            "#,
            msg_name = component.name(),
            fields = fields.join(", ")
        )
    }

    fn translate_layout_item_to_struct_field(&self, item: &LayoutItem, required: bool) -> Option<String> {
        let field_name = match item.kind() {
            LayoutItemKind::Component(c) => c.name().to_snake_case(),
            LayoutItemKind::Group() => return None,
            LayoutItemKind::Field(f) => f.name().to_snake_case(),
        };
        let field_type = match item.kind() {
            LayoutItemKind::Component(_c) => "()".to_string(),
            LayoutItemKind::Group() => "()".to_string(),
            LayoutItemKind::Field(f) => data_type_to_str(&f.data_type()).to_string(),
        };
        let field_tag = match item.kind() {
            LayoutItemKind::Component(_c) => 1337,
            LayoutItemKind::Group() => 42,
            LayoutItemKind::Field(f) => f.tag(),
        };
        let _field_doc = match item.kind() {
            LayoutItemKind::Component(_c) => "///".to_string(),
            LayoutItemKind::Group() => "///".to_string(),
            LayoutItemKind::Field(f) => docs::gen_field(self.get_version().to_string(), &f),
        };
        Some(format!(
            r#"
            #[fasters(tag = {field_tag}, rust_type = "{rust_type}", opt = {opt})]
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
        "PRICE" => "f64", // FIXME
        "EXCHANGE" => "String",
        "CURRENCY" => "String",
        "TIME" => "::std::time::Instant",
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
