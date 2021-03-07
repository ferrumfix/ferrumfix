const ENCODING_TYPE_FAST_OFFSET: u16 = 0xFA00;

/// Sum type that can be converted [`Into`](Into) and [`From`](From) a
/// `u16` encoding type value.
///
/// Each variant has a predetermined value or range of values, as specified by
/// the official guidelines. This type is marked with `#[non_exhaustive]` to
/// support new encoding types without breaking compatibility;
/// [`EncodingType::Unknown`](EncodingType::Unknown) contains all values that are
/// not included in the official guidelines; this way conversion is infallible
/// and doesn't lose any information.
///
/// # Equality
///
/// It's important to note that the behavior of [`Eq`](Eq) and
/// [`PartialEq`](PartialEq) for this type always falls back to equality on
/// `u16`. This may cause unusual behavior e.g.:
///
/// ```
/// use fefix::codec::sofh::EncodingType;
///
/// let e_type = EncodingType::Unknown(0xF500);
/// assert_eq!(e_type, EncodingType::Json);
/// ```
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum EncodingType {
    /// User-specified encoding type. Legal values and their respective semantics
    /// ought to be agreed upon out-of-band by counterparties.
    ///
    /// Please note that `0x0` is *not* a valid [`EncodingType::Private`] value.
    Private(u8),
    /// Simple Binary Encoding (SBE) v1.0, big-endian mode.
    /// Please refer to <https://www.fixtrading.org/standards/sbe/> for more
    /// information.
    SimpleBinaryEncodingV10BE,
    /// Simple Binary Encoding (SBE) v1.0, little-endian mode.
    /// Please refer to <https://www.fixtrading.org/standards/sbe/> for more
    /// information.
    SimpleBinaryEncodingV10LE,
    /// Google's "Protobuf".
    /// Please refer to <https://www.fixtrading.org/standards/gpb/> for more
    /// information.
    Protobuf,
    /// ASN.1 with Packed Encoding Rules (PER).
    /// Please refer to <https://www.fixtrading.org/standards/asn1/> for more
    /// information.
    Asn1PER,
    /// ASN.1 with Basic Encoding Rules (BER).
    /// Please refer to <https://www.fixtrading.org/standards/asn1/> for more
    /// information.
    Asn1BER,
    /// ASN.1 with Octet Encoding Rules (OER).
    /// Please refer to <https://www.fixtrading.org/standards/asn1/> for more
    /// information.
    Asn1OER,
    /// Tag-value (classic) encoding.
    /// Please refer to <https://www.fixtrading.org/standards/tagvalue/> for more
    /// information.
    TagValue,
    /// FIXML encoding.
    /// Please refer to <https://www.fixtrading.org/standards/fixml/> for more
    /// information.
    FixmlSchema,
    /// FAST encoding.
    /// Please refer to <https://www.fixtrading.org/standards/fast/> for more
    /// information.
    ///
    /// Please note that `0xFA00` is *not* a valid [`EncodingType::Fast`] value.
    Fast(u8),
    /// JSON encoding.
    /// Please refer to <https://www.fixtrading.org/standards/json/> for more
    /// information.
    Json,
    /// BSON encoding.
    /// Please refer to <https://www.fixtrading.org/standards/bson/> for more
    /// information.
    Bson,
    /// Unknown value.
    Unknown(u16),
}

impl From<u16> for EncodingType {
    fn from(encoding_type: u16) -> Self {
        // https://www.fixtrading.org/standards/fix-sofh-online/#encoding_type-field
        match encoding_type {
            0x1..=0xFF => EncodingType::Private(encoding_type as u8),
            0x4700 => EncodingType::Protobuf,
            0x5BE0 => EncodingType::SimpleBinaryEncodingV10BE,
            0xA500 => EncodingType::Asn1PER,
            0xA501 => EncodingType::Asn1BER,
            0xA502 => EncodingType::Asn1OER,
            0xEB50 => EncodingType::SimpleBinaryEncodingV10LE,
            0xF000 => EncodingType::TagValue,
            0xF100 => EncodingType::FixmlSchema,
            0xF500 => EncodingType::Json,
            0xFA01..=0xFAFF => {
                EncodingType::Fast((encoding_type - ENCODING_TYPE_FAST_OFFSET) as u8)
            }
            0xFB00 => EncodingType::Bson,
            _ => EncodingType::Unknown(encoding_type),
        }
    }
}

impl From<EncodingType> for u16 {
    fn from(encoding_type: EncodingType) -> Self {
        match encoding_type {
            EncodingType::Private(x) => x as u16,
            EncodingType::Protobuf => 0x4700,
            EncodingType::SimpleBinaryEncodingV10BE => 0x5BE0,
            EncodingType::Asn1PER => 0xA500,
            EncodingType::Asn1BER => 0xA501,
            EncodingType::Asn1OER => 0xA502,
            EncodingType::SimpleBinaryEncodingV10LE => 0xEB50,
            EncodingType::TagValue => 0xF000,
            EncodingType::FixmlSchema => 0xF100,
            EncodingType::Json => 0xF500,
            EncodingType::Fast(x) => ENCODING_TYPE_FAST_OFFSET + (x as u16),
            EncodingType::Bson => 0xFB00,
            EncodingType::Unknown(x) => x,
        }
    }
}

impl PartialEq for EncodingType {
    fn eq(&self, other: &Self) -> bool {
        u16::from(*self) == u16::from(*other)
    }
}

impl std::cmp::Eq for EncodingType {}

impl std::hash::Hash for EncodingType {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        u16::from(*self).hash(state)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encoding_type_conversion_is_correct() {
        let mut value = 0u16;
        loop {
            let encoding_type = EncodingType::from(value);
            assert_eq!(value, u16::from(encoding_type));
            if value == u16::MAX {
                return;
            }
            value += 1;
        }
    }

    #[test]
    fn low_values_correspond_to_private_encoding_types() {
        for value in &[0x1, 0x82, 0xff] {
            let encoding_type = EncodingType::from(*value);
            match encoding_type {
                EncodingType::Private(x) if x as u16 == *value => (),
                _ => panic!(),
            };
        }
    }

    #[test]
    fn every_encoding_type_is_equal_to_itself() {
        let mut value = 0u16;
        loop {
            let encoding_type = EncodingType::from(value);
            assert_eq!(encoding_type, encoding_type);
            if value == u16::MAX {
                return;
            }
            value += 1;
        }
    }

    #[test]
    fn value_0x100u16_is_not_a_private_encoding_type() {
        let encoding_type = EncodingType::from(0x100);
        if let EncodingType::Private(_) = encoding_type {
            panic!();
        }
    }
}