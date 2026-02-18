use super::{
    Backend, Configure, LlEvent, LlEventLoop, SessionKey, SessionRole, SessionStore,
    StoredAppMessage,
};
use crate::field_types::Timestamp;
use crate::tagvalue::{Decoder, Encoder, Message};
use crate::{Dictionary, FieldMap, SetField, TagU32};
use futures::{AsyncRead, AsyncWrite, AsyncWriteExt};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::time::Duration;

const BEGIN_STRING: u32 = 8;
const MSG_SEQ_NUM: u32 = 34;
const MSG_TYPE: u32 = 35;
const POSS_DUP_FLAG: u32 = 43;
const SENDER_COMP_ID: u32 = 49;
const SENDING_TIME: u32 = 52;
const TARGET_COMP_ID: u32 = 56;
const TEXT: u32 = 58;
const ORIG_SENDING_TIME: u32 = 122;
const RESET_SEQ_NUM_FLAG: u32 = 141;
const ENCRYPT_METHOD: u32 = 98;
const HEARTBT_INT: u32 = 108;
const GAP_FILL_FLAG: u32 = 123;
const TEST_REQ_ID: u32 = 112;
const BEGIN_SEQ_NO: u32 = 7;
const END_SEQ_NO: u32 = 16;
const NEW_SEQ_NO: u32 = 36;
const REF_TAG_ID: u32 = 371;
const REF_MSG_TYPE: u32 = 372;
const SESSION_REJECT_REASON: u32 = 373;
const REF_SEQ_NUM: u32 = 45;
const BUSINESS_REJECT_REASON: u32 = 380;
const TEST_MESSAGE_INDICATOR: u32 = 464;
const MESSAGE_ENCODING: u32 = 347;
const ON_BEHALF_OF_COMP_ID: u32 = 115;
const ON_BEHALF_OF_SUB_ID: u32 = 116;
const ON_BEHALF_OF_LOCATION_ID: u32 = 144;
const DELIVER_TO_COMP_ID: u32 = 128;
const DELIVER_TO_SUB_ID: u32 = 129;
const DELIVER_TO_LOCATION_ID: u32 = 145;

// SessionRejectReason for invalid SendingTime.
const REJECT_REASON_SENDING_TIME_ACCURACY: u32 = 10;
const REJECT_REASON_INVALID_TAG_NUMBER: u32 = 0;
const REJECT_REASON_REQUIRED_TAG_MISSING: u32 = 1;
const REJECT_REASON_TAG_NOT_DEFINED_FOR_MESSAGE: u32 = 2;
const REJECT_REASON_TAG_SPECIFIED_WITHOUT_VALUE: u32 = 4;
const REJECT_REASON_VALUE_OUT_OF_RANGE: u32 = 5;
const REJECT_REASON_INCORRECT_DATA_FORMAT: u32 = 6;
const REJECT_REASON_COMP_ID_PROBLEM: u32 = 9;
const REJECT_REASON_INVALID_MSGTYPE: u32 = 11;
const REJECT_REASON_TAG_APPEARS_MORE_THAN_ONCE: u32 = 13;
const REJECT_REASON_TAG_OUT_OF_ORDER: u32 = 14;
const REJECT_REASON_INCORRECT_NUMINGROUP_COUNT: u32 = 16;

const BUSINESS_REJECT_REASON_UNSUPPORTED_MESSAGE_TYPE: u32 = 3;

/// Runtime status for one FIX session.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SessionStatus {
    /// The session has not started yet.
    Disconnected,
    /// Session I/O has started and `Logon <A>` is pending.
    AwaitingLogon,
    /// Session is established and application flow is active.
    Active,
    /// `Logout <5>` has been sent and the session is waiting to terminate.
    LogoutSent,
    /// Session is terminated.
    Terminated,
}

/// Mutable session state tracked by [`FixConnection`].
#[derive(Debug, Clone)]
pub struct SessionState {
    status: SessionStatus,
    seq_numbers: super::SeqNumbers,
    negotiated_heartbeat: Duration,
    logon_sent: bool,
    test_request_counter: u64,
    resend_request_pending: bool,
    queued_inbound: BTreeMap<u64, Vec<u8>>,
}

impl SessionState {
    fn new<C>(config: &C) -> Self
    where
        C: Configure,
    {
        Self {
            status: SessionStatus::Disconnected,
            seq_numbers: config.seq_numbers(),
            negotiated_heartbeat: config.heartbeat(),
            logon_sent: false,
            test_request_counter: 0,
            resend_request_pending: false,
            queued_inbound: BTreeMap::new(),
        }
    }

    /// Current lifecycle status.
    pub fn status(&self) -> SessionStatus {
        self.status
    }

    /// Current session sequence numbers.
    pub fn seq_numbers(&self) -> super::SeqNumbers {
        self.seq_numbers
    }

    /// Current negotiated heartbeat interval.
    pub fn negotiated_heartbeat(&self) -> Duration {
        self.negotiated_heartbeat
    }
}

/// A non-I/O session protocol violation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionProtocolError {
    /// Missing mandatory FIX field.
    MissingField {
        /// Field tag.
        tag: u32,
        /// Field name.
        name: &'static str,
    },
    /// Field exists but has invalid value or format.
    InvalidField {
        /// Field tag.
        tag: u32,
        /// Field name.
        name: &'static str,
        /// Diagnostic reason.
        reason: &'static str,
    },
    /// Invalid tag number encountered while parsing a decoded inbound message.
    InvalidTagNumber(i64),
    /// BeginString mismatch.
    BeginStringMismatch,
    /// SenderCompID / TargetCompID mismatch.
    CompIdMismatch,
    /// Received test message in environment that disallows tests.
    TestingNotAllowed,
    /// MsgSeqNum lower than expected.
    MsgSeqNumTooLow {
        /// Expected inbound sequence number.
        expected: u64,
        /// Actual value received from peer.
        actual: u64,
    },
    /// Application message type is unsupported by current engine/backend policy.
    UnsupportedMessageType(Vec<u8>),
    /// Peer sent malformed FIX message bytes.
    MalformedInboundMessage,
}

impl SessionProtocolError {
    fn text(&self) -> String {
        match self {
            Self::MissingField { tag, name } => format!("Missing mandatory field {name}({tag})"),
            Self::InvalidField { tag, name, reason } => {
                format!("Invalid field {name}({tag}): {reason}")
            }
            Self::InvalidTagNumber(tag) => format!("Invalid tag number ({tag})"),
            Self::BeginStringMismatch => "BeginString(8) mismatch".to_string(),
            Self::CompIdMismatch => "SenderCompID(49) / TargetCompID(56) mismatch".to_string(),
            Self::TestingNotAllowed => {
                "TestMessageIndicator(464)=Y not allowed in current environment".to_string()
            }
            Self::MsgSeqNumTooLow { expected, actual } => {
                format!("MsgSeqNum too low, expecting {expected} but received {actual}")
            }
            Self::UnsupportedMessageType(msg_type) => {
                format!(
                    "Unsupported Message Type ({})",
                    String::from_utf8_lossy(msg_type.as_slice())
                )
            }
            Self::MalformedInboundMessage => "Malformed inbound FIX message".to_string(),
        }
    }
}

/// Final outcome of a session run loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunOutcome {
    /// Session ended because peer sent `Logout <5>`.
    PeerLogout,
    /// Session ended because heartbeat deadlines were missed.
    HeartbeatTimeout,
    /// Session ended due to protocol violation.
    ProtocolViolation(SessionProtocolError),
    /// Session ended because transport was closed.
    TransportClosed,
}

/// Hard errors raised by the session runtime.
#[derive(Debug)]
pub enum RunError<E> {
    /// I/O failure on the underlying transport.
    Io(std::io::Error),
    /// Backend callback failure.
    Backend(E),
}

#[derive(Debug)]
enum EngineError<E> {
    Backend(E),
    Protocol(SessionProtocolError),
}

#[derive(Default)]
struct InboundActions {
    outbound: Vec<Vec<u8>>,
    resend_requests: Vec<(u64, u64)>,
    reset_store: bool,
    reset_heartbeat: bool,
    heartbeat_update: Option<Duration>,
    terminate: Option<RunOutcome>,
}

struct ProtocolResolution {
    outbound: Vec<Vec<u8>>,
    reset_heartbeat: bool,
    terminate: bool,
}

/// Asynchronous FIX session engine.
///
/// This type implements transport-agnostic session behavior for:
///
/// - `Logon <A>` handshake.
/// - heartbeat / test request / logout lifecycle.
/// - inbound sequence checks and resend requests.
/// - basic `Reject <3>` generation for malformed session fields.
///
/// Compliance notes:
///
/// - This engine enforces strict sequence monotonicity and issues
///   `ResendRequest <2>` when gaps are detected.
/// - Persistent outbound replay semantics are managed via [`SessionStore`].
#[derive(Debug)]
pub struct FixConnection<B, S, C = super::Config> {
    backend: B,
    store: S,
    config: C,
    session_key: SessionKey,
    state: SessionState,
    encoder: Encoder,
    encode_buffer: Vec<u8>,
}

