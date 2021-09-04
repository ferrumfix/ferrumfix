use crate::tagvalue::{DecoderBuffered, Message};
use futures::select;
use futures::{AsyncRead, AsyncReadExt, FutureExt};
use futures_timer::Delay;
use std::io;
use std::time::Duration;
use std::time::Instant;

/// Asynchronous, executor-agnostic low-level event loop for FIX connectors.
///
/// This event loop allows FIX connectors to delegate event-tracking logic to a
/// single entity. This event loop keeps track of such events within a FIX
/// session. See [`LlEvent`] for more information.
#[derive(Debug)]
pub struct LlEventLoop<I> {
    decoder: DecoderBuffered,
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
    pub fn new(decoder: DecoderBuffered, input: I, heartbeat: Duration) -> Self {
        let heartbeat_soft_tolerance = heartbeat * 2;
        let heartbeat_hard_tolerance = heartbeat * 3;
        Self {
            decoder,
            input,
            heartbeat,
            heartbeat_soft_tolerance,
            heartbeat_hard_tolerance,
            last_reset: Instant::now(),
            last_heartbeat: Instant::now() - heartbeat / 2,
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

    pub async fn next<'a>(&'a mut self) -> Option<LlEvent<'a>> {
        if !self.is_alive {
            return None;
        }

        self.decoder.clear();

        let mut timer_heartbeat = self.timer_heartbeat().fuse();
        let mut timer_test_request = self.timer_test_request().fuse();
        let mut timer_logout = self.timer_logout().fuse();
        let next_message = decode_next_message(&mut self.decoder, &mut self.input).fuse();
        futures::pin_mut!(next_message);

        select! {
            decoder = next_message => {
                match decoder {
                    Ok(decoder) => {
                        let msg = decoder.message();
                        return Some(LlEvent::Message(msg));
                    }
                    Err(err) => {
                        self.is_alive = false;
                        return Some(LlEvent::IoError(err))
                    }
                }
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

    /// Resets the FIX counterparty's `Heartbeat <0>` -associated timers.
    pub fn ping_heartbeat(&mut self) {
        self.last_reset = Instant::now();
    }

    fn timer_heartbeat(&self) -> Delay {
        Delay::new(self.last_heartbeat + self.heartbeat - Instant::now())
    }

    fn timer_test_request(&self) -> Delay {
        Delay::new(self.last_reset + self.heartbeat_soft_tolerance - Instant::now())
    }

    fn timer_logout(&self) -> Delay {
        Delay::new(self.last_reset + self.heartbeat_hard_tolerance - Instant::now())
    }
}

/// A low level event produced by a [`LlEventLoop`].
#[derive(Debug)]
pub enum LlEvent<'a> {
    /// Incoming FIX message.
    Message(Message<'a, &'a [u8]>),
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

async fn decode_next_message<'a, I>(
    decoder: &'a mut DecoderBuffered,
    input: &mut I,
) -> io::Result<&'a mut DecoderBuffered>
where
    I: AsyncRead + std::marker::Unpin,
{
    loop {
        let buffer = decoder.supply_buffer();
        if buffer.is_empty() {
            return Ok(decoder); // FIXME
        }
        input.read_exact(buffer).await.ok();
        if let Ok(Some(())) = decoder.state() {
            return Ok(decoder);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::tagvalue::{Config, Decoder};
    use tokio::io::AsyncWriteExt;
    use tokio::net::{TcpListener, TcpStream};
    use tokio_util::compat::*;

    async fn events(events: &'static [(&[u8], Duration)]) -> TcpStream {
        let tcp_listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let local_addr = tcp_listener.local_addr().unwrap();

        tokio::spawn(async move {
            let mut stream = TcpStream::connect(local_addr).await.unwrap();
            for (event_bytes, delay) in events.iter() {
                stream.write(event_bytes).await.unwrap();
                tokio::time::sleep(*delay).await;
            }
        });

        tcp_listener.accept().await.unwrap().0
    }

    #[tokio::test]
    async fn dead_input_triggers_logout() {
        let input = events(&[]).await;
        let mut event_loop = LlEventLoop::new(
            Decoder::<Config>::new(crate::Dictionary::fix44()).buffered(),
            input.compat(),
            Duration::from_secs(3),
        );
        let event = event_loop.next().await;
        assert!(matches!(event, Some(LlEvent::TestRequest)));
    }
}
