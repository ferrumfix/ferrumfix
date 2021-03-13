//! FIX message processing between counterparties.
//!
//! To establish a reliable FIX connection, peers must adopt a session protocol.
//! [`Initiator`] is used to connect to service offerers and [`Acceptor`] is used
//! to accept incoming connections. This is akin to the client-server
//! architecture.
//!
//! ![](https://www.fixtrading.org/wp-content/uploads/2020/06/fixsessionlayerconceptualsimple.png)
//!
//! The above is a conceptual view of the FIX Session layer, complete with its
//! state machine and transitions between them. Both [`Initiator`] and
//! [`Acceptor`] abstract over such details and present users with a single entry
//! point, namely [`Initiator::feed`] and [`Acceptor::feed`].

use crate::tags::fix44 as tags;
use crate::tagvalue::FixFieldValue;
use crate::tagvalue::MessageRnd;
use futures_lite::prelude::*;
use std::time::Duration;
use uuid::Uuid;

mod errs;
mod heartbeat_rule;
mod seq_numbers;

pub use acceptor::*;
pub use heartbeat_rule::HeartbeatRule;
pub use initiator::Initiator;
pub use seq_numbers::{SeqNumberError, SeqNumbers};

/// An indicator for the kind of environment relative to a FIX Connection.
#[derive(Debug, Copy, Clone)]
#[non_exhaustive]
pub enum Environment {
    /// Test messages will be refused under this environment setting.
    ProductionDisallowTest,
    /// Test messages will be ignored under this environment setting.
    ProductionAllowTest,
    /// Production messages will be refused under this environment setting.
    Testing,
}

mod acceptor {
    use super::*;

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
}

mod initiator {
    use super::*;
    use futures::FutureExt;
    use futures::StreamExt;
    use std::time::Instant;

    #[derive(Debug, Clone)]
    pub struct Configuration {
        company_id_from: String,
        company_id_to: String,
        preferred_heartbeat: Duration,
        acceptable_heartbeat: HeartbeatRule,
    }

    #[derive(Debug, Clone)]
    pub enum ConfigurationError {
        CompIDNotAlphanumeric,
    }

    type Result<T> = std::result::Result<T, ConfigurationError>;

    fn is_alphanumeric(s: impl AsRef<str>) -> bool {
        s.as_ref().chars().all(|c| c.is_alphanumeric())
    }

    const DEFAULT_HEARTBEAT: Duration = Duration::from_secs(60);

    impl Configuration {
        pub fn new(from: String, to: String) -> Result<Self> {
            if !(is_alphanumeric(from.as_str()) && is_alphanumeric(to.as_str())) {
                return Err(ConfigurationError::CompIDNotAlphanumeric);
            }
            Ok(Self {
                company_id_from: from,
                company_id_to: to,
                preferred_heartbeat: DEFAULT_HEARTBEAT,
                acceptable_heartbeat: HeartbeatRule::Any,
            })
        }

        pub fn preferred_heartbeat(&mut self, heartbeat: Duration) -> &mut Self {
            self.preferred_heartbeat = heartbeat;
            self
        }

        pub fn allow_heartbeat_rule(&mut self, rule: HeartbeatRule) -> &mut Self {
            self.acceptable_heartbeat = rule;
            self
        }
    }

    //impl<S: Stream<Item = Message> + Unpin> Stream for Session<S> {
    //    type Item = Message;

    //    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    //        let heartbeat = sleep(self.initiator.heartbeat());
    //        let end_of_trading_hours = sleep_until(self.initiator.end_of_trading_hours().into());
    //        //let incoming_message = match (*self).events.poll_next(cx) {
    //        //    Poll::Ready(None) => Poll::Ready(None),
    //        //    Poll::Ready(Some(_event)) => unimplemented!(),
    //        //    Poll::Pending => Poll::Pending,
    //        //};
    //        select! {
    //            () = heartbeat => {
    //                Poll::Pending
    //            },
    //            () = end_of_trading_hours => {
    //                Poll::Pending
    //            },
    //            //() = incoming_message => (),
    //        }
    //    }
    //}

    #[derive(Debug)]
    pub struct Initiator {
        config: Configuration,
        seq_numbers: SeqNumbers,
        notifications: Vec<MessageRnd>,
    }

    impl Initiator {
        pub fn new(config: Configuration) -> Self {
            Self {
                config,
                seq_numbers: SeqNumbers::default(),
                notifications: vec![],
            }
        }