impl<B, S, C> FixConnection<B, S, C>
where
    B: Backend,
    S: SessionStore<Error = B::Error>,
    C: Configure,
{
    /// Creates a new session engine.
    pub fn new(backend: B, store: S, config: C) -> Self {
        let session_key = SessionKey {
            begin_string: config.begin_string().to_vec(),
            sender_comp_id: backend.sender_comp_id().to_vec(),
            target_comp_id: backend.target_comp_id().to_vec(),
        };
        Self {
            backend,
            store,
            state: SessionState::new(&config),
            config,
            session_key,
            encoder: Encoder::new(),
            encode_buffer: Vec::new(),
        }
    }

    /// Returns immutable access to the backend.
    pub fn backend(&self) -> &B {
        &self.backend
    }

    /// Returns mutable access to the backend.
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Returns immutable access to the session store.
    pub fn store(&self) -> &S {
        &self.store
    }

    /// Returns mutable access to the session store.
    pub fn store_mut(&mut self) -> &mut S {
        &mut self.store
    }

    /// Returns immutable access to runtime session state.
    pub fn state(&self) -> &SessionState {
        &self.state
    }

    /// Runs the session state machine over an arbitrary async transport.
    pub async fn run<I, O>(
        &mut self,
        input: I,
        mut output: O,
    ) -> Result<RunOutcome, RunError<B::Error>>
    where
        I: AsyncRead + Unpin,
        O: AsyncWrite + Unpin,
    {
        self.state.status = SessionStatus::AwaitingLogon;
        if let Some(seq_numbers) = self
            .store
            .load_seq_numbers(&self.session_key)
            .await
            .map_err(RunError::Backend)?
        {
            self.state.seq_numbers = seq_numbers;
        }
        let decoder = Decoder::new(default_dictionary_for_begin_string(
            self.config.begin_string(),
        ))
        .streaming(Vec::new());
        let mut event_loop = LlEventLoop::new(decoder, input, self.state.negotiated_heartbeat);

        if self.config.role() == SessionRole::Initiator {
            let logon = self.build_logon_message(false);
            self.send_raw(&mut output, &logon).await?;
            self.persist_seq_numbers()
                .await
                .map_err(RunError::Backend)?;
            self.state.logon_sent = true;
        }

        loop {
            self.flush_backend_outbound(&mut output).await?;

            let event = match event_loop.next_event().await {
                Some(event) => event,
                None => {
                    self.state.status = SessionStatus::Terminated;
                    return Ok(RunOutcome::TransportClosed);
                }
            };

            match event {
                LlEvent::Message(msg) => {
                    let inbound_before = self.state.seq_numbers.next_inbound();
                    match self.handle_inbound_message(msg) {
                        Ok(mut actions) => {
                            if actions.reset_store {
                                self.store
                                    .reset_session(&self.session_key, self.state.seq_numbers)
                                    .await
                                    .map_err(RunError::Backend)?;
                            }
                            for (begin_seq_no, end_seq_no) in actions.resend_requests.drain(..) {
                                let replay_outbound = self
                                    .build_resend_reply_messages(begin_seq_no, end_seq_no)
                                    .await
                                    .map_err(|err| match err {
                                        EngineError::Backend(e) => RunError::Backend(e),
                                        EngineError::Protocol(_) => RunError::Io(
                                            std::io::Error::other("failed to build resend reply"),
                                        ),
                                    })?;
                                actions.outbound.extend(replay_outbound);
                            }
                            for outbound in &actions.outbound {
                                self.send_raw(&mut output, outbound).await?;
                            }
                            if !actions.outbound.is_empty()
                                || self.state.seq_numbers.next_inbound() != inbound_before
                            {
                                self.persist_seq_numbers()
                                    .await
                                    .map_err(RunError::Backend)?;
                            }
                            if let Some(heartbeat) = actions.heartbeat_update {
                                event_loop.set_heartbeat(heartbeat);
                            }
                            if actions.reset_heartbeat {
                                event_loop.ping_heartbeat();
                            }
                            if let Some(outcome) = actions.terminate {
                                self.state.status = SessionStatus::Terminated;
                                return Ok(outcome);
                            }
                        }
                        Err(EngineError::Backend(err)) => return Err(RunError::Backend(err)),
                        Err(EngineError::Protocol(protocol_error)) => {
                            if self.state.status == SessionStatus::AwaitingLogon {
                                self.state.status = SessionStatus::Terminated;
                                return Ok(RunOutcome::ProtocolViolation(protocol_error));
                            }

                            if let Some(resolution) =
                                self.protocol_resolution_for_active_error(msg, &protocol_error)
                            {
                                for outbound in resolution.outbound.iter() {
                                    self.send_raw(&mut output, outbound).await?;
                                }
                                if !resolution.outbound.is_empty() {
                                    self.persist_seq_numbers()
                                        .await
                                        .map_err(RunError::Backend)?;
                                }
                                if resolution.reset_heartbeat {
                                    event_loop.ping_heartbeat();
                                }
                                if resolution.terminate {
                                    self.state.status = SessionStatus::Terminated;
                                    return Ok(RunOutcome::ProtocolViolation(protocol_error));
                                }
                                continue;
                            }

                            let logout_text = protocol_logout_text(&protocol_error);
                            let logout = self.build_logout_message(logout_text.as_deref());
                            self.send_raw(&mut output, &logout).await?;
                            self.persist_seq_numbers()
                                .await
                                .map_err(RunError::Backend)?;
                            self.state.status = SessionStatus::Terminated;
                            return Ok(RunOutcome::ProtocolViolation(protocol_error));
                        }
                    }
                }
                LlEvent::BadMessage(_decode_error) => {
                    let protocol_error = SessionProtocolError::MalformedInboundMessage;
                    if self.state.status == SessionStatus::AwaitingLogon {
                        self.state.status = SessionStatus::Terminated;
                        return Ok(RunOutcome::ProtocolViolation(protocol_error));
                    }
                    // Post-logon garbled bytes are ignored per QuickFIX acceptance semantics.
                    continue;
                }
                LlEvent::IoError(err) => return Err(RunError::Io(err)),
                LlEvent::Heartbeat => {
                    if self.state.status == SessionStatus::Active {
                        self.backend
                            .on_heartbeat_is_due()
                            .map_err(RunError::Backend)?;
                        let heartbeat = self.build_heartbeat_message(None);
                        self.send_raw(&mut output, &heartbeat).await?;
                        self.persist_seq_numbers()
                            .await
                            .map_err(RunError::Backend)?;
                    }
                }
                LlEvent::TestRequest => {
                    if self.state.status == SessionStatus::Active {
                        let test_request = self.build_test_request_message();
                        self.send_raw(&mut output, &test_request).await?;
                        self.persist_seq_numbers()
                            .await
                            .map_err(RunError::Backend)?;
                    }
                }
                LlEvent::Logout => {
                    let logout = self.build_logout_message(Some(b"Heartbeat timeout"));
                    self.send_raw(&mut output, &logout).await?;
                    self.persist_seq_numbers()
                        .await
                        .map_err(RunError::Backend)?;
                    self.state.status = SessionStatus::Terminated;
                    return Ok(RunOutcome::HeartbeatTimeout);
                }
            }
        }
    }

    fn handle_inbound_message(
        &mut self,
        message: Message<&[u8]>,
    ) -> Result<InboundActions, EngineError<B::Error>> {
        let mut actions = self.handle_inbound_message_inner(message)?;
        self.drain_queued_inbound_messages(&mut actions)?;
        Ok(actions)
    }

    fn handle_inbound_message_inner(
        &mut self,
        message: Message<&[u8]>,
    ) -> Result<InboundActions, EngineError<B::Error>> {
        if !has_ordered_session_header_prefix(message.as_bytes()) {
            return Ok(InboundActions::default());
        }

        self.validate_begin_string(message)?;

        let inbound_seq_num = match required_u64(message, MSG_SEQ_NUM, "MsgSeqNum") {
            Ok(seq_num) => seq_num,
            Err(SessionProtocolError::InvalidField {
                tag: MSG_SEQ_NUM, ..
            })
            | Err(SessionProtocolError::MissingField {
                tag: MSG_SEQ_NUM, ..
            }) => {
                if let Some(recovered) = self.recover_embedded_message(message.as_bytes())? {
                    return Ok(recovered);
                }
                if self.state.status == SessionStatus::AwaitingLogon {
                    return Err(EngineError::Protocol(SessionProtocolError::MissingField {
                        tag: MSG_SEQ_NUM,
                        name: "MsgSeqNum",
                    }));
                }
                if self.state.resend_request_pending {
                    return Ok(InboundActions::default());
                }
                let resend_request =
                    self.build_resend_request_message(self.state.seq_numbers.next_inbound(), 0);
                self.state.resend_request_pending = true;
                return Ok(InboundActions {
                    outbound: vec![resend_request],
                    reset_heartbeat: false,
                    heartbeat_update: None,
                    terminate: None,
                    ..InboundActions::default()
                });
            }
            Err(err) => return Err(EngineError::Protocol(err)),
        };
        let msg_type = required_raw(message, MSG_TYPE, "MsgType").map_err(EngineError::Protocol)?;
        let is_sequence_reset = msg_type.as_slice() == b"4";
        let gap_fill = if is_sequence_reset {
            message
                .get_opt::<bool>(GAP_FILL_FLAG)
                .map_err(|_| {
                    EngineError::Protocol(SessionProtocolError::InvalidField {
                        tag: GAP_FILL_FLAG,
                        name: "GapFillFlag",
                        reason: "invalid boolean value",
                    })
                })?
                .unwrap_or(false)
        } else {
            false
        };
        let poss_dup = message
            .get_opt::<bool>(POSS_DUP_FLAG)
            .map_err(|_| {
                EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: POSS_DUP_FLAG,
                    name: "PossDupFlag",
                    reason: "invalid boolean value",
                })
            })?
            .unwrap_or(false);
        let reset_seq_num = if msg_type.as_slice() == b"A" {
            message
                .get_opt::<bool>(RESET_SEQ_NUM_FLAG)
                .map_err(|_| {
                    EngineError::Protocol(SessionProtocolError::InvalidField {
                        tag: RESET_SEQ_NUM_FLAG,
                        name: "ResetSeqNumFlag",
                        reason: "invalid boolean value",
                    })
                })?
                .unwrap_or(false)
        } else {
            false
        };

        if self.state.status == SessionStatus::AwaitingLogon && msg_type.as_slice() != b"A" {
            return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: MSG_TYPE,
                name: "MsgType",
                reason: "first message must be Logon",
            }));
        }

        if has_malformed_tag_token(message.as_bytes()) || !has_valid_checksum(message.as_bytes()) {
            if let Some(recovered) = self.recover_embedded_message(message.as_bytes())? {
                return Ok(recovered);
            }
            return Ok(InboundActions::default());
        }

        if poss_dup {
            self.validate_possdup_orig_sending_time(message)?;
        }

        let mut reset_store = false;
        if reset_seq_num && msg_type.as_slice() == b"A" {
            self.reset_session_state();
            reset_store = true;
        }

        let expected_inbound_seq = self.state.seq_numbers.next_inbound();
        let mut recover_on_logon = false;
        let mut pre_outbound = Vec::new();
        let mut should_increment_inbound = !is_sequence_reset;
        let mut should_validate_sending_time = true;

        match self.state.seq_numbers.validate_inbound(inbound_seq_num) {
            Ok(()) => {
                self.state.resend_request_pending = false;
            }
            Err(super::SeqNumberError::Recover) => {
                if self.state.status == SessionStatus::AwaitingLogon && msg_type.as_slice() == b"A"
                {
                    recover_on_logon = true;
                    self.state.seq_numbers.next_inbound = inbound_seq_num.saturating_add(1);
                    self.validate_sending_time(message)?;
                } else if msg_type.as_slice() == b"5" {
                    let logout = self.build_logout_message(None);
                    return Ok(InboundActions {
                        outbound: vec![logout],
                        reset_heartbeat: false,
                        heartbeat_update: None,
                        terminate: Some(RunOutcome::PeerLogout),
                        ..InboundActions::default()
                    });
                } else if msg_type.as_slice() == b"2" {
                    if !self.state.resend_request_pending {
                        pre_outbound.push(self.build_resend_request_message(
                            self.state.seq_numbers.next_inbound(),
                            0,
                        ));
                        self.state.resend_request_pending = true;
                    }
                    should_increment_inbound = false;
                } else {
                    self.state
                        .queued_inbound
                        .entry(inbound_seq_num)
                        .or_insert_with(|| message.as_bytes().to_vec());
                    if self
                        .state
                        .queued_inbound
                        .contains_key(&self.state.seq_numbers.next_inbound())
                    {
                        return Ok(InboundActions::default());
                    }
                    if self.state.resend_request_pending {
                        return Ok(InboundActions::default());
                    }
                    let resend_request =
                        self.build_resend_request_message(self.state.seq_numbers.next_inbound(), 0);
                    self.state.resend_request_pending = true;
                    return Ok(InboundActions {
                        outbound: vec![resend_request],
                        reset_heartbeat: false,
                        heartbeat_update: None,
                        terminate: None,
                        ..InboundActions::default()
                    });
                }
            }
            Err(super::SeqNumberError::TooLow) => {
                if msg_type.as_slice() == b"5" {
                    let logout = self.build_logout_message(None);
                    return Ok(InboundActions {
                        outbound: vec![logout],
                        reset_heartbeat: false,
                        heartbeat_update: None,
                        terminate: Some(RunOutcome::PeerLogout),
                        ..InboundActions::default()
                    });
                }
                if poss_dup {
                    return Ok(InboundActions::default());
                }
                if is_sequence_reset && !gap_fill {
                    should_increment_inbound = false;
                    should_validate_sending_time = true;
                } else {
                    return Err(EngineError::Protocol(
                        SessionProtocolError::MsgSeqNumTooLow {
                            expected: self.state.seq_numbers.next_inbound(),
                            actual: inbound_seq_num,
                        },
                    ));
                }
            }
            Err(super::SeqNumberError::NoSeqNum) => {
                return Err(EngineError::Protocol(SessionProtocolError::MissingField {
                    tag: MSG_SEQ_NUM,
                    name: "MsgSeqNum",
                }));
            }
        }

        if !recover_on_logon {
            if should_validate_sending_time {
                self.validate_sending_time(message)?;
            }
            if should_increment_inbound {
                self.state.seq_numbers.incr_inbound();
            }
        }

        self.validate_comp_ids(message)?;
        self.validate_test_indicator(message)?;
        self.validate_inbound_fields(message, msg_type.as_slice())?;

        if msg_type.as_slice() == b"8" {
            return Err(EngineError::Protocol(
                SessionProtocolError::UnsupportedMessageType(msg_type.clone()),
            ));
        }

        let is_session = is_session_message_type(msg_type.as_slice());
        self.backend
            .on_inbound_message(message, !is_session)
            .map_err(EngineError::Backend)?;

        let mut actions = InboundActions {
            outbound: pre_outbound,
            reset_store,
            ..InboundActions::default()
        };
        match msg_type.as_slice() {
            b"A" => {
                let peer_heartbeat = required_u64(message, HEARTBT_INT, "HeartBtInt")
                    .map_err(EngineError::Protocol)?;
                let peer_heartbeat = Duration::from_secs(peer_heartbeat);
                self.config
                    .heartbeat_rule()
                    .validate(&peer_heartbeat)
                    .map_err(|_| {
                        EngineError::Protocol(SessionProtocolError::InvalidField {
                            tag: HEARTBT_INT,
                            name: "HeartBtInt",
                            reason: "outside configured heartbeat rule",
                        })
                    })?;

                self.state.negotiated_heartbeat = peer_heartbeat;
                actions.heartbeat_update = Some(peer_heartbeat);
                actions.reset_heartbeat = true;

                if !self.state.logon_sent || reset_seq_num {
                    let logon = self.build_logon_message(reset_seq_num);
                    actions.outbound.push(logon);
                    self.state.logon_sent = true;
                }

                if self.state.status != SessionStatus::Active {
                    self.state.status = SessionStatus::Active;
                    self.backend
                        .on_successful_handshake()
                        .map_err(EngineError::Backend)?;
                }
            }
            b"0" => {
                actions.reset_heartbeat = true;
            }
            b"1" => {
                let test_req_id = required_raw(message, TEST_REQ_ID, "TestReqID")
                    .map_err(EngineError::Protocol)?;
                actions
                    .outbound
                    .push(self.build_heartbeat_message(Some(test_req_id.as_slice())));
                actions.reset_heartbeat = true;
            }
            b"2" => {
                let begin_seq_no = required_u64(message, BEGIN_SEQ_NO, "BeginSeqNo")
                    .map_err(EngineError::Protocol)?;
                let end_seq_no =
                    required_u64(message, END_SEQ_NO, "EndSeqNo").map_err(EngineError::Protocol)?;
                actions.resend_requests.push((begin_seq_no, end_seq_no));
            }
            b"4" => {
                let new_seq_no =
                    required_u64(message, NEW_SEQ_NO, "NewSeqNo").map_err(EngineError::Protocol)?;
                if gap_fill {
                    if new_seq_no > self.state.seq_numbers.next_inbound() {
                        self.state.seq_numbers.next_inbound = new_seq_no;
                    }
                } else if new_seq_no >= self.state.seq_numbers.next_inbound() {
                    self.state.seq_numbers.next_inbound = new_seq_no;
                } else {
                    return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                        tag: NEW_SEQ_NO,
                        name: "NewSeqNo",
                        reason: "value is incorrect (out of range) for this tag",
                    }));
                }
            }
            b"5" => {
                if self.state.status != SessionStatus::LogoutSent {
                    actions.outbound.push(self.build_logout_message(None));
                    self.state.status = SessionStatus::LogoutSent;
                }
                actions.terminate = Some(RunOutcome::PeerLogout);
            }
            _ => {
                actions.reset_heartbeat = true;
            }
        }

        if recover_on_logon {
            actions
                .outbound
                .push(self.build_resend_request_message(expected_inbound_seq, 0));
        }

        Ok(actions)
    }

    fn drain_queued_inbound_messages(
        &mut self,
        actions: &mut InboundActions,
    ) -> Result<(), EngineError<B::Error>> {
        while actions.terminate.is_none() {
            let expected = self.state.seq_numbers.next_inbound();
            let Some(raw) = self.state.queued_inbound.remove(&expected) else {
                break;
            };
            let mut decoder = Decoder::new(default_dictionary_for_begin_string(
                self.config.begin_string(),
            ));
            let queued_message = decoder.decode(raw.as_slice()).map_err(|_| {
                EngineError::Protocol(SessionProtocolError::MalformedInboundMessage)
            })?;
            let queued_actions = self.handle_inbound_message_inner(queued_message)?;
            Self::merge_inbound_actions(actions, queued_actions);
        }
        Ok(())
    }

    fn recover_embedded_message(
        &mut self,
        raw: &[u8],
    ) -> Result<Option<InboundActions>, EngineError<B::Error>> {
        let begin_string = self.config.begin_string();
        let mut i = 1usize;
        while i + begin_string.len() + 2 < raw.len() {
            if raw[i] == b'8' && raw[i + 1] == b'=' && raw[i + 2..].starts_with(begin_string) {
                let tail = &raw[i..];
                let mut decoder = Decoder::new(default_dictionary_for_begin_string(begin_string));
                if let Ok(message) = decoder.decode(tail) {
                    let mut actions = self.handle_inbound_message_inner(message)?;
                    self.drain_queued_inbound_messages(&mut actions)?;
                    return Ok(Some(actions));
                }
            }
            i += 1;
        }
        Ok(None)
    }

    fn merge_inbound_actions(acc: &mut InboundActions, next: InboundActions) {
        acc.outbound.extend(next.outbound);
        acc.resend_requests.extend(next.resend_requests);
        acc.reset_store |= next.reset_store;
        acc.reset_heartbeat |= next.reset_heartbeat;
        if next.heartbeat_update.is_some() {
            acc.heartbeat_update = next.heartbeat_update;
        }
        if acc.terminate.is_none() {
            acc.terminate = next.terminate;
        }
    }

    fn reset_session_state(&mut self) {
        self.state.seq_numbers = self.config.seq_numbers();
        self.state.resend_request_pending = false;
        self.state.queued_inbound.clear();
    }

    async fn persist_seq_numbers(&mut self) -> Result<(), B::Error> {
        self.store
            .save_seq_numbers(&self.session_key, self.state.seq_numbers)
            .await
    }

    async fn build_resend_reply_messages(
        &mut self,
        begin_seq_no: u64,
        end_seq_no: u64,
    ) -> Result<Vec<Vec<u8>>, EngineError<B::Error>> {
        let last_sent = self.state.seq_numbers.next_outbound().saturating_sub(1);
        let start = begin_seq_no.max(1);
        let capped_end = if end_seq_no == 0 || end_seq_no == u64::MAX {
            last_sent
        } else {
            end_seq_no.min(last_sent)
        };
        if last_sent == 0 {
            return Ok(vec![self.build_gap_fill_message(start)]);
        }
        if start > capped_end {
            return Ok(vec![self.build_gap_fill_message(start)]);
        }

        let stored = self
            .store
            .load_outbound_app_range(&self.session_key, start, capped_end)
            .await
            .map_err(EngineError::Backend)?;
        let mut stored_by_seq = BTreeMap::<u64, Vec<u8>>::new();
        for message in stored {
            if message.seq_num < start || message.seq_num > capped_end {
                continue;
            }
            if extract_msg_seq_num(message.raw_message.as_slice()) != Some(message.seq_num) {
                continue;
            }
            let Some(msg_type) = extract_msg_type(message.raw_message.as_slice()) else {
                continue;
            };
            if is_session_message_type(msg_type.as_slice()) {
                continue;
            }
            if extract_raw_field_value(message.raw_message.as_slice(), SENDING_TIME).is_none() {
                continue;
            }
            if extract_raw_field_value(message.raw_message.as_slice(), SENDER_COMP_ID).is_none() {
                continue;
            }
            if extract_raw_field_value(message.raw_message.as_slice(), TARGET_COMP_ID).is_none() {
                continue;
            }
            stored_by_seq.insert(message.seq_num, message.raw_message);
        }

        let mut outbound = Vec::new();
        let mut current = start;
        let mut gap_start: Option<u64> = None;
        while current <= capped_end {
            if let Some(raw) = stored_by_seq.get(&current) {
                if gap_start.is_some() {
                    outbound.push(self.build_gap_fill_message(current));
                    gap_start = None;
                }
                let replay = self
                    .build_possdup_replay_from_stored(raw.as_slice())
                    .map_err(EngineError::Protocol)?;
                outbound.push(replay);
            } else if gap_start.is_none() {
                gap_start = Some(current);
            }
            current = current.saturating_add(1);
            if current == 0 {
                break;
            }
        }

        if gap_start.is_some() {
            outbound.push(self.build_gap_fill_message(capped_end.saturating_add(1)));
        }

        Ok(outbound)
    }

    fn build_possdup_replay_from_stored(
        &mut self,
        raw: &[u8],
    ) -> Result<Vec<u8>, SessionProtocolError> {
        let fields = parse_raw_fields(raw);
        let begin_string = fields
            .iter()
            .find(|field| field.tag_i64 == i64::from(BEGIN_STRING))
            .map(|field| field.value)
            .ok_or(SessionProtocolError::MissingField {
                tag: BEGIN_STRING,
                name: "BeginString",
            })?;
        let msg_type = fields
            .iter()
            .find(|field| field.tag_i64 == i64::from(MSG_TYPE))
            .map(|field| field.value)
            .ok_or(SessionProtocolError::MissingField {
                tag: MSG_TYPE,
                name: "MsgType",
            })?;
        let msg_seq_num = fields
            .iter()
            .find(|field| field.tag_i64 == i64::from(MSG_SEQ_NUM))
            .and_then(|field| std::str::from_utf8(field.value).ok())
            .and_then(|value| value.parse::<u64>().ok())
            .ok_or(SessionProtocolError::InvalidField {
                tag: MSG_SEQ_NUM,
                name: "MsgSeqNum",
                reason: "invalid integer",
            })?;
        let sender_comp_id = fields
            .iter()
            .find(|field| field.tag_i64 == i64::from(SENDER_COMP_ID))
            .map(|field| field.value)
            .ok_or(SessionProtocolError::MissingField {
                tag: SENDER_COMP_ID,
                name: "SenderCompID",
            })?;
        let target_comp_id = fields
            .iter()
            .find(|field| field.tag_i64 == i64::from(TARGET_COMP_ID))
            .map(|field| field.value)
            .ok_or(SessionProtocolError::MissingField {
                tag: TARGET_COMP_ID,
                name: "TargetCompID",
            })?;
        let orig_sending_time = fields
            .iter()
            .find(|field| field.tag_i64 == i64::from(SENDING_TIME))
            .map(|field| field.value)
            .ok_or(SessionProtocolError::MissingField {
                tag: SENDING_TIME,
                name: "SendingTime",
            })?;

        self.encode_buffer.clear();
        let mut replay =
            self.encoder
                .start_message(begin_string, &mut self.encode_buffer, msg_type);
        replay.set(MSG_SEQ_NUM, msg_seq_num);
        replay.set(POSS_DUP_FLAG, true);
        replay.set(SENDER_COMP_ID, sender_comp_id);
        replay.set(SENDING_TIME, Timestamp::utc_now());
        replay.set(TARGET_COMP_ID, target_comp_id);
        replay.set(ORIG_SENDING_TIME, orig_sending_time);

        for field in fields {
            let tag = field.tag_as_u32();
            if matches!(
                tag,
                BEGIN_STRING
                    | 9
                    | 10
                    | MSG_SEQ_NUM
                    | MSG_TYPE
                    | POSS_DUP_FLAG
                    | SENDER_COMP_ID
                    | SENDING_TIME
                    | TARGET_COMP_ID
                    | ORIG_SENDING_TIME
            ) {
                continue;
            }
            replay.set(tag, field.value);
        }

        let (all_bytes, offset) = replay.done();
        Ok(all_bytes[offset..].to_vec())
    }

    fn build_gap_fill_message(&mut self, new_seq_no: u64) -> Vec<u8> {
        self.build_admin_message(b"4", |message| {
            message.set(GAP_FILL_FLAG, true);
            message.set(NEW_SEQ_NO, new_seq_no);
        })
    }

    fn validate_begin_string(&self, message: Message<&[u8]>) -> Result<(), EngineError<B::Error>> {
        if !self.config.enforce_begin_string() {
            return Ok(());
        }

        let begin_string =
            required_raw(message, BEGIN_STRING, "BeginString").map_err(EngineError::Protocol)?;
        if begin_string.as_slice() != self.config.begin_string() {
            if begin_string.starts_with(b"FIX.") {
                return Err(EngineError::Protocol(
                    SessionProtocolError::BeginStringMismatch,
                ));
            }
            return Err(EngineError::Protocol(
                SessionProtocolError::MalformedInboundMessage,
            ));
        }
        Ok(())
    }

    fn validate_comp_ids(&self, message: Message<&[u8]>) -> Result<(), EngineError<B::Error>> {
        if !self.config.enforce_comp_id() {
            return Ok(());
        }

        let sender =
            required_raw(message, SENDER_COMP_ID, "SenderCompID").map_err(EngineError::Protocol)?;
        let target =
            required_raw(message, TARGET_COMP_ID, "TargetCompID").map_err(EngineError::Protocol)?;
        if sender.is_empty() {
            return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: SENDER_COMP_ID,
                name: "SenderCompID",
                reason: "tag specified without value",
            }));
        }
        if target.is_empty() {
            return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: TARGET_COMP_ID,
                name: "TargetCompID",
                reason: "tag specified without value",
            }));
        }
        if sender.as_slice() != self.backend.target_comp_id()
            || target.as_slice() != self.backend.sender_comp_id()
        {
            return Err(EngineError::Protocol(SessionProtocolError::CompIdMismatch));
        }
        Ok(())
    }

    fn validate_test_indicator(
        &self,
        message: Message<&[u8]>,
    ) -> Result<(), EngineError<B::Error>> {
        if !self.config.verify_test_indicator() {
            return Ok(());
        }
        let is_test = message
            .get_opt::<bool>(TEST_MESSAGE_INDICATOR)
            .map_err(|_| {
                EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: TEST_MESSAGE_INDICATOR,
                    name: "TestMessageIndicator",
                    reason: "invalid boolean value",
                })
            })?
            .unwrap_or(false);

        if is_test && !self.config.environment().allows_testing() {
            return Err(EngineError::Protocol(
                SessionProtocolError::TestingNotAllowed,
            ));
        }
        Ok(())
    }

    fn validate_sending_time(&self, message: Message<&[u8]>) -> Result<(), EngineError<B::Error>> {
        if !self.config.verify_sending_time() {
            return Ok(());
        }

        let sending_time = message.get::<Timestamp>(SENDING_TIME).map_err(|_| {
            EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: SENDING_TIME,
                name: "SendingTime",
                reason: "invalid UTC timestamp",
            })
        })?;

        #[cfg(feature = "utils-chrono")]
        {
            let sending_time = sending_time.to_chrono_utc().ok_or_else(|| {
                EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: SENDING_TIME,
                    name: "SendingTime",
                    reason: "cannot convert to chrono datetime",
                })
            })?;
            let now = chrono::Utc::now();
            let delta = now
                .signed_duration_since(sending_time)
                .num_milliseconds()
                .abs();
            let max_allowed_ms: i64 = self.config.max_allowed_latency().as_millis() as i64;
            if delta > max_allowed_ms {
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: SENDING_TIME,
                    name: "SendingTime",
                    reason: "outside configured max latency",
                }));
            }
        }

        #[cfg(not(feature = "utils-chrono"))]
        {
            let _ = sending_time;
        }

        Ok(())
    }

    fn validate_possdup_orig_sending_time(
        &self,
        message: Message<&[u8]>,
    ) -> Result<(), EngineError<B::Error>> {
        let sending_time = message.get::<Timestamp>(SENDING_TIME).map_err(|_| {
            EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: SENDING_TIME,
                name: "SendingTime",
                reason: "invalid UTC timestamp",
            })
        })?;
        if message.get_raw(ORIG_SENDING_TIME).is_none() {
            return Err(EngineError::Protocol(SessionProtocolError::MissingField {
                tag: ORIG_SENDING_TIME,
                name: "OrigSendingTime",
            }));
        }
        let orig_sending_time = message.get::<Timestamp>(ORIG_SENDING_TIME).map_err(|_| {
            EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: ORIG_SENDING_TIME,
                name: "OrigSendingTime",
                reason: "invalid UTC timestamp",
            })
        })?;

        #[cfg(feature = "utils-chrono")]
        {
            let sending_time = sending_time.to_chrono_utc().ok_or_else(|| {
                EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: SENDING_TIME,
                    name: "SendingTime",
                    reason: "invalid UTC timestamp",
                })
            })?;
            let orig_sending_time = orig_sending_time.to_chrono_utc().ok_or_else(|| {
                EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: ORIG_SENDING_TIME,
                    name: "OrigSendingTime",
                    reason: "invalid UTC timestamp",
                })
            })?;
            if orig_sending_time > sending_time {
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: SENDING_TIME,
                    name: "SendingTime",
                    reason: "outside configured max latency",
                }));
            }
        }

        #[cfg(not(feature = "utils-chrono"))]
        {
            let _ = (sending_time, orig_sending_time);
        }

        Ok(())
    }

    fn validate_inbound_fields(
        &self,
        message: Message<&[u8]>,
        msg_type: &[u8],
    ) -> Result<(), EngineError<B::Error>> {
        if msg_type.len() != 1 || !msg_type[0].is_ascii_alphanumeric() {
            return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: MSG_TYPE,
                name: "MsgType",
                reason: "invalid msg type",
            }));
        }

        let msg_type_str = std::str::from_utf8(msg_type).map_err(|_| {
            EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: MSG_TYPE,
                name: "MsgType",
                reason: "invalid msg type",
            })
        })?;

        let dict = default_dictionary_for_begin_string(self.config.begin_string());
        let message_def = dict.message_by_msgtype(msg_type_str).ok_or_else(|| {
            EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: MSG_TYPE,
                name: "MsgType",
                reason: "invalid msg type",
            })
        })?;
        let raw_fields = parse_raw_fields(message.as_bytes());

        if let Some(raw) = raw_fields.iter().find(|raw| raw.value.is_empty()) {
            return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                tag: raw.tag_as_u32(),
                name: "Field",
                reason: "tag specified without value",
            }));
        }

        for raw in raw_fields.iter() {
            if raw.tag_i64 <= 0 {
                return Err(EngineError::Protocol(
                    SessionProtocolError::InvalidTagNumber(raw.tag_i64),
                ));
            }
            if raw.tag_i64 > 4999 {
                return Err(EngineError::Protocol(
                    SessionProtocolError::InvalidTagNumber(raw.tag_i64),
                ));
            }
            if dict.field_by_tag(raw.tag_as_u32()).is_none() {
                return Err(EngineError::Protocol(
                    SessionProtocolError::InvalidTagNumber(raw.tag_i64),
                ));
            }
        }

        let (_allowed_tags, repeating_tags) = collect_allowed_and_repeating_tags(&message_def);
        if msg_type == b"0" {
            for raw in raw_fields.iter() {
                let tag = raw.tag_as_u32();
                if is_standard_header_or_trailer_tag(tag) || tag == TEST_REQ_ID {
                    continue;
                }
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag,
                    name: "Field",
                    reason: "tag not defined for this message type",
                }));
            }
        }

        let mut tag_counts: HashMap<u32, usize> = HashMap::new();
        for raw in raw_fields.iter() {
            *tag_counts.entry(raw.tag_as_u32()).or_insert(0) += 1;
        }
        for (tag, count) in tag_counts {
            if msg_type == b"D" && tag == 336 {
                continue;
            }
            if count > 1 && !repeating_tags.contains(&tag) {
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag,
                    name: "Field",
                    reason: "tag appears more than once",
                }));
            }
        }

        if msg_type == b"D" {
            if !raw_fields.iter().any(|raw| raw.tag_as_u32() == 11) {
                return Err(EngineError::Protocol(SessionProtocolError::MissingField {
                    tag: 11,
                    name: "ClOrdID",
                }));
            }

            let first_body_index = raw_fields
                .iter()
                .position(|raw| !is_standard_header_or_trailer_tag(raw.tag_as_u32()));
            let seq_num_index = raw_fields
                .iter()
                .position(|raw| raw.tag_as_u32() == MSG_SEQ_NUM);
            if let (Some(body_index), Some(seq_index)) = (first_body_index, seq_num_index) {
                if seq_index > body_index {
                    return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                        tag: MSG_SEQ_NUM,
                        name: "MsgSeqNum",
                        reason: "tag out of order",
                    }));
                }
            }

            if let Some(group_386) = raw_fields.iter().find(|raw| raw.tag_as_u32() == 386) {
                if let Ok(declared_count) = std::str::from_utf8(group_386.value)
                    .ok()
                    .unwrap_or("")
                    .parse::<usize>()
                {
                    let actual_count = raw_fields
                        .iter()
                        .filter(|field| field.tag_as_u32() == 336)
                        .count();
                    if actual_count != declared_count {
                        return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                            tag: 386,
                            name: "NumInGroup",
                            reason: "incorrect numingroup count",
                        }));
                    }
                }
            }
        }

        for raw in raw_fields.iter() {
            let tag = raw.tag_as_u32();
            if tag == 38 && raw.value.starts_with(b"+") {
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag,
                    name: "OrderQty",
                    reason: "incorrect data format",
                }));
            }
            if tag == 126 && Timestamp::parse(raw.value).is_none() {
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag,
                    name: "ExpireTime",
                    reason: "incorrect data format",
                }));
            }

            if let Some(field) = dict.field_by_tag(tag) {
                if let Some(enum_values) = field.enums() {
                    let valid = enum_values
                        .map(|item| item.value().as_bytes().to_vec())
                        .any(|valid_value| valid_value.as_slice() == raw.value);
                    if !valid {
                        return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                            tag,
                            name: "Field",
                            reason: "incorrect enum value",
                        }));
                    }
                }
            }
        }

        for raw in raw_fields.iter() {
            let group_tag = raw.tag_as_u32();
            let Some(group_tag) = TagU32::new(group_tag) else {
                continue;
            };
            let Some(first_group_field) = message_def.group_info(group_tag) else {
                continue;
            };
            let Ok(declared_count) = std::str::from_utf8(raw.value)
                .ok()
                .unwrap_or("")
                .parse::<usize>()
            else {
                continue;
            };
            let actual_count = raw_fields
                .iter()
                .filter(|f| f.tag_as_u32() == first_group_field.get())
                .count();
            if actual_count != declared_count {
                return Err(EngineError::Protocol(SessionProtocolError::InvalidField {
                    tag: group_tag.get(),
                    name: "NumInGroup",
                    reason: "incorrect numingroup count",
                }));
            }
        }

        Ok(())
    }

    fn protocol_resolution_for_active_error(
        &mut self,
        message: Message<&[u8]>,
        error: &SessionProtocolError,
    ) -> Option<ProtocolResolution> {
        let ref_seq_num = message.get::<u64>(MSG_SEQ_NUM).ok();
        let ref_msg_type = message.get_raw(MSG_TYPE).map(|v| v.to_vec());
        let route_fields = reverse_route_fields(message);

        match error {
            SessionProtocolError::BeginStringMismatch => Some(ProtocolResolution {
                outbound: vec![self.build_logout_message(Some(b"Incorrect BeginString"))],
                reset_heartbeat: false,
                terminate: true,
            }),
            SessionProtocolError::CompIdMismatch => {
                let Some(ref_seq_num) = ref_seq_num else {
                    return None;
                };
                let reject = self.build_reject_message(
                    ref_seq_num,
                    None,
                    ref_msg_type.as_deref(),
                    REJECT_REASON_COMP_ID_PROBLEM,
                    b"CompID problem",
                    route_fields.as_slice(),
                );
                Some(ProtocolResolution {
                    outbound: vec![reject, self.build_logout_message(None)],
                    reset_heartbeat: false,
                    terminate: true,
                })
            }
            SessionProtocolError::UnsupportedMessageType(msg_type) => {
                let Some(ref_seq_num) = ref_seq_num else {
                    return None;
                };
                let reject = self.build_business_reject_message(
                    ref_seq_num,
                    msg_type.as_slice(),
                    BUSINESS_REJECT_REASON_UNSUPPORTED_MESSAGE_TYPE,
                    b"Unsupported Message Type",
                    route_fields.as_slice(),
                );
                Some(ProtocolResolution {
                    outbound: vec![reject],
                    reset_heartbeat: true,
                    terminate: false,
                })
            }
            SessionProtocolError::MissingField { tag, .. } => {
                let Some(ref_seq_num) = ref_seq_num else {
                    return None;
                };
                let reject = self.build_reject_message(
                    ref_seq_num,
                    Some(i64::from(*tag)),
                    ref_msg_type.as_deref(),
                    REJECT_REASON_REQUIRED_TAG_MISSING,
                    b"Required tag missing",
                    route_fields.as_slice(),
                );
                Some(ProtocolResolution {
                    outbound: vec![reject],
                    reset_heartbeat: true,
                    terminate: false,
                })
            }
            SessionProtocolError::InvalidTagNumber(tag) => {
                let Some(ref_seq_num) = ref_seq_num else {
                    return None;
                };
                let reject = self.build_reject_message(
                    ref_seq_num,
                    Some(*tag),
                    ref_msg_type.as_deref(),
                    REJECT_REASON_INVALID_TAG_NUMBER,
                    b"Invalid tag number",
                    route_fields.as_slice(),
                );
                Some(ProtocolResolution {
                    outbound: vec![reject],
                    reset_heartbeat: true,
                    terminate: false,
                })
            }
            SessionProtocolError::InvalidField { tag, reason, .. } => {
                let Some(ref_seq_num) = ref_seq_num else {
                    return None;
                };
                let (reject_reason, text, terminate) = match *reason {
                    "tag specified without value" => (
                        REJECT_REASON_TAG_SPECIFIED_WITHOUT_VALUE,
                        b"Tag specified without a value".as_slice(),
                        false,
                    ),
                    "tag not defined for this message type" => (
                        REJECT_REASON_TAG_NOT_DEFINED_FOR_MESSAGE,
                        b"Tag not defined for this message type".as_slice(),
                        false,
                    ),
                    "incorrect enum value" => (
                        REJECT_REASON_VALUE_OUT_OF_RANGE,
                        b"Value is incorrect (out of range) for this tag".as_slice(),
                        false,
                    ),
                    "value is incorrect (out of range) for this tag" => (
                        REJECT_REASON_VALUE_OUT_OF_RANGE,
                        b"Value is incorrect (out of range) for this tag".as_slice(),
                        false,
                    ),
                    "incorrect data format" => (
                        REJECT_REASON_INCORRECT_DATA_FORMAT,
                        b"Incorrect data format for value".as_slice(),
                        false,
                    ),
                    "invalid msg type" => (
                        REJECT_REASON_INVALID_MSGTYPE,
                        b"Invalid MsgType".as_slice(),
                        false,
                    ),
                    "tag appears more than once" => (
                        REJECT_REASON_TAG_APPEARS_MORE_THAN_ONCE,
                        b"Tag appears more than once".as_slice(),
                        false,
                    ),
                    "tag out of order" => (
                        REJECT_REASON_TAG_OUT_OF_ORDER,
                        b"Tag specified out of required order".as_slice(),
                        false,
                    ),
                    "incorrect numingroup count" => (
                        REJECT_REASON_INCORRECT_NUMINGROUP_COUNT,
                        b"Incorrect NumInGroup count for repeating group".as_slice(),
                        false,
                    ),
                    "outside configured max latency" => (
                        REJECT_REASON_SENDING_TIME_ACCURACY,
                        b"SendingTime accuracy problem".as_slice(),
                        true,
                    ),
                    _ => return None,
                };

                let reject = self.build_reject_message(
                    ref_seq_num,
                    if *reason == "invalid msg type"
                        || *reason == "outside configured max latency"
                        || (*reason == "value is incorrect (out of range) for this tag"
                            && *tag == NEW_SEQ_NO)
                    {
                        None
                    } else {
                        Some(i64::from(*tag))
                    },
                    ref_msg_type.as_deref(),
                    reject_reason,
                    text,
                    route_fields.as_slice(),
                );

                let mut outbound = vec![reject];
                if terminate {
                    outbound.push(self.build_logout_message(None));
                }

                Some(ProtocolResolution {
                    outbound,
                    reset_heartbeat: !terminate,
                    terminate,
                })
            }
            SessionProtocolError::MalformedInboundMessage => Some(ProtocolResolution {
                outbound: vec![],
                reset_heartbeat: false,
                terminate: false,
            }),
            _ => None,
        }
    }

    async fn flush_backend_outbound<O>(&mut self, output: &mut O) -> Result<(), RunError<B::Error>>
    where
        O: AsyncWrite + Unpin,
    {
        loop {
            let next = self.backend.poll_outbound().map_err(RunError::Backend)?;
            let Some(bytes) = next else {
                break;
            };
            let msg_seq_num = extract_msg_seq_num(bytes.as_slice());
            let msg_type = extract_msg_type(bytes.as_slice());
            let should_advance_seq = msg_seq_num == Some(self.state.seq_numbers.next_outbound());
            if should_advance_seq {
                if let (Some(seq_num), Some(msg_type)) = (msg_seq_num, msg_type.as_deref()) {
                    if !is_session_message_type(msg_type) {
                        self.store
                            .save_outbound_app(
                                &self.session_key,
                                StoredAppMessage {
                                    seq_num,
                                    raw_message: bytes.clone(),
                                },
                            )
                            .await
                            .map_err(RunError::Backend)?;
                    }
                }
            }
            let send_result = self.send_raw(output, &bytes).await;
            if should_advance_seq && matches!(send_result, Ok(()) | Err(RunError::Backend(_))) {
                self.state.seq_numbers.incr_outbound();
                self.persist_seq_numbers()
                    .await
                    .map_err(RunError::Backend)?;
            }
            if let Err(err) = send_result {
                return Err(err);
            }
        }
        Ok(())
    }

    async fn send_raw<O>(&mut self, output: &mut O, bytes: &[u8]) -> Result<(), RunError<B::Error>>
    where
        O: AsyncWrite + Unpin,
    {
        output.write_all(bytes).await.map_err(RunError::Io)?;
        self.backend
            .on_outbound_message(bytes)
            .map_err(RunError::Backend)?;
        Ok(())
    }

    fn build_admin_message<F>(&mut self, msg_type: &[u8], f: F) -> Vec<u8>
    where
        F: FnOnce(&mut crate::tagvalue::EncoderHandle<Vec<u8>>),
    {
        self.encode_buffer.clear();
        let outbound_seq_num = self.state.seq_numbers.next_outbound();
        let mut message = self.encoder.start_message(
            self.config.begin_string(),
            &mut self.encode_buffer,
            msg_type,
        );
        message.set(MSG_SEQ_NUM, outbound_seq_num);
        message.set(SENDER_COMP_ID, self.backend.sender_comp_id());
        message.set(SENDING_TIME, Timestamp::utc_now());
        message.set(TARGET_COMP_ID, self.backend.target_comp_id());
        if let Some(message_encoding) = self.backend.message_encoding() {
            message.set(MESSAGE_ENCODING, message_encoding);
        }
        f(&mut message);
        let (all_bytes, offset) = message.done();
        self.state.seq_numbers.incr_outbound();
        all_bytes[offset..].to_vec()
    }

    fn build_logon_message(&mut self, reset_seq_num: bool) -> Vec<u8> {
        let heartbeat_secs = self.state.negotiated_heartbeat.as_secs();
        self.build_admin_message(b"A", |message| {
            message.set(ENCRYPT_METHOD, 0u32);
            message.set(HEARTBT_INT, heartbeat_secs);
            if reset_seq_num {
                message.set(RESET_SEQ_NUM_FLAG, true);
            }
        })
    }

    fn build_heartbeat_message(&mut self, test_req_id: Option<&[u8]>) -> Vec<u8> {
        self.build_admin_message(b"0", |message| {
            if let Some(test_req_id) = test_req_id {
                message.set(TEST_REQ_ID, test_req_id);
            }
        })
    }

    fn build_test_request_message(&mut self) -> Vec<u8> {
        self.state.test_request_counter += 1;
        let test_id = format!("TEST-{}", self.state.test_request_counter);
        self.build_admin_message(b"1", |message| {
            message.set(TEST_REQ_ID, test_id.as_bytes());
        })
    }

    fn build_logout_message(&mut self, text: Option<&[u8]>) -> Vec<u8> {
        self.state.status = SessionStatus::LogoutSent;
        self.build_admin_message(b"5", |message| {
            if let Some(text) = text {
                message.set(TEXT, text);
            }
        })
    }

    fn build_resend_request_message(&mut self, begin: u64, end: u64) -> Vec<u8> {
        self.build_admin_message(b"2", |message| {
            message.set(BEGIN_SEQ_NO, begin);
            message.set(END_SEQ_NO, end);
        })
    }

    #[allow(dead_code)]
    fn build_reject_message(
        &mut self,
        ref_seq_num: u64,
        ref_tag: Option<i64>,
        ref_msg_type: Option<&[u8]>,
        reason: u32,
        text: &[u8],
        route_fields: &[(u32, Vec<u8>)],
    ) -> Vec<u8> {
        let ref_tag_text = ref_tag.map(|tag| tag.to_string());
        self.build_admin_message(b"3", |message| {
            for (tag, value) in route_fields.iter() {
                message.set(*tag, value.as_slice());
            }
            message.set(REF_SEQ_NUM, ref_seq_num);
            message.set(TEXT, text);
            if let Some(ref_tag) = ref_tag_text.as_ref() {
                message.set(REF_TAG_ID, ref_tag.as_bytes());
            }
            if let Some(ref_msg_type) = ref_msg_type {
                message.set(REF_MSG_TYPE, ref_msg_type);
            }
            message.set(SESSION_REJECT_REASON, reason);
        })
    }

    fn build_business_reject_message(
        &mut self,
        ref_seq_num: u64,
        ref_msg_type: &[u8],
        reason: u32,
        text: &[u8],
        route_fields: &[(u32, Vec<u8>)],
    ) -> Vec<u8> {
        self.build_admin_message(b"j", |message| {
            for (tag, value) in route_fields.iter() {
                message.set(*tag, value.as_slice());
            }
            message.set(REF_SEQ_NUM, ref_seq_num);
            message.set(TEXT, text);
            message.set(REF_MSG_TYPE, ref_msg_type);
            message.set(BUSINESS_REJECT_REASON, reason);
        })
    }

    #[allow(dead_code)]
    fn build_reject_for_sending_time(&mut self, ref_seq_num: u64, ref_msg_type: &[u8]) -> Vec<u8> {
        self.build_reject_message(
            ref_seq_num,
            Some(i64::from(SENDING_TIME)),
            Some(ref_msg_type),
            REJECT_REASON_SENDING_TIME_ACCURACY,
            b"Invalid SendingTime(52)",
            &[],
        )
    }
}

