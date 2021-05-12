use crate::dict::IsFieldDefinition;
use crate::FixValue;
use crate::{OptError, OptResult};

/// A trait to retrieve field values in a FIX message.
///
/// # Field getters naming scheme
///
/// All getters start with `fv`, which stands for Field Value.
/// - `l` stands for *lossy*, i.e. invalid field values might not be detected to
/// improve performance.
/// - `_with_key` stands for *with key*, i.e. you won't use a field definition but
/// direct key (i.e. field name or tag).
/// - `_opt` stands for *optional*, for better error reporting.
pub trait FieldGetter<'a> {
    type Key;

    fn fv_raw_with_key<'b>(&'b self, key: Self::Key) -> Option<&'b [u8]>;

    fn fv_raw<'b, F>(&'b self, field: &'a F) -> Option<&'b [u8]>
    where
        'b: 'a,
        F: IsFieldDefinition;

    fn fv_opt<'b, V, F>(&'b self, field: &'a F) -> Option<Result<V, V::Error>>
    where
        'b: 'a,
        V: FixValue<'b>,
        F: IsFieldDefinition,
    {
        self.fv_raw(field).map(|raw| match V::deserialize(raw) {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into()),
        })
    }

    fn fv<'b, V, F>(&'b self, field: &'a F) -> OptResult<V, V::Error>
    where
        'b: 'a,
        V: FixValue<'b>,
        F: IsFieldDefinition,
    {
        match self.fv_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fvl_opt<'b, V, F>(&'b self, field: &'a F) -> Option<Result<V, V::Error>>
    where
        'b: 'a,
        V: FixValue<'b>,
        F: IsFieldDefinition,
    {
        self.fv_raw(field)
            .map(|raw| match V::deserialize_lossy(raw) {
                Ok(value) => Ok(value),
                Err(err) => Err(err.into()),
            })
    }

    fn fvl<'b, V, F>(&'b self, field: &'a F) -> Result<V, OptError<V::Error>>
    where
        'b: 'a,
        V: FixValue<'b>,
        F: IsFieldDefinition,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fvl_with_key<'b, V>(&'b self, key: Self::Key) -> OptResult<V, V::Error>
    where
        V: FixValue<'b>,
    {
        match self
            .fv_raw_with_key(key)
            .map(|raw| V::deserialize_lossy(raw))
        {
            Some(Ok(x)) => Ok(x),
            Some(Err(e)) => Err(OptError::Other(e)),
            None => Err(OptError::None),
        }
    }
}
