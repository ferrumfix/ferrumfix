use crate::ir;
use crate::presentation::Encoding;
use futures::Stream;
use futures::StreamExt;
use std::io;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

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
    Generic,
    Disconnect,
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
///
/// ```
/// let builder = ProcessorBuilder::new("FUNDMANAGER", "BROKER", input, output);
/// let fix_engine = builder.build();
/// ```
pub struct Processor<R: io::Read, W: io::Write> {
    profile: SessionProfile,
    sender: String,
    target: String,
    next_inbound: u64,
    next_outbound: u64,
    heartbeat: Duration,
    last_receipt: Instant,
    last_test_id: String,
    sent: Vec<ir::Message>,
    queue_sender: Sender<ir::Message>,
    queue_receiver: Receiver<ir::Message>,
    notify_sender: Sender<Event>,
    notify_receiver: Receiver<Event>,
    input: R,
    output: W,
}

enum SequenceResetMode {
    GapFill,
    Reset,
}

#[derive(Debug)]
pub enum Event {
    MessageReceived(ir::Message),
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
    pub fn new<S1: ToString, S2: ToString>(
        acceptor: S1,
        initiator: S2,
        input: R,
        output: W,
    ) -> Self {
        ProcessorBuilder {
            profile: SessionProfile::Fix4,
            sender: initiator.to_string(),
            target: acceptor.to_string(),
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

    /// Generate a FIX messaging processor (a.k.a. FIX engine) from the current
    /// configuration.
    pub fn build(self) -> Processor<R, W> {
        let queue = channel();
        let notifications = channel();
        Processor {
            profile: self.profile,
            sender: self.sender,
            target: self.target,
            next_inbound: 1,
            next_outbound: 1,
            heartbeat: self.hearbeat,
            last_receipt: Instant::now(),
            last_test_id: String::new(),
            sent: vec![],
            queue_sender: queue.0,
            queue_receiver: queue.1,
            notify_sender: notifications.0,
            notify_receiver: notifications.1,
            input: self.input,
            output: self.output,
        }
    }
}

impl<R: io::Read, W: io::Write> Processor<R, W> {
    fn new(logon_message: ir::Message, input: R, output: W) -> Self {
        let queue = channel();
        let notifications = channel();
        Processor {
            profile: SessionProfile::Fix4,
            sender: String::new(),
            target: String::new(),
            next_inbound: 1,
            next_outbound: 1,
            heartbeat: Duration::from_secs(30),
            queue_sender: queue.0,
            queue_receiver: queue.1,
            notify_sender: notifications.0,
            notify_receiver: notifications.1,
            last_receipt: Instant::now(),
            last_test_id: String::new(),
            sent: vec![],
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

    /// Incoming data feed to the event loop.
    pub async fn event_loop(
        &mut self,
        mut stream: impl Stream<Item = ir::Message> + Unpin,
    ) -> impl Stream<Item = Result<ir::Message>> {
        while let Some(_message) = stream.next().await {}
        futures::stream::empty()
    }

    pub fn channel(&self) -> (&Sender<ir::Message>, &Receiver<Event>) {
        (&self.queue_sender, &self.notify_receiver)
    }

    fn process_incoming(
        &mut self,
        message: ir::Message,
    ) -> Result<impl Stream<Item = ir::Message>> {
        self.heartbeat(Instant::now())?;
        if let ir::FixFieldValue::String(msg_type) = message.fields.get(&35).unwrap() {
            match msg_type.as_str() {
                // `Heartbeat(35=0)`.
                "0" => {
                    let test_id = message.fields.get(&112);
                    match test_id {
                        Some(ir::FixFieldValue::String(s)) => {
                            self.last_test_id = s.to_string();
                        }
                        Some(_) => (),
                        None => (),
                    };
                }
                // `Logon(35=A)`.
                "A" => {
                    let mut response = ir::Message::new();
                    // Populate response.
                    self.send(response)?;
                }
                // `TestRequest(35=1)`.
                "1" => {
                    let mut response = ir::Message::new();
                    let id = message.fields.get(&112).unwrap();
                    response
                        .fields
                        .insert(35, ir::FixFieldValue::String("0".to_string()));
                    response.fields.insert(112, id.clone());
                    self.send(response)?;
                }
                // `ResendRequest(35=2)`.
                "2" => {
                    let mut response = ir::Message::new();
                    let seq_from = message.fields.get(&7).unwrap();
                    let seq_to = message.fields.get(&16).unwrap();
                    let range = seq_from..=seq_to;
                    self.send(response)?;
                }
                // `Reject(35=3)`.
                "3" => {
                    let mut response = ir::Message::new();
                    response
                        .fields
                        .insert(45, ir::FixFieldValue::Int(self.next_inbound as i64));
                    self.send(response)?;
                }
                // `SequenceReset(35=4)`.
                "4" => (),
                // `Logout(35=5)`.
                "5" => return Err(Error::Disconnect),
                // Business-related message.
                _ => (),
            }
        };
        // If `MsgSeqNum(34)` is missing, a `Logout(35=5)` message should be
        // sent, terminating the FIX connection with the Text(58) field
        // describing the missing field, as this likely indicates a serious
        // application error that is likely only circumvented by software
        // modification.
        match message.fields.get(&34) {
            Some(ir::FixFieldValue::Int(seq_number)) => {
                if *seq_number == self.next_inbound as i64 {
                } else {
                    self.request_resend(self.next_inbound);
                }
            }
            _ => self.interrupt_connection(),
        };
        Ok(futures::stream::empty())
    }

    fn send(&mut self, message: ir::Message) -> Result<()> {
        self.next_outbound += 1;
        self.notify_sender
            .send(Event::MessageReceived(message))
            .unwrap();
        Ok(())
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::presentation;

    const MSG: &'static [u8] = &[0];

    fn processor(input: &[u8]) -> Processor<&[u8], Vec<u8>> {
        let output = vec![];
        let builder = ProcessorBuilder::new("BROKER", "FUND-MANAGER", input, output);
        builder.build()
    }

    #[test]
    fn logon_acknowledgement() {
        let mut msg = ir::Message::new();
        msg.fields
            .insert(8, ir::FixFieldValue::String("FIX.4.4".to_string()));
        msg.fields.insert(9, ir::FixFieldValue::Int(42));
        msg.fields
            .insert(35, ir::FixFieldValue::String("A".to_string()));
        msg.fields.insert(108, ir::FixFieldValue::Int(30));
        msg.fields
            .insert(49, ir::FixFieldValue::String("BROKER".to_string()));
        msg.fields
            .insert(56, ir::FixFieldValue::String("FUND-MANAGER".to_string()));
        msg.fields.insert(34, ir::FixFieldValue::Int(1));
        msg.fields.insert(52, ir::FixFieldValue::Int(980846));
        let encoding = presentation::TagValue::new();
        let input_buffer = vec![];
        let mut processor = processor(&input_buffer[..]);
        processor.process_incoming(msg);
        let (sender, recv) = processor.channel();
        let response = recv.recv();
        println!("{:?}", response);
        //assert!(result.is_ok());
    }
}
