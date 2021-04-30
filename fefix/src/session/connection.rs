use super::errs;
use super::SeqNumberError;
use crate::definitions::fixt11;
use crate::dict::IsFieldDefinition;
use crate::session::{Environment, SeqNumbers};
use crate::tagvalue::Fv;
use crate::tagvalue::Message;
use crate::tagvalue::{Decoder, Encoder, EncoderHandle};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::pin::Pin;
use std::time::Duration;

//#[derive(Debug, Clone)]
//pub enum Event {
//    TransportError,
//    MissedHeartbeat,
//    HeartbeatIsDue,
//    Inbound(FixMessage),
//    Outbound(FixMessage),
//    Garbled,
//}

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

    pub fn build(self) -> FixConnection<LogNone> {
        FixConnection {
            begin_string: self.begin_string,
            environment: self.environment,
            encoder: Encoder::from_buffer(Vec::new()),
            heartbeat: self.heartbeat,
            seq_numbers: self.seq_numbers,
            sender_comp_id: self.sender_comp_id,
            target_comp_id: self.target_comp_id,
            logger: LogNone {},
        }
    }
}

impl Default for FixConnectionBuilder {
    fn default() -> Self {
        Self {
            begin_string: "FIX-4.4".to_string(),
            environment: Environment::Testing,
            heartbeat: Duration::from_secs(30),
            seq_numbers: SeqNumbers::default(),
            sender_comp_id: "ABC".to_string(),
            target_comp_id: "XYZ".to_string(),
        }
    }
}

pub trait Logger {
    fn log_inbound(&mut self, _fix_message: &[u8]) {}
    fn log_outbound(&mut self, _fix_message: &[u8]) {}
    fn log_establis_conn(&mut self) {}
}

#[derive(Debug, Copy, Clone)]
pub struct LogNone {}

impl Logger for LogNone {}

/// A FIX connection message processor.
#[derive(Debug)]
pub struct FixConnection<L>
where
    L: Logger,
{
    begin_string: String,
    environment: Environment,
    encoder: Encoder,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
    sender_comp_id: String,
    target_comp_id: String,
    logger: L,
}

