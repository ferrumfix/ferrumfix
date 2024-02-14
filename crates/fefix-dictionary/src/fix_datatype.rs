use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};

/// Sum type for all possible FIX data types ever defined across all FIX
/// application versions.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter, IntoStaticStr)]
#[repr(u8)]
#[non_exhaustive]
pub enum FixDatatype {
    /// Single character value, can include any alphanumeric character or
    /// punctuation except the delimiter. All char fields are case sensitive
    /// (i.e. m != M). The following fields are based on char.
    Char,
    /// char field containing one of two values: 'Y' = True/Yes 'N' = False/No.
    Boolean,
    /// Sequence of digits with optional decimal point and sign character (ASCII
    /// characters "-", "0" - "9" and "."); the absence of the decimal point
    /// within the string will be interpreted as the float representation of an
    /// integer value. All float fields must accommodate up to fifteen
    /// significant digits. The number of decimal places used should be a factor
    /// of business/market needs and mutual agreement between counterparties.
    /// Note that float values may contain leading zeros (e.g. "00023.23" =
    /// "23.23") and may contain or omit trailing zeros after the decimal point
    /// (e.g. "23.0" = "23.0000" = "23" = "23."). Note that fields which are
    /// derived from float may contain negative values unless explicitly
    /// specified otherwise. The following data types are based on float.
    Float,
    /// float field typically representing a Price times a Qty.
    Amt,
    /// float field representing a price. Note the number of decimal places may
    /// vary. For certain asset classes prices may be negative values. For
    /// example, prices for options strategies can be negative under certain
    /// market conditions. Refer to Volume 7: FIX Usage by Product for asset
    /// classes that support negative price values.
    Price,
    /// float field representing a price offset, which can be mathematically
    /// added to a "Price". Note the number of decimal places may vary and some
    /// fields such as LastForwardPoints may be negative.
    PriceOffset,
    /// float field capable of storing either a whole number (no decimal places)
    /// of "shares" (securities denominated in whole units) or a decimal value
    /// containing decimal places for non-share quantity asset classes
    /// (securities denominated in fractional units).
    Qty,
    /// float field representing a percentage (e.g. 0.05 represents 5% and 0.9525
    /// represents 95.25%). Note the number of decimal places may vary.
    Percentage,
    /// Sequence of digits without commas or decimals and optional sign character
    /// (ASCII characters "-" and "0" - "9" ). The sign character utilizes one
    /// byte (i.e. positive int is "99999" while negative int is "-99999"). Note
    /// that int values may contain leading zeros (e.g. "00023" = "23").
    /// Examples: 723 in field 21 would be mapped int as |21=723|. -723 in field
    /// 12 would be mapped int as |12=-723| The following data types are based on
    /// int.
    Int,
    /// int field representing a day during a particular monthy (values 1 to 31).
    DayOfMonth,
    /// int field representing the length in bytes. Value must be positive.
    Length,
    /// int field representing the number of entries in a repeating group. Value
    /// must be positive.
    NumInGroup,
    /// int field representing a message sequence number. Value must be positive.
    SeqNum,
    /// `int` field representing a field's tag number when using FIX "Tag=Value"
    /// syntax. Value must be positive and may not contain leading zeros.
    TagNum,
    /// Alpha-numeric free format strings, can include any character or
    /// punctuation except the delimiter. All String fields are case sensitive
    /// (i.e. morstatt != Morstatt).
    String,
    /// string field containing raw data with no format or content restrictions.
    /// Data fields are always immediately preceded by a length field. The length
    /// field should specify the number of bytes of the value of the data field
    /// (up to but not including the terminating SOH). Caution: the value of one
    /// of these fields may contain the delimiter (SOH) character. Note that the
    /// value specified for this field should be followed by the delimiter (SOH)
    /// character as all fields are terminated with an "SOH".
    Data,
    /// string field representing month of a year. An optional day of the month
    /// can be appended or an optional week code. Valid formats: YYYYMM YYYYMMDD
    /// YYYYMMWW Valid values: YYYY = 0000-9999; MM = 01-12; DD = 01-31; WW = w1,
    /// w2, w3, w4, w5.
    MonthYear,
    /// string field containing one or more space delimited single character
    /// values (e.g. |18=2 A F| ).
    MultipleCharValue,
    /// string field representing a currency type using ISO 4217 Currency code (3
    /// character) values (see Appendix 6-A).
    Currency,
    /// string field representing a market or exchange using ISO 10383 Market
    /// Identifier Code (MIC) values (see"Appendix 6-C).
    Exchange,
    /// Identifier for a national language - uses ISO 639-1 standard.
    Language,
    /// string field represening a Date of Local Market (as oppose to UTC) in
    /// YYYYMMDD format. This is the "normal" date field used by the FIX
    /// Protocol. Valid values: YYYY = 0000-9999, MM = 01-12, DD = 01-31.
    LocalMktDate,
    /// string field containing one or more space delimited multiple character
    /// values (e.g. |277=AV AN A| ).
    MultipleStringValue,
    /// string field representing Date represented in UTC (Universal Time
    /// Coordinated, also known as "GMT") in YYYYMMDD format. This
    /// special-purpose field is paired with UTCTimeOnly to form a proper
    /// UTCTimestamp for bandwidth-sensitive messages. Valid values: YYYY =
    /// 0000-9999, MM = 01-12, DD = 01-31.
    UtcDateOnly,
    /// string field representing Time-only represented in UTC (Universal Time
    /// Coordinated, also known as "GMT") in either HH:MM:SS (whole seconds) or
    /// HH:MM:SS.sss (milliseconds) format, colons, and period required. This
    /// special-purpose field is paired with UTCDateOnly to form a proper
    /// UTCTimestamp for bandwidth-sensitive messages. Valid values: HH = 00-23,
    /// MM = 00-60 (60 only if UTC leap second), SS = 00-59. (without
    /// milliseconds) HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC leap
    /// second), sss=000-999 (indicating milliseconds).
    UtcTimeOnly,
    /// string field representing Time/date combination represented in UTC
    /// (Universal Time Coordinated, also known as "GMT") in either
    /// YYYYMMDD-HH:MM:SS (whole seconds) or YYYYMMDD-HH:MM:SS.sss (milliseconds)
    /// format, colons, dash, and period required. Valid values: * YYYY =
    /// 0000-9999, MM = 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60
    /// only if UTC leap second) (without milliseconds). * YYYY = 0000-9999, MM =
    /// 01-12, DD = 01-31, HH = 00-23, MM = 00-59, SS = 00-60 (60 only if UTC
    /// leap second), sss=000-999 (indicating milliseconds). Leap Seconds: Note
    /// that UTC includes corrections for leap seconds, which are inserted to
    /// account for slowing of the rotation of the earth. Leap second insertion
    /// is declared by the International Earth Rotation Service (IERS) and has,
    /// since 1972, only occurred on the night of Dec. 31 or Jun 30. The IERS
    /// considers March 31 and September 30 as secondary dates for leap second
    /// insertion, but has never utilized these dates. During a leap second
    /// insertion, a UTCTimestamp field may read "19981231-23:59:59",
    /// "19981231-23:59:60", "19990101-00:00:00". (see
    /// <http://tycho.usno.navy.mil/leapsec.html>)
    UtcTimestamp,
    /// Contains an XML document raw data with no format or content restrictions.
    /// XMLData fields are always immediately preceded by a length field. The
    /// length field should specify the number of bytes of the value of the data
    /// field (up to but not including the terminating SOH).
    XmlData,
    /// string field representing a country using ISO 3166 Country code (2
    /// character) values (see Appendix 6-B).
    Country,
}

