//! FIXML message types and field definitions.

use crate::error::EncodeError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// FIXML message types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MessageType {
    NewOrderSingle,
    ExecutionReport,
    MarketDataRequest,
    Heartbeat,
    Custom(u8),
}

/// FIXML field value types.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    Decimal(f64),
    Boolean(bool),
}

/// FIXML message representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FixmlMessage {
    pub message_type: MessageType,
    pub fields: HashMap<String, FieldValue>,
}

impl FixmlMessage {
    pub fn new(message_type: MessageType) -> Self {
        Self {
            message_type,
            fields: HashMap::new(),
        }
    }

    pub fn new_order_single(symbol: String, price: f64, qty: f64, side: String) -> Self {
        let mut message = Self::new(MessageType::NewOrderSingle);
        message.set_field("Symbol", FieldValue::String(symbol));
        message.set_field("Price", FieldValue::Decimal(price));
        message.set_field("OrderQty", FieldValue::Decimal(qty));
        message.set_field("Side", FieldValue::String(side));
        message.set_field("OrdType", FieldValue::String("2".to_string())); // Limit
        message
    }

    pub fn set_field(&mut self, name: &str, value: FieldValue) {
        self.fields.insert(name.to_string(), value);
    }

    pub fn get_field(&self, name: &str) -> Option<&FieldValue> {
        self.fields.get(name)
    }

    pub fn validate(&self) -> Result<(), EncodeError> {
        if self.message_type == MessageType::NewOrderSingle {
            self.require_field("Symbol")?;
            self.require_field("Side")?;
            self.require_field("OrderQty")?;
            self.require_field("OrdType")?;
        }
        Ok(())
    }

    fn require_field(&self, name: &str) -> Result<(), EncodeError> {
        if !self.fields.contains_key(name) {
            return Err(EncodeError::MissingRequiredField {
                field_name: name.to_string(),
            });
        }
        Ok(())
    }
}
