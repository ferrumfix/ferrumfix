use super::{errs, Backend, Config, Configure, LlEvent, LlEventLoop};
use crate::definitions::fix44;
use crate::dict::IsFieldDefinition;
use crate::session::{Environment, SeqNumbers};
use crate::tagvalue::FieldAccess;
use crate::tagvalue::Message;
use crate::tagvalue::{DecoderBuffered, Encoder, EncoderHandle};
use crate::Buffer;
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::cmp::Ordering;
use std::marker::Unpin;
use std::pin::Pin;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct MsgSeqNumCounter(pub u64);

impl MsgSeqNumCounter {
    pub const START: Self = Self(0);

    #[inline]
    pub fn next(&mut self) -> u64 {
        self.0 += 1;
        self.0
    }

    #[inline]
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
#[cfg_attr(test, derive(enum_as_inner::EnumAsInner))]
pub enum Response<'a> {
    None,
    ResetHeartbeat,
    TerminateTransport,
    Application(Message<'a, &'a [u8]>),
    Session(&'a [u8]),
    Inbound(Message<'a, &'a [u8]>),
    Outbound(Message<'a, &'a [u8]>),
    OutboundBytes(&'a [u8]),
    Resend {
        range: (),
    },
    /// The FIX session processor should log each encountered garbled message to
    /// assist in problem detection and diagnosis.
    LogGarbled,
}

/// A FIX connection message processor.
#[derive(Debug)]
pub struct FixConnection<C = Config> {
    uuid: Uuid,
    config: C,
    encoder: Encoder,
    buffer: Vec<u8>,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
}

#[allow(dead_code)]
impl<C> FixConnection<C>
where
    C: Configure,
{
    /// Creates a new [`FixConnection`] with the settings provided by `config`.
    pub fn new(config: C) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            config,
            encoder: Encoder::default(),
            buffer: vec![],
            msg_seq_num_inbound: MsgSeqNumCounter::START,
            msg_seq_num_outbound: MsgSeqNumCounter::START,
        }
    }

