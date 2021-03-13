use super::{errs, Environment, HeartbeatRule, SeqNumberError, SeqNumbers};
use crate::tags::fixt11 as tags;
use crate::tagvalue::{FixFieldValue, MessageRnd};
use std::time::Duration;

/// FIX Session Layer configuration, for acceptors only.
#[derive(Debug, Clone)]
pub struct Configuration {
    heartbeat_rule: HeartbeatRule,
    delivery_threshold: Duration,
    company_id: String,
    environment: Environment,
}

impl Configuration {
    pub fn new(company_id: String) -> Self {
        Self {
            heartbeat_rule: HeartbeatRule::Any,
            delivery_threshold: Duration::from_secs(60),
            company_id,
            environment: Environment::ProductionDisallowTest,
        }
    }

    /// Decide whether or not to allow test messages from initiators.
    ///
    /// Please refer to specs. §4.3.2 for more information.
    pub fn with_environment(&mut self, env: Environment) -> &mut Self {
        self.environment = env;
        self
    }

    /// Puts in place a custom restriction for the `HeartBeat` value. This is
    /// [`HeartbeatRule::Any`](HeartbeatRule::Any) by default by default.
    pub fn with_hb_rule(&mut self, rule: HeartbeatRule) -> &mut Self {
        self.heartbeat_rule = rule;
        self
    }

    /// Sets the "CompID" of the acceptor.
    pub fn with_company_id(&mut self, id: String) -> &mut Self {
        self.company_id = id;
        self
    }

