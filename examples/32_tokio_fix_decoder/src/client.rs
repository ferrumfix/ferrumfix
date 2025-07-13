use log::info;
use rustyfix::SetField;
use rustyfix::tagvalue::Encoder;
use rustyfix_dictionary::Dictionary;
use std::time::Duration;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::time::sleep;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Other error: {0}")]
    Other(String),
}

impl From<Box<dyn std::error::Error>> for ClientError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ClientError::Other(err.to_string())
    }
}

type Result<T> = std::result::Result<T, ClientError>;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize simple logger for this example
    env_logger::init();

    info!("Starting FIX Client - connecting to 127.0.0.1:8888");

    // Connect to the FIX server
    let mut stream = TcpStream::connect("127.0.0.1:8888").await?;
    info!("Connected to FIX server");

    // Create encoder for FIX messages
    let dict = Dictionary::fix42().map_err(|e| ClientError::Other(e.to_string()))?;
    let mut encoder = Encoder::new();

    // Send a Logon message
    info!("Sending Logon message...");
    let logon_msg = encode_message(&mut encoder, &dict, "A", &create_logon_fields())?;
    stream.write_all(&logon_msg).await?;
    stream.flush().await?;

    sleep(Duration::from_millis(500)).await;

    // Send a Heartbeat message
    info!("Sending Heartbeat message...");
    let heartbeat_msg = encode_message(&mut encoder, &dict, "0", &create_heartbeat_fields())?;
    stream.write_all(&heartbeat_msg).await?;
    stream.flush().await?;

    sleep(Duration::from_millis(500)).await;

    // Send a New Order Single message
    info!("Sending New Order Single message...");
    let order_msg = encode_message(&mut encoder, &dict, "D", &create_new_order_single_fields())?;
    stream.write_all(&order_msg).await?;
    stream.flush().await?;

    sleep(Duration::from_millis(500)).await;

    // Send a malformed message to test error handling
    info!("Sending malformed message to test error handling...");
    let malformed = b"8=FIX.4.2\x019=50\x0135=D\x01INVALID_FIELD\x0110=123\x01";
    stream.write_all(malformed).await?;
    stream.flush().await?;

    sleep(Duration::from_millis(500)).await;

    info!("Client finished sending messages");
    Ok(())
}

fn encode_message(
    encoder: &mut Encoder,
    _dict: &Dictionary,
    msg_type: &str,
    fields: &[(u32, &str)],
) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();

    // Start the message with the FIX version and message type
    let mut msg = encoder.start_message(b"FIX.4.2", &mut buffer, msg_type.as_bytes());

    // Add all the fields
    for &(tag, value) in fields {
        msg.set(tag, value);
    }

    // Finish the message (automatically adds CheckSum)
    let (_data, _len) = msg.done();

    Ok(buffer)
}

fn create_logon_fields() -> Vec<(u32, &'static str)> {
    vec![
        (49, "CLIENT"),            // SenderCompID
        (56, "SERVER"),            // TargetCompID
        (34, "1"),                 // MsgSeqNum
        (52, "20240115-10:30:00"), // SendingTime
        (98, "0"),                 // EncryptMethod = None
        (108, "30"),               // HeartBtInt = 30 seconds
    ]
}

fn create_heartbeat_fields() -> Vec<(u32, &'static str)> {
    vec![
        (49, "CLIENT"),            // SenderCompID
        (56, "SERVER"),            // TargetCompID
        (34, "2"),                 // MsgSeqNum
        (52, "20240115-10:30:30"), // SendingTime
    ]
}

fn create_new_order_single_fields() -> Vec<(u32, &'static str)> {
    vec![
        (49, "CLIENT"),            // SenderCompID
        (56, "SERVER"),            // TargetCompID
        (34, "3"),                 // MsgSeqNum
        (52, "20240115-10:31:00"), // SendingTime
        (11, "ORDER123"),          // ClOrdID
        (21, "1"),                 // HandlInst = Automated execution
        (55, "AAPL"),              // Symbol
        (54, "1"),                 // Side = Buy
        (60, "20240115-10:31:00"), // TransactTime
        (38, "100"),               // OrderQty
        (40, "2"),                 // OrdType = Limit
        (44, "150.50"),            // Price
        (59, "0"),                 // TimeInForce = Day
    ]
}
