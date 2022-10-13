use super::{errs, Backend, Config, Configure, LlEvent, LlEventLoop};
use crate::field_types::Timestamp;
use crate::session::{Environment, MsgSeqNumCounter, SeqNumbers};
use crate::tagvalue::Message;
use crate::tagvalue::{DecoderStreaming, Encoder, EncoderHandle};
use crate::FieldType;
use crate::{field_types, FieldMap, StreamingDecoder};
use crate::{Buffer, SetField};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use std::marker::{PhantomData, Unpin};
use std::pin::Pin;
use std::time::Duration;
use uuid::Uuid;

const BEGIN_SEQ_NO: u32 = 7;
const BEGIN_STRING: u32 = 8;
const END_SEQ_NO: u32 = 16;
const MSG_SEQ_NUM: u32 = 34;
const MSG_TYPE: u32 = 35;
const SENDER_COMP_ID: u32 = 49;
const SENDING_TIME: u32 = 52;
const TARGET_COMP_ID: u32 = 56;
const TEXT: u32 = 58;
const ENCRYPT_METHOD: u32 = 98;
const TEST_REQ_ID: u32 = 112;
const REF_TAG_ID: u32 = 371;
const REF_MSG_TYPE: u32 = 372;
const SESSION_REJECT_REASON: u32 = 373;
const TEST_MESSAGE_INDICATOR: u32 = 464;

const SENDING_TIME_ACCURACY_PROBLEM: u32 = 10;

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
pub struct FixConnection<B, C = Config> {
    uuid: Uuid,
    config: C,
    backend: B,
    encoder: Encoder,
    buffer: Vec<u8>,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
}

#[allow(dead_code)]
impl<B, C> FixConnection<B, C>
where
    B: Backend,
    C: Configure,
{
    /// The entry point for a [`FixConnection`].
    async fn start<I, O>(
        &mut self,
        mut input: I,
        mut output: O,
        mut decoder: DecoderStreaming<Vec<u8>>,
    ) where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        self.establish_connection(&mut input, &mut output, &mut decoder)
            .await;
        self.event_loop(input, output, decoder).await;
    }

    async fn establish_connection<I, O>(
        &mut self,
        mut input: &mut I,
        output: &mut O,
        decoder: &mut DecoderStreaming<Vec<u8>>,
    ) where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        let (logon, _) = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let heartbeat = self.config.heartbeat().as_secs();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"A");
            msg.set_fv_with_key(&SENDER_COMP_ID, sender_comp_id);
            msg.set_fv_with_key(&TARGET_COMP_ID, target_comp_id);
            msg.set_fv_with_key(&SENDING_TIME, field_types::Timestamp::utc_now());
            msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            msg.set_fv_with_key(&ENCRYPT_METHOD, 0);
            msg.set_fv_with_key(&108, heartbeat);
            msg.done()
        };
        output.write(logon).await.unwrap();
        self.backend.on_outbound_message(logon).ok();
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
        self.backend.on_inbound_message(logon, true).ok();
        decoder.clear();
        self.msg_seq_num_inbound.next();
        self.backend.on_successful_handshake().ok();
    }

    async fn event_loop<I, O>(
        &mut self,
        input: I,
        mut output: O,
        decoder: DecoderStreaming<Vec<u8>>,
    ) where
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
                    let response = self.on_inbound_message(msg, unimplemented!());
                    match response {
                        Response::OutboundBytes(bytes) => {
                            output.write_all(bytes).await.unwrap();
                            self.on_outbound_message(bytes).ok();
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
                    self.on_outbound_message(heartbeat).ok();
                }
                LlEvent::Logout => {}
                LlEvent::TestRequest => {}
            }
        }
    }
}

pub trait Verify {
    type Error;

