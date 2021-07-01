//! Decimal floating-point arithmetic for FAST.

use std::cmp::{self, Ordering};
use std::fmt;
use std::ops;

/// [`Decimal`] is a fixed 96-bit representation of a decimal number. It can be
/// used to represent all values of the form `m * 10**e`, with `-2**63 <= m <=
/// 2**63 - 1` and `-63 <= e <= 63`. [`Decimal`] can thus represent up to 63
/// decimal digits and hold values up to `(2**63 - 1) * 10**63`.
///
/// This module implements most of the API available for both primitive integer
/// types in Rust and [`rust_decimal::Decimal`](https://docs.rs/rust_decimal/1.10.1/rust_decimal/struct.Decimal.html), from which this module borrows
/// many design choices. Most examples available for the above mentioned modules
/// will work seamlessly even with this [`Decimal`].
///
/// Please note that [`Decimal`] is *not* a standardized floating-point
/// format; or
/// rather, it is a poorly specified and FAST-specific format. While basic
/// operations are available directly on [`Decimal`], you should
/// consider
/// converting to more battle-tested formats such as `decimal128` (see the
/// Wikipedia
/// [article](https://en.wikipedia.org/wiki/Decimal128_floating-point_format)).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Decimal {
    // In theory, 7 bits ought to suffice (we only need -63/+63), but reserving
    // more space allows for easier calculations and more relaxed overflow
    // detection.
    exp: i32,
    mantissa: i64,
}

#[derive(Debug)]
pub enum Error {
    //InvalidScale,
}

impl Decimal {
    /// The greatest value that can be represented by a [`Decimal`].
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// assert!(Decimal::MAX > Decimal::ZERO);
    /// ```
    pub const MAX: Decimal = Decimal {
        exp: 63,
        mantissa: i64::MAX,
    };

    /// The smallest value that can be represented by a [`Decimal`].
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// assert!(Decimal::MIN < Decimal::ZERO);
    /// ```
    pub const MIN: Decimal = Decimal {
        exp: 63,
        mantissa: i64::MIN,
    };

    /// The unit value of [`Decimal`]. It is the neutral element of
    /// multiplication.
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// assert_eq!(Decimal::MAX * Decimal::ONE, Decimal::MAX);
    /// assert_eq!(Decimal::MIN * Decimal::ONE, Decimal::MIN);
    ///
    /// let pi = Decimal::new(314, -2);
    /// assert_eq!(pi * Decimal::ONE, pi);
    /// ```
    pub const ONE: Decimal = Decimal {
        exp: 0,
        mantissa: 1,
    };

    /// The negative unit value.
    pub const NEG_ONE: Decimal = Decimal {
        exp: 0,
        mantissa: -1,
    };

    /// The zero value. It is the neutral element of addition.
    pub const ZERO: Decimal = Decimal {
        exp: 0,
        mantissa: 0,
    };

    /// The [machine epsilon](https://en.wikipedia.org/wiki/Machine_epsilon) of
    /// [`Decimal`].
    pub const MIN_POSITIVE: Decimal = Decimal {
        exp: -63,
        mantissa: 1,
    };

    /// Returns a [`Decimal`] with a 64 bit *m* representation and corresponding
    /// *e* scale.
    ///
    /// # Arguments
    ///
    /// * `exp` - An `i32` that represents the *e* portion of the decimal number
    /// * `mantissa` - An `i64` that represents the *m* portion of the decimal number.
    ///
    /// The value of *e* will be rounded towards 0 to stay within the legal
    /// interval, if necessary.
    ///
    /// # Example
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let pi = Decimal::new(3141, -3);
    /// assert_eq!(pi.to_string(), "3.141");
    /// ```
    pub fn new(mantissa: i64, exp: i32) -> Self {
        Self::new_unchecked(mantissa, exp.max(-16).min(16))
    }

    /// Returns a [`Decimal`] with a 64 bit *m* representation and corresponding
    /// *e* scale. *e* is assumed to already be in the valid interval.
    ///
    /// # Arguments
    ///
    /// * `exp` - An `i8` that represents the *e* portion of the decimal number
    /// * `mantissa` - An `i64` that represents the *m* portion of the decimal number.
    ///
    /// # Example
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let pi = Decimal::new(3141, -3);
    /// assert_eq!(pi.to_string(), "3.141");
    /// ```
    pub fn new_unchecked(mantissa: i64, exp: i32) -> Self {
        Self { exp, mantissa }.normalize()
    }

    fn exp_is_maxed_out(&self) -> bool {
        self.exp() >= 16 || self.exp() <= -16
    }

    fn normalize(&self) -> Self {
        let mut me = *self;
        while me.mantissa % 10 == 0 && !me.exp_is_maxed_out() {
            me.mantissa /= 10;
            me.exp += 1;
        }
        me
    }

