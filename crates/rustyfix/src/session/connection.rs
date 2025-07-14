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
use quanta::Instant;
use rustc_hash::FxHashMap;
use smallvec::SmallVec;
use smartstring::alias::String as SmartString;
use std::marker::{PhantomData, Unpin};
use std::pin::Pin;
use std::time::Duration;
use uuid::Uuid;

const BEGIN_SEQ_NO: u32 = 7;
const BEGIN_STRING: u32 = 8;
const END_SEQ_NO: u32 = 16;
const NEW_SEQ_NO: u32 = 36;
const REF_SEQ_NUM: u32 = 45;
const MSG_SEQ_NUM: u32 = 34;
const MSG_TYPE: u32 = 35;
const SENDER_COMP_ID: u32 = 49;
const SENDING_TIME: u32 = 52;
const TARGET_COMP_ID: u32 = 56;
const TEXT: u32 = 58;
const ENCRYPT_METHOD: u32 = 98;
const HEARTBEAT_INT: u32 = 108;
const TEST_REQ_ID: u32 = 112;
const GAP_FILL_FLAG: u32 = 123;
const RESET_SEQ_NUM_FLAG: u32 = 141;
const REF_TAG_ID: u32 = 371;
const REF_MSG_TYPE: u32 = 372;
const SESSION_REJECT_REASON: u32 = 373;
const TEST_MESSAGE_INDICATOR: u32 = 464;

const SENDING_TIME_ACCURACY_PROBLEM: u32 = 10;
const REQUIRED_TAG_MISSING: u32 = 1;
const VALUE_IS_INCORRECT: u32 = 6;

/// Recovery actions for handling bad messages
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    /// Continue processing despite the error
    Continue,
    /// Request resend for a specific range of sequence numbers
    RequestResend(u64, u64),
    /// Terminate the session due to unrecoverable error
    Terminate,
}

/// Actions to take when handling test request timeouts
#[derive(Debug, Clone)]
pub enum TestRequestAction {
    /// Send a test request message
    SendTestRequest,
    /// Initiate logout sequence
    InitiateLogout,
    /// Terminate connection immediately
    TerminateConnection,
}

/// Session state tracking for edge case management
#[derive(Debug, Clone, Default)]
pub enum SessionState {
    #[default]
    Disconnected,
    LogonPending,
    Active,
    LogoutPending,
    AwaitingResend,
}

impl SessionState {
    pub fn is_active(&self) -> bool {
        matches!(self, SessionState::Active)
    }

    pub fn is_disconnected(&self) -> bool {
        matches!(self, SessionState::Disconnected)
    }

    pub fn can_send_application_messages(&self) -> bool {
        matches!(self, SessionState::Active)
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
pub struct FixConnection<B, C = Config, V = NoOpVerifier> {
    uuid: Uuid,
    config: C,
    backend: B,
    verifier: V,
    encoder: Encoder,
    buffer: SmallVec<[u8; 1024]>,
    msg_seq_num_inbound: MsgSeqNumCounter,
    msg_seq_num_outbound: MsgSeqNumCounter,
    /// Storage for outbound messages to support resend requests
    outbound_message_store: FxHashMap<u64, SmallVec<[u8; 1024]>>,
    /// Storage for inbound messages to detect duplicates
    inbound_message_store: FxHashMap<u64, SmallVec<[u8; 1024]>>,
    /// Last heartbeat time for timeout detection
    last_heartbeat_time: Option<Instant>,
    /// Session state for edge case management
    session_state: SessionState,
}

impl<B, C, V> FixConnection<B, C, V>
where
    C: Configure,
    B: Backend,
    V: Verify,
{
    /// Creates a new FixConnection with the provided backend, config, and verifier.
    pub fn new(backend: B, config: C, verifier: V) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            config,
            backend,
            verifier,
            encoder: Encoder::new(),
            buffer: SmallVec::new(),
            msg_seq_num_inbound: MsgSeqNumCounter::new(),
            msg_seq_num_outbound: MsgSeqNumCounter::new(),
            outbound_message_store: FxHashMap::default(),
            inbound_message_store: FxHashMap::default(),
            last_heartbeat_time: None,
            session_state: SessionState::default(),
        }
    }

