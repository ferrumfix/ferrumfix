//! FIX message processing between counterparties.
//!
//! ![](https://www.fixtrading.org/wp-content/uploads/2020/06/fixsessionlayerconceptualsimple.png)
//!
//! The above is a conceptual view of the FIX Session layer, complete with its
//! state machine and transitions between initiator and acceptor.

pub mod backends;
mod config;
mod connection;
mod errs;
mod event_loop;
mod heartbeat_rule;
mod resend_request_range;
mod seq_numbers;

pub use config::{Config, Configure};
pub use connection::*;
pub use event_loop::*;
pub use heartbeat_rule::HeartbeatRule;
pub use resend_request_range::ResendRequestRange;
pub use seq_numbers::{SeqNumberError, SeqNumbers};

use crate::tagvalue::Message;
use std::ops::Range;

/// The owner of a [`FixConnection`]. It can react to events, store incoming
/// messages, send messages, etc..
pub trait Backend: Clone {
    /// The type of errors that can arise during a [`FixConnection`].
    type Error;

    #[inline]
    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Callback for processing incoming FIX application messages.
    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error>;

    /// Callback for post-processing outbound FIX messages.
    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error>;

    /// Callback for processing incoming FIX messages.
    #[inline]
    fn on_inbound_message(
        &mut self,
        message: Message<&[u8]>,
        is_app: bool,
    ) -> Result<(), Self::Error> {
        if is_app {
            self.on_inbound_app_message(message)
        } else {
            Ok(())
        }
    }

    /// Callback for processing `ResendRequest` messages.
    fn on_resend_request(&mut self, range: Range<u64>) -> Result<(), Self::Error>;

    /// Callback for additional logic to execute after a valid [`FixConnection`]
    /// is established with the counterparty.
    fn on_successful_handshake(&mut self) -> Result<(), Self::Error>;

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error>;

    fn pending_message(&mut self) -> Option<&[u8]>;
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
    Production {
        /// Flag that indicates whether or not test messages should be allowed
        /// in this production environment.
        allow_test: bool,
    },
    /// Production messages will be refused under this environment setting.
    Testing,
}

impl Environment {
    /// Returns `true` if an only if `self` allows test messages, depending on
    /// the provided configuration.
    ///
    /// This is used to determine whether to accept or refuse incoming test
    /// messages within a [`FixConnection`].
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix::session::Environment;
    ///
    /// assert_eq!(Environment::Testing.allows_testing(), true);
    /// assert_eq!(Environment::Production { allow_test: true }, true);
    /// assert_eq!(Environment::Production { allow_test: false }, false);
    /// ```
    #[inline]
    pub fn allows_testing(&self) -> bool {
        match self {
            Self::Production { allow_test } => *allow_test,
            Self::Testing => true,
        }
    }
}
