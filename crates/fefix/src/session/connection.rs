use super::{errs, Backend, Config, Configure, LlEvent, LlEventLoop};
use crate::field_types::Timestamp;
use crate::session::{
    Environment, FixConnectionError, MsgSeqNumCounter, SeqNumberError, SeqNumbers,
};
use crate::tagvalue::Message;
use crate::tagvalue::{
    Config as TagConfig, Configure as TagConfigure, DecoderStreaming, Encoder, EncoderHandle,
};
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
const HEART_BT_INT: u32 = 108;
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
pub struct FixConnection<B, C = Config, V = Verifier<C>, TC = TagConfig> {
    uuid: Uuid,
    config: C,
    backend: B,
    verifier: V,
    encoder: Encoder<TC>,
    buffer: Vec<u8>,
    seq_numbers: SeqNumbers,
}

#[allow(dead_code)]
impl<B, C, V, TC> FixConnection<B, C, V, TC>
where
    B: Backend,
    C: Configure,
    V: Verify,
    TC: TagConfigure,
{
    /// Create a new FIX connection
    pub fn new(
        backend: B,
        config: C,
        verifier: V,
        encoder: Encoder<TC>,
        seq_numbers: Option<SeqNumbers>,
    ) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            backend,
            config,
            encoder,
            verifier,
            buffer: vec![],
            seq_numbers: seq_numbers.unwrap_or(SeqNumbers::default()),
        }
    }

    /// The entry point for a [`FixConnection`].
    pub async fn start<I, O>(
        &mut self,
        mut input: I,
        mut output: O,
        mut decoder: DecoderStreaming<Vec<u8>>,
    ) -> Result<(), FixConnectionError>
    where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        self.establish_connection(&mut input, &mut output, &mut decoder)
            .await?;
        self.event_loop(input, output, decoder).await
    }

    async fn establish_connection<I, O>(
        &mut self,
        mut input: &mut I,
        mut output: &mut O,
        mut decoder: &mut DecoderStreaming<Vec<u8>>,
    ) -> Result<(), FixConnectionError>
    where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        let (logon, _) = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let heartbeat = self.config.heartbeat().as_secs();
            let msg_seq_num = self.seq_numbers.get_incr_outbound();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"A");
            msg.set(SENDER_COMP_ID, sender_comp_id);
            msg.set(TARGET_COMP_ID, target_comp_id);
            msg.set(SENDING_TIME, field_types::Timestamp::utc_now());
            msg.set(MSG_SEQ_NUM, msg_seq_num);
            msg.set(ENCRYPT_METHOD, 0);
            msg.set(HEART_BT_INT, heartbeat);
            msg.done()
        };
        output.write(logon).await?;
        self.backend.on_outbound_message(logon).ok();
        let logon;
        loop {
            let mut input = Pin::new(&mut input);
            let buf = decoder.fillable();
            let len = buf.len();
            input.read_exact(buf).await?;
            decoder.add_bytes_read(len);
            if let Ok(Some(())) = decoder.try_parse() {
                logon = decoder.message();
                break;
            }
        }
        self.on_logon(logon);
        self.backend.on_inbound_message(logon, false).ok();
        decoder.clear();
        self.seq_numbers.incr_inbound();
        self.backend.on_successful_handshake().ok();
        Ok(())
    }

    async fn event_loop<I, O>(
        &mut self,
        input: I,
        mut output: O,
        mut decoder: DecoderStreaming<Vec<u8>>,
    ) -> Result<(), FixConnectionError>
    where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        let mut backend = (&self.backend).clone();
        let event_loop = &mut LlEventLoop::new(decoder, input, self.heartbeat());
        loop {
            let event = event_loop
                .next_event()
                .await
                .ok_or_else(|| FixConnectionError::NotConnected)?;
            match event {
                LlEvent::Message(msg) => {
                    let response = self.on_inbound_message(msg);
                    match response {
                        Response::OutboundBytes(bytes) => {
                            output.write_all(bytes).await?;
                            backend.on_outbound_message(bytes).ok();
                        }
                        Response::ResetHeartbeat => {
                            event_loop.ping_heartbeat();
                        }
                        _ => {}
                    }
                }
                LlEvent::BadMessage(_err) => {}
                LlEvent::IoError(err) => {
                    return Err(FixConnectionError::IoError { source: err });
                }
                LlEvent::Heartbeat => {
                    dbglog!("Sending heartbeat");
                    let heartbeat = self.on_heartbeat_is_due();
                    backend.on_outbound_message(heartbeat).ok();
                    output.write_all(heartbeat).await?;
                }
                LlEvent::Logout => {}
                LlEvent::TestRequest => {}
            }
        }
    }

    fn on_inbound_app_message<'a>(
        &mut self,
        message: Message<&'a [u8]>,
    ) -> Result<(), FixConnectionError> {
        dbg!("Calling backend");
        self.backend
            .on_inbound_app_message(message)
            .map_err(|_| FixConnectionError::BackendProcessingError)
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), FixConnectionError> {
        self.backend
            .on_outbound_message(message)
            .map_err(|_| FixConnectionError::BackendProcessingError)
    }

    fn verifier(&self) -> &V {
        &self.verifier
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

    fn dispatch_by_msg_type<'a>(
        &'a mut self,
        msg_type: &[u8],
        msg: Message<'a, &'a [u8]>,
    ) -> Response<'a> {
        dbglog!("Dispatching");
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
            b"5" => Response::OutboundBytes(self.on_logout(msg)),
            b"0" => {
                self.on_heartbeat(msg);
                Response::ResetHeartbeat
            }
            _ => self.on_application_message(msg),
        };
    }

    fn on_inbound_message<'a>(&'a mut self, msg: Message<'a, &'a [u8]>) -> Response<'a> {
        dbglog!("Got message");
        if self.verifier().verify_test_message_indicator(&msg).is_err() {
            return self.on_wrong_environment(msg);
        }
        let seq_num = if let Ok(n) = msg.fv::<u64>(MSG_SEQ_NUM) {
            match self.seq_numbers.validate_inbound(n) {
                Ok(_) => {}
                Err(err) => {
                    match err {
                        SeqNumberError::Recover => {
                            // Refer to specs. §4.8 for more information.
                            return self.on_high_seqnum(msg);
                        }
                        SeqNumberError::TooLow => {
                            return self.on_low_seqnum(msg);
                        }
                        SeqNumberError::NoSeqNum => {
                            panic!("Not possible")
                        }
                    }
                }
            }
            n
        } else {
            // See §4.5.3.
            return self.on_missing_seqnum(msg);
        };

        // Increment immediately.
        self.seq_numbers.get_incr_inbound();

        if self.verifier.verify_sending_time(&msg).is_err() {
            return self.make_reject_for_inaccurate_sending_time(msg);
        }
        dbglog!("Sending time verified");

        let msg_type = if let Ok(x) = msg.fv::<&[u8]>(MSG_TYPE) {
            x
        } else {
            return self.on_application_message(msg);
        };
        self.dispatch_by_msg_type(msg_type, msg)
    }

    // TODO
    // fn on_resend_request(&mut self, msg: &Message<&[u8]>) {
    //     let begin_seq_num = msg.fv(BEGIN_SEQ_NO).unwrap();
    //     let end_seq_num = msg.fv(END_SEQ_NO).unwrap();
    //     self.make_resend_request(begin_seq_num, end_seq_num).ok();
    // }

    fn on_logout(&mut self, input_msg: Message<&[u8]>) -> &[u8] {
        self.backend
            .on_inbound_message(input_msg, false)
            .unwrap_or_else(|err| dbglog!("Error on logout: {:?}", err));
        let (fix_message, _) = {
            let msg_seq_num = self.seq_numbers.get_incr_outbound();
            let begin_string = self.config.begin_string();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"5");
            // TODO self.set_sender_and_target(&mut msg);
            msg.set(SENDER_COMP_ID, self.config.sender_comp_id());
            msg.set(TARGET_COMP_ID, self.config.target_comp_id());
            msg.set(MSG_SEQ_NUM, msg_seq_num);
            msg.set(TEXT, "Logout");
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
            let begin_string = self.config.begin_string();
            let msg_seq_num = self.seq_numbers.get_incr_outbound();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"0");
            Self::set_sender_and_target(&mut msg, &self.config);
            msg.set(MSG_SEQ_NUM, msg_seq_num);
            Self::set_sending_time(&mut msg);
            msg.done()
        };
        fix_message.0
    }

    fn set_sender_and_target(msg: &mut impl SetField<u32>, config: &C) {
        msg.set(SENDER_COMP_ID, config.sender_comp_id());
        msg.set(TARGET_COMP_ID, config.target_comp_id());
    }

    fn set_sending_time(msg: &mut impl SetField<u32>) {
        msg.set(SENDING_TIME, Timestamp::utc_now());
    }

    fn set_header_details(&self, _msg: &mut impl SetField<u32>) {}

    fn on_heartbeat(&mut self, msg: Message<&[u8]>) {
        self.backend
            .on_inbound_message(msg, false)
            .unwrap_or_else(|err| dbglog!("Error on heartbeat: {:?}", err));
    }

    fn on_test_request<'a>(&'a mut self, msg: Message<&[u8]>) -> &'a [u8] {
        self.backend
            .on_inbound_message(msg, false)
            .unwrap_or_else(|err| dbglog!("Error on test request: {:?}", err));
        let test_req_id = msg.fv::<&[u8]>(TEST_REQ_ID).unwrap();
        let begin_string = self.config.begin_string();
        let msg_seq_num = self.seq_numbers.get_incr_outbound();
        let mut msg = self
            .encoder
            .start_message(begin_string, &mut self.buffer, b"1");
        Self::set_sender_and_target(&mut msg, &self.config);
        msg.set(MSG_SEQ_NUM, msg_seq_num);
        Self::set_sending_time(&mut msg);
        msg.set(TEST_REQ_ID, test_req_id);
        msg.done().0
    }

    fn on_wrong_environment(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::production_env())
    }

    fn generate_error_seqnum_too_low(&mut self) -> &[u8] {
        let begin_string = self.config.begin_string();
        let msg_seq_num = self.seq_numbers.get_incr_outbound();
        let text = errs::msg_seq_num(self.seq_numbers.next_inbound() + 1);
        let mut msg = self
            .encoder
            .start_message(begin_string, &mut self.buffer, b"FIXME");
        msg.set(MSG_TYPE, "5");
        Self::set_sender_and_target(&mut msg, &self.config);
        msg.set(MSG_SEQ_NUM, msg_seq_num);
        msg.set(TEXT, text.as_str());
        msg.done().0
    }

    fn on_missing_seqnum(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::missing_field("MsgSeqNum", MSG_SEQ_NUM))
    }

    fn on_low_seqnum(&mut self, _message: Message<&[u8]>) -> Response {
        self.make_logout(errs::msg_seq_num(self.seq_numbers.next_inbound() + 1))
    }

    fn on_reject(
        &mut self,
        _ref_seq_num: u64,
        ref_tag: Option<u32>,
        ref_msg_type: Option<&[u8]>,
        reason: u32,
        err_text: String,
    ) -> Response {
        let begin_string = self.config.begin_string();
        let msg_seq_num = self.seq_numbers.get_incr_outbound();
        let mut msg = self
            .encoder
            .start_message(begin_string, &mut self.buffer, b"3");
        Self::set_sender_and_target(&mut msg, &self.config);
        msg.set(MSG_SEQ_NUM, msg_seq_num);
        if let Some(ref_tag) = ref_tag {
            msg.set(REF_TAG_ID, ref_tag);
        }
        if let Some(ref_msg_type) = ref_msg_type {
            msg.set(REF_MSG_TYPE, ref_msg_type);
        }
        msg.set(SESSION_REJECT_REASON, reason);
        msg.set(TEXT, err_text.as_str());
        Response::OutboundBytes(msg.done().0)
    }

    fn make_reject_for_inaccurate_sending_time<'a>(
        &'a mut self,
        offender: Message<&'a [u8]>,
    ) -> Response<'a> {
        let ref_seq_num = offender.fv(MSG_SEQ_NUM).unwrap();
        let ref_msg_type = offender.fv::<&str>(MSG_TYPE).unwrap();
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
            let begin_string = self.config.begin_string();
            let msg_seq_num = self.seq_numbers.get_incr_outbound();
            let mut msg = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"5");
            Self::set_sender_and_target(&mut msg, &self.config);
            msg.set(MSG_SEQ_NUM, msg_seq_num);
            msg.set(TEXT, text.as_str());
            msg.set(SENDING_TIME, Timestamp::utc_now());
            msg.done()
        };
        Response::OutboundBytes(fix_message.0)
    }

    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
        let begin_string = self.config.begin_string();
        let mut msg = self
            .encoder
            .start_message(begin_string, &mut self.buffer, b"2");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_seqnum(msg);
        msg.set(BEGIN_SEQ_NO, start);
        msg.set(END_SEQ_NO, end);
        Response::OutboundBytes(msg.done().0)
    }

    fn on_high_seqnum(&mut self, msg: Message<&[u8]>) -> Response {
        let msg_seq_num = msg.fv(MSG_SEQ_NUM).unwrap();
        let actual_seq_num = self.seq_numbers.get_incr_inbound();
        return self.make_resend_request(actual_seq_num, msg_seq_num);
    }

    fn on_logon(&mut self, logon: Message<&[u8]>) {
        self.backend
            .on_inbound_message(logon, false)
            .unwrap_or_else(|err| dbglog!("Error on logon: {:?}", err));
        let mut _msg =
            self.encoder
                .start_message(self.config.begin_string(), &mut self.buffer, b"A");
        //Self::add_comp_id(msg);
        //self.add_sending_time(msg);
        //self.add_sending_time(msg);
    }

    fn on_application_message<'a>(&'a mut self, msg: Message<'a, &'a [u8]>) -> Response<'a> {
        dbg!("Got an app message");
        self.on_inbound_app_message(msg).ok();
        Response::Application(msg)
    }
}

