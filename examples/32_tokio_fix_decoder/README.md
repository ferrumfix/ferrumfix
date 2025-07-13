# Tokio FIX Decoder Example

This example demonstrates how to use RustyFix's `TokioDecoder` for asynchronous FIX message processing using Tokio.

## What This Example Shows

- **Async FIX Server**: TCP server that accepts FIX messages using `TokioDecoder`
- **Message Processing**: Parse and handle different FIX message types (Logon, Heartbeat, NewOrderSingle)
- **Error Handling**: Graceful handling of malformed or invalid FIX messages
- **Field Extraction**: Access specific fields from FIX messages using type-safe getters
- **Real-time Processing**: Stream-based processing of incoming FIX messages

## Architecture

```
Client ──TCP──> Server (TokioDecoder) ──> Message Processing
```

The `TokioDecoder` automatically:
- **Frames messages**: Detects complete FIX messages in the TCP stream
- **Prevents data loss**: Only consumes exact message bytes (no buffer corruption)
- **Validates structure**: Ensures proper FIX message format
- **Provides owned data**: Returns `OwnedMessage` instances that don't borrow from buffers

## Running the Example

### Terminal 1: Start the Server
```bash
cargo run --bin fix_server
```

### Terminal 2: Run the Client  
```bash
cargo run --bin fix_client
```

## Expected Output

**Server Output:**
```
INFO Starting FIX Server on 127.0.0.1:8888
INFO FIX Server listening on 127.0.0.1:8888
INFO New connection from 127.0.0.1:xxxxx
INFO Client connected, waiting for FIX messages...
INFO Received FIX message:
INFO   Message Type: A
INFO   Length: 89 bytes
INFO   Raw: 8=FIX.4.2☁9=52☁35=A☁49=CLIENT☁56=SERVER☁34=1☁52=20240115-10:30:00☁98=0☁108=30☁10=123☁
INFO   Fields:
INFO     8: FIX.4.2
INFO     9: 52
INFO     35: A
INFO     49: CLIENT
INFO     56: SERVER
INFO     34: 1
INFO     52: 20240115-10:30:00
INFO     98: 0
INFO     108: 30
INFO     10: 123
INFO Processing Logon message
```

## Key Features Demonstrated

### 1. **TokioDecoder Setup**
```rust
let dict = Dictionary::fix42();
let decoder = TokioDecoder::new(dict);
let mut framed = Framed::new(stream, decoder);
```

### 2. **Async Message Processing**
```rust
while let Some(result) = framed.next().await {
    match result {
        Ok(message) => {
            // Process FIX message
        }
        Err(e) => {
            // Handle decode errors
        }
    }
}
```

### 3. **Type-Safe Field Access**
```rust
let msg_type = message.msg_type()?;           // String
let symbol = message.get::<String>(55)?;      // Symbol
let quantity = message.get::<u32>(38)?;       // OrderQty
let price = message.get::<f64>(44)?;          // Price
```

### 4. **Error Resilience**
- Malformed messages don't crash the decoder
- Invalid fields are handled gracefully
- Connection errors are logged and handled

## Message Types Processed

| MsgType | Description | Fields Extracted |
|---------|-------------|------------------|
| `A` | Logon | Basic session setup |
| `0` | Heartbeat | Keep-alive processing |
| `D` | NewOrderSingle | Symbol, Side, Quantity, Price |

## Performance Characteristics

- **Zero-copy when possible**: Fields reference original byte slices
- **Minimal allocations**: Only allocates for owned message storage
- **Async-first**: Non-blocking I/O throughout
- **Memory safe**: No unsafe buffer sharing or aliasing

## Extending the Example

### Add More Message Types
```rust
match message.msg_type().as_deref() {
    Ok("8") => {
        info!("Processing Execution Report");
        // Handle execution report...
    }
    Ok("G") => {
        info!("Processing Order Cancel Replace");
        // Handle order modification...
    }
    // Add more message types as needed
}
```

### Add Response Messages
Combine with `Encoder` to send responses:
```rust
let mut encoder = Encoder::new();
let response = create_execution_report();
let encoded = encoder.encode(&response, &dict)?;
stream.write_all(&encoded).await?;
```

### Add Session Management
Track sequence numbers, heartbeat intervals, and session state for production FIX implementations.

## Production Considerations

1. **Sequence Number Validation**: Track and validate MsgSeqNum(34)
2. **Heartbeat Management**: Implement heartbeat timeouts and TestRequest handling
3. **Checksum Validation**: Ensure message integrity in production
4. **Session Recovery**: Handle gap fills and sequence resets
5. **Error Recovery**: Implement Reject messages for invalid requests

This example provides a solid foundation for building production FIX applications with async Rust. 