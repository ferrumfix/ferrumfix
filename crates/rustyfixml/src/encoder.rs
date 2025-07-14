//! FIXML encoder for XML generation.

use crate::{EncodeError, FieldValue, FixmlError, FixmlMessage, MessageType};
use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Writer,
};
use std::io::Cursor;

#[derive(Debug, Default)]
pub struct EncodeConfig {
    pub pretty_print: bool,
    pub include_schema: bool,
}

#[derive(Debug)]
pub struct FixmlEncoder {
    config: EncodeConfig,
}

impl FixmlEncoder {
    pub fn new() -> Self {
        Self {
            config: EncodeConfig::default(),
        }
    }

    pub fn encode(&self, message: &FixmlMessage) -> Result<String, FixmlError> {
        message.validate()?;

        let mut writer = Writer::new(Cursor::new(Vec::new()));

        // XML declaration
        writer.write_event(Event::Decl(quick_xml::events::BytesDecl::new(
            "1.0",
            Some("UTF-8"),
            None,
        )))?;

        // FIXML root element
        let mut fixml_start = BytesStart::new("FIXML");
        fixml_start.push_attribute(("xmlns", "http://www.fixprotocol.org/FIXML-4-4"));
        writer.write_event(Event::Start(fixml_start))?;

        // Message element
        let message_tag = self.get_message_tag(&message.message_type)?;
        let mut msg_start = BytesStart::new(message_tag);

        // Add fields as attributes with type hints
        for (name, value) in &message.fields {
            let (attr_name, value_str) = self.format_field_with_type_hint(name, value);
            msg_start.push_attribute((attr_name.as_str(), value_str.as_str()));
        }

        writer.write_event(Event::Empty(msg_start))?;

        // Close FIXML
        writer.write_event(Event::End(BytesEnd::new("FIXML")))?;

        let result = writer.into_inner().into_inner();
        String::from_utf8(result).map_err(FixmlError::from)
    }

    fn format_field_with_type_hint(&self, name: &str, value: &FieldValue) -> (String, String) {
        match value {
            FieldValue::String(s) => (format!("{name}:String"), s.clone()),
            FieldValue::Integer(i) => (format!("{name}:Integer"), i.to_string()),
            FieldValue::Decimal(d) => (format!("{name}:Decimal"), d.to_string()),
            FieldValue::Boolean(b) => (format!("{name}:Boolean"), b.to_string()),
        }
    }

    fn get_message_tag(&self, msg_type: &MessageType) -> Result<&'static str, EncodeError> {
        match msg_type {
            MessageType::NewOrderSingle => Ok("Order"),
            MessageType::ExecutionReport => Ok("ExecRpt"),
            MessageType::MarketDataRequest => Ok("MktDataReq"),
            MessageType::Heartbeat => Ok("Heartbeat"),
            MessageType::Custom(_) => Err(EncodeError::UnsupportedMessageType {
                message_type: "Custom".to_string(),
            }),
        }
    }
}

impl Default for FixmlEncoder {
    fn default() -> Self {
        Self::new()
    }
}