#[allow(dead_code)]
impl<L> FixConnection<L>
where
    L: Logger,
{
    pub async fn initiate<I, O>(&mut self, mut input: I, mut output: O, decoder: Decoder)
    where
        I: AsyncRead + std::marker::Unpin,
        O: AsyncWrite + std::marker::Unpin,
    {
        let logon = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.seq_numbers.next_inbound() as i64;
            let mut msg = self.encoder.start_message(begin_string, b"A");
            msg.set(fixt11::SENDER_COMP_ID, sender_comp_id);
            msg.set(fixt11::TARGET_COMP_ID, target_comp_id);
            msg.set(fixt11::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fixt11::HEART_BT_INT, self.heartbeat.as_secs());
            msg.set(fixt11::SENDING_TIME, chrono::Utc::now());
            msg.wrap()
        };
        output.write(logon).await.unwrap();
        let mut decoder = decoder.buffered();
        let logon;
        loop {
            let mut input = Pin::new(&mut input);
            let buffer = decoder.supply_buffer();
            input.read(buffer).await.unwrap();
            if let Ok(Some(_)) = decoder.current_message() {
                logon = decoder.message();
                break;
            }
        }
        println!("{:?} recv logon", logon);
    }

    pub async fn accept<I>(&mut self, mut data: I, decoder: Decoder)
    where
        I: AsyncRead + std::marker::Unpin,
    {
        use futures::future::{self, Either};
        let mut heartbeat_timer = futures_timer::Delay::new(self.heartbeat);
        let mut decoder = decoder.buffered();
        loop {
            let mut pinned = Pin::new(&mut data);
            let buffer = decoder.supply_buffer();
            let bytes = pinned.read(buffer);
            match future::select(heartbeat_timer, bytes).await {
                Either::Left((y, _)) => {
                    y.unwrap();
                    self.on_heartbeat_is_due();
                    heartbeat_timer = futures_timer::Delay::new(self.heartbeat);
                }
                Either::Right((res, x)) => {
                    if res.is_err() {
                        break;
                    }
                    if let Ok(Some(_)) = decoder.current_message() {
                        let msg = decoder.message();
                        self.on_inbound_message(msg);
                    }
                    heartbeat_timer = x;
                }
            };
        }
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
    fn on_inbound_message(&mut self, msg: Message) -> Response {
        self.logger.log_inbound(msg.as_bytes());
        let seq_numbers = self.seq_numbers();
        let env = self.environment();
        // Check `TestMessageIndicator(464)`.
        match (env, msg.fv(fixt11::TEST_MESSAGE_INDICATOR).unwrap()) {
            (Environment::ProductionDisallowTest, true) => {
                return self.on_wrong_environment(msg);
            }
            _ => (),
        };
        // Compare seq. numbers.
        let seqnum_state = msg
            .fv(fixt11::MSG_SEQ_NUM)
            .map(|seqnum| seq_numbers.validate_inbound(seqnum))
            .unwrap_or(Err(SeqNumberError::NoSeqNum));
        // Compare the incoming seq. number to the one we expected and act
        // accordingly.
        match seqnum_state {
            Ok(()) => {}
            // See §4.5.3.
            Err(SeqNumberError::NoSeqNum) => {
                self.on_missing_seqnum(msg);
                todo!();
                //return self.queue();
            }
            // Refer to specs. §4.8 for more information.
            Err(SeqNumberError::Recover) => {
                self.on_high_seqnum(msg);
                todo!();
                //return self.queue();
            }
            Err(SeqNumberError::TooLow) => {
                self.on_low_seqnum(msg);
                todo!();
                //return self.queue();
            }
        };
        if !self.sending_time_is_ok(&msg) {
            self.make_reject_for_inaccurate_sending_time(msg);
            todo!();
            //return self.queue();
        }
        // Detect Logon <A>.
        if let Ok("A") = msg.fv(fixt11::MSG_TYPE) {
            self.on_logon(msg);
        } else {
            self.on_application_message(msg);
        }
        todo!()
        //self.queue()
    }

    fn sending_time_is_ok(&self, msg: &Message) -> bool {
        let sending_time = msg.fv::<&str, _>(fixt11::SENDING_TIME);
        if let Ok(_sending_time) = sending_time {
            // TODO
            true
        } else {
            false
        }
    }

    fn add_comp_id(msg: &mut EncoderHandle, sender: &str, target: &str) {
        msg.set(fixt11::SENDER_COMP_ID, sender);
        msg.set(fixt11::TARGET_COMP_ID, target);
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
    pub fn on_heartbeat_is_due(&mut self) -> Response {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.seq_numbers.next_inbound() as i64;
            let mut msg = self.encoder.start_message(begin_string, b"0");
            msg.set(fixt11::SENDER_COMP_ID, sender_comp_id);
            msg.set(fixt11::TARGET_COMP_ID, target_comp_id);
            msg.set(fixt11::MSG_SEQ_NUM, msg_seq_num);
            msg.wrap()
        };
        self.logger.log_outbound(fix_message);
        Response::OutboundBytes(fix_message)
    }

    fn on_wrong_environment(&mut self, _message: Message) -> Response {
        self.make_logout(errs::production_env())
    }

    fn generate_error_seqnum_too_low(&mut self) {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.seq_numbers.next_inbound() as i64;
            let text = errs::msg_seq_num(self.seq_numbers().next_inbound());
            let mut msg = self.encoder.start_message(begin_string, b"FIXME");
            msg.set(fixt11::MSG_TYPE, "5");
            msg.set(fixt11::SENDER_COMP_ID, sender_comp_id);
            msg.set(fixt11::TARGET_COMP_ID, target_comp_id);
            msg.set(fixt11::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fixt11::TEXT, text.as_str());
            msg.wrap()
        };
        self.logger.log_outbound(fix_message);
    }

    fn on_missing_seqnum(&mut self, _message: Message) -> Response {
        self.make_logout(errs::missing_field(
            fixt11::MSG_SEQ_NUM.name(),
            fixt11::MSG_SEQ_NUM.tag().get().into(),
        ))
    }

    fn on_low_seqnum(&mut self, _message: Message) -> Response {
        self.make_logout(errs::msg_seq_num(self.seq_numbers().next_inbound()))
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
            let msg_seq_num = self.seq_numbers.next_inbound() as i64;
            let mut msg = self.encoder.start_message(begin_string, b"3");
            msg.set(fixt11::SENDER_COMP_ID, sender_comp_id);
            msg.set(fixt11::TARGET_COMP_ID, target_comp_id);
            msg.set(fixt11::MSG_SEQ_NUM, msg_seq_num);
            if let Some(ref_tag) = ref_tag {
                msg.set(fixt11::REF_TAG_ID, ref_tag);
            }
            if let Some(ref_msg_type) = ref_msg_type {
                msg.set(fixt11::REF_MSG_TYPE, ref_msg_type);
            }
            msg.set(fixt11::SESSION_REJECT_REASON, reason);
            msg.set(fixt11::TEXT, err_text.as_str());
            msg.wrap()
        };
        self.logger.log_outbound(fix_message);
        Response::OutboundBytes(fix_message)
    }

    fn make_reject_for_inaccurate_sending_time(&mut self, offender: Message) -> Response {
        let ref_seq_num = offender.fv(fixt11::MSG_SEQ_NUM).unwrap();
        let ref_msg_type = offender.fv::<&str, _>(fixt11::MSG_TYPE).unwrap();
        self.on_reject(
            ref_seq_num,
            Some(fixt11::SENDING_TIME.tag().get().into()),
            Some(ref_msg_type.as_bytes()),
            fixt11::SessionRejectReason::SendingtimeAccuracyProblem as u32,
            "Bad SendingTime".to_string(),
        )
    }

    fn make_logout(&mut self, text: String) -> Response {
        let fix_message = {
            let begin_string = self.begin_string.as_bytes();
            let sender_comp_id = self.sender_comp_id.as_str();
            let target_comp_id = self.target_comp_id.as_str();
            let msg_seq_num = self.seq_numbers.next_inbound() as i64;
            let mut msg = self.encoder.start_message(begin_string, b"5");
            msg.set(fixt11::SENDER_COMP_ID, sender_comp_id);
            msg.set(fixt11::TARGET_COMP_ID, target_comp_id);
            msg.set(fixt11::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fixt11::TEXT, text.as_str());
            msg.wrap()
        };
        self.logger.log_outbound(fix_message);
        Response::OutboundBytes(fix_message)
    }

    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
        let begin_string = self.begin_string.as_bytes();
        let mut msg = self.encoder.start_message(begin_string, b"2");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_seqnum(msg);
        msg.set(fixt11::BEGIN_SEQ_NO, start);
        msg.set(fixt11::END_SEQ_NO, end);
        Response::OutboundBytes(msg.wrap())
    }

    fn on_high_seqnum(&mut self, _message: Message) {
        //self.make_resend_request(
        //    self.seq_numbers().next_inbound(),
        //    message.f_seq_num().unwrap(),
        //);
    }

    fn on_logon(&mut self, _logon: Message) {
        let begin_string = self.begin_string.as_bytes();
        let mut _msg = self.encoder.start_message(begin_string, b"A");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_sending_time(msg);
    }

    fn on_application_message(&mut self, _message: Message) {
        //self.enqueue(Response::Inbound(message));
    }
}

//fn add_time_to_msg(mut msg: EncoderHandle) {
//    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
//    let time = chrono::Utc::now();
//    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
//    msg.set(fixt11::SENDING_TIME, timestamp.to_string().as_str());
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
