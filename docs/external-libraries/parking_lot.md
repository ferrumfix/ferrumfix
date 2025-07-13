# parking_lot - High-Performance Synchronization Primitives

## Overview

`parking_lot` is a Rust library providing more efficient synchronization primitives than the standard library's `std::sync` module. It offers compact storage, superior performance, and additional features while maintaining a similar API for easy migration.

## Key Advantages

### 1. **Compact Storage**
- `Mutex` and `Once`: Only 1 byte
- `Condvar` and `RwLock`: 1 word (8 bytes on 64-bit)
- Standard library equivalents use significantly more memory

### 2. **Performance**
- `Mutex`: 1.5x faster uncontended, up to 5x faster under contention
- `RwLock`: Generally faster, especially under contention
- Fast inline paths for uncontended locks
- Adaptive spinning before thread suspension

### 3. **Additional Features**
- No poisoning on panic (simplifies error handling)
- Supports Windows XP
- Hardware lock elision support
- Task-fair locking policy
- No spurious wakeups in condition variables
- Atomic lock downgrading/upgrading for RwLock
- `ReentrantMutex` for recursive locking

## Installation

```toml
[dependencies]
parking_lot = "0.12"
```

### Optional Features

```toml
[dependencies]
parking_lot = { version = "0.12", features = ["deadlock_detection", "send_guard"] }
```

Available features:
- `nightly`: Enable nightly-only functionality
- `deadlock_detection`: Experimental deadlock detector
- `send_guard`: Allow sending lock guards between threads
- `hardware-lock-elision`: Hardware lock optimization (x86_64 only)
- `serde`: Serialization support
- `arc_lock`: Arc-based locks with `'static` lifetime

## Core Types

### Mutex

Mutual exclusion primitive for protecting shared data.

```rust
use parking_lot::Mutex;

let mutex = Mutex::new(5);

// Lock the mutex
let mut guard = mutex.lock();
*guard += 1;
assert_eq!(*guard, 6);
// Mutex is automatically unlocked when guard goes out of scope
```

Key differences from `std::sync::Mutex`:
- No poisoning
- `lock()` doesn't return a `Result`
- More efficient implementation

### RwLock

Reader-writer lock allowing multiple readers or one writer.

```rust
use parking_lot::RwLock;

let lock = RwLock::new(5);

// Multiple readers
{
    let r1 = lock.read();
    let r2 = lock.read();
    assert_eq!(*r1, 5);
    assert_eq!(*r2, 5);
} // Read locks released

// Single writer
{
    let mut w = lock.write();
    *w += 1;
    assert_eq!(*w, 6);
} // Write lock released
```

Unique features:
- Upgradable read locks
- Atomic downgrading from write to read
- Try-upgrade from upgradable to write

### ReentrantMutex

Allows recursive locking by the same thread.

```rust
use parking_lot::ReentrantMutex;

let mutex = ReentrantMutex::new(5);
let guard1 = mutex.lock();
let guard2 = mutex.lock(); // Same thread can lock again
```

### Condvar

Condition variable for thread synchronization.

```rust
use parking_lot::{Mutex, Condvar};
use std::sync::Arc;
use std::thread;

let pair = Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = Arc::clone(&pair);

thread::spawn(move || {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock();
    *started = true;
    cvar.notify_one();
});

let (lock, cvar) = &*pair;
let mut started = lock.lock();
while !*started {
    cvar.wait(&mut started);
}
```

### Once

One-time initialization primitive.

```rust
use parking_lot::Once;

static INIT: Once = Once::new();
static mut DATA: usize = 0;

INIT.call_once(|| {
    // This will only execute once
    unsafe { DATA = 1; }
});
```

## Advanced Usage

### Mapped Guards

Lock guards can be mapped to access specific fields:

```rust
use parking_lot::{Mutex, MutexGuard};

struct Foo {
    value: i32,
}

let mutex = Mutex::new(Foo { value: 42 });
let guard = mutex.lock();
let value_guard = MutexGuard::map(guard, |foo| &mut foo.value);
```

### Fair Mutex

Prevents thread starvation by ensuring fair access:

```rust
use parking_lot::FairMutex;

let mutex = FairMutex::new(0);
// Threads acquire the lock in FIFO order
```

### Deadlock Detection

Enable with the `deadlock_detection` feature:

