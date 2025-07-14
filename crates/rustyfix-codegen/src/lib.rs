use heck::{ToPascalCase, ToShoutySnakeCase};
use indoc::formatdoc;
use rustc_hash::FxHashSet;
use rustyfix_dictionary::{self as dict, TagU32};
use smartstring::alias::String as SmartString;

const RUSTYFIX_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Creates a [`String`] that contains a multiline Rust "Doc" comment explaining
/// that all subsequent code was automatically generated.
///
/// The following example is for illustrative purposes only and the actual
/// contents might change. The string is guaranteed not to have any trailing or
/// leading whitespace.
///
/// ```text
/// //! Generated automatically by RustyFix. Do not modify manually.
/// ```
pub fn generated_code_notice() -> SmartString {
    formatdoc!(
        r#"
            // Generated automatically by RustyFix {} on {}.
            //
            // DO NOT MODIFY MANUALLY.
            // DO NOT COMMIT TO VERSION CONTROL.
            // ALL CHANGES WILL BE OVERWRITTEN."#,
        RUSTYFIX_VERSION,
        chrono::Utc::now().to_rfc2822(),
    )
    .into()
}

/// Generates the Rust code for an `enum` that has variants that map 1:1 the
/// available values for `field`.
pub fn codegen_field_type_enum(field: dict::Field, settings: &Settings) -> String {
    let derives = settings.derives_for_allowed_values.join(", ");
    let attributes_str = if settings.attributes_for_allowed_values.is_empty() {
        String::new()
    } else {
        format!("{}\n", settings.attributes_for_allowed_values.join("\n"))
    };
    let mut variant_identifiers = FxHashSet::default();
    let variants = field
        .enums()
        .unwrap()
        .map(|v| codegen_field_type_enum_variant(v, settings, &mut variant_identifiers))
        .collect::<Vec<_>>()
        .join("\n");
    formatdoc!(
        r#"
            /// Field type variants for [`{field_name}`].
            #[derive({derives})]
            #[allow(clippy::enum_variant_names)]
            {attributes}pub enum {identifier} {{
            {variants}
            }}"#,
        field_name = field.name().to_pascal_case(),
        derives = derives,
        attributes = attributes_str,
        identifier = field.name().to_pascal_case(),
        variants = variants,
    )
}

fn codegen_field_type_enum_variant(
    allowed_value: dict::FieldEnum,
    settings: &Settings,
    variant_identifiers: &mut FxHashSet<String>,
) -> SmartString {
    let mut identifier = allowed_value.description().to_pascal_case();
    let original_identifier = identifier.clone();
    let identifier_needs_prefix = !allowed_value
        .description()
        .chars()
        .next()
        .unwrap_or('_')
        .is_ascii_alphabetic();
    if identifier_needs_prefix {
        identifier = format!("_{identifier}");
    }
    // E.g. `TickDirection::PlusTick` -> `TickDirection::Plus`.
    if let Some(s) = identifier.strip_suffix("Tick")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    // E.g. `QuoteCancelType::CancelForSymbol` -> `QuoteCancelType::Symbol`
    if let Some(s) = identifier.strip_prefix("CancelFor")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    // E.g. `SecurityRequestType::RequestSecurityIdentityAndSpecifications`
    if let Some(s) = identifier.strip_prefix("Request")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    // E.g. `MultiLegReportingType::SingleSecurity`
    if let Some(s) = identifier.strip_suffix("Security")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_prefix("RelatedTo")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_suffix("Price")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_prefix("No")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_suffix("Trade")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_prefix("At")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_suffix("Deal")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_suffix("Sale")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_prefix("As")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    if let Some(s) = identifier.strip_prefix("Of")
        && !s.is_empty()
    {
        identifier = s.to_string();
    }
    // Ensure identifier is valid Rust identifier
    let mut final_identifier = identifier.to_pascal_case();
    if final_identifier
        .chars()
        .next()
        .is_none_or(|c| !c.is_ascii_alphabetic())
    {
        final_identifier = format!("_{final_identifier}");
    }

    // Handle duplicates on the final identifier
    if variant_identifiers.contains(&final_identifier) {
        // Fall back to original identifier if there's a collision
        final_identifier = original_identifier.to_pascal_case();
        if final_identifier
            .chars()
            .next()
            .is_none_or(|c| !c.is_ascii_alphabetic())
        {
            final_identifier = format!("_{final_identifier}");
        }

        // If still duplicated, add a suffix
        let mut counter = 2;
        let base_identifier = final_identifier.clone();
        while variant_identifiers.contains(&final_identifier) {
            final_identifier = format!("{base_identifier}{counter}");
            counter += 1;
        }
    }
    variant_identifiers.insert(final_identifier.clone());
    let value_literal = allowed_value.value();
    indent_string(
        formatdoc!(
            r#"
                /// {doc}
                #[rustyfix(variant = "{value_literal}")]
                {identifier},"#,
            doc = format!("Field variant '{}'.", value_literal),
            value_literal = value_literal,
            identifier = final_identifier,
        )
        .as_str(),
        settings.indentation.as_str(),
    )
}

