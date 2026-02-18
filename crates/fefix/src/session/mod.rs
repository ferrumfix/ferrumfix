//! FIX message processing between counterparties.
//!
//! ![](https://www.fixtrading.org/wp-content/uploads/2020/06/fixsessionlayerconceptualsimple.png)
//!
//! The above is a conceptual view of the FIX Session layer, complete with its
//! state machine and transitions between initiator and acceptor.

pub mod backends;
mod config;
mod connection;
mod environment;
mod errs;
mod event_loop;
mod heartbeat_rule;
mod resend_request_range;
mod seq_numbers;
/// Durable session storage primitives.
///
/// Includes [`InMemorySessionStore`] and optional sqlite support via
/// `session-store-sqlite`.
pub mod store;

use crate::tagvalue::Message;
use crate::SetField;
pub use config::{Config, Configure, SessionRole};
pub use connection::{FixConnection, RunError, RunOutcome, SessionState, SessionStatus};
pub use environment::Environment;
pub use event_loop::*;
pub use heartbeat_rule::HeartbeatRule;
pub use resend_request_range::ResendRequestRange;
pub use seq_numbers::{SeqNumberError, SeqNumbers};
pub use store::{InMemorySessionStore, SessionKey, SessionStore, StoredAppMessage};
#[cfg(feature = "session-store-sqlite")]
pub use store::{SqliteSessionStore, SqliteStoreOptions};

/// The owner of a [`FixConnection`]. It can react to events, handle inbound
/// application messages, and provide outbound application flow.
pub trait Backend {
    /// The type of errors that can arise during a [`FixConnection`].
    type Error;

    /// Returns this session's `SenderCompID <49>`.
    fn sender_comp_id(&self) -> &[u8];
    /// Returns this session's `TargetCompID <56>`.
    fn target_comp_id(&self) -> &[u8];

    /// Optional `MessageEncoding <347>`.
    fn message_encoding(&self) -> Option<&[u8]> {
        None
    }

    /// Writes sender and target IDs into an outbound message.
    fn set_sender_and_target(&self, msg: &mut impl SetField<u32>) {
        msg.set(49, self.sender_comp_id());
        msg.set(56, self.target_comp_id());
    }

    /// Returns this backend's environment mode.
    fn environment(&self) -> Environment {
        Environment::Production { allow_test: false }
    }

    /// Callback triggered when a heartbeat deadline is hit.
    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Callback for processing incoming FIX application messages.
    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error>;

    /// Callback for post-processing outbound FIX messages.
    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error>;

    /// Callback for processing incoming FIX messages.
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

    /// Callback for additional logic to execute after a valid [`FixConnection`]
    /// is established with the counterparty.
    fn on_successful_handshake(&mut self) -> Result<(), Self::Error>;

    /// Polls one outbound message to be sent on the wire.
    ///
    /// Outbound messages returned by this callback are expected to be complete,
    /// serialized FIX messages. The engine sends the bytes as-is and manages
    /// durable resend/replay semantics via [`SessionStore`].
    fn poll_outbound(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }
}
