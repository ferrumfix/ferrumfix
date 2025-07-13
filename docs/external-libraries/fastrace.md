# fastrace & fastrace_macro Documentation

**fastrace Version:** 0.7.14
**fastrace_macro Version:** 0.7.14

## Overview

`fastrace` is a high-performance, library-level timeline tracing library for Rust designed for minimal overhead and excellent performance. It provides zero overhead when tracing is disabled and is significantly faster than other tracing solutions when enabled.

## Table of Contents

1. [Core Concepts](#core-concepts)
2. [Installation](#installation)
3. [API Reference](#api-reference)
4. [Usage Patterns](#usage-patterns)
5. [Performance Characteristics](#performance-characteristics)
6. [Integration Guide](#integration-guide)
7. [Advanced Features](#advanced-features)

## Core Concepts

### 1. Span (`fastrace::Span`)

A thread-safe representation of a unit of work with:
- Name identifier
- Start/end timestamps
- Key-value properties
- Parent-child relationships
- Event tracking

**Key Methods:**
- `Span::root(name, context)` - Create a new trace root
- `Span::noop()` - Create a non-recording placeholder span
- `Span::enter_with_parent(name, parent)` - Create child span with explicit parent
- `Span::enter_with_local_parent(name)` - Create child using thread-local parent
- `span.set_local_parent()` - Set as thread-local parent (returns `LocalParentGuard`)
- `span.add_property(|| (key, value))` - Add key-value metadata
- `span.add_event(event)` - Add timestamped event
- `span.cancel()` - Prevent entire trace from being reported
- `span.push_child_spans(local_spans)` - Attach collected LocalSpans

### 2. LocalSpan (`fastrace::local::LocalSpan`)

A non-thread-safe, optimized span for single-thread execution with lower overhead.

**Requirements:**
- Must be created within a local context established by `Span::set_local_parent()`
- Automatically becomes the new local parent when created

**Key Methods:**
- `LocalSpan::enter_with_local_parent(name)` - Create with local parent
- `local_span.add_property(|| (key, value))` - Add metadata
- `local_span.add_event(event)` - Add event

### 3. Event (`fastrace::Event`)

Represents a point-in-time occurrence with name and properties.

**Creation:**
```rust
Event::new("event_name")
    .with_property(|| ("key", "value"))
```

### 4. SpanContext, SpanId, TraceId

- `SpanContext` - Contains trace ID, span ID, and sampling flag
- `SpanId` - 64-bit unique span identifier
- `TraceId` - 128-bit trace identifier grouping related spans

## Installation

### For Libraries
```toml
[dependencies]
fastrace = "0.7"
```

### For Applications
```toml
[dependencies]
fastrace = { version = "0.7", features = ["enable"] }
```

## API Reference

### Modules

#### `fastrace::collector`
Span collection and reporting infrastructure.

**Types:**
- `Config` - Global collector configuration
- `ConsoleReporter` - Built-in stderr reporter
- `SpanRecord` - Complete span information
- `EventRecord` - Event occurrence record
- `Reporter` trait - Custom reporter interface

**Functions:**
- `fastrace::set_reporter(reporter, config)` - Initialize reporting
- `fastrace::flush()` - Force pending spans to report

#### `fastrace::future`
Async tracing support.

**Trait: `FutureExt`**
- `future.in_span(span)` - Bind span to future (crucial for outermost futures)
- `future.enter_on_poll(name)` - Create LocalSpan on each poll

#### `fastrace::local`
Thread-local span optimization.

**Types:**
- `LocalCollector` - Manual LocalSpan collection
- `LocalParentGuard` - RAII guard for local parent
- `LocalSpans` - Collection of LocalSpan instances

#### `fastrace::prelude`
Common imports:
- `Span`, `LocalSpan`, `Event`
- `SpanContext`, `SpanId`, `TraceId`
- `FutureExt` trait
- All macros

### Macros

#### `#[fastrace::trace]` (from fastrace_macro)
Attribute macro for automatic span instrumentation.

**Parameters:**
- `name: "custom_name"` - Override default span name
- `short_name: true` - Use function name only (default: false)
- `enter_on_poll: true` - For async: enter span on each poll (default: false)
- `properties: { "key": "value", "arg": "arg is {arg}" }` - Add properties

**Requirements:**
- Requires local parent context via `Span::set_local_parent()`

**Example:**
```rust
#[fastrace::trace(properties = { "user_id": "{user_id}" })]
async fn process_user_request(user_id: u64) {
    // Automatically traced
}
```

#### Utility Macros
- `fastrace::func_name!()` - Current function name
- `fastrace::func_path!()` - Full function path
- `fastrace::file_location!()` - File:line location

### Functions

- `fastrace::set_reporter(reporter, config)` - Initialize global reporter
- `fastrace::flush()` - Flush pending spans to reporter

## Usage Patterns

### Basic Synchronous Tracing

```rust
use fastrace::prelude::*;

fn main() {
    // Initialize reporter (application only)
    fastrace::set_reporter(
        ConsoleReporter::new(),
        Config::default()
    );

    // Create root span
    let root = Span::root("main", SpanContext::random());
    let _guard = root.set_local_parent();

    // Traced work
    process_data();

    // Flush before exit
    fastrace::flush();
}

#[fastrace::trace]
fn process_data() {
    // Automatically traced
    let event = Event::new("data_processed")
        .with_property(|| ("count", "100"));

    LocalSpan::current().add_event(event);
}
```

### Async Tracing

```rust
use fastrace::prelude::*;

async fn async_operation() {
    let root = Span::root("async_op", SpanContext::random());

    let task = async {
        let _guard = root.set_local_parent();

        // Inner async work
        process_async().await;
    }
    .in_span(root); // Critical: outermost future must use in_span

    task.await;
}

#[fastrace::trace(enter_on_poll = true)]
async fn process_async() {
    // New LocalSpan created on each poll
    tokio::time::sleep(Duration::from_millis(100)).await;
}
```

### Manual Span Management

```rust
use fastrace::prelude::*;

fn manual_tracing() {
    let parent = Span::root("parent", SpanContext::random());

    // Explicit child
    let child = Span::enter_with_parent("child", &parent);
    child.add_property(|| ("key", "value"));

    // Thread-local child
    let _guard = parent.set_local_parent();
    let local = LocalSpan::enter_with_local_parent("local_child");
    local.add_event(Event::new("milestone"));
}
```

### LocalSpan Collection

```rust
use fastrace::prelude::*;
use fastrace::local::LocalCollector;

fn collect_spans() {
    let mut collector = LocalCollector::new();
    let local_spans = collector.collect(|| {
        let span = LocalSpan::enter_with_local_parent("collected");
        // Work...
    });

    // Attach to parent later
    let parent = Span::root("parent", SpanContext::random());
    parent.push_child_spans(local_spans);
}
```

## Performance Characteristics

### Overhead Scenarios

1. **Tracing Disabled** (no "enable" feature): Zero overhead
2. **Sample Tracing** (`Span::noop()`): Minimal overhead (~1ns)
3. **Tail Sampling** (`span.cancel()`): Low collection overhead
4. **Full Tracing**: 10-100x faster than alternatives

### Performance Tips

- Use `LocalSpan` for single-threaded paths
- Leverage `#[trace]` macro to reduce boilerplate
- Use `span.cancel()` for selective trace filtering
- Properties are lazily evaluated - use closures

## Integration Guide

### With log Crate

#### Via tracing Bridge
```rust
// Setup fastrace
let reporter = fastrace::console::ConsoleReporter::new();
fastrace::set_reporter(reporter, fastrace::Config::default());

// Bridge through tracing
let fastrace_layer = fastrace_tracing::FastraceCompatLayer::new();
let subscriber = tracing_subscriber::Registry::default()
    .with(fastrace_layer);

tracing::subscriber::set_global_default(subscriber).unwrap();
tracing_log::LogTracer::init().unwrap();
```

#### Direct Custom Integration
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

        // Create span for log entry
        let span = match LocalSpan::current() {
            Some(_) => LocalSpan::enter_with_local_parent("log"),
            None => return, // No tracing context
        };

        span.add_event(
            Event::new("log")
                .with_property(|| ("level", record.level().to_string()))
                .with_property(|| ("message", format!("{}", record.args())))
        );
    }

    fn flush(&self) {
        fastrace::flush();
    }
}
```

### With quanta for High-Precision Timestamps

```rust
use fastrace::prelude::*;
use quanta::Clock;

fn precise_tracing() {
    let clock = Clock::new();

    // Root span with precise start time
    let start_ns = clock.now().as_u64();
    let root = Span::root_with_start_time(
        "precise_operation",
        SpanContext::random(),
        start_ns
    );

    // Precise event timing
    let event_time = clock.now().as_u64();
    root.add_event(
        Event::new_with_timestamp("checkpoint", event_time)
    );

    // Optional: explicit end time
    let end_ns = clock.now().as_u64();
    root.end_with_timestamp(end_ns);
}
```

## Advanced Features

### Custom Reporter Implementation

```rust
use fastrace::collector::{Reporter, SpanRecord};

struct CustomReporter;

impl Reporter for CustomReporter {
    fn report(&mut self, spans: Vec<SpanRecord>) {
        for span in spans {
            // Send to monitoring service
            send_to_jaeger(&span);
            send_to_datadog(&span);
        }
    }
}
```

### Distributed Tracing

```rust
use fastrace::collector::SpanContext;

// Extract context from incoming request
fn extract_context(headers: &HeaderMap) -> SpanContext {
    // Parse W3C Trace Context headers
    let trace_id = parse_trace_id(headers.get("traceparent"));
    let span_id = generate_span_id();

    SpanContext::new(trace_id, span_id, 0, 1) // sampled
}

// Inject context for outgoing request
fn inject_context(span: &Span, headers: &mut HeaderMap) {
    let ctx = span.context();
    headers.insert(
        "traceparent",
        format!("00-{:032x}-{:016x}-01", ctx.trace_id, ctx.span_id)
    );
}
```

### Configuration Options

```rust
use fastrace::collector::Config;

let config = Config::default()
    .report_interval(Duration::from_secs(1))
    .max_spans_per_trace(Some(1000))
    .report_before_root_finish(true);
```

## Best Practices

1. **Always use `in_span()` on outermost futures** - Otherwise traces are lost
2. **Set local parent before using `#[trace]` or `LocalSpan`** - Required for context
3. **Use properties closures** - Lazy evaluation for performance
4. **Call `flush()` before exit** - Ensures all spans are reported
5. **Use `LocalSpan` for hot paths** - Lower overhead than `Span`
6. **Leverage tail sampling** - Use `cancel()` to filter traces
7. **Batch span reporting** - Configure appropriate report intervals

## Common Pitfalls

1. **Forgetting `in_span()` on async tasks** - Traces disappear
2. **No local parent for `#[trace]`** - Macro will panic
3. **Not calling `flush()`** - Lost traces on exit
4. **Using `Span` in tight loops** - Use `LocalSpan` instead
5. **Blocking in property closures** - Degrades performance

## Performance Benchmarks

Typical overhead per span:
- `Span::noop()`: ~1ns
- `LocalSpan`: ~20ns
- `Span` with properties: ~50-100ns
- Full featured span: ~200ns

Compared to alternatives:
- 10-100x faster than tracing
- 50x faster than opentelemetry
- Near-zero overhead when disabled

## Changelog Highlights

### 0.7.x
- Stable API with production readiness
- Enhanced async support
- Improved reporter interface
- Better distributed tracing support

---

This documentation covers the essential aspects of fastrace for high-performance tracing in Rust applications. For the latest updates and additional examples, refer to the official repository and documentation.