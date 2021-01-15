use crate::app::slr;
use boolinator::Boolinator;
use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{Duration, Instant};

/// The acceptor should convey the rules placed on the expected heartbeat
/// interval via out-of-band rules of engagement when such rules are required by
/// the acceptor.
///
/// There are three methods for determining the heartbeat interval:
/// - Acceptor requires a specific heartbeat interval.
/// - Acceptor requires initiator to specify a value within a heartbeat interval
/// range.
/// - Acceptor accepts the initiator specified heartbeat interval.
#[derive(Debug)]
pub enum HeartbeatRule {
    Exact(Duration),
    Range(RangeInclusive<Duration>),
    Any,
}

impl HeartbeatRule {
    /// Validate a proposed heartbeat value depending on the selected configuration.
    pub fn validate(&self, proposal: &Duration) -> std::result::Result<(), String> {
        match self {
            HeartbeatRule::Exact(expected) => {
                (proposal == expected).ok_or_else(|| errs::heartbeat_exact(expected.as_secs()))
            }
            HeartbeatRule::Range(range) => range.contains(proposal).ok_or_else(|| {
                errs::heartbeat_range(range.start().as_secs(), range.end().as_secs())
            }),
            HeartbeatRule::Any => {
                (*proposal != Duration::from_secs(0)).ok_or_else(|| errs::heartbeat_gt_0())
            }
        }
    }
}

#[derive(Debug)]
pub struct Configuration {
    heartbeat_rule: HeartbeatRule,
    delivery_threshold: Duration,
    company_id: String,
}

#[derive(Debug)]
pub struct Acceptor {
    config: Configuration,
    state: InternalState,
    heartbeat: Duration,
    queue_events_outbound: Vec<EventOutbound>,
    expected_seqnum_inbound: u64,
    expected_seqnum_outbound: u64,
}

#[derive(Debug, PartialEq)]
enum InternalState {
    Disconnected,
    Active,
}

#[derive(Clone, Debug)]
pub enum EventInbound {
    Terminated,
    IncomingMessage(slr::Message),
}

#[derive(Clone, Debug)]
pub enum EventOutbound {
    Terminate,
    Message(slr::Message),
}

impl Acceptor {
    pub fn new(config: Configuration) -> Self {
        Acceptor {
            config,
            state: InternalState::Disconnected,
            heartbeat: Duration::default(),
            queue_events_outbound: Vec::new(),
            expected_seqnum_inbound: 1,
            expected_seqnum_outbound: 1,
        }
    }

    /// Get the expected heartbeat interval on the underlying FIX connection.
    pub fn heartbeat(&self) -> Duration {
        self.heartbeat
    }

    pub fn seq_nums(&self) -> (u64, u64) {
        (self.expected_seqnum_inbound, self.expected_seqnum_outbound)
    }

    pub fn feed(&mut self, event: EventInbound) {
        match event {
            EventInbound::Terminated => self.notify(EventOutbound::Terminate),
            EventInbound::IncomingMessage(msg) => {
                self.feed_incoming_message(msg);
            }
        }
    }

    fn feed_incoming_message(&mut self, message: slr::Message) {
        let msg_type = message.msg_type();
        let seqnum_ord = message
            .seq_num()
            .map(|seq_num| seq_num.cmp(&self.expected_seqnum_inbound));
        match seqnum_ord {
            Some(Ordering::Greater) => {
                let mut response = slr::Message::new();
                // TODO: add other details to response message.
                response.add_str(35, "2");
                response.add_str(49, self.config.company_id.as_str());
                response.add_int(7, self.expected_seqnum_inbound as i64);
                response.add_int(16, message.seq_num().unwrap() as i64);
                self.expected_seqnum_outbound += 1;
                self.notify(EventOutbound::Message(response));
                return;
            }
            Some(Ordering::Equal) => {}
            Some(Ordering::Less) => {
                let error_message = format!(
                    "Invalid MsgSeqNum <34>, expected value {}",
                    self.expected_seqnum_inbound
                );
                let mut response = slr::Message::new();
                response.add_str(35, "5");
                response.add_str(49, self.config.company_id.as_str());
                response.add_int(7, self.expected_seqnum_outbound as i64);
                response.add_str(58, error_message);
            }
            None => {}
        };
        if msg_type != Some("A") && self.state == InternalState::Disconnected {
            self.notify(EventOutbound::Terminate);
            return;
        }
        if let Some("A") = msg_type {
            if self.state == InternalState::Active {
                return;
            }
            let mut response = slr::Message::new();
            // TODO: add other details to response message.
            response.add_field(35, slr::FixFieldValue::String("A".to_string()));
            response.add_field(
                49,
                slr::FixFieldValue::String(self.config.company_id.clone()),
            );
            self.expected_seqnum_outbound += 1;
            self.notify(EventOutbound::Message(response));
            self.state = InternalState::Active;
        }
    }

