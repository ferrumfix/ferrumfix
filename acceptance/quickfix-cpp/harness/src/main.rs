use fefix::field_types::Timestamp;
use fefix::session::{
    Backend, Config, Environment, FixConnection, HeartbeatRule, RunError, RunOutcome, SessionRole,
};
use fefix::tagvalue::{Encoder, Message};
use fefix::{FieldMap, SetField};
use std::collections::{BTreeMap, VecDeque};
use std::env;
use std::io;
use std::ops::Range;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const ENV_BIND_HOST: &str = "FFX_QF_BIND_HOST";
const ENV_BIND_PORT: &str = "FFX_QF_BIND_PORT";
const ENV_ROLE: &str = "FFX_QF_ROLE";
const ENV_BEGIN_STRING: &str = "FFX_QF_BEGIN_STRING";
const ENV_SENDER_COMP_ID: &str = "FFX_QF_SENDER_COMP_ID";
const ENV_TARGET_COMP_ID: &str = "FFX_QF_TARGET_COMP_ID";
const ENV_HEARTBEAT_SECS: &str = "FFX_QF_HEARTBEAT_SECS";
const ENV_HEARTBEAT_RULE: &str = "FFX_QF_HEARTBEAT_RULE";
const ENV_VERIFY_SENDING_TIME: &str = "FFX_QF_VERIFY_SENDING_TIME";
const ENV_VERIFY_TEST_INDICATOR: &str = "FFX_QF_VERIFY_TEST_INDICATOR";
const ENV_ENFORCE_BEGIN_STRING: &str = "FFX_QF_ENFORCE_BEGIN_STRING";
const ENV_ENFORCE_COMP_ID: &str = "FFX_QF_ENFORCE_COMP_ID";
const ENV_ALLOW_TEST_MESSAGES: &str = "FFX_QF_ALLOW_TEST_MESSAGES";
const ENV_MAX_ALLOWED_LATENCY_MS: &str = "FFX_QF_MAX_ALLOWED_LATENCY_MS";
const ENV_MAX_SESSIONS: &str = "FFX_QF_MAX_SESSIONS";
const ENV_CONTINUE_ON_PROTOCOL_VIOLATION: &str = "FFX_QF_CONTINUE_ON_PROTOCOL_VIOLATION";
const ENV_CONTINUE_ON_IO_ERROR: &str = "FFX_QF_CONTINUE_ON_IO_ERROR";
const ENV_TRACE_WIRE: &str = "FFX_QF_TRACE_WIRE";

#[derive(Debug)]
struct HarnessOptions {
    bind_host: String,
    bind_port: u16,
    role: SessionRole,
    begin_string: String,
    sender_comp_id: Vec<u8>,
    target_comp_id: Vec<u8>,
    heartbeat: Duration,
    heartbeat_rule: HeartbeatRule,
    verify_sending_time: bool,
    verify_test_indicator: bool,
    enforce_begin_string: bool,
    enforce_comp_id: bool,
    allow_test_messages: bool,
    max_allowed_latency: Duration,
    max_sessions: usize,
    continue_on_protocol_violation: bool,
    continue_on_io_error: bool,
    trace_wire: bool,
}

impl HarnessOptions {
    fn from_env() -> Result<Self, io::Error> {
        let bind_host = env_string(ENV_BIND_HOST, "127.0.0.1");
        let bind_port = env_parse_u16(ENV_BIND_PORT, 7001)?;
        let role = parse_role(&env_string(ENV_ROLE, "acceptor"))?;
        let begin_string = env_string(ENV_BEGIN_STRING, "FIX.4.4");
        let sender_comp_id = env_string(ENV_SENDER_COMP_ID, "ISLD").into_bytes();
        let target_comp_id = env_string(ENV_TARGET_COMP_ID, "TW").into_bytes();
        let heartbeat_secs = env_parse_u64(ENV_HEARTBEAT_SECS, 30)?;
        let heartbeat = Duration::from_secs(heartbeat_secs);
        let heartbeat_rule =
            parse_heartbeat_rule(&env_string(ENV_HEARTBEAT_RULE, "any"), heartbeat)?;
        let verify_sending_time = env_parse_bool(ENV_VERIFY_SENDING_TIME, true)?;
        let verify_test_indicator = env_parse_bool(ENV_VERIFY_TEST_INDICATOR, true)?;
        let enforce_begin_string = env_parse_bool(ENV_ENFORCE_BEGIN_STRING, true)?;
        let enforce_comp_id = env_parse_bool(ENV_ENFORCE_COMP_ID, true)?;
        let allow_test_messages = env_parse_bool(ENV_ALLOW_TEST_MESSAGES, true)?;
        let max_allowed_latency_ms = env_parse_u64(ENV_MAX_ALLOWED_LATENCY_MS, 3_000)?;
        let max_sessions = env_parse_usize(ENV_MAX_SESSIONS, 0)?;
        let continue_on_protocol_violation =
            env_parse_bool(ENV_CONTINUE_ON_PROTOCOL_VIOLATION, true)?;
        let continue_on_io_error = env_parse_bool(ENV_CONTINUE_ON_IO_ERROR, true)?;
        let trace_wire = env_parse_bool(ENV_TRACE_WIRE, false)?;

        Ok(Self {
            bind_host,
            bind_port,
            role,
            begin_string,
            sender_comp_id,
            target_comp_id,
            heartbeat,
            heartbeat_rule,
            verify_sending_time,
            verify_test_indicator,
            enforce_begin_string,
            enforce_comp_id,
            allow_test_messages,
            max_allowed_latency: Duration::from_millis(max_allowed_latency_ms),
            max_sessions,
            continue_on_protocol_violation,
            continue_on_io_error,
            trace_wire,
        })
    }

    fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.bind_port)
    }

    fn to_config(&self) -> Config {
        let mut config = Config::default();
        config.role = self.role;
        config.begin_string = self.begin_string.clone();
        config.heartbeat = self.heartbeat;
        config.heartbeat_rule = self.heartbeat_rule.clone();
        config.verify_sending_time = self.verify_sending_time;
        config.verify_test_indicator = self.verify_test_indicator;
        config.enforce_begin_string = self.enforce_begin_string;
        config.enforce_comp_id = self.enforce_comp_id;
        config.max_allowed_latency = self.max_allowed_latency;
        config.environment = Environment::Production {
            allow_test: self.allow_test_messages,
        };
        config
    }
}

#[derive(Debug)]
struct AcceptanceBackend {
    sender_comp_id: Vec<u8>,
    target_comp_id: Vec<u8>,
    trace_wire: bool,
    next_outbound_seq: u64,
    outbound_queue: VecDeque<Vec<u8>>,
    sent_app_messages: BTreeMap<u64, Vec<u8>>,
    inbound_app_messages: usize,
    successful_handshakes: usize,
    resend_requests: Vec<Range<u64>>,
}

impl AcceptanceBackend {
    fn new(sender_comp_id: Vec<u8>, target_comp_id: Vec<u8>, trace_wire: bool) -> Self {
        Self {
            sender_comp_id,
            target_comp_id,
            trace_wire,
            next_outbound_seq: 1,
            outbound_queue: VecDeque::new(),
            sent_app_messages: BTreeMap::new(),
            inbound_app_messages: 0,
            successful_handshakes: 0,
            resend_requests: Vec::new(),
        }
    }
}

impl Backend for AcceptanceBackend {
    type Error = io::Error;

    fn sender_comp_id(&self) -> &[u8] {
        self.sender_comp_id.as_slice()
    }

    fn target_comp_id(&self) -> &[u8] {
        self.target_comp_id.as_slice()
    }

    fn on_inbound_message(
        &mut self,
        message: Message<&[u8]>,
        is_app: bool,
    ) -> Result<(), Self::Error> {
        if self.trace_wire {
            eprintln!(
                "[harness] << {} {}",
                if is_app { "app" } else { "session" },
                pretty_fix(message.as_bytes())
            );
        }
        if is_app {
            self.inbound_app_messages += 1;
            if let Some(reflected) = reflected_app_message(
                message,
                self.sender_comp_id.as_slice(),
                self.target_comp_id.as_slice(),
                self.next_outbound_seq,
            ) {
                if let Some(seq) = msg_seq_num(reflected.as_slice()) {
                    self.next_outbound_seq = self.next_outbound_seq.max(seq.saturating_add(1));
                    self.sent_app_messages.insert(seq, reflected.clone());
                }
                self.outbound_queue.push_back(reflected);
            }
        }
        Ok(())
    }

    fn on_inbound_app_message(&mut self, _message: Message<&[u8]>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        if let Some(seq) = msg_seq_num(message) {
            self.next_outbound_seq = self.next_outbound_seq.max(seq.saturating_add(1));
        }
        if self.trace_wire {
            eprintln!("[harness] >> {}", pretty_fix(message));
        }
        Ok(())
    }