```rust
#[cfg(feature = "deadlock_detection")]
{
    use parking_lot::deadlock;
    use std::thread;

    // Create a background thread to check for deadlocks
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(10));
            let deadlocks = deadlock::check_deadlock();
            if !deadlocks.is_empty() {
                println!("Deadlock detected!");
                for (i, threads) in deadlocks.iter().enumerate() {
                    println!("Deadlock #{}", i);
                    for t in threads {
                        println!("Thread Id {:#?}", t.thread_id());
                        println!("{:#?}", t.backtrace());
                    }
                }
            }
        }
    });
}
```

## Migration from std::sync

### Basic Migration

```rust
// Before
use std::sync::{Mutex, RwLock};

let mutex = Mutex::new(5);
let guard = mutex.lock().unwrap(); // Returns Result

// After
use parking_lot::{Mutex, RwLock};

let mutex = Mutex::new(5);
let guard = mutex.lock(); // Direct lock, no Result
```

### Handling Poisoning

Since parking_lot doesn't implement poisoning:

```rust
// std::sync - must handle poisoning
match mutex.lock() {
    Ok(guard) => { /* use guard */ }
    Err(poisoned) => {
        // Recover from poisoning
        let guard = poisoned.into_inner();
    }
}

// parking_lot - no poisoning to handle
let guard = mutex.lock();
// Use guard directly
```

## Performance Considerations

### When to Use parking_lot

- **High contention scenarios**: Superior performance under lock contention
- **Memory-constrained systems**: Significantly smaller memory footprint
- **Latency-sensitive applications**: Faster lock acquisition
- **Simple error handling**: No poisoning complexity

### When to Stick with std::sync

- **Poisoning required**: If you need poison detection on panic
- **Minimal dependencies**: When avoiding external dependencies
- **Standard library compatibility**: When working with APIs expecting std types

## Implementation Details

### The "Parking Lot" Algorithm

The library uses a global hash table mapping lock addresses to queues of waiting threads. This approach:
- Reduces per-lock memory overhead
- Enables efficient thread parking/unparking
- Allows for adaptive spinning strategies

### Memory Ordering

All operations use appropriate memory ordering guarantees:
- Acquire ordering on lock acquisition
- Release ordering on lock release
- Sequential consistency where required

## Thread Safety

All types implement `Send` and `Sync` where appropriate:
- `Mutex<T>`: `Send` and `Sync` if `T: Send`
- `RwLock<T>`: `Send` and `Sync` if `T: Send + Sync`
- Guards are `Send` only with the `send_guard` feature

## Best Practices

1. **Prefer parking_lot for new projects**: Better performance and simpler API
2. **Use const constructors**: `const MUTEX: Mutex<i32> = Mutex::const_new(0);`
3. **Avoid holding locks across await points**: Can cause deadlocks in async code
4. **Use RwLock for read-heavy workloads**: Multiple readers improve throughput
5. **Consider FairMutex for fairness**: Prevents thread starvation
6. **Enable deadlock detection in debug builds**: Helps catch issues early

## Common Patterns

### Lazy Initialization

```rust
use parking_lot::{Mutex, Once};
use std::sync::Arc;

struct Config {
    data: String,
}

static INIT: Once = Once::new();
static mut CONFIG: Option<Arc<Config>> = None;

fn get_config() -> Arc<Config> {
    unsafe {
        INIT.call_once(|| {
            CONFIG = Some(Arc::new(Config {
                data: "initialized".to_string(),
            }));
        });
        CONFIG.as_ref().unwrap().clone()
    }
}
```

### Producer-Consumer

```rust
use parking_lot::{Mutex, Condvar};
use std::collections::VecDeque;

struct Queue<T> {
    items: Mutex<VecDeque<T>>,
    not_empty: Condvar,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue {
            items: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new(),
        }
    }

    fn push(&self, item: T) {
        self.items.lock().push_back(item);
        self.not_empty.notify_one();
    }

    fn pop(&self) -> T {
        let mut items = self.items.lock();
        while items.is_empty() {
            self.not_empty.wait(&mut items);
        }
        items.pop_front().unwrap()
    }
}
```

## Version Requirements

- Minimum Rust version: 1.64
- Version changes to MSRV are considered breaking changes

## References

- [GitHub Repository](https://github.com/Amanieu/parking_lot)
- [API Documentation](https://docs.rs/parking_lot)
- [WebKit's Parking Lot](https://webkit.org/blog/6161/locking-in-webkit/) (inspiration)