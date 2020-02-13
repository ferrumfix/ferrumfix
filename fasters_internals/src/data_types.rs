use std::convert::From;

pub trait DataType: From<&'static [u8]> {}

/// Sequence of digits without commas or decimals and optional sign character
/// (ASCII characters "-" and "0" - "9" ). The sign character utilizes one byte
/// (i.e. positive int is "99999" while negative int is "-99999").
///
/// Note that int values may contain leading zeros (e.g. "00023" = "23").
///
/// Examples: 723 in field 21 would be mapped int as |21=723|, -723 in field 12
/// would be mapped int as |12=-723|.
struct Int(i32);

/// Int field (see definition of "int" above) representing the length in bytes.
/// Value must be positive.
type Length = Int;

/// Int field (see definition of "int" above) representing the number of entries
/// in a repeating group. Value must be positive.
type NumInGroup = Int;

/// Int field (see definition of "int" above) representing a message sequence
/// number. Value must be positive.
type SeqNum = Int;

/// Sequence of digits with optional decimal point and sign character (ASCII
/// characters "-", "0" - "9" and "."); the absence of the decimal point within
/// the string will be interpreted as the float representation of an integer
/// value. All float fields must accommodate up to fifteen significant digits.
/// The number of decimal places used should be a factor of business/market needs
/// and mutual agreement between counterparties. Note that float values may
/// contain leading zeros (e.g. "00023.23" = "23.23") and may contain or omit
/// trailing zeros after the decimal point (e.g. "23.0" = "23.0000" = "23" =
/// "23.").
///
/// Note that fields which are derived from float may contain negative values
/// unless explicitly specified otherwise.
struct Float(f32);

/// Float field (see definition of "float" above) capable of storing either a
/// whole number (no decimal places) of "shares" (securities denominated in whole
/// units) or a decimal value containing decimal places for non-share quantity
/// asset classes (securities denominated in fractional units).
type Qty = Float;

/// Float field (see definition of "float" above) representing a price. Note the
/// number of decimal places may vary. For certain asset classes prices may be
/// negative values. For example, options strategies can be negative under
/// certain market conditions. Refer to Volume 7: FIX Usage by Product (460) for
/// asset classes that support negative price values.
type Price = Float;

/// Float field (see definition of "float" above) representing a price offset,
/// which can be mathematically added to a "Price". Note the number of decimal
/// places may vary and some fields such as LastForwardPoints (195) may be
/// negative.
type PriceOffset = Float;

/// Float field (see definition of "float" above) typically representing a Price
/// (44) times a Qty.
type Amt = Float;

/// Float field (see definition of "float" above) representing a percentage (e.g.
/// .05 represents 5% and .9525 represents 95.25%). Note the number of decimal
/// places may vary.
type Percentage = Float;

/// Single character value, can include any alphanumeric character or punctuation
/// except the delimiter. All char fields are case sensitive (i.e. m != M).
struct Char(char);

/// Char field (see definition of "char" above) containing one of two values: 'Y'
/// = True/Yes, 'N' = False/No.
type Boolean = Char;

/// Alpha-numeric free format strings, can include any character or punctuation
/// except the delimiter. All char fields are case sensitive (i.e. morstatt !=
/// Morstatt).
struct String(std::string::String);
