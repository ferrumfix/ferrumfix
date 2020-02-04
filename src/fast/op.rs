use crate::codec::Codec;
use std::marker::PhantomData;
use std::ops::Sub;

/// *Field encoding operator* in FAST terminology.
pub trait Operator {
    /// The type of the (de)serializable item.
    type Item: Codec;

    /// See section 6.3.1 of FAST 1.1 documentation.
    fn previous_value(&self) -> Option<&Self::Item>;
    /// Replace the previous value (or set it if unset) with a new one.
    fn replace(&mut self, new_value: Self::Item);
    /// Determine whether the specified value can be omitted from the final
    /// payload. This behavior is custom to every field operator.
    fn can_omit(&self, value: &Self::Item) -> bool;
    /// Unset previous state.
    fn reset(&mut self);
}

/// The constant operator specifies that the value of a field will always be the
/// same, as initialized with `new`.
pub struct Constant<T>(T);

impl<T> Constant<T> {
    pub fn new(value: T) -> Self {
        Constant(value)
    }
}

impl<T: Codec> Operator for Constant<T> {
    type Item = T;

    fn previous_value(&self) -> Option<&T> {
        Some(&self.0)
    }

    fn can_omit(&self, _value: &T) -> bool {
        true
    }

    fn replace(&mut self, _new_value: T) {}

    fn reset(&mut self) {}
}

pub struct Delta<T, U> {
    prev: Option<T>,
    delta: U,
}

impl<U, T: Codec> Operator for Delta<T, U>
where
    U: PartialEq,
    T: Sub<Output = U> + std::marker::Copy,
{
    type Item = T;

    fn previous_value(&self) -> Option<&T> {
        self.prev.as_ref()
    }

    fn can_omit(&self, value: &T) -> bool {
        if let Some(prev) = self.prev {
            *value - prev == self.delta
        } else {
            false
        }
    }

    fn replace(&mut self, new_value: T) {
        self.prev = Some(new_value)
    }

    fn reset(&mut self) {
        self.prev = Option::None
    }
}

pub struct Copy<T> {
    prev: Option<T>,
}

impl<T: Codec> Operator for Copy<T>
where
    T: PartialEq + std::marker::Copy,
{
    type Item = T;

    fn previous_value(&self) -> Option<&T> {
        self.prev.as_ref()
    }

    fn can_omit(&self, value: &T) -> bool {
        if let Some(previous_value) = self.previous_value() {
            value == previous_value
        } else {
            false
        }
    }

    fn replace(&mut self, new_value: T) {
        self.prev = Some(new_value)
    }

    fn reset(&mut self) {
        self.prev = Option::None
    }
}

pub struct None<T>(PhantomData<T>);

impl<T> Default for None<T> {
    fn default() -> Self {
        None(PhantomData)
    }
}

impl<T: Codec> Operator for None<T> {
    type Item = T;

    fn previous_value(&self) -> Option<&T> {
        Option::None
    }

    fn can_omit(&self, _value: &T) -> bool {
        false
    }

    fn replace(&mut self, _new_value: T) {}

    fn reset(&mut self) {}
}
