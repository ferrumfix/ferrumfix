/// Sum type for all SOFH encoding types.
///
/// Each variant is associated with a single value or range of values, as
/// specified by the SOFH specification. This type is marked with
/// `#[non_exhaustive]` to
/// support new encoding types without breaking compatibility.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
}

impl EncodingType {
    const PRIVATE_START: u16 = 0x1;
    const PRIVATE_END: u16 = 0xFF;
    const PROTOBUF: u16 = 0x4700;
    const SBE10BE: u16 = 0x5BE0;
    const ASN1PER: u16 = 0xA500;
    const ASN1BER: u16 = 0xA501;
    const ASN1OER: u16 = 0xA502;
    const SBE10LE: u16 = 0xEB50;
    const TAGVALUE: u16 = 0xF000;
    const FIXML_SCHEMA: u16 = 0xF100;
    const JSON: u16 = 0xF500;
    const FAST_OFFSET: u16 = 0xFA00;
    const FAST_START: u16 = Self::FAST_OFFSET + 0x1;
    const FAST_END: u16 = Self::FAST_OFFSET + 0xFF;
    const BSON: u16 = 0xFB00;

    /// Tries to create a [`EncodingType`] from its [`u16`] representation.
    ///
    /// ```
    /// use rustysofh::EncodingType;
    ///
    /// assert_eq!(EncodingType::new(0x4700), Some(EncodingType::Protobuf));
    /// assert_eq!(EncodingType::new(0), None);
    /// ```
    pub const fn new(value: u16) -> Option<Self> {
        // https://www.fixtrading.org/standards/fix-sofh-online/#encoding_type-field
        Some(match value {
            Self::PRIVATE_START..=Self::PRIVATE_END => EncodingType::Private(value as u8),
            Self::PROTOBUF => EncodingType::Protobuf,
            Self::SBE10BE => EncodingType::SimpleBinaryEncodingV10BE,
            Self::ASN1PER => EncodingType::Asn1PER,
            Self::ASN1BER => EncodingType::Asn1BER,
            Self::ASN1OER => EncodingType::Asn1OER,
            Self::SBE10LE => EncodingType::SimpleBinaryEncodingV10LE,
            Self::TAGVALUE => EncodingType::TagValue,
            Self::FIXML_SCHEMA => EncodingType::FixmlSchema,
            Self::JSON => EncodingType::Json,
            Self::FAST_START..=Self::FAST_END => {
                EncodingType::Fast((value - Self::FAST_OFFSET) as u8)
            }
            EncodingType::BSON => EncodingType::Bson,
            _ => return None,
        })
    }

    /// Deserializes [`EncodingType`] from two bytes. Big-endian byte order is
    /// assumed, as mandated by the SOFH specification.
    ///
    /// ```
    /// use rustysofh::EncodingType;
    ///
    /// assert_eq!(EncodingType::from_bytes([0xF0, 0x00]), Some(EncodingType::TagValue));
    /// assert_eq!(EncodingType::from_bytes([0xFA, 0x42]), Some(EncodingType::Fast(0x42)));
    /// ```
    pub const fn from_bytes(bytes: [u8; 2]) -> Option<Self> {
        Self::new(u16::from_be_bytes(bytes))
    }

    /// Serializes `self` to two bytes. Big-endian byte order is
    /// assumed, as mandated by the SOFH specification.
    ///
    /// ```
    /// use rustysofh::EncodingType;
    ///
    /// assert_eq!(EncodingType::TagValue.to_bytes(), [0xF0, 0x00]);
    /// assert_eq!(EncodingType::Fast(0x42).to_bytes(), [0xFA, 0x42]);
    /// ```
    pub const fn to_bytes(&self) -> [u8; 2] {
        to_u16(*self).to_be_bytes()
    }
}

const fn to_u16(etype: EncodingType) -> u16 {
    match etype {
        EncodingType::Private(x) => x as u16,
        EncodingType::Protobuf => EncodingType::PROTOBUF,
        EncodingType::SimpleBinaryEncodingV10BE => EncodingType::SBE10BE,
        EncodingType::Asn1PER => EncodingType::ASN1PER,
        EncodingType::Asn1BER => EncodingType::ASN1BER,
        EncodingType::Asn1OER => EncodingType::ASN1OER,
        EncodingType::SimpleBinaryEncodingV10LE => EncodingType::SBE10LE,
        EncodingType::TagValue => EncodingType::TAGVALUE,
        EncodingType::FixmlSchema => EncodingType::FIXML_SCHEMA,
        EncodingType::Json => EncodingType::JSON,
        EncodingType::Fast(x) => EncodingType::FAST_OFFSET | (x as u16),
        EncodingType::Bson => EncodingType::BSON,
    }
}

impl From<EncodingType> for u16 {
    fn from(etype: EncodingType) -> Self {
        to_u16(etype)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encoding_type_takes_up_two_bytes() {
        assert_eq!(std::mem::size_of::<EncodingType>(), 2);
    }

    #[test]
    fn convert_encoding_type_to_bytes_then_back_has_no_side_effects() {
        for value in 0..=u16::MAX {
            if let Some(etype) = EncodingType::new(value) {
                let etype_after = EncodingType::from_bytes(etype.to_bytes());
                assert_eq!(Some(etype), etype_after);
            }
        }
    }

    #[test]
    fn convert_u16_into_encoding_type_then_back_has_no_side_effects() {
        for value in 0..=u16::MAX {
            if let Some(etype) = EncodingType::new(value) {
                assert_eq!(value, u16::from(etype));
            }
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
            let etype = EncodingType::new(*value).unwrap();
            assert!(matches!(etype, EncodingType::Private(x) if x as u16 == *value));
        }
    }

    #[test]
    fn boundary_values_for_private_encoding_type() {
        assert!(!matches!(
            EncodingType::new(0x0u16),
            Some(EncodingType::Private(_))
        ));
        assert!(!matches!(
            EncodingType::new(0x100u16),
            Some(EncodingType::Private(_))
        ));
    }

    #[test]
    fn boundary_values_for_fast_encoding_type() {
        assert!(!matches!(
            EncodingType::new(0xFA00u16),
            Some(EncodingType::Fast(_))
        ));
        assert!(!matches!(
            EncodingType::new(0xFB00u16),
            Some(EncodingType::Fast(_))
        ));
    }
}
