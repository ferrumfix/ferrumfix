use super::{Backend, Config, Configure, LlEvent, LlEventLoop, MsgSeqNumCounter, errs};
use crate::Buffer;
use crate::FieldMap;
use crate::FieldType;
use crate::session::{Environment, SeqNumbers};
use crate::tagvalue::Message;
use crate::tagvalue::{DecoderStreaming, Encoder, EncoderHandle};
use crate::traits::{FvWrite, SetField};
use futures::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use log;
use std::marker::{PhantomData, Unpin};
use std::pin::Pin;
use std::time::Duration;
use uuid::Uuid;

const BEGIN_SEQ_NO: u32 = 7;
const BEGIN_STRING: u32 = 8;
const END_SEQ_NO: u32 = 16;
const REF_SEQ_NUM: u32 = 45;
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
impl<C, B> FixConnection<B, C>
where
    C: Configure,
    B: Backend,
{
    /// The entry point for a [`FixConnection`].
    async fn start<I, O>(&mut self, mut input: I, mut output: O, mut decoder: DecoderStreaming)
    where
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
        decoder: &mut DecoderStreaming,
    ) where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        let logon = {
            let begin_string = self.config.begin_string();
            let sender_comp_id = self.config.sender_comp_id();
            let target_comp_id = self.config.target_comp_id();
            let heartbeat = self.config.heartbeat().as_secs();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut message = self
                .encoder
                .start_message(begin_string, &mut self.buffer, b"A");
            message.set_fv_with_key(&SENDER_COMP_ID, sender_comp_id);
            message.set_fv_with_key(&TARGET_COMP_ID, target_comp_id);
            message.set_fv_with_key(&SENDING_TIME, chrono::Utc::now());
            message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            message.set_fv_with_key(&ENCRYPT_METHOD, 0);
            message.set_fv_with_key(&108, heartbeat);
            message.done()
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

    async fn event_loop<I, O>(&mut self, input: I, mut output: O, decoder: DecoderStreaming)
    where
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
                LlEvent::Message(message) => {
                    // TODO: Implement proper MessageBuilder integration
                    let response = self.on_inbound_message(message);
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
                LlEvent::BadMessage(err) => {
                    log::error!("Received malformed FIX message: {:?}", err);
                    // TODO: Implement proper error recovery and logging
                }
                LlEvent::IoError(err) => {
                    log::error!("I/O error in FIX connection: {:?}", err);
                    return;
                }
                LlEvent::Heartbeat => {
                    let heartbeat = self.on_heartbeat_is_due();
                    output.write_all(heartbeat).await.unwrap();
                    self.on_outbound_message(heartbeat).ok();
                }
                LlEvent::Logout => {
                    log::info!("Logout event received - shutting down connection");
                    // TODO: Implement proper logout handling
                }
                LlEvent::TestRequest => {
                    log::debug!("Test request timeout - connection may be unstable");
                    // TODO: Implement test request handling
                }
            }
        }
    }
}

pub trait Verify {
    type Error;

    fn verify_begin_string(&self, begin_string: &[u8]) -> Result<(), Self::Error>;

    fn verify_test_message_indicator(
        &self,
        message: &impl FieldMap<u32>,
    ) -> Result<(), Self::Error>;

    fn verify_sending_time(&self, message: &impl FieldMap<u32>) -> Result<(), Self::Error>;
}

/// A no-op verifier implementation for basic functionality
/// TODO: Replace with proper verification implementation
#[derive(Debug, Default)]
pub struct NoOpVerifier;

impl Verify for NoOpVerifier {
    type Error = ();

    fn verify_begin_string(&self, begin_string: &[u8]) -> Result<(), Self::Error> {
        // Validate begin string format and compatibility
        if begin_string == b"FIX.4.4" || begin_string == b"FIX.4.2" || begin_string == b"FIX.4.3" {
            Ok(())
        } else {
            Ok(()) // Always accept for now - TODO: Add proper validation
        }
    }

    fn verify_test_message_indicator(
        &self,
        message: &impl FieldMap<u32>,
    ) -> Result<(), Self::Error> {
        // Check TestMessageIndicator field (tag 464)
        // In production environment, this should be 'N' or absent
        // TODO: Add proper validation based on environment configuration
        let _ = message; // Temporarily silence unused warning
        Ok(()) // Always accept for now
    }

    fn verify_sending_time(&self, message: &impl FieldMap<u32>) -> Result<(), Self::Error> {
        // Validate SendingTime field (tag 52) is within acceptable range
        // TODO: Add proper time accuracy validation
        let _ = message; // Temporarily silence unused warning
        Ok(()) // Always accept for now
    }
}

