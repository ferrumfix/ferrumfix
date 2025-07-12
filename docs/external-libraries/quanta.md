# Crate `quanta` (version 0.12.5)

Performant cross-platform timing with goodies. `quanta` provides a simple and fast API for measuring the current time and the duration between events. It uses native OS timing functions or the Time Stamp Counter (TSC) on modern CPUs.

**Key Concepts:**
* **Reference Clock:** OS-provided, always available.
* **Source Clock:** TSC-based, used if present and reliable; otherwise, falls back to the reference clock.
* **Calibration:** The source clock is calibrated to the reference clock to provide wall clock time. This happens on the first `Clock` creation and can take up to 200ms (typically 10-20ms).

**Features:**
* **Mocking:** `Clock` can be mocked for testing using `Clock::mock()` and `Mock`.
* **Recent Time:** Ultra-low-overhead access to a slightly-delayed global time via `Clock::recent()` or `Instant::recent()`. This requires an "upkeep" thread (managed by `Upkeep`) or manual updates via `set_recent()`. This can be 4-10x faster than `Clock::now()`.
* **Feature Flags:**
    * `prost`: Provides conversion to `prost_types::Timestamp`.

**Platform Support:**
* Windows (`QueryPerformanceCounter`)
* macOS/iOS (`mach_absolute_time`)
* Linux/\*BSD/Solaris (`clock_gettime`)
* **WASM:**
    * `wasm32-unknown-unknown` (browsers): Uses `window.performance.now` (millisecond accuracy).
    * `wasm32-wasi`: Accuracy depends on VM implementation.

**TSC Support:**
* Requires `x86_64` with SSE2, and constant or nonstop/invariant TSC.
* Nonstop/invariant TSC is preferred for stability across power states (e.g., Intel Nehalem/Skylake+, AMD Phenom+).
* `quanta` queries CPUID to determine TSC usability.

**Caveats:**
* CPU hotplug behavior is undefined.
* Raw TSC values may time warp.
* TSC measurements may drift over time if not using invariant TSC.
* Does not track time across system suspends.

---

## Structs

### `struct Clock`
Unified clock for taking measurements.

* **Private fields**

**Methods:**
* `pub fn new() -> Clock`: Creates a new clock with optimal reference and source clocks. Checks TSC support at creation time.
* `pub fn mock() -> (Clock, Arc<Mock>)`: Creates a new mocked clock and a handle to control its time.
* `pub fn now(&self) -> Instant`: Gets the current time, scaled to reference time. Monotonically increasing for the same `Clock` instance.
* `pub fn raw(&self) -> u64`: Gets underlying time from the fastest source. Not guaranteed to be nanoseconds or monotonic. Must be scaled.
* `pub fn scaled(&self, value: u64) -> Instant`: Scales a raw measurement to reference time (nanoseconds).
* `pub fn delta_as_nanos(&self, start: u64, end: u64) -> u64`: Calculates delta between two raw measurements in nanoseconds. Lower overhead than `delta`.
* `pub fn delta(&self, start: u64, end: u64) -> Duration`: Calculates delta between two raw measurements.
* `pub fn recent(&self) -> Instant`: Gets the most recent (slightly-delayed) global time. Requires upkeep thread or `set_recent()`. Returns `0` if upkeep not running.

**Trait Implementations:**
* `Clone`, `Debug`, `Default`
* Auto traits: `Freeze`, `RefUnwindSafe`, `Send`, `Sync`, `Unpin`, `UnwindSafe`
* Blanket: `Any`, `Borrow<T>`, `BorrowMut<T>`, `From<T>`, `Into<U>`, `ToOwned`, `TryFrom<U>`, `TryInto<U>`

---

### `struct Mock`
Controllable time source for use in tests. Allows adjusting time forwards/backwards. Monotonic guarantees of `Clock` do not apply in mocked mode.

* **Private fields**

**Methods:**
* `pub fn increment<N: IntoNanoseconds>(&self, amount: N)`: Increments the mock time.
* `pub fn decrement<N: IntoNanoseconds>(&self, amount: N)`: Decrements the mock time.
* `pub fn value(&self) -> u64`: Gets the current value of the mock time.

**Trait Implementations:**
* `Clone`, `Debug`
* Auto traits: `Freeze`, `RefUnwindSafe`, `Send`, `Sync`, `Unpin`, `UnwindSafe`
* Blanket: (Same as `Clock`)

---

### `struct Handle`
Handle to a running upkeep thread for recent time. If dropped, the upkeep thread stops.

* **Private fields**

**Trait Implementations:**
* `Debug`, `Drop` (stops the upkeep thread)
* Auto traits: `Freeze`, `Send`, `Sync`, `Unpin` (`!RefUnwindSafe`, `!UnwindSafe`)
* Blanket: (Same as `Clock`, excluding `CloneToUninit` and `ToOwned`)

