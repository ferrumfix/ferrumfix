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

/// A standard-compliant FIX session processor (also known as FIX engine). It has
/// support for all session profiles.
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

enum SequenceResetMode {
    GapFill,
    Reset,
}

pub struct ProcessorBuilder<R: io::Read, W: io::Write> {
    profile: SessionProfile,
    sender: String,
    target: String,
    hearbeat: Duration,
    input: R,
    output: W,
    seq_reset_mode: SequenceResetMode,
}

impl<R: io::Read, W: io::Write> ProcessorBuilder<R, W> {

    pub fn new(acceptor: String, initiator: String, input: R, output: W) -> Self {
        ProcessorBuilder {
            profile: SessionProfile::Fix4,
            sender: initiator,
            target: acceptor,
            hearbeat: Duration::from_secs(30),
            input,
            output,
            seq_reset_mode: SequenceResetMode::GapFill,
        }
    }

    pub fn enable_fixt11(mut self) -> Self {
        self.profile = SessionProfile::Fixt;
        self
    }

    pub fn enable_fix4(mut self) -> Self {
        self.profile = SessionProfile::Fix4;
        self
    }

    pub fn enable_fix42(mut self) -> Self {
        self.profile = SessionProfile::Fix42;
        self
    }

    pub fn enable_lightweight_fixt(mut self) -> Self {
        self.profile = SessionProfile::LightweightFixt;
        self
    }

    pub fn with_heartbeat(mut self, heartbeat: Duration) -> Self {
        self.hearbeat = heartbeat;
        self
    }

    pub fn enable_gap_fill_mode(mut self) -> Self {
        self.seq_reset_mode = SequenceResetMode::GapFill;
        self
    }

    pub fn enable_reset_mode(mut self) -> Self {
        self.seq_reset_mode = SequenceResetMode::Reset;
        self
    }

    pub fn build(self) -> Processor<R, W> {
        Processor {
            profile: self.profile,
            sender: self.sender,
            target: self.target,
            next_inbound: 1,
            next_outbound: 1,
            heartbeat: self.hearbeat,
            last_receipt: Instant::now(),
            input: self.input,
            output: self.output,
        }
    }
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

    fn heartbeat(&mut self, now: Instant) -> Result<()> {
        let interval = now.duration_since(self.last_receipt);
        self.last_receipt = now;
        if interval > self.heartbeat {
            Err(Error::Generic)
        } else {
            Ok(())
        }
    }

    fn process_incoming(&mut self, message: ir::Message) -> Result<()> {
        self.heartbeat(Instant::now());
        if let ir::FixFieldValue::String(msg_type) = message.fields.get(&35).unwrap() {
            match msg_type.as_str() {
                // `Heartbeat(35=0)`.
                "0" => (),
                // `Logon(35=A)`.
                "A" => (),
                // `TestRequest(35=1)`.
                "1" => {
                    let mut response = ir::Message::new();
                    let id = message.fields.get(&112).unwrap();
                    response.fields.insert(112, id.clone());
                    self.send(response)?;
                },
                // `ResendRequest(35=2)`.
                "2" => {
                    let mut response = ir::Message::new();
                    let seq_from = message.fields.get(&7).unwrap();
                    let seq_to = message.fields.get(&16).unwrap();
                    let range = seq_from ..= seq_to;
                    self.send(response)?;
                },
                // `Reject(35=3)`.
                "3" => {
                    let mut response = ir::Message::new();
                    response.fields.insert(45, ir::FixFieldValue::Int(self.next_inbound as i64));
                    self.send(response)?;
                },
                // `SequenceReset(35=4)`.
                "4" => (),
                // `Logout(35=5)`.
                "5" => (),
                // Business-related message.
                _ => (),
            }
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