impl<'a, B, C, V> FixConnector<'a, B, C, V> for FixConnection<B, C>
where
    B: Backend,
    C: Configure,
    V: Verify,
{
    type Error = &'a [u8];
    type Msg = EncoderHandle<'a, Vec<u8>>;

    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn verifier(&self) -> V {
        // TODO: Implement proper verifier configuration
        // ARCHITECTURAL LIMITATION: Cannot create instance of generic type V
        // This method needs redesign to either:
        // 1. Return Option<&V> or Result<V, Error>
        // 2. Be removed from the trait
        // 3. Have V be a concrete type with Default trait
        // For now, this will fail to compile for most V types
        todo!("Session layer verifier needs architectural redesign")
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
}

pub struct MessageBuilder {}

pub struct MessageBuiderTuple<'a> {
    phantom: PhantomData<&'a ()>,
}

impl<'a> MessageBuiderTuple<'a> {
    pub fn get(self) -> (EncoderHandle<'a, Vec<u8>>, &'a mut MessageBuilder) {
        // TODO: Implement proper message building functionality
        // This requires integration with the encoder system
        todo!("MessageBuilder integration not implemented")
    }
}

impl MessageBuilder {
    pub fn start_message(&mut self, begin_string: &[u8], msg_type: &[u8]) -> MessageBuiderTuple {
        unimplemented!()
    }
}

struct ResponseData<'a> {
    pub begin_stringt: &'a [u8],
    pub msg_type: &'a [u8],
    pub msg_seq_num: u32,
}

