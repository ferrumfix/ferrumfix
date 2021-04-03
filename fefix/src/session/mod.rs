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

//pub mod abstract_connection;
mod connection;
mod errs;
mod heartbeat_rule;
mod resend_request_range;
mod seq_numbers;

//pub use abstract_connection::AbstractConnection;
pub use connection::*;
pub use heartbeat_rule::HeartbeatRule;
pub use resend_request_range::ResendRequestRange;
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
