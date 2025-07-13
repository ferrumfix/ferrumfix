# Yawc (Yet Another WebSocket Client) - Complete Documentation

**Version**: 0.2.4
**License**: LGPL-3.0-or-later
**Platform Support**: Linux (i686, x86_64), WebAssembly

## Overview

**yawc** is a fast, secure WebSocket implementation with full RFC 6455 compliance and compression support. It provides a high-performance, production-ready WebSocket library that combines essential features not found together in other Rust WebSocket libraries.

### Motivation

"While several WebSocket libraries exist... none provide the full combination of features needed for high-performance, production-ready applications." Yawc fills this gap with:

- Zero-copy design for maximum performance
- Built-in TLS support with rustls
- Automatic control frame handling
- RFC-compliant permessage-deflate compression
- Cross-platform support (native and WASM)
- Ergonomic async/await API

## Core Features

- **Full RFC 6455 Compliance**: Complete WebSocket protocol implementation, Autobahn test suite compliant
- **RFC 7692 Compression**: permessage-deflate with fine-grained control
- **Automatic Control Frame Handling**: Transparent ping/pong and close frame management
- **Zero-Copy Design**: Efficient frame processing without unnecessary allocations
- **Async/Await Support**: Built on Tokio with Stream/Sink trait implementations
- **WASM Support**: Works in browser environments (with some limitations)
- **Flexible HTTP Client Options**: Native implementation, reqwest integration, or custom
- **Memory Safety**: Configurable limits to prevent DoS attacks
- **UTF-8 Validation**: Optional validation for text frames
- **TLS Support**: Built-in secure WebSocket connections using rustls

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
yawc = { version = "0.2", features = ["log", "zlib"] }
```

### Feature Flags

- **`reqwest`**: Use reqwest as HTTP client (recommended for WebSocket clients)
- **`axum`**: Enable Axum web framework integration for server-side WebSocket
- **`log`**: Enable debug logging for connection events and protocol details
- **`zlib`**: Advanced compression options with window size control
- **`json`**: JSON serialization support for sending/receiving JSON data
- **Default features**: Basic WebSocket functionality with native HTTP client

## Module Structure

### Core Modules

1. **`yawc::frame`** - WebSocket frame implementation
   - `Frame`: Full mutable frame representation
   - `FrameView`: Lightweight immutable frame view
   - `OpCode`: Frame type enumeration
   - Frame serialization and deserialization

2. **`yawc::close`** - Connection closure handling
   - `CloseCode`: Standard close codes per RFC 6455
   - Close frame parsing and generation

3. **`yawc::codec`** - Low-level encoding/decoding
   - `Codec`: Combined encoder/decoder
   - `Encoder`: Frame serialization
   - `Decoder`: Frame parsing with state management

## Client Connection APIs

### 1. Simple Connection

```rust
use yawc::WebSocket;

// Connect with default options
let ws = WebSocket::connect("wss://echo.websocket.org".parse() ? )
.await?;
```

### 2. Connection with Options

```rust
use yawc::{WebSocket, Options};

let options = Options::builder()
.compression(true)
.max_frame_size(1024 * 1024) // 1MB
.build();

let ws = WebSocket::connect("wss://example.com/ws".parse() ? )
.with_options(options)
.await?;
```

### 3. Connection with Custom Headers

```rust
let ws = WebSocket::connect("wss://example.com/ws".parse() ? )
.with_header("Authorization", "Bearer token123")
.with_header("User-Agent", "MyApp/1.0")
.await?;
```

### 4. Using Existing TCP Stream

```rust
use tokio::net::TcpStream;
use yawc::{WebSocket, Options};

let stream = TcpStream::connect("example.com:80").await?;
let ws = WebSocket::handshake(
"ws://example.com/socket".parse() ?,
stream,
Options::default ()
).await?;
```

### 5. Using Reqwest Client

```rust
use reqwest::Client;
use yawc::{WebSocket, Options};

