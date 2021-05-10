use crate::tagvalue::{DecoderBuffered, Message};
use futures::select;
use futures::{AsyncRead, AsyncReadExt, FutureExt};
use futures_timer::Delay;
use std::io;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct EventLoop<I>
where
    I: AsyncRead,
{
    decoder: DecoderBuffered,
    input: I,
    heartbeat: Duration,
    heartbeat_soft_tolerance: Duration,
    heartbeat_hard_tolerance: Duration,
    last_reset: Instant,
    last_heartbeat: Instant,
}

impl<I> EventLoop<I>
where
    I: AsyncRead + std::marker::Unpin,
{
    pub fn new(decoder: DecoderBuffered, input: I, heartbeat: Duration) -> Self {
        let heartbeat_soft_tolerance = heartbeat + Duration::from_secs(10);
        let heartbeat_hard_tolerance = heartbeat + Duration::from_secs(30);
        Self {
            decoder,
            input,
            heartbeat,
            heartbeat_soft_tolerance,
            heartbeat_hard_tolerance,
            last_reset: Instant::now(),
            last_heartbeat: Instant::now() - heartbeat / 2,
        }
    }

    pub async fn next<'a>(&'a mut self) -> Event<'a> {
        self.decoder.clear();
        let mut timer_heartbeat = self.timer_heartbeat().fuse();
        let mut timer_test_request = self.timer_test_request().fuse();
        let mut timer_logout = self.timer_logout().fuse();
        let next_message = decode_next_message(&mut self.decoder, &mut self.input).fuse();
        futures::pin_mut!(next_message);
        select! {
            decoder = next_message => {
                let msg = decoder.unwrap().message();
                return Event::Message { msg };
            },
            () = timer_heartbeat => {
                self.last_heartbeat = Instant::now();
                return Event::Heartbeat;
            },
            () = timer_test_request => {
                return Event::TestRequest;
            },
            () = timer_logout => {
                return Event::Logout;
            }
        }
    }

    pub fn heartbeat(&mut self) {
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

#[derive(Debug)]
pub enum Event<'a> {
    Message { msg: Message<'a> },
    Heartbeat,
    TestRequest,
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
