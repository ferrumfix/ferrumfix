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

pub mod backends;
mod config;
mod connection;
mod errs;
mod heartbeat_rule;
mod resend_request_range;
mod seq_numbers;

pub use config::{Config, Configure};
pub use connection::*;
pub use heartbeat_rule::HeartbeatRule;
pub use resend_request_range::ResendRequestRange;
pub use seq_numbers::{SeqNumberError, SeqNumbers};

use std::future::Future;
use std::ops::Range;

pub trait Backend<'a> {
    type Error;
    type IStore: Future<Output = Result<(), Self::Error>>;
    type IMessages: Future<Output = Result<Vec<u8>, Self::Error>>;
    type ISessionState: Future<Output = Result<Option<State>, Self::Error>>;
    type ICreateSession: Future<Output = Result<u128, Self::Error>>;

    fn create_session(&mut self) -> Self::ICreateSession;

    fn session_state(&self, session_id: u128) -> Self::ISessionState;

    fn messages(&'a self, session_id: u128, range: Range<u64>) -> Self::IMessages;

    fn store(&mut self, session_id: u128, fix_message: &[u8]) -> Self::IStore;
}

#[derive(Debug, Clone)]
pub struct State {
    next_inbound: u64,
    next_outbound: u64,
}

/// An indicator for the kind of environment relative to a FIX Connection.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Environment {
    /// Test messages will be ignored or refused under this environment setting.
    Production { allow_test: bool },
    /// Production messages will be refused under this environment setting.
    Testing,
}

impl Environment {
    #[inline(always)]
    pub fn allows_testing(&self) -> bool {
        match self {
            Self::Production { allow_test } => *allow_test,
            Self::Testing => true,
        }
    }
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
