//! FIX message types and field definitions for Protocol Buffers encoding.

use crate::error::EncodeError;
use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// FIX message types commonly used in trading systems.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MessageType {
    /// Heartbeat (0)
    Heartbeat,
    /// Test Request (1)
    TestRequest,
    /// Resend Request (2)
    ResendRequest,
    /// Reject (3)
    Reject,
    /// Sequence Reset (4)
    SequenceReset,
    /// Logout (5)
    Logout,
    /// Indication of Interest (6)
    IndicationOfInterest,
    /// Advertisement (7)
    Advertisement,
    /// Execution Report (8)
    ExecutionReport,
    /// Order Cancel Reject (9)
    OrderCancelReject,
    /// Logon (A)
    Logon,
    /// News (B)
    News,
    /// Email (C)
    Email,
    /// New Order Single (D)
    NewOrderSingle,
    /// New Order List (E)
    NewOrderList,
    /// Order Cancel Request (F)
    OrderCancelRequest,
    /// Order Cancel/Replace Request (G)
    OrderCancelReplaceRequest,
    /// Order Status Request (H)
    OrderStatusRequest,
    /// Allocation Instruction (J)
    AllocationInstruction,
    /// List Cancel Request (K)
    ListCancelRequest,
    /// List Execute (L)
    ListExecute,
    /// List Status Request (M)
    ListStatusRequest,
    /// List Status (N)
    ListStatus,
    /// Allocation Instruction Ack (P)
    AllocationInstructionAck,
    /// Don't Know Trade (Q)
    DontKnowTrade,
    /// Quote Request (R)
    QuoteRequest,
    /// Quote (S)
    Quote,
    /// Settlement Instructions (T)
    SettlementInstructions,
    /// Market Data Request (V)
    MarketDataRequest,
    /// Market Data Snapshot/Full Refresh (W)
    MarketDataSnapshotFullRefresh,
    /// Market Data Incremental Refresh (X)
    MarketDataIncrementalRefresh,
    /// Market Data Request Reject (Y)
    MarketDataRequestReject,
    /// Quote Cancel (Z)
    QuoteCancel,
    /// Quote Status Request (a)
    QuoteStatusRequest,
    /// Mass Quote Acknowledgement (b)
    MassQuoteAcknowledgement,
    /// Security Definition Request (c)
    SecurityDefinitionRequest,
    /// Security Definition (d)
    SecurityDefinition,
    /// Security Status Request (e)
    SecurityStatusRequest,
    /// Security Status (f)
    SecurityStatus,
    /// Trading Session Status Request (g)
    TradingSessionStatusRequest,
    /// Trading Session Status (h)
    TradingSessionStatus,
    /// Mass Quote (i)
    MassQuote,
    /// Business Message Reject (j)
    BusinessMessageReject,
    /// Bid Request (k)
    BidRequest,
    /// Bid Response (l)
    BidResponse,
    /// List Strike Price (m)
    ListStrikePrice,
    /// Custom message type
    Custom(String),
}

