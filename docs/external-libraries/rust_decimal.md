# Crate: `rust_decimal_macros` (version 1.37.1)

## Overview

The `rust_decimal_macros` crate provides a helpful procedural macro, `dec!`, for instantiating `rust_decimal::Decimal`
numbers directly from literal values at compile time. This allows for more ergonomic creation of `Decimal` types.

## Macro: `dec!`

The `dec!` macro transforms a literal number into a `Decimal` at compile time.

**Syntax:**

```rust
dec!(literal_number)
dec!(integer_literal, radix <N>)
dec!(integer_literal, exp <E>)
dec!(integer_literal, radix <N>, exp <E>)
````

**Features:**

1. **Supported Number Formats:**

    * **Integers:** `dec!(123)`, `dec!(-456)`, `dec!(1_000_000)`
    * **Binary:** `dec!(0b1010)` (evaluates to 10)
    * **Octal:** `dec!(0o777)` (evaluates to 511)
    * **Hexadecimal:** `dec!(0xFF)` (evaluates to 255)
    * **Floating-point like:** `dec!(1.2345)`, `dec!(-0.005)`
    * **Scientific notation:** `dec!(1e6)` (evaluates to 1,000,000), `dec!(-2.5e-3)` (evaluates to -0.0025)

2. **Option: `radix`**

    * Allows specifying a custom radix (base) for integer literals.
    * The radix can be any integer from 2 to 36 (inclusive).
    * Letters `a-z` (case-insensitive) are used for digits greater than 9.
    * **Example:**
        * `dec!(100, radix 2)` is equivalent to `Decimal::from(4)`
        * `dec!(-1_222, radix 3)` is equivalent to `Decimal::from(-53)`
        * `dec!(z1, radix 36)` is equivalent to `Decimal::from(1261)`

3. **Option: `exp`**

    * Specifies a 10's exponent, similar to the `e` in float syntax.
    * This option is necessary when using `radix` if an exponent is also needed.
    * The exponent value must be between -28 and +28 (inclusive).
    * **Example:**
        * `dec!(10, radix 2, exp 5)` is equivalent to `dec!(2, exp 5)` which results in `Decimal::new(200_000, 0)` (2 \*
          10^5)
        * `dec!(-1_777, radix 8, exp -3)` is equivalent to `dec!(-1023, exp -3)` which results in `dec!(-1.023)`

4. **`reexportable` Feature Flag:**

    * **Default Behavior:** When the `reexportable` feature is *not* enabled, the macro generates code that refers to
      `Decimal` using its full path: `::rust_decimal::Decimal::from_parts(...)`. This requires `rust_decimal` to be a
      direct dependency available at the project root.
    * **With `reexportable` Enabled:** If the `reexportable` feature is enabled, the macro generates code that refers to
      `Decimal` directly: `Decimal::from_parts(...)`. This is useful for crates that re-export `rust_decimal` and want
      the macro to use the re-exported `Decimal` type. In this case, `Decimal` must be in scope where `dec!` is used.

**Usage Examples:**

```rust
use rust_decimal_macros::dec;

// If the `reexportable` feature is enabled for rust_decimal_macros,
// `Decimal` from `rust_decimal` needs to be in scope.
#[cfg(feature = "reexportable")]
use rust_decimal::Decimal; // Or your re-exported path to Decimal

// Basic usage
let simple_decimal = dec!(1.2345);
assert_eq!("1.2345", simple_decimal.to_string());

let negative_decimal = dec!(-5.4321);
assert_eq!("-5.4321", negative_decimal.to_string());

// Integer with different base (octal)
let octal_decimal = dec!(-0o1_777); // Interpreted as -1023
assert_eq!("-1023", octal_decimal.to_string());

// Integer with explicit radix
let radix_decimal = dec!(-1_777, radix 8); // -1*8^3 + 7*8^2 + 7*8^1 + 7*8^0 = -1023
assert_eq!("-1023", radix_decimal.to_string());

// Using exp
let exp_decimal = dec!(123, exp -2); // 1.23
assert_eq!("1.23", exp_decimal.to_string());