/// Code generation settings. Instantiate with [`Default::default`] and then
/// change field values if necessary.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Settings {
    /// The indentation prefix of all generated Rust code. Four
    /// spaces by default.
    pub indentation: String,
    /// The base indentation level of all generated Rust code. Zero by default.
    pub indentation_depth: u32,
    /// The name of the `rustyfix` crate for imports. `rustyfix` by default.
    pub rustyfix_crate_name: String,
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
    /// Contains [`Debug`], [`Copy`], [`Clone`], [`PartialEq`], [`Eq`],
    /// [`Hash`], [`FieldType`](crate::FieldType) by default.
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
            rustyfix_crate_name: "rustyfix".to_string(),
        }
    }
}

/// Generates the Rust code for a FIX field definition.
pub fn codegen_field_definition_struct(
    fix_dictionary: &dict::Dictionary,
    field: dict::Field,
) -> String {
    let mut header = FxHashSet::default();
    let mut trailer = FxHashSet::default();
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
pub fn gen_definitions(fix_dictionary: &dict::Dictionary, settings: &Settings) -> String {
    let enums = fix_dictionary
        .fields()
        .iter()
        .filter(|f| f.enums().is_some())
        .map(|f| codegen_field_type_enum(*f, settings))
        .collect::<Vec<String>>()
        .join("\n\n");
    let field_defs = fix_dictionary
        .fields()
        .iter()
        .map(|field| codegen_field_definition_struct(fix_dictionary, *field))
        .collect::<Vec<String>>()
        .join("\n\n");
    let top_comment = onixs_link_to_dictionary(fix_dictionary.version()).unwrap_or_default();
    let code = formatdoc!(
        r#"
            {notice}

            // {top_comment}

            use {rustyfix_path}::dict::{{FieldLocation, FixDatatype}};
            use {rustyfix_path}::definitions::HardCodedFixFieldDefinition;
            use {rustyfix_path}::FieldType;

            {enum_definitions}

            {field_defs}"#,
        notice = generated_code_notice(),
        top_comment = top_comment,
        enum_definitions = enums,
        field_defs = field_defs,
        rustyfix_path = settings.rustyfix_crate_name,
    );
    code
}

fn indent_string(s: &str, prefix: &str) -> SmartString {
    s.lines()
        .map(|line| format!("{prefix}{line}"))
        .collect::<Vec<String>>()
        .join("\n")
        .into()
}

fn onixs_link_to_field(fix_version: &str, field: dict::Field) -> Option<SmartString> {
    Some(
        format!(
            "https://www.onixs.biz/fix-dictionary/{}/tagnum_{}.html",
            onixs_dictionary_id(fix_version)?,
            field.tag().get()
        )
        .into(),
    )
}

fn onixs_link_to_dictionary(fix_version: &str) -> Option<SmartString> {
    Some(
        format!(
            "https://www.onixs.biz/fix-dictionary/{}/index.html",
            onixs_dictionary_id(fix_version)?
        )
        .into(),
    )
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
    fix_dictionary: &dict::Dictionary,
    header_tags: &FxHashSet<TagU32>,
    trailer_tags: &FxHashSet<TagU32>,
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
    let doc_link = onixs_link_to_field(fix_dictionary.version(), field);
    let doc = if let Some(doc_link) = doc_link {
        format!("/// Field attributes for [`{name} <{tag}>`]({doc_link}).")
    } else {
        format!("/// Field attributes for `{name} <{tag}>`.")
    };

    formatdoc!(
        r#"
            {doc}
            pub const {identifier}: &HardCodedFixFieldDefinition = &HardCodedFixFieldDefinition {{
                name: "{name}",
                tag: {tag},
                data_type: FixDatatype::{data_type},
                location: FieldLocation::{field_location},
            }};"#,
        doc = doc,
        identifier = name,
        name = field.name(),
        tag = tag,
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
        for dict in dict::Dictionary::common_dictionaries().into_iter() {
            let code = gen_definitions(&dict, &codegen_settings);
            syn::parse_file(code.as_str()).unwrap();
        }
    }

    #[test]
    fn generated_code_notice_is_trimmed() {
        let notice = generated_code_notice();
        assert_eq!(notice, notice.trim());
    }
}
