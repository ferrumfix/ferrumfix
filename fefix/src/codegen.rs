//! Code generation utilities.

use super::data_type::DataType;
use super::dictionary::{Dictionary, Field, LayoutItem, LayoutItemKind, Message};
use indoc::indoc;
use inflector::Inflector;

fn generated_code_notice() -> String {
    use chrono::prelude::*;
    format!(
        indoc!(r#"
            // Generated automatically by FerrumFIX {} on {}.
            //
            // DO NOT MODIFY MANUALLY.
            // DO NOT COMMIT TO VERSION CONTROL.
            // ALL CHANGES WILL BE OVERWRITTEN.
            "#
        ),
        FEFIX_VERSION,
        Utc::now().to_rfc2822(),
    )
}

const FEFIX_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn message(dict: Dictionary, message: Message, custom_derive_line: &str) -> String {
    let identifier = message.name().to_pascal_case();
    let fields = message
        .layout()
        .map(|layout_item| {
            dict.translate_layout_item_to_struct_field(&layout_item, layout_item.required())
        })
        .filter(|opt| opt.is_some())
        .map(|opt| opt.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        indoc!(
            r#"
            #[derive(Debug)]
            {custom_derive_line}
            pub struct {identifier}<'a> {{
                lifetime: PhantomData<&'a ()>,
                {fields}
            }}

            impl<'a> {identifier}<'a> {{
                pub const MSG_TYPE: &'static [u8] = b"{msg_type}";
            }}
            "#
        ),
        custom_derive_line = custom_derive_line,
        identifier = identifier,
        msg_type = message.msg_type(),
        fields = fields,
    )
}

pub fn field_def(field: Field, fefix_path: &str) -> String {
    let name = field.name().to_screaming_snake_case();
    let tag = field.tag().to_string();
    format!(
        indoc!(
            r#"
            pub const {identifier}: &FieldDef<'static, {type_param}> = &FieldDef{{
                name: "{name}",
                tag: {tag},
                is_group_leader: {group},
                data_type: DataType::{data_type},
                location: FieldLocation::Body,
                phantom: PhantomData,
            }};
            "#
        ),
        identifier = name,
        type_param = suggested_type(field.tag(), field.data_type().basetype(), fefix_path),
        name = field.name(),
        tag = tag,
        group = field.name().ends_with("Len"),
        data_type = <&'static str as From<DataType>>::from(field.data_type().basetype()),
    )
}

pub fn fields(dict: Dictionary, fefix_path: &str) -> String {
    let field_defs = dict
        .iter_fields()
        .map(|field| field_def(field, fefix_path))
        .collect::<Vec<String>>()
        .join("\n");
    let code = format!(
        indoc!(
            r#"
            #![allow(dead_code)]

            {notice}

            use {fefix_path}::fields::{{FieldDef, FieldLocation}};
            use {fefix_path}::DataType;
            use std::marker::PhantomData;

            {field_defs}
            "#
        ),
        notice = generated_code_notice(),
        field_defs = field_defs,
        fefix_path = fefix_path,
    );
    code
}

fn suggested_type(tag: u32, data_type: DataType, fefix_path: &str) -> String {
    if tag == 10 {
        return format!("{}::dtf::CheckSum", fefix_path);
    }
    if data_type.base_type() == DataType::Float {
        return "rust_decimal::Decimal".to_string();
    }
    match data_type {
        DataType::String => "&[u8]".to_string(),
        DataType::Char => "u8".to_string(),
        DataType::Boolean => "bool".to_string(),
        DataType::Country => "&[u8; 2]".to_string(),
        DataType::Currency => "&[u8; 3]".to_string(),
        DataType::Exchange => "&[u8; 4]".to_string(),
        DataType::Data => "&[u8]".to_string(),
        DataType::Length => "usize".to_string(),
        DataType::DayOfMonth => "u32".to_string(),
        DataType::Int => "i64".to_string(),
        DataType::Language => "&[u8; 2]".to_string(),
        DataType::SeqNum => "u64".to_string(),
        DataType::NumInGroup => "usize".to_string(),
        DataType::UtcDateOnly => format!("{}::dtf::Date", fefix_path),
        DataType::UtcTimeOnly => format!("{}::dtf::Time", fefix_path),
        DataType::UtcTimestamp => format!("{}::dtf::Timestamp", fefix_path),
        _ => "&[u8]".to_string(), // TODO
    }
}

