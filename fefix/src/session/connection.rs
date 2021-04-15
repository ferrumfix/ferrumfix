//use crate::{DtfTimestamp, models::FixFieldValue, tagvalue::{RawEncoder, RawEncoderState}};
//use crate::{
//    session::{errs, Environment, ResendRequestRange, SeqNumberError, SeqNumbers},
//    tagvalue::Message,
//};
//use crate::{tags, FixMessage};
//use std::time::Duration;
//
//#[derive(Debug, Clone)]
//pub enum Event {
//    TransportError,
//    MissedHeartbeat,
//    HeartbeatIsDue,
//    Inbound(FixMessage),
//    Outbound(FixMessage),
//    Garbled,
//}
//
//#[derive(Debug, Clone)]
//#[cfg_attr(test, derive(enum_as_inner::EnumAsInner))]
//pub enum Response<'a> {
//    TerminateTransport,
//    Application(Message<'a>),
//    Session(&'a [u8]),
//    Inbound(FixMessage),
//    Outbound(FixMessage),
//    OutboundBytes(&'a [u8]),
//    Resend {
//        range: ResendRequestRange,
//    },
//    /// The FIX session processor should log each encountered garbled message to
//    /// assist in problem detection and diagnosis.
//    LogGarbled,
//}
//
//#[derive(Debug)]
//pub struct Responses<'a> {
//    connection: &'a mut FixConnection,
//}
//
//impl<'a> Iterator for Responses<'a> {
//    type Item = Response<'a>;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        None
//    }
//}
//
//#[derive(Debug, Clone)]
//pub struct FixConnectionBuilder {
//    pub begin_string: String,
//    pub environment: Environment,
//    pub heartbeat: Duration,
//    pub seq_numbers: SeqNumbers,
//    pub sender_comp_id: String,
//    pub target_comp_id: String,
//}
//
//impl FixConnectionBuilder {
//    pub fn build(self) -> FixConnection {
//        FixConnection {
//            begin_string: self.begin_string,
//            environment: self.environment,
//            heartbeat: self.heartbeat,
//            seq_numbers: self.seq_numbers,
//            sender_comp_id: self.sender_comp_id,
//            target_comp_id: self.target_comp_id,
//            encoder: RawEncoder::from_buffer(Vec::new()),
//            log: true,
//        }
//    }
//}
//
///// A FIX connection message processor.
//#[derive(Debug, Clone)]
//pub struct FixConnection {
//    begin_string: String,
//    environment: Environment,
//    heartbeat: Duration,
//    seq_numbers: SeqNumbers,
//    sender_comp_id: String,
//    target_comp_id: String,
//    encoder: RawEncoder,
//    log: bool,
//}
//
//#[allow(dead_code)]
//impl FixConnection {
//    fn generate_error_seqnum_too_low(&mut self) -> FixMessage {
//        let error_message = errs::msg_seq_num(self.seq_numbers().next_inbound());
//        let mut response = FixMessage::new();
//        response.add_str(tags::MSG_TYPE, "5");
//        response.add_str(tags::SENDER_COMP_ID, self.sender_comp_id());
//        response.add_i64(
//            tags::TARGET_COMP_ID,
//            self.seq_numbers().next_outbound() as i64,
//        );
//        response.add_str(tags::TEXT, error_message);
//        add_time_to_msg(response)
//    }
//
//    fn seq_numbers(&self) -> SeqNumbers {
//        self.seq_numbers
//    }
//
//    fn seq_numbers_mut(&mut self) -> &mut SeqNumbers {
//        &mut self.seq_numbers
//    }
//
//    fn sending_time(&self) -> i128 {
//        unimplemented!()
//    }
//
//    pub fn on_missed_heartbeat(&mut self) -> Responses {
//        self.queue()
//    }
//
//    fn environment(&self) -> Environment {
//        self.environment
//    }
//
//    fn sender_comp_id(&self) -> &str {
//        self.sender_comp_id.as_str()
//    }
//
//    fn target_comp_id(&self) -> &str {
//        self.target_comp_id.as_str()
//    }
//
//    fn begin_string(&self) -> &[u8] {
//        self.begin_string.as_bytes()
//    }
//
//    fn ping(&mut self) -> Option<Message> {
//        None
//    }
//
//    #[must_use]
//    fn on_garbled_message(&mut self) {}
//
//    pub fn queue(&mut self) -> Responses {
//        Responses { connection: self }
//    }
//
//    #[must_use]
//    pub fn on_inbound_message(&mut self, msg: Message) -> Response {
//        let seq_numbers = self.seq_numbers();
//        let env = self.environment();
//        // Check `TestMessageIndicator(464)`.
//        match (env, msg.f_test_indicator()) {
//            (Environment::ProductionDisallowTest, Some(true)) => {
//                return self.on_wrong_environment(msg);
//            }
//            _ => (),
//        };
//        // Compare seq. numbers.
//        let seqnum_state = msg
//            .f_seq_num()
//            .map(|seqnum| seq_numbers.validate_inbound(seqnum))
//            .unwrap_or(Err(SeqNumberError::NoSeqNum));
//        // Compare the incoming seq. number to the one we expected and act
//        // accordingly.
//        match seqnum_state {
//            Ok(()) => {}
//            // See §4.5.3.
//            Err(SeqNumberError::NoSeqNum) => {
//                self.on_missing_seqnum(msg);
//                return self.queue();
//            }
//            // Refer to specs. §4.8 for more information.
//            Err(SeqNumberError::Recover) => {
//                self.on_high_seqnum(msg);
//                return self.queue();
//            }
//            Err(SeqNumberError::TooLow) => {
//                self.on_low_seqnum(msg);
//                return self.queue();
//            }
//        };
//        if !self.sending_time_is_ok(&msg) {
//            self.on_message_with_inaccurate_sending_time();
//            return self.queue();
//        }
//        // Detect Logon <A>.
//        if let Some("A") = msg.f_msg_type() {
//            self.on_logon(msg);
//        } else {
//            self.on_application_message(msg);
//        }
//        self.queue()
//    }
//
//    fn sending_time_is_ok(&self, msg: Message) -> bool {
//        let sending_time = msg.field_as_chrono_dt(tags::SENDING_TIME);
//        if let Some(sending_time) = sending_time {
//            // TODO
//            true
//        } else {
//            false
//        }
//    }
//
//    fn add_comp_id(&self, msg: &mut RawEncoderState) {
//        msg.add_field(tags::SENDER_COMP_ID, self.sender_comp_id().as_bytes());
//        msg.add_field(tags::TARGET_COMP_ID, self.target_comp_id().as_bytes());
//    }
//
//    fn add_seqnum(&self, msg: &mut RawEncoderState) {
//        msg.add_field(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound());
//        self.seq_numbers_mut().incr_outbound();
//    }
//
//    fn add_sending_time(&self, msg: &mut RawEncoderState) {
//        msg.add_field(tags::SENDING_TIME, DtfTimestamp::utc_now());
//    }
//
//    #[must_use]
//    pub fn on_heartbeat_is_due(&mut self) -> Response {
//        let msg = &mut self.encoder.new_message(self.begin_string(), b"0");
//        self.add_comp_id(msg);
//        self.add_sending_time(msg);
//        Response::OutboundBytes(msg.wrap())
//    }
//
//    fn on_wrong_environment(&mut self, _message: Message) -> Response {
//        self.make_logout(errs::production_env())
//    }
//
//    fn on_missing_seqnum(&mut self, _message: Message) -> Response {
//        self.make_logout(errs::missing_field("MsgSeqNum", tags::MSG_SEQ_NUM))
//    }
//
//    fn on_low_seqnum(&mut self, _message: Message) -> Response {
//        self.make_logout(errs::msg_seq_num(self.seq_numbers().next_inbound()))
//    }
//
//    fn on_reject(
//        &mut self,
//        ref_seq_num: u64,
//        ref_tag: Option<u32>,
//        ref_msg_type: Option<&[u8]>,
//        reason: u32,
//        err_text: String,
//    ) -> Response {
//        let msg = &mut self.encoder.new_message(self.begin_string(), b"3");
//        self.add_comp_id(msg);
//        self.add_sending_time(msg);
//        self.add_seqnum(msg);
//        if let Some(ref_tag) = ref_tag {
//            msg.add_field(tags::REF_TAG_ID, ref_tag);
//        }
//        if let Some(ref_msg_type) = ref_msg_type {
//            msg.add_field(tags::REF_MSG_TYPE, ref_msg_type);
//        }
//        msg.add_field(tags::SESSION_REJECT_REASON, reason);
//        msg.add_field(tags::TEXT, err_text.as_bytes());
//        Response::OutboundBytes(msg.wrap())
//    }
//
//    fn make_reject_for_inaccurate_sending_time(&mut self, offender: &Message) -> Response {
//        let ref_seq_num = offender.f_seq_num().unwrap();
//        let ref_msg_type = offender.f_msg_type().unwrap();
//        self.on_reject(
//            ref_seq_num,
//            Some(tags::SENDING_TIME),
//            Some(ref_msg_type.as_bytes()),
//            10,
//            "Bad SendingTime".to_string(),
//        )
//    }
//
//    fn make_logout(&mut self, text: String) -> Response {
//        let msg = &mut self.encoder.new_message(self.begin_string(), b"5");
//        self.add_comp_id(msg);
//        msg.add_field(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound() as i64);
//        msg.add_field(tags::TEXT, text.as_bytes());
//        Response::OutboundBytes(msg.wrap())
//    }
//
//    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
//        let msg = &mut self.encoder.new_message(self.begin_string(), b"2");
//        self.add_comp_id(msg);
//        self.add_sending_time(msg);
//        self.add_seqnum(msg);
//        msg.add_field(tags::BEGIN_SEQ_NO, start);
//        msg.add_field(tags::END_SEQ_NO, end);
//        Response::OutboundBytes(msg.wrap())
//    }
//
//    fn on_high_seqnum(&mut self, message: Message) {
//        self.make_resend_request(
//            self.seq_numbers().next_inbound(),
//            message.f_seq_num().unwrap(),
//        );
//    }
//
//    fn on_logon(&mut self, logon: Message) {
//        let msg = &mut self.encoder.new_message(self.begin_string(), b"A");
//        self.add_comp_id(msg);
//        self.add_sending_time(msg);
//        self.add_sending_time(msg);
//    }
//
//    fn on_application_message(&mut self, message: FixMessage) {
//        self.enqueue(Response::Inbound(message));
//    }
//}
//
//pub fn add_time_to_msg(mut msg: FixMessage) -> FixMessage {
//    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
//    let time = chrono::Utc::now();
//    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
//    msg.add_str(tags::SENDING_TIME, timestamp.to_string());
//    msg
//}
//
//#[cfg(test)]
//mod test {
//    use super::*;
//    use std::time::Duration;
//
//    fn conn() -> FixConnection {
//        let builder = FixConnectionBuilder {
//            environment: Environment::ProductionDisallowTest,
//            heartbeat: Duration::from_secs(30),
//            seq_numbers: SeqNumbers::default(),
//            sender_comp_id: "SENDER".to_string(),
//            target_comp_id: "TARGET".to_string(),
//        };
//        builder.build()
//    }
//
//    #[test]
//    fn on_heartbeat_is_due() {
//        let conn = &mut conn();
//        let responses = &mut conn.on_heartbeat_is_due();
//        let next = responses.next().unwrap();
//        let msg = next.as_outbound().unwrap();
//        assert_eq!(msg.field_str(tags::MSG_TYPE), Some("0"));
//        assert_eq!(msg.field_str(tags::SENDER_COMP_ID), Some("SENDER"));
//        assert_eq!(msg.field_str(tags::TARGET_COMP_ID), Some("TARGET"));
//        assert_eq!(msg.field_bool(tags::POSS_DUP_FLAG), None);
//        assert_eq!(msg.field_i64(tags::TEST_REQ_ID), None);
//        assert!(responses.next().is_none());
//    }
//
//    #[test]
//    fn terminate_transport_when_error() {
//        let conn = &mut conn();
//        let responses = &mut conn.on_transport_error();
//        let next = responses.next().unwrap();
//        assert!(next.as_terminate_transport().is_some());
//    }
//
//    #[test]
//    fn inaccurate_sending_time() {
//        let conn = &mut conn();
//        let mut msg = FixMessage::new();
//        msg.add_str(tags::MSG_TYPE, "BE");
//        msg.add_str(tags::SENDER_COMP_ID, "SENDER");
//        msg.add_str(tags::TARGET_COMP_ID, "TARGET");
//        msg.add_i64(tags::MSG_SEQ_NUM, 1);
//        msg.add_str(
//            tags::USER_REQUEST_ID,
//            "47b6f4a6-993d-4430-b68f-d9b680a1a772",
//        );
//        msg.add_i64(tags::USER_REQUEST_TYPE, 1);
//        msg.add_str(tags::USERNAME, "john-doe");
//        let mut responses = conn.on_inbound_message(msg);
//        let next = responses.next().unwrap();
//        let msg = next.as_outbound().unwrap();
//        assert_eq!(msg.field_str(tags::MSG_TYPE), Some("3"));
//        assert_eq!(msg.field_str(tags::SENDER_COMP_ID), Some("SENDER"));
//        assert_eq!(msg.field_str(tags::TARGET_COMP_ID), Some("TARGET"));
//        assert_eq!(msg.field_bool(tags::POSS_DUP_FLAG), None);
//        assert_eq!(msg.field_i64(tags::TEST_REQ_ID), None);
//        assert_eq!(msg.field_i64(tags::SESSION_REJECT_REASON), Some(10));
//        assert_eq!(msg.field_i64(tags::REF_SEQ_NUM), Some(10));
//        assert!(responses.next().is_none());
//    }
//}
