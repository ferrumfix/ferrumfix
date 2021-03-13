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