    fn verify_begin_string(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error>;

    fn verify_test_message_indicator(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error>;

    fn verify_sending_time(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error>;
}

struct Verifier<C>
where
    C: Configure,
{
    config: C,
}

/// Basic verifier
impl<C> Verify for Verifier<C>
where
    C: Configure,
{
    type Error = ();

    fn verify_begin_string(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error> {
        if msg.fv(BEGIN_STRING) == self.config.begin_string() {
            Ok(())
        }
        Err(())
    }

    /// Verify whether test message indicator is for the correct environment
    /// If the field is not set than the verification fails
    fn verify_test_message_indicator(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error> {
        if !self.config.verify_test_indicator() {
            return Ok(());
        }
        let env = self.config.environment();
        return match msg.fv_raw(TEST_MESSAGE_INDICATOR) {
            Some(value) => {
                if (value == b'Y')
                    && ((env == Environment::Testing)
                        || (env == Environment::Production { allow_test: true }))
                {
                    return Ok(());
                };
                if (value == b'N') && matches!(env, Environment::Production { .. }) {
                    return Ok(());
                };
                Err(())
            }
            None => Err(()),
        };
    }

    fn verify_sending_time(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error> {
        if let Ok(timestamp) = msg.fv::<field_types::Timestamp>(SENDING_TIME) {
            if let Some(time) = timestamp.to_chrono_utc() {
                let utc_now = chrono::Utc::now();
                if (utc_now - time) < chrono::Duration::seconds(1) {
                    Ok(())
                }
            }
        };
        return Err(());
    }
}

impl<'a, B, C, V> FixConnector<'a, B, C, V> for FixConnection<B, C>
where
    B: Backend,
    C: Configure,
    V: Verify,
{
    type Error = &'a [u8];

    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn verifier(&self) -> V {
        unimplemented!()
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

    fn heartbeat(&self) -> Duration {
        self.config.heartbeat()
    }

    fn encoder(&self) -> &Encoder {
        return &self.encoder;
    }
}

pub struct MessageBuilder {}

pub struct MessageBuiderTuple<'a> {
    phantom: PhantomData<&'a ()>,
}

impl<'a> MessageBuiderTuple<'a> {
    pub fn get(self) -> (EncoderHandle<'a, Vec<u8>>, &'a mut MessageBuilder) {
        unimplemented!()
    }
}

impl MessageBuilder {
    pub fn start_message(&mut self, begin_string: &[u8], msg_type: &[u8]) -> MessageBuiderTuple {
        unimplemented!()
    }
}

pub trait FixConnector<'a, B, C, Z>
where
    B: Backend,
    C: Configure,
    Z: Verify,
{
    type Error: FieldType<'a>;

    fn target_comp_id(&self) -> &[u8];

    fn sender_comp_id(&self) -> &[u8];

    fn verifier(&self) -> &Z;

    fn encoder(&self) -> &Encoder;

    fn dispatch_by_msg_type(&mut self, msg_type: &[u8], msg: Message<&[u8]>) -> Response {
        return match msg_type {
            b"A" => {
                self.on_logon(msg);
                Response::None
            }
            b"1" => {
                let msg = self.on_test_request(msg);
                Response::OutboundBytes(msg)
            }
            b"2" => Response::None,
            b"5" => Response::OutboundBytes(self.on_logout(&msg)),
            b"0" => {
                self.on_heartbeat(msg);
                Response::ResetHeartbeat
            }
            _ => self.on_application_message(msg),
        };
    }

    /// Callback for processing incoming FIX application messages.
    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error>;

    /// Callback for post-processing outbound FIX messages.
    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error>;

    fn environment(&self) -> Environment;

    fn heartbeat(&self) -> Duration;

    fn seq_numbers(&self) -> SeqNumbers;

    fn msg_seq_num(&mut self) -> &mut MsgSeqNumCounter;

    fn on_inbound_message(
        &'a mut self,
        msg: Message<&[u8]>,
        builder: MessageBuilder,
    ) -> Response<'a> {
        if self.verifier().verify_test_message_indicator(&msg).is_err() {
            return self.on_wrong_environment(msg);
        }
        let seq_num = if let Ok(n) = msg.fv::<u64>(&MSG_SEQ_NUM) {
            let expected = self.msg_seq_num_inbound.expected();
            if n < expected {
                return self.on_low_seqnum(msg);
            } else if n > expected {
                // Refer to specs. §4.8 for more information.
                return self.on_high_seqnum(msg);
            }
            n
        } else {
            // See §4.5.3.
            return self.on_missing_seqnum(msg);
        };

        // Increment immediately.
        self.msg_seq_num_inbound.next();

        if self.verifier().verify_sending_time(&msg).is_err() {
            return self.make_reject_for_inaccurate_sending_time(msg);
        }

        let msg_type = if let Ok(x) = msg.fv::<&[u8]>(&MSG_TYPE) {
            x
        } else {
            self.on_inbound_app_message(msg).ok();
            return self.on_application_message(msg);
        };
        self.dispatch_by_msg_type(msg_type, msg)
    }

    fn on_resend_request(&mut self, msg: &Message<&[u8]>) {
        let begin_seq_num = msg.fv(&BEGIN_SEQ_NO).unwrap();
        let end_seq_num = msg.fv(&END_SEQ_NO).unwrap();
        self.make_resend_request(begin_seq_num, end_seq_num).ok();
    }

    fn on_logout(&mut self, input_msg: &Message<&[u8]>) -> &[u8] {
        let (fix_message, _) = {
            let msg_seq_num = self.next();
            let begin_string = input_msg
                .fv_raw(BEGIN_STRING)
                .expect("Message must have a begin string");
            let mut msg = self.encoder().start_message(begin_string, b"5");
            self.set_sender_and_target(&mut msg);
            msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            msg.set_fv_with_key(&TEXT, "Logout");
            msg.done()
        };
        fix_message
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
    fn on_heartbeat_is_due(&mut self) -> &[u8] {
        let fix_message = {
            let begin_string = self.begin_string();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"0");
            self.set_sender_and_target(&mut msg);
            msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            self.set_sending_time(&mut msg);
            msg.done()
        };
        fix_message
    }

    fn set_sender_and_target(&'a self, msg: &mut impl SetField<u32>) {
        msg.set_fv_with_key(&SENDER_COMP_ID, self.sender_comp_id());
        msg.set_fv_with_key(&TARGET_COMP_ID, self.target_comp_id());
    }

    fn set_sending_time(&'a self, msg: &mut impl SetField<u32>) {
        msg.set_fv_with_key(&SENDING_TIME, chrono::Utc::now());
    }

    fn set_header_details(&'a self, _msg: &mut impl SetField<u32>) {}

    fn on_heartbeat(&mut self, _msg: Message<&[u8]>) {
        // TODO: verify stuff.
    }

    fn on_test_request(&mut self, msg: Message<&[u8]>) -> &[u8] {
        let test_req_id = msg.fv::<&[u8]>(&TEST_REQ_ID).unwrap();
        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut msg = self.start_message(begin_string, b"1");
        self.set_sender_and_target(&mut msg);
        msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        self.set_sending_time(&mut msg);
        msg.set_fv_with_key(&TEST_REQ_ID, test_req_id);
        msg.done()
    }

    fn on_wrong_environment(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::production_env())
    }

    fn generate_error_seqnum_too_low(&mut self) -> &[u8] {
        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let text = errs::msg_seq_num(self.msg_seq_num_inbound.0 + 1);
        let mut msg = self.start_message(begin_string, b"FIXME");
        msg.set_fv_with_key(&MSG_TYPE, "5");
        self.set_sender_and_target(&mut msg);
        msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        msg.set_fv_with_key(&TEXT, text.as_str());
        msg.done()
    }

    fn on_missing_seqnum(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::missing_field("MsgSeqNum", MSG_SEQ_NUM))
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
        let begin_string = self.begin_string();
        let sender_comp_id = self.sender_comp_id();
        let target_comp_id = self.target_comp_id();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut msg = self.start_message(begin_string, b"3");
        self.set_sender_and_target(&mut msg);
        msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        if let Some(ref_tag) = ref_tag {
            msg.set_fv_with_key(&REF_TAG_ID, ref_tag);
        }
        if let Some(ref_msg_type) = ref_msg_type {
            msg.set_fv_with_key(&REF_MSG_TYPE, ref_msg_type);
        }
        msg.set_fv_with_key(&SESSION_REJECT_REASON, reason);
        msg.set_fv_with_key(&TEXT, err_text.as_str());
        Response::OutboundBytes(msg.done())
    }

    fn make_reject_for_inaccurate_sending_time(&mut self, offender: Message<&[u8]>) -> Response {
        let ref_seq_num = offender.fv(&MSG_SEQ_NUM).unwrap();
        let ref_msg_type = offender.fv::<&str>(&MSG_TYPE).unwrap();
        self.on_reject(
            ref_seq_num,
            Some(SENDING_TIME),
            Some(ref_msg_type.as_bytes()),
            SENDING_TIME_ACCURACY_PROBLEM,
            "Bad SendingTime".to_string(),
        )
    }

    fn make_logout(&mut self, text: String) -> Response {
        let fix_message = {
            let begin_string = self.begin_string();
            let sender_comp_id = self.sender_comp_id();
            let target_comp_id = self.target_comp_id();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut msg = self.start_message(begin_string, b"5");
            self.set_sender_and_target(&mut msg);
            msg.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            msg.set_fv_with_key(&TEXT, text.as_str());
            self.set_sending_time(&mut msg);
            msg.done()
        };
        Response::OutboundBytes(fix_message)
    }

    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
        let begin_string = self.begin_string();
        let mut msg = self.start_message(begin_string, b"2");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_seqnum(msg);
        msg.set_fv_with_key(&BEGIN_SEQ_NO, start);
        msg.set_fv_with_key(&END_SEQ_NO, end);
        Response::OutboundBytes(msg.done())
    }

    fn on_high_seqnum(&mut self, msg: Message<&[u8]>) -> Response {
        let msg_seq_num = msg.fv(&MSG_SEQ_NUM).unwrap();
        self.make_resend_request(self.seq_numbers().next_inbound(), msg_seq_num);
        todo!()
    }

    fn on_logon(&mut self, _logon: Message<&[u8]>) {
        let begin_string = self.begin_string();
        let mut _msg = self.start_message(begin_string, b"A");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_sending_time(msg);
    }

    fn on_application_message(&mut self, msg: Message<'a, &'a [u8]>) -> Response<'a> {
        Response::Application(msg)
    }
}

//fn add_time_to_msg(mut msg: EncoderHandle) {
//    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
//    let time = chrono::Utc::now();
//    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
//    msg.set_fv_with_key(fix44::SENDING_TIME, timestamp.to_string().as_str());
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
