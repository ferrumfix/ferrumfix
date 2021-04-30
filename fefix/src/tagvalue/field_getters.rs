use crate::{datatypes, dict};
use crate::{OptError, OptResult};

/// A trait to retrieve field values in a FIX message.
pub trait FieldGetter<'a> {
    type Key;

    fn fv_raw_with_key<'b>(&'b self, key: Self::Key) -> Option<&'b [u8]>;

    fn fv_raw<'b, F>(&'b self, field: &'a F) -> Option<&'b [u8]>
    where
        'b: 'a,
        F: dict::IsFieldDefinition;

    fn fv_opt<'b, V, F>(&'b self, field: &'a F) -> Option<Result<V, V::Error>>
    where
        'b: 'a,
        V: datatypes::DataType<'b>,
        F: dict::IsFieldDefinition,
    {
        self.fv_raw(field).map(|raw| match V::deserialize(raw) {
            Ok(value) => Ok(value),
            Err(err) => Err(err.into()),
        })
    }

    fn fv<'b, V, F>(&'b self, field: &'a F) -> OptResult<V, V::Error>
    where
        'b: 'a,
        V: datatypes::DataType<'b>,
        F: dict::IsFieldDefinition,
    {
        match self.fv_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fvu<'b, V, F>(&'b self, field: &'a F) -> OptResult<V, V::Error>
    where
        'b: 'a,
        V: datatypes::DataType<'b>,
        F: dict::IsFieldDefinition,
    {
        match self.fvu_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fvu_opt<'b, V, F>(&'b self, field: &'a F) -> Option<Result<V, V::Error>>
    where
        'b: 'a,
        V: datatypes::DataType<'b>,
        F: dict::IsFieldDefinition,
    {
        self.fv_raw(field).map(|raw| V::deserialize(raw))
    }

    fn fvl_opt<'b, V, F>(&'b self, field: &'a F) -> Option<Result<V, V::Error>>
    where
        'b: 'a,
        V: datatypes::DataType<'b>,
        F: dict::IsFieldDefinition,
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
        V: datatypes::DataType<'b>,
        F: dict::IsFieldDefinition,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fvtl<'b, V, F>(&'b self, field: &'a F) -> OptResult<V, V::Error>
    where
        'b: 'a,
        V: datatypes::DataType<'b>,
        F: dict::IsTypedFieldDefinition<V>,
    {
        match self.fvl_opt(field) {
            Some(Ok(x)) => Ok(x),
            Some(Err(err)) => Err(OptError::Other(err)),
            None => Err(OptError::None),
        }
    }

    fn fv_with_key<'b, V>(&'b self, key: Self::Key) -> OptResult<V, V::Error>
    where
        V: datatypes::DataType<'b>,
    {
        match self.fv_raw_with_key(key).map(|raw| V::deserialize(raw)) {
            Some(Ok(x)) => Ok(x),
            Some(Err(e)) => Err(OptError::Other(e)),
            None => Err(OptError::None),
        }
    }
}