let client = Client::builder()
.timeout(Duration::from_secs(30))
.build() ?;

let ws = WebSocket::reqwest(
"wss://example.com/ws".parse() ?,
client,
Options::default ()
).await?;
```

## Sending and Receiving Messages

### Basic Message Exchange

```rust
use futures::{SinkExt, StreamExt};
use yawc::frame::{FrameView, OpCode};

// Send text message
ws.send(FrameView::text("Hello WebSocket!")).await?;

// Send binary message
ws.send(FrameView::binary(vec![1, 2, 3, 4])).await?;

// Receive messages
while let Some(frame) = ws.next().await {
match frame.opcode {
OpCode::Text => {
let text = std::str::from_utf8( &frame.payload) ?;
println ! ("Text: {}", text);
}
OpCode::Binary => {
println ! ("Binary: {} bytes", frame.payload.len());
}
OpCode::Close => {
println ! ("Connection closed");
break;
}
_ => {} // Ping/Pong handled automatically
}
}
```

### Using Stream/Sink Traits

```rust
use futures::{stream::StreamExt, sink::SinkExt};

// As a Stream
let ws_stream = ws.map( | frame| {
// Transform frames
frame
});

// As a Sink
let mut ws_sink = ws;
ws_sink.send(FrameView::text("Message")).await?;
ws_sink.flush().await?;
```

## Advanced Usage

### Splitting Read/Write

```rust
let (ws_read, ws_write) = ws.split();

// Spawn reader task
tokio::spawn( async move {
let mut ws_read = ws_read;
while let Some(frame) = ws_read.next().await {
// Handle incoming frames
}
});

// Use writer in main task
ws_write.send(FrameView::text("Hello")).await?;
```

### Low-Level Frame Access

```rust
// WARNING: This disables automatic protocol handling
let (stream, read_half, write_half) = unsafe { ws.split_stream() };

// Manual frame processing required
std::future::poll_fn( | cx| {
read_half.poll_frame( & mut stream, cx)
}).await?;
```

## Frame Types and OpCodes

### OpCode Enum

The `OpCode` enum represents WebSocket frame types as defined in RFC 6455:

```rust
pub enum OpCode {
    // Data Frame OpCodes
    Continuation = 0x0,  // Continues a fragmented message
    Text = 0x1,         // Contains UTF-8 encoded text data
    Binary = 0x2,       // Contains raw binary data

    // Control Frame OpCodes
    Close = 0x8,        // Initiates or confirms connection closure
    Ping = 0x9,         // Tests connection liveness, requires Pong response
    Pong = 0xA,         // Responds to a Ping frame
}

impl OpCode {
    /// Returns true if this is a control frame
    pub fn is_control(&self) -> bool {
        matches!(self, OpCode::Close | OpCode::Ping | OpCode::Pong)
    }
}
```

### Control Frame Constraints

Control frames have special requirements:
- Cannot be fragmented
- Maximum payload size of 125 bytes
- Must be processed immediately upon receipt
- Take priority over data frames

## FrameView API

The `FrameView` struct provides a lightweight, immutable representation of WebSocket frames:

```rust
pub struct FrameView {
    pub opcode: OpCode,     // Frame type indicator
    pub payload: Bytes,     // Immutable payload data
}

impl FrameView {
    // Factory methods for creating frames
    pub fn text(text: impl Into<String>) -> Self;
    pub fn binary(data: impl Into<Bytes>) -> Self;
    pub fn close(code: CloseCode, reason: &str) -> Self;
    pub fn ping(payload: impl Into<Bytes>) -> Self;
    pub fn pong(payload: impl Into<Bytes>) -> Self;

