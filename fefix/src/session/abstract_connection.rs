use crate::{session::{errs, Environment, ResendRequestRange, SeqNumberError, SeqNumbers}, tagvalue::FieldSetter};
use crate::tagvalue::FixFieldValue;
use crate::{tags, FixFieldAccess, FixMessage};
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Event {
    TransportError,
    MissedHeartbeat,
    HeartbeatIsDue,
    Inbound(FixMessage),
    Outbound(FixMessage),
    Garbled,
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(enum_as_inner::EnumAsInner))]
pub enum Response {
    TerminateTransport,
    Inbound(FixMessage),
    Outbound(FixMessage),
    Resend {
        range: ResendRequestRange,
    },
    /// The FIX session processor should log each encountered garbled message to
    /// assist in problem detection and diagnosis.
    LogGarbled,
}

#[derive(Debug)]
pub struct Responses<'a> {
    connection: &'a mut AbstractConnection,
}

impl<'a> Iterator for Responses<'a> {
    type Item = Response;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.connection.queue.pop();
        println!("{:?}", a);
        a
    }
}

/// A FIX connection message processor.
#[derive(Debug, Clone)]
pub struct AbstractConnection<O> where O: FieldSetter {
    environment: Environment,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
    sender_comp_id: String,
    target_comp_id: String,
    queue: Vec<Response>,
}

#[derive(Debug, Clone)]
pub struct AbstractConnectionBuilder {
    pub environment: Environment,
    pub heartbeat: Duration,
    pub seq_numbers: SeqNumbers,
    pub sender_comp_id: String,
    pub target_comp_id: String,
}

