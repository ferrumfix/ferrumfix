//! FIX message processing between counterparties.
//!
//! ![](https://www.fixtrading.org/wp-content/uploads/2020/06/fixsessionlayerconceptualsimple.png)
//!
//! The above is a conceptual view of the FIX Session layer, complete with its
//! state machine and transitions between initiator and acceptor.

pub mod backends;
mod config;
//mod connection; FIXME
mod environment;
mod errs;
mod event_loop;
mod heartbeat_rule;
mod resend_request_range;
mod seq_numbers;

pub use config::{Config, Configure};
// pub use connection::*; FIXME
pub use environment::Environment;
pub use event_loop::*;
pub use heartbeat_rule::HeartbeatRule;
pub use resend_request_range::ResendRequestRange;
pub use seq_numbers::{SeqNumberError, SeqNumbers};

use crate::{tagvalue::Message, FieldType, SetField};
use std::ops::Range;

/// The owner of a [`FixConnection`]. It can react to events, store incoming
/// messages, send messages, etc..
pub trait Backend: Clone {
    /// The type of errors that can arise during a [`FixConnection`].
    type Error: for<'a> FieldType<'a>;

    fn sender_comp_id(&self) -> &[u8];
    fn target_comp_id(&self) -> &[u8];

    fn message_encoding(&self) -> Option<&[u8]> {
        None
    }

    fn set_sender_and_target<'a>(&'a self, msg: &mut impl SetField<u32>) {
        msg.set(49, self.sender_comp_id());
        msg.set(56, self.target_comp_id());
    }

    fn environment(&self) -> Environment {
        Environment::Production { allow_test: false }
    }

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

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MsgSeqNumCounter(u64);

impl MsgSeqNumCounter {
    pub const START: Self = Self(0);

    #[inline]
    pub fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    #[inline]
    pub fn expected(&self) -> u64 {
        self.0 + 1
    }
}

impl Iterator for MsgSeqNumCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(MsgSeqNumCounter::next(self))
    }
}

// FIXME
#[derive(Debug)]
pub struct FixConnection;
