# Crate `quanta` (version 0.12.6)

Performant cross-platform timing with goodies. `quanta` is a high-speed timing library, useful for getting the current time _very quickly_, as well as manipulating it.

## Overview

`quanta` provides a simple and fast API for measuring the current time and the duration between events. It uses native OS timing functions or the Time Stamp Counter (TSC) on modern CPUs to achieve low-overhead, high-precision time measurements.

**Key Design Features:**
- Measures CPU cycles via Time Stamp Counter (TSC) when available
- Gets monotonic time in nanoseconds
- Low overhead with performance comparable to or better than standard library timing
- Safe wrapper for accessing Time Stamp Counter
- Ability to pause or discretely update time for testing

## Key Concepts

### Clock Sources
* **Reference Clock:** OS-provided, always available. Uses platform-specific high-resolution timing APIs.
* **Source Clock:** TSC-based when present and reliable; otherwise, falls back to the reference clock.
* **Calibration:** The source clock is calibrated to the reference clock to provide wall clock time. This happens on the first `Clock` creation and can take up to 200ms (typically 10-20ms).

### Main Features

#### 1. Mocked Time
- Allows substituting `Clock` for testing
- Enables simulating time passage and warping backwards/forwards
- Access via `Clock::mock()` which returns `(Clock, Arc<Mock>)`

#### 2. Coarsely-Updated "Recent" Time
- Ultra-low-overhead access to a slightly-delayed global time
- Can be 4-10x faster than `Clock::now()` (typically 4-5ns vs 12-14ns)
- Requires an "upkeep" thread (managed by `Upkeep`) or manual updates via `set_recent()`
- Access via `Clock::recent()` or `Instant::recent()`

### Feature Flags
* `prost`: Provides conversion to `prost_types::Timestamp` for protobuf integration

## Platform Support