impl FixDatatype {
    /// Compares `name` to the set of strings commonly used by QuickFIX's custom
    /// specification format and returns its associated
    /// [`Datatype`](super::Datatype) if a match
    /// was found. The query is case-insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix_dictionary::FixDatatype;
    ///
    /// assert_eq!(FixDatatype::from_quickfix_name("AMT"), Some(FixDatatype::Amt));
    /// assert_eq!(FixDatatype::from_quickfix_name("Amt"), Some(FixDatatype::Amt));
    /// assert_eq!(FixDatatype::from_quickfix_name("MONTHYEAR"), Some(FixDatatype::MonthYear));
    /// assert_eq!(FixDatatype::from_quickfix_name(""), None);
    /// ```
    pub fn from_quickfix_name(name: &str) -> Option<Self> {
        // https://github.com/quickfix/quickfix/blob/b6760f55ac6a46306b4e081bb13b65e6220ab02d/src/C%2B%2B/DataDictionary.cpp#L646-L680
        Some(match name.to_ascii_uppercase().as_str() {
            "AMT" => FixDatatype::Amt,
            "BOOLEAN" => FixDatatype::Boolean,
            "CHAR" => FixDatatype::Char,
            "COUNTRY" => FixDatatype::Country,
            "CURRENCY" => FixDatatype::Currency,
            "DATA" => FixDatatype::Data,
            "DATE" => FixDatatype::UtcDateOnly, // FIXME?
            "DAYOFMONTH" => FixDatatype::DayOfMonth,
            "EXCHANGE" => FixDatatype::Exchange,
            "FLOAT" => FixDatatype::Float,
            "INT" => FixDatatype::Int,
            "LANGUAGE" => FixDatatype::Language,
            "LENGTH" => FixDatatype::Length,
            "LOCALMKTDATE" => FixDatatype::LocalMktDate,
            "MONTHYEAR" => FixDatatype::MonthYear,
            "MULTIPLECHARVALUE" | "MULTIPLEVALUESTRING" => FixDatatype::MultipleCharValue,
            "MULTIPLESTRINGVALUE" => FixDatatype::MultipleStringValue,
            "NUMINGROUP" => FixDatatype::NumInGroup,
            "PERCENTAGE" => FixDatatype::Percentage,
            "PRICE" => FixDatatype::Price,
            "PRICEOFFSET" => FixDatatype::PriceOffset,
            "QTY" => FixDatatype::Qty,
            "STRING" => FixDatatype::String,
            "TZTIMEONLY" => FixDatatype::UtcTimeOnly, // FIXME
            "TZTIMESTAMP" => FixDatatype::UtcTimestamp, // FIXME
            "UTCDATE" => FixDatatype::UtcDateOnly,
            "UTCDATEONLY" => FixDatatype::UtcDateOnly,
            "UTCTIMEONLY" => FixDatatype::UtcTimeOnly,
            "UTCTIMESTAMP" => FixDatatype::UtcTimestamp,
            "SEQNUM" => FixDatatype::SeqNum,
            "TIME" => FixDatatype::UtcTimestamp,
            "XMLDATA" => FixDatatype::XmlData,
            _ => {
                return None;
            }
        })
    }

