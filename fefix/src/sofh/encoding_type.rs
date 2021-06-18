/// Sum type for all SOFH encoding types.
///
/// Each variant is associated with a single value or range of values, as
/// specified by the SOFH specification. This type is marked with
/// `#[non_exhaustive]` to
/// support new encoding types without breaking compatibility.
/// [`EncodingType::Unknown`] can represent all values that are
/// not included in the SOFH specification; this way conversion is infallible
/// and doesn't lose any information.
///
/// # Equality
///
/// It's important to note that the behavior of [`Eq`] and
/// [`PartialEq`] for this type always falls back to equality on
/// `u16`. This may cause unusual behavior when comparing
/// [`EncodingType::Unknown`] values e.g.:
///
/// ```
/// use fefix::sofh::EncodingType;
///
/// assert_eq!(EncodingType::Unknown(0xF500), EncodingType::Json);
/// ```
#[derive(Copy, Clone, Debug)]
#[non_exhaustive]
pub enum EncodingType {
    /// User-specified encoding types. Legal values and their respective semantics
    /// ought to be agreed upon out-of-band by counterparties.
    ///
    /// The SOFH specification allows for up to 255 different private encoding
    /// types.
    Private(u8),
    /// Simple Binary Encoding (SBE) v1.0, big-endian mode.
    /// Please refer to <https://www.fixtrading.org/standards/sbe/> for more
    /// information.
    SimpleBinaryEncodingV10BE,
    /// Simple Binary Encoding (SBE) v1.0, little-endian mode.
    /// Please refer to <https://www.fixtrading.org/standards/sbe/> for more
    /// information.
    SimpleBinaryEncodingV10LE,
    /// Google's "Protobuf". See the
    /// [docs](https://www.fixtrading.org/standards/gpb/) for more information.
    Protobuf,
    /// ASN.1 with Packed Encoding Rules (PER). See the
    /// [docs](https://www.fixtrading.org/standards/asn1/) for more information.
    Asn1PER,
    /// ASN.1 with Basic Encoding Rules (BER). See the
    /// [docs](https://www.fixtrading.org/standards/asn1/) for more information.
    Asn1BER,
    /// ASN.1 with Octet Encoding Rules (OER). See the
    /// [docs](https://www.fixtrading.org/standards/asn1/) for more information.
    Asn1OER,
    /// Tag-value (classic) encoding. See the
    /// [docs](https://www.fixtrading.org/standards/tagvalue/) for more
    /// information.
    TagValue,
    /// Custom schema for FIXML encoding. See the
    /// [docs](https://www.fixtrading.org/standards/fixml/) for more information.
    FixmlSchema,
    /// FAST encoding. See the [docs](https://www.fixtrading.org/standards/fast/)
    /// for more information.
    ///
    /// The SOFH specification allows for up to 255 different values for FAST.
    Fast(u8),
    /// JSON encoding. See the [docs](https://www.fixtrading.org/standards/json/)
    /// for more information.
    Json,
    /// BSON encoding. See the [docs](https://www.fixtrading.org/standards/bson/)
    /// for more information.
    Bson,
    /// This variant can represent any encoding type that isn't officially
    /// supported by FIX. The original `u16` value is available for custom
    /// processing or logging.
    Unknown(u16),
}

impl EncodingType {
    /// Deserializes [`EncodingType`] from two bytes. Big-endian byte order is
    /// assumed, as mandated by the SOFH specification.
    ///
    /// ```
    /// use fefix::sofh::EncodingType;
    ///
    /// assert_eq!(EncodingType::from_bytes([0xF0, 0x00]), EncodingType::TagValue);
    /// assert_eq!(EncodingType::from_bytes([0xFA, 0x42]), EncodingType::Fast(0x42));
    /// ```
    pub const fn from_bytes(bytes: [u8; 2]) -> Self {
        from_u16(u16::from_be_bytes(bytes))
    }

    /// Serializes `self` to two bytes. Big-endian byte order is
    /// assumed, as mandated by the SOFH specification.
    ///
    /// ```
    /// use fefix::sofh::EncodingType;
    ///
    /// assert_eq!(EncodingType::TagValue.to_bytes(), [0xF0, 0x00]);
    /// assert_eq!(EncodingType::Fast(0x42).to_bytes(), [0xFA, 0x42]);
    /// ```
    pub const fn to_bytes(&self) -> [u8; 2] {
        to_u16(*self).to_be_bytes()
    }
}

impl From<u16> for EncodingType {
    fn from(value: u16) -> Self {
        from_u16(value)
    }
}

