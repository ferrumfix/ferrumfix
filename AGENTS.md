# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

RustyFix (fork of FerrumFIX) is a Financial Information eXchange (FIX) protocol implementation in pure Rust. The project aims to provide a comprehensive, fast, and safe FIX protocol implementation with support for multiple encodings and versions.

## Architecture

The codebase is organized as a Rust workspace with multiple crates that follow OSI model layers:

### Core Crates

1. **rustyfix** - Main crate providing FIX message encoding/decoding
   - Tag-value encoding/decoding (`tagvalue` module)
   - JSON encoding support (`json` module)
   - Session layer implementation (`session` module)
   - Field type implementations for FIX data types
   - Buffer abstractions for zero-copy operations

2. **rustyfix-dictionary** - FIX Dictionary management
   - Handles FIX protocol specifications
   - Provides field, message, and component definitions
   - Supports QuickFIX XML spec parsing
   - Version-specific FIX definitions (4.0 through 5.0 SP2, FIXT.1.1)

3. **rustyfix-codegen** - Code generation for FIX definitions
   - Generates Rust code from FIX dictionaries
   - Creates type-safe field definitions
   - Produces enum types for FIX field values

4. **rustyfix-derive** - Derive macros for FIX types
   - Provides `#[derive(FieldType)]` for custom field types

5. **rustyfast** - FIX Adapted for STreaming (FAST) protocol support
   - FAST encoding/decoding implementation
   - Template-based message compression
   - Field operators (constant, delta, copy)

6. **rustyfixp** - FIX Performance Session Layer (FIXP)
   - Support for different flow types
   - Session negotiation and establishment

7. **rustyfixs** - FIX-over-TLS (FIXS) support
   - TLS configuration presets
   - IANA to OpenSSL cipher suite conversion

8. **rustysofh** - Simple Open Framing Header (SOFH)
   - Message framing for FIX over TCP
   - Tokio codec implementation for async I/O

## Build and Development Commands

```bash
# Install development tools
just install-tools

# Run linting
just lint

# Run all tests
just test

# Check all feature combinations
just check-features

# Find unused dependencies
just udeps

# Format code (requires nightly)
cargo +nightly fmt

# Build with all features
cargo build --all-features

# Run specific test
cargo nextest run -p rustyfix test_name

# Generate documentation
RUSTDOCFLAGS="--cfg doc_cfg" cargo +nightly doc --all-features --open
```

## Key Concepts and Patterns

### Zero-Copy Design
The codebase emphasizes zero-copy operations wherever possible:
- `Buffer` trait abstracts over `Vec<u8>` and `bytes::BytesMut`
- Messages hold references to underlying byte buffers
- Field access returns byte slices that can be deserialized on-demand

### Type Safety with Code Generation
- Use `rustyfix-codegen` in build scripts to generate type-safe field definitions
- Generated code provides compile-time guarantees for field tags and types
- See examples in `examples/05_coinbase_codegen` and `examples/20_tokio_tradeclient`

### Flexible Encoding Support
- Tag-value (classic FIX) encoding in `rustyfix::tagvalue`
- JSON encoding in `rustyfix::json`
- FAST encoding in the `rustyfast` crate
- SOFH framing in the `rustysofh` crate

### Field Types System
- `FieldType` trait provides serialization/deserialization
- Built-in support for FIX data types (dates, times, decimals, etc.)
- Integration with `chrono`, `rust_decimal`, and `decimal` crates
- Custom field types can be derived with `#[derive(FieldType)]`

### Async/Stream Support
- Streaming decoders for processing message streams
- Tokio codec implementations for async I/O
- Event loop abstractions for session management

## Migration from FerrumFIX to RustyFix

When forking to RustyFix, ensure all references are updated:
- Update crate names from `fefix*` to `rustyfix*`
- Update repository URLs and documentation links
- Maintain compatibility with existing FIX specifications

## Testing Strategy

1. **Unit Tests**: Each module has focused unit tests
2. **Integration Tests**: Cross-crate functionality testing
3. **Property Testing**: QuickCheck tests for encoders/decoders
4. **Example Programs**: Demonstrate real-world usage patterns