impl MessageType {
    /// Get the FIX message type code.
    pub fn as_str(&self) -> &str {
        match self {
            MessageType::Heartbeat => "0",
            MessageType::TestRequest => "1",
            MessageType::ResendRequest => "2",
            MessageType::Reject => "3",
            MessageType::SequenceReset => "4",
            MessageType::Logout => "5",
            MessageType::IndicationOfInterest => "6",
            MessageType::Advertisement => "7",
            MessageType::ExecutionReport => "8",
            MessageType::OrderCancelReject => "9",
            MessageType::Logon => "A",
            MessageType::News => "B",
            MessageType::Email => "C",
            MessageType::NewOrderSingle => "D",
            MessageType::NewOrderList => "E",
            MessageType::OrderCancelRequest => "F",
            MessageType::OrderCancelReplaceRequest => "G",
            MessageType::OrderStatusRequest => "H",
            MessageType::AllocationInstruction => "J",
            MessageType::ListCancelRequest => "K",
            MessageType::ListExecute => "L",
            MessageType::ListStatusRequest => "M",
            MessageType::ListStatus => "N",
            MessageType::AllocationInstructionAck => "P",
            MessageType::DontKnowTrade => "Q",
            MessageType::QuoteRequest => "R",
            MessageType::Quote => "S",
            MessageType::SettlementInstructions => "T",
            MessageType::MarketDataRequest => "V",
            MessageType::MarketDataSnapshotFullRefresh => "W",
            MessageType::MarketDataIncrementalRefresh => "X",
            MessageType::MarketDataRequestReject => "Y",
            MessageType::QuoteCancel => "Z",
            MessageType::QuoteStatusRequest => "a",
            MessageType::MassQuoteAcknowledgement => "b",
            MessageType::SecurityDefinitionRequest => "c",
            MessageType::SecurityDefinition => "d",
            MessageType::SecurityStatusRequest => "e",
            MessageType::SecurityStatus => "f",
            MessageType::TradingSessionStatusRequest => "g",
            MessageType::TradingSessionStatus => "h",
            MessageType::MassQuote => "i",
            MessageType::BusinessMessageReject => "j",
            MessageType::BidRequest => "k",
            MessageType::BidResponse => "l",
            MessageType::ListStrikePrice => "m",
            MessageType::Custom(ref s) => s,
        }
    }

    /// Parse from string.
    pub fn from_str(s: &str) -> Self {
        match s {
            "0" => MessageType::Heartbeat,
            "1" => MessageType::TestRequest,
            "2" => MessageType::ResendRequest,
            "3" => MessageType::Reject,
            "4" => MessageType::SequenceReset,
            "5" => MessageType::Logout,
            "6" => MessageType::IndicationOfInterest,
            "7" => MessageType::Advertisement,
            "8" => MessageType::ExecutionReport,
            "9" => MessageType::OrderCancelReject,
            "A" => MessageType::Logon,
            "B" => MessageType::News,
            "C" => MessageType::Email,
            "D" => MessageType::NewOrderSingle,
            "E" => MessageType::NewOrderList,
            "F" => MessageType::OrderCancelRequest,
            "G" => MessageType::OrderCancelReplaceRequest,
            "H" => MessageType::OrderStatusRequest,
            "J" => MessageType::AllocationInstruction,
            "K" => MessageType::ListCancelRequest,
            "L" => MessageType::ListExecute,
            "M" => MessageType::ListStatusRequest,
            "N" => MessageType::ListStatus,
            "P" => MessageType::AllocationInstructionAck,
            "Q" => MessageType::DontKnowTrade,
            "R" => MessageType::QuoteRequest,
            "S" => MessageType::Quote,
            "T" => MessageType::SettlementInstructions,
            "V" => MessageType::MarketDataRequest,
            "W" => MessageType::MarketDataSnapshotFullRefresh,
            "X" => MessageType::MarketDataIncrementalRefresh,
            "Y" => MessageType::MarketDataRequestReject,
            "Z" => MessageType::QuoteCancel,
            "a" => MessageType::QuoteStatusRequest,
            "b" => MessageType::MassQuoteAcknowledgement,
            "c" => MessageType::SecurityDefinitionRequest,
            "d" => MessageType::SecurityDefinition,
            "e" => MessageType::SecurityStatusRequest,
            "f" => MessageType::SecurityStatus,
            "g" => MessageType::TradingSessionStatusRequest,
            "h" => MessageType::TradingSessionStatus,
            "i" => MessageType::MassQuote,
            "j" => MessageType::BusinessMessageReject,
            "k" => MessageType::BidRequest,
            "l" => MessageType::BidResponse,
            "m" => MessageType::ListStrikePrice,
            _ => MessageType::Custom(s.to_string()),
        }
    }
}

