/// The `MsgSeqNum` range in a `ResendRequest` message.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResendRequestRange {
    start: usize,
    end: Option<usize>,
}

impl ResendRequestRange {
    /// Creates a new resend-request range.
    pub fn new(start: usize, end: Option<usize>) -> Self {
        Self { start, end }
    }
}
