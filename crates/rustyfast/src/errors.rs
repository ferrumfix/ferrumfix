use std::fmt::Display;
use std::{fmt, io};

/// Any error that is detected solely by examining a template definition, thus
/// even before receiving any data stream. Counterparties MUST signal static
/// errors and the template where the original error occurred must be discarded.
#[derive(Copy, Clone, Debug)]
pub enum StaticError {
    /// It is a static error if templates encoded in the concrete XML syntax are
    /// in fact not well-formed, do not follow the rules of XML namespaces or are
    /// invalid with respect to the schema in Appendix 1.
    S1 = 1,
    /// It is a static error if an operator is specified for a field of a type to
    /// which the operator is not applicable.
    S2 = 2,
    /// It is a static error if an initial value specified by the value attribute
    /// in the concrete syntax cannot be converted to a value of the type of the
    /// field.
    S3 = 3,
    /// It is a static error if no initial value is specified for a constant
    /// operator.
    S4 = 4,
    /// It is a static error if no initial value is specified for a default
    /// operator on a mandatory field.
    S5 = 5,
}

/// Any error detected when encoding or decoding a FAST stream. Counterparties
/// MUST signal dynamic errors.
#[derive(Copy, Clone, Debug)]
pub enum DynamicError {
    /// It is a dynamic error if type of a field in a template cannot be
    /// converted to or from the type of the corresponding application field.
    D1 = 1,
    /// It is a dynamic error if an integer in the stream does not fall within
    /// the bounds of the specific integer type specified on the corresponding
    /// field.
    D2 = 2,
    /// It is a dynamic error if a decimal value cannot be encoded due to
    /// limitations introduced by using individual operators on exponent and
    /// mantissa.
    D3 = 3,
    /// It is a dynamic error if the type of the previous value is not the same
    /// as the type of the field of the current operator.
    D4 = 4,
    /// It is a dynamic error if a mandatory field is not present in the stream,
    /// has an undefined previous value and there is no initial value in the
    /// instruction context.
    D5 = 5,
    /// It is a dynamic error if a mandatory field is not present in the stream
    /// and has an empty previous value.
    D6 = 6,
    /// It is a dynamic error if the subtraction length exceeds the length of the
    /// base value or if it does not fall in the value rang of an int32.
    D7 = 7,
    /// It is a dynamic error if the name specified on a static template
    /// reference does not point to a template known by the encoder or decoder.
    D8 = 8,
    /// It is a dynamic error if a decoder cannot find a template associated with
    /// a template identifier appearing in the stream.
    D9 = 9,
    /// It is a dynamic error if the syntax of a string does not follow the rules
    /// for the type converted to.
    D10 = 10,
    /// It is a dynamic error if the syntax of a string does not follow the rules
    /// for the type converted to
    D11 = 11,
    /// It is a dynamic error if a block length preamble is zero.
    D12 = 12,
}

/// Any error detected when encoding or decoding a FAST stream. Contrary to
/// dynamic errors, counterparties are not obligated to signal dynamic errors an
/// may choose not to do so, e.g. to improve performance.
#[derive(Copy, Clone, Debug)]
pub enum ReportableError {
    /// It is a reportable error if a decimal cannot be represented by an
    /// exponent in the range [-63 … 63] or if the mantissa does not fit in an
    /// int64.
    R1 = 1,
    /// It is a reportable error if the combined value after applying a tail or
    /// delta operator to a Unicode string is not a valid UTF-8 sequence.
    R2 = 2,
    /// It is a reportable error if a Unicode string that is being converted to
    /// an ASCII string contains characters that are outside the ASCII character
    /// set.
    R3 = 3,
    /// It is a reportable error if a value of an integer type cannot be
    /// represented in the target integer type in a conversion.
    R4 = 4,
    /// It is a reportable error if a decimal being converted to an integer has a
    /// negative exponent or if the resulting integer does not fit the target
    /// integer type.
    R5 = 5,
    /// It is a reportable error if an integer appears in an overlong encoding.
    R6 = 6,
    /// It is a reportable error if a presence map is overlong.
    R7 = 7,
    /// It is a reportable error if a presence map contains more bits than required.
    R8 = 8,
    /// It is a reportable error if a string appears in an overlong encoding.
    R9 = 9,
}