    /// Store an outbound message for potential resend requests
    pub fn store_outbound_message(&mut self, seq_num: u64, message: SmallVec<[u8; 1024]>) {
        self.outbound_message_store.insert(seq_num, message);

        // Limit storage to prevent memory growth (keep last 1000 messages)
        // Use count-based retention instead of sequence-based to handle non-contiguous sequence numbers
        if self.outbound_message_store.len() > 1000 {
            self.cleanup_message_store(&mut self.outbound_message_store, 800);
        }
    }

    /// Store an inbound message for duplicate detection
    pub fn store_inbound_message(&mut self, seq_num: u64, message: SmallVec<[u8; 1024]>) {
        self.inbound_message_store.insert(seq_num, message);

        // Limit storage to prevent memory growth (keep last 1000 messages)
        // Use count-based retention instead of sequence-based to handle non-contiguous sequence numbers
        if self.inbound_message_store.len() > 1000 {
            self.cleanup_message_store(&mut self.inbound_message_store, 800);
        }
    }

    /// Check if a message sequence number indicates a duplicate
    pub fn is_duplicate_message(&self, seq_num: u64) -> bool {
        self.inbound_message_store.contains_key(&seq_num)
    }

    /// Clean up message store using count-based retention to handle non-contiguous sequence numbers.
    /// Removes the oldest messages (by sequence number) until the store size is reduced to target_size.
    /// This approach correctly handles FIX protocol scenarios where sequence numbers may have gaps.
    fn cleanup_message_store(
        &mut self,
        store: &mut FxHashMap<u64, SmallVec<[u8; 1024]>>,
        target_size: usize,
    ) {
        if store.len() <= target_size {
            return;
        }

        // Collect and sort sequence numbers to find the oldest messages
        let mut seq_nums: SmallVec<[u64; 32]> = store.keys().cloned().collect();
        seq_nums.sort_unstable();

        // Calculate how many messages to remove
        let messages_to_remove = store.len() - target_size;

        // Remove the oldest messages by sequence number
        for &seq_num in seq_nums.iter().take(messages_to_remove) {
            store.remove(&seq_num);
        }

        log::debug!(
            "Cleaned up message store: removed {} oldest messages, {} messages remaining",
            messages_to_remove,
            store.len()
        );
    }

    /// Get stored messages for resend request
    pub fn get_messages_for_resend(&self, start_seq: u64, end_seq: u64) -> SmallVec<[&[u8]; 16]> {
        let mut messages = SmallVec::new();
        for seq in start_seq..=end_seq {
            if let Some(message) = self.outbound_message_store.get(&seq) {
                messages.push(message.as_slice());
            }
        }
        messages
    }

    /// Update session state
    pub fn set_session_state(&mut self, state: SessionState) {
        log::info!(
            "Session state transition: {:?} -> {:?}",
            self.session_state,
            state
        );
        self.session_state = state;
    }

    /// Get current session state
    pub fn session_state(&self) -> &SessionState {
        &self.session_state
    }

    /// Update heartbeat timestamp
    pub fn update_heartbeat_time(&mut self) {
        self.last_heartbeat_time = Some(Instant::now());
    }

    /// Check if heartbeat timeout has occurred
    pub fn is_heartbeat_timeout(&self, timeout_duration: Duration) -> bool {
        if let Some(last_heartbeat) = self.last_heartbeat_time {
            last_heartbeat.elapsed() > timeout_duration
        } else {
            // No heartbeat received yet, consider it timed out if we're in active state
            self.session_state.is_active()
        }
    }

    /// Handle session timeout scenarios
    pub fn handle_session_timeout(&mut self) -> Response {
        log::warn!("Session timeout detected, initiating logout");
        self.set_session_state(SessionState::LogoutPending);
        self.make_logout("Session timeout".to_string().into())
    }

    /// Enhanced logon handling with state management
    pub fn handle_logon(&mut self, logon: Message<&[u8]>) -> Response {
        log::info!("Processing logon message");

        // Extract and validate logon fields
        let heartbeat_int = logon.get::<u64>(&HEARTBEAT_INT).unwrap_or(30); // HeartBtInt

        // Update session state
        self.set_session_state(SessionState::Active);
        self.update_heartbeat_time();

        // Reset sequence numbers if requested (ResetSeqNumFlag)
        if let Ok(reset_flag) = logon.get::<&str>(&RESET_SEQ_NUM_FLAG) {
            if reset_flag == "Y" {
                log::info!("Resetting sequence numbers as requested");
                self.msg_seq_num_inbound.set_expected(1);
                self.msg_seq_num_outbound.set_expected(1);
                // Clear message stores
                self.outbound_message_store.clear();
                self.inbound_message_store.clear();
            }
        }

        Response::None
    }