// Using radix and exp
let radix_exp_decimal = dec!(10, radix 2, exp 3); // (1*2^1 + 0*2^0) * 10^3 = 2 * 1000 = 2000
assert_eq!("2000", radix_exp_decimal.to_string());
```

This crate simplifies the creation of `Decimal` instances, especially for constants and literal values defined directly
in code, by handling the conversion at compile time.

```

---

# `rust_decimal` Crate Summary for LLM Context

## 1. Crate Overview

The `rust_decimal` crate provides a Decimal number implementation in pure Rust, designed for financial calculations and scenarios requiring high precision with significant integral and fractional digits, without round-off errors.

* **Representation**: `Decimal` values are stored as a 128-bit fixed-precision number. This consists of:
    * A 96-bit unsigned integer mantissa.
    * A 1-bit sign.
    * An 8-bit scaling factor (exponent `e`), indicating the power of 10 to divide the mantissa by. The scale can range from 0 to 28 inclusive.
* **Form**: Values are of the form `m / 10^e`.
* **Trailing Zeros**: The representation preserves trailing zeros, which might be visible in string conversions. These can be managed using methods like `normalize()` or `round_dp()`.
* **Minimum Rust Version**: `1.67.1` (as of `rust_decimal 1.37.1`).

## 2. Core Type: `Decimal`

The primary type provided by the crate.