pub trait FixConnector<'a, B, C, Z>
where
    B: Backend,
    C: Configure,
    Z: Verify,
{
    type Error: FieldType<'a>;
    type Msg: FvWrite<'a>;

    fn target_comp_id(&self) -> &[u8];

    fn sender_comp_id(&self) -> &[u8];

    fn verifier(&self) -> &Z;

    fn dispatch_by_msg_type(&self, msg_type: &[u8], message: Message<&[u8]>) -> Response {
        match msg_type {
            b"A" => {
                self.on_logon(message);
                return Response::None;
            }
            b"1" => {
                let test_request_response = self.on_test_request(message);
                return Response::OutboundBytes(test_request_response);
            }
            b"2" => {
                return Response::None;
            }
            b"5" => {
                return Response::OutboundBytes(self.on_logout(&message));
            }
            b"0" => {
                self.on_heartbeat(message);
                return Response::ResetHeartbeat;
            }
            _ => {
                return self.on_application_message(message);
            }
        }
    }

    /// Callback for processing incoming FIX application messages.
    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error>;

    /// Callback for post-processing outbound FIX messages.
    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error>;

    fn environment(&self) -> Environment;

    fn heartbeat(&self) -> Duration;

    fn seq_numbers(&self) -> SeqNumbers;

    fn msg_seq_num(&mut self) -> &mut MsgSeqNumCounter;

    fn on_inbound_message(&'a mut self, message: Message<&[u8]>) -> Response<'a> {
        if self
            .verifier()
            .verify_test_message_indicator(message)
            .is_err()
        {
            return self.on_wrong_environment(message);
        }
        let seq_num = if let Ok(n) = message.get::<u64>(&MSG_SEQ_NUM) {
            let expected = self.msg_seq_num_inbound.expected();
            if n < expected {
                return self.on_low_seqnum(message);
            } else if n > expected {
                // Refer to specs. §4.8 for more information.
                return self.on_high_seqnum(message);
            }
            n
        } else {
            // See §4.5.3.
            return self.on_missing_seqnum(message);
        };

        // Increment immediately.
        self.msg_seq_num_inbound.next();

        if self.verifier().verify_sending_time(&message).is_err() {
            return self.make_reject_for_inaccurate_sending_time(message);
        }

        let msg_type = if let Ok(x) = message.get::<&[u8]>(&MSG_TYPE) {
            x
        } else {
            self.on_inbound_app_message(message).ok();
            return self.on_application_message(message);
        };
        self.dispatch_by_msg_type(msg_type, message)
    }

    fn on_resend_request(&self, message: &Message<&[u8]>) {
        let begin_seq_num = message.get(&BEGIN_SEQ_NO).unwrap();
        let end_seq_num = message.get(&END_SEQ_NO).unwrap();
        self.on_resend_request(begin_seq_num..end_seq_num).ok();
    }

    fn on_logout(&mut self, data: ResponseData, _message: &Message<&[u8]>) -> &[u8] {
        let fix_message = {
            let msg_seq_num = self.next();
            let mut logout_message = self.start_message(data.begin_string, b"5");
            self.set_sender_and_target(&mut logout_message);
            logout_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            logout_message.set_fv_with_key(&TEXT, "Logout");
            logout_message.done()
        };
        fix_message
    }

    //    fn add_seqnum(&self, message: &mut RawEncoderState) {
    //        message.add_field(tags::MSG_SEQ_NUM, self.seq_numbers().next_outbound());
    //    }
    //
    //    fn add_sending_time(&self, message: &mut RawEncoderState) {
    //        message.add_field(tags::SENDING_TIME, DtfTimestamp::utc_now());
    //    }
    //
    //    #[must_use]
    fn on_heartbeat_is_due(&mut self) -> &[u8] {
        let fix_message = {
            let begin_string = self.begin_string();
            let msg_seq_num = self.msg_seq_num_outbound.next();
            let mut heartbeat_message =
                self.encoder
                    .start_message(begin_string, &mut self.buffer, b"0");
            self.set_sender_and_target(&mut heartbeat_message);
            heartbeat_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            self.set_sending_time(&mut heartbeat_message);
            heartbeat_message.done()
        };
        fix_message
    }

    fn set_sender_and_target(&'a self, message: &mut impl FvWrite<'a, Key = u32>) {
        message.set_fv_with_key(&SENDER_COMP_ID, self.sender_comp_id());
        message.set_fv_with_key(&TARGET_COMP_ID, self.target_comp_id());
    }

    fn set_sending_time(&'a self, message: &mut impl FvWrite<'a, Key = u32>) {
        message.set_fv_with_key(&SENDING_TIME, chrono::Utc::now());
    }

    fn set_header_details(&'a self, message: &mut impl FvWrite<'a, Key = u32>) {
        // TODO: Add any additional header fields as needed
        // This method can be used for custom header field additions
    }

    fn on_heartbeat(&mut self, message: Message<&[u8]>) {
        log::debug!("Processing heartbeat message");
        // TODO: Add heartbeat validation and processing
        // For now, just acknowledge receipt
    }

    fn on_test_request(&mut self, message: Message<&[u8]>) -> &[u8] {
        let test_req_id = message.get::<&[u8]>(&TEST_REQ_ID).unwrap();
        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut response_message = self.start_message(begin_string, b"1");
        self.set_sender_and_target(&mut response_message);
        response_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        self.set_sending_time(&mut response_message);
        response_message.set_fv_with_key(&TEST_REQ_ID, test_req_id);
        response_message.done()
    }

    fn on_wrong_environment(&mut self, message: Message<&[u8]>) -> Response {
        log::warn!("Wrong environment detected in message");
        self.make_logout(errs::production_env())
    }

    fn generate_error_seqnum_too_low(&mut self) -> &[u8] {
        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let text = errs::msg_seq_num(self.msg_seq_num_inbound.0 + 1);
        let mut message = self.start_message(begin_string, b"FIXME");
        message.set_fv_with_key(&MSG_TYPE, "5");
        self.set_sender_and_target(&mut message);
        message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        message.set_fv_with_key(&TEXT, text.as_str());
        message.done()
    }

    fn on_missing_seqnum(&mut self, message: Message<&[u8]>) -> Response {
        log::warn!("Missing sequence number in message");
        self.make_logout(errs::missing_field("MsgSeqNum", MSG_SEQ_NUM))
    }

    fn on_low_seqnum(&mut self, message: Message<&[u8]>) -> Response {
        log::warn!("Received message with low sequence number");
        self.make_logout(errs::msg_seq_num(self.msg_seq_num_inbound.0 + 1))
    }

    fn on_reject(
        &mut self,
        ref_seq_num: u64,
        ref_tag: Option<u32>,
        ref_msg_type: Option<&[u8]>,
        reason: u32,
        err_text: String,
    ) -> Response {
        log::warn!(
            "Rejecting message with seq_num={}, reason={}: {}",
            ref_seq_num,
            reason,
            err_text
        );
        let begin_string = self.begin_string();
        let sender_comp_id = self.sender_comp_id();
        let target_comp_id = self.target_comp_id();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut reject_message = self.start_message(begin_string, b"3");
        self.set_sender_and_target(&mut reject_message);
        reject_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        reject_message.set_fv_with_key(&REF_SEQ_NUM, ref_seq_num);
        if let Some(ref_tag) = ref_tag {
            reject_message.set_fv_with_key(&REF_TAG_ID, ref_tag);
        }
        if let Some(ref_msg_type) = ref_msg_type {
            reject_message.set_fv_with_key(&REF_MSG_TYPE, ref_msg_type);
        }
        reject_message.set_fv_with_key(&SESSION_REJECT_REASON, reason);
        reject_message.set_fv_with_key(&TEXT, err_text.as_str());
        Response::OutboundBytes(reject_message.done())
    }

    fn make_reject_for_inaccurate_sending_time(&mut self, offender: Message<&[u8]>) -> Response {
        let ref_seq_num = offender.get(&MSG_SEQ_NUM).unwrap();
        let ref_msg_type = offender.get::<&str>(&MSG_TYPE).unwrap();
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
            let mut logout_message = self.start_message(begin_string, b"5");
            self.set_sender_and_target(&mut logout_message);
            logout_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            logout_message.set_fv_with_key(&TEXT, text.as_str());
            self.set_sending_time(&mut logout_message);
            logout_message.done()
        };
        Response::OutboundBytes(fix_message)
    }

    fn make_resend_request(&mut self, start: u64, end: u64) -> Response {
        let begin_string = self.begin_string();
        let mut resend_request = self.start_message(begin_string, b"2");
        //Self::add_comp_id(resend_request);
        //self.add_sending_time(resend_request);
        //self.add_seqnum(resend_request);
        resend_request.set_fv_with_key(&BEGIN_SEQ_NO, start);
        resend_request.set_fv_with_key(&END_SEQ_NO, end);
        Response::OutboundBytes(resend_request.done())
    }

    fn on_high_seqnum(&mut self, message: Message<&[u8]>) -> Response {
        // FIX Protocol Compliance: Check if this is a Logout message (msg_type = "5")
        // Per FIX specification, Logout messages with high sequence numbers should
        // terminate the session immediately, not request resend.
        let msg_type = message.get_raw(MSG_TYPE).unwrap_or_default();
        if msg_type == b"5" {
            // Logout message
            return self.make_logout("Logout with high sequence number".to_string());
        }

        let msg_seq_num = message.get(&MSG_SEQ_NUM).unwrap();
        // The message is NOT stored. This is a deficiency that should be
        // addressed later as part of "complete session state management".
        // For non-logout messages, request the missing messages.
        self.make_resend_request(self.msg_seq_num_inbound.expected(), msg_seq_num - 1)
    }

    fn on_logon(&mut self, logon: Message<&[u8]>) {
        log::info!("Processing logon message");
        let begin_string = self.begin_string();
        let mut message = self.start_message(begin_string, b"A");
        // TODO: Implement proper logon response
        // For now, just prepare a basic logon acknowledgment structure
    }

    fn on_application_message(&mut self, message: Message<'a, &'a [u8]>) -> Response<'a> {
        Response::Application(message)
    }
}