    // Utility methods for frame inspection
    pub fn close_code(&self) -> Option<CloseCode>;
    pub fn close_reason(&self) -> Option<&str>;
    pub fn as_str(&self) -> &str;  // Panics if payload is not valid UTF-8
}
```

### Frame vs FrameView

- **`Frame`**: Full mutable frame with all protocol metadata (fin flag, masking, etc.)
- **`FrameView`**: Simplified immutable view for application-level usage
- Use `FrameView` for normal WebSocket operations
- Use `Frame` only when you need low-level protocol control

## Options Configuration

### Options Struct

The `Options` struct provides fine-grained control over WebSocket behavior:

```rust
pub struct Options {
    pub max_payload_read: Option<usize>,    // Max incoming message size (default: 1 MiB)
    pub max_read_buffer: Option<usize>,     // Max buffer for fragments (default: 2 MiB)
    pub compression: Option<DeflateOptions>, // Compression configuration
    pub check_utf8: bool,                   // Validate UTF-8 in text frames (default: false)
}
```

### Builder Pattern Configuration

```rust
let options = Options::default()
    // Compression settings
    .with_compression_level(CompressionLevel::new(6))  // 0-9 scale
    .with_client_max_window_bits(15)                   // 8-15 bits
    .with_server_max_window_bits(15)                   // 8-15 bits
    .client_no_context_takeover(true)                  // Reset context per message
    .server_no_context_takeover(false)                 // Keep context between messages

    // Safety limits
    .with_max_payload_read(10 * 1024 * 1024)          // 10MB max message
    .with_max_read_buffer(20 * 1024 * 1024)           // 20MB fragment buffer

    // Validation
    .with_utf8()                                       // Enable UTF-8 validation

    // Or disable compression entirely
    .without_compression();
```

### DeflateOptions

Fine-grained compression control per RFC 7692:

```rust
pub struct DeflateOptions {
    pub level: CompressionLevel,                      // 0-9 compression level
    pub server_max_window_bits: Option<u8>,           // Server window size (8-15)
    pub client_max_window_bits: Option<u8>,           // Client window size (8-15)
    pub server_no_context_takeover: bool,             // Reset server context
    pub client_no_context_takeover: bool,             // Reset client context
}
```

### Compression Levels

- **0**: No compression (fastest)
- **1-3**: Low compression, minimal CPU usage
- **4-6**: Balanced compression (default)
- **7-9**: Maximum compression, highest CPU usage

## Error Handling

### WebSocketError Enum

Comprehensive error types for all WebSocket operations:

```rust
pub enum WebSocketError {
    // Protocol Errors
    InvalidFragment,           // Fragment violates protocol rules
    InvalidOpCode,            // Unknown or invalid OpCode
    ReservedBitSet,           // Reserved bits must be 0
    UnexpectedContinuation,   // Continuation without initial frame

    // Data Validation Errors
    InvalidUTF8,              // Invalid UTF-8 in text frame
    PayloadTooLarge,          // Exceeds max_payload_read limit
    MessageTooLarge,          // Fragmented message exceeds buffer

    // Connection Errors
    ConnectionClosed,         // Operation on closed connection
    InvalidHandshake,         // WebSocket handshake failed
    InvalidHeader,            // Malformed HTTP headers

    // Frame Errors
    InvalidCloseFrame,        // Improperly formatted close frame
    FrameTooLarge,           // Single frame exceeds limits
    ControlFrameFragmented,   // Control frames cannot fragment

    // Network/IO Errors
    Io(std::io::Error),      // Underlying I/O error
    Http(http::Error),       // HTTP protocol error

    // Other Errors
    Json(serde_json::Error), // JSON serialization error (with json feature)
}
```

### Error Handling Patterns

```rust
use yawc::WebSocketError;

