use crate::ir::Message;
use crate::ir;
use crate::presentation::Encoding;
use std::time::{Duration, Instant};
use std::io;

const HEARTBEAT_ERR: &str = "Invalid HeartBtInt(108), expected value {} seconds";
const MAX_MESSAGE_SIZE_ERR: &str = "MaxMessageSize(383) = {} < required message size {}";

pub trait FixConnection<T> {
    fn read(&mut self) -> Result<T>;
    fn write(&mut self, message: T) -> Result<()>;
}

pub enum Error {
    /// A message shall be considered garbled if any of the following occur as a
    /// result of encoding, decoding, or transmission errors:
    ///
    /// - `BeginString(8)` is not the first tag in a message or is not one of the
    /// defined FIX session profile identifiers.
    /// - `BodyLength(9)` is not the second tag in a message or does not contain
    /// the correct byte count.
    /// - `MsgType(35)` is not the third tag in a message.
    /// - `Checksum(10)` is not the last tag or contains an incorrect value.
    /// 
    /// The FIX session layer presumes that a garbled message is received due to
    /// a transmission error rather than a FIX session processor defect and
    /// should be recoverable from the peer.
    Garbled,
    Generic
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum SessionProfile {
    Fix42,
    Fix4,
    Fixt,
    LightweightFixt,
}

pub struct Processor<R: io::Read, W: io::Write> {
    profile: SessionProfile,
    sender: String,
    target: String,
    next_inbound: u64,
    next_outbound: u64,
    heartbeat: Duration,
    last_receipt: Instant,
    input: R,
    output: W,
}

impl<R: io::Read, W: io::Write> Processor<R, W> {
    fn new(logon_message: ir::Message, input: R, output: W) -> Self {
        Processor {
            profile: SessionProfile::Fix4,
            sender: String::new(),
            target: String::new(),
            next_inbound: 1,
            next_outbound: 1,
            heartbeat: Duration::from_secs(30),
            last_receipt: Instant::now(),
            input,
            output,
        }
    }

    fn process_incoming(&mut self, message: ir::Message) -> Result<()> {
        let now = Instant::now();
        let interval = now.duration_since(self.last_receipt);
        self.last_receipt = now;
        if interval > self.heartbeat {
            return Err(Error::Generic);
        }
        if let Some(ir::FixFieldValue::Int(1)) = message.fields.get(&35) {
        }
        // If `MsgSeqNum(34)` is missing, a `Logout(35=5)` message should be
        // sent, terminating the FIX connection with the Text(58) field
        // describing the missing field, as this likely indicates a serious
        // application error that is likely only circumvented by software
        // modification.
        match message.fields.get(&34) {
            Some(ir::FixFieldValue::Int(seq_number)) => {
                if *seq_number > self.next_inbound as i64 {

                } else {
                    self.request_resend(self.next_inbound);
                }
            },
            _ => self.interrupt_connection(),
        };
        Ok(())
    }

    fn send(&mut self, message: ir::Message) -> Result<()> {
        self.next_outbound += 1;
        Ok(())
    }

    fn message_is_garbled(&self, message: ir::Message) -> bool {
        todo!()
    }

    fn interrupt_connection(&mut self) {
        todo!()
    }

    fn reject_message(&mut self, msg: ir::Message) {
        self.next_inbound += 1;
    }

    fn request_resend(&mut self, _from: u64) {
        todo!();
    }
}