fn default_dictionary_for_begin_string(begin_string: &[u8]) -> Dictionary {
    match begin_string {
        #[cfg(feature = "fix40")]
        b"FIX.4.0" => Dictionary::fix40(),
        #[cfg(feature = "fix41")]
        b"FIX.4.1" => Dictionary::fix41(),
        #[cfg(feature = "fix42")]
        b"FIX.4.2" => Dictionary::fix42(),
        #[cfg(feature = "fix43")]
        b"FIX.4.3" => Dictionary::fix43(),
        b"FIX.4.4" => Dictionary::fix44(),
        #[cfg(feature = "fix50")]
        b"FIX.5.0" => Dictionary::fix50(),
        #[cfg(feature = "fix50sp1")]
        b"FIX.5.0-SP1" => Dictionary::fix50sp1(),
        #[cfg(feature = "fix50sp2")]
        b"FIX.5.0-SP2" => Dictionary::fix50sp2(),
        #[cfg(feature = "fixt11")]
        b"FIXT.1.1" => Dictionary::fixt11(),
        _ => Dictionary::fix44(),
    }
}

fn required_raw(
    message: Message<&[u8]>,
    tag: u32,
    name: &'static str,
) -> Result<Vec<u8>, SessionProtocolError> {
    message
        .get_raw(tag)
        .map(|bytes| bytes.to_vec())
        .ok_or(SessionProtocolError::MissingField { tag, name })
}