// Comprehensive error handling
match ws.send(frame).await {
    Ok(_) => println!("Sent successfully"),
    Err(WebSocketError::ConnectionClosed) => {
        // Handle reconnection logic
        println!("Connection lost, attempting reconnect...");
    }
    Err(WebSocketError::PayloadTooLarge) => {
        // Split message or compress
        println!("Message too large, consider compression");
    }
    Err(WebSocketError::InvalidUTF8) => {
        // Send as binary instead
        println!("Invalid UTF-8, sending as binary");
    }
    Err(e) => {
        // Log and handle unexpected errors
        eprintln!("WebSocket error: {}", e);
    }
}
```

## Advanced Features

### WebSocket Struct Methods

The main `WebSocket` struct provides comprehensive functionality:

```rust
impl WebSocket {
    // Connection establishment
    pub async fn connect(url: Url) -> WebSocketBuilder;
    pub async fn handshake(url: Url, io: impl AsyncRead + AsyncWrite, options: Options) -> Result<Self>;
    pub async fn upgrade(request: Request<Body>) -> Result<Self>;

    // Frame operations
    pub async fn next_frame(&mut self) -> Option<FrameView>;

    // JSON support (requires json feature)
    #[cfg(feature = "json")]
    pub async fn send_json<T: Serialize>(&mut self, value: &T) -> Result<()>;

    // Low-level access (unsafe)
    pub unsafe fn split_stream(self) -> (Stream, ReadHalf, WriteHalf);
}

// Stream trait implementation
impl Stream for WebSocket {
    type Item = FrameView;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

// Sink trait implementation
impl Sink<FrameView> for WebSocket {
    type Error = WebSocketError;
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
    fn start_send(self: Pin<&mut Self>, item: FrameView) -> Result<()>;
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>>;
}
```

### WebSocketBuilder

Advanced connection configuration:

```rust
impl WebSocketBuilder {
    // TLS configuration
    pub fn with_connector(self, connector: impl Into<TlsConnector>) -> Self;

    // Network configuration
    pub fn with_tcp_address(self, addr: impl Into<String>) -> Self;

    // WebSocket options
    pub fn with_options(self, options: Options) -> Self;

    // HTTP request customization
    pub fn with_request<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut http::Request<()>);
}
```

### Codec Module

Low-level frame processing:

```rust
pub struct Codec {
    // Combined encoder/decoder for bidirectional communication
}

pub struct Encoder {
    // Serializes WebSocket frames to bytes
}

