use futures_util::stream::StreamExt;
use log::{error, info, warn};
use rustyfix::tagvalue::TokioDecoder;
use rustyfix_dictionary::Dictionary;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::Framed;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("FIX decode error: {0}")]
    Decode(#[from] rustyfix::tagvalue::DecodeError),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<Box<dyn std::error::Error>> for ServerError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ServerError::Other(err.to_string())
    }
}

type Result<T> = std::result::Result<T, ServerError>;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize fastrace with console reporter
    let reporter = fastrace::collector::ConsoleReporter {};
    fastrace::set_reporter(reporter, fastrace::collector::Config::default());

    // Initialize simple logger for this example
    env_logger::init();

    info!("Starting FIX Server on 127.0.0.1:8888");

    let listener = TcpListener::bind("127.0.0.1:8888").await?;
    info!("FIX Server listening on 127.0.0.1:8888");

    while let Ok((stream, addr)) = listener.accept().await {
        info!("New connection from {addr}");
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream).await {
                error!("Error handling client {addr}: {e}");
            }
        });
    }

    info!("Client disconnected");
    Ok(())
}

async fn handle_client(stream: TcpStream) -> Result<()> {
    // Create a FIX decoder with FIX 4.2 dictionary
    let dict = Dictionary::fix42().map_err(|e| ServerError::Other(e.to_string()))?;
    let decoder = TokioDecoder::new(dict);

    // Frame the TCP stream with the FIX decoder
    let mut framed = Framed::new(stream, decoder);

    info!("Client connected, waiting for FIX messages...");

    while let Some(result) = framed.next().await {
        match result {
            Ok(message) => {
                info!("Received FIX message:");

                // Display message type
                match message.msg_type() {
                    Ok(msg_type) => info!("  Message Type: {msg_type}"),
                    Err(e) => warn!("  Could not parse message type: {e}"),
                }

                // Display message length and raw bytes
                info!("  Length: {} bytes", message.len());
                info!("  Raw: {}", String::from_utf8_lossy(message.as_bytes()));

                // Display all fields
                info!("  Fields:");
                for (tag, value) in message.fields() {
                    let value_str = String::from_utf8_lossy(value);
                    info!("    {tag}: {value_str}");
                }

                // Process specific message types
                match message.msg_type().as_deref() {
                    Ok("A") => {
                        info!("Processing Logon message");
                        // Handle logon logic here
                    }
                    Ok("D") => {
                        info!("Processing New Order Single");
                        // Extract order details
                        if let Ok(symbol) = message.get::<String>(55) {
                            info!("  Symbol: {symbol}");
                        }
                        if let Ok(side) = message.get::<String>(54) {
                            info!("  Side: {side}");
                        }
                        if let Ok(quantity) = message.get::<u32>(38) {
                            info!("  Quantity: {quantity}");
                        }
                    }
                    Ok("0") => {
                        info!("Processing Heartbeat message");
                        // Handle heartbeat logic here
                    }
                    Ok(msg_type) => {
                        info!("Processing message type: {msg_type}");
                    }
                    Err(e) => {
                        warn!("Could not determine message type: {e}");
                    }
                }

                println!(); // Add spacing between messages
            }
            Err(e) => {
                error!("Error decoding FIX message: {e}");
                // You could choose to break here or continue depending on error handling strategy
                break;
            }
        }
    }

    info!("Client disconnected");
    Ok(())
}