fn required_u64(
    message: Message<&[u8]>,
    tag: u32,
    name: &'static str,
) -> Result<u64, SessionProtocolError> {
    let raw = message
        .get_raw(tag)
        .ok_or(SessionProtocolError::MissingField { tag, name })?;
    let raw = std::str::from_utf8(raw).map_err(|_| SessionProtocolError::InvalidField {
        tag,
        name,
        reason: "invalid integer",
    })?;
    raw.parse::<u64>()
        .map_err(|_| SessionProtocolError::InvalidField {
            tag,
            name,
            reason: "invalid integer",
        })
}

fn is_session_message_type(msg_type: &[u8]) -> bool {
    matches!(msg_type, b"A" | b"0" | b"1" | b"2" | b"3" | b"4" | b"5")
}

fn protocol_logout_text(error: &SessionProtocolError) -> Option<Vec<u8>> {
    match error {
        SessionProtocolError::MsgSeqNumTooLow { .. } => Some(error.text().into_bytes()),
        _ => None,
    }
}

fn reverse_route_fields(message: Message<&[u8]>) -> Vec<(u32, Vec<u8>)> {
    let mut fields = Vec::new();
    push_route_field(
        message,
        ON_BEHALF_OF_COMP_ID,
        DELIVER_TO_COMP_ID,
        &mut fields,
    );
    push_route_field(message, ON_BEHALF_OF_SUB_ID, DELIVER_TO_SUB_ID, &mut fields);
    push_route_field(
        message,
        ON_BEHALF_OF_LOCATION_ID,
        DELIVER_TO_LOCATION_ID,
        &mut fields,
    );
    push_route_field(
        message,
        DELIVER_TO_COMP_ID,
        ON_BEHALF_OF_COMP_ID,
        &mut fields,
    );
    push_route_field(message, DELIVER_TO_SUB_ID, ON_BEHALF_OF_SUB_ID, &mut fields);
    push_route_field(
        message,
        DELIVER_TO_LOCATION_ID,
        ON_BEHALF_OF_LOCATION_ID,
        &mut fields,
    );
    fields
}