        pub async fn session(
            self,
            events: impl Stream<Item = MessageRnd> + Unpin,
        ) -> impl Stream<Item = MessageRnd> {
            let _events = events.into_future();
            let heartbeat_sleep = tokio::time::sleep(Duration::from_secs(1)).fuse();
            tokio::pin!(heartbeat_sleep);
            //loop {
            //    select! {
            //        () = heartbeat_sleep => (),
            //        _event = events => (),
            //    }
            //}
            futures::stream::empty()
            //Session {
            //    events,
            //    initiator: self,
            //}
        }

        pub fn heartbeat(&self) -> Duration {
            self.config.preferred_heartbeat
        }

        pub fn end_of_trading_hours(&self) -> Instant {
            Instant::now()
        }

        pub fn initiate(&mut self) -> MessageRnd {
            let mut msg = MessageRnd::new();
            msg.add_str(tags::MSG_TYPE, "A".to_string());
            msg.add_str(tags::SENDER_COMP_ID, self.config.company_id_from.clone());
            msg.add_str(tags::TARGET_COMP_ID, self.config.company_id_to.clone());
            msg.add_int(tags::HEART_BT_INT, 30);
            msg.add_int(tags::MSG_SEQ_NUM, self.seq_numbers.next_outbound() as i64);
            msg.add_int(tags::SENDING_TIME, 1337); // TODO
            msg
        }

        pub async fn terminate(&mut self) -> std::result::Result<(), Vec<MessageRnd>> {
            let test_request_id = Uuid::new_v4().to_string();
            // Send `TestRequest` to ensure we didn't miss any messages.
            let mut msg = MessageRnd::new();
            msg.add_str(tags::MSG_TYPE, "1".to_string());
            msg.add_str(tags::SENDER_COMP_ID, self.config.company_id_from.clone());
            msg.add_str(tags::TARGET_COMP_ID, self.config.company_id_to.clone());
            msg.add_int(tags::HEART_BT_INT, 30);
            msg.add_str(tags::TEST_REQ_ID, test_request_id.clone());
            msg.add_int(tags::MSG_SEQ_NUM, self.seq_numbers.next_inbound() as i64);
            msg.add_int(tags::SENDING_TIME, 1337); // TODO
            self.send_message(msg).await;
            // Wait for heartbeat:
            let heartbeat = self.next_msg().await;
            assert_eq!(
                heartbeat.get_field(tags::TEST_REQ_ID),
                Some(&FixFieldValue::string(test_request_id.as_bytes()).unwrap())
            );
            // TODO: check seq number.
            // TODO: resend missed messages.
            // Finally send `Logout`.
            let mut msg = MessageRnd::new();
            msg.add_str(tags::MSG_TYPE, "5".to_string());
            msg.add_str(tags::SENDER_COMP_ID, self.config.company_id_from.clone());
            msg.add_str(tags::TARGET_COMP_ID, self.config.company_id_to.clone());
            msg.add_int(tags::HEART_BT_INT, 30);
            msg.add_int(tags::MSG_SEQ_NUM, self.seq_numbers.next_outbound() as i64);
            msg.add_int(tags::SENDING_TIME, 1337); // FIXME
            self.send_message(msg).await;
            Ok(())
        }

        async fn send_message(&mut self, msg: MessageRnd) {
            self.notifications.push(msg);
        }

        pub async fn next_msg(&mut self) -> MessageRnd {
            unimplemented!()
        }

        pub fn feed_event(&mut self, event: MessageRnd) {
            match event.msg_type() {
                Some("A") => (),
                Some("0") => (),
                Some("5") => (),
                Some("3") => (),
                Some("2") => (),
                Some("4") => (),
                Some("1") => (),
                Some(_) => {}
                None => (),
            }
        }

        pub async fn notifications(&mut self) -> impl Stream<Item = MessageRnd> {
            // FIXME
            futures_lite::stream::empty()
        }

        pub fn notify(&mut self, _event: MessageRnd) -> impl Iterator<Item = EventOutbound> {
            std::iter::empty()
        }
    }
}

#[derive(Debug, Clone)]
pub enum SessionRejectReason {
    InvalidTagNumber,
    RequiredTagMissing,
    TagNotDefinedForThisMessageType,
    UndefinedTag,
    TagSpecifiedWithoutAValue,
    ValueIsIncorrect,
    IncorrectDataFormatForValue,
    DecryptionProblem,
    SignatureProblem,
    CompIDProblem,
    SendingTimeAccuracyProblem,
    InvalidMsgType,
    XMLValidationError,
    TagAppearsMoreThanOnce,
    TagSpecifiedOutOfRequiredOrder,
    RepeatingGroupFieldsOutOfOrder,
    IncorrectNumInGroupCountForRepeatingGroup,
    FieldDelimiterInFieldValue,
    InvalidUnsupportedAppVersion,
    Other,
}

