/// Configuration options for the FIX JSON encoding format.
#[derive(Debug, Clone)]
#[non_exhaustive]
#[derive(Default)]
pub struct Config {
    /// This setting indicates that all encoded messages should be "prettified"
    /// if possible, i.e. the JSON code will not be compressed and instead it
    /// will have indentation and other whitespace that favors human
    /// readability. Some performance loss and increased payload size is
    /// expected.
    ///
    /// This is turned **off** be default.
    ///
    /// This setting has no effect when decoding messages.
    ///
    /// # Output examples
    ///
    /// With "pretty print":
    ///
    /// ```json
    /// {
    ///     "Header": {
    ///         "...": "..."
    ///     },
    ///     "Body": {
    ///         "...": "..."
    ///     },
    ///     "Trailer": {
    ///         "...": "..."
    ///     }
    /// }
    /// ```
    ///
    /// Without "pretty print":
    ///
    /// ```json
    /// {"Header":{"...":"..."},"Body":{"...":"..."},"Trailer":{"...":"..."}}
    /// ```
    pub pretty_print: bool,
}
