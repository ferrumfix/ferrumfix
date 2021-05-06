use super::errs;
use crate::definitions::fix44;
use crate::dict::IsFieldDefinition;
use crate::session::{Environment, SeqNumbers};
use crate::tagvalue::Fv;
use crate::tagvalue::Message;
use crate::tagvalue::{Decoder, DecoderBuffered, Encoder, EncoderHandle};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::cmp::Ordering;
use std::ops::Range;
use std::pin::Pin;
use std::time::Duration;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MsgSeqNumCounter(pub u64);

impl MsgSeqNumCounter {
    pub const START: Self = Self(0);

    pub fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    pub fn expected(&self) -> u64 {
        self.0 + 1
    }
}

impl Iterator for MsgSeqNumCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        Some(MsgSeqNumCounter::next(self))
    }
}

#[derive(Debug, Clone)]
pub enum Event<'a> {
    Inbound(Message<'a>),
    Resend(Range<u64>),
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(enum_as_inner::EnumAsInner))]
pub enum Response<'a> {
    TerminateTransport,
    Application(Message<'a>),
    Session(&'a [u8]),
    Inbound(Message<'a>),
    Outbound(Message<'a>),
    OutboundBytes(&'a [u8]),
    Resend {
        range: (),
    },
    /// The FIX session processor should log each encountered garbled message to
    /// assist in problem detection and diagnosis.
    LogGarbled,
}

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

#[derive(Debug, Clone)]
pub struct FixConnectionBuilder {
    begin_string: String,
    environment: Environment,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
    sender_comp_id: String,
    target_comp_id: String,
}

impl FixConnectionBuilder {
    pub fn set_begin_string<S>(&mut self, begin_string: S)
    where
        S: Into<String>,
    {
        self.begin_string = begin_string.into();
    }

    pub fn set_environmen(&mut self, env: Environment) {
        self.environment = env;
    }

    pub fn set_seq_numbers(&mut self, inbound: u64, outbound: u64) {
        if inbound == 0 || outbound == 0 {
            panic!("FIX sequence numbers must be strictly positive");
        }
        self.seq_numbers = SeqNumbers {
            next_inbound: inbound,
            next_outbound: outbound,
        };
    }

    pub fn set_sender_comp_id<S>(&mut self, sender_comp_id: S)
    where
        S: Into<String>,
    {
        self.sender_comp_id = sender_comp_id.into();
    }

    pub fn set_target_comp_id<S>(&mut self, target_comp_id: S)
    where
        S: Into<String>,
    {
        self.target_comp_id = target_comp_id.into();
    }

    pub fn build(self) -> FixConnection {
        FixConnection {
            begin_string: self.begin_string,
            environment: self.environment,
            encoder: Encoder::from_buffer(Vec::new()),
            heartbeat: self.heartbeat,
            seq_numbers: self.seq_numbers,
            msg_seq_num_inbound: self.msg_seq_num_inbound,
            msg_seq_num_outbound: self.msg_seq_num_outbound,
            sender_comp_id: self.sender_comp_id,
            target_comp_id: self.target_comp_id,
        }
    }
}

impl Default for FixConnectionBuilder {
    fn default() -> Self {
        Self {
            msg_seq_num_inbound: MsgSeqNumCounter::START,
            msg_seq_num_outbound: MsgSeqNumCounter::START,
            begin_string: "FIX-4.4".to_string(),
            environment: Environment::Testing,
            heartbeat: Duration::from_secs(30),
            seq_numbers: SeqNumbers::default(),
            sender_comp_id: "ABC".to_string(),
            target_comp_id: "XYZ".to_string(),
        }
    }
}

/// A FIX connection message processor.
#[derive(Debug)]
pub struct FixConnection {
    begin_string: String,
    environment: Environment,
    encoder: Encoder,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
    sender_comp_id: String,
    target_comp_id: String,
}

pub trait Application: Clone {
    type Error;

    #[inline(always)]
    fn on_heartbeat_is_due(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_inbound_app_message(&mut self, message: Message) -> Result<(), Self::Error>;

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error>;

    #[inline(always)]
    fn on_inbound_message(&mut self, message: Message, is_app: bool) -> Result<(), Self::Error> {
        println!("received message");
        if is_app {
            self.on_inbound_app_message(message)
        } else {
            Ok(())
        }
    }

    fn on_resend_request(&mut self, range: Range<u64>) -> Result<(), Self::Error>;

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error>;

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error>;

    fn pending_message(&mut self) -> Option<&[u8]>;
}

#[allow(dead_code)]
impl FixConnection {
    pub async fn initiate<A, I, O>(
        &mut self,
        mut app: A,
        mut input: I,
        mut output: O,
        decoder: Decoder,
    ) where
        A: Application,
        I: AsyncRead + std::marker::Unpin,
        O: AsyncWrite + std::marker::Unpin,
    {
        let logon = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self.encoder.start_message(begin_string, b"A");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::SENDING_TIME, chrono::Utc::now());
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::ENCRYPT_METHOD, fix44::EncryptMethod::None);
            msg.set(fix44::HEART_BT_INT, self.heartbeat.as_secs());
            msg.wrap()
        };
        output.write(logon).await.unwrap();
        app.on_outbound_message(logon).ok();
        let mut decoder = decoder.buffered();
        let logon;
        loop {
            let mut input = Pin::new(&mut input);
            let buffer = decoder.supply_buffer();
            input.read_exact(buffer).await.unwrap();
            if let Ok(Some(_)) = decoder.current_message() {
                logon = decoder.message();
                break;
            }
        }
        self.on_logon(logon);
        app.on_inbound_message(logon, true).ok();
        self.msg_seq_num_inbound.next();
        app.on_successful_handshake().ok();
        self.event_loop(app, input, output, decoder).await;
    }

    pub async fn event_loop<A, I, O>(
        &mut self,
        mut app: A,
        mut data: I,
        mut output: O,
        mut decoder: DecoderBuffered,
    ) where
        A: Application,
        I: AsyncRead + std::marker::Unpin,
        O: AsyncWrite + std::marker::Unpin,
    {
        use futures::future::{self, Either};
        let mut heartbeat_timer = futures_timer::Delay::new(Duration::from_secs(3));
        loop {
            let mut pinned = Pin::new(&mut data);
            let buffer = decoder.supply_buffer();
            let bytes = pinned.read_exact(buffer);
            match future::select(bytes, heartbeat_timer).await {
                Either::Left((res, x)) => {
                    if res.is_err() {
                        break;
                    }
                    if let Ok(Some(_)) = decoder.current_message() {
                        let msg = decoder.message();
                        let response = self.on_inbound_message(msg, &mut app);
                        match response {
                            Response::OutboundBytes(bytes) => {
                                output.write_all(bytes).await.unwrap();
                                app.on_outbound_message(bytes).ok();
                            }
                            _ => {}
                        }
                        self.msg_seq_num_inbound.next();
                        decoder.clear();
                    }
                    heartbeat_timer = x;
                }
                Either::Right((_y, _)) => {
                    let heartbeat = self.on_heartbeat_is_due();
                    output.write_all(heartbeat).await.unwrap();
                    app.on_outbound_message(heartbeat).ok();
                    heartbeat_timer = futures_timer::Delay::new(Duration::from_secs(3));
                }
            };
        }
    }

    pub async fn accept<A, I, O>(&mut self, app: A, data: I, output: O, decoder: DecoderBuffered)
    where
        A: Application,
        I: AsyncRead + std::marker::Unpin,
        O: AsyncWrite + std::marker::Unpin,
    {
        self.event_loop(app, data, output, decoder).await;
    }

    fn seq_numbers(&self) -> SeqNumbers {
        self.seq_numbers
    }

    fn seq_numbers_mut(&mut self) -> &mut SeqNumbers {
        &mut self.seq_numbers
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

    fn begin_string(&self) -> &[u8] {
        self.begin_string.as_bytes()
    }

    fn on_inbound_message<'a, A>(&'a mut self, msg: Message<'a>, app: &mut A) -> Response<'a>
    where
        A: Application,
    {
        let env = self.environment();
        // Check `TestMessageIndicator <464>`.
        if let Ok(indicator) = msg.fv::<bool, _>(fix44::TEST_MESSAGE_INDICATOR) {
            if !env.allows_testing() && indicator {
                return self.on_wrong_environment(msg);
            }
        }
        // Compare seq. numbers.
        let msg_seq_num_cmp = msg
            .fv::<u64, _>(fix44::MSG_SEQ_NUM)
            .map(|seqnum| seqnum.cmp(&self.msg_seq_num_inbound.expected()));
        // Compare the incoming seq. number to the one we expected and act
        // accordingly.
        match msg_seq_num_cmp {
            Ok(Ordering::Equal) => {}
            Ok(Ordering::Less) => {
                return self.on_low_seqnum(msg);
            }
            Ok(Ordering::Greater) => {
                // Refer to specs. §4.8 for more information.
                return self.on_high_seqnum(msg);
            }
            Err(_) => {
                // See §4.5.3.
                return self.on_missing_seqnum(msg);
            }
        };
        if !self.sending_time_is_ok(&msg) {
            return self.make_reject_for_inaccurate_sending_time(msg);
        }
        match msg.fv::<&[u8], _>(fix44::MSG_TYPE) {
            Ok(b"A") => {
                self.on_logon(msg);
                app.on_inbound_message(msg, false).ok();
                return Response::OutboundBytes(b"");
            }
            Ok(b"1") => {
                app.on_inbound_message(msg, false).ok();
                self.on_test_request(msg);
            }
            Ok(b"2") => {
                app.on_inbound_message(msg, false).ok();
            }
            Ok(b"5") => {
                app.on_inbound_message(msg, false).ok();
            }
            Ok(b"0") => {
                self.on_heartbeat(msg);
                app.on_inbound_message(msg, false).ok();
            }
            _ => {
                app.on_inbound_app_message(msg).ok();
                return self.on_application_message(msg);
            }
        }
        todo!()
    }

    fn sending_time_is_ok(&self, msg: &Message) -> bool {
        let sending_time = msg.fv::<&str, _>(fix44::SENDING_TIME);
        if let Ok(_sending_time) = sending_time {
            // TODO
            true
        } else {
            false
        }
    }

    fn add_comp_id(msg: &mut EncoderHandle, sender: &str, target: &str) {
        msg.set(fix44::SENDER_COMP_ID, sender);
        msg.set(fix44::TARGET_COMP_ID, target);
    }

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
    pub fn on_heartbeat_is_due(&mut self) -> &[u8] {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self.encoder.start_message(begin_string, b"0");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::SENDING_TIME, chrono::Utc::now());
            msg.wrap()
        };
        fix_message
    }

    pub fn on_heartbeat(&mut self, _msg: Message) {
        // TODO: verify stuff.
    }

    fn on_test_request(&mut self, msg: Message) -> &[u8] {
        let test_req_id = msg.fv::<&[u8], _>(fix44::TEST_REQ_ID).unwrap();
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self.encoder.start_message(begin_string, b"1");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::SENDING_TIME, chrono::Utc::now());
            msg.set(fix44::TEST_REQ_ID, test_req_id);
            msg.wrap()
        };
        fix_message
    }

    fn on_wrong_environment(&mut self, _message: Message) -> Response {
        self.make_logout(errs::production_env())
    }

    fn generate_error_seqnum_too_low(&mut self) -> &[u8] {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let text = errs::msg_seq_num(self.msg_seq_num_inbound.0 + 1);
            let mut msg = self.encoder.start_message(begin_string, b"FIXME");
            msg.set(fix44::MSG_TYPE, "5");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::TEXT, text.as_str());
            msg.wrap()
        };
        fix_message
    }

    fn on_missing_seqnum(&mut self, _message: Message) -> Response {
        self.make_logout(errs::missing_field(
            fix44::MSG_SEQ_NUM.name(),
            fix44::MSG_SEQ_NUM.tag().get().into(),
        ))
    }

    fn on_low_seqnum(&mut self, _message: Message) -> Response {
        self.make_logout(errs::msg_seq_num(self.msg_seq_num_inbound.0 + 1))
    }

    fn on_reject(
        &mut self,
        _ref_seq_num: u64,
        ref_tag: Option<u32>,
        ref_msg_type: Option<&[u8]>,
        reason: u32,
        err_text: String,
    ) -> Response {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self.encoder.start_message(begin_string, b"3");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            if let Some(ref_tag) = ref_tag {
                msg.set(fix44::REF_TAG_ID, ref_tag);
            }
            if let Some(ref_msg_type) = ref_msg_type {
                msg.set(fix44::REF_MSG_TYPE, ref_msg_type);
            }
            msg.set(fix44::SESSION_REJECT_REASON, reason);
            msg.set(fix44::TEXT, err_text.as_str());
            msg.wrap()
        };
        Response::OutboundBytes(fix_message)
    }

    fn make_reject_for_inaccurate_sending_time(&mut self, offender: Message) -> Response {
        let ref_seq_num = offender.fv(fix44::MSG_SEQ_NUM).unwrap();
        let ref_msg_type = offender.fv::<&str, _>(fix44::MSG_TYPE).unwrap();
        self.on_reject(
            ref_seq_num,
            Some(fix44::SENDING_TIME.tag().get().into()),
            Some(ref_msg_type.as_bytes()),
            fix44::SessionRejectReason::SendingtimeAccuracyProblem as u32,
            "Bad SendingTime".to_string(),
        )
    }

    fn make_logout(&mut self, text: String) -> Response {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self.encoder.start_message(begin_string, b"5");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::TEXT, text.as_str());
            msg.wrap()
        };
        Response::OutboundBytes(fix_message)
    }

    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
        let begin_string = self.begin_string.as_bytes();
        let mut msg = self.encoder.start_message(begin_string, b"2");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_seqnum(msg);
        msg.set(fix44::BEGIN_SEQ_NO, start);
        msg.set(fix44::END_SEQ_NO, end);
        Response::OutboundBytes(msg.wrap())
    }

    fn on_high_seqnum(&mut self, msg: Message) -> Response {
        let msg_seq_num = msg.fv(fix44::MSG_SEQ_NUM).unwrap();
        self.make_resend_request(self.seq_numbers().next_inbound(), msg_seq_num);
        todo!()
    }

    fn on_logon(&mut self, _logon: Message) {
        let begin_string = self.begin_string.as_bytes();
        let mut _msg = self.encoder.start_message(begin_string, b"A");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_sending_time(msg);
    }

    fn on_application_message<'a>(&mut self, msg: Message<'a>) -> Response<'a> {
        Response::Application(msg)
    }
}

//fn add_time_to_msg(mut msg: EncoderHandle) {
//    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
//    let time = chrono::Utc::now();
//    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
//    msg.set(fix44::SENDING_TIME, timestamp.to_string().as_str());
//}

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