fn push_route_field(
    message: Message<&[u8]>,
    source: u32,
    destination: u32,
    out: &mut Vec<(u32, Vec<u8>)>,
) {
    let Some(value) = message.get_raw(source) else {
        return;
    };
    if value.is_empty() {
        return;
    }
    out.push((destination, value.to_vec()));
}

fn has_malformed_tag_token(bytes: &[u8]) -> bool {
    let separator = if bytes.contains(&b'\x01') {
        b'\x01'
    } else {
        b'|'
    };
    bytes
        .split(|byte| *byte == separator)
        .filter(|raw| !raw.is_empty())
        .any(|raw| {
            let Some(eq_pos) = raw.iter().position(|byte| *byte == b'=') else {
                return true;
            };
            let tag_raw = &raw[..eq_pos];
            if tag_raw.is_empty() {
                return true;
            }
            if tag_raw[0] == b'-' {
                if tag_raw.len() == 1 {
                    return true;
                }
                tag_raw[1..].iter().any(|byte| !byte.is_ascii_digit())
            } else {
                tag_raw.iter().any(|byte| !byte.is_ascii_digit())
            }
        })
}

fn has_ordered_session_header_prefix(bytes: &[u8]) -> bool {
    let separator = if bytes.contains(&b'\x01') {
        b'\x01'
    } else {
        b'|'
    };
    let mut fields = bytes
        .split(|byte| *byte == separator)
        .filter(|raw| !raw.is_empty());
    let Some(first) = fields.next() else {
        return false;
    };
    let Some(second) = fields.next() else {
        return false;
    };
    let Some(third) = fields.next() else {
        return false;
    };
    raw_tag_number(first) == Some(8)
        && raw_tag_number(second) == Some(9)
        && raw_tag_number(third) == Some(35)
}