## Common Tasks

### Adding a New FIX Version
1. Add the specification XML to `rustyfix-dictionary/resources/`
2. Update `Dictionary` with a new constructor method
3. Add corresponding module in `rustyfix/src/definitions/`
4. Update feature flags in `Cargo.toml`

### Implementing a New Encoding
1. Create a new module under `rustyfix/src/`
2. Implement `Decoder` and `Encoder` types
3. Add configuration options following existing patterns
4. Provide streaming variants if applicable

### Creating Custom Field Types
1. Implement the `FieldType` trait
2. Define serialization format
3. Implement both strict and lossy deserialization
4. Add comprehensive tests including round-trip testing

## Performance Considerations

- Prefer stack allocation for small types
- Use `SmartString` for short strings to avoid heap allocation
- Implement lossy deserialization variants for hot paths
- Batch operations when processing message streams
- Profile with `cargo flamegraph` for bottleneck identification

## Security Notes

- Always validate message checksums in production
- Set appropriate message size limits
- Sanitize string fields to prevent injection attacks
- Use TLS for network communication (see `rustyfixs` crate)
- Implement proper session sequence number validation

## Rust 2024 Features to Leverage

- Use workspace inheritance for common dependencies
- Leverage improved async trait support
- Utilize pattern matching improvements
- Take advantage of const generics where applicable
- Use new formatting and diagnostic attributes

### Let chains: `let` chains in `if` and `while`

[Summary]

-   Allow chaining of `let` expressions in the condition operand of `if` and `while`.

[Details]

Starting with the 2024 Edition, it is now allowed to have chaining of `let` expressions inside `if` and `while` condition operands, where chaining refers to `&&` chains. The `let` expressions still have to appear at the top level, so `if (let Some(hi) = foo || let Some(hi) = bar)` is not allowed.

Before 2024, the `let` had to appear directly after the `if` or `while`, forming a `if let` or `while let` special variant. Now, `if` and `while` allow chains of one or more `let` expressions, possibly mixed with expressions that are `bool` typed.

```rust
fn sum_first_two(nums: &[u8]) -> Option<u8> {
    let mut iter = nums.iter();
    if let Some(first) = iter.next()
        && let Some(second) = iter.next()
    {
        first.checked_add(*second)
    } else {
        None
    }
}
```

### RPIT Lifetime Capture Rules

[Summary]

- Return Position Impl Trait (RPIT) now unconditionally captures all in-scope generic lifetime parameters
- New `use<..>` bounds allow explicit control over which parameters to capture

[Details]

In Rust 2021 and earlier, generic lifetime parameters were only captured when they appeared syntactically within a bound. Starting in Rust 2024, these parameters are unconditionally captured:

```rust
// Rust 2024: explicitly specify captured lifetimes
fn capture<'a, T>(x: &'a (), y: T) -> impl Sized + use<'a, T> {
    (x, y)
}
```

The `impl_trait_overcaptures` lint helps with migration by flagging RPIT types that will capture additional lifetimes. This lint is part of the `rust-2024-compatibility` group and can often automatically insert `use<..>` bounds.

### Unsafe Attributes

[Summary]

- Certain attributes that can cause undefined behavior now require `unsafe`
- Affected attributes: `no_mangle`, `export_name`, `link_section`

[Details]

Attributes that can undermine Rust's safety guarantees must now be marked as unsafe:

```rust
// SAFETY: There should only be a single definition of the loop symbol
#[unsafe(export_name="loop")]
fn arduino_loop() {
    // ...
}

// SAFETY: The name must be a valid C identifier
#[unsafe(no_mangle)]
pub extern "C" fn calculate(x: i32) -> i32 {
    x * 2
}
```

The `unsafe_attr_outside_unsafe` lint automatically migrates these during edition updates.

### Static Mutable References

[Summary]

- Direct references to `static mut` are now disallowed
- Must use pointer APIs or synchronization primitives

[Details]

Accessing `static mut` directly is no longer allowed. Use proper synchronization:

