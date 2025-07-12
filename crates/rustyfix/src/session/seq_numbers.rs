use std::cmp::Ordering;
use std::num::NonZeroU64;

/// A tracker for seq. numbers inside a FIX session.
#[derive(Debug, Copy, Clone)]
pub struct SeqNumbers {
    /// The expected sequence number of the next inbound message.
    pub next_inbound: u64,
    /// The expected sequence number of the next outbound message.
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

    /// Increments the next inbound sequence number.
    pub fn incr_inbound(&mut self) {
        self.next_inbound += 1;
    }

    /// Increments the next outbound sequence number.
    pub fn incr_outbound(&mut self) {
        self.next_outbound += 1;
    }

    /// Validates an inbound sequence number.
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

/// The error type for sequence number-related errors.
#[derive(Debug, Clone)]
pub enum SeqNumberError {
    /// A `ResendRequest` should be sent.
    Recover,
    /// The sequence number is too low and a `Logout` should be sent.
    TooLow,
    /// The message has no sequence number.
    NoSeqNum,
}