fn raw_tag_number(field: &[u8]) -> Option<u32> {
    let eq_pos = field.iter().position(|byte| *byte == b'=')?;
    if eq_pos == 0 {
        return None;
    }
    std::str::from_utf8(&field[..eq_pos]).ok()?.parse().ok()
}

fn has_valid_checksum(bytes: &[u8]) -> bool {
    let separator = if bytes.contains(&b'\x01') {
        b'\x01'
    } else {
        b'|'
    };
    let marker = [separator, b'1', b'0', b'='];
    let Some(checksum_field_start) = bytes
        .windows(marker.len())
        .rposition(|window| window == marker)
    else {
        return false;
    };
    let value_start = checksum_field_start + marker.len();
    let value_end = bytes[value_start..]
        .iter()
        .position(|byte| *byte == separator)
        .map(|offset| value_start + offset)
        .unwrap_or(bytes.len());
    let checksum_str = match std::str::from_utf8(&bytes[value_start..value_end]) {
        Ok(value) => value,
        Err(_) => return false,
    };
    let Ok(expected_checksum) = checksum_str.parse::<u32>() else {
        return false;
    };
    if expected_checksum > 255 {
        return false;
    }
    let actual_checksum = bytes[..=checksum_field_start]
        .iter()
        .fold(0u32, |sum, byte| sum + u32::from(*byte))
        % 256;
    expected_checksum == actual_checksum
}