```rust
// Before (Rust 2021)
static mut COUNTER: u32 = 0;
unsafe { COUNTER += 1; }

// After (Rust 2024)
use std::sync::Mutex;
static COUNTER: Mutex<u32> = Mutex::new(0);
*COUNTER.lock().unwrap() += 1;
```

### Unsafe Extern Blocks

[Summary]

- `extern` blocks must now be marked `unsafe`
- Clarifies that FFI boundaries are inherently unsafe

[Details]

```rust
// Rust 2024 requires unsafe extern
unsafe extern "C" {
    fn external_function(x: i32) -> i32;
}
```

### Public/Private Dependencies (Cargo)

[Summary]

- Cargo now distinguishes between public and private dependencies
- Prevents accidental exposure of internal dependencies in public APIs

[Details]

In `Cargo.toml`:

```toml
[dependencies]
# Public dependency - types from this crate can appear in our public API
serde = { version = "1", features = ["derive"], public = true }

# Private dependency - only used internally
regex = "1"  # private by default
```

The `exported_private_dependencies` lint warns when private dependencies leak into public APIs.

### Prelude Changes

[Summary]

- `Future` and `IntoFuture` are now in the prelude
- Reduces boilerplate for async code

[Details]

```rust
// No longer need to import these explicitly
// use std::future::Future;
// use std::future::IntoFuture;

async fn example() -> impl Future<Output = u32> {
    async { 42 }
}
```
   ```

## Robust Coding Practices

| ID  | Guideline                                                                                                                                                                                        |
| --- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1   | **Stay Current** – Align with latest libraries, standards, avoid deprecated APIs                                                                                                                 |
| 2   | **Complete Implementation** – Never skip, postpone, or omit required details                                                                                                                     |
| 3   | **Style & Documentation** – Comment every public function; avoid global variables                                                                                                                |
| 4   | **Performance & Safety** – Ensure low-latency, memory-/thread-safety, and cache-awareness                                                                                                        |
| 5   | **Concurrency** – Provide `synchronization` & `memory layout` analysis of concurrent codebases, including lock ordering                                                                          |
| 6   | **Memory Visualization** – Draw memory layout/state diagrams when using `unsafe`                                                                                                                 |
| 7   | **Honest Limitations** – Acknowledge when a solution is infeasible or unknown                                                                                                                    |
| 8   | **Avoid Over-Engineering** – Keep solutions simple and straightforward                                                                                                                           |
| 9   | **Error Handling** – Use `Result` and `Option` types; avoid panics in production code                                                                                                            |
| 10  | **Testing** – Write unit tests for all public functions; use `#[cfg(test)]` for test code                                                                                                        |
| 11  | **Documentation** – Use `///` for public functions; `//!` for module-level documentation                                                                                                         |
| 12  | **Human Readability** – Use clear and descriptive variable/function names; avoid cryptic abbreviations. Also abstraction components' names should be easily understandable and self-explanatory. |

## Naming Conventions & Guidelines

### Core Principles

1. **Clarity over Brevity** - Use full words instead of abbreviations
2. **Consistency across Modules** - Same concept should use same name everywhere
3. **Context-Aware Naming** - Names should indicate their module/purpose
4. **Self-Documenting** - Names should explain what they do without needing comments

### Standard Naming Patterns

#### Variables & Fields

- **Timestamps**: Use explicit naming
  - `timestamp` - General purpose timestamp
  - `exchange_timestamp` - Time from exchange (nanoseconds)
  - `system_timestamp` - Local system time (nanoseconds)
  - `created_at`, `updated_at` - For records/persistence
  - ❌ Avoid: `ts`, `time`, `t`, `exch_ts`

- **Quantities & Amounts**:
  - `quantity` - Order/trade quantity
  - `volume` - Market volume
  - `amount` - Monetary amount
  - ❌ Avoid: `qty`, `vol`, `amt`

- **Identifiers**:
  - `order_id`, `trade_id`, `user_id` - Use full `_id` suffix
  - Consider newtype pattern: `struct OrderId(u64);`
  - ❌ Avoid: `oid`, `tid`, `uid`

#### Method Names

