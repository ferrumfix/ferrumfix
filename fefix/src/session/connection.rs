use crate::session::{errs, Environment, ResendRequestRange, SeqNumberError, SeqNumbers};
use crate::tags;
use crate::tagvalue::{FixFieldValue, Message};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Event {
    TransportError,
    MissedHeartbeat,
    HeartbeatIsDue,
    Inbound(Message),
    Outbound(Message),
    Garbled,
}

#[derive(Debug, Clone)]
pub enum Response {
    TerminateTransport,
    Inbound(Message),
    Outbound(Message),
    Resend {
        range: ResendRequestRange,
    },
    /// The FIX session processor should log each encountered garbled message to
    /// assist in problem detection and diagnosis.
    LogGarbled,
}

#[derive(Debug)]
pub struct Responses<'a> {
    connection: &'a mut FixConnection,
}

impl<'a> Iterator for Responses<'a> {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        self.connection.queue.pop()
    }
}

/// A FIX connection message processor.
#[derive(Debug, Clone)]
pub struct FixConnection {
    environment: Environment,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
    sender_comp_id: String,
    target_comp_id: String,
    queue: Vec<Response>,
}

#[derive(Debug, Clone)]
pub struct FixConnectionBuilder {
    pub environment: Environment,
    pub heartbeat: Duration,
    pub seq_numbers: SeqNumbers,
    pub sender_comp_id: String,
    pub target_comp_id: String,
}

impl FixConnectionBuilder {
    pub fn build(self) -> FixConnection {
        FixConnection {
            environment: self.environment,
            heartbeat: self.heartbeat,
            seq_numbers: self.seq_numbers,
            sender_comp_id: self.sender_comp_id,
            target_comp_id: self.target_comp_id,
            queue: Vec::new(),
        }
    }
}

#[allow(dead_code)]
impl FixConnection {
    fn generate_error_seqnum_too_low(&mut self) -> Message {
        let error_message = errs::msg_seq_num(self.seq_numbers().next_inbound());
        let mut response = Message::new();
        response.add_str(tags::MSG_TYPE, "5");
        response.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        response.add_int(
            tags::TARGET_COMP_ID,
            self.seq_numbers().next_outbound() as i64,
        );
        response.add_str(tags::TEXT, error_message);
        add_time_to_msg(response)
    }

    /// Places a `response` to the outbound queue.
    fn enqueue(&mut self, _response: Response) {}

    fn seq_numbers(&self) -> SeqNumbers {
        self.seq_numbers
    }

    fn seq_numbers_mut(&mut self) -> &mut SeqNumbers {
        &mut self.seq_numbers
    }

    fn sending_time(&self) -> i128 {
        unimplemented!()
    }

    fn on_missed_heartbeat(&mut self) {
        self.enqueue(Response::TerminateTransport);
    }

    fn on_inbound_message(&mut self, _message: Message) {
        self.seq_numbers_mut().next_outbound();
    }

    fn environment(&self) -> Environment {
        self.environment
    }

    fn sender_comp_id(&self) -> &str {
        self.sender_comp_id.as_str()
    }

    fn target_comp_id(&self) -> &str {
        self.target_comp_id.as_str()
    }

    #[must_use]
    pub fn on_event(&mut self, event: Event) -> Responses {
        match event {
            Event::Garbled => self.on_garbled_message(),
            Event::Inbound(message) => self.on_inbound_message(message),
            Event::Outbound(message) => self.on_application_message(message),
            Event::TransportError => self.on_transport_error(),
            Event::MissedHeartbeat => self.on_missed_heartbeat(),
            Event::HeartbeatIsDue => self.on_heartbeat_is_due(),
        }
        Responses { connection: self }
    }

    fn on_garbled_message(&mut self) {
        self.queue.push(Response::LogGarbled);
    }

