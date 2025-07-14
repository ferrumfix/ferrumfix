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

                    // Parse attributes as fields
                    for attr in e.attributes() {
                        let attr = attr?;
                        let key = std::str::from_utf8(attr.key.as_ref())?;
                        let value = std::str::from_utf8(&attr.value)?;

                        let field_value = self.parse_field_value(value);
                        fields.insert(key.to_string(), field_value);
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

    fn parse_field_value(&self, value: &str) -> FieldValue {
        // Try to parse as different types
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