/// FIX field value types.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FieldValue {
    /// String value
    String(String),
    /// Integer value
    Int(i64),
    /// Unsigned integer value
    UInt(u64),
    /// Float value
    Float(f64),
    /// Boolean value
    Bool(bool),
    /// Raw bytes
    Bytes(Vec<u8>),
    /// Decimal value (for prices, quantities)
    Decimal {
        /// Mantissa
        mantissa: i64,
        /// Scale/exponent
        scale: i32,
    },
    /// Timestamp in nanoseconds since epoch
    Timestamp(u64),
    /// Optional field (None represents absent field)
    Optional(Option<Box<FieldValue>>),
}

impl FieldValue {
    /// Convert to string if possible.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            FieldValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Convert to integer if possible.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            FieldValue::Int(i) => Some(*i),
            FieldValue::UInt(u) => (*u as i64).into(),
            _ => None,
        }
    }

    /// Convert to float if possible.
    pub fn as_float(&self) -> Option<f64> {
        match self {
            FieldValue::Float(f) => Some(*f),
            FieldValue::Int(i) => Some(*i as f64),
            FieldValue::UInt(u) => Some(*u as f64),
            FieldValue::Decimal { mantissa, scale } => Some(*mantissa as f64 / 10_f64.powi(*scale)),
            _ => None,
        }
    }

    /// Check if field is present (not None in Optional).
    pub fn is_present(&self) -> bool {
        match self {
            FieldValue::Optional(None) => false,
            _ => true,
        }
    }
}

/// FIX message representation optimized for Protocol Buffers encoding.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FixMessage {
    /// Message type
    pub message_type: MessageType,
    /// FIX fields as tag-value pairs
    pub fields: HashMap<u32, FieldValue>,
    /// Sequence number
    pub seq_num: Option<u32>,
    /// Sender CompID
    pub sender_comp_id: Option<String>,
    /// Target CompID  
    pub target_comp_id: Option<String>,
    /// Sending timestamp (nanoseconds since epoch)
    pub sending_time: Option<u64>,
}

impl FixMessage {
    /// Create a new FIX message.
    pub fn new(message_type: MessageType) -> Self {
        Self {
            message_type,
            fields: HashMap::new(),
            seq_num: None,
            sender_comp_id: None,
            target_comp_id: None,
            sending_time: None,
        }
    }

    /// Create a New Order Single message (MessageType D).
    pub fn new_order_single(symbol: String, price: f64, quantity: f64, side: String) -> Self {
        let mut message = Self::new(MessageType::NewOrderSingle);

        // Add required fields for New Order Single
        message.set_field(55, FieldValue::String(symbol)); // Symbol
        message.set_field(44, FieldValue::Float(price)); // Price
        message.set_field(38, FieldValue::Float(quantity)); // OrderQty
        message.set_field(54, FieldValue::String(side)); // Side
        message.set_field(40, FieldValue::String("2".to_string())); // OrdType = Limit
        message.set_field(59, FieldValue::String("0".to_string())); // TimeInForce = Day

        message
    }

    /// Create an Execution Report message.
    pub fn execution_report(
        order_id: String,
        exec_id: String,
        exec_type: String,
        ord_status: String,
        symbol: String,
        side: String,
        exec_qty: f64,
        exec_price: f64,
    ) -> Self {
        let mut message = Self::new(MessageType::ExecutionReport);

        message.set_field(37, FieldValue::String(order_id)); // OrderID
        message.set_field(17, FieldValue::String(exec_id)); // ExecID
        message.set_field(150, FieldValue::String(exec_type)); // ExecType
        message.set_field(39, FieldValue::String(ord_status)); // OrdStatus
        message.set_field(55, FieldValue::String(symbol)); // Symbol
        message.set_field(54, FieldValue::String(side)); // Side
        message.set_field(32, FieldValue::Float(exec_qty)); // LastQty
        message.set_field(31, FieldValue::Float(exec_price)); // LastPx

        message
    }