#[derive(Debug, Clone, Copy)]
struct RawField<'a> {
    tag_i64: i64,
    value: &'a [u8],
}

impl<'a> RawField<'a> {
    fn tag_as_u32(self) -> u32 {
        u32::try_from(self.tag_i64).unwrap_or(0)
    }
}

fn parse_raw_fields(bytes: &[u8]) -> Vec<RawField<'_>> {
    let separator = if bytes.contains(&b'\x01') {
        b'\x01'
    } else {
        b'|'
    };
    bytes
        .split(|byte| *byte == separator)
        .filter_map(|raw| {
            if raw.is_empty() {
                return None;
            }
            let mut parts = raw.splitn(2, |byte| *byte == b'=');
            let tag_raw = parts.next()?;
            let value = parts.next().unwrap_or_default();
            let tag_i64 = std::str::from_utf8(tag_raw)
                .ok()
                .and_then(|s| s.parse::<i64>().ok())
                .unwrap_or(-1);
            Some(RawField { tag_i64, value })
        })
        .collect()
}

fn extract_msg_seq_num(bytes: &[u8]) -> Option<u64> {
    parse_raw_fields(bytes)
        .into_iter()
        .find(|field| field.tag_i64 == i64::from(MSG_SEQ_NUM))
        .and_then(|field| std::str::from_utf8(field.value).ok())
        .and_then(|value| value.parse::<u64>().ok())
}

fn extract_msg_type(bytes: &[u8]) -> Option<Vec<u8>> {
    parse_raw_fields(bytes)
        .into_iter()
        .find(|field| field.tag_i64 == i64::from(MSG_TYPE))
        .map(|field| field.value.to_vec())
}

fn extract_raw_field_value(bytes: &[u8], tag: u32) -> Option<Vec<u8>> {
    parse_raw_fields(bytes)
        .into_iter()
        .find(|field| field.tag_i64 == i64::from(tag))
        .map(|field| field.value.to_vec())
}

fn collect_allowed_and_repeating_tags(
    message_def: &crate::dict::Message<'_>,
) -> (HashSet<u32>, HashSet<u32>) {
    let mut allowed_tags = HashSet::new();
    let mut repeating_tags = HashSet::new();
    for item in message_def.layout() {
        collect_layout_item_tags(item, &mut allowed_tags, &mut repeating_tags, false);
    }
    (allowed_tags, repeating_tags)
}

fn collect_layout_item_tags(
    item: crate::dict::LayoutItem<'_>,
    allowed_tags: &mut HashSet<u32>,
    repeating_tags: &mut HashSet<u32>,
    in_repeating_group: bool,
) {
    match item.kind() {
        crate::dict::LayoutItemKind::Field(field) => {
            let tag = field.tag().get();
            allowed_tags.insert(tag);
            if in_repeating_group {
                repeating_tags.insert(tag);
            }
        }
        crate::dict::LayoutItemKind::Group(len_field, items) => {
            allowed_tags.insert(len_field.tag().get());
            for child in items {
                collect_layout_item_tags(child, allowed_tags, repeating_tags, true);
            }
        }
        crate::dict::LayoutItemKind::Component(component) => {
            for child in component.items() {
                collect_layout_item_tags(child, allowed_tags, repeating_tags, in_repeating_group);
            }
        }
    }
}