    /// The entry point for a [`FixConnection`].
    pub async fn start<B, I, O>(
        &mut self,
        mut app: B,
        mut input: I,
        mut output: O,
        mut decoder: DecoderBuffered,
    ) where
        B: Backend,
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        self.establish_connection(&mut app, &mut input, &mut output, &mut decoder)
            .await;
        self.event_loop(app, input, output, decoder).await;
    }

    async fn establish_connection<A, I, O>(
        &mut self,
        app: &mut A,
        mut input: &mut I,
        output: &mut O,
        decoder: &mut DecoderBuffered,
    ) where
        A: Backend,
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        let logon = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let heartbeat = self.config.heartbeat().as_secs();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"A");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::SENDING_TIME, chrono::Utc::now());
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::ENCRYPT_METHOD, fix44::EncryptMethod::None);
            msg.set(fix44::HEART_BT_INT, heartbeat);
            msg.wrap()
        };
        output.write(logon).await.unwrap();
        app.on_outbound_message(logon).ok();
        let logon;
        loop {
            let mut input = Pin::new(&mut input);
            let buffer = decoder.supply_buffer();
            input.read_exact(buffer).await.unwrap();
            if let Ok(Some(())) = decoder.parse() {
                logon = decoder.message();
                break;
            }
        }
        self.on_logon(logon);
        app.on_inbound_message(logon, true).ok();
        decoder.clear();
        self.msg_seq_num_inbound.next();
        app.on_successful_handshake().ok();
    }

    async fn event_loop<A, I, O>(
        &mut self,
        mut app: A,
        input: I,
        mut output: O,
        decoder: DecoderBuffered,
    ) where
        A: Backend,
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        let event_loop = &mut LlEventLoop::new(decoder, input, self.heartbeat());
        loop {
            let event = event_loop
                .next_event()
                .await
                .expect("The connection died unexpectedly.");
            match event {
                LlEvent::Message(msg) => {
                    let response = self.on_inbound_message(msg, &mut app);
                    match response {
                        Response::OutboundBytes(bytes) => {
                            output.write_all(bytes).await.unwrap();
                            app.on_outbound_message(bytes).ok();
                        }
                        Response::ResetHeartbeat => {
                            event_loop.ping_heartbeat();
                        }
                        _ => {}
                    }
                }
                LlEvent::BadMessage(_err) => {}
                LlEvent::IoError(_) => {
                    return;
                }
                LlEvent::Heartbeat => {
                    let heartbeat = self.on_heartbeat_is_due();
                    output.write_all(heartbeat).await.unwrap();
                    app.on_outbound_message(heartbeat).ok();
                }
                LlEvent::Logout => {}
                LlEvent::TestRequest => {}
            }
        }
    }

    fn seq_numbers(&self) -> SeqNumbers {
        SeqNumbers {
            next_inbound: self.msg_seq_num_inbound.expected(),
            next_outbound: self.msg_seq_num_outbound.expected(),
        }
    }

    fn environment(&self) -> Environment {
        self.config.environment()
    }

    fn sender_comp_id(&self) -> &[u8] {
        self.config.sender_comp_id()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.config.target_comp_id()
    }

    fn begin_string(&self) -> &[u8] {
        self.config.begin_string()
    }

    fn heartbeat(&self) -> Duration {
        self.config.heartbeat()
    }

    fn on_inbound_message<'a, B>(
        &'a mut self,
        msg: Message<'a, &'a [u8]>,
        app: &mut B,
    ) -> Response<'a>
    where
        B: Backend,
    {
        let env = self.environment();
        // Check `TestMessageIndicator <464>`.
        if let Ok(indicator) = msg.fv::<bool>(fix44::TEST_MESSAGE_INDICATOR) {
            if !env.allows_testing() && indicator {
                return self.on_wrong_environment(msg);
            }
        }
        let msg_seq_num = msg.fv::<u64>(fix44::MSG_SEQ_NUM);
        // Compare seq. numbers.
        let msg_seq_num_cmp =
            msg_seq_num.map(|seqnum| seqnum.cmp(&self.msg_seq_num_inbound.expected()));
        // Increment immediately.
        self.msg_seq_num_inbound.next();
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
        let msg_type = msg.fv::<&[u8]>(fix44::MSG_TYPE).unwrap();
        match msg_type {
            b"A" => {
                self.on_logon(msg);
                app.on_inbound_message(msg, false).ok();
                return Response::None;
            }
            b"1" => {
                app.on_inbound_message(msg, false).ok();
                let msg = self.on_test_request(msg);
                return Response::OutboundBytes(msg);
            }
            b"2" => {
                app.on_inbound_message(msg, false).ok();
                self.on_resend_request(&msg, app);
                return Response::None;
            }
            b"5" => {
                app.on_inbound_message(msg, false).ok();
                return Response::OutboundBytes(self.on_logout(&msg));
            }
            b"0" => {
                self.on_heartbeat(msg);
                app.on_inbound_message(msg, false).ok();
                return Response::ResetHeartbeat;
            }
            _ => {
                app.on_inbound_app_message(msg).ok();
                return self.on_application_message(msg);
            }
        }
    }

    fn on_resend_request<B>(&self, msg: &Message<&[u8]>, app: &mut B)
    where
        B: Backend,
    {
        let begin_seq_num = msg.fv(fix44::BEGIN_SEQ_NO).unwrap();
        let end_seq_num = msg.fv(fix44::END_SEQ_NO).unwrap();
        app.on_resend_request(begin_seq_num..end_seq_num).ok();
    }

    fn on_logout(&mut self, _msg: &Message<&[u8]>) -> &[u8] {
        let fix_message = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"5");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::TEXT, "Logout");
            msg.wrap()
        };
        fix_message
    }

    fn sending_time_is_ok(&self, msg: &Message<&[u8]>) -> bool {
        let sending_time = msg.fv::<&str>(fix44::SENDING_TIME);
        if let Ok(_sending_time) = sending_time {
            // TODO
            true
        } else {
            false
        }
    }

    fn add_comp_id<B, T>(msg: &mut EncoderHandle<B, T>, sender: &str, target: &str)
    where
        B: Buffer,
        T: crate::tagvalue::Configure,
    {
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
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"0");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::SENDING_TIME, chrono::Utc::now());
            msg.wrap()
        };
        fix_message
    }

    pub fn on_heartbeat(&mut self, _msg: Message<&[u8]>) {
        // TODO: verify stuff.
    }

    fn on_test_request(&mut self, msg: Message<&[u8]>) -> &[u8] {
        let test_req_id = msg.fv::<&[u8]>(fix44::TEST_REQ_ID).unwrap();
        let fix_message = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"1");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::SENDING_TIME, chrono::Utc::now());
            msg.set(fix44::TEST_REQ_ID, test_req_id);
            msg.wrap()
        };
        fix_message
    }

    fn on_wrong_environment(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::production_env())
    }

    fn generate_error_seqnum_too_low(&mut self) -> &[u8] {
        let fix_message = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let text = errs::msg_seq_num(self.msg_seq_num_inbound.0 + 1);
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"FIXME");
            msg.set(fix44::MSG_TYPE, "5");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::TEXT, text.as_str());
            msg.wrap()
        };
        fix_message
    }

    fn on_missing_seqnum(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::missing_field(
            fix44::MSG_SEQ_NUM.name(),
            fix44::MSG_SEQ_NUM.tag().get().into(),
        ))
    }

    fn on_low_seqnum(&mut self, _message: Message<&[u8]>) -> Response {
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
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"3");
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

    fn make_reject_for_inaccurate_sending_time(&mut self, offender: Message<&[u8]>) -> Response {
        let ref_seq_num = offender.fv(fix44::MSG_SEQ_NUM).unwrap();
        let ref_msg_type = offender.fv::<&str>(fix44::MSG_TYPE).unwrap();
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
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"5");
            msg.set(fix44::SENDER_COMP_ID, sender_comp_id);
            msg.set(fix44::TARGET_COMP_ID, target_comp_id);
            msg.set(fix44::MSG_SEQ_NUM, msg_seq_num);
            msg.set(fix44::TEXT, text.as_str());
            msg.wrap()
        };
        Response::OutboundBytes(fix_message)
    }

    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
        let begin_string = self.config.begin_string();
        let mut msg = self
            .encoder
            .start_message(begin_string, &mut self.buffer, b"2");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_seqnum(msg);
        msg.set(fix44::BEGIN_SEQ_NO, start);
        msg.set(fix44::END_SEQ_NO, end);
        Response::OutboundBytes(msg.wrap())
    }

    fn on_high_seqnum(&mut self, msg: Message<&[u8]>) -> Response {
        let msg_seq_num = msg.fv(fix44::MSG_SEQ_NUM).unwrap();
        self.make_resend_request(self.seq_numbers().next_inbound(), msg_seq_num);
        todo!()
    }

    fn on_logon(&mut self, _logon: Message<&[u8]>) {
        let begin_string = self.config.begin_string();
        let mut _msg = self
            .encoder
            .start_message(begin_string, &mut self.buffer, b"A");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_sending_time(msg);
    }

    fn on_application_message<'a>(&mut self, msg: Message<'a, &'a [u8]>) -> Response<'a> {
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