impl AbstractConnectionBuilder {
    pub fn build(self) -> AbstractConnection<FieldSetter<Error = ()>> {
        AbstractConnection {
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
impl<O> AbstractConnection<O> where O: FieldSetter {
    fn generate_error_seqnum_too_low(&mut self) -> FixMessage {
        let error_message = errs::msg_seq_num(self.seq_numbers().next_inbound());
        let mut response = FixMessage::new();
        response.add_str(tags::MSG_TYPE, "5");
        response.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        response.add_i64(
            tags::TARGET_COMP_ID,
            self.seq_numbers().next_outbound() as i64,
        );
        response.add_str(tags::TEXT, error_message);
        add_time_to_msg(response)
    }

    /// Places a `response` to the outbound queue.
    fn enqueue(&mut self, response: Response) {
        self.queue.push(response);
    }

    fn seq_numbers(&self) -> SeqNumbers {
        self.seq_numbers
    }

    fn seq_numbers_mut(&mut self) -> &mut SeqNumbers {
        &mut self.seq_numbers
    }

    fn sending_time(&self) -> i128 {
        unimplemented!()
    }

    pub fn on_missed_heartbeat(&mut self) -> Responses {
        self.enqueue(Response::TerminateTransport);
        self.queue()
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
    fn on_garbled_message(&mut self) {
        self.queue.push(Response::LogGarbled);
    }

    pub fn queue(&mut self) -> Responses {
        Responses { connection: self }
    }

    #[must_use]
    pub fn on_inbound_message(&mut self, msg: FixMessage) -> Responses {
        let seq_numbers = self.seq_numbers();
        let env = self.environment();
        // Check `TestMessageIndicator(464)`.
        match (env, msg.f_test_indicator()) {
            (Environment::ProductionDisallowTest, Some(true)) => {
                self.on_message_in_wrong_env(msg);
                return self.queue();
            }
            _ => (),
        };
        // Compare seq. numbers.
        let seqnum_state = msg
            .f_seq_num()
            .map(|seqnum| seq_numbers.validate_inbound(seqnum))
            .unwrap_or(Err(SeqNumberError::NoSeqNum));
        // Compare the incoming seq. number to the one we expected and act
        // accordingly.
        match seqnum_state {
            Ok(()) => {}
            // See §4.5.3.
            Err(SeqNumberError::NoSeqNum) => {
                self.on_message_without_seq_num(msg);
                return self.queue();
            }
            // Refer to specs. §4.8 for more information.
            Err(SeqNumberError::Recover) => {
                self.on_message_with_high_seqnum(msg);
                return self.queue();
            }
            Err(SeqNumberError::TooLow) => {
                self.on_message_with_low_seqnum(msg);
                return self.queue();
            }
        };
        if self.sending_time_is_inaccurate(&msg) {
            self.on_message_with_inaccurate_sending_time();
            return self.queue();
        }
        // Detect Logon <A>.
        if let Some("A") = msg.f_msg_type() {
            self.on_logon(msg);
        } else {
            self.on_application_message(msg);
        }
        self.queue()
    }

    fn on_message_with_inaccurate_sending_time(&mut self) {
        let mut msg = FixMessage::new();
        msg.add_str(tags::MSG_TYPE, "3");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_i64(tags::SESSION_REJECT_REASON, 10);
        msg = add_time_to_msg(msg);
        self.enqueue(Response::Outbound(msg));
    }

    fn sending_time_is_inaccurate(&self, _msg: &FixMessage) -> bool {
        false
        //let sending_time = msg.field_utctimestamp(tags::SENDING_TIME);
        //if let Some(sending_time) = sending_time {
        //    false
        //} else {
        //    false
        //}
    }

    #[must_use]
    pub fn on_heartbeat_is_due(&mut self) -> Responses {
        let mut msg = FixMessage::new();
        msg.add_str(tags::MSG_TYPE, "0");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg = add_time_to_msg(msg);
        self.enqueue(Response::Outbound(msg));
        self.queue()
    }

    fn on_message_in_wrong_env(&mut self, _message: FixMessage) {
        let mut msg = FixMessage::new();
        msg.add_str(tags::MSG_TYPE, "5");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_str(tags::TEXT, errs::production_env());
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_message_without_seq_num(&mut self, _message: FixMessage) {
        let mut msg = FixMessage::new();
        msg.add_str(tags::MSG_TYPE, "5");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_i64(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound() as i64);
        msg.add_str(
            tags::TEXT,
            errs::missing_field("MsgSeqNum", tags::MSG_SEQ_NUM),
        );
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_message_with_low_seqnum(&mut self, _message: FixMessage) {
        let mut msg = FixMessage::new();
        msg.add_str(tags::MSG_TYPE, "5");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_i64(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound() as i64);
        msg.add_str(
            tags::TEXT,
            errs::msg_seq_num(self.seq_numbers().next_inbound()),
        );
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_message_with_high_seqnum(&mut self, message: FixMessage) {
        let mut msg = FixMessage::new();
        // Standard header.
        msg.add_str(tags::MSG_TYPE, "2");
        msg.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
        msg.add_str(tags::TARGET_COMP_ID, self.target_comp_id());
        msg.add_i64(tags::BEGIN_SEQ_NO, self.seq_numbers().next_inbound() as i64);
        msg.add_i64(tags::END_SEQ_NO, message.f_seq_num().unwrap() as i64);
        self.seq_numbers_mut().incr_outbound();
        self.enqueue(Response::Outbound(add_time_to_msg(msg)));
    }

    fn on_logon(&mut self, _message: FixMessage) {
        let mut response = FixMessage::new();
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

    fn on_resend_request(&mut self, message: FixMessage) {
        let start_seq_no = message.field_i64(tags::BEGIN_SEQ_NO).unwrap();
        let end_seq_no = message.field_i64(tags::END_SEQ_NO).map(|x| x as usize);
        self.enqueue(Response::Resend {
            range: ResendRequestRange::new(start_seq_no as usize, end_seq_no),
        });
    }

    #[must_use]
    pub fn on_transport_error(&mut self) -> Responses {
        self.enqueue(Response::TerminateTransport);
        self.queue()
    }

    fn on_application_message(&mut self, message: FixMessage) {
        self.enqueue(Response::Inbound(message));
    }
}

pub fn add_time_to_msg(mut msg: FixMessage) -> FixMessage {
    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
    let time = chrono::Utc::now();
    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
    msg.add_str(tags::SENDING_TIME, timestamp.to_string());
    msg
}

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    fn conn() -> AbstractConnection {
        let builder = AbstractConnectionBuilder {
            environment: Environment::ProductionDisallowTest,
            heartbeat: Duration::from_secs(30),
            seq_numbers: SeqNumbers::default(),
            sender_comp_id: "SENDER".to_string(),
            target_comp_id: "TARGET".to_string(),
        };
        builder.build()
    }

    #[test]
    fn on_heartbeat_is_due() {
        let conn = &mut conn();
        let responses = &mut conn.on_heartbeat_is_due();
        let next = responses.next().unwrap();
        let msg = next.as_outbound().unwrap();
        assert_eq!(msg.field_str(tags::MSG_TYPE), Some("0"));
        assert_eq!(msg.field_str(tags::SENDER_COMP_ID), Some("SENDER"));
        assert_eq!(msg.field_str(tags::TARGET_COMP_ID), Some("TARGET"));
        assert_eq!(msg.field_bool(tags::POSS_DUP_FLAG), None);
        assert_eq!(msg.field_i64(tags::TEST_REQ_ID), None);
        assert!(responses.next().is_none());
    }

    #[test]
    fn terminate_transport_when_error() {
        let conn = &mut conn();
        let responses = &mut conn.on_transport_error();
        let next = responses.next().unwrap();
        assert!(next.as_terminate_transport().is_some());
    }

    #[test]
    fn inaccurate_sending_time() {
        let conn = &mut conn();
        let mut msg = FixMessage::new();
        msg.add_str(tags::MSG_TYPE, "BE");
        msg.add_str(tags::SENDER_COMP_ID, "SENDER");
        msg.add_str(tags::TARGET_COMP_ID, "TARGET");
        msg.add_i64(tags::MSG_SEQ_NUM, 1);
        msg.add_str(
            tags::USER_REQUEST_ID,
            "47b6f4a6-993d-4430-b68f-d9b680a1a772",
        );
        msg.add_i64(tags::USER_REQUEST_TYPE, 1);
        msg.add_str(tags::USERNAME, "john-doe");
        let mut responses = conn.on_inbound_message(msg);
        let next = responses.next().unwrap();
        let msg = next.as_outbound().unwrap();
        assert_eq!(msg.field_str(tags::MSG_TYPE), Some("3"));
        assert_eq!(msg.field_str(tags::SENDER_COMP_ID), Some("SENDER"));
        assert_eq!(msg.field_str(tags::TARGET_COMP_ID), Some("TARGET"));
        assert_eq!(msg.field_bool(tags::POSS_DUP_FLAG), None);
        assert_eq!(msg.field_i64(tags::TEST_REQ_ID), None);
        assert_eq!(msg.field_i64(tags::SESSION_REJECT_REASON), Some(10));
        assert_eq!(msg.field_i64(tags::REF_SEQ_NUM), Some(10));
        assert!(responses.next().is_none());
    }
}
