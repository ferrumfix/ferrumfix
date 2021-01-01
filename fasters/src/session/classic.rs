use crate::app::slr;
use boolinator::Boolinator;
use std::cmp::Ordering;
use std::ops::Range;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

const MAX_MESSAGE_SIZE_ERR: &str = "MaxMessageSize(383) = {} < required message size {}";

fn reasonable_amount_of_time(heartbeat: &Duration) -> Duration {
    Duration::from_secs_f32(heartbeat.as_secs_f32() / 5.0)
}

pub trait FixConnection<T> {
    fn read(&mut self) -> Result<T>;
    fn write(&mut self, message: T) -> Result<()>;
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SessionProfile {
    Fix42,
    Fix4,
    Fixt,
    LightweightFixt,
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Disconnected,
    Established,
}

struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Default for Channel<T> {
    fn default() -> Self {
        let channel = channel();
        Channel {
            sender: channel.0,
            receiver: channel.1,
        }
    }
}

/// Config by acceptor.
pub enum HeartbeatConfig {
    Predetermined(Duration),
    Settable(Range<Duration>),
    Custom,
}

impl HeartbeatConfig {
    pub fn validate(&self, proposal: &Duration) -> std::result::Result<(), String> {
        match self {
            HeartbeatConfig::Predetermined(expected) => (proposal == expected).ok_or(format!(
                "Invalid HeartBtInt(108), expected value {} seconds",
                expected.as_secs()
            )),
            HeartbeatConfig::Settable(range) => range.contains(proposal).ok_or(format!(
                "Invalid HeartBtInt(108), expected value between {} and {} seconds",
                range.start.as_secs(),
                range.end.as_secs() + 1,
            )),
            Custom => Ok(()),
        }
    }
}

pub trait Authenticator {
    fn auth(&mut self, message: &slr::Message) -> bool;
}

// TODO:
// - Maximum message size
// - Specifying application version
// - Specifying supported message types
// - Identification of application system and FIX session processor
// "MaxMessageSize(383) = InboundValue < required message size M"

struct Configuration {
    profile: SessionProfile,
    sender: String,
    target: String,
    app_encryption: bool,
    // There are three methods for determining the heartbeat interval:
    //
    // 1. Acceptor requires a specific heartbeat interval
    // 2. Acceptor requires initiator to specify a value within a heartbeat interval range
    // 3. Acceptor accepts the initiator specified heartbeat interval
    heartbeat: HeartbeatConfig,
    heartbeat_relaxed: Duration,
    delivery_threshold: Duration,
    channel_inbound: Channel<slr::Message>,
    channel_outbound: Channel<Event>,
}

/// A standard-compliant FIX session processor (also known as FIX engine). It has
/// support for all session profiles.
///
/// ```
/// let builder = ProcessorBuilder::new("FUNDMANAGER", "BROKER", input, output);
/// let fix_engine = builder.build();
/// ```
pub struct Processor {
    state: State,
    config: Configuration,
    expected_seqnum_inbound: u64,
    expected_seqnum_outbound: u64,
    last_receipt: Instant,
    last_test_id: String,
    queue_inbound: Vec<slr::Message>,
    history_outbound: Vec<slr::Message>,
}

enum SequenceResetMode {
    GapFill,
    Reset,
}

#[derive(Debug)]
pub enum Event {
    Message(slr::Message),
    Disconnected,
}

/// For `TestMessageIndicator(464)`.
enum Environment {
    Production,
    Testing,
}

pub struct ProcessorBuilder {
    profile: SessionProfile,
    sender: String,
    target: String,
    hearbeat: HeartbeatConfig,
    seq_reset_mode: SequenceResetMode,
}

impl ProcessorBuilder {
    pub fn new<S1: ToString, S2: ToString>(acceptor: S1, initiator: S2) -> Self {
        ProcessorBuilder {
            profile: SessionProfile::Fix4,
            sender: initiator.to_string(),
            target: acceptor.to_string(),
            hearbeat: HeartbeatConfig::Predetermined(Duration::from_secs(30)),
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
        self.hearbeat = HeartbeatConfig::Predetermined(heartbeat);
        self
    }

    pub fn with_heartbeat_range(mut self, range: Range<Duration>) -> Self {
        self.hearbeat = HeartbeatConfig::Settable(range);
        self
    }

    pub fn with_any_heartbeat(mut self) -> Self {
        self.hearbeat = HeartbeatConfig::Custom;
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
    pub fn build(self) -> Processor {
        Processor {
            config: Configuration {
                profile: self.profile,
                sender: self.sender,
                target: self.target,
                app_encryption: false,
                heartbeat: self.hearbeat,
                heartbeat_relaxed: Duration::from_secs(5), //self.hearbeat + reasonable_amount_of_time(&self.hearbeat),
                delivery_threshold: Duration::from_secs(5),
                channel_inbound: Channel::default(),
                channel_outbound: Channel::default(),
            },
            state: State::Disconnected,
            expected_seqnum_inbound: 1,
            expected_seqnum_outbound: 1,
            last_receipt: Instant::now(),
            last_test_id: String::new(),
            queue_inbound: vec![],
            history_outbound: vec![],
        }
    }
}

impl Processor {
    fn heartbeat(&mut self, now: Instant) -> Result<()> {
        let interval = now.duration_since(self.last_receipt);
        self.last_receipt = now;
        if interval > Duration::from_secs(30) {
            // self.config.heartbeat {
            Err(Error::Generic)
        } else {
            Ok(())
        }
    }

    pub fn identifier(&self) -> (SessionProfile, &str, &str) {
        (
            self.config.profile,
            self.config.sender.as_str(),
            self.config.target.as_str(),
        )
    }

    pub fn channel(&self) -> (&Sender<slr::Message>, &Receiver<Event>) {
        (
            &self.config.channel_inbound.sender,
            &self.config.channel_outbound.receiver,
        )
    }

    fn process_incoming_disconnected(&mut self, message: slr::Message) -> Result<()> {
        let id_is_ok = self.identification_is_ok(&message);
        if !id_is_ok {
            self.config
                .channel_outbound
                .sender
                .send(Event::Disconnected)
                .unwrap();
            return Ok(());
        }
        self.heartbeat(Instant::now())?;
        // TODO: Authenticator check.
        let proposed_heartbeat = message.fields.get(&108).unwrap();
        if let slr::FixFieldValue::Int(hb) = proposed_heartbeat {
            let heartbeat_result = self
                .config
                .heartbeat
                .validate(&Duration::from_secs(*hb as u64));
            if let Err(err_string) = heartbeat_result {
                let mut logout = slr::Message::new();
                logout.add_field(35, slr::FixFieldValue::String("5".to_string()));
                logout.add_field(
                    58,
                    slr::FixFieldValue::String("invalid Heartbeat(108)".to_string()),
                );
                self.config
                    .channel_outbound
                    .sender
                    .send(Event::Message(logout))
                    .unwrap();
                self.config
                    .channel_outbound
                    .sender
                    .send(Event::Disconnected)
                    .unwrap();
            }
        }
        Ok(())
    }

    fn process_incoming_established(&mut self, message: slr::Message) -> Result<()> {
        // If `MsgSeqNum(34)` is missing, a `Logout(35=5)` message should be
        // sent, terminating the FIX connection with the Text(58) field
        // describing the missing field, as this likely indicates a serious
        // application error that is likely only circumvented by software
        // modification.
        if let Some(slr::FixFieldValue::Int(seq_number)) = message.fields.get(&34) {
            match seq_number.cmp(&(self.expected_seqnum_inbound as i64)) {
                Ordering::Equal => {}
                Ordering::Less => {
                    let mut logout = slr::Message::new();
                    // Standard header.
                    logout.add_field(35, slr::FixFieldValue::String("5".to_string()));
                    // Body.
                    let err = format!(
                        "Expected '{}' MsgSeqNum, received '{}'.",
                        self.expected_seqnum_inbound, seq_number
                    );
                    logout.add_field(58, slr::FixFieldValue::String(err));
                    self.send(logout)?;
                }
                Ordering::Greater => {
                    // Start message recovery.
                    let range = SeqNumRange {
                        start: self.expected_seqnum_inbound.into(),
                        end: Some(*seq_number as u64),
                    };
                    self.request_resend(range);
                }
            }
        } else {
            let mut response = slr::Message::new();
            response.add_field(35, slr::FixFieldValue::String("2".to_string()));
        };
        if let slr::FixFieldValue::String(msg_type) = message.fields.get(&35).unwrap() {
            match msg_type.as_str() {
                // `Heartbeat(35=0)`.
                "0" => {
                    let test_id = message.fields.get(&112);
                    match test_id {
                        Some(slr::FixFieldValue::String(s)) => {
                            self.last_test_id = s.to_string();
                        }
                        Some(_) => (),
                        None => (),
                    };
                }
                // `Logon(35=A)`.
                "A" => {
                    let seq_num = match message.fields.get(&34).unwrap() {
                        slr::FixFieldValue::Int(x) => x,
                        _ => Err(Error::Generic)?,
                    };
                    let mut response = slr::Message::new();
                    if *seq_num == self.expected_seqnum_inbound as i64 {
                        response
                            .fields
                            .insert(35, slr::FixFieldValue::String("A".to_string()));
                        self.send(response)?;
                    } else if *seq_num > self.expected_seqnum_inbound as i64 {
                        response
                            .fields
                            .insert(35, slr::FixFieldValue::String("2".to_string()));
                        self.send(response)?;
                    } else {
                        response
                            .fields
                            .insert(35, slr::FixFieldValue::String("5".to_string()));
                        self.send(response)?;
                        return Err(Error::Disconnect);
                    }
                }
                // `TestRequest(35=1)`.
                "1" => {
                    let mut response = slr::Message::new();
                    let id = message.fields.get(&112).unwrap();
                    response
                        .fields
                        .insert(35, slr::FixFieldValue::String("0".to_string()));
                    response.add_field(112, id.clone());
                    self.send(response)?;
                }
                // `ResendRequest(35=2)`.
                "2" => {
                    let mut response = slr::Message::new();
                    let seq_from = message.fields.get(&7).unwrap();
                    let seq_to = message.fields.get(&16).unwrap();
                    let range = seq_from..=seq_to;
                    self.send(response)?;
                }
                // `Reject(35=3)`.
                "3" => {
                    let mut response = slr::Message::new();
                    response.add_field(
                        45,
                        slr::FixFieldValue::Int(self.expected_seqnum_inbound as i64),
                    );
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
        Ok(())
    }

    fn process_incoming(&mut self, message: slr::Message) -> Result<()> {
        match self.state {
            State::Disconnected => self.process_incoming_disconnected(message),
            State::Established => self.process_incoming_established(message),
        }
    }

    fn send(&mut self, mut message: slr::Message) -> Result<()> {
        self.expected_seqnum_outbound += 1;
        message.add_field(49, slr::FixFieldValue::String(self.config.sender.clone()));
        message.add_field(56, slr::FixFieldValue::String(self.config.target.clone()));
        message.add_field(
            34,
            slr::FixFieldValue::Int(self.expected_seqnum_outbound as i64),
        );
        self.config
            .channel_outbound
            .sender
            .send(Event::Message(message))
            .unwrap();
        Ok(())
    }

    fn interrupt_connection(&mut self) {
        todo!()
    }

    fn reject_message(&mut self, msg: slr::Message) {
        self.expected_seqnum_inbound += 1;
    }

    fn request_resend(&mut self, range: SeqNumRange) {
        let mut message = slr::Message::new();
        message.add_field(35, slr::FixFieldValue::String("2".to_string()));
        message.add_field(7, slr::FixFieldValue::Int(range.start as i64));
        message.add_field(16, slr::FixFieldValue::Int(range.end.unwrap_or(0) as i64));
    }

    fn identification_is_ok(&self, msg: &slr::Message) -> bool {
        let begin_string = msg.fields.get(&35);
        let sender_comp_id = msg.fields.get(&49);
        let target_comp_id = msg.fields.get(&56);
        match begin_string {
            Some(slr::FixFieldValue::String(x)) => (),
            _ => return false,
        };
        match sender_comp_id {
            Some(slr::FixFieldValue::String(sender)) => {
                if *sender == self.config.sender {
                    return false;
                }
            }
            _ => return false,
        };
        match target_comp_id {
            Some(slr::FixFieldValue::String(target)) => {
                if *target == self.config.target {
                    return false;
                }
            }
            _ => return false,
        };
        true
    }

    fn sending_time_within_threshold(&self, msg: &slr::Message) -> bool {
        true
    }
}

struct SeqNumRange {
    start: u64,
    end: Option<u64>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::presentation;

    const MSG: &'static [u8] = &[0];

    fn processor(input: &[u8]) -> Processor {
        let builder = ProcessorBuilder::new("BROKER", "FUND-MANAGER");
        builder.build()
    }

    fn msg_add_identifier(msg: &mut slr::Message) {
        msg.add_field(8, slr::FixFieldValue::String("FIX.4.4".to_string()));
        msg.add_field(9, slr::FixFieldValue::Int(421337));
        msg.add_field(49, slr::FixFieldValue::String("BROKER".to_string()));
        msg.add_field(56, slr::FixFieldValue::String("FUND-MANAGER".to_string()));
    }

    #[test]
    fn testcase_1b_b() {
        let mut msg = slr::Message::new();
        msg_add_identifier(&mut msg);
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(1));
        msg.add_field(52, slr::FixFieldValue::Int(980846));
        let input_buffer = vec![];
        let mut processor = processor(&input_buffer[..]);
        processor.process_incoming(msg).unwrap();
        let (_, recv) = processor.channel();
        let mut event = recv.try_recv().unwrap();
        match event {
            Event::Message(response) => {
                assert_eq!(
                    *response.fields.get(&35).unwrap(),
                    slr::FixFieldValue::String("3".to_string())
                );
                assert!(response.fields.get(&112).is_none());
            }
            Event::Disconnected => panic!(),
        }
        event = recv.try_recv().unwrap();
        match event {
            Event::Message(response) => {
                assert_eq!(
                    *response.fields.get(&35).unwrap(),
                    slr::FixFieldValue::String("5".to_string())
                );
            }
            Event::Disconnected => panic!(),
        }
    }

    #[test]
    fn testcase_1b_c() {
        let mut msg = slr::Message::new();
        msg_add_identifier(&mut msg);
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(42));
        msg.add_field(52, slr::FixFieldValue::Int(980846));
        let input_buffer = vec![];
        let mut processor = processor(&input_buffer[..]);
        processor.process_incoming(msg).unwrap();
        let (_, recv) = processor.channel();
        let event = recv.try_recv().unwrap();
        match event {
            Event::Message(response) => {
                assert_eq!(
                    *response.fields.get(&35).unwrap(),
                    slr::FixFieldValue::String("2".to_string())
                );
            }
            Event::Disconnected => panic!(),
        }
    }

    #[test]
    fn testcase_1b_e() {
        let mut msg = slr::Message::new();
        msg_add_identifier(&mut msg);
        msg.add_field(35, slr::FixFieldValue::String("G".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        let input_buffer = vec![];
        let mut processor = processor(&input_buffer[..]);
        processor.process_incoming(msg).unwrap();
        let (_, recv) = processor.channel();
        let event = recv.try_recv().unwrap();
        match event {
            Event::Message(response) => {
                assert_eq!(
                    *response.fields.get(&35).unwrap(),
                    slr::FixFieldValue::String("2".to_string())
                );
            }
            Event::Disconnected => panic!(),
        }
    }

    #[test]
    fn testcase_1s_a() {
        let mut msg = slr::Message::new();
        msg_add_identifier(&mut msg);
        msg.add_field(35, slr::FixFieldValue::String("G".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        let input_buffer = vec![];
        let mut processor = processor(&input_buffer[..]);
        processor.process_incoming(msg).unwrap();
        let (_, recv) = processor.channel();
        let event = recv.try_recv().unwrap();
        match event {
            Event::Message(response) => {
                assert_eq!(
                    *response.fields.get(&35).unwrap(),
                    slr::FixFieldValue::String("2".to_string())
                );
            }
            Event::Disconnected => panic!(),
        }
    }

    #[test]
    fn testcase_2s_a() {
        let mut msg = slr::Message::new();
        msg_add_identifier(&mut msg);
        msg.add_field(35, slr::FixFieldValue::String("G".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        let input_buffer = vec![];
        let mut processor = processor(&input_buffer[..]);
        processor.process_incoming(msg).unwrap();
        let (_, recv) = processor.channel();
        let event = recv.try_recv().unwrap();
        match event {
            Event::Message(response) => {
                assert_eq!(
                    *response.fields.get(&35).unwrap(),
                    slr::FixFieldValue::String("2".to_string())
                );
            }
            Event::Disconnected => panic!(),
        }
    }
}