- **Data Access**:
  - `get_*` - Synchronous accessor, returns owned or reference
  - `fetch_*` - Asynchronous operation, usually external data
  - `load_*` - Load from storage/file
  - `find_*` - Search operation that may return None
  - ❌ Avoid: Mixing these patterns inconsistently

- **State Mutations**:
  - `set_*` - Direct setter
  - `update_*` - Partial update with logic
  - `apply_*` - Apply a change/event
  - `process_*` - Complex processing logic

- **Creation & Conversion**:
  - `new` - Simple constructor
  - `create_*` - Factory method with logic
  - `from_*` - Conversion from specific type
  - `to_*` - Conversion to specific type
  - `into_*` - Consuming conversion

#### Type Names

- **Module-Specific Types**: When same concept exists in multiple modules
  - `model::Order` → Could be `ModelOrder` or keep in module namespace
  - `oms::OrderRequest` → Could be `OmsOrderRequest`
  - Use module prefix when types are used across modules

- **Event Types**:
  - `OrderEvent`, `TradeEvent`, `MarketEvent` - Clear event types
  - Include source/context: `ExchangeOrderEvent`, `StrategySignalEvent`

- **Generic Names to Avoid**:
  - ❌ `Manager` - Too generic, use `OrderManager`, `RiskManager`
  - ❌ `Handler` - Specify what: `OrderHandler`, `MessageHandler`
  - ❌ `Service` - Be specific: `ExecutionService`, `DataService`
  - ❌ `Helper` - Refactor into proper abstraction

### Common Abbreviations to Expand

| Abbreviation | Full Name        |
| ------------ | ---------------- |
| `auth`       | `authentication` |
| `sig`        | `signature`      |
| `req`        | `request`        |
| `res`/`resp` | `response`       |
| `msg`        | `message`        |
| `cfg`/`conf` | `configuration`  |
| `ctx`        | `context`        |
| `conn`       | `connection`     |
| `mgr`        | `manager`        |
| `qty`        | `quantity`       |
| `ts`         | `timestamp`      |
| `ws`         | `websocket`      |
| `addr`       | `address`        |
| `recv`       | `receive`        |
| `init`       | `initialize`     |

### Acceptable Abbreviations

These are widely understood and may be kept:

- `id` - identifier
- `api` - application programming interface
- `url` - uniform resource locator
- `json` - JavaScript Object Notation
- `http`/`https` - hypertext transfer protocol
- `io` - input/output
- `tx`/`rx` - transmit/receive (in channel context)
- `db` - database
- `ui` - user interface
- `sdk` - software development kit

### Error Types

- Each module should have its own error type
- Use descriptive variant names: `OrderNotFound`, not `NotFound`
- Include context in error messages

### Module Organization

- Place related types together
- Use submodules for logical grouping
- Re-export commonly used types at module root
- Keep public API surface minimal

## Cargo.toml Handling

### Workspace Dependencies

**IMPORTANT: Always use workspace versions when available in the global Cargo.toml**

- When adding dependencies to any crate, check `[workspace.dependencies]` in the root `Cargo.toml`
- If a dependency is defined at the workspace level, use: `dependency = { workspace = true }`
- This ensures version consistency across all crates in the workspace
- Example:
  ```toml
  # In individual crate's Cargo.toml
  [dependencies]
  tokio = { workspace = true }
  quanta = { workspace = true }
  serde = { workspace = true }
  simd-json = { workspace = true }
  ```

## Enhanced Performance Guidelines

Append the following to the existing "Performance Considerations" section:

- **Mandatory Performance Libraries**

**ALWAYS use these high-performance alternatives:**

- `smallvec`/`arrayvec` – Stack-allocated dynamic arrays (**MANDATORY**: Always use SmallVec instead of Vec for collections < 32 items)
- `smartstring` instead of `String` (for short text data)
- `FxHashMap`/`FxHashSet` from `rustc-hash` instead of std `HashMap`/`HashSet`
  - Also consider using `dashmap` for concurrent access
- `simd-json` instead of `serde_json` (for all JSON operations)
- `parking_lot::Mutex` instead of `std::sync::Mutex`
- `tachyonix` channels instead of `tokio::sync::mpsc`
- `simd_aligned` for SIMD usage
- `zerocopy` for zero-copy manipulations
- `quanta` for high-resolution timekeeping (instead of `chrono`)
- `smallbytes` instead of `bytes` crate from tokio team
- `fastrace` instead of `tracing` if logging is needed

