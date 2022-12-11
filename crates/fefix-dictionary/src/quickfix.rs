use super::*;

pub struct QuickFixReader<'a> {
    node_with_header: roxmltree::Node<'a, 'a>,
    node_with_trailer: roxmltree::Node<'a, 'a>,
    node_with_components: roxmltree::Node<'a, 'a>,
    node_with_messages: roxmltree::Node<'a, 'a>,
    node_with_fields: roxmltree::Node<'a, 'a>,
    builder: DictionaryBuilder,
}

impl<'a> QuickFixReader<'a> {
    pub fn new(xml_document: &'a roxmltree::Document<'a>) -> ParseResult<Dictionary> {
        let mut reader = Self::empty(xml_document)?;
        for child in reader.node_with_fields.children() {
            if child.is_element() {
                import_field(&mut reader.builder, child)?;
            }
        }
        for child in reader.node_with_components.children() {
            if child.is_element() {
                let name = child
                    .attribute("name")
                    .ok_or(ParseDictionaryError::InvalidFormat)?
                    .to_string();
                import_component(&mut reader.builder, child, name)?;
            }
        }
        for child in reader.node_with_messages.children() {
            if child.is_element() {
                import_message(&mut reader.builder, child)?;
            }
        }
        // `StandardHeader` and `StandardTrailer` are defined in ad-hoc
        // sections of the XML files. They're always there, even if
        // potentially empty (e.g. FIX 5.0+).
        import_component(
            &mut reader.builder,
            reader.node_with_header,
            "StandardHeader",
        )?;
        import_component(
            &mut reader.builder,
            reader.node_with_trailer,
            "StandardTrailer",
        )?;
        Ok(reader.builder.build())
    }

    fn empty(xml_document: &'a roxmltree::Document<'a>) -> ParseResult<Self> {
        let root = xml_document.root_element();
        let find_tagged_child = |tag: &str| {
            root.children()
                .find(|n| n.has_tag_name(tag))
                .ok_or_else(|| {
                    ParseDictionaryError::InvalidData(format!("<{}> tag not found", tag))
                })
        };
        let version_type = root
            .attribute("type")
            .ok_or(ParseDictionaryError::InvalidData(
                "No version attribute.".to_string(),
            ))?;
        let version_major = root
            .attribute("major")
            .ok_or(ParseDictionaryError::InvalidData(
                "No major version attribute.".to_string(),
            ))?;
        let version_minor = root
            .attribute("minor")
            .ok_or(ParseDictionaryError::InvalidData(
                "No minor version attribute.".to_string(),
            ))?;
        let version_sp = root.attribute("servicepack").unwrap_or("0");
        let version = format!(
            "{}.{}.{}{}",
            version_type,
            version_major,
            version_minor,
            // Omit Service Pack ID if set to zero.
            if version_sp != "0" {
                format!("-SP{}", version_sp)
            } else {
                String::new()
            }
        );
        Ok(QuickFixReader {
            builder: DictionaryBuilder::new(version),
            node_with_header: find_tagged_child("header")?,
            node_with_trailer: find_tagged_child("trailer")?,
            node_with_messages: find_tagged_child("messages")?,
            node_with_components: find_tagged_child("components")?,
            node_with_fields: find_tagged_child("fields")?,
        })
    }
}

fn import_field(builder: &mut DictionaryBuilder, node: roxmltree::Node) -> ParseResult<InternalId> {
    if node.tag_name().name() != "field" {
        return Err(ParseDictionaryError::InvalidFormat);
    }
    let data_type_iid = import_datatype(builder, node);
    let value_restrictions = value_restrictions_from_node(node, data_type_iid);
    let name = node
        .attribute("name")
        .ok_or(ParseDictionaryError::InvalidFormat)?
        .to_string();
    let tag = node
        .attribute("number")
        .ok_or(ParseDictionaryError::InvalidFormat)?
        .parse()
        .map_err(|_| ParseDictionaryError::InvalidFormat)?;
    let field = FieldData {
        name,
        tag,
        data_type_iid,
        associated_data_tag: None,
        value_restrictions,
        required: true,
        abbr_name: None,
        base_category_abbr_name: None,
        base_category_id: None,
        description: None,
    };
    Ok(builder.add_field(field))
}

fn import_message(
    builder: &mut DictionaryBuilder,
    node: roxmltree::Node,
) -> ParseResult<InternalId> {
    debug_assert_eq!(node.tag_name().name(), "message");
    let category_iid = import_category(builder, node)?;
    let mut layout_items = LayoutItems::new();
    for child in node.children() {
        if child.is_element() {
            // We don't need to generate new IID's because we're dealing
            // with ranges.
            layout_items.push(import_layout_item(builder, child)?);
        }
    }
    let message = MessageData {
        name: node
            .attribute("name")
            .ok_or(ParseDictionaryError::InvalidFormat)?
            .to_string(),
        msg_type: node
            .attribute("msgtype")
            .ok_or(ParseDictionaryError::InvalidFormat)?
            .to_string(),
        component_id: 0,
        category_iid,
        section_id: String::new(),
        layout_items,
        abbr_name: None,
        required: true,
        elaboration: None,
        description: String::new(),
    };
    Ok(builder.add_message(message))
}

