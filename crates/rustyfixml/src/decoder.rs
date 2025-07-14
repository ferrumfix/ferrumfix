//! FIXML decoder for XML parsing.

use crate::{DecodeError, FieldValue, FixmlError, FixmlMessage, MessageType};
use quick_xml::{events::Event, Reader};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct DecodeConfig {
    pub validate_schema: bool,
    pub strict_parsing: bool,
}

#[derive(Debug)]
pub struct FixmlDecoder {
    config: DecodeConfig,
}

impl FixmlDecoder {
    pub fn new() -> Self {
        Self {
            config: DecodeConfig::default(),
        }
    }

    pub fn decode(&self, xml: &str) -> Result<FixmlMessage, FixmlError> {
        let mut reader = Reader::from_str(xml);
        let mut buf = Vec::new();
        let mut message: Option<FixmlMessage> = None;

        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    let name_ref = e.name();
                    let name = std::str::from_utf8(name_ref.as_ref())?;

                    if name == "FIXML" {
                        continue; // Skip root element
                    }

                    // Parse message
                    let msg_type = self.parse_message_type(name)?;
                    let mut fields = HashMap::new();

                    // Parse attributes as fields with type hints
                    for attr in e.attributes() {
                        let attr = attr?;
                        let key = std::str::from_utf8(attr.key.as_ref())?;
                        let value = std::str::from_utf8(&attr.value)?;

                        let (field_name, field_value) =
                            self.parse_field_with_type_hint(key, value)?;
                        fields.insert(field_name, field_value);
                    }

                    message = Some(FixmlMessage {
                        message_type: msg_type,
                        fields,
                    });
                }
                Event::Eof => break,
                _ => {}
            }
            buf.clear();
        }

        message.ok_or_else(|| {
            FixmlError::Decode(DecodeError::InvalidXmlStructure {
                reason: "No message found".to_string(),
            })
        })
    }

    fn parse_message_type(&self, tag: &str) -> Result<MessageType, DecodeError> {
        match tag {
            "Order" => Ok(MessageType::NewOrderSingle),
            "ExecRpt" => Ok(MessageType::ExecutionReport),
            "MktDataReq" => Ok(MessageType::MarketDataRequest),
            "Heartbeat" => Ok(MessageType::Heartbeat),
            _ => Err(DecodeError::UnknownMessageType {
                message_type: tag.to_string(),
            }),
        }
    }

    fn parse_field_with_type_hint(
        &self,
        attr_name: &str,
        value: &str,
    ) -> Result<(String, FieldValue), DecodeError> {
        // Parse attribute name for type hint (format: "FieldName:Type")
        if let Some(colon_pos) = attr_name.find(':') {
            let field_name = attr_name[..colon_pos].to_string();
            let type_hint = &attr_name[colon_pos + 1..];

            let field_value = match type_hint {
                "String" => FieldValue::String(value.to_string()),
                "Integer" => value.parse::<i64>().map(FieldValue::Integer).map_err(|_| {
                    DecodeError::InvalidFieldValue {
                        field_name: field_name.clone(),
                        value: value.to_string(),
                        expected_type: "Integer".to_string(),
                    }
                })?,
                "Decimal" => value.parse::<f64>().map(FieldValue::Decimal).map_err(|_| {
                    DecodeError::InvalidFieldValue {
                        field_name: field_name.clone(),
                        value: value.to_string(),
                        expected_type: "Decimal".to_string(),
                    }
                })?,
                "Boolean" => value
                    .parse::<bool>()
                    .map(FieldValue::Boolean)
                    .map_err(|_| DecodeError::InvalidFieldValue {
                        field_name: field_name.clone(),
                        value: value.to_string(),
                        expected_type: "Boolean".to_string(),
                    })?,
                _ => {
                    // Unknown type hint, fall back to inference
                    self.parse_field_value(value)
                }
            };

            Ok((field_name, field_value))
        } else {
            // No type hint, fall back to inference for backward compatibility
            Ok((attr_name.to_string(), self.parse_field_value(value)))
        }
    }

    fn parse_field_value(&self, value: &str) -> FieldValue {
        // Try to parse as different types (fallback for backward compatibility)
        if let Ok(i) = value.parse::<i64>() {
            FieldValue::Integer(i)
        } else if let Ok(d) = value.parse::<f64>() {
            FieldValue::Decimal(d)
        } else if let Ok(b) = value.parse::<bool>() {
            FieldValue::Boolean(b)
        } else {
            FieldValue::String(value.to_string())
        }
    }
}

impl Default for FixmlDecoder {
    fn default() -> Self {
        Self::new()
    }
}