### Primary Platforms
* **Windows**: Uses `QueryPerformanceCounter`
* **macOS/iOS**: Uses `mach_absolute_time`
* **Linux/*BSD/Solaris**: Uses `clock_gettime`

### WASM Support
* `wasm32-unknown-unknown` (browsers): Uses `window.performance.now` (millisecond accuracy)
* `wasm32-wasi`: Accuracy depends on VM implementation

### TSC Support
* Requires `x86_64` with SSE2, and constant or nonstop/invariant TSC
* Nonstop/invariant TSC is preferred for stability across power states
* Supported on:
  - Intel Nehalem/Skylake+ processors
  - AMD Phenom+ processors
* `quanta` queries CPUID to determine TSC usability

## API Reference

### Structs

#### `struct Clock`
Unified clock for taking measurements. The primary interface for time measurement.

**Construction:**
```rust
// Creates a new clock with optimal reference and source clocks
let clock = Clock::new();

// Creates a mocked clock for testing
let (clock, mock_handle) = Clock::mock();
```

**Key Methods:**
* `pub fn new() -> Clock`
  - Creates a new clock with optimal reference and source clocks
  - Checks TSC support at creation time
  - First creation triggers calibration (up to 200ms)

* `pub fn mock() -> (Clock, Arc<Mock>)`
  - Creates a new mocked clock and a handle to control its time
  - Useful for testing time-dependent code

* `pub fn now(&self) -> Instant`
  - Gets the current time, scaled to reference time
  - Monotonically increasing for the same `Clock` instance
  - Returns nanosecond-precision `Instant`

* `pub fn raw(&self) -> u64`
  - Gets underlying time from the fastest source
  - Not guaranteed to be nanoseconds or monotonic
  - Must be scaled for meaningful use

* `pub fn scaled(&self, value: u64) -> Instant`
  - Scales a raw measurement to reference time (nanoseconds)
  - Converts raw clock cycles to wall clock time

* `pub fn delta_as_nanos(&self, start: u64, end: u64) -> u64`
  - Calculates delta between two raw measurements in nanoseconds
  - Lower overhead than `delta` method

* `pub fn delta(&self, start: u64, end: u64) -> Duration`
  - Calculates delta between two raw measurements
  - Returns standard `Duration` type

* `pub fn recent(&self) -> Instant`
  - Gets the most recent (slightly-delayed) global time
  - Requires upkeep thread or `set_recent()` calls
  - Returns `Instant(0)` if upkeep not running

**Trait Implementations:**
`Clone`, `Debug`, `Default`, `Send`, `Sync`

---

#### `struct Instant`
A point-in-time wall-clock measurement. Mimics `std::time::Instant` with additional features.

**Monotonicity Guarantees:**
* Tries to use OS APIs guaranteeing monotonic behavior
* Under rare circumstances, monotonicity can be violated
* Workarounds for bugs: `duration_since`, `elapsed`, `sub` saturate to zero if monotonicity is violated (previously panicked)
* `checked_duration_since` can detect violations

**Key Methods:**
* `pub fn now() -> Instant`
  - Gets current time using a lazily initialized global clock
  - First call may take up to 200ms for calibration
  - Monotonically increasing

* `pub fn recent() -> Instant`
  - Gets the most recent global time
  - Uses upkeep thread or `set_recent()`
  - Falls back to `Instant::now()` if not set

* `pub fn duration_since(&self, earlier: Instant) -> Duration`
  - Time elapsed from `earlier` to `self`
  - Saturates to zero if `earlier` is later

* `pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration>`
  - Time elapsed, or `None` if `earlier` is later or monotonicity violated

* `pub fn saturating_duration_since(&self, earlier: Instant) -> Duration`
  - Time elapsed, or zero if `earlier` is later

* `pub fn elapsed(&self) -> Duration`
  - Time elapsed since this instant was created
  - Saturates to zero if current time is earlier

* `pub fn checked_add(&self, duration: Duration) -> Option<Instant>`
  - Adds duration, returns `None` on overflow

* `pub fn checked_sub(&self, duration: Duration) -> Option<Instant>`
  - Subtracts duration, returns `None` on underflow

**Operator Support:**
- `+` and `+=` with `Duration`
- `-` and `-=` with `Duration`
- `-` between two `Instant`s (returns `Duration`)
- Comparison operators (`<`, `>`, `==`, etc.)

**Example Usage:**
```rust
let start = Instant::now();
// ... do some work ...
let elapsed = start.elapsed();
println!("Work took {:?}", elapsed);
```

---

#### `struct Mock`
Controllable time source for use in tests. Allows adjusting time forwards/backwards.

**Important:** Monotonic guarantees of `Clock` do not apply in mocked mode.

**Methods:**
* `pub fn increment<N: IntoNanoseconds>(&self, amount: N)`
  - Increments the mock time
  - Accepts `u64` or `Duration`

* `pub fn decrement<N: IntoNanoseconds>(&self, amount: N)`
  - Decrements the mock time
  - Allows time to go backwards for testing

* `pub fn value(&self) -> u64`
  - Gets the current value of the mock time in nanoseconds

**Example Usage:**
```rust
let (clock, mock) = Clock::mock();
let start = clock.now();
mock.increment(Duration::from_secs(1));
let end = clock.now();
assert_eq!(end.duration_since(start), Duration::from_secs(1));
```

---

#### `struct Upkeep`
Manages the ultra-low-overhead "recent time" feature by spawning a background thread.

**Purpose:**
- Addresses scenarios where frequent time checking becomes computationally expensive
- Provides a background thread that updates a global "recent" time at a specified interval
- Reading recent time can be 2-3x faster than reading current time

**Performance Benefits:**
- Typical read time reduced from 12-14ns to 4-5ns
- Eliminates need for reference scale conversion
- Ultra-low overhead for high-frequency time checking

**Methods:**
* `pub fn new(interval: Duration) -> Upkeep`
  - Creates a new `Upkeep` with a new internal clock
  - `interval` specifies how often to update the recent time

* `pub fn new_with_clock(interval: Duration, clock: Clock) -> Upkeep`
  - Creates `Upkeep` with an existing `Clock`
  - Avoids recalibration overhead

* `pub fn start(self) -> Result<Handle, Error>`
  - Starts the upkeep thread
  - Returns a `Handle` (drop guard)
  - Errors if thread already running or spawn fails

**Important Constraints:**
- Only one upkeep thread can exist simultaneously
- A new thread can be started after the previous one is dropped
- Time is read and written atomically
- Global to the entire application

**Example Usage:**
```rust
let upkeep = Upkeep::new(Duration::from_millis(1));
let handle = upkeep.start()?; // Start background update thread

// Now Clock::recent() and Instant::recent() will return
// time values updated every millisecond
let recent_time = Instant::recent();
```

---

#### `struct Handle`
Handle to a running upkeep thread for recent time. Dropping this handle stops the upkeep thread.

**Behavior:**
- Implements `Drop` to stop the upkeep thread
- Thread-safe (`Send` and `Sync`)
- No public methods - purely a guard type

---

### Enums

#### `enum Error`
Errors thrown during the creation/spawning of the upkeep thread.

**Variants:**
* `UpkeepRunning`
  - An upkeep thread is already running
  - Only one upkeep thread allowed per process

* `FailedToSpawnUpkeepThread(std::io::Error)`
  - Error occurred when spawning the upkeep thread
  - Contains the underlying IO error

**Trait Implementations:**
`Debug`, `Display`, `std::error::Error`

---

### Traits

#### `trait IntoNanoseconds`
Type which can be converted into a nanosecond representation (u64). Used by `Mock` for flexible time increments.

**Required Methods:**
```rust
fn into_nanos(self) -> u64
```

**Implementors:**
* `u64` - Direct interpretation as nanoseconds
* `core::time::Duration` - Converts standard duration to nanoseconds

**Purpose:**
Allows `Mock::increment` and `Mock::decrement` to accept both raw nanosecond values and `Duration` types for convenience.

---

### Functions

#### `pub fn with_clock<T>(clock: &Clock, f: impl FnOnce() -> T) -> T`
Sets the given `clock` as the default for `Instant` calls within the closure `f`.

**Purpose:**
- Temporarily overrides the global clock for a specific scope
- Only affects `Instant::now()` and `Instant::recent()` calls
- Does not affect direct `Clock` instance calls

**Example:**
```rust
let custom_clock = Clock::new();
with_clock(&custom_clock, || {
    // Instant::now() uses custom_clock here
    let instant = Instant::now();
});
```

---

#### `pub fn set_recent(instant: Instant)`
Sets the global recent time manually.

**Purpose:**
- Alternative to using `Upkeep` for updating recent time
- Useful for custom scheduling in async runtimes
- Allows integration with existing event loops

**Use Cases:**
- Programs using async runtimes that want to avoid extra threads
- Custom time update strategies
- Integration with application-specific scheduling

**Example:**
```rust
// In an async runtime task
loop {
    set_recent(Instant::now());
    tokio::time::sleep(Duration::from_millis(1)).await;
}
```

---

## Performance Considerations

### Benchmarks
- Direct time access (`Clock::now()`): ~12-14ns
- Recent time access (`Clock::recent()`): ~4-5ns
- Raw clock reading (`Clock::raw()`): Minimal overhead

### Optimization Tips
1. Use `Clock::recent()` for high-frequency, low-precision needs
2. Keep `Clock` instances around to avoid recalibration
3. Use `delta_as_nanos()` when you only need nanosecond deltas
4. Consider platform-specific behavior when optimizing

### Memory and Thread Safety
- All types are thread-safe (`Send` + `Sync`)
- Minimal memory footprint
- Lock-free implementation for recent time updates

---

## Common Usage Patterns

### Basic Time Measurement
```rust
use quanta::Clock;

let clock = Clock::new();
let start = clock.now();
// ... perform operation ...
let end = clock.now();
println!("Operation took {} ns", clock.delta_as_nanos(start.raw(), end.raw()));
```

### Using Recent Time for High-Frequency Checks
```rust
use quanta::{Upkeep, Instant};
use std::time::Duration;

// Start upkeep thread
let upkeep = Upkeep::new(Duration::from_millis(1));
let _handle = upkeep.start().expect("Failed to start upkeep");

// Use recent time in hot loop
loop {
    let now = Instant::recent(); // Much faster than Instant::now()
    // ... check timeouts or perform time-based logic ...
}
```

### Testing with Mocked Time
```rust
use quanta::Clock;
use std::time::Duration;

let (clock, mock) = Clock::mock();

let start = clock.now();
mock.increment(Duration::from_secs(60)); // Simulate 1 minute passing
let end = clock.now();

assert_eq!(end.duration_since(start), Duration::from_secs(60));
```

### Custom Recent Time Updates
```rust
use quanta::{Clock, Instant, set_recent};

// In async context
let clock = Clock::new();
tokio::spawn(async move {
    loop {
        set_recent(clock.now());
        tokio::time::sleep(Duration::from_millis(1)).await;
    }
});
```

---

## Caveats and Limitations

### General Limitations
* CPU hotplug behavior is undefined
* Raw TSC values may time warp on some systems
* TSC measurements may drift over time if not using invariant TSC
* Does not track time across system suspends
* First clock creation incurs calibration overhead (up to 200ms)

### Platform-Specific Notes

#### x86/x86_64
- Best performance with invariant TSC
- Falls back to OS timing if TSC unavailable
- Requires SSE2 support

#### WASM
- Browser: Limited to millisecond accuracy
- WASI: Accuracy depends on VM implementation
- No TSC support

#### ARM/Other Architectures
- Falls back to OS-provided timing
- No TSC optimization available

### When Not to Use `quanta`
- When you need time zone information
- When tracking time across system suspends is critical
- When nanosecond accuracy on WASM is required
- When working with dates/calendars (use `chrono` instead)

---

## Best Practices

1. **Reuse Clock Instances**: Create once and share to avoid recalibration
2. **Choose the Right Method**:
   - Use `now()` for accurate timestamps
   - Use `recent()` for high-frequency, approximate time
   - Use `raw()` + `scaled()` for custom optimization
3. **Handle Monotonicity**: Use `checked_duration_since` when time warping is possible
4. **Test Time-Dependent Code**: Always use mocked clocks in tests
5. **Consider Platform Differences**: Test on target platforms for performance

---

## Migration Guide

### From `std::time::Instant`
```rust
// Before
use std::time::Instant;
let start = Instant::now();

// After
use quanta::Instant;
let start = Instant::now();
// API is largely compatible
```

### From Manual TSC Reading
```rust
// Before
unsafe { core::arch::x86_64::_rdtsc() }

// After
let clock = Clock::new();
let raw = clock.raw(); // Safe and portable
```

---

## Related Crates
- `chrono`: For date/time manipulation
- `time`: Alternative time library
- `coarsetime`: Similar "recent time" concept
- `minstant`: Minimal instant implementation