    /// Returns the exponent of the decimal number, also known as exponent or
    /// *e*.
    ///
    /// # Example
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(1234, 3);
    /// assert_eq!(num.exp(), 3i32);
    /// ```
    pub const fn exp(&self) -> i32 {
        self.exp
    }

    /// Returns the mantissa of `self`, also known as significand or
    /// *m*.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(314159, -5);
    /// assert_eq!(num.mantissa(), 314159);
    /// ```
    pub const fn mantissa(&self) -> i64 {
        self.mantissa
    }

    /// Returns a [`Decimal`] number representing the sign of `self`:
    ///
    /// - 0 if it is zero.
    /// - 1 if it is positive.
    /// - -1 if it is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(314159, -5);
    /// assert_eq!(num.signum(), Decimal::ONE);
    /// assert_eq!((-num).signum(), Decimal::NEG_ONE);
    /// ```
    pub fn signum(&self) -> Self {
        const SIGNUMS: &[Decimal; 3] = &[Decimal::NEG_ONE, Decimal::ZERO, Decimal::ONE];
        let index = (self.mantissa().signum() + 1) as usize;
        SIGNUMS[index]
    }

    /// Computes the absolute value of `self`.
    ///
    /// # Overflow behavior
    ///
    /// Panics when the mantissa *m* is at its minimum value.
    /// The absolute value of [`Decimal::MIN`] cannot be represented as a
    /// [`Decimal`], and attempting to calculate it will cause an
    /// overflow. This means that code in debug mode will trigger a panic on this
    /// case and optimized code will return [`Decimal::MIN`] without a
    /// panic.
    ///
    /// # Example
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(-1337, 0);
    /// assert_eq!(num.abs().to_string(), "1337");
    /// ```
    pub fn abs(&self) -> Self {
        debug_assert_ne!(self.mantissa(), i64::MIN);
        Self {
            exp: self.exp(),
            mantissa: self.mantissa().abs(),
        }
    }

    pub fn checked_abs(&self) -> Option<Self> {
        let mantissa = self.mantissa().checked_abs()?;
        Some(Self {
            exp: self.exp(),
            mantissa,
        })
    }

    /// Checked addition. Computes `self + other`, returning `None` if overflow
    /// occurred.
    pub fn checked_add(&self, other: Decimal) -> Option<Self> {
        let exp_diff = self.exp() - other.exp();
        let mantissa_1 = self.mantissa();
        let mantissa_2 = other
            .mantissa()
            // FIXME
            .checked_mul(10i64.checked_pow(exp_diff as u32)?)?;
        Some(Self {
            exp: self.exp(),
            mantissa: mantissa_1.checked_add(mantissa_2)?,
        })
    }

    /// Checked subtraction. Computes `self - other`, returning `None` if overflow
    /// occurred.
    pub fn checked_sub(&self, other: Decimal) -> Option<Self> {
        let neg_other = other.checked_neg()?;
        self.checked_add(neg_other)
    }

    /// Checked multiplication. Computes `self * other`, returning `None` if overflow
    /// occurred.
    pub fn checked_mul(&self, other: Decimal) -> Option<Self> {
        let exp = self.exp() + other.exp();
        let mantissa = self.mantissa().checked_mul(other.mantissa())?;
        Some(Self { exp, mantissa })
    }

    /// Checked division. Computes `self / other`, returning `None` if `other ==
    /// 0.0` or the division results in overflow.
    pub fn checked_div(self, _other: Decimal) -> Option<Self> {
        unimplemented!()
    }

    pub fn checked_neg(self) -> Option<Self> {
        let mantissa = self.mantissa().checked_neg()?;
        let exp = self.exp();
        Some(Self { exp, mantissa })
    }

    /// Returns `true` if and only if `self` is strictly negative, `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(-130, -4);
    /// assert!(num.is_negative());
    ///
    /// assert!(!Decimal::ZERO.is_negative());
    /// assert!(!Decimal::ONE.is_negative());
    /// ```
    pub const fn is_negative(&self) -> bool {
        self.mantissa().is_negative()
    }

    /// Returns `true` if and only if `self` is strictly positive, `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(1, -8);
    /// assert!(num.is_positive());
    ///
    /// assert!(!Decimal::ZERO.is_positive());
    /// assert!(!Decimal::NEG_ONE.is_positive());
    /// ```
    pub const fn is_positive(&self) -> bool {
        self.mantissa().is_positive()
    }

