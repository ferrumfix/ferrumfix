use crate::Buffer;

/// Provides (de)serialization logic for a Rust type as FIX field values.
///
/// See the [`fix_values`](crate::fix_values) module for more information.
pub trait FixValue<'a>
where
    Self: Sized,
{
    /// The error type that can arise during deserialization.
    type Error;
    /// A type with values that customize the serialization algorithm, e.g.
    /// padding information.
    type SerializeSettings: Default;

    /// Writes `self` to `buffer` using default settings.
    #[inline]
    fn serialize<B>(&self, buffer: &mut B) -> usize
    where
        B: Buffer,
    {
        self.serialize_with(buffer, Self::SerializeSettings::default())
    }

    /// Writes `self` to `buffer` using custom serialization `settings`.
    fn serialize_with<B>(&self, buffer: &mut B, _settings: Self::SerializeSettings) -> usize
    where
        B: Buffer;

    /// Parses and deserializes from `data`.
    fn deserialize(data: &'a [u8]) -> Result<Self, Self::Error>;

    /// Like [`FixValue::deserialize`], but it's allowed to skip *some* amount of
    /// input checking. Invalid inputs might not trigger errors and instead be
    /// deserialized as random values.
    fn deserialize_lossy(data: &'a [u8]) -> Result<Self, Self::Error> {
        Self::deserialize(data)
    }

    /// Serializes `self` to a [`Vec`] of bytes, allocated on the fly.
    fn to_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        self.serialize(&mut buffer);
        buffer
    }

    /// Allocates a [`String`] representation of `self`.
    ///
    /// # Panics
    /// This function will panic if the underlying byte representation is not
    /// valid UTF-8. As such, you should only *ever* use this function for
    /// [`FixValue`] implementors that are guaranteed to be representable with
    /// valid UTF-8 (like numbers with ASCII digits).
    fn to_string(&self) -> String {
        String::from_utf8(self.to_bytes()).expect("Invalid UTF-8 representation of FIX field.")
    }
}