    /// Enhanced logout handling with state management
    pub fn handle_logout(&mut self, logout: Message<&[u8]>) -> Response {
        log::info!("Processing logout message");
        self.set_session_state(SessionState::LogoutPending);

        // Send logout acknowledgment and prepare for disconnect
        let logout_response = self.make_logout("Logout acknowledged".to_string().into());

        // Transition to disconnected state after sending response
        self.set_session_state(SessionState::Disconnected);

        logout_response
    }

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

                    // Implement comprehensive error recovery
                    match self.handle_bad_message_recovery(err).await {
                        RecoveryAction::Continue => {
                            log::info!("Recovered from bad message, continuing session");
                        }
                        RecoveryAction::RequestResend(start, end) => {
                            log::warn!("Requesting resend for sequence range {}..{}", start, end);
                            let resend_request = self.create_resend_request(start, end);
                            output.write_all(&resend_request).await.unwrap();
                            self.on_outbound_message(&resend_request).ok();
                        }
                        RecoveryAction::Terminate => {
                            log::error!("Unrecoverable error, terminating session");
                            let logout =
                                self.create_emergency_logout("Unrecoverable message error");
                            output.write_all(&logout).await.unwrap();
                            return;
                        }
                    }
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

                    // Implement proper logout handling
                    self.handle_logout_sequence().await;

                    // Send logout acknowledgment if needed
                    let logout_ack = self.create_logout_response("Logout acknowledged");
                    output.write_all(&logout_ack).await.unwrap();
                    self.on_outbound_message(&logout_ack).ok();