    fn on_resend_request(&mut self, range: Range<u64>) -> Result<(), Self::Error> {
        let replay_end = if range.end == u64::MAX {
            u64::MAX
        } else {
            range.end.saturating_add(1)
        };
        for (_seq, sent_message) in self.sent_app_messages.range(range.start..replay_end) {
            if let Some(replayed) = possdup_replay_message(sent_message.as_slice()) {
                self.outbound_queue.push_back(replayed);
            }
        }
        self.resend_requests.push(range);
        Ok(())
    }

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
        self.successful_handshakes += 1;
        Ok(())
    }

    fn poll_outbound(&mut self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(self.outbound_queue.pop_front())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = HarnessOptions::from_env()?;
    let listener = TcpListener::bind(options.bind_addr().as_str()).await?;

    eprintln!(
        "[harness] listening on {} as {:?} begin_string={} sender={} target={} max_sessions={}",
        options.bind_addr(),
        options.role,
        options.begin_string,
        String::from_utf8_lossy(&options.sender_comp_id),
        String::from_utf8_lossy(&options.target_comp_id),
        options.max_sessions
    );

    let mut sessions_handled = 0usize;
    loop {
        if options.max_sessions != 0 && sessions_handled >= options.max_sessions {
            eprintln!(
                "[harness] reached max sessions ({}), exiting",
                options.max_sessions
            );
            return Ok(());
        }

        let (stream, peer_addr) = listener.accept().await?;
        sessions_handled += 1;
        eprintln!(
            "[harness] accepted session {} from {}",
            sessions_handled, peer_addr
        );

        let backend = AcceptanceBackend::new(
            options.sender_comp_id.clone(),
            options.target_comp_id.clone(),
            options.trace_wire,
        );
        let config = options.to_config();
        let mut connection = FixConnection::new(backend, config);

        let (read_half, write_half) = stream.into_split();
        let run_result = connection
            .run(read_half.compat(), write_half.compat_write())
            .await;

        let backend = connection.backend();
        eprintln!(
            "[harness] session {} stats: handshakes={} inbound_app_messages={} resend_requests={}",
            sessions_handled,
            backend.successful_handshakes,
            backend.inbound_app_messages,
            backend.resend_requests.len()
        );

        match run_result {
            Ok(outcome) => {
                eprintln!(
                    "[harness] session {} outcome: {:?}",
                    sessions_handled, outcome
                );
                if matches!(outcome, RunOutcome::ProtocolViolation(_))
                    && !options.continue_on_protocol_violation
                {
                    return Err(io::Error::other(
                        "protocol violation and FFX_QF_CONTINUE_ON_PROTOCOL_VIOLATION=0",
                    )
                    .into());
                }
            }
            Err(RunError::Io(err)) => {
                eprintln!(
                    "[harness] session {} io error: {} ({:?})",
                    sessions_handled,
                    err,
                    err.kind()
                );
                if !options.continue_on_io_error {
                    return Err(io::Error::new(
                        err.kind(),
                        format!("session {} I/O error: {err}", sessions_handled),
                    )
                    .into());
                }
            }
            Err(RunError::Backend(err)) => {
                return Err(io::Error::other(format!(
                    "session {} backend error: {err}",
                    sessions_handled
                ))
                .into());
            }
        }
    }
}

fn env_string(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}

fn env_parse_bool(name: &str, default: bool) -> Result<bool, io::Error> {
    let raw = env::var(name).unwrap_or_else(|_| if default { "1" } else { "0" }.to_string());
    parse_bool(&raw).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid boolean for {name}: {raw}"),
        )
    })
}

fn env_parse_u16(name: &str, default: u16) -> Result<u16, io::Error> {
    let raw = env::var(name).unwrap_or_else(|_| default.to_string());
    raw.parse::<u16>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid u16 for {name}: {raw}"),
        )
    })
}

fn env_parse_u64(name: &str, default: u64) -> Result<u64, io::Error> {
    let raw = env::var(name).unwrap_or_else(|_| default.to_string());
    raw.parse::<u64>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid u64 for {name}: {raw}"),
        )
    })
}

fn env_parse_usize(name: &str, default: usize) -> Result<usize, io::Error> {
    let raw = env::var(name).unwrap_or_else(|_| default.to_string());
    raw.parse::<usize>().map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid usize for {name}: {raw}"),
        )
    })
}

fn parse_role(value: &str) -> Result<SessionRole, io::Error> {
    match value.to_ascii_lowercase().as_str() {
        "initiator" => Ok(SessionRole::Initiator),
        "acceptor" => Ok(SessionRole::Acceptor),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported {ENV_ROLE} value: {value}"),
        )),
    }
}

