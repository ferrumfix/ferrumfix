//! Code generation utilities.

use super::dict::{Dictionary, Field, LayoutItem, LayoutItemKind, Message};
use super::fix_data_type::FixDataType;
use super::TagU16;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use indoc::indoc;

fn generated_code_notice() -> String {
    use chrono::prelude::*;
    format!(
        indoc!(
            r#"
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
    let identifier = message.name().to_camel_case();
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
    let name = field.name().to_shouty_snake_case();
    let tag = field.tag().to_string();
    let (enum_type_name, enum_variants) = if let Some(variants) = field.enums() {
        let variants: Vec<String> = variants
            .map(|e| {
                format!(
                    indoc!(
                        r#"
                        {indentation}/// {doc}
                        {indentation}#[fefix(variant = "{variant}")]
                        {indentation}{}{},"#
                    ),
                    if e.description().chars().next().unwrap().is_ascii_digit() {
                        "N"
                    } else {
                        ""
                    },
                    e.description().to_camel_case(),
                    doc = format!("Field variant '{}'.", e.value()),
                    variant = e.value(),
                    indentation = "    ",
                )
            })
            .collect();
        (
            Some(field.name().to_camel_case()),
            format!(
                indoc!(
                    r#"
                    /// Field type variants for [`{struct_name}`].
                    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, DataType)]
                    pub enum {identifier} {{
                    {variants}
                    }}
                "#
                ),
                struct_name = field.name().to_shouty_snake_case(),
                identifier = field.name().to_camel_case(),
                variants = variants.join("\n")
            ),
        )
    } else {
        (None, String::new())
    };
    format!(
        indoc!(
            r#"
            /// Field attributes for [`{name} <{tag}>`](https://www.onixs.biz/fix-dictionary/{major}.{minor}/tagnum_{tag}.html).
            pub const {identifier}: &GeneratedFieldDef<'static, {type_param}> = &GeneratedFieldDef{{
                name: "{name}",
                tag: {tag},
                is_group_leader: {group},
                data_type: FixDataType::{data_type},
                phantom: PhantomData,
                location: FieldLocation::Body,
            }};
            {enum_variants}
            "#
        ),
        major = "4",
        minor = "4",
        identifier = name,
        type_param = suggested_type(
            field.tag(),
            field.data_type().basetype(),
            enum_type_name,
            fefix_path
        ),
        name = field.name(),
        tag = tag,
        enum_variants = enum_variants,
        group = field.name().ends_with("Len"),
        data_type = <&'static str as From<FixDataType>>::from(field.data_type().basetype()),
    )
}

/// Generates `const` implementors of
/// [`IsFieldDefinition`](super::dict::IsFieldDefinition).
pub fn fields(dict: Dictionary, fefix_path: &str) -> String {
    let field_defs = dict
        .iter_fields()
        .map(|field| field_def(field, fefix_path))
        .collect::<Vec<String>>()
        .join("\n");
    let code = format!(
        indoc!(
            r#"
            //! Field and message definitions for {version}.

            #![allow(dead_code)]

            {notice}

            use {fefix_path}::dict::FieldLocation;
            use {fefix_path}::{{FixDataType, Buffer}};
            use {fefix_path}::definitions::GeneratedFieldDef;
            {import_data_field}
            use std::marker::PhantomData;

            {field_defs}
            "#
        ),
        version = dict.get_version(),
        notice = generated_code_notice(),
        import_data_field = if fefix_path == "fefix" {
            "use fefix::DataType;"
        } else {
            "use crate::DataType;"
        },
        field_defs = field_defs,
        fefix_path = fefix_path,
    );
    code
}

fn suggested_type(
    tag: TagU16,
    data_type: FixDataType,
    enum_type_name: Option<String>,
    fefix_path: &str,
) -> String {
    if let Some(name) = enum_type_name {
        return name;
    }
    if tag.get() == 10 {
        return format!("{}::datatypes::CheckSum", fefix_path);
    }
    if data_type.base_type() == FixDataType::Float {
        return "rust_decimal::Decimal".to_string();
    }
    match data_type {
        FixDataType::String => "&[u8]".to_string(),
        FixDataType::Char => "u8".to_string(),
        FixDataType::Boolean => "bool".to_string(),
        FixDataType::Country => "&[u8; 2]".to_string(),
        FixDataType::Currency => "&[u8; 3]".to_string(),
        FixDataType::Exchange => "&[u8; 4]".to_string(),
        FixDataType::Data => "&[u8]".to_string(),
        FixDataType::Length => "usize".to_string(),
        FixDataType::DayOfMonth => "u32".to_string(),
        FixDataType::Int => "i64".to_string(),
        FixDataType::Language => "&[u8; 2]".to_string(),
        FixDataType::SeqNum => "u64".to_string(),
        FixDataType::NumInGroup => "usize".to_string(),
        FixDataType::UtcDateOnly => format!("{}::datatypes::Date", fefix_path),
        FixDataType::UtcTimeOnly => format!("{}::datatypes::Time", fefix_path),
        FixDataType::UtcTimestamp => format!("{}::datatypes::Timestamp", fefix_path),
        _ => "&[u8]".to_string(), // TODO
    }
}

fn suggested_type_with_lifetime(tag: TagU16, data_type: FixDataType) -> &'static str {
    if tag.get() == 10 {
        return "crate::datatypes::CheckSum";
    }
    if data_type.base_type() == FixDataType::Float {
        return "rust_decimal::Decimal";
    }
    match data_type {
        FixDataType::String => "&'a [u8]",
        FixDataType::Char => "u8",
        FixDataType::Boolean => "bool",
        FixDataType::Country => "[u8; 2]",
        FixDataType::Currency => "[u8; 3]",
        FixDataType::Exchange => "[u8; 4]",
        FixDataType::Data => "&'a [u8]",
        FixDataType::Length => "usize",
        FixDataType::DayOfMonth => "u32",
        FixDataType::Int => "i64",
        FixDataType::Language => "[u8; 2]",
        FixDataType::SeqNum => "u64",
        FixDataType::NumInGroup => "usize",
        FixDataType::UtcDateOnly => "crate::datatypes::Date",
        FixDataType::UtcTimeOnly => "crate::datatypes::Time",
        FixDataType::UtcTimestamp => "crate::datatypes::Timestamp",
        _ => "&'a [u8]", // TODO
    }
}

struct RustTypeName {
    s: String,
}

impl RustTypeName {
    fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { s: name.into() }
    }

    fn optional(self, opt: bool) -> Self {
        if opt {
            Self {
                s: format!("::std::option::Option<{}>", self.s),
            }
        } else {
            self
        }
    }
}

impl Dictionary {
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
            field_type = RustTypeName::new(field_type).optional(!required).s
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