    /// Builds a new [`Acceptor`] and uses `self` to configure it.
    pub fn acceptor(self) -> Acceptor {
        Acceptor::new(self)
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EventInbound {
    HeartbeatIsDue,
    IncomingMessage(MessageRnd),
    Terminated,
}

#[derive(Clone, Debug)]
pub enum EventOutbound {
    Terminate,
    Message(MessageRnd),
}

/// A FIX Session acceptor.
#[derive(Debug, Clone)]
pub struct Acceptor {
    config: Configuration,
    state: State,
    heartbeat: Duration,
    seq_numbers: SeqNumbers,
}

impl Acceptor {
    /// Creates a new FIX session acceptor based on the given configuration.
    pub fn new(config: Configuration) -> Self {
        Acceptor {
            config,
            state: State::Disconnected,
            heartbeat: Duration::default(),
            seq_numbers: SeqNumbers::default(),
        }
    }

    /// Get the expected heartbeat interval on the underlying FIX connection.
    pub fn heartbeat(&self) -> Option<Duration> {
        match self.state {
            State::Active => Some(self.heartbeat),
            _ => None,
        }
    }

    /// Returns the internal seq. numbers state.
    pub fn seq_numbers(&self) -> SeqNumbers {
        self.seq_numbers
    }

    /// Notifies `self` about an event on the underlying FIX session and
    /// returns an [`Iterator`] of the ountbound response events.
    pub fn notify(&mut self, event: EventInbound) -> impl Iterator<Item = EventOutbound> {
        self.notify_to_vec(event).into_iter()
    }

    pub fn notify_to_vec(&mut self, event: EventInbound) -> Vec<EventOutbound> {
        let mut outbound_events = Vec::default();
        match event {
            EventInbound::Terminated => {
                // We also disconnect then.
                outbound_events.push(EventOutbound::Terminate);
            }
            EventInbound::IncomingMessage(msg) => {
                self.feed_incoming_message(msg, &mut outbound_events);
            }
            EventInbound::HeartbeatIsDue => {
                let heartbeat = self.generate_heartbeat_message();
                outbound_events.push(EventOutbound::Message(heartbeat));
            }
        };
        outbound_events
    }

    fn feed_incoming_message(&mut self, message: MessageRnd, to: &mut Vec<EventOutbound>) {
        let msg_type = message.msg_type();
        // Check `TestMessageIndicator(464)`.
        match (self.config.environment, message.test_indicator()) {
            (Environment::ProductionDisallowTest, Some(true)) => {
                // Generate Logout!
                let mut msg = MessageRnd::new();
                msg.add_str(tags::MSG_TYPE, "5");
                msg.add_str(tags::SENDER_COMP_ID, self.config.company_id.as_str());
                msg.add_int(tags::BEGIN_SEQ_NO, self.seq_numbers().next_inbound() as i64);
                msg.add_int(tags::END_SEQ_NO, message.seq_num().unwrap() as i64);
                msg.add_str(tags::TEXT, errs::production_env());
                to.push(EventOutbound::Message(add_time_to_msg(msg)));
                return;
            }
            _ => (),
        };
        // Compare seq. numbers.
        let seqnum_state = message
            .seq_num()
            .map(|seqnum| self.seq_numbers().validate_inbound(seqnum))
            .unwrap_or(Err(SeqNumberError::NoSeqNum));
        // Compare the incoming seq. number to the one we expected and act
        // accordingly.
        match seqnum_state {
            Ok(()) => {}
            // See §4.5.3.
            Err(SeqNumberError::NoSeqNum) => {
                // Generate Logout!
                let mut msg = MessageRnd::new();
                msg.add_str(tags::MSG_TYPE, "5");
                msg.add_str(tags::SENDER_COMP_ID, self.config.company_id.as_str());
                msg.add_int(tags::BEGIN_SEQ_NO, self.seq_numbers().next_inbound() as i64);
                msg.add_int(tags::END_SEQ_NO, message.seq_num().unwrap() as i64);
                msg.add_str(
                    tags::TEXT,
                    errs::missing_field("MsgSeqNum", tags::MSG_SEQ_NUM),
                );
                to.push(EventOutbound::Message(add_time_to_msg(msg)));
                return;
            }
            // Refer to specs. §4.8 for more information.
            Err(SeqNumberError::Recover) => {
                // Begin message recovery.
                let mut response = MessageRnd::new();
                response.add_str(tags::MSG_TYPE, "2");
                response.add_str(tags::SENDER_COMP_ID, self.config.company_id.as_str());
                response.add_int(tags::BEGIN_SEQ_NO, self.seq_numbers().next_inbound() as i64);
                response.add_int(tags::END_SEQ_NO, message.seq_num().unwrap() as i64);
                self.seq_numbers.incr_outbound();
                // TODO: add other details to response message.
                to.push(EventOutbound::Message(add_time_to_msg(response)));
                return;
            }
            Err(SeqNumberError::TooLow) => {
                let msg = self.generate_error_seqnum_too_low();
                to.push(EventOutbound::Message(add_time_to_msg(msg)));
            }
        };
        if self.state == State::Disconnected && msg_type != Some("A") {
            to.push(EventOutbound::Terminate);
            return;
        }
        // Logon <A>
        if let Some("A") = msg_type {
            if self.state == State::Active {
                return;
            }
            let mut response = MessageRnd::new();
            // TODO: add other details to response message.
            response.add_field(tags::MSG_TYPE, FixFieldValue::string(b"A").unwrap());
            response.add_field(
                tags::SENDER_COMP_ID,
                FixFieldValue::string(self.config.company_id.as_bytes()).unwrap(),
            );
            self.seq_numbers.incr_outbound();
            to.push(EventOutbound::Message(add_time_to_msg(response)));
            self.state = State::Active;
        }
    }

    fn generate_error_seqnum_too_low(&mut self) -> MessageRnd {
        let error_message = errs::msg_seq_num(self.seq_numbers().next_inbound());
        let mut response = MessageRnd::new();
        response.add_str(tags::MSG_TYPE, "5");
        response.add_str(tags::SENDER_COMP_ID, self.config.company_id.as_str());
        response.add_int(
            tags::TARGET_COMP_ID,
            self.seq_numbers().next_outbound() as i64,
        );
        response.add_str(tags::TEXT, error_message);
        add_time_to_msg(response)
    }

    fn generate_heartbeat_message(&mut self) -> MessageRnd {
        let mut heartbeat = MessageRnd::new();
        heartbeat.add_str(tags::MSG_TYPE, "0");
        heartbeat.add_str(tags::SENDER_COMP_ID, self.config.company_id.as_str());
        heartbeat.add_int(
            tags::BEGIN_SEQ_NO,
            self.seq_numbers().next_outbound() as i64,
        );
        add_time_to_msg(heartbeat)
    }
}

fn add_time_to_msg(mut msg: MessageRnd) -> MessageRnd {
    // https://www.onixs.biz/fix-dictionary/4.4/index.html#UTCTimestamp.
    let time = chrono::Utc::now();
    let timestamp = time.format("%Y%m%d-%H:%M:%S.%.3f");
    msg.add_str(tags::SENDING_TIME, timestamp.to_string());
    msg
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum State {
    /// No FIX Session currently active.
    Disconnected,
    /// The FIX Session is over, we just need to terminate the transport layer
    /// connection.
    #[allow(dead_code)]
    Terminate,
    /// The FIX Session is completely over. Nothing left to do.
    Terminated,
    /// Normal active state of a FIX Session.
    Active,
}

struct AcceptorPendingEvents<'a> {
    acceptor: &'a mut Acceptor,
}

impl<'a> Iterator for AcceptorPendingEvents<'a> {
    type Item = EventOutbound;

    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.acceptor.state {
            State::Terminate => Some(Self::Item::Terminate),
            State::Terminated => None,
            State::Disconnected => None,
            State::Active => None,
        };
        self.acceptor.state = State::Terminated;
        next
    }
}