    /// Raises `self` to the power of `exp`, using exponentiation by squaring.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(11, -1);
    /// assert_eq!(num.pow(2), Decimal::new(121, -2));
    /// ```
    pub fn pow(&self, exp: i32) -> Self {
        match exp.signum() {
            0 => Self::ONE,
            1 if exp % 2 == 0 => (*self * *self).pow(exp / 2),
            1 => *self * ((*self * *self).pow(exp / 2)),
            _ => todo!(),
        }
    }

    /// Returns the integer part of the number `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(314, -2);
    /// assert_eq!(num.truncate(), Decimal::new(3, 0));
    /// ```
    pub fn truncate(&self) -> Self {
        let mut me = *self;
        me.mantissa -= me.mantissa() % 10i64.pow(me.exp().abs() as u32);
        me.normalize()
    }

    /// Returns the fractional part of the number `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefast::Decimal;
    ///
    /// let num = Decimal::new(314, -2);
    /// assert_eq!(num.fract(), Decimal::new(14, -2));
    /// ```
    pub fn fract(&self) -> Self {
        let mut me = *self;
        me.mantissa %= 10i64.pow(me.exp().abs() as u32);
        me
    }

    /// Returns the power of 10 of mantissa, i.e. 10<sup>*e*</sup>.
    pub fn pow_of_ten(&self) -> i64 {
        10i64.pow(self.exp().abs() as u32)
    }

    /// Serializes `self` into a bytes array.
    pub fn to_be_bytes(self) -> [u8; 9] {
        let mut bytes = [0u8; 9];
        bytes[0] = (self.exp() + 16) as u8;
        bytes[1..9].clone_from_slice(&self.mantissa().to_be_bytes()[..]);
        bytes
    }

    /// Deserializes `self` from a bytes array.
    pub fn from_be_bytes(mut bytes: [u8; 9]) -> Self {
        let mut mantissa_bytes = [0u8; 8];
        mantissa_bytes.clone_from_slice(&mut bytes[1..]);
        Self {
            exp: (bytes[0] as i32) - 16,
            mantissa: i64::from_be_bytes(mantissa_bytes),
        }
    }
}

impl Default for Decimal {
    /// The [`Default`] value for [`Decimal`] is [`Decimal::ZERO`].
    fn default() -> Self {
        Self::ZERO
    }
}

impl fmt::Display for Decimal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_negative() {
            write!(f, "-")?;
        }
        if self.mantissa().abs() < self.pow_of_ten() {
            write!(f, "0")?;
        } else {
            // First, the transform the mantissa into a string.
            let digits = self.mantissa().abs().to_string();
            // We then calculate the total number of digits that we're supposed
            // to print. Note that this can never become <= 0, due to the if
            // guard.
            let len = digits.chars().count() as i32 + self.exp();
            // We then concatenate the string with 0's in case we don't have
            // enough digits.
            for digit in digits
                .chars()
                .chain(std::iter::repeat('0'))
                .take(len as usize)
            {
                write!(f, "{}", digit)?;
            }
        }
        if self.fract().is_positive() {
            // We just verified we have some decimal digits, so we're gonna need
            // the decimal point.
            write!(f, ".{}", "")?;
            // Here we take the mantissa of the fractional part.
            let digits = self.fract().mantissa().to_string();
            let digits_len = digits.chars().count();
            let len = -self.exp();
            debug_assert!(len > 0);
            for digit in std::iter::repeat('0')
                .take(len as usize - digits_len)
                .chain(digits.chars())
            {
                write!(f, "{}", digit)?;
            }
        }
        Ok(())
    }
}

impl cmp::PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for Decimal {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        // If signs differ, there's no need for any further calculations.
        let cmp_sign = self.mantissa().signum().cmp(&other.mantissa().signum());
        if cmp_sign != Ordering::Equal {
            return cmp_sign;
        }
        let cmp_exp = self.exp().signum().cmp(&other.exp().signum());
        let cmp_mantissa = self.mantissa().cmp(&other.mantissa());
        cmp_exp.then(cmp_mantissa)
    }
}

impl ops::Neg for Decimal {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            exp: self.exp(),
            mantissa: -self.mantissa(),
        }
    }
}

impl ops::Add for Decimal {
    type Output = Self;

    fn add(self, other: Self::Output) -> Self::Output {
        let a = self.normalize();
        let b = other.normalize();
        Decimal {
            exp: a.exp(),
            mantissa: a.mantissa() - b.mantissa(),
        }
        .normalize()
    }
}

impl ops::Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self::Output) -> Self::Output {
        let a = self.normalize();
        let b = other.normalize();
        Decimal {
            exp: a.exp(),
            mantissa: a.mantissa() - b.mantissa(),
        }
        .normalize()
    }
}

impl ops::Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let exp = self.exp() + other.exp();
        let mantissa = self.mantissa() * other.mantissa();
        Decimal { exp, mantissa }
    }
}
