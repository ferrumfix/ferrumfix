# Tachyonix 0.3.1 Documentation

## Overview

Tachyonix is a high-performance asynchronous, multi-producer, single-consumer (MPSC) bounded channel library for Rust. It is designed to be "extremely fast, without taking any shortcuts on correctness."

**Key Characteristics:**
- Asynchronous channel operations
- Multi-producer, single-consumer (MPSC) design
- Bounded channel with configurable capacity
- Automatic disconnection mechanism
- Thread-safe operations
- High-performance focus with correctness guarantees

## Channel Creation

### Function: `channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>)`

Creates a new asynchronous, bounded channel.

**Parameters:**
- `capacity: usize` - The maximum number of messages the channel can hold

**Returns:**
- `(Sender<T>, Receiver<T>)` - A tuple containing the sender and receiver ends

**Panic Conditions:**
- Panics if `capacity` is 0
- Panics if `capacity` is greater than `usize::MAX/2 + 1`

**Example:**
```rust
let (s, mut r) = tachyonix::channel(3);

block_on(async move {
    pool.spawn_ok(async move {
        assert_eq!(s.send("Hello").await, Ok(()));
    });
     
    assert_eq!(r.recv().await, Ok("Hello"));
});
```

## Core Components

### Sender<T>

The sending side of a channel that supports multiple producers.

**Key Features:**
- Multiple senders can be created by cloning
- Thread-safe (implements `Send` and `Sync`)
- Generic over message type `T`

**Methods:**

#### `try_send(message: T) -> Result<(), TrySendError<T>>`
Attempts to send a message immediately without waiting.

#### `send(message: T) -> Result<(), SendError<T>>` (async)
Sends a message asynchronously, waiting if necessary until capacity is available.

#### `send_timeout(message: T, deadline: D) -> Result<(), SendTimeoutError<T>>` (async)
Sends a message asynchronously with a deadline. Waits until capacity is available or the deadline elapses.
- `deadline: D` - A Future that resolves to `()`

#### `close()`
Prevents further message sending while allowing previously sent messages to be received.

#### `is_closed() -> bool`
Checks if the channel is closed (occurs if Receiver is dropped or `close()` is called).

**Trait Implementations:**
- `Clone` - Can create multiple sender instances
- `Debug` - Supports debug formatting
- `Drop` - Handles resource cleanup

### Receiver<T>

The receiving side of a channel that supports single consumer.

**Key Constraints:**
- The receiver can only be called from a single thread
- Single-consumer design

**Methods:**

#### `try_recv() -> Result<T, TryRecvError>`
Attempts to receive a message immediately without waiting.

#### `recv() -> Result<T, RecvError>` (async)
Receives a message asynchronously, waiting if necessary until one becomes available.

#### `recv_timeout() -> Result<T, RecvTimeoutError>` (async)
Receives a message with a deadline. Accepts a Future that resolves to `()`.

#### `close()`
Prevents further messages from being sent. It's recommended to follow this with a loop receiving remaining messages.

**Trait Implementations:**
- `Debug` - Supports debug formatting
- `Drop` - Handles resource cleanup
- `Stream` - Implements the futures-core Stream trait
- `Send` - Where `T: Send`
- `Sync` - Where `T: Send`

## Error Types

The crate provides comprehensive error handling through several error types:

### SendError<T>
Error returned by unsuccessful send operations.

### RecvError
Error returned by unsuccessful receive operations.

### TrySendError<T>
Error returned by unsuccessful immediate send attempts.

### TryRecvError
Error returned by unsuccessful immediate receive attempts.

### SendTimeoutError<T>
Error returned by unsuccessful timeout-based send operations.

### RecvTimeoutError
Error returned by unsuccessful timeout-based receive operations.

## Channel Behavior

### Disconnection Mechanism

**Automatic Disconnection:**
- Channel disconnects when all Senders are dropped
- Channel disconnects when the Receiver is dropped

**Manual Disconnection:**
- Can be manually disconnected by calling `close()` on any sender or receiver
- Sending attempts fail after disconnection
- Receiver can still process existing messages in the channel after disconnection

### Performance Characteristics

- Designed for extremely high performance
- Bounded channel design prevents unlimited memory growth
- Efficient asynchronous operations without blocking
- Optimized for multi-producer scenarios
- Single-consumer constraint allows for optimized receiver operations

## Usage Patterns

### Basic Channel Operations
```rust
use tachyonix;

let (sender, mut receiver) = tachyonix::channel(10);

// Sending messages
sender.send("message").await?;

// Receiving messages
let msg = receiver.recv().await?;
```

### Multi-Producer Scenario
```rust
let (sender, mut receiver) = tachyonix::channel(100);

// Clone sender for multiple producers
let sender1 = sender.clone();
let sender2 = sender.clone();

// Use in different async tasks
tokio::spawn(async move {
    sender1.send("from producer 1").await
});

tokio::spawn(async move {
    sender2.send("from producer 2").await
});
```

### Timeout Operations
```rust
// Send with timeout
let deadline = tokio::time::sleep(Duration::from_secs(5));
match sender.send_timeout(message, deadline).await {
    Ok(()) => println!("Message sent"),
    Err(e) => println!("Send timed out: {:?}", e),
}

// Receive with timeout
let deadline = tokio::time::sleep(Duration::from_secs(5));
match receiver.recv_timeout(deadline).await {
    Ok(msg) => println!("Received: {:?}", msg),
    Err(e) => println!("Receive timed out: {:?}", e),
}
```

### Graceful Shutdown
```rust
// Close the channel
sender.close();

// Process remaining messages
while let Ok(msg) = receiver.recv().await {
    process_message(msg);
}
```

## License

Licensed under MIT or Apache-2.0.

## Maintainer

Maintained by "sbarral".

---

*This documentation is based on tachyonix version 0.3.1 from docs.rs*