fn parse_heartbeat_rule(value: &str, heartbeat: Duration) -> Result<HeartbeatRule, io::Error> {
    match value.to_ascii_lowercase().as_str() {
        "exact" => Ok(HeartbeatRule::Exact(heartbeat)),
        "any" => Ok(HeartbeatRule::Any),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("unsupported {ENV_HEARTBEAT_RULE} value: {value}"),
        )),
    }
}

fn pretty_fix(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    for b in bytes {
        if *b == 0x01 {
            out.push('|');
        } else {
            out.push(*b as char);
        }
    }
    out
}

fn reflected_app_message(
    message: Message<&[u8]>,
    sender_comp_id: &[u8],
    target_comp_id: &[u8],
    outbound_seq_num: u64,
) -> Option<Vec<u8>> {
    let begin_string = message.get_raw(8)?;
    let msg_type = message.get_raw(35)?;

    let mut encoder = Encoder::new();
    let mut buf = Vec::new();
    let mut outbound = encoder.start_message(begin_string, &mut buf, msg_type);
    outbound.set(34, outbound_seq_num);
    outbound.set(49, sender_comp_id);
    outbound.set(52, Timestamp::utc_now());
    outbound.set(56, target_comp_id);

    for raw in message
        .as_bytes()
        .split(|byte| *byte == 0x01)
        .filter(|raw| !raw.is_empty())
    {
        let Some(eq_pos) = raw.iter().position(|byte| *byte == b'=') else {
            continue;
        };
        let Ok(tag) = std::str::from_utf8(&raw[..eq_pos])
            .ok()
            .and_then(|value| value.parse::<u32>().ok())
            .ok_or(())
        else {
            continue;
        };
        if matches!(tag, 8 | 9 | 10 | 34 | 35 | 43 | 49 | 52 | 56 | 122) {
            continue;
        }
        outbound.set(tag, &raw[eq_pos + 1..]);
    }

    let (all_bytes, offset) = outbound.done();
    Some(all_bytes[offset..].to_vec())
}

fn possdup_replay_message(sent_message: &[u8]) -> Option<Vec<u8>> {
    let fields = parse_raw_fields(sent_message);
    let begin_string = fields.iter().find(|(tag, _)| *tag == 8)?.1;
    let msg_type = fields.iter().find(|(tag, _)| *tag == 35)?.1;
    let msg_seq_num = fields
        .iter()
        .find(|(tag, _)| *tag == 34)
        .and_then(|(_, value)| std::str::from_utf8(value).ok())
        .and_then(|value| value.parse::<u64>().ok())?;
    let original_sending_time = fields.iter().find(|(tag, _)| *tag == 52)?.1;

    let mut encoder = Encoder::new();
    let mut buf = Vec::new();
    let mut outbound = encoder.start_message(begin_string, &mut buf, msg_type);
    outbound.set(34, msg_seq_num);
    outbound.set(43, true);
    outbound.set(49, fields.iter().find(|(tag, _)| *tag == 49)?.1);
    outbound.set(52, Timestamp::utc_now());
    outbound.set(56, fields.iter().find(|(tag, _)| *tag == 56)?.1);
    outbound.set(122, original_sending_time);

    for (tag, value) in fields.iter().copied() {
        if matches!(tag, 8 | 9 | 10 | 34 | 35 | 43 | 49 | 52 | 56 | 122) {
            continue;
        }
        outbound.set(tag, value);
    }

    let (all_bytes, offset) = outbound.done();
    Some(all_bytes[offset..].to_vec())
}

fn parse_raw_fields(bytes: &[u8]) -> Vec<(u32, &[u8])> {
    bytes
        .split(|byte| *byte == 0x01)
        .filter_map(|raw| {
            if raw.is_empty() {
                return None;
            }
            let eq_pos = raw.iter().position(|byte| *byte == b'=')?;
            let tag = std::str::from_utf8(&raw[..eq_pos]).ok()?.parse::<u32>().ok()?;
            Some((tag, &raw[eq_pos + 1..]))
        })
        .collect()
}

fn msg_seq_num(bytes: &[u8]) -> Option<u64> {
    parse_raw_fields(bytes)
        .into_iter()
        .find(|(tag, _)| *tag == 34)
        .and_then(|(_, value)| std::str::from_utf8(value).ok())
        .and_then(|value| value.parse::<u64>().ok())
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}