    /// Returns the name adopted by QuickFIX for `self`.
    pub fn to_quickfix_name(&self) -> &str {
        match self {
            FixDatatype::Int => "INT",
            FixDatatype::Length => "LENGTH",
            FixDatatype::Char => "CHAR",
            FixDatatype::Boolean => "BOOLEAN",
            FixDatatype::Float => "FLOAT",
            FixDatatype::Amt => "AMT",
            FixDatatype::Price => "PRICE",
            FixDatatype::PriceOffset => "PRICEOFFSET",
            FixDatatype::Qty => "QTY",
            FixDatatype::Percentage => "PERCENTAGE",
            FixDatatype::DayOfMonth => "DAYOFMONTH",
            FixDatatype::NumInGroup => "NUMINGROUP",
            FixDatatype::Language => "LANGUAGE",
            FixDatatype::SeqNum => "SEQNUM",
            FixDatatype::TagNum => "TAGNUM",
            FixDatatype::String => "STRING",
            FixDatatype::Data => "DATA",
            FixDatatype::MonthYear => "MONTHYEAR",
            FixDatatype::Currency => "CURRENCY",
            FixDatatype::Exchange => "EXCHANGE",
            FixDatatype::LocalMktDate => "LOCALMKTDATE",
            FixDatatype::MultipleStringValue => "MULTIPLESTRINGVALUE",
            FixDatatype::UtcTimeOnly => "UTCTIMEONLY",
            FixDatatype::UtcTimestamp => "UTCTIMESTAMP",
            FixDatatype::UtcDateOnly => "UTCDATEONLY",
            FixDatatype::Country => "COUNTRY",
            FixDatatype::MultipleCharValue => "MULTIPLECHARVALUE",
            FixDatatype::XmlData => "XMLDATA",
        }
    }

