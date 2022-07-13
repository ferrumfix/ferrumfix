//! Code generation utilities.

use super::{dict, TagU16};
use fnv::FnvHashSet;
use heck::{ToPascalCase, ToShoutySnakeCase};
use indoc::indoc;
use std::marker::PhantomData;

const FEFIX_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Creates a [`String`] that contains a multiline Rust "Doc" comment explaining
/// that all subsequent code was automatically generated.
///
/// The following example is for illustrative purposes only and the actual
/// contents might change. The string is guaranteed not to have any trailing or
/// leading whitespace.
///
/// ```text
/// // Generated automatically by FerrumFIX. Do not modify manually.
/// ```
pub fn generated_code_notice() -> String {
    use chrono::prelude::*;

    format!(
        indoc!(
            r#"
            // Generated automatically by FerrumFIX {} on {}.
            //
            // DO NOT MODIFY MANUALLY.
            // DO NOT COMMIT TO VERSION CONTROL.
            // ALL CHANGES WILL BE OVERWRITTEN."#
        ),
        FEFIX_VERSION,
        Utc::now().to_rfc2822(),
    )
}

/// Generates the Rust code for an `enum` that has variants that map 1:1 the
/// available values for `field`.
pub fn codegen_field_type_enum(field: dict::Field, settings: &Settings) -> String {
    let derives = settings.derives_for_allowed_values.join(", ");
    let attributes = settings.attributes_for_allowed_values.join("\n");
    let variants = field
        .enums()
        .unwrap()
        .map(|v| codegen_field_type_enum_variant(v, settings))
        .collect::<Vec<String>>()
        .join("\n");
    format!(
        indoc!(
            r#"
            /// Field type variants for [`{field_name}`].
            #[derive({derives})]
            {attributes}
            pub enum {identifier} {{
            {variants}
            }}"#
        ),
        field_name = field.name().to_pascal_case(),
        derives = derives,
        attributes = attributes,
        identifier = field.name().to_pascal_case(),
        variants = variants,
    )
}

fn codegen_field_type_enum_variant(allowed_value: dict::FieldEnum, settings: &Settings) -> String {
    let mut identifier = allowed_value.description().to_pascal_case();
    let identifier_needs_prefix = !allowed_value
        .description()
        .chars()
        .next()
        .unwrap_or('_')
        .is_ascii_alphabetic();
    if identifier_needs_prefix {
        identifier = format!("_{}", identifier);
    }
    let value_literal = allowed_value.value();
    indent_string(
        format!(
            indoc!(
                r#"
                /// {doc}
                #[fefix(variant = "{value_literal}")]
                {identifier},"#
            ),
            doc = format!("Field variant '{}'.", value_literal),
            value_literal = value_literal,
            identifier = identifier,
        )
        .as_str(),
        settings.indentation.as_str(),
    )
}

/// Code generation settings. Instantiate with [`Default::default`] and then
/// change field values if necessary.
#[derive(Debug, Clone)]
pub struct Settings {
    phantom: PhantomData<()>,

    /// The indentation prefix of all generated Rust code. Four
    /// spaces by default.
    pub indentation: String,
    /// The indentation level of all generated Rust code. Zero by default.
    pub indentation_depth: u32,
    /// The name of the `fefix` crate for imports. `fefix` by default.
    pub fefix_crate_name: String,
    /// A list of derive macros on top of all generated FIX datatype `enum`s. E.g.:
    ///
    /// ```
    /// // #[derive(Foobar, Spam)]
    /// enum FoodOrDrink {
    ///     Food,
    ///     Drink,
    /// }
    /// ```
    ///
    /// Contains [`Debug`], [`Copy`], [`PartialEq`], [`Eq`], [`Hash`],
    /// [`FieldType`](crate::FieldType) by default.
    pub derives_for_allowed_values: Vec<String>,
    /// A list of attribute macros for generated `enum`s variants. E.g.:
    ///
    /// ```
    /// enum FoodOrDrink {
    ///     // #[foobar]
    ///     Food,
    ///     // #[spam]
    ///     Drink,
    /// }
    /// ```
    ///
    /// Empty by default.
    pub attributes_for_allowed_values: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            indentation: "    ".to_string(),
            indentation_depth: 0,
            derives_for_allowed_values: vec![
                "Debug".to_string(),
                "Copy".to_string(),
                "Clone".to_string(),
                "PartialEq".to_string(),
                "Eq".to_string(),
                "Hash".to_string(),
                "FieldType".to_string(),
            ],
            attributes_for_allowed_values: vec![],
            fefix_crate_name: "fefix".to_string(),
            phantom: PhantomData::default(),
        }
    }
}

/// Generates the Rust code for a FIX field definition.
pub fn codegen_field_definition_struct(
    fix_dictionary: dict::Dictionary,
    field: dict::Field,
) -> String {
    let mut header = FnvHashSet::default();
    let mut trailer = FnvHashSet::default();
    for item in fix_dictionary
        .component_by_name("StandardHeader")
        .unwrap()
        .items()
    {
        if let dict::LayoutItemKind::Field(f) = item.kind() {
            header.insert(f.tag());
        }
    }
    for item in fix_dictionary
        .component_by_name("StandardTrailer")
        .unwrap()
        .items()
    {
        if let dict::LayoutItemKind::Field(f) = item.kind() {
            trailer.insert(f.tag());
        }
    }
    gen_field_definition_with_hashsets(fix_dictionary, &header, &trailer, field)
}

