# GitHub Copilot Instructions

This file provides guidance to GitHub Copilot when assisting with code generation, reviews, and other tasks in this repository. It is based on the guidelines in CLAUDE.md.

## Robust Coding Practices

| ID  | Guideline |
| --- | --- |
| 1   | **Stay Current** – Align with latest libraries, standards, avoid deprecated APIs |
| 2   | **Complete Implementation** – Never skip, postpone, or omit required details |
| 3   | **Style & Documentation** – Comment every public function; avoid global variables |
| 4   | **Performance & Safety** – Ensure low-latency, memory-/thread-safety, and cache-awareness |
| 5   | **Concurrency** – Provide `synchronization` & `memory layout` analysis of concurrent codebases, including lock ordering |
| 6   | **Memory Visualization** – Draw memory layout/state diagrams when using `unsafe` |
| 7   | **Honest Limitations** – Acknowledge when a solution is infeasible or unknown |
| 8   | **Avoid Over-Engineering** – Keep solutions simple and straightforward |
| 9   | **Error Handling** – Use `Result` and `Option` types; avoid panics in production code |
| 10  | **Testing** – Write unit tests for all public functions; use `#[cfg(test)]` for test code |
| 11  | **Documentation** – Use `///` for public functions; `//!` for module-level documentation |
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

## Enhanced Performance Guidelines

- **Mandatory Performance Libraries**

**ALWAYS use these high-performance alternatives:**

- `SmallVec` instead of `Vec` (for collections < 32 items)
- `smartstring` instead of `String` (for short text data)
- `FxHashMap`/`FxHashSet` from `rustc-hash` instead of std `HashMap`/`HashSet`
  - Also consider using `dashmap` for concurrent access
- `simd-json` instead of `serde_json` (for all JSON operations)
- `parking_lot::Mutex` instead of `std::sync::Mutex`
- `flume` channels instead of `std::sync::mpsc`
- `simd_aligned` for SIMD usage
- `zerocopy` for zero-copy manipulations
- `yawc` for WebSocket connections (instead of `tungstenite` or `tokio-tungstenite`)
- `quanta` for high-resolution timekeeping (instead of `chrono`)

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