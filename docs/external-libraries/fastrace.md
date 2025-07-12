## `fastrace_macro` Crate Documentation (Concise)

**Version:** 0.7.9

**Overview:**
`fastrace_macro` provides the `#[trace]` attribute macro to simplify `fastrace` tracing integration by automating span creation and reducing boilerplate.

**The `#[trace]` Attribute Macro:**

* **Purpose:** Automates `fastrace::Span` creation for annotated functions, focusing logic over tracing infrastructure and cleaning the codebase.
* **Key Requirement: Local Parent Context:** For `#[trace]` to work, the annotated function **must** be called when a local parent context from an existing `fastrace::Span` has been set using `Span::set_local_parent()`.
* **Arguments:**
    1.  `name: <string_literal>`: Custom span name. Default: full function path (e.g., `my_crate::my_module::my_function`).
    2.  `short_name: <bool>`: If `true`, uses function name without module path (e.g., `my_function`). Default: `false`.
    3.  `enter_on_poll: <bool>` (For `async fn` only): If `true`, span is entered on each poll of the async function's future. If `false` (default), the future executes within a single span context using `.in_span()`.
    4.  `properties: { <key_string_literal>: <value_format_string_literal>, ... }`: Adds custom key-value properties. Function arguments are accessible in value format strings. Default: `{}`.
* **Conceptual Expansion Snippets:**
    * Synchronous: `fn simple() { let __guard__ = LocalSpan::enter_with_local_parent("your_crate::simple"); /* ... logic ... */ }`
    * Asynchronous (default, `enter_on_poll = false`): `async fn simple_async() { let __span__ = Span::enter_with_local_parent("simple_async"); async { /* ... async logic ... */ }.in_span(__span__).await }`
    * Asynchronous (`enter_on_poll = true`): `async fn process_item() { async { /* ... async logic ... */ }.enter_on_poll("process_item_poll").await }`

---

## `fastrace` Crate Documentation Summary (Concise)

**Overview:**
`fastrace` is a high-performance, ergonomic, library-level timeline tracing library for Rust. It's designed for low overhead and is zero-overhead when disabled.

**Key Concepts:**

1.  **Span (`fastrace::Span`)**:
    * A thread-safe representation of a unit of work (name, timestamps, properties, parent reference).
    * **Creation**: `Span::root()` (new trace), `Span::enter_with_parent()` (child), `Span::enter_with_local_parent()` (child from thread-local parent). `Span::noop()` is a non-recording span.
    * **Local Context**: `span.set_local_parent()` sets the `Span` as the current thread's local parent, returning a `LocalParentGuard`. Essential for `LocalSpan` and `#[trace]`.
    * **Features**: Can add properties and events. `span.push_child_spans()` attaches collected `LocalSpan`s. Dropping a `Span` (especially root) submits it for reporting, unless `span.cancel()` is called on the root.

2.  **Local Span (`fastrace::local::LocalSpan`)**:
    * An optimized, non-thread-safe `Span` for single-thread use with lower overhead.
    * **Precondition**: Must be created within a local context established by `Span::set_local_parent()`.
    * **Creation**: `LocalSpan::enter_with_local_parent(name)`. This new `LocalSpan` becomes the new local parent.
    * **Features**: Can have properties/events. Dropping records its info and reverts local parent.

3.  **Event (`fastrace::Event`)**:
    * Represents a single point-in-time occurrence (like a log) with a name and properties.
    * **Creation**: `Event::new(name)`. Added via `Span::add_event(event)` or `LocalSpan::add_event(event)`.

4.  **Macros**:
    * `#[fastrace::trace]` (Attribute Macro from `fastrace_macro`): Auto-creates a `LocalSpan`. Args: `name`, `short_name`, `enter_on_poll` (async), `properties`. Requires local parent context.
    * Informational: `fastrace::func_name!()`, `fastrace::func_path!()`, `fastrace::file_location!()`.

5.  **Reporter (`fastrace::collector::Reporter`)**:
    * A trait for handling `SpanRecord`s (e.g., sending to Jaeger, Datadog, OpenTelemetry).
    * **Setup (Applications)**: `fastrace::set_reporter(reporter_impl, config)`. `fastrace::flush()` ensures reporting before exit.
    * **Implementation**: `fastrace::collector::ConsoleReporter` (prints to stderr) is provided.
    * **Configuration (`fastrace::collector::Config`)**: Options like report interval, max spans per trace.

6.  **Collector (`fastrace::collector`)**:
    * Module with types for span collection/reporting: `SpanRecord`, `EventRecord`, `TraceId (u128)`, `SpanId (u64)`, `SpanContext` (holds IDs and sampled flag, supports W3C Trace Context).

7.  **Local Collector (`fastrace::local::LocalCollector`)**:
    * Allows manual collection of `LocalSpan`s (via `collector.collect()`) without an initial local parent, to be attached to a parent `Span` later using `parent_span.push_child_spans(local_spans)`.

8.  **Futures (`fastrace::future`)**:
    * `FutureExt` trait extends `Future`:
        * `future.in_span(span)`: Binds a `Span` to a `Future`. Crucial for outermost futures.
        * `future.enter_on_poll(name)`: Starts a new `LocalSpan` on *every poll* of the future.

**Getting Started:**

* **In Libraries**: Add `fastrace = "0.7"` to `Cargo.toml`. Use `#[fastrace::trace]` or manually create `Span`s/`LocalSpan`s. If a library needs its own root trace: `let root = Span::root(...); let _guard = root.set_local_parent();`.
* **In Applications**: Add `fastrace = { version = "0.7", features = ["enable"] }`. Initialize a `Reporter`. Create root spans for tasks: `let root = Span::root(...); let _guard = root.set_local_parent();`. Call `fastrace::flush()` before exit.

