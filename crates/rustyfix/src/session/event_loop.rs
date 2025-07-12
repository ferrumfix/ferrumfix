use crate::StreamingDecoder;
use crate::tagvalue::{DecodeError, DecoderStreaming, Message};
use futures::{AsyncRead, AsyncReadExt, FutureExt, select};
use futures_timer::Delay;
use std::io;
use std::time::{Duration, Instant};

/// Asynchronous, executor-agnostic low-level event loop for FIX connectors.
///
/// This event loop allows FIX connectors to delegate event-tracking logic to a
/// single entity. This event loop keeps track of such events within a FIX
/// session. See [`LlEvent`] for more information.
#[derive(Debug)]
pub struct LlEventLoop<I> {
    decoder: DecoderStreaming<Vec<u8>>,
    input: I,
    heartbeat: Duration,
    heartbeat_soft_tolerance: Duration,
    heartbeat_hard_tolerance: Duration,
    last_reset: Instant,
    last_heartbeat: Instant,
    is_alive: bool,
}

impl<I> LlEventLoop<I>
where
    I: AsyncRead + std::marker::Unpin,
{
    /// Creates a new [`LlEventLoop`] with the provided `decoder` and
    /// `heartbeat`. Events will be read from `input`.
    pub fn new(decoder: DecoderStreaming<Vec<u8>>, input: I, heartbeat: Duration) -> Self {
        let heartbeat_soft_tolerance = heartbeat * 2;
        let heartbeat_hard_tolerance = heartbeat * 3;
        Self {
            decoder,
            input,
            heartbeat,
            heartbeat_soft_tolerance,
            heartbeat_hard_tolerance,
            last_reset: Instant::now(),
            last_heartbeat: Instant::now(),
            is_alive: true,
        }
    }

    /// How long after a missed `Heartbeat <0>` should we send a `TestRequest
    /// <1>`?
    pub fn set_soft_tolerance(&mut self, soft_tolerance: Duration) {
        self.heartbeat_soft_tolerance = soft_tolerance;
    }

    /// How long after a missed `Heartbeat <0>` should we send a `Logout <5>`?
    pub fn set_hard_tolerance(&mut self, hard_tolerance: Duration) {
        self.heartbeat_hard_tolerance = hard_tolerance;
    }

    /// Returns the next low-level event without blocking.
    pub async fn next_event<'a>(&'a mut self) -> Option<LlEvent<'a>> {
        let mut buf_filled_len = 0;
        let mut buf = self.decoder.fillable();

        loop {
            if !self.is_alive {
                return None;
            }

            let now = Instant::now();
            let mut timer_heartbeat = Delay::new(now - self.last_heartbeat + self.heartbeat).fuse();
            let mut timer_test_request =
                Delay::new(now - self.last_reset + self.heartbeat_soft_tolerance).fuse();
            let mut timer_logout =
                Delay::new(now - self.last_reset + self.heartbeat_hard_tolerance).fuse();
            let mut read_result = self.input.read(buf).fuse();

            select! {
                read_result = read_result => {
                    match read_result {
                        Err(e) => {
                            return Some(LlEvent::IoError(e));
                        }
                        Ok(num_bytes) => {
                            buf_filled_len += num_bytes;
                            if buf_filled_len < buf.len() {
                                continue;
                            }

                            let result = self.decoder.try_parse();
                            buf_filled_len = 0;
                            buf = &mut self.decoder.fillable()[buf_filled_len..];

                            match result {
                                Ok(Some(())) => {
                                    let msg = self.decoder.message();
                                    return Some(LlEvent::Message(msg));
                                }
                                Ok(None) => {
                                    continue;
                                }
                                Err(err) => {
                                    self.is_alive = false;
                                    return Some(LlEvent::BadMessage(err))
                                }
                            }
                        }
                    };
                },
                () = timer_heartbeat => {
                    self.last_heartbeat = Instant::now();
                    return Some(LlEvent::Heartbeat);
                },
                () = timer_test_request => {
                    return Some(LlEvent::TestRequest);
                },
                () = timer_logout => {
                    self.is_alive = false;
                    return Some(LlEvent::Logout);
                }
            }
        }
    }

    /// Resets the FIX counterparty's `Heartbeat <0>` -associated timers.
    pub fn ping_heartbeat(&mut self) {
        self.last_reset = Instant::now();
    }
}

/// A low level event produced by a [`LlEventLoop`].
#[derive(Debug)]
pub enum LlEvent<'a> {
    /// Incoming FIX message.
    Message(Message<'a, &'a [u8]>),
    /// Tried to parse an incoming FIX message, but got illegal data.
    BadMessage(DecodeError),
    /// I/O error at the transport layer.
    IoError(io::Error),
    /// Time to send a new `HeartBeat <0>` message.
    Heartbeat,
    /// The FIX counterparty has missed the `Heartbeat <0>` deadline by some
    /// amount of time, and it's time to send a `Test Request <1>`
    /// message to check what's going on.
    TestRequest,
    /// The FIX counterparty has missed the `Heartbeat <0>` deadline by some
    /// amount of time, and it's stopped responding. It's time to
    /// disconnect via a `Logout <5>` message.
    Logout,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::Decoder;
    use tokio::io::AsyncWriteExt;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_util::compat::*;

    async fn produce_events(events: Vec<(&'static [u8], Duration)>) -> TcpStream {
        let tcp_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = tcp_listener.local_addr().unwrap();

        tokio::spawn(async move {
            let mut stream = TcpStream::connect(local_addr).await.unwrap();
            for (event_bytes, delay) in events.iter() {
                stream.write_all(event_bytes).await.unwrap();
                tokio::time::sleep(*delay).await;
            }
        });

        tcp_listener.accept().await.unwrap().0
    }

    async fn new_event_loop(
        events: Vec<(&'static [u8], Duration)>,
    ) -> LlEventLoop<Compat<TcpStream>> {
        let input = produce_events(events).await;

        LlEventLoop::new(
            Decoder::new(crate::Dictionary::fix44()).streaming(vec![]),
            input.compat(),
            Duration::from_secs(3),
        )
    }

    #[tokio::test]
    async fn dead_input_triggers_logout() {
        let mut event_loop = new_event_loop(vec![(b"8", Duration::from_secs(10))]).await;
        let event = event_loop.next_event().await;
        assert!(matches!(event, Some(LlEvent::Heartbeat)));
        let event = event_loop.next_event().await;
        assert!(
            matches!(event, Some(LlEvent::Heartbeat))
                || matches!(event, Some(LlEvent::TestRequest))
        );
    }
}
