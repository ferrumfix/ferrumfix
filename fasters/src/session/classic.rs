use crate::app::slr;
use boolinator::Boolinator;
use futures_lite::prelude::*;
use std::cmp::Ordering;
use std::num::NonZeroU64;
use std::ops::RangeInclusive;
use std::time::Duration;
use uuid::Uuid;

pub use acceptor::*;
pub use initiator::Initiator;

/// The acceptor should convey the rules placed on the expected heartbeat
/// interval via out-of-band rules of engagement when such rules are required by
/// the acceptor.
///
/// Please note that [`HeartbeatRule`](HeartbeatRule) is marked with
/// `#[non_exhaustive]`, which future-proofs the enumeration type in case more
/// variants are added.
///
/// Please refer to specs. §4.3.5 for more information.
#[derive(Debug, Clone, Hash)]
#[non_exhaustive]
pub enum HeartbeatRule {
    /// The acceptor requires a specific heartbeat interval, expressed as a
    /// [`Duration`](std::time::Duration). Please refer to specs. §4.3.5.1 for
    /// more information.
    Exact(Duration),
    /// The acceptor requires the initiator to specify a heartbeat value within a
    /// [`RangeInclusive`](std::ops::RangeInclusive) of
    /// [`Duration`s](std::time::Duration). Please refer to specs. §4.3.5.3 for
    /// more information.
    Range(RangeInclusive<Duration>),
    /// The acceptor poses no restrictions on the heartbeat interval and the
    /// initiator can choose any value. Please refer to specs. §4.3.5.3 for more
    /// information.
    Any,
}

impl HeartbeatRule {
    /// Validates an initiator-provided heartbeat value according to the
    /// heartbeat rule.
    ///
    /// # Examples
    /// Require exact matching with [`HeartbeatRule::Exact`](HeartbeatRule):
    /// ```
    /// use fasters::session::classic::HeartbeatRule;
    /// use std::time::Duration;
    ///
    /// let rule = HeartbeatRule::Exact(Duration::from_secs(30));
    /// assert!(rule.validate(&Duration::from_secs(60)).is_err());
    /// assert!(rule.validate(&Duration::from_secs(20)).is_err());
    /// assert!(rule.validate(&Duration::from_secs(30)).is_ok());
    /// ```
    /// Accepting any proposed heartbeat value:
    /// ```
    /// use fasters::session::classic::HeartbeatRule;
    /// use std::time::Duration;
    ///
    /// let rule = HeartbeatRule::Any;
    /// assert!(rule.validate(&Duration::from_secs(1000)).is_ok());
    /// assert!(rule.validate(&Duration::from_secs(1)).is_ok());
    /// ```
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

#[derive(Debug, Copy, Clone)]
pub struct SeqNumbers {
    next_inbound: u64,
    next_outbound: u64,
}

impl SeqNumbers {
    pub fn new(inbound: NonZeroU64, outbound: NonZeroU64) -> Self {
        Self {
            next_inbound: inbound.get(),
            next_outbound: outbound.get(),
        }
    }

    pub fn next_inbound(&self) -> u64 {
        self.next_inbound
    }

    pub fn next_outbound(&self) -> u64 {
        self.next_outbound
    }

    pub fn incr_inbound(&mut self) {
        self.next_inbound += 1;
    }

    pub fn incr_outbound(&mut self) {
        self.next_outbound += 1;
    }

    pub fn validate_inbound(&self, inbound: u64) -> Result<(), SeqNumberError> {
        match inbound.cmp(&self.next_inbound) {
            Ordering::Equal => Ok(()),
            Ordering::Less => Err(SeqNumberError::TooLow),
            Ordering::Greater => Err(SeqNumberError::Recover),
        }
    }
}

impl Default for SeqNumbers {
    fn default() -> Self {
        Self {
            next_inbound: 1,
            next_outbound: 1,
        }
    }
}

pub enum SeqNumberError {
    Recover,
    TooLow,
    NoSeqNum,
}

pub trait Session {
    type Inbound;
    type Outbound;

    fn feed(&mut self, event: Self::Inbound);
    fn notifications(&mut self) -> Vec<Self::Outbound>;
}

type CompID = String;

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

    /// An indicator for the kind of environment relative to a FIX Connection.
    #[derive(Debug, Copy, Clone)]
    #[non_exhaustive]
    pub enum Environment {
        ProductionDisallowTest,
        ProductionAllowTest,
        Testing,
    }