fn import_component<S: AsRef<str>>(
    builder: &mut DictionaryBuilder,
    node: roxmltree::Node,
    name: S,
) -> ParseResult<InternalId> {
    let mut layout_items = LayoutItems::new();
    for child in node.children() {
        if child.is_element() {
            layout_items.push(import_layout_item(builder, child)?);
        }
    }
    let component = ComponentData {
        id: 0,
        component_type: FixmlComponentAttributes::Block {
            // FIXME
            is_implicit: false,
            is_repeating: false,
            is_optimized: false,
        },
        layout_items,
        category_iid: 0, // FIXME
        name: name.as_ref().to_string(),
        abbr_name: None,
    };
    let iid = builder.add_component(component);
    match builder.symbol(KeyRef::ComponentByName(name.as_ref())) {
        Some(x) => Ok(*x),
        None => {
            builder
                .symbol_table
                .insert(Key::ComponentByName(name.as_ref().to_string()), iid);
            Ok(iid)
        }
    }
}

fn import_datatype(builder: &mut DictionaryBuilder, node: roxmltree::Node) -> InternalId {
    // References should only happen at <field> tags.
    debug_assert_eq!(node.tag_name().name(), "field");
    let datatype = {
        // The idenfier that QuickFIX uses for this type.
        let quickfix_name = node.attribute("type").unwrap();
        // Translate that into a real datatype.
        FixDatatype::from_quickfix_name(quickfix_name).unwrap()
    };
    // Get the official (not QuickFIX's) name of `datatype`.
    let name = datatype.name();
    match builder.symbol(KeyRef::DatatypeByName(name)) {
        Some(x) => *x,
        None => {
            let iid = builder.data_types.len() as u32;
            let data = DatatypeData {
                datatype,
                description: String::new(),
                examples: Vec::new(),
            };
            builder.data_types.push(data);
            builder
                .symbol_table
                .insert(Key::DatatypeByName(name.to_string()), iid);
            iid
        }
    }
}

fn value_restrictions_from_node(
    node: roxmltree::Node,
    _datatype: InternalId,
) -> Option<Vec<FieldEnumData>> {
    let mut values = Vec::new();
    for child in node.children() {
        if child.is_element() {
            let variant = child.attribute("enum").unwrap().to_string();
            let description = child.attribute("description").unwrap().to_string();
            let enum_value = FieldEnumData {
                value: variant,
                description,
            };
            values.push(enum_value);
        }
    }
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn import_layout_item(
    builder: &mut DictionaryBuilder,
    node: roxmltree::Node,
) -> ParseResult<LayoutItemData> {
    // This processing step requires on fields being already present in
    // the dictionary.
    debug_assert_ne!(builder.fields.len(), 0);
    let name = node.attribute("name").unwrap();
    let required = node.attribute("required").unwrap() == "Y";
    let tag = node.tag_name().name();
    let kind = match tag {
        "field" => {
            let field_iid = builder.symbol(KeyRef::FieldByName(name)).unwrap();
            LayoutItemKindData::Field { iid: *field_iid }
        }
        "component" => {
            // Components may *not* be already present.
            let component_iid = import_component(builder, node, name)?;
            LayoutItemKindData::Component { iid: component_iid }
        }
        "group" => {
            let len_field_iid = *builder.symbol(KeyRef::FieldByName(name)).unwrap();
            let mut items = Vec::new();
            for child in node.children().filter(|n| n.is_element()) {
                items.push(import_layout_item(builder, child)?);
            }
            LayoutItemKindData::Group {
                len_field_iid,
                items,
            }
        }
        _ => {
            return Err(ParseDictionaryError::InvalidFormat);
        }
    };
    let item = LayoutItemData { required, kind };
    Ok(item)
}

fn import_category(
    builder: &mut DictionaryBuilder,
    node: roxmltree::Node,
) -> ParseResult<InternalId> {
    debug_assert_eq!(node.tag_name().name(), "message");
    let name = node.attribute("msgcat").ok_or(ParseError::InvalidFormat)?;
    Ok(match builder.symbol(KeyRef::CategoryByName(name)) {
        Some(x) => *x,
        None => {
            let iid = builder.categories.len() as u32;
            builder.categories.push(CategoryData {
                name: name.to_string(),
                fixml_filename: String::new(),
            });
            builder
                .symbol_table
                .insert(Key::CategoryByName(name.to_string()), iid);
            iid
        }
    })
}

type ParseError = ParseDictionaryError;
type ParseResult<T> = Result<T, ParseError>;

/// The error type that can arise when decoding a QuickFIX Dictionary.
#[derive(Clone, Debug)]
pub enum ParseDictionaryError {
    InvalidFormat,
    InvalidData(String),
}