pub struct Decoder {
    // Parses bytes into WebSocket frames with state management
}
```

## Close Codes

Standard close codes per RFC 6455:

```rust
pub enum CloseCode {
    NormalClosure = 1000,      // Normal connection closure
    GoingAway = 1001,          // Server going down or browser navigating away
    ProtocolError = 1002,      // Protocol error detected
    UnsupportedData = 1003,    // Received data type cannot be accepted
    NoStatusReceived = 1005,   // No close code received
    AbnormalClosure = 1006,    // Connection closed abnormally
    InvalidPayload = 1007,     // Inconsistent message data
    PolicyViolation = 1008,    // Generic policy violation
    MessageTooBig = 1009,      // Message too large to process
    ExtensionError = 1010,     // Client expected extension not offered
    UnexpectedError = 1011,    // Server encountered unexpected condition
    // Reserved codes 1004, 1012-1014, 1015
    // Custom codes 3000-3999, 4000-4999
}
```

## Best Practices

### Connection Management

1. **Always handle errors**: WebSocket connections can fail at any time
   - Implement comprehensive error handling for all operations
   - Consider automatic reconnection with exponential backoff

2. **Use appropriate timeouts**:
   - Set connection timeouts to detect dead connections
   - Configure ping/pong intervals for keepalive

3. **Resource limits**:
   - Set `max_payload_read` to prevent memory exhaustion
   - Configure `max_read_buffer` for fragmented messages
   - Monitor memory usage in high-traffic scenarios

### Performance Optimization

4. **Compression strategy**:
   - Use compression for text-heavy payloads
   - Disable for already-compressed data (images, video)
   - Consider `no_context_takeover` for memory-constrained environments
   - Balance compression level with CPU usage

5. **Message batching**:
   - Combine small messages when possible
   - Use binary frames for structured data
   - Consider message fragmentation for large payloads

### Security Considerations

6. **Validate inputs**:
   - Enable UTF-8 validation for untrusted sources
   - Implement application-level message validation
   - Use TLS for sensitive data

7. **Handle close frames**:
   - Always send close frames before disconnecting
   - Process close codes to understand disconnection reasons
   - Implement graceful shutdown procedures

## Example: Complete Client

```rust
use futures::{SinkExt, StreamExt};
use yawc::{WebSocket, Options, frame::{FrameView, OpCode}};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure connection
    let options = Options::builder()
        .compression(true)
        .ping_interval(Duration::from_secs(30))
        .max_message_size(10 * 1024 * 1024) // 10MB
        .build();

    // Connect with options
    let mut ws = WebSocket::connect("wss://echo.websocket.org".parse()?)
        .with_options(options)
        .with_header("User-Agent", "MyApp/1.0")
        .await?;

    // Send initial message
    ws.send(FrameView::text("Hello from yawc!")).await?;

    // Process messages
    while let Some(frame) = ws.next().await {
        match frame.opcode {
            OpCode::Text => {
                let text = std::str::from_utf8(&frame.payload)?;
                println!("Received text: {}", text);

                // Echo back
                ws.send(FrameView::text(format!("Echo: {}", text))).await?;
            }
            OpCode::Binary => {
                println!("Received {} bytes", frame.payload.len());
            }
            OpCode::Close => {
                println!("Server closed connection");
                break;
            }
            _ => {} // Control frames handled automatically
        }
    }

    Ok(())
}
```

## Performance Characteristics

### Zero-Copy Design

- Frames are processed without unnecessary allocations
- `Bytes` type enables efficient payload sharing
- Direct I/O operations minimize memory copies

### Compression Performance

| Level | Compression Ratio | CPU Usage | Use Case |
|-------|------------------|-----------|----------|
| 0     | 1.0x (none)      | Minimal   | Binary data, low latency |
| 1-3   | 1.5-2x           | Low       | Real-time applications |
| 4-6   | 2-3x             | Moderate  | General purpose |
| 7-9   | 3-4x             | High      | Bandwidth-constrained |

### Memory Usage

- Base WebSocket overhead: ~1KB per connection
- Compression context: 32-256KB depending on window size
- Frame buffers: Configurable via Options

## Integration Examples

### With Tokio Streams

```rust
use tokio_stream::StreamExt;
use futures::sink::SinkExt;

// Process messages with stream combinators
let processed = ws
    .filter_map(|frame| {
        match frame.opcode {
            OpCode::Text => Some(frame.payload),
            _ => None
        }
    })
    .map(|payload| {
        // Transform payload
        payload
    });
```

### With Select! Macro

```rust
use tokio::select;

loop {
    select! {
        Some(frame) = ws.next() => {
            // Handle incoming frame
        }
        Some(msg) = rx.recv() => {
            // Send outgoing message
            ws.send(FrameView::text(msg)).await?;
        }
        else => break,
    }
}
```

## Troubleshooting

### Common Issues

1. **"Invalid handshake" errors**:
   - Check URL format (ws:// or wss://)
   - Verify server supports WebSocket
   - Check for proxy interference

2. **"Connection closed" during operation**:
   - Implement automatic reconnection
   - Check for network timeouts
   - Verify keepalive configuration

3. **High memory usage**:
   - Reduce compression window size
   - Lower max_payload_read limits
   - Enable no_context_takeover

4. **Performance issues**:
   - Disable UTF-8 validation if trusted
   - Reduce compression level
   - Use binary frames for data

## Summary

Yawc provides a comprehensive, high-performance WebSocket implementation that combines:

- Full protocol compliance (RFC 6455, RFC 7692)
- Production-ready features (compression, TLS, limits)
- Excellent performance (zero-copy, efficient compression)
- Ergonomic API (async/await, Stream/Sink traits)
- Cross-platform support (native and WASM)

The library is designed for developers who need a robust, efficient WebSocket implementation with the flexibility to handle diverse use cases from simple echo clients to high-performance trading systems.