    /// Create a Market Data Request message.
    pub fn market_data_request(
        md_req_id: String,
        subscription_type: String,
        market_depth: u32,
        symbols: Vec<String>,
    ) -> Self {
        let mut message = Self::new(MessageType::MarketDataRequest);

        message.set_field(262, FieldValue::String(md_req_id)); // MDReqID
        message.set_field(263, FieldValue::String(subscription_type)); // SubscriptionRequestType
        message.set_field(264, FieldValue::UInt(market_depth as u64)); // MarketDepth

        // Add symbols (simplified - real implementation would use repeating groups)
        message.set_field(146, FieldValue::UInt(symbols.len() as u64)); // NoRelatedSym
        for (i, symbol) in symbols.into_iter().enumerate() {
            message.set_field(55 + (i as u32) * 1000, FieldValue::String(symbol));
            // Symbol[i]
        }

        message
    }

    /// Set a field value.
    pub fn set_field(&mut self, tag: u32, value: FieldValue) {
        self.fields.insert(tag, value);
    }

    /// Get a field value.
    pub fn get_field(&self, tag: u32) -> Option<&FieldValue> {
        self.fields.get(&tag)
    }

    /// Remove a field.
    pub fn remove_field(&mut self, tag: u32) -> Option<FieldValue> {
        self.fields.remove(&tag)
    }

    /// Check if field exists.
    pub fn has_field(&self, tag: u32) -> bool {
        self.fields.contains_key(&tag)
    }

    /// Get all field tags.
    pub fn field_tags(&self) -> impl Iterator<Item = u32> + '_ {
        self.fields.keys().copied()
    }

    /// Set sequence number.
    pub fn with_seq_num(mut self, seq_num: u32) -> Self {
        self.seq_num = Some(seq_num);
        self
    }

    /// Set sender CompID.
    pub fn with_sender_comp_id(mut self, sender_comp_id: String) -> Self {
        self.sender_comp_id = Some(sender_comp_id);
        self
    }

    /// Set target CompID.
    pub fn with_target_comp_id(mut self, target_comp_id: String) -> Self {
        self.target_comp_id = Some(target_comp_id);
        self
    }

    /// Set sending time.
    pub fn with_sending_time(mut self, sending_time: u64) -> Self {
        self.sending_time = Some(sending_time);
        self
    }

    /// Validate required fields for the message type.
    pub fn validate(&self) -> Result<(), EncodeError> {
        match self.message_type {
            MessageType::NewOrderSingle => {
                self.require_field(55)?; // Symbol
                self.require_field(54)?; // Side
                self.require_field(38)?; // OrderQty
                self.require_field(40)?; // OrdType
            }
            MessageType::ExecutionReport => {
                self.require_field(37)?; // OrderID
                self.require_field(17)?; // ExecID
                self.require_field(150)?; // ExecType
                self.require_field(39)?; // OrdStatus
                self.require_field(55)?; // Symbol
                self.require_field(54)?; // Side
            }
            MessageType::MarketDataRequest => {
                self.require_field(262)?; // MDReqID
                self.require_field(263)?; // SubscriptionRequestType
            }
            // Add validation for other message types as needed
            _ => {}
        }

        Ok(())
    }

    /// Helper to check required field presence.
    fn require_field(&self, tag: u32) -> Result<(), EncodeError> {
        if !self.has_field(tag) {
            return Err(EncodeError::MissingRequiredField { field_id: tag });
        }
        Ok(())
    }

    /// Calculate approximate message size for buffer pre-allocation.
    pub fn estimated_size(&self) -> usize {
        let mut size = 100; // Base overhead

        for (_tag, value) in &self.fields {
            size += 8; // Tag overhead
            size += match value {
                FieldValue::String(s) => s.len() + 4,
                FieldValue::Int(_) | FieldValue::UInt(_) => 12,
                FieldValue::Float(_) => 12,
                FieldValue::Bool(_) => 4,
                FieldValue::Bytes(b) => b.len() + 4,
                FieldValue::Decimal { .. } => 16,
                FieldValue::Timestamp(_) => 12,
                FieldValue::Optional(Some(v)) => v.estimated_size(),
                FieldValue::Optional(None) => 0,
            };
        }

        size
    }
}

