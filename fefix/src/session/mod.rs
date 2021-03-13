//! FIX message processing between counterparties.
//!
//! To establish a reliable FIX connection, peers must adopt a session protocol.
//! [`Initiator`] is used to connect to service offerers and [`Acceptor`] is used
//! to accept incoming connections. This is akin to the client-server
//! architecture.
//!
//! ![](https://www.fixtrading.org/wp-content/uploads/2020/06/fixsessionlayerconceptualsimple.png)
//!
//! The above is a conceptual view of the FIX Session layer, complete with its
//! state machine and transitions between them. Both [`Initiator`] and
//! [`Acceptor`] abstract over such details and present users with a single entry
//! point, namely [`Initiator::feed`] and [`Acceptor::feed`].

use crate::tags::fix44 as tags;
use crate::tagvalue::FixFieldValue;
use crate::tagvalue::MessageRnd;
use futures_lite::prelude::*;
use std::time::Duration;
use uuid::Uuid;

mod acceptor;
mod errs;
mod heartbeat_rule;
mod initiator;
mod seq_numbers;

pub use acceptor::{Acceptor, Configuration, EventInbound, EventOutbound};
pub use heartbeat_rule::HeartbeatRule;
pub use initiator::Initiator;
pub use seq_numbers::{SeqNumberError, SeqNumbers};

/// An indicator for the kind of environment relative to a FIX Connection.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Environment {
    /// Test messages will be refused under this environment setting.
    ProductionDisallowTest,
    /// Test messages will be ignored under this environment setting.
    ProductionAllowTest,
    /// Production messages will be refused under this environment setting.
    Testing,
}

#[derive(Debug, Clone)]
pub enum SessionRejectReason {
    InvalidTagNumber,
    RequiredTagMissing,
    TagNotDefinedForThisMessageType,
    UndefinedTag,
    TagSpecifiedWithoutAValue,
    ValueIsIncorrect,
    IncorrectDataFormatForValue,
    DecryptionProblem,
    SignatureProblem,
    CompIDProblem,
    SendingTimeAccuracyProblem,
    InvalidMsgType,
    XMLValidationError,
    TagAppearsMoreThanOnce,
    TagSpecifiedOutOfRequiredOrder,
    RepeatingGroupFieldsOutOfOrder,
    IncorrectNumInGroupCountForRepeatingGroup,
    FieldDelimiterInFieldValue,
    InvalidUnsupportedAppVersion,
    Other,
}

impl From<u32> for SessionRejectReason {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::InvalidTagNumber,
            1 => Self::RequiredTagMissing,
            2 => Self::TagNotDefinedForThisMessageType,
            3 => Self::UndefinedTag,
            4 => Self::TagSpecifiedWithoutAValue,
            5 => Self::ValueIsIncorrect,
            6 => Self::IncorrectDataFormatForValue,
            7 => Self::DecryptionProblem,
            8 => Self::SignatureProblem,
            9 => Self::CompIDProblem,
            10 => Self::SendingTimeAccuracyProblem,
            11 => Self::InvalidMsgType,
            12 => Self::XMLValidationError,
            13 => Self::TagAppearsMoreThanOnce,
            14 => Self::TagSpecifiedOutOfRequiredOrder,
            15 => Self::RepeatingGroupFieldsOutOfOrder,
            16 => Self::IncorrectNumInGroupCountForRepeatingGroup,
            17 => Self::FieldDelimiterInFieldValue,
            18 => Self::InvalidUnsupportedAppVersion,
            _ => Self::Other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::FixFieldValue;

    const COMPANY_ID: &str = "FOOBAR-INC";

    fn acceptor() -> Acceptor {
        let mut config = Configuration::new(COMPANY_ID.to_string());
        config.with_hb_rule(HeartbeatRule::Any);
        config.with_environment(Environment::ProductionDisallowTest);
        config.acceptor()
    }

    /// Condition:
    ///
    /// > Valid Logon(35=A) request message received.
    ///
    /// Expected behavior:
    ///
    /// > Respond with Logon(35=A) acknowledgement message.
    #[tokio::test]
    async fn testcase_1s_a_1() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "A".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 1);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(tags::MSG_TYPE).unwrap(),
                    FixFieldValue::string(b"A").unwrap()
                );
                assert_eq!(
                    *response.get_field(tags::SENDER_COMP_ID).unwrap(),
                    FixFieldValue::string(COMPANY_ID.as_bytes()).unwrap()
                );
                assert!(response.get_field(tags::TEST_REQ_ID).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        assert!(events.next().is_none());
    }

    /// Condition:
    ///
    /// > Valid Logon(35=A) request message received.
    ///
    /// Expected behavior:
    ///
    /// > If MsgSeqNum(34) > NextNumIn send ResendRequest(35=2).
    #[tokio::test]
    async fn testcase_1s_a_2() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "A".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 42);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(tags::MSG_TYPE).unwrap(),
                    FixFieldValue::string(b"2").unwrap()
                );
                assert_eq!(
                    *response.get_field(tags::SENDER_COMP_ID).unwrap(),
                    FixFieldValue::string(COMPANY_ID.as_bytes()).unwrap()
                );
                assert!(response.get_field(tags::TEST_REQ_ID).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        assert!(events.next().is_none());
    }

    /// Condition:
    ///
    /// > Logon(35=A) message received with duplicate identity (e.g. same IP,
    /// port, SenderCompID(49), TargetCompID(56), etc. as existing connection).
    ///
    /// Expected behavior:
    ///
    /// > 1. Generate an error condition in test output.
    /// > 2. Disconnect without sending a message (Note: sending a Reject or
    /// Logout(35=5) would consume a MsgSeqNum(34)).
    #[tokio::test]
    async fn testcase_1s_b() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "A".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 1);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg.clone()));
        // First Logon message is fine.
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(tags::MSG_TYPE).unwrap(),
                    FixFieldValue::string(b"A").unwrap()
                );
                assert_eq!(
                    *response.get_field(tags::SENDER_COMP_ID).unwrap(),
                    FixFieldValue::string(COMPANY_ID.as_bytes()).unwrap()
                );
                assert!(response.get_field(tags::TEST_REQ_ID).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        // The second one is ignored.
        assert!(events.next().is_none());
    }

    /// Condition:
    ///
    /// > First message received is not a Logon(35=A) message.
    ///
    /// Expected behavior:
    ///
    /// > 1. Log an error “First message not a logon”.
    /// > 2. Disconnect.
    #[test]
    fn testcase_2s() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "0".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 1);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
        // First Logon message is fine.
        match events.next().unwrap() {
            EventOutbound::Terminate => (),
            _ => assert!(false),
        };
        // The second one is ignored.
        assert!(events.next().is_none());
    }
}