pub trait Verify {
    type Error;

    fn verify_begin_string(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error>;

    fn verify_test_message_indicator(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error>;

    fn verify_sending_time(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Verifier<C>
where
    C: Configure,
{
    config: C,
}

impl<C> Verifier<C>
where
    C: Configure,
{
    pub fn new(config: C) -> Self {
        Self { config }
    }
}

/// Basic verifier
impl<C> Verify for Verifier<C>
where
    C: Configure,
{
    type Error = ();

    fn verify_begin_string(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error> {
        if msg.fv(BEGIN_STRING) == Ok(self.config.begin_string()) {
            return Ok(());
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
                if (value == b"Y")
                    && ((env == Environment::Testing)
                        || (env == Environment::Production { allow_test: true }))
                {
                    return Ok(());
                };
                if (value == b"N") && matches!(env, Environment::Production { .. }) {
                    return Ok(());
                };
                Err(())
            }
            None => Ok(()),
        };
    }

    fn verify_sending_time(&self, msg: &impl FieldMap<u32>) -> Result<(), Self::Error> {
        if let Ok(timestamp) = msg.fv::<field_types::Timestamp>(SENDING_TIME) {
            if let Some(time) = timestamp.to_chrono_utc() {
                let utc_now = chrono::Utc::now();
                if (utc_now - time) < chrono::Duration::seconds(1) {
                    return Ok(());
                }
            }
        };
        return Err(());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::Decoder;
    use crate::{Dictionary, GetConfig};
    use futures::SinkExt;
    use std::borrow::BorrowMut;
    use std::ops::Range;
    use std::time::Duration;

    #[derive(Clone)]
    struct TestBackend {
        sender: futures::channel::mpsc::Sender<Vec<u8>>,
    }

    impl Backend for TestBackend {
        type Error = FixConnectionError;

        fn sender_comp_id(&self) -> &[u8] {
            b"SENDER"
        }

        fn target_comp_id(&self) -> &[u8] {
            b"TARGET"
        }

        fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error> {
            self.on_inbound_message(message, true)
        }

        fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }

        fn on_inbound_message(
            &mut self,
            message: Message<&[u8]>,
            _is_app: bool,
        ) -> Result<(), Self::Error> {
            Ok(self.sender.try_send(message.as_bytes().to_vec()).unwrap())
        }

        fn on_resend_request(&mut self, _range: Range<u64>) -> Result<(), Self::Error> {
            Ok(())
        }

        fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
            Ok(self.sender.try_send(b"hand shook".to_bytes()).unwrap())
        }

        fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error> {
            Ok(&[b""])
        }

        fn pending_message(&mut self) -> Option<&[u8]> {
            None
        }
    }

    fn conn() -> (
        FixConnection<TestBackend>,
        futures::channel::mpsc::Receiver<Vec<u8>>,
    ) {
        let (sender, receiver) = futures::channel::mpsc::channel::<Vec<u8>>(10);
        let mut config = Config::default();
        config.sender_comp_id = "SENDER".to_string();
        config.target_comp_id = "TARGET".to_string();
        let mut encoder = Encoder::<TagConfig>::new();
        let fix_connection = FixConnection::<TestBackend>::new(
            TestBackend { sender },
            config.clone(),
            Verifier::<Config>::new(config),
            encoder,
            None, // TODO seq numbers
        );

        return (fix_connection, receiver);
    }

    #[test]
    fn test_on_heartbeat_is_due() {
        let conn = &mut conn().0;
        let response = conn.on_heartbeat_is_due();
        let mut decoder = Decoder::<TagConfig>::new(Dictionary::fix44());
        let msg = decoder.decode(response).unwrap();
        assert_eq!(msg.fv::<&str>(MSG_TYPE).unwrap(), "0");
        assert_eq!(msg.fv::<&str>(SENDER_COMP_ID).unwrap(), "SENDER");
        assert_eq!(msg.fv::<&str>(TARGET_COMP_ID).unwrap(), "TARGET");
        assert_eq!(msg.fv::<u64>(MSG_SEQ_NUM).unwrap(), 1);
    }

    /// Test a rejection is returned on missing sending time
    #[test]
    fn test_inaccurate_sending_time() {
        let conn = &mut conn().0;
        let mut encoder = Encoder::<TagConfig>::new();
        let mut buffer = Vec::<u8>::new();
        let mut input_msg = encoder.start_message(b"FIX.4.4", &mut buffer, b"BE");
        input_msg.set(SENDER_COMP_ID, "SENDER");
        input_msg.set(TARGET_COMP_ID, "TARGET");
        input_msg.set(MSG_SEQ_NUM, 1);
        let input_bytes = input_msg.done().0;

        let mut input_decoder = Decoder::<TagConfig>::new(Dictionary::fix44());
        let response = conn.on_inbound_message(input_decoder.decode(input_bytes).unwrap());

        let response_bytes = match response {
            Response::OutboundBytes(msg_bytes) => msg_bytes,
            _ => {
                panic!("Expected outbound bytes");
            }
        };
        let mut output_decoder = Decoder::<TagConfig>::new(Dictionary::fix44());
        let response_msg = output_decoder.decode(response_bytes).unwrap();
        assert_eq!(response_msg.fv::<&str>(MSG_TYPE).unwrap(), "3");
        assert_eq!(response_msg.fv::<&str>(SENDER_COMP_ID).unwrap(), "SENDER");
        assert_eq!(response_msg.fv::<&str>(TARGET_COMP_ID).unwrap(), "TARGET");
        assert_eq!(response_msg.fv::<u64>(MSG_SEQ_NUM).unwrap(), 1);
        assert_eq!(response_msg.fv::<u32>(REF_TAG_ID).unwrap(), SENDING_TIME);
        assert_eq!(response_msg.fv::<&str>(REF_MSG_TYPE).unwrap(), "BE");
        assert_eq!(
            response_msg.fv::<u32>(SESSION_REJECT_REASON).unwrap(),
            SENDING_TIME_ACCURACY_PROBLEM
        );
        assert_eq!(response_msg.fv::<&str>(TEXT).unwrap(), "Bad SendingTime");
        assert_eq!(response_msg.fv_opt::<&str>(TEST_REQ_ID).unwrap(), None);
    }
}