```rust
pub struct Decimal { /* private fields */ }
````

It represents a finite set of values of the form `m / 10^e`, where `m` is an integer such that `-2^96 < m < 2^96`, and `e` is an integer between 0 and 28 inclusive.

### 2.1. Associated Constants

  * `Decimal::MAX`: The largest representable value (`79_228_162_514_264_337_593_543_950_335`).
  * `Decimal::MIN`: The smallest representable value (`-79_228_162_514_264_337_593_543_950_335`).
  * `Decimal::ZERO`: `0`.
  * `Decimal::ONE`: `1`.
  * `Decimal::NEGATIVE_ONE`: `-1`.
  * `Decimal::TWO`: `2`.
  * `Decimal::TEN`: `10`.
  * `Decimal::ONE_HUNDRED`: `100`.
  * `Decimal::ONE_THOUSAND`: `1000`.
  * `Decimal::MAX_SCALE`: `28u32` (maximum supported scaling factor).

### 2.2. Creation

`Decimal` instances can be created in various ways:

  * **Using the `dec!` macro (requires `macros` feature):**
    ```rust
    // Ensure `macros` feature is enabled or use `rust_decimal_macros::dec`
    use rust_decimal::dec;
    let d = dec!(1.23); // efficient, compile-time parsing
    let d_sci = dec!(-5.67e-3);
    ```
  * **From integer and scale:**
      * `Decimal::new(num: i64, scale: u32) -> Decimal`: Panics if scale \> `MAX_SCALE`.
        ```rust
        let d = Decimal::new(202, 2); // Represents 2.02
        ```
      * `Decimal::try_new(num: i64, scale: u32) -> Result<Decimal>`: Checked version of `new`.
      * `Decimal::from_i128_with_scale(num: i128, scale: u32) -> Decimal`: For larger mantissas. Panics if scale \> `MAX_SCALE` or `num` exceeds 96 bits.
        ```rust
        let d = Decimal::from_i128_with_scale(5_897_932_384_626_433_832, 2); // 58979323846264338.32
        ```
      * `Decimal::try_from_i128_with_scale(num: i128, scale: u32) -> Result<Decimal>`: Checked version.
  * **From parts:**
      * `Decimal::from_parts(lo: u32, mid: u32, hi: u32, negative: bool, scale: u32) -> Decimal`: Constructs from raw 96-bit integer parts, sign, and scale.
        ```rust
        let pi_approx = Decimal::from_parts(1102470952, 185874565, 1703060790, false, 28); // 3.1415926535897932384626433832
        ```
  * **From strings:**
      * `Decimal::from_str("2.02")?` (via `std::str::FromStr` trait):
      * `Decimal::from_scientific("9.7e-7")? -> Result<Decimal, Error>`:
        ```rust
        let sci = Decimal::from_scientific("9.7e-7")?; // 0.00000097
        ```
      * `Decimal::from_str_radix(s: &str, radix: u32)? -> Result<Decimal, Error>`:
        ```rust
        let hex_val = Decimal::from_str_radix("A", 16)?; // 10
        ```
      * `Decimal::from_str_exact(s: &str)? -> Result<Decimal, Error>`: Parses string; errors on underflow if scale is too large for representation.
        ```rust
        assert_eq!(Decimal::from_str_exact("0.00000_00000_00000_00000_00000_001")?.to_string(), "0.0000000000000000000000000001");
        assert!(Decimal::from_str_exact("0.00000_00000_00000_00000_00000_0001").is_err()); // Underflow
        ```
  * **From primitive types (via `From<T>` and `FromPrimitive` traits):**
    ```rust
    use rust_decimal::prelude::*;
    let d_from_int: Decimal = 3_i32.into();
    let d_from_prim: Option<Decimal> = Decimal::from_i32(123);
    ```
  * **From floats (via `FromPrimitive` or specific `TryFrom` / `_retain` methods):**
      * `Decimal::try_from(0.1_f32)?` or `Decimal::from_f32(0.1_f32).unwrap()`: Standard conversion, may lose precision beyond float guarantees.
      * `Decimal::from_f32_retain(0.1_f32).unwrap()`: Retains all bits of the float, potentially including approximation errors.
        ```rust
        // Standard conversion
        assert_eq!(Decimal::from_f32(0.1_f32).unwrap().to_string(), "0.1");
        // Retain full float precision (including potential inaccuracies)
        assert_eq!(Decimal::from_f32_retain(0.1_f32).unwrap().to_string(), "0.100000001490116119384765625");
        ```
      * Similar methods exist for `f64`.

### 2.3. Key Methods & Properties

  * **Accessors**:
      * `scale() -> u32`: Returns the exponent `e`.
      * `mantissa() -> i128`: Returns the 96-bit integer part `m` as an `i128`.
      * `is_sign_negative() -> bool`, `is_sign_positive() -> bool`.
      * `is_zero() -> bool`.
      * `is_integer() -> bool`: Checks if the fractional part is zero.
  * **Manipulation**:
      * `set_sign_positive(is_positive: bool)`, `set_sign_negative(is_negative: bool)`: Modifies the sign.
      * `set_scale(scale: u32) -> Result<()>`: Changes the scale, potentially causing overflow/underflow.
      * `rescale(scale: u32)`: In-place; attempts to change scale without altering value. Rounds if new scale is smaller (using `MidpointAwayFromZero`). If new scale is too large, it uses max possible scale for the mantissa.
      * `normalize() -> Decimal`, `normalize_assign()`: Removes trailing zeros from the fractional part and converts -0 to 0.
        ```rust
        let d = dec!(3.100);
        assert_eq!(d.normalize().to_string(), "3.1");
        ```
  * **Rounding and Truncation**:
      * `round() -> Decimal`: Rounds to the nearest integer using Bankers Rounding (e.g., 6.5 -\> 6, 7.5 -\> 8).
      * `round_dp(dp: u32) -> Decimal`: Rounds to `dp` decimal places using Bankers Rounding.
      * `round_dp_with_strategy(dp: u32, strategy: RoundingStrategy) -> Decimal`: Rounds to `dp` decimal places using a specified `RoundingStrategy`.
      * `round_sf(digits: u32) -> Option<Decimal>`: Rounds to `digits` significant figures using `MidpointNearestEven`. Returns `None` if result is unrepresentable.
      * `round_sf_with_strategy(digits: u32, strategy: RoundingStrategy) -> Option<Decimal>`: Rounds to `digits` significant figures with a specific strategy.
      * `trunc() -> Decimal`: Returns the integer part, removing the fractional part without rounding.
      * `trunc_with_scale(scale: u32) -> Decimal`: Truncates to the specified `scale`.
  * **Mathematical Operations**:
      * `abs() -> Decimal`.
      * `floor() -> Decimal`.
      * `ceil() -> Decimal`.
      * `fract() -> Decimal`: Returns the fractional part.
      * `max(other: Decimal) -> Decimal`, `min(other: Decimal) -> Decimal`.
      * **Checked Arithmetic**: `checked_add(other)`, `checked_sub(other)`, `checked_mul(other)`, `checked_div(other)`, `checked_rem(other)` all return `Option<Decimal>`.
      * **Saturating Arithmetic**: `saturating_add(other)`, `saturating_sub(other)`, `saturating_mul(other)`.
      * With `maths` feature: `pow()`, `ln()`, `log10()`, `exp()`, etc. (see `MathematicalOps` trait).
  * **Serialization (Raw Bytes)**:
      * `serialize() -> [u8; 16]`: Serializes to a 16-byte array (flags, lo, mid, hi parts).
      * `deserialize(bytes: [u8; 16]) -> Decimal`.
  * **Debugging**:
      * `unpack() -> UnpackedDecimal`: Returns an internal representation (fields: `negative`, `scale`, `hi`, `mid`, `lo`). Primarily for library maintainers.

### 2.4. Trait Implementations

`Decimal` implements many standard traits:

  * **Arithmetic**: `Add`, `Sub`, `Mul`, `Div`, `Rem` (and `*-Assign` versions for in-place operations), `Neg`. These are implemented for `Decimal` and `&Decimal` combinations.
  * **Comparison**: `PartialEq`, `Eq`, `PartialOrd`, `Ord`. Comparison considers the numerical value, so `dec!(1.2)` equals `dec!(1.20)`.
  * **Hashing**: `Hash`. Hashing is based on the normalized value.
  * **Formatting**: `Display` (standard decimal string), `Debug` (same as `Display`), `LowerExp`, `UpperExp` (scientific notation).
  * **Conversion**:
      * `From<integer_type>` for all Rust primitive integer types (`i8` through `i128`, `u8` through `u128`, `isize`, `usize`).
      * `TryFrom<Decimal>` for primitive numeric types (e.g., `i32::try_from(my_decimal)`).
      * `FromStr` for parsing from string (e.g., `"123.45".parse::<Decimal>()`).
      * `TryFrom<&str>` (equivalent to `FromStr`).
      * `TryFrom<f32>`, `TryFrom<f64>`.
  * **Numeric Traits (from `num-traits` via prelude or direct import)**:
      * `Zero`: `Decimal::zero()`, `is_zero()`.
      * `One`: `Decimal::one()`, `is_one()`.
      * `Num`: Includes `from_str_radix()`.
      * `Signed`: `abs()`, `abs_sub()`, `signum()`, `is_positive()`, `is_negative()`.
      * `FromPrimitive`: `from_i64()`, `from_u64()`, etc. (e.g., `Decimal::from_i32(10)`).
      * `ToPrimitive`: `to_i64()`, `to_u64()`, etc. (e.g., `my_decimal.to_i32()`).
      * Checked operations: `CheckedAdd`, `CheckedSub`, `CheckedMul`, `CheckedDiv`, `CheckedRem`.
  * **Iteration**: `Sum`, `Product`.
  * **Other**: `Default` (returns `Decimal::ZERO`), `Copy`, `Clone`, `Send`, `Sync`.
  * **Serde**: (If features enabled) `Serialize`, `Deserialize`.

## 3\. Error Handling: `Error` Enum

The crate's error type.

```rust
pub enum Error {
    ErrorString(String), // Legacy, generic error message
    ExceedsMaximumPossibleValue, // Value > Decimal::MAX
    LessThanMinimumPossibleValue, // Value < Decimal::MIN
    Underflow, // More fractional digits than can be represented
    ScaleExceedsMaximumPrecision(u32), // Scale > MAX_SCALE (28)
    ConversionTo(String), // Failure to convert to/from Decimal and another type (e.g. Decimal::MAX to i32)
}
```

Implements `std::error::Error`, `Display`, `Debug`, `Clone`, `PartialEq`.

## 4\. Rounding: `RoundingStrategy` Enum

Specifies how rounding should be performed, typically used with `round_dp_with_strategy`.

```rust
pub enum RoundingStrategy {
    MidpointNearestEven,    // Default for .round(), .round_dp(). AKA "Banker's Rounding". (e.g., 6.5 -> 6, 7.5 -> 8)
    MidpointAwayFromZero,   // (e.g., 6.5 -> 7, -6.5 -> -7)
    MidpointTowardZero,     // (e.g., 6.5 -> 6, -6.5 -> -6)
    ToZero,                 // Truncates toward zero (e.g., -6.8 -> -6, 6.8 -> 6)
    AwayFromZero,           // (e.g., -6.8 -> -7, 6.8 -> 7)
    ToNegativeInfinity,     // Floor (e.g., 6.8 -> 6, -6.8 -> -7)
    ToPositiveInfinity,     // Ceiling (e.g., 6.8 -> 7, -6.8 -> -6)