---

### `struct Instant`
A point-in-time wall-clock measurement. Mimics `std::time::Instant`.

* **Private fields**

**Monotonicity:**
* Tries to use OS APIs guaranteeing monotonic behavior.
* Workarounds for bugs: `duration_since`, `elapsed`, `sub` saturate to zero if monotonicity is violated (previously panicked).
* `checked_duration_since` can detect violations.

**Methods:**
* `pub fn now() -> Instant`: Gets current time using a lazily initialized global clock (up to 200ms for first call). Monotonically increasing.
* `pub fn recent() -> Instant`: Gets the most recent global time. Uses upkeep thread or `set_recent()`. Falls back to `Instant::now()` if not set.
* `pub fn duration_since(&self, earlier: Instant) -> Duration`: Time elapsed from `earlier` to `self`. Saturates to zero if `earlier` is later.
* `pub fn checked_duration_since(&self, earlier: Instant) -> Option<Duration>`: Time elapsed, or `None` if `earlier` is later or monotonicity violated.
* `pub fn saturating_duration_since(&self, earlier: Instant) -> Duration`: Time elapsed, or zero if `earlier` is later.
* `pub fn elapsed(&self) -> Duration`: Time elapsed since this instant was created. Saturates to zero if current time is earlier.
* `pub fn checked_add(&self, duration: Duration) -> Option<Instant>`: Adds duration, returns `None` on overflow.
* `pub fn checked_sub(&self, duration: Duration) -> Option<Instant>`: Subtracts duration, returns `None` on underflow.

**Trait Implementations:**
* `Add<Duration>`, `AddAssign<Duration>`, `Clone`, `Copy`, `Debug`, `Eq`, `Ord`, `PartialEq`, `PartialOrd`, `Sub<Instant, Output = Duration>`, `Sub<Duration>`, `SubAssign<Duration>`
* Auto traits: `Freeze`, `RefUnwindSafe`, `Send`, `Sync`, `Unpin`, `UnwindSafe`
* Blanket: (Same as `Clock`)

---

### `struct Upkeep`
Manages the ultra-low-overhead "recent time" feature by spawning a background thread.

* **Private fields**

**Methods:**
* `pub fn new(interval: Duration) -> Upkeep`: Creates a new `Upkeep` with a new internal clock.
* `pub fn new_with_clock(interval: Duration, clock: Clock) -> Upkeep`: Creates `Upkeep` with an existing `Clock` (avoids recalibration).
* `pub fn start(self) -> Result<Handle, Error>`: Starts the upkeep thread. Returns a `Handle` (drop guard). Errors if thread already running or spawn fails.

**Trait Implementations:**
* `Debug`
* Auto traits: `Freeze`, `RefUnwindSafe`, `Send`, `Sync`, `Unpin`, `UnwindSafe`
* Blanket: (Same as `Clock`, excluding `CloneToUninit` and `ToOwned`)

---

## Enums

### `enum Error`
Errors thrown during the creation/spawning of the upkeep thread.

**Variants:**
* `UpkeepRunning`: An upkeep thread is already running.
* `FailedToSpawnUpkeepThread(std::io::Error)`: Error occurred when spawning the upkeep thread.

**Trait Implementations:**
* `Debug`, `Display`, `std::error::Error`
* Auto traits: `Freeze`, `Send`, `Sync`, `Unpin` (`!RefUnwindSafe`, `!UnwindSafe`)
* Blanket: `Any`, `Borrow<T>`, `BorrowMut<T>`, `From<T>`, `Into<U>`, `ToString`, `TryFrom<U>`, `TryInto<U>`

---

## Traits

### `trait IntoNanoseconds`
Type which can be converted into a nanosecond representation (u64). Used by `Mock`.

**Required Methods:**
* `fn into_nanos(self) -> u64`: Consumes value, converts to nanoseconds.

**Implementors:**
* `u64`
* `core::time::Duration`

---

## Functions

### `pub fn with_clock<T>(clock: &Clock, f: impl FnOnce() -> T) -> T`
Sets the given `clock` as the default for `Instant` calls within the closure `f`. Does not affect direct `Clock` instance calls.

### `pub fn set_recent(instant: Instant)`
Sets the global recent time manually. Alternative to using `Upkeep`. Useful for custom scheduling, e.g., in async runtimes.

---
**General Crate Items (from `all.html`):**
* **Structs:** `Clock`, `Handle`, `Instant`, `Mock`, `Upkeep`
* **Enums:** `Error`
* **Traits:** `IntoNanoseconds`
* **Functions:** `set_recent`, `with_clock`
