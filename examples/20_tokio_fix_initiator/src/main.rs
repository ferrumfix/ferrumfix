use fefix::prelude::*;
use fefix::tagvalue::Decoder;
use slog::{debug, info, o, Logger};
use std::io;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::ops::Range;
use tokio::net::TcpSocket;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

const PORT: u16 = 0xF13;

#[tokio::main]
async fn main() -> io::Result<()> {
    let tcp_socket = TcpSocket::new_v4()?;
    let socket_address = SocketAddrV4::new(Ipv4Addr::LOCALHOST, PORT);
    let tcp_stream = tcp_socket.connect(socket_address.into()).await?;
    tcp_stream.set_nodelay(true)?;
    let app = Application::new(logger());
    let mut config = fefix::session::Config::default();
    config.set_begin_string("FIX.4.2");
    config.set_target_comp_id("TW");
    config.set_sender_comp_id("INCA");
    let fix_dictionary = Dictionary::fix42();
    let fix_decoder = Decoder::new(fix_dictionary);
    let (reader, writer) = tokio::io::split(tcp_stream);
    fefix::session::FixConnection::new(config)
        .start(
            app,
            reader.compat(),
            writer.compat_write(),
            fix_decoder.buffered(),
        )
        .await;
    Ok(())
}

#[derive(Clone)]
struct Application {
    logger: Logger,
    messages: Vec<Vec<u8>>,
}

impl Application {
    fn new(logger: Logger) -> Self {
        Self {
            logger,
            messages: Vec::new(),
        }
    }
}

impl fefix::session::Backend for Application {
    type Error = ();

    fn on_inbound_app_message(
        &mut self,
        message: fefix::tagvalue::Message<&[u8]>,
    ) -> Result<(), Self::Error> {
        self.on_inbound_message(message, true)
    }

    fn on_inbound_message(
        &mut self,
        message: fefix::tagvalue::Message<&[u8]>,
        _is_app: bool,
    ) -> Result<(), Self::Error> {
        if let Ok(s) = std::str::from_utf8(message.as_bytes()) {
            debug!(
                self.logger,
                "Inbound FIX message.";
                "message" => s,
            );
        } else {
            debug!(
                self.logger,
                "Inbound FIX message.";
                "message" => "(invalid UTF-8)",
            );
        }
        Ok(())
    }

    fn on_outbound_message(&mut self, message: &[u8]) -> Result<(), Self::Error> {
        if let Ok(s) = std::str::from_utf8(message) {
            debug!(
                self.logger,
                "Outbound FIX message.";
                "message" => s,
            );
        }
        Ok(())
    }

    fn on_successful_handshake(&mut self) -> Result<(), Self::Error> {
        info!(self.logger, "Logon successful.");
        Ok(())
    }

    fn on_resend_request(&mut self, _range: Range<u64>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn fetch_messages(&mut self) -> Result<&[&[u8]], Self::Error> {
        Ok(&[b""])
    }

    fn pending_message(&mut self) -> Option<&[u8]> {
        None
    }
}

fn logger() -> Logger {
    use slog::Drain;
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!());
    log
}
