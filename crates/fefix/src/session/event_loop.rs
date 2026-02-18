use crate::tagvalue::{DecodeError, DecoderStreaming, Message};
use crate::StreamingDecoder;
use futures::{select, AsyncRead, AsyncReadExt, FutureExt};
use futures_timer::Delay;
use std::env;
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
    must_clear_decoder: bool,
    stashed_bytes: Vec<u8>,
}

impl<I> LlEventLoop<I>
where
    I: AsyncRead + std::marker::Unpin,
{
    fn resync_after_parse_error(&mut self) {
        let snapshot = self.decoder.buffer().to_vec();
        if trace_io_enabled() {
            eprintln!(
                "[fefix/session] parse error, buffer='{}'",
                render_fix(snapshot.as_slice())
            );
        }
        // If parsing started from a candidate frame boundary (`8=` at offset 0),
        // avoid promoting embedded payload bytes as a fresh frame start.
        let restart_from = if snapshot.starts_with(b"8=") {
            None
        } else {
            find_embedded_message_start(snapshot.as_slice())
        };
        if trace_io_enabled() {
            eprintln!(
                "[fefix/session] resync start offset={}",
                restart_from
                    .map(|offset| offset.to_string())
                    .unwrap_or_else(|| "none".to_string())
            );
        }
        self.decoder.clear();
        if let Some(start) = restart_from {
            self.decoder.buffer().extend_from_slice(&snapshot[start..]);
        } else {
            // Keep a small trailing suffix so the next read can bridge into a
            // potential embedded `8=` message boundary split across packets.
            let keep = snapshot.len().min(8);
            if keep != 0 {
                self.decoder
                    .buffer()
                    .extend_from_slice(&snapshot[snapshot.len() - keep..]);
            }
        }
    }

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
            must_clear_decoder: false,
            stashed_bytes: Vec::new(),
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

    /// Sets the expected heartbeat interval used for timeout and scheduling.
    pub fn set_heartbeat(&mut self, heartbeat: Duration) {
        self.heartbeat = heartbeat;
    }

    /// Waits for the next low-level session event.
    pub async fn next_event<'a>(&'a mut self) -> Option<LlEvent<'a>> {
        if self.must_clear_decoder {
            self.decoder.clear();
            if !self.stashed_bytes.is_empty() {
                self.decoder.buffer().extend_from_slice(&self.stashed_bytes);
                self.stashed_bytes.clear();
            }
            self.must_clear_decoder = false;
        }

        let mut read_window: Option<(usize, usize)> = None;
        let mut read_cursor = 0usize;
        loop {
            if !self.is_alive {
                if self.decoder.buffer().is_empty() {
                    return Some(LlEvent::IoError(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "connection closed by peer",
                    )));
                }
                let result = self.decoder.try_parse();
                return match result {
                    Ok(Some(())) => {
                        self.must_clear_decoder = true;
                        Some(LlEvent::Message(self.decoder.message()))
                    }
                    Ok(None) => Some(LlEvent::IoError(io::Error::new(
                        io::ErrorKind::UnexpectedEof,
                        "connection closed by peer",
                    ))),
                    Err(err) => Some(LlEvent::BadMessage(err)),
                };
            }

            if read_window.is_none() {
                let mut current_len = self.decoder.buffer().len();
                let required_len = self.decoder.num_bytes_required();
                if trace_io_enabled() {
                    eprintln!(
                        "[fefix/session] window current_len={} required_len={} stashed={}",
                        current_len,
                        required_len,
                        self.stashed_bytes.len()
                    );
                }
                if current_len > required_len {
                    let overflow = self.decoder.buffer().split_off(required_len);
                    if !overflow.is_empty() {
                        if self.stashed_bytes.is_empty() {
                            self.stashed_bytes = overflow;
                        } else {
                            let mut merged = overflow;
                            merged.extend_from_slice(&self.stashed_bytes);
                            self.stashed_bytes = merged;
                        }
                    }
                    current_len = required_len;
                }
                self.decoder.buffer().resize(required_len, 0);
                let start = current_len;
                read_cursor = start;
                read_window = Some((start, required_len));
            }

            let (_start, end) = read_window.expect("read window must exist");
            if read_cursor >= end {
                let result = self.decoder.try_parse();
                if trace_io_enabled() {
                    eprintln!(
                        "[fefix/session] parse attempt (pre-read) result={:?}",
                        result
                    );
                }
                read_window = None;
                match result {
                    Ok(Some(())) => {
                        self.must_clear_decoder = true;
                        return Some(LlEvent::Message(self.decoder.message()));
                    }
                    Ok(None) => continue,
                    Err(err) => {
                        self.resync_after_parse_error();
                        return Some(LlEvent::BadMessage(err));
                    }
                }
            }

            let now = Instant::now();
            let mut timer_heartbeat = Delay::new(now - self.last_heartbeat + self.heartbeat).fuse();
            let mut timer_test_request =
                Delay::new(now - self.last_reset + self.heartbeat_soft_tolerance).fuse();
            let mut timer_logout =
                Delay::new(now - self.last_reset + self.heartbeat_hard_tolerance).fuse();

            let mut read_result = {
                let read_buf = &mut self.decoder.buffer().as_mut_slice()[read_cursor..end];
                self.input.read(read_buf).fuse()
            };

            select! {
                read_result = read_result => {
                    match read_result {
                        Err(e) => {
                            self.decoder.buffer().truncate(read_cursor);
                            return Some(LlEvent::IoError(e));
                        }
                        Ok(num_bytes) => {
                            if num_bytes == 0 {
                                self.decoder.buffer().truncate(read_cursor);
                                self.is_alive = false;
                                continue;
                            }
                            if trace_io_enabled() {
                                let read_start = read_cursor;
                                let read_end = read_cursor + num_bytes;
                                let bytes = &self.decoder.buffer()[read_start..read_end];
                                eprintln!(
                                    "[fefix/session] read bytes='{}'",
                                    render_fix(bytes)
                                );
                            }
                            read_cursor += num_bytes;
                            if read_cursor < end {
                                continue;
                            }

                            let result = self.decoder.try_parse();
                            if trace_io_enabled() {
                                eprintln!("[fefix/session] parse attempt (post-read) result={:?}", result);
                            }
                            read_window = None;

                            match result {
                                Ok(Some(())) => {
                                    self.must_clear_decoder = true;
                                    let msg = self.decoder.message();
                                    return Some(LlEvent::Message(msg));
                                }
                                Ok(None) => {
                                    continue;
                                }
                                Err(err) => {
                                    self.resync_after_parse_error();
                                    return Some(LlEvent::BadMessage(err));
                                }
                            }
                        }
                    };
                },
                () = timer_heartbeat => {
                    self.decoder.buffer().truncate(read_cursor);
                    self.last_heartbeat = Instant::now();
                    return Some(LlEvent::Heartbeat);
                },
                () = timer_test_request => {
                    self.decoder.buffer().truncate(read_cursor);
                    return Some(LlEvent::TestRequest);
                },
                () = timer_logout => {
                    self.decoder.buffer().truncate(read_cursor);
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

fn find_embedded_message_start(raw: &[u8]) -> Option<usize> {
    let mut i = 1usize;
    while i + 1 < raw.len() {
        let preceded_by_delimiter = matches!(raw[i - 1], 0x01 | b'\n' | b'\r');
        if preceded_by_delimiter && raw[i] == b'8' && raw[i + 1] == b'=' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn trace_io_enabled() -> bool {
    env::var_os("FEFIX_SESSION_TRACE_IO").is_some()
}

fn render_fix(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len());
    for b in bytes {
        match *b {
            0x01 => out.push('|'),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            other => out.push(other as char),
        }
    }
    out
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
    use tokio::io::{duplex, AsyncWriteExt, DuplexStream};
    use tokio_util::compat::*;

    async fn produce_events(events: Vec<(&'static [u8], Duration)>) -> DuplexStream {
        let (mut tx, rx) = duplex(4096);
        tokio::spawn(async move {
            for (event_bytes, delay) in events.iter() {
                tx.write_all(event_bytes).await.unwrap();
                tokio::time::sleep(*delay).await;
            }
        });
        rx
    }

    async fn new_event_loop(
        events: Vec<(&'static [u8], Duration)>,
    ) -> LlEventLoop<Compat<DuplexStream>> {
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

    #[tokio::test]
    async fn eof_input_triggers_io_error() {
        let mut event_loop = new_event_loop(vec![]).await;
        let event = event_loop.next_event().await;
        match event {
            Some(LlEvent::IoError(err)) => {
                assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
            }
            other => panic!("expected EOF io error, got {:?}", other),
        }
    }
}