* Look for @docs/external-libraries/ for crates' information

### Core Performance Mandates

1. **Latency & Predictability First**
  - Microsecond or sub-microsecond latency
  - Nanosecond precision for all timestamps
  - Avoid unpredictable constructs (I/O, syscalls, complex locking)

2. **Zero Heap Allocation in Critical Paths**
  - Avoid `Box`, `Vec`, `String`, `HashMap` with default allocators
  - Use stack allocation or specialized allocators
  - **MANDATORY**: Always use `SmallVec` instead of `Vec` for collections < 32 items
  - **MANDATORY**: Always use `smartstring` instead of `String` for short text data
  - **MANDATORY**: Always use `FxHashMap`/`FxHashSet` from `rustc-hash` instead of std `HashMap`/`HashSet`
  - **MANDATORY**: Always use `simd-json` instead of `serde_json` for JSON parsing

3. **Data Locality**
  - Optimize for L1/L2 cache residency
  - Use cache-line alignment
  - Prevent false sharing via padding

4. **Concurrency Efficiency**
  - Prefer lock-free algorithms
  - Use `parking_lot` for necessary locks
  - Minimize critical sections
  - Use minimal atomic ordering

5. **Memory Management**
  - Prioritize stack allocation
  - Use `#[repr(align(N))]` for cache alignment
  - Avoid `Rc`, `Arc` in performance-critical paths

6. **Use fast hashing functions**
  - Cryptographic safety is not needed
  - Use faster hashing algorithms focused on performance without security
  - Avoid hash collisions in critical paths
  - Use custom performant hash functions for specific data types

7. **Vectorization Strategy**
  - **PRIORITY 1: Safe SIMD with `simd_aligned` + `wide`**
  - **PRIORITY 2: Compiler Auto-Vectorization**
  - **PRIORITY 3: Custom x86-64 SIMD (only as last resort)**

8. **Using `unsafe`**
  - Document all `unsafe` blocks with safety invariants
  - Ensure `unsafe` code is well-tested and reviewed
  - Use `unsafe` for performance-critical paths only
  - Aggressively optimize for performance
  - Use `unsafe` for low-level memory operations

9. **Panic-Free Code Patterns**
  - **Avoid `unwrap()`**: Use `?` operator or explicit error handling
  - **Replace with `expect()`**: When panic is acceptable, use descriptive messages
  - **Test Edge Cases**: All unsafe operations must have comprehensive tests

10. **Use `quanta` for high-resolution timekeeping**
  - Use `quanta` for high-resolution timekeeping
  - Avoid using `std::time` for performance-critical paths
  - Never use `chrono`, it's not suitable for high-performance applications
  - Use `quanta::Instant` for nanosecond precision

## Development Workflow

### Code Checks

```bash
# Basic checks
cargo check --all-targets --all-features --workspace --tests

# Run clippy linter with strict warnings
cargo clippy --all-targets --all-features --workspace --tests

# Format code
cargo fmt --all
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_subscribe_spot_trades -- --nocapture
```

### Documentation Standards

- Document performance optimization rationale
- Explain concurrency logic and memory ordering
- Justify `unsafe` blocks with safety invariants
- Document memory management in complex code

## Rust-Specific Guidelines

### Best Practices

- Use borrowing and lifetimes idiomatically
- Minimize dynamic dispatch and heap allocations
- Implement zero-copy operations
- Avoid unnecessary `clone()` calls
- Run `cargo check --all-targets --all-features --workspace --tests` and
  `cargo clippy --all-targets --all-features --workspace --tests` regularly
- Use `unsafe` only when absolutely necessary, comment the safety rationale of unsafe block with full documentation
- Aggressively optimize for performance

### Design Considerations

Before implementing significant changes:

- Analyze cause-effect relationships
- Map data flow and transformations
- Consider edge cases and error conditions
- Evaluate performance implications
- Document specific trade-offs
