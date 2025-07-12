`smartstring` is a Rust crate that provides a drop-in replacement for `std::string::String` implementing the Small
String Optimization (SSO). This is highly relevant for low-latency crypto exchange trading systems where minimizing heap
allocations and maximizing cache locality are critical for performance and reducing jitter.

### Core Concept for Low-Latency Trading

`SmartString` avoids heap allocation for strings that are small enough to fit within the struct's own memory space on
the stack.

* **Heap Allocation Avoidance**: Heap allocation is a primary source of non-deterministic latency (jitter) in
  high-performance applications. It involves system calls and can be slow. By storing short, common strings like trading
  symbols (`"BTC-USDT"`), order IDs, or status messages (`"FILLED"`, `"CANCELED"`) directly on the stack, `smartstring`
  eliminates this performance bottleneck.
* **Cache Locality**: Storing string data inline means it resides on the stack along with other local variables. This
  drastically increases the probability that the data is in the CPU's L1/L2 cache. Accessing cached data is orders of
  magnitude faster than fetching from main memory (RAM), which is a common result of dereferencing a heap pointer. This
  is crucial when using strings as keys in `HashMap` or `BTreeMap` for order books or state management, as lookups
  become significantly faster.

### Key Implementation Details

* **Size**: `SmartString` is the same size as a standard `String` (24 bytes on 64-bit architectures).
* **Inline Capacity**: It can store strings up to `23` bytes in length inline (`MAX_INLINE`). This is sufficient for
  most exchange symbols, status codes, and identifiers.
* **Mechanism**: It transparently switches between an inline representation and a heap-allocated (boxed) `String`. The
  switch is managed using a discriminant bit stored within the pointer space, avoiding any extra storage overhead for
  the flag itself.

### Critical Performance Tuning: `LazyCompact` vs. `Compact`

`SmartString` offers two modes, which are critical to understand for performance tuning in a trading context. The mode
is selected via a generic type parameter.

1. **`LazyCompact` (Recommended for Trading)**
    * **Type Alias**: `use smartstring::alias::String;`
    * **Behavior**: Once a string's length exceeds the inline capacity, it is moved to the heap. If the string later
      shrinks to a size that could be inlined, it **remains on the heap**.
    * **Low-Latency Impact**: This is the **preferred mode** for most trading applications. It avoids the performance
      cost and unpredictable latency of deallocating and re-inlining a string if its length fluctuates around the
      23-byte threshold. The goal in low-latency is predictable performance; `LazyCompact` ensures that after the
      one-time cost of heap allocation, the performance characteristics remain stable.

2. **`Compact` (Use with Caution)**
    * **Type Alias**: `use smartstring::alias::CompactString;`
    * **Behavior**: Aggressively re-inlines a heap-allocated string if it shrinks to fit within the inline capacity.
    * **Low-Latency Impact**: This mode prioritizes memory usage and cache locality over predictable performance. The
      documentation warns this can cause "multiple unintended allocations" if a string's length varies across the inline
      threshold. This allocation/deallocation churn introduces jitter, making `Compact` generally **unsuitable for the
      critical execution path** of a low-latency trading system. It might be acceptable for non-critical configuration
      or setup code.

### Practical Application in a Trading System

* **Data Structures**: Use `smartstring::alias::String` for keys in `HashMap` or `BTreeMap` that store order books,
  client order states, or symbol-specific data. This leverages cache locality for faster lookups.
* **Message Parsing/Serialization**: When handling FIX or proprietary binary protocols, identifiers, tags, and symbols
  are often short. Using `SmartString` during parsing can prevent numerous small heap allocations. The `serde` feature
  flag allows for direct, efficient serialization and deserialization.
* **Core Logic**: Represent all short, fixed-size, or typically-small text identifiers with `SmartString` instead of
  `std::string::String` throughout the application logic to benefit from SSO.

In summary, `smartstring` is an essential optimization tool for Rust-based low-latency trading. For maximum performance
and predictability, developers should use the `LazyCompact` mode (via the `smartstring::alias::String` type alias) to
eliminate heap allocations for the vast majority of string-based identifiers used in trading systems.