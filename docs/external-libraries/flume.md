# Flume Crate Documentation Summary for LLMs

## 1. Crate Overview

**Flume** is a high-performance, multi-producer, multi-consumer (MPMC) channel library for Rust. It aims to be a fast, safe, and flexible alternative to `std::sync::mpsc` and `crossbeam-channel`.

**Key Features**:
- Supports unbounded, bounded, and rendezvous (zero-capacity bounded) channels.
- `Sender` and `Receiver` types are `Send + Sync + Clone`.
- Offers a drop-in replacement API for `std::sync::mpsc`.
- Provides advanced features like send timeouts, deadlines, and asynchronous support.
- Includes an ergonomic `select`-like interface for managing multiple channel operations.
- Written entirely in safe Rust.

## 2. Core Concepts

### Channels
Channels in Flume allow communication between different parts of a program, especially across threads. They consist of two main parts: a `Sender` and a `Receiver`.

### Senders (`Sender<T>`)
- The transmitting end of a channel.
- Used to send messages of type `T` into the channel.
- Can be cloned to have multiple producers sending to the same channel.
- Dropping all `Sender`s (or `WeakSender`s failing to upgrade) will cause the channel to be closed from the sending side.

### Receivers (`Receiver<T>`)
- The receiving end of a channel.
- Used to receive messages of type `T` from the channel.
- Can be cloned to have multiple consumers (work-stealing pattern, not broadcast). Each message is received by only one `Receiver`.
- Dropping all `Receiver`s will cause the channel to be closed from the receiving side.

### Bounded Channels
- Created using `flume::bounded(cap: usize)`.
- Have a fixed maximum capacity.
- If the channel is full, `send` operations will block until space becomes available. `try_send` can be used for non-blocking sends.
- A capacity of `0` creates a "rendezvous" channel, where senders block until a receiver is ready to take the message.

### Unbounded Channels
- Created using `flume::unbounded()`.
- Have no practical limit on capacity (limited by system memory).
- `send` operations on an unbounded channel typically do not block (unless memory allocation fails).

## 3. Key Structs and Enums

### 3.1. Core Types
-   **`Sender<T>`**: The transmitting end of a channel.
    -   `send(msg: T)`: Sends a message, blocking if the channel is full (for bounded channels).
    -   `try_send(msg: T)`: Attempts to send a message without blocking.
    -   `send_timeout(msg: T, dur: Duration)`: Sends with a timeout.
    -   `send_deadline(msg: T, deadline: Instant)`: Sends with a deadline.
    -   `downgrade()`: Creates a `WeakSender`.
    -   `is_disconnected()`: Checks if all receivers are dropped.
-   **`Receiver<T>`**: The receiving end of a channel.
    -   `recv()`: Receives a message, blocking if the channel is empty.
    -   `try_recv()`: Attempts to receive a message without blocking.
    -   `recv_timeout(dur: Duration)`: Receives with a timeout.
    -   `recv_deadline(deadline: Instant)`: Receives with a deadline.
    -   `is_disconnected()`: Checks if all senders are dropped and the channel is empty.
-   **`WeakSender<T>`**: A sender that does not prevent the channel from being closed.
    -   `upgrade()`: Attempts to upgrade to a `Sender`. Returns `None` if the channel is already closed.

### 3.2. Error Types
-   **`SendError<T>`**: Returned when sending on a channel where all receivers are dropped. Contains the unsent message `T`.
-   **`TrySendError<T>`**: Returned by `try_send`.
    -   `Full(T)`: Channel is bounded and full. Contains the unsent message.
    -   `Disconnected(T)`: All receivers dropped. Contains the unsent message.
-   **`SendTimeoutError<T>`**: Returned by `send_timeout` and `send_deadline`.
    -   `Timeout(T)`: Send operation timed out. Contains the unsent message.
    -   `Disconnected(T)`: All receivers dropped. Contains the unsent message.
-   **`RecvError`**: Returned by `recv` when all senders are dropped and the channel is empty.
    -   `Disconnected`: The only variant.
-   **`TryRecvError`**: Returned by `try_recv`.
    -   `Empty`: Channel is empty.
    -   `Disconnected`: All senders dropped and channel empty.
-   **`RecvTimeoutError`**: Returned by `recv_timeout` and `recv_deadline`.
    -   `Timeout`: Receive operation timed out.
    -   `Disconnected`: All senders dropped and channel empty.
-   **`select::SelectError`**: Returned by `Selector::wait_timeout` or `Selector::wait_deadline`.
    -   `Timeout`: The select operation timed out.

### 3.3. Iterators
-   **`Iter<'a, T>`**: A blocking iterator over messages from a `Receiver`. Created by `Receiver::iter()`.
-   **`TryIter<'a, T>`**: A non-blocking iterator over messages currently in a `Receiver`. Created by `Receiver::try_iter()`.
-   **`IntoIter<T>`**: An owned, blocking iterator over messages from a `Receiver`. Created by `Receiver::into_iter()`.
-   **`Drain<'a, T>`**: A fixed-size iterator over messages drained from a `Receiver` at the moment of creation. Created by `Receiver::drain()`.

## 4. Key Functions

-   **`flume::bounded<T>(cap: usize) -> (Sender<T>, Receiver<T>)`**: Creates a new channel with a specified maximum capacity.
-   **`flume::unbounded<T>() -> (Sender<T>, Receiver<T>)`**: Creates a new channel with no capacity limit.

## 5. Modules

### 5.1. `flume::async`
Provides types for asynchronous interaction with Flume channels, compatible with async runtimes.
-   **`RecvFut<'a, T>`**: A future for asynchronously receiving a message.
    - Created by `Receiver::recv_async()` or `Receiver::into_recv_async()`.
-   **`SendFut<'a, T>`**: A future for asynchronously sending a message.
    - Created by `Sender::send_async()` or `Sender::into_send_async()`.
-   **`RecvStream<'a, T>`**: A stream for asynchronously receiving messages.
    - Implements `futures::Stream`.
    - Created by `Receiver::stream()` or `Receiver::into_stream()`.
-   **`SendSink<'a, T>`**: A sink for asynchronously sending messages.
    - Implements `futures::Sink`.
    - Created by `Sender::sink()` or `Sender::into_sink()`.

### 5.2. `flume::select`
Provides a mechanism to wait on multiple channel operations simultaneously.
-   **`Selector<'a, T>`**: A type used to wait upon multiple blocking send or receive operations.
    -   `Selector::new()`: Creates a new selector.
    -   `recv(&'a Receiver<U>, FnMut(Result<U, RecvError>) -> T)`: Adds a receive operation.
    -   `send(&'a Sender<U>, U, FnMut(Result<(), SendError<U>>) -> T)`: Adds a send operation.
    -   `wait()`: Blocks until one of the registered operations completes, then runs its handler.
    -   `wait_timeout(Duration)` / `wait_deadline(Instant)`: Waits with a timeout/deadline.
-   **`SelectError`**: Error type for select operations (currently only `Timeout`).

## 6. General Usage Notes for LLMs

-   When deciding between bounded and unbounded channels, consider backpressure. Bounded channels provide natural backpressure.
-   Error types often contain the message that failed to send/receive, allowing for recovery or logging.
-   Cloning `Receiver`s creates a multi-consumer setup (work stealing), not a broadcast. Each message is delivered once.
-   `WeakSender` is useful for breaking cycles or allowing channels to close even if some references to senders exist but are not critical.
-   The `async` module allows Flume channels to be integrated into `async/await` code seamlessly.
-   The `select` module is powerful for scenarios where a thread needs to react to the first available message from multiple sources or the first successful send to multiple destinations.
