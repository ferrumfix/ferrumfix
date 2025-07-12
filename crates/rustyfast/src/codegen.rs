use super::{FieldType, PrimitiveType, Template};
use heck::{CamelCase, SnakeCase};
use indoc::indoc;

const GENERATED_CODE_NOTICE: &str = indoc!(
    r#"
// Generated automaticaly by FerrumFIX.
//
// DO NOT MODIFY MANUALLY. ALL CHANGES WILL BE OVERWRITTEN.
"#
);

pub fn template_struct(template: &Template, custom_derive_line: &str) -> String {
    let identifier = template.name().to_camel_case();
    let fields = template
        .iter_items()
        .map(|field_instruction| {
            let field_name = field_instruction.name().to_snake_case();
            let field_type_str = match field_instruction.kind() {
                FieldType::Primitive(pt) => optional_rust_type(
                    primitive_fast_type_to_rust_type(*pt),
                    !field_instruction.is_mandatory(),
                ),
                FieldType::Group(_group) => String::new(), // No fields for groups for now.
            };
            if field_type_str.is_empty() {
                String::new()
            } else {
                format!("{}: {},", field_name, field_type_str)
            }
        })
        .collect::<Vec<String>>()
        .join("\n                ");
    let methods = template
        .iter_items()
        .map(|field_instruction| {
            let field_name = field_instruction.name().to_snake_case();
            let field_type_str = match field_instruction.kind() {
                FieldType::Primitive(pt) => {
                    let rust_type = primitive_fast_type_to_rust_type(*pt);
                    optional_rust_type(rust_type, !field_instruction.is_mandatory())
                }
                FieldType::Group(_) => return String::new(), // No methods for groups for now.
            };

            if field_type_str.is_empty() {
                return String::new();
            }

            let getter = format!(
                "pub fn {field_name}(&self) -> {field_type_str} {{\n                    self.{field_name}\n                }}",
            );

            let setter = format!(
                "pub fn set_{field_name}(&mut self, value: {field_type_str}) {{\n                    self.{field_name} = value;\n                }}",
            );

            format!("{}\n\n                {}", getter, setter)
        })
        .collect::<Vec<String>>()
        .join("\n\n                ");
    format!(
        indoc!(
            r#"
            {notice}

            #[derive(Debug)]
            {custom_derive_line}
            pub struct {identifier}<'a> {{
                lifetime: PhantomData<&'a ()>,
                {fields}
            }}

            impl<'a> {identifier}<'a> {{
                {methods}
            }}
            "#
        ),
        notice = GENERATED_CODE_NOTICE,
        custom_derive_line = custom_derive_line,
        identifier = identifier,
        fields = fields,
        methods = methods,
    )
}

fn optional_rust_type(t: &str, optional: bool) -> String {
    if optional {
        format!("Option<{t}>")
    } else {
        t.to_string()
    }
}

fn primitive_fast_type_to_rust_type(pt: PrimitiveType) -> &'static str {
    match pt {
        PrimitiveType::AsciiString => "&'a [u8]",
        PrimitiveType::Utf8String => "&'a str",
        PrimitiveType::I32 => "i32",
        PrimitiveType::U32 => "u32",
        PrimitiveType::U64 => "u64",
        PrimitiveType::I64 => "i64",
        PrimitiveType::Bytes => "&'a [u8]",
        PrimitiveType::Decimal => "Decimal",
    }
}