    fn on_message(&mut self, msg: Message) {
        let seq_numbers = self.seq_numbers();
        let env = self.environment();
        // Check `TestMessageIndicator(464)`.
        match (env, msg.test_indicator()) {
            (Environment::ProductionDisallowTest, Some(true)) => {
                self.on_message_in_wrong_env(msg);
                return;
            }
            _ => (),
        };
        // Compare seq. numbers.
        let seqnum_state = msg
            .seq_num()
            .map(|seqnum| seq_numbers.validate_inbound(seqnum))
            .unwrap_or(Err(SeqNumberError::NoSeqNum));
        // Compare the incoming seq. number to the one we expected and act
        // accordingly.
        match seqnum_state {
            Ok(()) => {}
            // See §4.5.3.
            Err(SeqNumberError::NoSeqNum) => {
                self.on_message_without_seq_num(msg);
                return;
            }
            // Refer to specs. §4.8 for more information.
            Err(SeqNumberError::Recover) => {
                self.on_message_with_high_seqnum(msg);
                return;
            }
            Err(SeqNumberError::TooLow) => {
                self.on_message_with_low_seqnum(msg);
                return;
            }
        };
        // Detect Logon <A>.
        if let Some("A") = msg.msg_type() {
            self.on_logon(msg);
        } else if msg.msg_type() != Some("A") {
            self.enqueue(Response::TerminateTransport);
            return;
        } else {
            self.on_application_message(msg);
        }
    }

    fn on_heartbeat_is_due(&mut self) {
        let mut msg = Message::new();
        msg.add_str(tags::MSG_TYPE, "0");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg = add_time_to_msg(msg);
        self.enqueue(Response::Outbound(msg))
    }

    fn on_message_in_wrong_env(&mut self, _message: Message) {
        let mut msg = Message::new();
        msg.add_str(tags::MSG_TYPE, "5");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_str(tags::TEXT, errs::production_env());
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_message_without_seq_num(&mut self, _message: Message) {
        let mut msg = Message::new();
        msg.add_str(tags::MSG_TYPE, "5");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_int(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound() as i64);
        msg.add_str(
            tags::TEXT,
            errs::missing_field("MsgSeqNum", tags::MSG_SEQ_NUM),
        );
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_message_with_low_seqnum(&mut self, _message: Message) {
        let mut msg = Message::new();
        msg.add_str(tags::MSG_TYPE, "5");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_int(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound() as i64);
        msg.add_str(
            tags::TEXT,
            errs::msg_seq_num(self.seq_numbers().next_inbound()),
        );
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_message_with_high_seqnum(&mut self, message: Message) {
        let mut msg = Message::new();
        // Standard header.
        msg.add_str(tags::MSG_TYPE, "2");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.sender_comp_id());
        msg.add_int(tags::BEGIN_SEQ_NO, self.seq_numbers().next_inbound() as i64);
        msg.add_int(tags::END_SEQ_NO, message.seq_num().unwrap() as i64);
        self.seq_numbers_mut().incr_outbound();
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_logon(&mut self, _message: Message) {
        let mut response = Message::new();
        // TODO: add other details to response message.
        response
            .add_field(tags::MSG_TYPE, FixFieldValue::string(b"A").unwrap())
            .unwrap();
        response
            .add_field(
                tags::SENDER_COMP_ID,
                FixFieldValue::string(self.sender_comp_id().as_bytes()).unwrap(),
            )
            .unwrap();
        self.seq_numbers_mut().incr_outbound();
        self.enqueue(Response::Outbound(add_time_to_msg(response)));
    }

    fn on_resend_request(&mut self, message: Message) {
        let start_seq_no = message
            .field(tags::BEGIN_SEQ_NO)
            .and_then(|f| f.as_int().map(|x| x as usize))
            .unwrap();
        let end_seq_no = message
            .field(tags::END_SEQ_NO)
            .and_then(|f| f.as_int().map(|x| x as usize));
        self.enqueue(Response::Resend {
            range: ResendRequestRange::new(start_seq_no, end_seq_no),
        });
    }

    fn on_transport_error(&mut self) {
        self.enqueue(Response::TerminateTransport);
    }

    fn on_application_message(&mut self, message: Message) {
        self.enqueue(Response::Inbound(message));
    }
}

pub fn add_time_to_msg(mut msg: Message) -> Message {
    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
    let time = chrono::Utc::now();
    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
    msg.add_str(tags::SENDING_TIME, timestamp.to_string());
    msg
}
