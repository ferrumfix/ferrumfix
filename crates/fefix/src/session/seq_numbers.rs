use std::cmp::Ordering;
use std::num::NonZeroU64;

/// A tracker for seq. numbers inside a FIX session.
#[derive(Debug, Copy, Clone)]
pub struct SeqNumbers {
    /// Expected inbound `MsgSeqNum <34>`.
    pub next_inbound: u64,
    /// Next outbound `MsgSeqNum <34>`.
    pub next_outbound: u64,
}

impl SeqNumbers {
    /// Creates a new tracker starting from `inbound` and `outbound`.
    pub fn new(inbound: NonZeroU64, outbound: NonZeroU64) -> Self {
        Self {
            next_inbound: inbound.get(),
            next_outbound: outbound.get(),
        }
    }

    /// Returns the expected seq. number of the next inbound message.
    pub fn next_inbound(&self) -> u64 {
        self.next_inbound
    }

    /// Returns the expected seq. number of the next outbound message.
    pub fn next_outbound(&self) -> u64 {
        self.next_outbound
    }

    /// Increments expected inbound sequence number by one.
    pub fn incr_inbound(&mut self) {
        self.next_inbound += 1;
    }

    /// Increments next outbound sequence number by one.
    pub fn incr_outbound(&mut self) {
        self.next_outbound += 1;
    }

    /// Validates inbound sequence progression against the current expectation.
    pub fn validate_inbound(&self, inbound: u64) -> Result<(), SeqNumberError> {
        match inbound.cmp(&self.next_inbound) {
            Ordering::Equal => Ok(()),
            Ordering::Less => Err(SeqNumberError::TooLow),
            Ordering::Greater => Err(SeqNumberError::Recover),
        }
    }
}

impl Default for SeqNumbers {
    /// Returns the default [`SeqNumbers`](SeqNumbers) at the start of a new FIX
    /// session, i.e. both 1.
    fn default() -> Self {
        Self {
            next_inbound: 1,
            next_outbound: 1,
        }
    }
}

/// Sequence-number validation outcomes for inbound FIX messages.
#[derive(Debug, Clone)]
pub enum SeqNumberError {
    /// Sequence gap detected; recovery logic is required.
    Recover,
    /// Inbound sequence number is behind expectation.
    TooLow,
    /// Mandatory `MsgSeqNum <34>` was missing.
    NoSeqNum,
}