**Performance Scenarios:**
    1.  No Tracing (`"enable"` off): Zero overhead.
    2.  Sample Tracing (`Span::noop()`): Minimal overhead.
    3.  Full Tracing with Tail Sampling (`Span::cancel()`): Low collection overhead.
    4.  Full Tracing (all reported): Significantly faster than many alternatives.

**Prelude (`fastrace::prelude::*`)**:
    Imports common types like `Span`, `LocalSpan`, `FutureExt`, `#[trace]`, etc.

**Important Notes:**
* **Local Parent Context**: `LocalSpan::enter_with_local_parent()` and `#[trace]` need a `Span` set as local parent via `span.set_local_parent()`. The returned guard must live for the required scope.
* **Async Tracing**: The outermost future of a task should use `.in_span(span)`.
* **Reporting**: Spans typically reported when root `Span` is dropped, configurable via `Config::report_before_root_finish(true)`.

## `fastrace` Integration with `log` Crate

`fastrace` can be integrated with the standard `log` crate in several ways:

1. **Via `tracing` Bridge (Official)**:
   * The `fastrace-tracing` crate provides `FastraceCompatLayer` which integrates with the `tracing` ecosystem
   * The `tracing-log` crate bridges `log` records to `tracing`
   * Setup example:
     ```rust
     // Set up fastrace reporter
     let reporter = fastrace::console::ConsoleReporter::new();
     fastrace::set_reporter(reporter, fastrace::Config::default());

     // Create a bridge between fastrace and tracing
     let fastrace_layer = fastrace_tracing::FastraceCompatLayer::new();
     let subscriber = tracing_subscriber::Registry::default()
         .with(fastrace_layer)
         .with(tracing_subscriber::fmt::Layer::default().with_writer(std::io::stderr));

     // Install the tracing subscriber
     tracing::subscriber::set_global_default(subscriber).unwrap();

     // Enable log -> tracing bridge
     tracing_log::LogTracer::init().unwrap();
     ```

2. **Direct Custom Integration**:
   * Create a custom `Log` implementation that sends log records directly to `fastrace`
   * Register it with `log::set_boxed_logger()`
   * Example implementation:
     ```rust
     struct FastraceLogger;

     impl log::Log for FastraceLogger {
         fn enabled(&self, metadata: &log::Metadata) -> bool {
             metadata.level() <= log::Level::Debug
         }

         fn log(&self, record: &log::Record) {
             if !self.enabled(record.metadata()) {
                 return;
             }

             // Create a new span for the log entry
             let span = match Span::get_local_parent() {
                 Some(parent) => Span::enter_with_local_parent("log", parent),
                 None => Span::root("log", SpanContext::random()),
             };

             // Add the log event with structured properties
             span.add_event(
                 Event::new("log_event").with_properties(move || {
                     vec![
                         ("level", std::borrow::Cow::from(record.level().to_string())),
                         ("message", std::borrow::Cow::from(format!("{}", record.args()))),
                         ("target", std::borrow::Cow::from(record.target().to_string())),
                         ("file", std::borrow::Cow::from(record.file().unwrap_or("unknown").to_string())),
                         ("line", std::borrow::Cow::from(record.line().unwrap_or(0).to_string())),
                     ]
                 })
             );
         }

         fn flush(&self) {
             fastrace::flush();
         }
     }

     // Initialize
     log::set_boxed_logger(Box::new(FastraceLogger))?;
     log::set_max_level(log::LevelFilter::Debug);
     ```

3. **Contextual Logging**:
   * The custom `Log` implementation can leverage the current `fastrace` context
   * By checking for an existing local parent span, log events can be nested within the current execution context
   * This provides a hierarchical view of logs within traces
   * Avoids creating new trace contexts for every log message

Key advantages of `fastrace` + `log` integration:
* Use familiar `log` macros throughout the codebase
* Capture structured metadata (file, line, target) with each log
* Benefit from hierarchical tracing context
* Enable/disable log levels with standard `log` filters
* Maintain backward compatibility with existing code

## Time Precision with `quanta` and `fastrace`

For high-performance tracing with nanosecond precision timestamps (crucial for HFT systems), `fastrace` works well with the `quanta` crate:

```rust
use fastrace::prelude::*;
use quanta::Clock;

fn main() {
    // Set up fastrace reporter
    let reporter = fastrace::console::ConsoleReporter::new();
    fastrace::set_reporter(reporter, fastrace::Config::default());

    // Create a high-precision clock
    let clock = Clock::new();

    // Create a root span with timestamp from quanta
    let timestamp_ns = clock.now().as_u64();
    let ctx = SpanContext::random();
    let root = Span::root_with_start_time("main", ctx, timestamp_ns);

    // Set up thread-local parent context
    let _guard = root.set_local_parent();

    // Record precise event timestamps
    let event_time = clock.now().as_u64();
    root.add_event(
        Event::new_with_timestamp("precise_event", event_time)
            .with_property(|| ("precision", std::borrow::Cow::from("nanosecond")))
    );

    // Automatically record end time on drop
    // Alternative: root.end_with_timestamp(clock.now().as_u64());
}
```

Advantages of `quanta` with `fastrace`:
* Cross-platform high-precision timestamps
* Minimal overhead for time measurements
* Consistent timing precision across the application
* More accurate span durations
* Better latency profiling for performance-critical paths