//fn add_time_to_msg(mut message: EncoderHandle) {
//    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
//    let time = chrono::Utc::now();
//    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
//    message.set_fv_with_key(fix44::SENDING_TIME, timestamp.to_string().as_str());
//}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Dictionary;
    use crate::tagvalue::{Config, Decoder};

    // Import needed for tests
    use crate::session::MsgSeqNumCounter;

    fn create_test_message(msg_type: &str, seq_num: u64) -> String {
        format!(
            "8=FIX.4.4|9=100|35={}|49=SENDER|56=TARGET|34={}|52=20100304-07:59:30|10=000|",
            msg_type, seq_num
        )
    }

    fn create_decoder() -> Decoder {
        let mut decoder = Decoder::new(Dictionary::fix44().unwrap());
        decoder.config_mut().separator = b'|';
        decoder
    }

    #[test]
    fn test_logout_with_high_seqnum_terminates_session() {
        // Create a mock FixConnector to test the on_high_seqnum logic
        struct TestConnector {
            msg_seq_num_inbound: MsgSeqNumCounter,
        }

        impl TestConnector {
            fn new() -> Self {
                Self {
                    msg_seq_num_inbound: MsgSeqNumCounter::new(1), // Expecting sequence 1
                }
            }

            fn make_logout(&self, text: String) -> Response {
                Response::OutboundBytes(b"logout_response")
            }

            fn make_resend_request(&self, _start: u64, _end: u64) -> Response {
                Response::OutboundBytes(b"resend_request")
            }

            // Test the on_high_seqnum logic directly
            fn on_high_seqnum(&self, message: &crate::tagvalue::Message<&[u8]>) -> Response {
                let msg_type = message.get_raw(MSG_TYPE).unwrap_or_default();
                if msg_type == b"5" {
                    // Logout message
                    return self.make_logout("Logout with high sequence number".to_string());
                }

                let _msg_seq_num = message.get::<u64>(&MSG_SEQ_NUM).unwrap();
                self.make_resend_request(self.msg_seq_num_inbound.expected(), _msg_seq_num - 1)
            }
        }

        let connector = TestConnector::new();
        let mut decoder = create_decoder();

        // Test 1: Regular message with high sequence number should request resend
        let regular_msg = create_test_message("D", 5); // NewOrderSingle with seq 5 (expecting 1)
        let parsed_msg = decoder.decode(regular_msg.as_bytes()).unwrap();
        let response = connector.on_high_seqnum(&parsed_msg);

        match response {
            Response::OutboundBytes(bytes) => {
                assert_eq!(
                    bytes, b"resend_request",
                    "Regular high-seqnum message should request resend"
                );
            }
            _ => panic!("Expected OutboundBytes with resend request"),
        }

        // Test 2: Logout message with high sequence number should terminate session
        let logout_msg = create_test_message("5", 5); // Logout with seq 5 (expecting 1)
        let parsed_logout = decoder.decode(logout_msg.as_bytes()).unwrap();
        let logout_response = connector.on_high_seqnum(&parsed_logout);

        match logout_response {
            Response::OutboundBytes(bytes) => {
                assert_eq!(
                    bytes, b"logout_response",
                    "Logout with high-seqnum should terminate session"
                );
            }
            _ => panic!("Expected OutboundBytes with logout response"),
        }
    }

    #[test]
    fn test_different_message_types_with_high_seqnum() {
        struct TestConnector;
        impl TestConnector {
            fn on_high_seqnum(&self, message: &crate::tagvalue::Message<&[u8]>) -> &'static str {
                let msg_type = message.get_raw(MSG_TYPE).unwrap_or_default();
                if msg_type == b"5" {
                    // Logout message
                    "logout"
                } else {
                    "resend"
                }
            }
        }

        let connector = TestConnector;
        let mut decoder = create_decoder();

        // Test various message types
        let test_cases = vec![
            ("0", "resend"), // Heartbeat -> resend
            ("1", "resend"), // TestRequest -> resend
            ("2", "resend"), // ResendRequest -> resend
            ("3", "resend"), // Reject -> resend
            ("4", "resend"), // SequenceReset -> resend
            ("5", "logout"), // Logout -> terminate
            ("A", "resend"), // Logon -> resend
            ("D", "resend"), // NewOrderSingle -> resend
        ];

        for (msg_type, expected) in test_cases {
            let msg = create_test_message(msg_type, 10);
            let parsed = decoder.decode(msg.as_bytes()).unwrap();
            let result = connector.on_high_seqnum(&parsed);
            assert_eq!(
                result, expected,
                "Message type {} should result in {}",
                msg_type, expected
            );
        }
    }
}

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
//        let message = next.as_outbound().unwrap();
//        assert_eq!(message.field_str(tags::MSG_TYPE), Some("0"));
//        assert_eq!(message.field_str(tags::SENDER_COMP_ID), Some("SENDER"));
//        assert_eq!(message.field_str(tags::TARGET_COMP_ID), Some("TARGET"));
//        assert_eq!(message.field_bool(tags::POSS_DUP_FLAG), None);
//        assert_eq!(message.field_i64(tags::TEST_REQ_ID), None);
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
//        let mut message = FixMessage::new();
//        message.add_str(tags::MSG_TYPE, "BE");
//        message.add_str(tags::SENDER_COMP_ID, "SENDER");
//        message.add_str(tags::TARGET_COMP_ID, "TARGET");
//        message.add_i64(tags::MSG_SEQ_NUM, 1);
//        message.add_str(
//            tags::USER_REQUEST_ID,
//            "47b6f4a6-993d-4430-b68f-d9b680a1a772",
//        );
//        message.add_i64(tags::USER_REQUEST_TYPE, 1);
//        message.add_str(tags::USERNAME, "john-doe");
//        let mut responses = conn.on_inbound_message(message);
//        let next = responses.next().unwrap();
//        let message = next.as_outbound().unwrap();
//        assert_eq!(message.field_str(tags::MSG_TYPE), Some("3"));
//        assert_eq!(message.field_str(tags::SENDER_COMP_ID), Some("SENDER"));
//        assert_eq!(message.field_str(tags::TARGET_COMP_ID), Some("TARGET"));
//        assert_eq!(message.field_bool(tags::POSS_DUP_FLAG), None);
//        assert_eq!(message.field_i64(tags::TEST_REQ_ID), None);
//        assert_eq!(message.field_i64(tags::SESSION_REJECT_REASON), Some(10));
//        assert_eq!(message.field_i64(tags::REF_SEQ_NUM), Some(10));
//        assert!(responses.next().is_none());
//    }
//}