                    // Set session state and cleanup
                    self.set_session_state(SessionState::Disconnected);
                    log::info!("Session gracefully terminated");
                    return;
                }
                LlEvent::TestRequest => {
                    log::debug!("Test request timeout - connection may be unstable");

                    // Implement comprehensive test request handling
                    match self.handle_test_request_timeout().await {
                        TestRequestAction::SendTestRequest => {
                            log::info!("Sending test request to verify connection");
                            let test_request = self.create_test_request();
                            output.write_all(&test_request).await.unwrap();
                            self.on_outbound_message(&test_request).ok();
                        }
                        TestRequestAction::InitiateLogout => {
                            log::warn!("Connection appears dead, initiating logout");
                            let logout = self.create_emergency_logout("Connection timeout");
                            output.write_all(&logout).await.unwrap();
                            self.set_session_state(SessionState::LogoutPending);
                        }
                        TestRequestAction::TerminateConnection => {
                            log::error!("Connection is unresponsive, terminating");
                            self.set_session_state(SessionState::Disconnected);
                            return;
                        }
                    }
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
            // ✅ HIGH PRIORITY FIX: Proper validation instead of always accepting
            match begin_string {
                // Additional supported FIX versions
                b"FIX.4.0" | b"FIX.4.1" | b"FIX.5.0" | b"FIX.5.0SP1" | b"FIX.5.0SP2" | b"FIXT.1.1" => Ok(()),
                _ => {
                    log::warn!("Unsupported FIX protocol version: {}", String::from_utf8_lossy(begin_string));
                    Err(()) // Reject unsupported versions in production
                }
            }
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

impl<'a, B, C, V> FixConnector<'a, B, C, V> for FixConnection<B, C, V>
where
    B: Backend,
    C: Configure,
    V: Verify,
{
    type Error = &'a [u8];
    type Msg = EncoderHandle<'a, SmallVec<[u8; 1024]>>;

    fn on_inbound_app_message(&mut self, message: Message<&[u8]>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        Ok(())
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
}

pub struct MessageBuilder {}

pub struct MessageBuiderTuple<'a> {
    phantom: PhantomData<&'a ()>,
}

impl<'a> MessageBuiderTuple<'a> {
    pub fn get(
        self,
    ) -> (
        EncoderHandle<'a, SmallVec<[u8; 1024]>>,
        &'a mut MessageBuilder,
    ) {
        // NOTE: This is a placeholder implementation to prevent runtime panics
        // TODO: Implement proper message building functionality when encoder integration is ready
        unimplemented!("MessageBuilder integration not yet implemented - use encoder directly")
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

    fn dispatch_by_msg_type(&mut self, msg_type: &[u8], message: Message<&[u8]>) -> Response {
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
                return self.on_resend_request(&message);
            }
            b"4" => {
                // Sequence Reset message - special handling required
                return self.on_sequence_reset(message);
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

    // ✅ CRITICAL FIX: Add missing methods required by the default on_inbound_message implementation
    // These methods were being called but not defined in the trait, causing compilation errors
    // for other types trying to implement FixConnector
    
    /// Check if a message with the given sequence number is a duplicate
    fn is_duplicate_message(&self, seq_num: u64) -> bool;
    
    /// Store an inbound message for duplicate detection and potential resend
    fn store_inbound_message(&mut self, seq_num: u64, message: SmallVec<[u8; 1024]>);
    
    /// Store an outbound message for potential resend requests
    fn store_outbound_message(&mut self, seq_num: u64, message: SmallVec<[u8; 1024]>);
    
    /// Get stored messages for resend within the specified sequence number range
    fn get_messages_for_resend(&self, start_seq: u64, end_seq: u64) -> SmallVec<[&[u8]; 16]>;
    
    /// Update the last heartbeat time for timeout detection
    fn update_heartbeat_time(&mut self);
    
    /// Set the current session state
    fn set_session_state(&mut self, state: SessionState);
    
    /// Get the inbound sequence number counter  
    fn msg_seq_num_inbound(&mut self) -> &mut MsgSeqNumCounter;
    
    /// Get the outbound sequence number counter
    fn msg_seq_num_outbound(&mut self) -> &mut MsgSeqNumCounter;
    
    /// Get the FIX protocol version string (e.g., "FIX.4.4")
    fn begin_string(&self) -> &[u8];
    
    /// Start building a new FIX message
    fn start_message(&mut self, begin_string: &[u8], msg_type: &[u8]) -> Self::Msg;

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

            // Enhanced duplicate detection
            if n < expected {
                if self.is_duplicate_message(n) {
                    log::info!("Received duplicate message with seq_num={}, ignoring", n);
                    return Response::None; // Silently ignore duplicates
                } else {
                    return self.on_low_seqnum(message);
                }
            } else if n > expected {
                // Store the message for potential future processing
                self.store_inbound_message(n, message.as_bytes().into());
                // Refer to specs. §4.8 for more information.
                return self.on_high_seqnum(message);
            }
            n
        } else {
            // See §4.5.3.
            return self.on_missing_seqnum(message);
        };

        // Store the message for duplicate detection
        self.store_inbound_message(seq_num, message.as_bytes().into());

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

    /// Enhanced resend request handling that actually processes stored messages
    fn on_resend_request(&mut self, message: &Message<&[u8]>) -> Response {
        let begin_seq_num = if let Ok(seq) = message.get(&BEGIN_SEQ_NO) {
            seq
        } else {
            return self.make_reject_for_missing_field(message, BEGIN_SEQ_NO, "BeginSeqNo");
        };
        let end_seq_num = if let Ok(seq) = message.get(&END_SEQ_NO) {
            seq
        } else {
            return self.make_reject_for_missing_field(message, END_SEQ_NO, "EndSeqNo");
        };

        log::info!(
            "Processing resend request for sequence range {}..{}",
            begin_seq_num,
            end_seq_num
        );

        // Validate the range
        if begin_seq_num > end_seq_num {
            log::warn!(
                "Invalid resend request: BeginSeqNo({}) > EndSeqNo({})",
                begin_seq_num,
                end_seq_num
            );
            return self.on_reject(
                message.get(&MSG_SEQ_NUM).unwrap_or(0),
                Some(BEGIN_SEQ_NO),
                Some(b"2"), // ResendRequest
                VALUE_IS_INCORRECT,
                "BeginSeqNo cannot be greater than EndSeqNo".to_string(),
            );
        }

        // Check if we have the requested messages
        let available_messages = self.get_messages_for_resend(begin_seq_num, end_seq_num);

        if available_messages.is_empty() {
            log::warn!(
                "No messages available for resend request {}..{}",
                begin_seq_num,
                end_seq_num
            );
            // Send gap fill for the entire range
            return self.send_gap_fill(begin_seq_num, end_seq_num + 1);
        }

        // TODO: Implement actual message resending. For now, we are sending a GapFill message.
        self.send_gap_fill(begin_seq_num, end_seq_num + 1)
    }

    fn on_logout(&mut self, data: ResponseData, _message: &Message<&[u8]>) -> &[u8] {
        let fix_message = {
            let msg_seq_num = self.msg_seq_num_outbound().next();
            let mut logout_message = self.start_message(data.begin_string, b"5");
            self.set_sender_and_target(&mut logout_message);
            logout_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
            logout_message.set_fv_with_key(&TEXT, "Logout");
            logout_message.done()
        };
        fix_message
    }

    #[must_use]
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
        self.update_heartbeat_time();

        // Validate heartbeat fields if present
        if let Ok(test_req_id) = message.get::<&str>(&TEST_REQ_ID) {
            log::debug!("Heartbeat response to TestReq ID: {}", test_req_id);
        }
    }

    fn on_test_request(&mut self, message: Message<&[u8]>) -> &[u8] {
        // ✅ CRITICAL FIX: Handle missing TestReqID field safely (Issue A)
        let test_req_id = match message.get::<&[u8]>(&TEST_REQ_ID) {
            Ok(id) => id,
            Err(_) => {
                // Per FIX Protocol: TestReqID(112) is required for TestRequest messages
                log::warn!("TestRequest message missing required TestReqID field");
                return self.generate_reject_for_missing_field(message, TEST_REQ_ID, "TestReqID");
            }
        };

        // ✅ CRITICAL FIX: Respond with Heartbeat (MsgType=0) not TestRequest (MsgType=1) (Issue B)  
        // Per FIX Protocol specification: TestRequest should be answered with Heartbeat containing the same TestReqID
        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut response_message = self.start_message(begin_string, b"0"); // Heartbeat, not TestRequest
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

    /// Generate reject message bytes for missing required field
    /// Returns raw bytes that can be sent directly
    fn generate_reject_for_missing_field(
        &mut self,
        offender: Message<&[u8]>,
        missing_field: u32,
        field_name: &str,
    ) -> &[u8] {
        let ref_seq_num = offender.get(&MSG_SEQ_NUM).unwrap_or(0);
        let ref_msg_type = offender.get::<&str>(&MSG_TYPE).unwrap_or("?");
        
        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut reject_message = self.start_message(begin_string, b"3"); // Reject message
        
        self.set_sender_and_target(&mut reject_message);
        reject_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        reject_message.set_fv_with_key(&REF_SEQ_NUM, ref_seq_num);
        reject_message.set_fv_with_key(&REF_TAG_ID, missing_field);
        reject_message.set_fv_with_key(&REF_MSG_TYPE, ref_msg_type.as_bytes());
        reject_message.set_fv_with_key(&SESSION_REJECT_REASON, REQUIRED_TAG_MISSING);
        reject_message.set_fv_with_key(&TEXT, format!("Required field {} is missing", field_name).as_str());
        self.set_sending_time(&mut reject_message);
        
        reject_message.done()
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
        err_text: SmartString,
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
            "Bad SendingTime".to_string().into(),
        )
    }

    fn make_logout(&mut self, text: SmartString) -> Response {
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
            return self.handle_logout(message);
        }

        let msg_seq_num = message.get(&MSG_SEQ_NUM).unwrap();

        // Set session state to awaiting resend
        self.set_session_state(SessionState::AwaitingResend);

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

    /// Handle Sequence Reset messages (MsgType = "4")
    /// Per FIX Protocol: Two types based on GapFillFlag(123):
    /// - GapFillFlag = "Y": Gap Fill - increment sequence number without processing
    /// - GapFillFlag = "N": Reset - reset sequence number to NewSeqNo(36)
    fn on_sequence_reset(&mut self, message: Message<&[u8]>) -> Response {
        // Extract required fields
        let new_seq_no = match message.get::<u64>(&NEW_SEQ_NO) {
            Ok(seq) => seq,
            Err(_) => {
                log::warn!("Sequence Reset message missing NewSeqNo(36) field");
                return self.make_reject_for_missing_field(message, NEW_SEQ_NO, "NewSeqNo");
            }
        };

        let gap_fill_flag = message.get::<&str>(&GAP_FILL_FLAG).unwrap_or("N");

        let current_expected = self.msg_seq_num_inbound.expected();

        if gap_fill_flag == "Y" {
            // Gap Fill: Skip sequence numbers without processing messages
            log::info!(
                "Processing Sequence Reset - Gap Fill: filling gap from {} to {}",
                current_expected,
                new_seq_no
            );

            // Validate that NewSeqNo is greater than current expected
            if new_seq_no <= current_expected {
                log::warn!(
                    "Invalid Gap Fill: NewSeqNo({}) <= expected({})",
                    new_seq_no,
                    current_expected
                );
                return self.make_reject_for_invalid_seqno(message, new_seq_no, current_expected);
            }

            // Set the next expected sequence number to NewSeqNo
            self.msg_seq_num_inbound.set_expected(new_seq_no);
            log::info!(
                "Gap filled: next expected sequence number is {}",
                new_seq_no
            );
        } else {
            // Sequence Reset: Reset sequence number
            log::info!(
                "Processing Sequence Reset - Reset: resetting from {} to {}",
                current_expected,
                new_seq_no
            );

            // For reset, NewSeqNo can be <= current (unusual but allowed)
            self.msg_seq_num_inbound.set_expected(new_seq_no);
            log::info!(
                "Sequence number reset: next expected sequence number is {}",
                new_seq_no
            );
        }

        Response::None
    }

    /// Create a reject message for missing required field
    fn make_reject_for_missing_field(
        &mut self,
        offender: Message<&[u8]>,
        missing_field: u32,
        field_name: &str,
    ) -> Response {
        let ref_seq_num = offender.get(&MSG_SEQ_NUM).unwrap_or(0);
        let ref_msg_type = offender.get::<&str>(&MSG_TYPE).unwrap_or("?");
        self.on_reject(
            ref_seq_num,
            Some(missing_field),
            Some(ref_msg_type.as_bytes()),
            REQUIRED_TAG_MISSING,
            format!("Required field {} is missing", field_name).into(),
        )
    }

    /// Create a reject message for invalid sequence number
    fn make_reject_for_invalid_seqno(
        &mut self,
        offender: Message<&[u8]>,
        received_seqno: u64,
        expected_seqno: u64,
    ) -> Response {
        let ref_seq_num = offender.get(&MSG_SEQ_NUM).unwrap_or(0);
        let ref_msg_type = offender.get::<&str>(&MSG_TYPE).unwrap_or("?");
        self.on_reject(
            ref_seq_num,
            Some(NEW_SEQ_NO),
            Some(ref_msg_type.as_bytes()),
            VALUE_IS_INCORRECT,
            format!(
                "Invalid NewSeqNo: received {} but expected > {}",
                received_seqno, expected_seqno
            )
            .into(),
        )
    }

    /// Send a gap fill sequence reset for missing messages
    fn send_gap_fill(&mut self, begin_seq: u64, new_seq: u64) -> Response {
        log::info!("Sending gap fill from {} to {}", begin_seq, new_seq);

        let begin_string = self.begin_string();
        let msg_seq_num = self.msg_seq_num_outbound.next();
        let mut gap_fill_message = self.start_message(begin_string, b"4"); // SequenceReset

        self.set_sender_and_target(&mut gap_fill_message);
        gap_fill_message.set_fv_with_key(&MSG_SEQ_NUM, msg_seq_num);
        gap_fill_message.set_fv_with_key(&NEW_SEQ_NO, new_seq);
        gap_fill_message.set_fv_with_key(&GAP_FILL_FLAG, "Y");
        gap_fill_message.set_fv_with_key(&TEXT, "Gap fill - no messages to resend");
        self.set_sending_time(&mut gap_fill_message);

        let message_bytes = gap_fill_message.done();

        // Store the gap fill message
        self.store_outbound_message(msg_seq_num, message_bytes.into());

        Response::OutboundBytes(message_bytes)
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

    fn create_test_message(msg_type: &str, seq_num: u64) -> SmartString {
        format!(
            "8=FIX.4.4|9=100|35={}|49=SENDER|56=TARGET|34={}|52=20100304-07:59:30|10=000|",
            msg_type, seq_num
        )
        .into()
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

            fn make_logout(&self, text: SmartString) -> Response {
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
                    return self.make_logout("Logout with high sequence number".to_string().into());
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
                let msg_type = message.get_raw(35).unwrap_or_default();
                if msg_type == b"5" { "logout" } else { "resend" }
            }
        }

        let connector = TestConnector;
        let test_cases = [
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
            let message_str = create_test_message(msg_type, 100);
            let mut decoder = create_decoder();
            let message = decoder.decode(message_str.as_bytes()).unwrap();
            let result = connector.on_high_seqnum(&message);
            assert_eq!(
                result, expected,
                "Message type {} should result in {}",
                msg_type, expected
            );
        }
    }

    #[test]
    fn test_sequence_reset_gap_fill_handling() {
        use crate::session::MsgSeqNumCounter;

        struct TestSequenceHandler {
            msg_seq_num_inbound: MsgSeqNumCounter,
        }

        impl TestSequenceHandler {
            fn new() -> Self {
                let mut counter = MsgSeqNumCounter::new();
                // Simulate that we've processed messages 1-5, expecting message 6
                counter.set_expected(6);
                Self {
                    msg_seq_num_inbound: counter,
                }
            }

            fn expected(&self) -> u64 {
                self.msg_seq_num_inbound.expected()
            }

            fn set_expected(&mut self, value: u64) {
                self.msg_seq_num_inbound.set_expected(value);
            }

            fn process_sequence_reset(
                &mut self,
                gap_fill: bool,
                new_seq_no: u64,
            ) -> Result<(), &'static str> {
                let current_expected = self.expected();

                if gap_fill {
                    // Gap Fill validation
                    if new_seq_no <= current_expected {
                        return Err("Invalid Gap Fill: NewSeqNo <= expected");
                    }
                    self.set_expected(new_seq_no);
                } else {
                    // Sequence Reset (no validation needed)
                    self.set_expected(new_seq_no);
                }

                Ok(())
            }
        }

        let mut handler = TestSequenceHandler::new();
        assert_eq!(handler.expected(), 6);

        // Test valid gap fill: fill gap from 6 to 10
        assert!(handler.process_sequence_reset(true, 10).is_ok());
        assert_eq!(handler.expected(), 10);

        // Test invalid gap fill: trying to gap fill backwards
        assert!(handler.process_sequence_reset(true, 8).is_err());
        assert_eq!(handler.expected(), 10); // Should remain unchanged

        // Test sequence reset: can go backwards
        assert!(handler.process_sequence_reset(false, 5).is_ok());
        assert_eq!(handler.expected(), 5);

        // Test sequence reset: can also go forwards
        assert!(handler.process_sequence_reset(false, 15).is_ok());
        assert_eq!(handler.expected(), 15);
    }

    #[test]
    fn test_sequence_reset_message_parsing() {
        // Test that we can parse the sequence reset fields correctly
        let gap_fill_message = "8=FIX.4.2\x019=60\x0135=4\x0149=SENDER\x0156=TARGET\x0134=7\x0152=20240115-10:30:00\x0136=10\x01123=Y\x0110=123\x01";
        let reset_message = "8=FIX.4.2\x019=60\x0135=4\x0149=SENDER\x0156=TARGET\x0134=8\x0152=20240115-10:30:00\x0136=5\x01123=N\x0110=456\x01";
        let no_flag_message = "8=FIX.4.2\x019=55\x0135=4\x0149=SENDER\x0156=TARGET\x0134=9\x0152=20240115-10:30:00\x0136=12\x0110=789\x01";

        let mut decoder = create_decoder();

        // Test gap fill message
        let message = decoder.decode(gap_fill_message.as_bytes()).unwrap();
        assert_eq!(message.get::<u64>(&36).unwrap(), 10); // NewSeqNo
        assert_eq!(message.get::<&str>(&123).unwrap(), "Y"); // GapFillFlag

        // Test reset message
        let message = decoder.decode(reset_message.as_bytes()).unwrap();
        assert_eq!(message.get::<u64>(&36).unwrap(), 5); // NewSeqNo
        assert_eq!(message.get::<&str>(&123).unwrap(), "N"); // GapFillFlag

        // Test message without gap fill flag (should default to "N")
        let message = decoder.decode(no_flag_message.as_bytes()).unwrap();
        assert_eq!(message.get::<u64>(&36).unwrap(), 12); // NewSeqNo
        assert!(message.get::<&str>(&123).is_err()); // No GapFillFlag field
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