    fn notify(&mut self, event: EventOutbound) {
        self.queue_events_outbound.push(event);
    }

    pub fn notifications(&mut self) -> impl Iterator<Item = EventOutbound> {
        let queue = self.queue_events_outbound.clone();
        self.queue_events_outbound = Vec::new();
        queue.into_iter()
    }
}

/// Error messages generation.
mod errs {
    pub fn heartbeat_exact(secs: u64) -> String {
        format!("Invalid HeartBtInt(108), expected value {} seconds", secs)
    }

    pub fn heartbeat_range(a: u64, b: u64) -> String {
        format!(
            "Invalid HeartBtInt(108), expected value between {} and {} seconds",
            a, b,
        )
    }

    pub fn heartbeat_gt_0() -> String {
        "Invalid HeartBtInt(108), expected value greater than 0 seconds".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const COMPANY_ID: &str = "FOOBAR-INC";

    fn acceptor() -> Acceptor {
        Acceptor::new(Configuration {
            heartbeat_rule: HeartbeatRule::Any,
            delivery_threshold: Duration::from_secs(10),
            company_id: COMPANY_ID.to_string(),
        })
    }

    #[test]
    fn heartebeat_validation() {
        let rule_exact_1 = HeartbeatRule::Exact(Duration::from_secs(1));
        let rule_range_5_30 =
            HeartbeatRule::Range(Duration::from_secs(5)..=Duration::from_secs(30));
        let rule_any = HeartbeatRule::Any;
        assert!(rule_exact_1.validate(&Duration::from_secs(1)).is_ok());
        assert!(!rule_exact_1.validate(&Duration::from_secs(2)).is_ok());
        assert!(!rule_exact_1.validate(&Duration::from_secs(0)).is_ok());
        assert!(rule_range_5_30.validate(&Duration::from_secs(5)).is_ok());
        assert!(rule_range_5_30.validate(&Duration::from_secs(10)).is_ok());
        assert!(rule_range_5_30.validate(&Duration::from_secs(30)).is_ok());
        assert!(!rule_range_5_30.validate(&Duration::from_secs(0)).is_ok());
        assert!(!rule_range_5_30.validate(&Duration::from_secs(4)).is_ok());
        assert!(!rule_range_5_30.validate(&Duration::from_secs(60)).is_ok());
        assert!(rule_any.validate(&Duration::from_secs(1)).is_ok());
        assert!(!rule_any.validate(&Duration::from_secs(0)).is_ok());
    }

    #[test]
    fn testcase_1s_a_1() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(1));
        let mut acceptor = acceptor();
        acceptor.feed(EventInbound::IncomingMessage(msg));
        let mut events = acceptor.notifications();
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(35).unwrap(),
                    slr::FixFieldValue::String("A".to_string())
                );
                assert_eq!(
                    *response.get_field(49).unwrap(),
                    slr::FixFieldValue::String(COMPANY_ID.to_string())
                );
                assert!(response.get_field(112).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        assert!(events.next().is_none());
    }

    #[test]
    fn testcase_1s_a_2() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(42));
        let mut acceptor = acceptor();
        acceptor.feed(EventInbound::IncomingMessage(msg));
        let mut events = acceptor.notifications();
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(35).unwrap(),
                    slr::FixFieldValue::String("2".to_string())
                );
                assert_eq!(
                    *response.get_field(49).unwrap(),
                    slr::FixFieldValue::String(COMPANY_ID.to_string())
                );
                assert!(response.get_field(112).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        assert!(events.next().is_none());
    }

    #[test]
    fn testcase_1s_b() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(1));
        let mut acceptor = acceptor();
        acceptor.feed(EventInbound::IncomingMessage(msg.clone()));
        acceptor.feed(EventInbound::IncomingMessage(msg));
        let mut events = acceptor.notifications();
        // First Logon message is fine.
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(35).unwrap(),
                    slr::FixFieldValue::String("A".to_string())
                );
                assert_eq!(
                    *response.get_field(49).unwrap(),
                    slr::FixFieldValue::String(COMPANY_ID.to_string())
                );
                assert!(response.get_field(112).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        // The second one is ignored.
        assert!(events.next().is_none());
    }

    #[test]
    fn testcase_2s() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("0".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(1));
        let mut acceptor = acceptor();
        acceptor.feed(EventInbound::IncomingMessage(msg));
        let mut events = acceptor.notifications();
        // First Logon message is fine.
        match events.next().unwrap() {
            EventOutbound::Terminate => (),
            _ => assert!(false),
        };
        // The second one is ignored.
        assert!(events.next().is_none());
    }
}