    /// Returns the name of `self`, character by character identical to the name
    /// that appears in the official guidelines. **Generally** primitive datatypes
    /// will use `snake_case` and non-primitive ones will have `PascalCase`, but
    /// that's not true for every [`Datatype`](super::Datatype).
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix_dictionary::FixDatatype;
    ///
    /// assert_eq!(FixDatatype::Qty.name(), "Qty");
    /// assert_eq!(FixDatatype::Float.name(), "float");
    /// assert_eq!(FixDatatype::String.name(), "String");
    /// ```
    pub fn name(&self) -> &'static str {
        // 1. Most primitive data types have `snake_case` names.
        // 2. Most derivative data types have `PascalCase` names.
        // 3. `data` and `String` ruin the party and mess it up.
        //    Why, you ask? Oh, you sweet summer child. You'll learn soon enough
        //    that nothing makes sense in FIX land.
        match self {
            FixDatatype::Int => "int",
            FixDatatype::Length => "Length",
            FixDatatype::Char => "char",
            FixDatatype::Boolean => "Boolean",
            FixDatatype::Float => "float",
            FixDatatype::Amt => "Amt",
            FixDatatype::Price => "Price",
            FixDatatype::PriceOffset => "PriceOffset",
            FixDatatype::Qty => "Qty",
            FixDatatype::Percentage => "Percentage",
            FixDatatype::DayOfMonth => "DayOfMonth",
            FixDatatype::NumInGroup => "NumInGroup",
            FixDatatype::Language => "Language",
            FixDatatype::SeqNum => "SeqNum",
            FixDatatype::TagNum => "TagNum",
            FixDatatype::String => "String",
            FixDatatype::Data => "data",
            FixDatatype::MonthYear => "MonthYear",
            FixDatatype::Currency => "Currency",
            FixDatatype::Exchange => "Exchange",
            FixDatatype::LocalMktDate => "LocalMktDate",
            FixDatatype::MultipleStringValue => "MultipleStringValue",
            FixDatatype::UtcTimeOnly => "UTCTimeOnly",
            FixDatatype::UtcTimestamp => "UTCTimestamp",
            FixDatatype::UtcDateOnly => "UTCDateOnly",
            FixDatatype::Country => "Country",
            FixDatatype::MultipleCharValue => "MultipleCharValue",
            FixDatatype::XmlData => "XMLData",
        }
    }

    /// Returns `true` if and only if `self` is a "base type", i.e. a primitive;
    /// returns `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix_dictionary::FixDatatype;
    ///
    /// assert_eq!(FixDatatype::Float.is_base_type(), true);
    /// assert_eq!(FixDatatype::Price.is_base_type(), false);
    /// ```
    pub fn is_base_type(&self) -> bool {
        match self {
            Self::Char | Self::Float | Self::Int | Self::String => true,
            _ => false,
        }
    }

    /// Returns the primitive [`Datatype`](super::Datatype) from which `self` is derived. If
    /// `self` is primitive already, returns `self` unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use fefix_dictionary::FixDatatype;
    ///
    /// assert_eq!(FixDatatype::Float.base_type(), FixDatatype::Float);
    /// assert_eq!(FixDatatype::Price.base_type(), FixDatatype::Float);
    /// ```
    pub fn base_type(&self) -> Self {
        let dt = match self {
            Self::Char | Self::Boolean => Self::Char,
            Self::Float
            | Self::Amt
            | Self::Price
            | Self::PriceOffset
            | Self::Qty
            | Self::Percentage => Self::Float,
            Self::Int
            | Self::DayOfMonth
            | Self::Length
            | Self::NumInGroup
            | Self::SeqNum
            | Self::TagNum => Self::Int,
            _ => Self::String,
        };
        debug_assert!(dt.is_base_type());
        dt
    }

    /// Returns an [`Iterator`] over all variants of
    /// [`Datatype`](super::Datatype).
    pub fn iter_all() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn iter_all_unique() {
        let as_vec = FixDatatype::iter_all().collect::<Vec<FixDatatype>>();
        let as_set = FixDatatype::iter_all().collect::<HashSet<FixDatatype>>();
        assert_eq!(as_vec.len(), as_set.len());
    }

    #[test]
    fn more_than_20_datatypes() {
        // According to the official documentation, FIX has "about 20 data
        // types". Including recent revisions, we should well exceed that
        // number.
        assert!(FixDatatype::iter_all().count() > 20);
    }

    #[test]
    fn names_are_unique() {
        let as_vec = FixDatatype::iter_all()
            .map(|dt| dt.name())
            .collect::<Vec<&str>>();
        let as_set = FixDatatype::iter_all()
            .map(|dt| dt.name())
            .collect::<HashSet<&str>>();
        assert_eq!(as_vec.len(), as_set.len());
    }

    #[test]
    fn base_type_is_itself() {
        for dt in FixDatatype::iter_all() {
            if dt.is_base_type() {
                assert_eq!(dt.base_type(), dt);
            } else {
                assert_ne!(dt.base_type(), dt);
            }
        }
    }

    #[test]
    fn base_type_is_actually_base_type() {
        for dt in FixDatatype::iter_all() {
            assert!(dt.base_type().is_base_type());
        }
    }
}