    impl Configuration {
        pub fn new() -> Self {
            Self {
                heartbeat_rule: HeartbeatRule::Any,
                delivery_threshold: Duration::from_secs(60),
                company_id: "FOOBAR".to_string(),
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

        pub fn with_hb_rule(&mut self, rule: HeartbeatRule) -> &mut Self {
            self.heartbeat_rule = rule;
            self
        }

        pub fn with_company_id(&mut self, id: String) -> &mut Self {
            self.company_id = id;
            self
        }

        pub fn acceptor(self) -> Acceptor {
            Acceptor::new(self)
        }
    }

    /// A FIX Session acceptor.
    #[derive(Debug, Clone)]
    pub struct Acceptor {
        config: Configuration,
        state: State,
        heartbeat: Duration,
        seq_numbers: SeqNumbers,
    }

    #[derive(Clone, Debug)]
    #[non_exhaustive]
    pub enum EventInbound {
        Terminated,
        IncomingMessage(slr::Message),
    }

    #[derive(Clone, Debug)]
    pub enum EventOutbound {
        Terminate,
        Message(slr::Message),
    }

    pub trait Session {
        type Inbound;
        type Outbound;

        fn feed(&mut self, event: Self::Inbound);
        fn notifications(&mut self) -> Vec<Self::Outbound>;
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
            Some(self.heartbeat)
        }

        /// Returns the internal seq. numbers state.
        pub fn seq_numbers(&self) -> SeqNumbers {
            self.seq_numbers
        }

        async fn feed_incoming_message(&mut self, message: slr::Message) {
            let msg_type = message.msg_type();
            match (self.config.environment, message.test_indicator()) {
                (Environment::ProductionAllowTest, Some(true)) => (),
                _ => (),
            };
            let seqnum_state = message
                .seq_num()
                .map(|seqnum| self.seq_numbers().validate_inbound(seqnum))
                .unwrap_or(Err(SeqNumberError::NoSeqNum));
            // Compare the incoming seq. number to the one we expected and act
            // accordingly.
            match seqnum_state {
                Ok(()) => {}
                Err(SeqNumberError::NoSeqNum) => {}
                Err(SeqNumberError::Recover) => {
                    // Refer to specs. §4.8 for more information.
                    let mut response = slr::Message::new();
                    // TODO: add other details to response message.
                    response.add_str(35, "2");
                    response.add_str(49, self.config.company_id.as_str());
                    response.add_int(7, self.seq_numbers().next_inbound() as i64);
                    response.add_int(16, message.seq_num().unwrap() as i64);
                    self.seq_numbers.incr_outbound();
                    self.send_message(response).await;
                    return;
                }
                Err(SeqNumberError::TooLow) => {
                    self.send_error_seqnum_too_low().await;
                }
            };
            if msg_type != Some("A") && self.state == State::Terminated {
                self.send_terminate_signal().await;
                return;
            }
            if let Some("A") = msg_type {
                if self.state == State::Active {
                    return;
                }
                let mut response = slr::Message::new();
                // TODO: add other details to response message.
                response.add_field(35, slr::FixFieldValue::String("A".to_string()));
                response.add_field(
                    49,
                    slr::FixFieldValue::String(self.config.company_id.clone()),
                );
                self.seq_numbers.incr_outbound();
                self.send_message(response).await;
                self.state = State::Active;
            }
        }

        async fn send_error_seqnum_too_low(&mut self) -> EventOutbound {
            let error_message = errs::msg_seq_num(self.seq_numbers().next_inbound());
            let mut response = slr::Message::new();
            response.add_str(35, "5");
            response.add_str(49, self.config.company_id.as_str());
            response.add_int(7, self.seq_numbers().next_outbound() as i64);
            response.add_str(58, error_message);
            self.send_message(response).await
        }

        async fn send_message(&mut self, message: slr::Message) -> EventOutbound {
            EventOutbound::Message(message)
        }

        async fn send_terminate_signal(&mut self) -> EventOutbound {
            EventOutbound::Terminate
        }

        pub async fn session_loop(
            &mut self,
            mut events: impl Stream<Item = EventInbound> + Unpin,
        ) -> impl Stream<Item = EventOutbound> {
            while let Some(event) = events.next().await {
                match event {
                    EventInbound::Terminated => {}
                    EventInbound::IncomingMessage(_msg) => {}
                };
            }
            futures_lite::stream::empty()
        }

        pub fn notify<'a>(
            &'a mut self,
            event: EventInbound,
        ) -> impl Iterator<Item = EventOutbound> + 'a {
            match event {
                EventInbound::Terminated => {}
                EventInbound::IncomingMessage(_msg) => {}
            };
            // TODO...
            AcceptorPendingEvents { acceptor: self }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum State {
        /// No FIX Session currently active.
        Disconnected,
        /// The FIX Session is over, we just need to terminate the transport layer
        /// connection.
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
    use futures::select;
    use futures::FutureExt;
    use futures::StreamExt;
    use std::time::Instant;

    enum State {
        NoActiveSession,
        LogonInitiated,
        ActiveSession,
        LogoffReceived,
        Retransmit,
    }

    #[derive(Debug, Clone)]
    pub struct Configuration {
        company_id_from: String,
        company_id_to: String,
        preferred_heartbeat: Duration,
        acceptable_heartbeat: HeartbeatRule,
    }

    pub enum ConfigurationError {
        CompIDNotAlphanumeric,
    }

    type Result<T> = std::result::Result<T, ConfigurationError>;

    fn is_alphanumeric(s: impl AsRef<str>) -> bool {
        s.as_ref().chars().all(|c| c.is_alphanumeric())
    }

    const DEFAULT_HEARTBEAT: Duration = Duration::from_secs(60);

    impl Configuration {
        pub fn new(from: CompID, to: CompID) -> Result<Self> {
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

    struct Session<S: Stream<Item = slr::Message>> {
        initiator: Initiator,
        next_heartbeat: tokio::time::Sleep,
        end_of_trading_hours: tokio::time::Sleep,
        events: S,
    }

    //impl<S: Stream<Item = slr::Message> + Unpin> Stream for Session<S> {
    //    type Item = slr::Message;

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
        notifications: Vec<slr::Message>,
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
            events: impl Stream<Item = slr::Message> + Unpin,
        ) -> impl Stream<Item = slr::Message> {
            let mut events = events.into_future();
            let heartbeat_sleep = tokio::time::sleep(Duration::from_secs(1)).fuse();
            tokio::pin!(heartbeat_sleep);
            loop {
                select! {
                    () = heartbeat_sleep => (),
                    _event = events => (),
                }
            }
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

        fn initiate(&mut self) -> slr::Message {
            let mut msg = slr::Message::new();
            msg.add_str(35, "A".to_string());
            msg.add_str(49, self.config.company_id_from.clone());
            msg.add_str(56, self.config.company_id_to.clone());
            msg.add_int(108, 30);
            msg.add_int(34, self.seq_numbers.next_outbound as i64);
            msg.add_int(52, 1337); // TODO
            msg
        }

        pub async fn terminate(&mut self) -> std::result::Result<(), Vec<slr::Message>> {
            let test_request_id = Uuid::new_v4().to_string();
            // Send `TestRequest` to ensure we didn't miss any messages.
            let mut msg = slr::Message::new();
            msg.add_str(35, "1".to_string());
            msg.add_str(49, self.config.company_id_from.clone());
            msg.add_str(56, self.config.company_id_to.clone());
            msg.add_int(108, 30);
            msg.add_str(112, test_request_id.clone());
            msg.add_int(34, self.seq_numbers.next_inbound as i64);
            msg.add_int(52, 1337); // TODO
            self.send_message(msg).await;
            // Wait for heartbeat:
            let heartbeat = self.next_msg().await;
            assert_eq!(
                heartbeat.get_field(112),
                Some(&slr::FixFieldValue::String(test_request_id))
            );
            // TODO: check seq number.
            // TODO: resend missed messages.
            // Finally send `Logout`.
            let mut msg = slr::Message::new();
            msg.add_str(35, "5".to_string());
            msg.add_str(49, self.config.company_id_from.clone());
            msg.add_str(56, self.config.company_id_to.clone());
            msg.add_int(108, 30);
            msg.add_int(34, self.seq_numbers.next_outbound as i64);
            msg.add_int(52, 1337); // FIXME
            self.send_message(msg).await;
            Ok(())
        }

        async fn send_message(&mut self, msg: slr::Message) {
            self.notifications.push(msg);
        }

        pub async fn next_msg(&mut self) -> slr::Message {
            unimplemented!()
        }

        async fn feed_event(&mut self, event: slr::Message) {
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

        pub async fn notifications(&mut self) -> impl Stream<Item = slr::Message> {
            // FIXME
            futures_lite::stream::empty()
        }

        fn notify<'a>(
            &'a mut self,
            _event: slr::Message,
        ) -> impl Iterator<Item = EventOutbound> + 'a {
            std::iter::empty()
        }
    }
}

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

    pub fn inbound_seqnum() -> String {
        "NextExpectedMsgSeqNum(789) > than last message sent".to_string()
    }

    pub fn msg_seq_num(seq_number: u64) -> String {
        format!("Invalid MsgSeqNum <34>, expected value {}", seq_number)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const COMPANY_ID: &str = "FOOBAR-INC";

    fn acceptor() -> Acceptor {
        let mut config = Configuration::new();
        config.with_hb_rule(HeartbeatRule::Any);
        config.with_environment(Environment::ProductionDisallowTest);
        config.acceptor()
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

    #[tokio::test]
    async fn testcase_1s_a_1() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(1));
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
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

    #[tokio::test]
    async fn testcase_1s_a_2() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(42));
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg));
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

    #[tokio::test]
    async fn testcase_1s_b() {
        let mut msg = slr::Message::new();
        msg.add_field(35, slr::FixFieldValue::String("A".to_string()));
        msg.add_field(108, slr::FixFieldValue::Int(30));
        msg.add_field(34, slr::FixFieldValue::Int(1));
        let mut acceptor = acceptor();
        let mut events = acceptor.notify(EventInbound::IncomingMessage(msg.clone()));
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