fn is_standard_header_or_trailer_tag(tag: u32) -> bool {
    matches!(
        tag,
        8 | 9
            | 10
            | 34
            | 35
            | 43
            | 49
            | 50
            | 52
            | 56
            | 57
            | 89
            | 90
            | 91
            | 93
            | 97
            | 115
            | 116
            | 122
            | 128
            | 129
            | 142
            | 143
            | 144
            | 145
            | 212
            | 213
            | 347
            | 369
            | 1128
            | 1129
            | 1137
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::Decoder;
    use crate::GetConfig;
    use futures::executor::block_on;
    use futures::io::{sink, Cursor};
    use std::collections::VecDeque;
    use std::num::NonZeroU64;

    #[derive(Debug, Default)]
    struct TestBackend {
        inbound_app_count: usize,
        fail_on_outbound: bool,
        outbound_queue: VecDeque<Vec<u8>>,
    }

    impl Backend for TestBackend {
        type Error = ();

        fn sender_comp_id(&self) -> &[u8] {
            b"SENDER"
        }

        fn target_comp_id(&self) -> &[u8] {
            b"TARGET"
        }

        fn on_inbound_app_message(&mut self, _message: Message<&[u8]>) -> Result<(), Self::Error> {
            self.inbound_app_count += 1;
            Ok(())
        }

        fn on_outbound_message(&mut self, _message: &[u8]) -> Result<(), Self::Error> {
            if self.fail_on_outbound {
                return Err(());
            }
            Ok(())
        }

        fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }

        fn poll_outbound(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
            Ok(self.outbound_queue.pop_front())
        }
    }

    fn with_decoded_message<R>(
        bytes: &[u8],
        separator: u8,
        f: impl FnOnce(Message<&[u8]>) -> R,
    ) -> R {
        let mut decoder = Decoder::new(Dictionary::fix44());
        decoder.config_mut().separator = separator;
        let message = decoder.decode(bytes).unwrap();
        f(message)
    }

    fn make_inbound_message(
        msg_type: &[u8],
        seq_num: u64,
        extra_fields: &[(u32, &[u8])],
    ) -> Vec<u8> {
        let mut encoder = Encoder::new();
        encoder.config_mut().separator = b'|';
        let mut buffer = Vec::new();
        let mut message = encoder.start_message(b"FIX.4.4", &mut buffer, msg_type);
        message.set(SENDER_COMP_ID, b"TARGET" as &[u8]);
        message.set(TARGET_COMP_ID, b"SENDER" as &[u8]);
        message.set(MSG_SEQ_NUM, seq_num);
        message.set(SENDING_TIME, Timestamp::utc_now());
        for &(tag, value) in extra_fields {
            message.set(tag, value);
        }
        let (all, offset) = message.done();
        all[offset..].to_vec()
    }

    fn make_outbound_app_message(seq_num: u64) -> Vec<u8> {
        let mut encoder = Encoder::new();
        let mut buffer = Vec::new();
        let mut message = encoder.start_message(b"FIX.4.4", &mut buffer, b"D");
        message.set(MSG_SEQ_NUM, seq_num);
        message.set(SENDER_COMP_ID, b"SENDER" as &[u8]);
        message.set(SENDING_TIME, Timestamp::utc_now());
        message.set(TARGET_COMP_ID, b"TARGET" as &[u8]);
        message.set(11, b"ID" as &[u8]);
        message.set(21, b"3" as &[u8]);
        message.set(40, b"1" as &[u8]);
        message.set(54, b"1" as &[u8]);
        message.set(55, b"INTC" as &[u8]);
        message.set(60, Timestamp::utc_now());
        let (all, offset) = message.done();
        all[offset..].to_vec()
    }

    fn default_session_key() -> super::super::SessionKey {
        super::super::SessionKey {
            begin_string: b"FIX.4.4".to_vec(),
            sender_comp_id: b"SENDER".to_vec(),
            target_comp_id: b"TARGET".to_vec(),
        }
    }

    fn new_connection(
        backend: TestBackend,
    ) -> FixConnection<TestBackend, super::super::InMemorySessionStore<()>, super::super::Config>
    {
        FixConnection::new(
            backend,
            super::super::InMemorySessionStore::default(),
            super::super::Config::default(),
        )
    }

    #[test]
    fn high_seqnum_triggers_resend_request() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers = super::super::SeqNumbers::default();

        let inbound = make_inbound_message(b"0", 3, &[]);
        let actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();

        assert_eq!(actions.outbound.len(), 1);
        with_decoded_message(&actions.outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"2");
            assert_eq!(outbound.get::<u64>(BEGIN_SEQ_NO).unwrap(), 1);
            assert_eq!(outbound.get::<u64>(END_SEQ_NO).unwrap(), 0);
        });
    }

    #[test]
    fn test_request_generates_heartbeat_with_test_id() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers = super::super::SeqNumbers::default();

        let inbound = make_inbound_message(b"1", 1, &[(TEST_REQ_ID, b"abc")]);
        let actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert_eq!(actions.outbound.len(), 1);
        with_decoded_message(&actions.outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"0");
            assert_eq!(outbound.get::<&[u8]>(TEST_REQ_ID).unwrap(), b"abc");
        });
    }

    #[test]
    fn logon_transitions_to_active() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::AwaitingLogon;
        connection.state.seq_numbers = super::super::SeqNumbers::default();

        let inbound =
            make_inbound_message(b"A", 1, &[(ENCRYPT_METHOD, b"0"), (HEARTBT_INT, b"30")]);

        let actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert_eq!(connection.state.status, SessionStatus::Active);
        assert!(actions.reset_heartbeat);
    }

    #[test]
    fn resend_request_replays_stored_app_message_as_possdup() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers =
            super::super::SeqNumbers::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(2).unwrap());
        let stored = make_outbound_app_message(1);
        block_on(connection.store.save_outbound_app(
            &connection.session_key,
            StoredAppMessage {
                seq_num: 1,
                raw_message: stored,
            },
        ))
        .unwrap();

        let inbound = make_inbound_message(b"2", 1, &[(BEGIN_SEQ_NO, b"1"), (END_SEQ_NO, b"1")]);
        let mut actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert_eq!(actions.resend_requests, vec![(1, 1)]);
        let mut replay_outbound = Vec::new();
        for (begin_seq_no, end_seq_no) in actions.resend_requests.drain(..) {
            replay_outbound.extend(
                block_on(connection.build_resend_reply_messages(begin_seq_no, end_seq_no)).unwrap(),
            );
        }

        assert_eq!(replay_outbound.len(), 1);
        with_decoded_message(&replay_outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"D");
            assert_eq!(outbound.get::<u64>(MSG_SEQ_NUM).unwrap(), 1);
            assert!(outbound.get::<bool>(POSS_DUP_FLAG).unwrap());
            assert!(outbound.get_raw(ORIG_SENDING_TIME).is_some());
        });
    }

    #[test]
    fn resend_request_missing_admin_seq_emits_gapfill() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers =
            super::super::SeqNumbers::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(2).unwrap());

        let inbound = make_inbound_message(b"2", 1, &[(BEGIN_SEQ_NO, b"1"), (END_SEQ_NO, b"1")]);
        let mut actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert_eq!(actions.resend_requests, vec![(1, 1)]);
        let mut replay_outbound = Vec::new();
        for (begin_seq_no, end_seq_no) in actions.resend_requests.drain(..) {
            replay_outbound.extend(
                block_on(connection.build_resend_reply_messages(begin_seq_no, end_seq_no)).unwrap(),
            );
        }

        assert_eq!(replay_outbound.len(), 1);
        with_decoded_message(&replay_outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"4");
            assert!(outbound.get::<bool>(GAP_FILL_FLAG).unwrap());
            assert_eq!(outbound.get::<u64>(NEW_SEQ_NO).unwrap(), 2);
        });
    }

    #[test]
    fn resend_request_mixed_range_interleaves_gapfill_and_replay_correctly() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers =
            super::super::SeqNumbers::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(5).unwrap());
        for seq in [1_u64, 3_u64] {
            block_on(connection.store.save_outbound_app(
                &connection.session_key,
                StoredAppMessage {
                    seq_num: seq,
                    raw_message: make_outbound_app_message(seq),
                },
            ))
            .unwrap();
        }

        let inbound = make_inbound_message(b"2", 1, &[(BEGIN_SEQ_NO, b"1"), (END_SEQ_NO, b"4")]);
        let mut actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert_eq!(actions.resend_requests, vec![(1, 4)]);
        let mut replay_outbound = Vec::new();
        for (begin_seq_no, end_seq_no) in actions.resend_requests.drain(..) {
            replay_outbound.extend(
                block_on(connection.build_resend_reply_messages(begin_seq_no, end_seq_no)).unwrap(),
            );
        }

        assert_eq!(replay_outbound.len(), 4);
        with_decoded_message(&replay_outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"D");
            assert_eq!(outbound.get::<u64>(MSG_SEQ_NUM).unwrap(), 1);
        });
        with_decoded_message(&replay_outbound[1], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"4");
            assert_eq!(outbound.get::<u64>(NEW_SEQ_NO).unwrap(), 3);
        });
        with_decoded_message(&replay_outbound[2], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"D");
            assert_eq!(outbound.get::<u64>(MSG_SEQ_NUM).unwrap(), 3);
        });
        with_decoded_message(&replay_outbound[3], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"4");
            assert_eq!(outbound.get::<u64>(NEW_SEQ_NO).unwrap(), 5);
        });
    }

    #[test]
    fn resend_request_endseqno_zero_caps_to_last_sent() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers =
            super::super::SeqNumbers::new(NonZeroU64::new(1).unwrap(), NonZeroU64::new(5).unwrap());
        block_on(connection.store.save_outbound_app(
            &connection.session_key,
            StoredAppMessage {
                seq_num: 2,
                raw_message: make_outbound_app_message(2),
            },
        ))
        .unwrap();

        let inbound = make_inbound_message(b"2", 1, &[(BEGIN_SEQ_NO, b"2"), (END_SEQ_NO, b"0")]);
        let mut actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert_eq!(actions.resend_requests, vec![(2, 0)]);
        let mut replay_outbound = Vec::new();
        for (begin_seq_no, end_seq_no) in actions.resend_requests.drain(..) {
            replay_outbound.extend(
                block_on(connection.build_resend_reply_messages(begin_seq_no, end_seq_no)).unwrap(),
            );
        }

        assert_eq!(replay_outbound.len(), 2);
        with_decoded_message(&replay_outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"D");
            assert_eq!(outbound.get::<u64>(MSG_SEQ_NUM).unwrap(), 2);
        });
        with_decoded_message(&replay_outbound[1], b'\x01', |outbound| {
            assert_eq!(outbound.get::<&[u8]>(MSG_TYPE).unwrap(), b"4");
            assert_eq!(outbound.get::<u64>(NEW_SEQ_NO).unwrap(), 5);
        });
    }

    #[test]
    fn restart_loads_seq_numbers_from_store() {
        let mut store = super::super::InMemorySessionStore::<()>::default();
        let restored =
            super::super::SeqNumbers::new(NonZeroU64::new(7).unwrap(), NonZeroU64::new(9).unwrap());
        block_on(store.save_seq_numbers(&default_session_key(), restored)).unwrap();

        let mut connection = FixConnection::new(
            TestBackend::default(),
            store,
            super::super::Config::default(),
        );
        let result = block_on(connection.run(Cursor::new(Vec::<u8>::new()), sink()));
        assert!(!matches!(result, Err(RunError::Backend(_))));
        assert_eq!(connection.state.seq_numbers.next_inbound(), 7);
        assert_eq!(
            connection.state.seq_numbers.next_outbound(),
            restored.next_outbound() + 1
        );
    }

    #[test]
    fn reset_seqnum_flag_resets_store_and_runtime_seq() {
        let mut connection = new_connection(TestBackend::default());
        connection.state.status = SessionStatus::AwaitingLogon;
        connection.state.seq_numbers = super::super::SeqNumbers::new(
            NonZeroU64::new(11).unwrap(),
            NonZeroU64::new(13).unwrap(),
        );
        block_on(connection.store.save_outbound_app(
            &connection.session_key,
            StoredAppMessage {
                seq_num: 11,
                raw_message: make_outbound_app_message(11),
            },
        ))
        .unwrap();
        block_on(
            connection
                .store
                .save_seq_numbers(&connection.session_key, connection.state.seq_numbers),
        )
        .unwrap();

        let inbound = make_inbound_message(
            b"A",
            1,
            &[
                (ENCRYPT_METHOD, b"0"),
                (HEARTBT_INT, b"30"),
                (RESET_SEQ_NUM_FLAG, b"Y"),
            ],
        );
        let actions = with_decoded_message(&inbound, b'|', |message| {
            connection.handle_inbound_message(message)
        })
        .unwrap();
        assert!(actions.reset_store);
        if actions.reset_store {
            block_on(
                connection
                    .store
                    .reset_session(&connection.session_key, connection.state.seq_numbers),
            )
            .unwrap();
        }

        assert_eq!(connection.state.seq_numbers.next_inbound(), 2);
        assert_eq!(connection.state.seq_numbers.next_outbound(), 2);
        assert!(block_on(connection.store.load_outbound_app_range(
            &connection.session_key,
            1,
            100,
        ))
        .unwrap()
        .is_empty());
    }

    #[test]
    fn persist_before_send_crash_model_at_least_once() {
        let mut backend = TestBackend::default();
        backend.fail_on_outbound = true;
        backend
            .outbound_queue
            .push_back(make_outbound_app_message(1));
        let mut connection = new_connection(backend);
        connection.state.status = SessionStatus::Active;
        connection.state.seq_numbers = super::super::SeqNumbers::default();

        let result = block_on(connection.flush_backend_outbound(&mut sink()));
        assert!(matches!(result, Err(RunError::Backend(()))));

        let persisted = block_on(connection.store.load_outbound_app_range(
            &connection.session_key,
            1,
            1,
        ))
        .unwrap();
        assert_eq!(persisted.len(), 1);

        let store = connection.store;
        let mut restarted = FixConnection::new(
            TestBackend::default(),
            store,
            super::super::Config::default(),
        );
        restarted.state.status = SessionStatus::Active;
        if let Some(loaded_seq) =
            block_on(restarted.store.load_seq_numbers(&restarted.session_key)).unwrap()
        {
            restarted.state.seq_numbers = loaded_seq;
        }
        let inbound = make_inbound_message(b"2", 1, &[(BEGIN_SEQ_NO, b"1"), (END_SEQ_NO, b"1")]);
        let mut actions = with_decoded_message(&inbound, b'|', |message| {
            restarted.handle_inbound_message(message)
        })
        .unwrap();
        let mut replay_outbound = Vec::new();
        for (begin_seq_no, end_seq_no) in actions.resend_requests.drain(..) {
            replay_outbound.extend(
                block_on(restarted.build_resend_reply_messages(begin_seq_no, end_seq_no)).unwrap(),
            );
        }
        assert_eq!(replay_outbound.len(), 1);
        with_decoded_message(&replay_outbound[0], b'\x01', |outbound| {
            assert_eq!(outbound.get::<u64>(MSG_SEQ_NUM).unwrap(), 1);
            assert!(outbound.get::<bool>(POSS_DUP_FLAG).unwrap());
        });
    }
}