    // Deprecated aliases:
    // BankersRounding (use MidpointNearestEven)
    // RoundHalfUp (use MidpointAwayFromZero)
    // RoundHalfDown (use MidpointTowardZero)
    // RoundDown (use ToZero)
    // RoundUp (use AwayFromZero)
}
```

## 5\. `Result<T>` Type Alias

A shortcut for `core::result::Result<T, rust_decimal::Error>`.

## 6\. Prelude Module

`use rust_decimal::prelude::*;` imports common types and traits:
`Decimal`, `RoundingStrategy`, and numeric traits like `Zero`, `One`, `FromPrimitive`, `ToPrimitive`, `Signed`, `FromStr`. If the `macros` feature is enabled, `dec!` is also part of the prelude.

## 7\. Key Crate Features

  * **`macros`**: Enables the `dec!(...)` macro for compile-time `Decimal` creation.
    ```rust
    use rust_decimal::dec;
    let cost = dec!(19.99);
    ```
  * **`maths`**: Enables extended mathematical functions like `pow()`, `ln()`, `exp()`, `sqrt()`, etc., available via the `MathematicalOps` trait.
      * `maths-nopanic`: Modifies `ln()` and `log10()` to return `0` on invalid input instead of panicking (use `checked_ln`/`checked_log10` for error handling).
  * **Serialization (Serde)**:
      * Default: Serializes as a string (e.g., `"123.45"`).
      * `serde-float`: Serializes as a JSON float number (e.g., `123.45`).
      * `serde-str`: Ensures string serialization/deserialization, useful for formats like `bincode`.
      * `serde-arbitrary-precision`: Uses `serde_json`'s `arbitrary_precision` feature for deserializing floats without precision loss. Serializes as float by default.
      * `serde-with-float`, `serde-with-str`, `serde-with-arbitrary-precision`: Provide modules for `#[serde(with = "...")]` or `serialize_with`/`deserialize_with` for field-level control.
  * **Database Integration**:
      * `db-postgres`: For `postgres` crate integration (NUMERIC type).
      * `db-tokio-postgres`: For `tokio-postgres` async integration.
      * `db-diesel-postgres`, `db-diesel-mysql`: For Diesel ORM integration.
  * **`std`**: Enabled by default. For `no_std` environments, compile with `--no-default-features`.
  * **`c-repr`**: Forces `Decimal` to use `#[repr(C)]` for C ABI compatibility (128-bit aligned).
  * **Other**: `borsh` (Borsh serialization), `legacy-ops` (deprecated, for old arithmetic algorithm), `ndarray` (ndarray operations), `proptest` (proptest strategies), `rand` (random Decimal generation), `rkyv` (rkyv serialization), `rocket-traits` (Rocket form support), `rust-fuzz` (fuzzing support).

## 8\. `str` Module

Contains string parsing utilities.

  * `fn overflow_128(val: u128) -> bool`: Likely checks if a u128 value would overflow the 96-bit mantissa of `Decimal`.

This summary provides a foundational context for an LLM to understand and generate code using the `rust_decimal` crate.