impl From<EncodingType> for u16 {
    fn from(etype: EncodingType) -> Self {
        to_u16(etype)
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

const ENCODING_TYPE_PRIVATE_START: u16 = 0x1;
const ENCODING_TYPE_PRIVATE_END: u16 = 0xFF;
const ENCODING_TYPE_PROTOBUF: u16 = 0x4700;
const ENCODING_TYPE_SBE10BE: u16 = 0x5BE0;
const ENCODING_TYPE_ASN1PER: u16 = 0xA500;
const ENCODING_TYPE_ASN1BER: u16 = 0xA501;
const ENCODING_TYPE_ASN1OER: u16 = 0xA502;
const ENCODING_TYPE_SBE10LE: u16 = 0xEB50;
const ENCODING_TYPE_TAGVALUE: u16 = 0xF000;
const ENCODING_TYPE_FIXML_SCHEMA: u16 = 0xF100;
const ENCODING_TYPE_JSON: u16 = 0xF500;
const ENCODING_TYPE_FAST_OFFSET: u16 = 0xFA00;
const ENCODING_TYPE_FAST_START: u16 = ENCODING_TYPE_FAST_OFFSET + 0x1;
const ENCODING_TYPE_FAST_END: u16 = ENCODING_TYPE_FAST_OFFSET + 0xFF;
const ENCODING_TYPE_BSON: u16 = 0xFB00;

const fn from_u16(value: u16) -> EncodingType {
    // https://www.fixtrading.org/standards/fix-sofh-online/#encoding_type-field
    match value {
        ENCODING_TYPE_PRIVATE_START..=ENCODING_TYPE_PRIVATE_END => {
            EncodingType::Private(value as u8)
        }
        ENCODING_TYPE_PROTOBUF => EncodingType::Protobuf,
        ENCODING_TYPE_SBE10BE => EncodingType::SimpleBinaryEncodingV10BE,
        ENCODING_TYPE_ASN1PER => EncodingType::Asn1PER,
        ENCODING_TYPE_ASN1BER => EncodingType::Asn1BER,
        ENCODING_TYPE_ASN1OER => EncodingType::Asn1OER,
        ENCODING_TYPE_SBE10LE => EncodingType::SimpleBinaryEncodingV10LE,
        ENCODING_TYPE_TAGVALUE => EncodingType::TagValue,
        ENCODING_TYPE_FIXML_SCHEMA => EncodingType::FixmlSchema,
        ENCODING_TYPE_JSON => EncodingType::Json,
        ENCODING_TYPE_FAST_START..=ENCODING_TYPE_FAST_END => {
            EncodingType::Fast((value - ENCODING_TYPE_FAST_OFFSET) as u8)
        }
        ENCODING_TYPE_BSON => EncodingType::Bson,
        _ => EncodingType::Unknown(value),
    }
}

const fn to_u16(etype: EncodingType) -> u16 {
    match etype {
        EncodingType::Private(x) => x as u16,
        EncodingType::Protobuf => ENCODING_TYPE_PROTOBUF,
        EncodingType::SimpleBinaryEncodingV10BE => ENCODING_TYPE_SBE10BE,
        EncodingType::Asn1PER => ENCODING_TYPE_ASN1PER,
        EncodingType::Asn1BER => ENCODING_TYPE_ASN1BER,
        EncodingType::Asn1OER => ENCODING_TYPE_ASN1OER,
        EncodingType::SimpleBinaryEncodingV10LE => ENCODING_TYPE_SBE10LE,
        EncodingType::TagValue => ENCODING_TYPE_TAGVALUE,
        EncodingType::FixmlSchema => ENCODING_TYPE_FIXML_SCHEMA,
        EncodingType::Json => ENCODING_TYPE_JSON,
        EncodingType::Fast(x) => ENCODING_TYPE_FAST_OFFSET + (x as u16),
        EncodingType::Bson => ENCODING_TYPE_BSON,
        EncodingType::Unknown(x) => x,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_encoding_type_to_bytes_then_back_has_no_side_effects() {
        for value in 0..=u16::MAX {
            let etype = EncodingType::from(value);
            let etype_after = EncodingType::from_bytes(etype.to_bytes());
            assert_eq!(etype, etype_after);
        }
    }

    #[test]
    fn convert_u16_into_encoding_type_then_back_has_no_side_effects() {
        for value in 0..=u16::MAX {
            let value_after = u16::from(EncodingType::from(value));
            assert_eq!(value, value_after);
        }
    }

    #[test]
    fn equality_is_reflexive() {
        for value in 0..=u16::MAX {
            let etype = EncodingType::from(value);
            assert_eq!(etype, etype);
        }
    }

    #[test]
    fn encoding_types_with_ranges_use_prefix_tagging() {
        assert_eq!(EncodingType::Private(42).to_bytes()[1], 42);
        assert_eq!(EncodingType::Fast(100).to_bytes()[1], 100);
    }

    #[test]
    fn low_values_correspond_to_private_encoding_types() {
        for value in &[0x1, 0x82, 0xff] {
            let etype = EncodingType::from(*value);
            assert!(matches!(etype, EncodingType::Private(x) if x as u16 == *value));
        }
    }

    #[test]
    fn boundary_values_for_private_encoding_type() {
        assert!(!matches!(
            EncodingType::from(0x0u16),
            EncodingType::Private(_)
        ));
        assert!(!matches!(
            EncodingType::from(0x100u16),
            EncodingType::Private(_)
        ));
    }

    #[test]
    fn boundary_values_for_fast_encoding_type() {
        assert!(!matches!(
            EncodingType::from(0xFA00u16),
            EncodingType::Fast(_)
        ));
        assert!(!matches!(
            EncodingType::from(0xFB00u16),
            EncodingType::Fast(_)
        ));
    }
}
