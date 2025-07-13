# rust_decimal Crate Documentation

## Overview

The `rust_decimal` crate provides a pure Rust implementation of a decimal number type suitable for financial calculations and scenarios requiring high precision without round-off errors. It represents values as a 128-bit fixed-precision number.

- **Repository**: [https://github.com/paupino/rust-decimal](https://github.com/paupino/rust-decimal)
- **Documentation**: [https://docs.rs/rust_decimal](https://docs.rs/rust_decimal)
- **Current Version**: 1.37.2
- **Minimum Rust Version**: 1.67.1

## Core Architecture

### Decimal Representation

The `Decimal` type internally stores:
- **96-bit unsigned integer mantissa**: The significant digits
- **1-bit sign**: Positive or negative
- **8-bit scaling factor**: Exponent `e` (0-28 inclusive)

Values are represented as: `m / 10^e` where:
- `m` is an integer where `-2^96 < m < 2^96`
- `e` is an integer between 0 and 28 inclusive

### Key Characteristics

- Supports up to ~28 significant decimal digits
- Preserves trailing zeros (e.g., `1.20` != `1.2` in string representation, but equal in value)
- Designed for financial calculations requiring exact decimal representation
- No implicit rounding or precision loss in basic arithmetic

## The Decimal Type

```rust
pub struct Decimal { /* private fields */ }
```

### Associated Constants

**Mathematical Constants:**
- `Decimal::PI`: π (3.1415926535897932384626433832)
- `Decimal::HALF_PI`: π/2
- `Decimal::QUARTER_PI`: π/4
- `Decimal::TWO_PI`: 2π
- `Decimal::E`: Euler's number (2.7182818284590452353602874713)
- `Decimal::E_INVERSE`: 1/e

**Numeric Constants:**
- `Decimal::ZERO`: 0
- `Decimal::ONE`: 1
- `Decimal::NEGATIVE_ONE`: -1
- `Decimal::TWO`: 2
- `Decimal::TEN`: 10
- `Decimal::ONE_HUNDRED`: 100
- `Decimal::ONE_THOUSAND`: 1000

**Boundary Values:**
- `Decimal::MAX`: 79,228,162,514,264,337,593,543,950,335
- `Decimal::MIN`: -79,228,162,514,264,337,593,543,950,335
- `Decimal::MAX_SCALE`: 28 (maximum decimal places)

### Creation Methods

#### Using the `dec!` Macro (Recommended)

```rust
use rust_decimal::dec;  // Requires 'macros' feature

let price = dec!(19.99);
let scientific = dec!(1.23e-5);
let negative = dec!(-456.789);
```

#### From Integer and Scale

```rust
// Create 2.02
let d = Decimal::new(202, 2);

// Checked version
let d = Decimal::try_new(202, 2)?;

// From i128 with scale
let d = Decimal::from_i128_with_scale(5_897_932_384_626_433_832, 2);
// Result: 58979323846264338.32

// Checked version
let d = Decimal::try_from_i128_with_scale(large_num, scale)?;
```

#### From Parts

```rust
// Construct from raw 96-bit parts
let pi = Decimal::from_parts(1102470952, 185874565, 1703060790, false, 28);
// Result: 3.1415926535897932384626433832
```

#### From Strings

```rust
// Standard parsing
let d: Decimal = "123.45".parse()?;
let d = Decimal::from_str("123.45")?;

// Scientific notation
let d = Decimal::from_scientific("9.7e-7")?; // 0.00000097

// Specific radix
let hex = Decimal::from_str_radix("FF", 16)?; // 255

// Exact parsing (fails on underflow)
let d = Decimal::from_str_exact("0.0000000000000000000000000001")?;
```

#### From Primitive Types

```rust
use rust_decimal::prelude::*;

// Using From trait
let d: Decimal = 42i32.into();
let d: Decimal = 123u64.into();

// Using FromPrimitive
let d = Decimal::from_i32(123);
let d = Decimal::from_u64(456);
```

#### From Floating Point

```rust
// Standard conversion (may lose precision)
let d = Decimal::try_from(0.1_f32)?;
let d = Decimal::from_f32(0.1)?;

// Retain exact float representation
let d = Decimal::from_f32_retain(0.1)?;
// Result: 0.100000001490116119384765625 (exact f32 representation)

// Similar for f64
let d = Decimal::from_f64_retain(0.1)?;
// Result: 0.1000000000000000055511151231 (exact f64 representation)
```

### Accessor Methods

```rust
let decimal = dec!(123.45);

decimal.scale();            // 2 (decimal places)
decimal.mantissa();         // 12345 (as i128)
decimal.is_sign_negative(); // false
decimal.is_sign_positive(); // true
decimal.is_zero();          // false
decimal.is_integer();       // false (has fractional part)
```

### Manipulation Methods

```rust
let mut d = dec!(-123.45);

// Change sign
d.set_sign_positive(true);  // Now 123.45
d.set_sign_negative(true);  // Now -123.45

// Change scale (may cause overflow/underflow)
d.set_scale(1)?;  // Attempts to represent as -123.4

// Rescale in-place
d.rescale(3);  // -123.450

// Normalize (remove trailing zeros)
let normalized = dec!(3.100).normalize();  // 3.1
```

### Rounding Methods

```rust
let d = dec!(6.5);

// Default rounding (Banker's Rounding)
d.round();     // 6 (rounds to nearest even)

// Round to decimal places
d.round_dp(0); // 6

// Round with specific strategy
d.round_dp_with_strategy(0, RoundingStrategy::MidpointAwayFromZero); // 7

// Round to significant figures
d.round_sf(1);     // Some(7)
d.round_sf_with_strategy(2, RoundingStrategy::ToZero); // Some(6.5)

// Truncation
dec!(6.789).trunc();              // 6
dec!(6.789).trunc_with_scale(1);  // 6.7
```

### Mathematical Operations

```rust
let a = dec!(10.5);
let b = dec!(2.5);

// Basic operations
a + b;  // 13.0
a - b;  // 8.0
a * b;  // 26.25
a / b;  // 4.2
a % b;  // 0.5

// Checked operations (return Option)
a.checked_add(b);    // Some(13.0)
a.checked_sub(b);    // Some(8.0)
a.checked_mul(b);    // Some(26.25)
a.checked_div(b);    // Some(4.2)
a.checked_rem(b);    // Some(0.5)

// Saturating operations
Decimal::MAX.saturating_add(dec!(1));  // Decimal::MAX
Decimal::MIN.saturating_sub(dec!(1));  // Decimal::MIN

// Other operations
a.abs();    // 10.5
a.floor();  // 10
a.ceil();   // 11
a.fract();  // 0.5
a.max(b);   // 10.5
a.min(b);   // 2.5
```

### Advanced Mathematical Operations (requires 'maths' feature)

```rust
use rust_decimal::MathematicalOps;

let d = dec!(2.0);

// Exponential and logarithmic
d.exp();           // e^2
d.exp_with_tolerance(dec!(0.0001));
d.ln();            // Natural log
d.log10();         // Base 10 log
d.checked_ln();    // Returns Option

// Power functions
d.powi(3);         // 2^3 = 8
d.powu(3);         // 2^3 = 8 (unsigned exponent)
d.powf(2.5);       // 2^2.5
d.powd(dec!(2.5)); // 2^2.5 (Decimal exponent)
d.sqrt();          // Square root

// Trigonometric
d.sin();
d.cos();
d.tan();

// Statistical
d.erf();           // Error function
d.norm_cdf();      // Normal cumulative distribution
d.norm_pdf();      // Normal probability density
```

### Serialization

```rust
// To/from raw bytes
let bytes = decimal.serialize();        // [u8; 16]
let decimal = Decimal::deserialize(bytes);

// Unpacking (for debugging)
let unpacked = decimal.unpack();
// Returns UnpackedDecimal { negative, scale, hi, mid, lo }
```

## RoundingStrategy Enum

Controls how rounding is performed:

```rust
pub enum RoundingStrategy {
    // Default for .round() and .round_dp()
    MidpointNearestEven,    // 6.5 → 6, 7.5 → 8 (Banker's Rounding)

    MidpointAwayFromZero,   // 6.5 → 7, -6.5 → -7
    MidpointTowardZero,     // 6.5 → 6, -6.5 → -6

    ToZero,                 // 6.8 → 6, -6.8 → -6 (Truncate)
    AwayFromZero,           // 6.8 → 7, -6.8 → -7

    ToNegativeInfinity,     // 6.8 → 6, -6.8 → -7 (Floor)
    ToPositiveInfinity,     // 6.8 → 7, -6.8 → -6 (Ceiling)
}
```

## Error Handling

```rust
pub enum Error {
    ErrorString(String),                    // Generic error (legacy)
    ExceedsMaximumPossibleValue,           // Value > Decimal::MAX
    LessThanMinimumPossibleValue,          // Value < Decimal::MIN
    Underflow,                             // Too many fractional digits
    ScaleExceedsMaximumPrecision(u32),    // Scale > 28
    ConversionTo(String),                  // Conversion failure
}
```

## Trait Implementations

### Arithmetic Traits
- `Add`, `Sub`, `Mul`, `Div`, `Rem` (and `*Assign` variants)
- `Neg` for negation
- Works with both owned and borrowed values

### Comparison Traits
- `PartialEq`, `Eq`: Value-based equality (ignores representation)
- `PartialOrd`, `Ord`: Numerical ordering
- `Hash`: Based on normalized value

### Formatting Traits
- `Display`: Standard decimal notation
- `Debug`: Same as Display
- `LowerExp`, `UpperExp`: Scientific notation

### Conversion Traits
- `From<T>` for all integer types
- `TryFrom<Decimal>` for numeric types
- `FromStr` for string parsing
- `TryFrom<f32>`, `TryFrom<f64>`

### Numeric Traits (num-traits)
- `Zero`, `One`: Identity elements
- `Num`: Basic numeric operations
- `Signed`: Sign-related operations
- `FromPrimitive`, `ToPrimitive`: Conversions
- `CheckedAdd`, `CheckedSub`, etc.: Checked operations

### Other Traits
- `Default`: Returns `Decimal::ZERO`
- `Copy`, `Clone`
- `Send`, `Sync`
- `Sum`, `Product`: For iteration

## Feature Flags

### Core Features

- **`macros`**: Enables the `dec!` macro for compile-time creation
- **`std`**: Standard library support (default, disable for no_std)
- **`maths`**: Advanced mathematical operations
- **`maths-nopanic`**: Makes ln/log10 return 0 instead of panicking

### Serialization Features

- **`serde`**: Basic serde support (serializes as string)
- **`serde-float`**: Serialize as JSON float
- **`serde-str`**: Force string serialization
- **`serde-arbitrary-precision`**: Use serde_json's arbitrary precision
- **`serde-with-*`**: Modules for field-level serde control

### Database Features

- **`db-postgres`**: PostgreSQL NUMERIC type support
- **`db-tokio-postgres`**: Async PostgreSQL support
- **`db-diesel-postgres`**: Diesel PostgreSQL support
- **`db-diesel-mysql`**: Diesel MySQL support

### Other Features

- **`borsh`**: Borsh serialization
- **`ndarray`**: ndarray integration
- **`rand`**: Random Decimal generation
- **`rocket-traits`**: Rocket web framework support
- **`c-repr`**: C ABI compatibility (#[repr(C)])
- **`legacy-ops`**: Old arithmetic algorithm (deprecated)

## The dec! Macro (rust_decimal_macros)

The `dec!` macro provides compile-time Decimal creation:

### Syntax

```rust
dec!(literal_number)
dec!(integer_literal, radix <N>)
dec!(integer_literal, exp <E>)
dec!(integer_literal, radix <N>, exp <E>)
```

### Supported Formats

```rust
// Integers
dec!(123)
dec!(-456)
dec!(1_000_000)

// Binary, octal, hex
dec!(0b1010)  // 10
dec!(0o777)   // 511
dec!(0xFF)    // 255

// Decimal
dec!(1.2345)
dec!(-0.005)

// Scientific notation
dec!(1e6)     // 1,000,000
dec!(-2.5e-3) // -0.0025
```

### Options

**radix**: Custom base (2-36)
```rust
dec!(100, radix 2)      // Binary 100 = 4
dec!(-1_222, radix 3)   // Base 3 = -53
dec!(z1, radix 36)      // Base 36 = 1261
```

**exp**: Power of 10 exponent (-28 to 28)
```rust
dec!(123, exp -2)                    // 1.23
dec!(10, radix 2, exp 5)            // 2 * 10^5 = 200,000
dec!(-1_777, radix 8, exp -3)       // -1.023
```

### Feature: reexportable

- **Default**: Generates `::rust_decimal::Decimal::from_parts(...)`
- **With `reexportable`**: Generates `Decimal::from_parts(...)` (requires `Decimal` in scope)

## Prelude Module

For convenient imports:

```rust
use rust_decimal::prelude::*;
```

Includes:
- `Decimal` struct
- `RoundingStrategy` enum
- `dec!` macro (if `macros` feature enabled)
- Numeric traits: `Zero`, `One`, `FromPrimitive`, `ToPrimitive`, `Signed`
- `FromStr` trait
- `MathematicalOps` trait (if `maths` feature enabled)

## Common Usage Patterns

### Financial Calculations

```rust
use rust_decimal::prelude::*;

let price = dec!(19.99);
let tax_rate = dec!(0.0875);  // 8.75%
let quantity = dec!(3);

let subtotal = price * quantity;
let tax = (subtotal * tax_rate).round_dp(2);
let total = subtotal + tax;

println!("Subtotal: ${}", subtotal);  // $59.97
println!("Tax: ${}", tax);            // $5.25
println!("Total: ${}", total);        // $65.22
```

### Avoiding Float Precision Issues

```rust
// Float arithmetic problem
let float_result = 0.1 + 0.2;  // 0.30000000000000004

// Decimal arithmetic is exact
let decimal_result = dec!(0.1) + dec!(0.2);  // 0.3
```

### Currency Conversion

```rust
let usd_amount = dec!(100.00);
let exchange_rate = dec!(1.2567);

let eur_amount = (usd_amount / exchange_rate).round_dp(2);
println!("€{}", eur_amount);  // €79.57
```

### Working with Percentages

```rust
let value = dec!(250);
let percentage = dec!(15);  // 15%

let increase = value * (percentage / dec!(100));
let new_value = value + increase;

println!("15% of {} is {}", value, increase);  // 37.5
println!("New value: {}", new_value);           // 287.5
```

## Performance Considerations

- Decimal operations are slower than floating-point
- Each operation may allocate (unlike primitives)
- Best for correctness over raw performance
- Consider caching frequently used values
- Use `normalize()` to reduce memory usage when many trailing zeros

## Common Pitfalls

1. **Scale Overflow**: Operations can increase scale beyond 28
   ```rust
   let a = dec!(0.0000000000001);  // scale 13
   let b = dec!(0.0000000000001);  // scale 13
   let c = a * b;  // Would need scale 26 - OK
   let d = c * a;  // Would need scale 39 - ERROR!
   ```

2. **String Representation**: Trailing zeros are preserved
   ```rust
   let a = dec!(1.20);
   let b = dec!(1.2);
   assert_eq!(a, b);              // Values are equal
   assert_ne!(a.to_string(), b.to_string());  // "1.20" != "1.2"
   ```

3. **Division Precision**: Division may require rounding
   ```rust
   let a = dec!(10);
   let b = dec!(3);
   let c = a / b;  // 3.3333333333333333333333333333 (28 decimal places)
   ```

## Best Practices

1. Use the `dec!` macro for literals
2. Always handle potential errors in conversions
3. Be explicit about rounding strategies
4. Use checked operations when overflow is possible
5. Normalize values when trailing zeros aren't needed
6. Consider the scale implications of operations
7. Use appropriate feature flags for your use case