/// Generates `const` implementors of
/// [`IsFieldDefinition`](super::dict::IsFieldDefinition).
///
/// The generated module will contain:
///
/// - A generated code notice ([generated_code_notice]).
/// - `enum` definitions for FIX field types.
/// - A constant implementor of
///   [`IsFieldDefinition`](super::dict::IsFieldDefinition)
///   for each FIX field.
///
/// The Rust code will be free of any leading and trailing whitespace.
/// An effort is made to provide good formatting, but users shouldn't rely on it
/// and assume that formatting might be bad.
pub fn gen_definitions(fix_dictionary: dict::Dictionary, settings: &Settings) -> String {
    let enums = fix_dictionary
        .iter_fields()
        .filter(|f| f.enums().is_some())
        .map(|f| codegen_field_type_enum(f, settings))
        .collect::<Vec<String>>()
        .join("\n\n");
    let field_defs = fix_dictionary
        .iter_fields()
        .map(|field| codegen_field_definition_struct(fix_dictionary.clone(), field))
        .collect::<Vec<String>>()
        .join("\n");
    let top_comment =
        onixs_link_to_dictionary(fix_dictionary.get_version()).unwrap_or_default();
    let code = format!(
        indoc!(
            r#"
            {notice}

            // {top_comment}

            use {fefix_path}::dict::FieldLocation;
            use {fefix_path}::dict::FixDatatype;
            use {fefix_path}::definitions::HardCodedFixFieldDefinition;
            use {fefix_path}::FieldType;

            {enum_definitions}

            {field_defs}"#
        ),
        notice = generated_code_notice(),
        top_comment = top_comment,
        enum_definitions = enums,
        field_defs = field_defs,
        fefix_path = settings.fefix_crate_name,
    );
    code
}

fn indent_string(s: &str, prefix: &str) -> String {
    s.lines().fold(String::new(), |mut s, line| {
        if line.contains(char::is_whitespace) {
            s.push_str(prefix);
        }
        s.push_str(line);
        s.push('\n');
        s
    })
}

fn onixs_link_to_field(fix_version: &str, field: dict::Field) -> Option<String> {
    Some(format!(
        "https://www.onixs.biz/fix-dictionary/{}/tagnum_{}.html",
        onixs_dictionary_id(fix_version)?,
        field.tag().get()
    ))
}

fn onixs_link_to_dictionary(fix_version: &str) -> Option<String> {
    Some(format!(
        "https://www.onixs.biz/fix-dictionary/{}/index.html",
        onixs_dictionary_id(fix_version)?
    ))
}

fn onixs_dictionary_id(fix_version: &str) -> Option<&str> {
    Some(match fix_version {
        "FIX.4.0" => "4.0",
        "FIX.4.1" => "4.1",
        "FIX.4.2" => "4.2",
        "FIX.4.3" => "4.3",
        "FIX.4.4" => "4.4",
        "FIX.5.0" => "5.0",
        "FIX.5.0-SP1" => "5.0.sp1",
        "FIX.5.0-SP2" => "5.0.sp2",
        "FIXT.1.1" => "fixt1.1",
        _ => return None,
    })
}

fn gen_field_definition_with_hashsets(
    fix_dictionary: dict::Dictionary,
    header_tags: &FnvHashSet<TagU16>,
    trailer_tags: &FnvHashSet<TagU16>,
    field: dict::Field,
) -> String {
    let name = field.name().to_shouty_snake_case();
    let tag = field.tag().to_string();
    let field_location = if header_tags.contains(&field.tag()) {
        "Header"
    } else if trailer_tags.contains(&field.tag()) {
        "Trailer"
    } else {
        "Body"
    };
    let doc_link = onixs_link_to_field(fix_dictionary.get_version(), field);
    let doc = if let Some(doc_link) = doc_link {
        format!(
            "/// Field attributes for [`{} <{}>`]({}).",
            name, tag, doc_link
        )
    } else {
        format!("/// Field attributes for `{} <{}>`.", name, tag)
    };
    format!(
        indoc!(
            r#"
                {doc}
                pub const {identifier}: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {{
                    name: "{name}",
                    tag: {tag},
                    is_group_leader: {group},
                    data_type: FixDatatype::{data_type},
                    location: FieldLocation::{field_location},
                }};"#
        ),
        doc = doc,
        identifier = name,
        name = field.name(),
        tag = tag,
        group = field.name().ends_with("Len"),
        field_location = field_location,
        data_type = <&'static str as From<dict::FixDatatype>>::from(field.data_type().basetype()),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn syntax_of_field_definitions_is_ok() {
        let codegen_settings = Settings::default();
        for dict in dict::Dictionary::all().into_iter() {
            let code = gen_definitions(dict, &codegen_settings);
            syn::parse_file(code.as_str()).unwrap();
        }
    }

    #[test]
    fn generated_code_notice_is_trimmed() {
        let notice = generated_code_notice();
        assert_eq!(notice, notice.trim());
    }
}
