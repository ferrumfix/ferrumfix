use std::convert::{From, TryInto};
use std::fmt;
use std::fmt::Write;
use std::io;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DataType {
    Char,
    Boolean,
    Float,
    Amt,
    Price,
    PriceOffset,
    Qty,
    Percentage,
    Int,
    DayOfMonth,
    Length,
    NumInGroup,
    SeqNum,
    TagNum,
    String,
    Data,
    MonthYear,
    MultipleCharValue,
    Currency,
    Exchange,
    LocalMktDate,
    MultipleStringValue,
    UtcDate,
    UtcTimeOnly,
    UtcTimestamp,
    XmlData,
    Country,
}

impl DataType {

    /// Returns the name of `self`, character by character identical to the name
    /// that appears in the official guidelines. **Generally** primitive datatypes
    /// will use `snake_case` and non-primitive ones will have `PascalCase`, but
    /// that's not true for every [`DataType`](DataType).
    ///
    /// # Examples
    ///
    /// ```
    /// use fasters::datatypes::DataType;
    ///
    /// assert_eq!(DataType::Qty.name(), "Qty");
    /// assert_eq!(DataType::Float.name(), "float");
    /// assert_eq!(DataType::String.name(), "String");
    /// ```
    pub fn name(&self) -> &'static str {
        // 1. Most primitive data types have `snake_case` names.
        // 2. Most derivative data types have `PascalCase` names.
        // 3. `data` and `String` ruin the party and mess it up.
        //
        // Why, you ask? Because nothing makes sense in FIX land.
        match self {
            DataType::Int => "int",
            DataType::Length => "Length",
            DataType::Char => "char",
            DataType::Boolean => "Boolean",
            DataType::Float => "float",
            DataType::Amt => "Amt",
            DataType::Price => "Price",
            DataType::PriceOffset => "PriceOffset",
            DataType::Qty => "Qty",
            DataType::Percentage => "Percentage",
            DataType::DayOfMonth => "DayOfMonth",
            DataType::NumInGroup => "NumInGroup",
            DataType::SeqNum => "SeqNum",
            DataType::TagNum => "TagNum",
            DataType::String => "String",
            DataType::Data => "data",
            DataType::MonthYear => "MonthYear",
            DataType::Currency => "Currency",
            DataType::Exchange => "Exchange",
            DataType::LocalMktDate => "LocalMktDate",
            DataType::MultipleStringValue => "MultipleStringValue",
            DataType::UtcTimeOnly => "UTCTimeOnly",
            DataType::UtcTimestamp => "UTCTimestamp",
            DataType::UtcDate => "UTCDate",
            DataType::Country => "Country",
            DataType::MultipleCharValue => "MultipleCharValue",
            DataType::XmlData => "XMLData",
        }
    }

    pub fn decode(&self, data: &[u8]) -> Option<DataTypeValue> {
        match self {
            DataType::Int => {
                let mut n: i64 = 0;
                for byte in data {
                    if *byte >= '0' as u8 && *byte <= '9' as u8 {
                        let digit = byte - '0' as u8;
                        n = n * 10 + digit as i64;
                    } else if *byte == '-' as u8 {
                        n *= -1;
                    } else if *byte != '+' as u8 {
                        return None;
                    }
                }
                Some(DataTypeValue::Int(Int(n as i32)))
            }
            DataType::Char => Some(DataTypeValue::Char(Char(data[0] as char))),
            DataType::Boolean => Some(DataTypeValue::Boolean(Boolean(data[0] == 'Y' as u8))),
            DataType::Country => {
                if data.len() != 2 {
                    None
                } else {
                    Some(DataTypeValue::Country(Country(data.try_into().unwrap())))
                }
            }
            DataType::Exchange => Some(DataTypeValue::Exchange(Exchange(
                std::str::from_utf8(data).unwrap().to_string(),
            ))),
            DataType::DayOfMonth => {
                let n = str::parse::<u32>(std::str::from_utf8(data).unwrap()).unwrap();
                Some(DataTypeValue::DayOfMonth(DayOfMonth(n)))
            }
            DataType::SeqNum => {
                let mut n: u64 = 0;
                for byte in data.iter() {
                    n = n * 10 + (byte - '0' as u8) as u64;
                }
                Some(DataTypeValue::SeqNum(SeqNum(n)))
            }
            DataType::String => Some(DataTypeValue::String(String(
                std::str::from_utf8(data).unwrap().to_string(),
            ))),
            DataType::XmlData => Some(DataTypeValue::XmlData(XmlData(
                std::str::from_utf8(data).unwrap().to_string(),
            ))),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataTypeValue {
    Char(Char),
    Boolean(Boolean),
    Float(Float),
    Amt(Amt),
    Price(Price),
    PriceOffset(PriceOffset),
    Qty(Qty),
    Percentage(Percentage),
    Int(Int),
    DayOfMonth(DayOfMonth),
    Length(Length),
    NumInGroup(NumInGroup),
    SeqNum(SeqNum),
    TagNum(TagNum),
    String(String),
    MultipleCharValue(MultipleCharValue),
    Data(()),
    MonthYear(MonthYear),
    Currency(Currency),
    Exchange(Exchange),
    LocalMktDate(LocalMktDate),
    MultipleStringValue(MultipleStringValue),
    UtcDate(UtcDate),
    UtcTimeOnly(UtcTimeOnly),
    UtcTimestamp(UtcTimestamp),
    XmlData(XmlData),
    Country(Country),
}

impl fmt::Display for DataTypeValue {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

pub trait DataTypeConvert: From<&'static [u8]> {}

pub trait PrimitiveDataType {}

pub trait DerivedDataType {
    type Primitive: PrimitiveDataType;
}

/// Sequence of digits without commas or decimals and optional sign character
/// (ASCII characters "-" and "0" - "9" ). The sign character utilizes one byte
/// (i.e. positive int is "99999" while negative int is "-99999").
///
/// Note that int values may contain leading zeros (e.g. "00023" = "23").
///
/// Examples: 723 in field 21 would be mapped int as |21=723|, -723 in field 12
/// would be mapped int as |12=-723|.
#[derive(Debug, Clone, PartialEq)]
pub struct Int(i32);

impl PrimitiveDataType for Int {}

impl fmt::Display for Int {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// Int field (see definition of "int" above) representing the length in bytes.
/// Value must be positive.
#[derive(Debug, Clone, PartialEq)]
pub struct Length(u32);

impl fmt::Display for Length {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl DerivedDataType for Length {
    type Primitive = Int;
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayOfMonth(u32);

impl fmt::Display for DayOfMonth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl DerivedDataType for DayOfMonth {
    type Primitive = Int;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MonthYear {
    year: u16,
    month: u8,
    day_or_week: u8,
}

impl MonthYear {
    const WEEK_CUTOFF: u8 = 35;
}

impl fmt::Display for MonthYear {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.year.fmt(f)?;
        self.month.fmt(f)?;
        if self.day_or_week > Self::WEEK_CUTOFF {
            f.write_char('w')?;
            let week = self.day_or_week - Self::WEEK_CUTOFF;
            f.write_char(std::char::from_digit(week as u32, 10).unwrap())?;
        } else {
            self.day_or_week.fmt(f)?;
        }
        Ok(())
    }
}

impl DerivedDataType for MonthYear {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Currency([u8; 3]);

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = std::str::from_utf8(&self.0[..]).unwrap();
        s.fmt(f)
    }
}

impl DerivedDataType for Currency {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Exchange(std::string::String);

impl DerivedDataType for Exchange {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct LocalMktDate(u32);

impl DerivedDataType for LocalMktDate {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipleStringValue(Vec<String>);

impl DerivedDataType for MultipleStringValue {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct UtcDate(u32);

impl DerivedDataType for UtcDate {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct UtcTimeOnly(u32);

impl DerivedDataType for UtcTimeOnly {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct UtcTimestamp(i64);

impl DerivedDataType for UtcTimestamp {
    type Primitive = String;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Country([u8; 2]);

#[derive(Debug, Clone, PartialEq)]
pub struct Language([u8; 3]);

impl DerivedDataType for Language {
    type Primitive = String;
}

impl DerivedDataType for Country {
    type Primitive = String;
}

/// Int field (see definition of "int" above) representing the number of entries
/// in a repeating group. Value must be positive.
#[derive(Debug, Clone, PartialEq)]
pub struct NumInGroup(u32);

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
#[derive(Debug, Clone, PartialEq)]
pub struct Float(f32);

impl From<f32> for Float {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl PrimitiveDataType for Float {}

/// Float field (see definition of "float" above) capable of storing either a
/// whole number (no decimal places) of "shares" (securities denominated in whole
/// units) or a decimal value containing decimal places for non-share quantity
/// asset classes (securities denominated in fractional units).
#[derive(Debug, Clone, PartialEq)]
pub struct Qty(Float);

/// Float field (see definition of "float" above) representing a price. Note the
/// number of decimal places may vary. For certain asset classes prices may be
/// negative values. For example, options strategies can be negative under
/// certain market conditions. Refer to Volume 7: FIX Usage by Product (460) for
/// asset classes that support negative price values.
#[derive(Debug, Clone, PartialEq)]
pub struct Price(Float);

/// Float field (see definition of "float" above) representing a price offset,
/// which can be mathematically added to a "Price". Note the number of decimal
/// places may vary and some fields such as LastForwardPoints (195) may be
/// negative.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceOffset(Price);

/// Float field (see definition of "float" above) typically representing a Price
/// (44) times a Qty.
#[derive(Debug, Clone, PartialEq)]
pub struct Amt(Float);

/// Float field (see definition of "float" above) representing a percentage (e.g.
/// .05 represents 5% and .9525 represents 95.25%). Note the number of decimal
/// places may vary.
#[derive(Debug, Clone, PartialEq)]
pub struct Percentage(i64);

impl DerivedDataType for Percentage {
    type Primitive = Float;
}

/// Single character value, can include any alphanumeric character or punctuation
/// except the delimiter. All char fields are case sensitive (i.e. m != M).
#[derive(Debug, Clone, PartialEq)]
pub struct Char(char);

impl From<char> for Char {
    fn from(c: char) -> Self {
        Self(c)
    }
}

impl PrimitiveDataType for Char {}

/// Char field (see definition of "char" above) containing one of two values: 'Y'
/// = True/Yes, 'N' = False/No.
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean(bool);

impl DerivedDataType for Boolean {
    type Primitive = Char;
}

#[derive(Debug, Clone, PartialEq)]
pub struct XmlData(std::string::String);

/// Alpha-numeric free format strings, can include any character or punctuation
/// except the delimiter. All char fields are case sensitive (i.e. morstatt !=
/// Morstatt).
#[derive(Debug, Clone, PartialEq)]
pub struct String(std::string::String);

impl PrimitiveDataType for String {}

/// Int field (see definition of "int" above) representing a message sequence
/// number. Value must be positive.
#[derive(Debug, Clone, PartialEq)]
pub struct SeqNum(u64);

impl DerivedDataType for SeqNum {
    type Primitive = Int;
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultipleCharValue(String);

/// Value for the field `MsgType (35)`.
#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagNum(u16);

impl TagNum {
    pub fn write(&self, writer: &mut impl io::Write) -> io::Result<()> {
        let bytes = self.0.to_be_bytes();
        writer.write(&bytes[..])?;
        Ok(())
    }
}

impl fmt::Display for TagNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<u16> for TagNum {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<&[u8]> for TagNum {
    fn from(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() >= std::mem::size_of::<u16>());
        let value = ((bytes[0] as u16) << 8) + bytes[1] as u16;
        Self(value)
    }
}
