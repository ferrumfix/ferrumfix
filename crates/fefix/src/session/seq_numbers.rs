use std::cmp::Ordering;
use std::num::NonZeroU64;

/// A tracker for inbound and outbound sequence numbers inside a FIX session.
///
/// You can initialize [`SeqNumbers`] with [`Default`] (which sets both inbound
/// and outbound values to 1), or use custom values with [`SeqNumbers::new`].
#[derive(Debug, Copy, Clone)]
pub struct SeqNumbers {
    pub next_inbound: u64,
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

    /// Increments the expected next inbound seq. number.
    pub fn incr_inbound(&mut self) {
        self.next_inbound += 1;
    }

    /// Increments the expected next outbound seq. number.
    pub fn incr_outbound(&mut self) {
        self.next_outbound += 1;
    }

    /// Compares the given `inbound` seq. number with the expected next inbound
    /// value.
    pub fn validate_inbound(&self, inbound: u64) -> Result<(), SeqNumberError> {
        match inbound.cmp(&self.next_inbound) {
            Ordering::Equal => Ok(()),
            Ordering::Less => Err(SeqNumberError::TooLow),
            Ordering::Greater => Err(SeqNumberError::TooHigh),
        }
    }
}

impl Default for SeqNumbers {
    /// Returns the default [`SeqNumbers`] at the start of a new FIX
    /// session, i.e. both 1.
    fn default() -> Self {
        Self {
            next_inbound: 1,
            next_outbound: 1,
        }
    }
}

/// The error type for [`SeqNumbers::validate_inbound`].
#[derive(Debug, Clone)]
pub enum SeqNumberError {
    /// The given sequence number was higher than expected; some messages where
    /// likely lost.
    TooHigh,
    /// The given sequence number was lower than expected; this is a protocol
    /// violation.
    TooLow,
}