fn suggested_type_with_lifetime(tag: u32, data_type: DataType) -> &'static str {
    if tag == 10 {
        return "crate::dtf::CheckSum";
    }
    if data_type.base_type() == DataType::Float {
        return "rust_decimal::Decimal";
    }
    match data_type {
        DataType::String => "&'a [u8]",
        DataType::Char => "u8",
        DataType::Boolean => "bool",
        DataType::Country => "[u8; 2]",
        DataType::Currency => "[u8; 3]",
        DataType::Exchange => "[u8; 4]",
        DataType::Data => "&'a [u8]",
        DataType::Length => "usize",
        DataType::DayOfMonth => "u32",
        DataType::Int => "i64",
        DataType::Language => "[u8; 2]",
        DataType::SeqNum => "u64",
        DataType::NumInGroup => "usize",
        DataType::UtcDateOnly => "crate::dtf::Date",
        DataType::UtcTimeOnly => "crate::dtf::Time",
        DataType::UtcTimestamp => "crate::dtf::Timestamp",
        _ => "&'a [u8]", // TODO
    }
}

fn make_type_optional(required: bool, typ: String) -> String {
    if required {
        typ
    } else {
        format!("::std::option::Option<{}>", typ)
    }
}

impl Dictionary {
    //fn build_message_struct(&self, msg_type: &str) -> String {
    //    let message = self.message_by_msgtype(msg_type).unwrap();
    //    let fields: Vec<String> = message
    //        .layout()
    //        .map(|layout_item| {
    //            self.translate_layout_item_to_struct_field(&layout_item, layout_item.required())
    //        })
    //        .filter(|opt| opt.is_some())
    //        .map(|opt| opt.unwrap())
    //        .collect();
    //    format!(
    //        r#"
    //        /// Message information: {msg_name}
    //        #[derive(Debug, Clone, ReadFields)]
    //        #[fefix(msg_type = "{msg_type}")]
    //        pub struct {msg_name} {{
    //            {fields}
    //        }}
    //        "#,
    //        msg_type = message.msg_type(),
    //        msg_name = message.name(),
    //        fields = fields.join(", ")
    //    )
    //}

    //fn build_component_struct(&self, component: &Component) -> String {
    //    let fields: Vec<String> = component
    //        .items()
    //        .map(|layout_item| {
    //            self.translate_layout_item_to_struct_field(&layout_item, layout_item.required())
    //        })
    //        .filter(|opt| opt.is_some())
    //        .map(|opt| opt.unwrap())
    //        .collect();
    //    format!(
    //        r#"
    //        /// Component information: {msg_name}
    //        #[fefix(msg_type = "TODO")]
    //        #[derive(Debug, Clone, ReadFields)]
    //        pub struct {msg_name} {{
    //            {fields}
    //        }}
    //        "#,
    //        msg_name = component.name(),
    //        fields = fields.join(", ")
    //    )
    //}

    fn translate_layout_item_to_struct_field(
        &self,
        item: &LayoutItem,
        required: bool,
    ) -> Option<String> {
        let field_name = match item.kind() {
            LayoutItemKind::Component(c) => c.name().to_snake_case(),
            LayoutItemKind::Group(_, _) => return None,
            LayoutItemKind::Field(f) => f.name().to_snake_case(),
        };
        let field_type = match item.kind() {
            LayoutItemKind::Component(_c) => "()".to_string(),
            LayoutItemKind::Group(_, _) => "()".to_string(),
            LayoutItemKind::Field(f) => {
                suggested_type_with_lifetime(f.tag(), f.data_type().basetype()).to_string()
            }
        };
        //let field_tag = match item.kind() {
        //    LayoutItemKind::Component(_c) => 1337,
        //    LayoutItemKind::Group(_, _) => 42,
        //    LayoutItemKind::Field(f) => f.tag(),
        //};
        let _field_doc = match item.kind() {
            LayoutItemKind::Component(_c) => "///".to_string(),
            LayoutItemKind::Group(_, _) => "///".to_string(),
            LayoutItemKind::Field(f) => docs::gen_field(self.get_version().to_string(), &f),
        };
        Some(format!(
            r#"
            pub {identifier}: {field_type},
            "#,
            identifier = field_name,
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
        let code = fields(fix_v42, "fefix");
        assert!(syn::parse_file(code.as_str()).is_ok());
    }

    #[test]
    fn syntax_of_field_tags_is_ok() {
        for version in AppVersion::ALL.iter().copied() {
            let dict = Dictionary::from_version(version);
            let code = fields(dict, "crate");
            syn::parse_file(code.as_str()).unwrap();
        }
    }
}