impl From<u32> for SessionRejectReason {
    fn from(v: u32) -> Self {
        match v {
            0 => Self::InvalidTagNumber,
            1 => Self::RequiredTagMissing,
            2 => Self::TagNotDefinedForThisMessageType,
            3 => Self::UndefinedTag,
            4 => Self::TagSpecifiedWithoutAValue,
            5 => Self::ValueIsIncorrect,
            6 => Self::IncorrectDataFormatForValue,
            7 => Self::DecryptionProblem,
            8 => Self::SignatureProblem,
            9 => Self::CompIDProblem,
            10 => Self::SendingTimeAccuracyProblem,
            11 => Self::InvalidMsgType,
            12 => Self::XMLValidationError,
            13 => Self::TagAppearsMoreThanOnce,
            14 => Self::TagSpecifiedOutOfRequiredOrder,
            15 => Self::RepeatingGroupFieldsOutOfOrder,
            16 => Self::IncorrectNumInGroupCountForRepeatingGroup,
            17 => Self::FieldDelimiterInFieldValue,
            18 => Self::InvalidUnsupportedAppVersion,
            _ => Self::Other,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::FixFieldValue;

    const COMPANY_ID: &str = "FOOBAR-INC";

    fn acceptor() -> Acceptor {
        let mut config = Configuration::new(COMPANY_ID.to_string());
        config.with_hb_rule(HeartbeatRule::Any);
        config.with_environment(Environment::ProductionDisallowTest);
        config.acceptor()
    }

    /// Condition:
    ///
    /// > Valid Logon(35=A) request message received.
    ///
    /// Expected behavior:
    ///
    /// > Respond with Logon(35=A) acknowledgement message.
    #[tokio::test]
    async fn testcase_1s_a_1() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "A".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 1);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(tags::MSG_TYPE).unwrap(),
                    FixFieldValue::string(b"A").unwrap()
                );
                assert_eq!(
                    *response.get_field(tags::SENDER_COMP_ID).unwrap(),
                    FixFieldValue::string(COMPANY_ID.as_bytes()).unwrap()
                );
                assert!(response.get_field(tags::TEST_REQ_ID).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        assert!(events.next().is_none());
    }

    /// Condition:
    ///
    /// > Valid Logon(35=A) request message received.
    ///
    /// Expected behavior:
    ///
    /// > If MsgSeqNum(34) > NextNumIn send ResendRequest(35=2).
    #[tokio::test]
    async fn testcase_1s_a_2() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "A".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 42);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(tags::MSG_TYPE).unwrap(),
                    FixFieldValue::string(b"2").unwrap()
                );
                assert_eq!(
                    *response.get_field(tags::SENDER_COMP_ID).unwrap(),
                    FixFieldValue::string(COMPANY_ID.as_bytes()).unwrap()
                );
                assert!(response.get_field(tags::TEST_REQ_ID).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        assert!(events.next().is_none());
    }

    /// Condition:
    ///
    /// > Logon(35=A) message received with duplicate identity (e.g. same IP,
    /// port, SenderCompID(49), TargetCompID(56), etc. as existing connection).
    ///
    /// Expected behavior:
    ///
    /// > 1. Generate an error condition in test output.
    /// > 2. Disconnect without sending a message (Note: sending a Reject or
    /// Logout(35=5) would consume a MsgSeqNum(34)).
    #[tokio::test]
    async fn testcase_1s_b() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "A".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 1);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg.clone()));
        // First Logon message is fine.
        match events.next().unwrap() {
            EventOutbound::Message(response) => {
                assert_eq!(
                    *response.get_field(tags::MSG_TYPE).unwrap(),
                    FixFieldValue::string(b"A").unwrap()
                );
                assert_eq!(
                    *response.get_field(tags::SENDER_COMP_ID).unwrap(),
                    FixFieldValue::string(COMPANY_ID.as_bytes()).unwrap()
                );
                assert!(response.get_field(tags::TEST_REQ_ID).is_none());
            }
            EventOutbound::Terminate => panic!(),
        }
        // The second one is ignored.
        assert!(events.next().is_none());
    }

    /// Condition:
    ///
    /// > First message received is not a Logon(35=A) message.
    ///
    /// Expected behavior:
    ///
    /// > 1. Log an error “First message not a logon”.
    /// > 2. Disconnect.
    #[test]
    fn testcase_2s() {
        let mut msg = MessageRnd::new();
        msg.add_str(tags::MSG_TYPE, "0".to_string());
        msg.add_int(tags::HEART_BT_INT, 30);
        msg.add_int(tags::MSG_SEQ_NUM, 1);
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
        // First Logon message is fine.
        match events.next().unwrap() {
            EventOutbound::Terminate => (),
            _ => assert!(false),
        };
        // The second one is ignored.
        assert!(events.next().is_none());
    }
}