#[derive(Debug)]
pub enum Error {
    Static(StaticError),
    Dynamic(DynamicError),
    Reportable(ReportableError),
}

impl From<io::Error> for Error {
    fn from(_err: io::Error) -> Self {
        Error::Dynamic(DynamicError::D1)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dynamic(e) => {
                write!(f, "Dynamic Error (D{}): ", *e as u8)?;
                (*e).fmt(f)
            }
            Error::Reportable(e) => {
                write!(f, "Reportable Error (R{}): ", *e as u8)?;
                (*e).fmt(f)
            }
            Error::Static(e) => {
                write!(f, "Dynamic Error (D{}): ", *e as u8)?;
                (*e).fmt(f)
            }
        }
    }
}

impl Display for StaticError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::S1 => "The template is not encoded correctly according to the XML spec.",
            Self::S2 => {
                "An operator is specified for a field of a type to which the operator is not applicable."
            }
            Self::S3 => {
                "No initial value is specified for a default operator on a mandatory field."
            }
            Self::S4 => "No initial value is specified for a constant operator.",
            Self::S5 => {
                "No initial value is specified for a default operator on a mandatory field."
            }
        };
        write!(f, "{message}")
    }
}

impl Display for DynamicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::D1 => {
                "A field in a template cannot be converted to or from the type of the corresponding application field"
            }
            Self::D2 => {
                "An integer in the stream does not fall within the bounds of the specific integer type specified on the corresponding field."
            }
            Self::D3 => {
                "A decimal value cannot be encoded due to limitations introduced by using individual operators on exponent and mantissa."
            }
            Self::D4 => {
                "The type of the previous value is not the same as the type of the field of the current operator."
            }
            Self::D5 => {
                "A mandatory field is not present in the stream, has an undefined previous value and there is no initial value in the instruction context."
            }
            Self::D6 => {
                "A mandatory field is not present in the stream and has an empty previous value."
            }
            Self::D7 => {
                "The subtraction length exceeds the length of the base value or if it does not fall in the value rang of an int32."
            }
            Self::D8 => {
                "The name specified on a static template reference does not point to a template known by the encoder or decoder."
            }
            Self::D9 => {
                "A decoder cannot find a template associated with a template identifier appearing in the stream."
            }
            Self::D10 => {
                "The syntax of a string does not follow the rules for the type converted to."
            }
            Self::D11 => {
                "The syntax of a string does not follow the rules for the type converted to."
            }
            Self::D12 => "A block length preamble is zero.",
        };
        write!(f, "{message}")
    }
}

impl Display for ReportableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::R1 => {
                "A decimal cannot be represented by an exponent in the range [-63 … 63] or if the mantissa does not fit in an int64."
            }
            Self::R2 => {
                "The combined value after applying a tail or delta operator to a Unicode string is not a valid UTF-8 sequence."
            }
            Self::R3 => {
                "A Unicode string that is being converted to an ASCII string contains characters that are outside the ASCII character set."
            }
            Self::R4 => {
                "A value of an integer type cannot be represented in the target integer type in a conversion."
            }
            Self::R5 => {
                "A decimal being converted to an integer has a negative exponent or if the resulting integer does not fit the target integer type."
            }
            Self::R6 => "An integer appears in an overlong encoding.",
            Self::R7 => "A presence map is overlong.",
            Self::R8 => "A presence map contains more bits than required.",
            Self::R9 => "A string appears in an overlong encoding.",
        };
        write!(f, "{message}")
    }
}