impl FieldValue {
    /// Calculate estimated size for this field value.
    fn estimated_size(&self) -> usize {
        match self {
            FieldValue::String(s) => s.len() + 4,
            FieldValue::Int(_) | FieldValue::UInt(_) => 12,
            FieldValue::Float(_) => 12,
            FieldValue::Bool(_) => 4,
            FieldValue::Bytes(b) => b.len() + 4,
            FieldValue::Decimal { .. } => 16,
            FieldValue::Timestamp(_) => 12,
            FieldValue::Optional(Some(v)) => v.estimated_size(),
            FieldValue::Optional(None) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_type_conversion() {
        assert_eq!(MessageType::NewOrderSingle.as_str(), "D");
        assert_eq!(MessageType::from_str("D"), MessageType::NewOrderSingle);
        assert_eq!(MessageType::ExecutionReport.as_str(), "8");
        assert_eq!(MessageType::from_str("8"), MessageType::ExecutionReport);
    }

    #[test]
    fn test_new_order_single_creation() {
        let message = FixMessage::new_order_single(
            "BTCUSD".to_string(),
            50000.0,
            1.5,
            "1".to_string(), // Buy
        );

        assert_eq!(message.message_type, MessageType::NewOrderSingle);
        assert_eq!(
            message.get_field(55).unwrap().as_string().unwrap(),
            "BTCUSD"
        );
        assert_eq!(message.get_field(44).unwrap().as_float().unwrap(), 50000.0);
        assert_eq!(message.get_field(38).unwrap().as_float().unwrap(), 1.5);
        assert_eq!(message.get_field(54).unwrap().as_string().unwrap(), "1");
    }

    #[test]
    fn test_field_value_conversions() {
        let string_val = FieldValue::String("test".to_string());
        assert_eq!(string_val.as_string().unwrap(), "test");

        let int_val = FieldValue::Int(42);
        assert_eq!(int_val.as_int().unwrap(), 42);
        assert_eq!(int_val.as_float().unwrap(), 42.0);

        let decimal_val = FieldValue::Decimal {
            mantissa: 12345,
            scale: 2,
        };
        assert_eq!(decimal_val.as_float().unwrap(), 123.45);
    }

    #[test]
    fn test_message_validation() {
        let mut message = FixMessage::new(MessageType::NewOrderSingle);

        // Should fail validation - missing required fields
        assert!(message.validate().is_err());

        // Add required fields
        message.set_field(55, FieldValue::String("BTCUSD".to_string()));
        message.set_field(54, FieldValue::String("1".to_string()));
        message.set_field(38, FieldValue::Float(1.0));
        message.set_field(40, FieldValue::String("2".to_string()));

        // Should pass validation
        assert!(message.validate().is_ok());
    }

    #[test]
    fn test_optional_field_values() {
        let present = FieldValue::Optional(Some(Box::new(FieldValue::String("test".to_string()))));
        let absent = FieldValue::Optional(None);

        assert!(present.is_present());
        assert!(!absent.is_present());
    }

    #[test]
    fn test_execution_report_creation() {
        let message = FixMessage::execution_report(
            "ORDER123".to_string(),
            "EXEC456".to_string(),
            "F".to_string(), // Fill
            "2".to_string(), // Filled
            "BTCUSD".to_string(),
            "1".to_string(), // Buy
            1.0,
            50000.0,
        );

        assert_eq!(message.message_type, MessageType::ExecutionReport);
        assert_eq!(
            message.get_field(37).unwrap().as_string().unwrap(),
            "ORDER123"
        );
        assert_eq!(
            message.get_field(17).unwrap().as_string().unwrap(),
            "EXEC456"
        );
    }
}
