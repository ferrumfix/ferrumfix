use crate::codec::Codec;
use crate::{Message, Result};

pub struct Nullable<U>(FieldInner<U>);

impl<U: Operator> Nullable<U> {
    pub fn new(op: U) -> Self {
        Nullable(FieldInner {
            index_within_template: 0,
            op,
        })
    }
}

pub struct Mandatory<U>(FieldInner<U>);

impl<U: Operator> Mandatory<U> {
    pub fn new(op: U) -> Self {
        Mandatory(FieldInner {
            index_within_template: 0,
            op,
        })
    }
}
pub struct FieldInner<U> {
    index_within_template: usize,
    op: U,
}

pub trait Field<U: Operator> {
    type Item;

    fn op(&self) -> &U;
    fn op_mut(&mut self) -> &mut U;
    fn encode_to_message(&mut self, value: Self::Item, msg: &mut Message) -> Result<()>;
}

impl<U: Operator> Field<U> for Nullable<U> {
    type Item = Option<U::Item>;

    fn op(&self) -> &U {
        &self.0.op
    }

    fn op_mut(&mut self) -> &mut U {
        &mut self.0.op
    }

    fn encode_to_message(&mut self, value: Self::Item, msg: &mut Message) -> Result<()> {
        if let Some(value) = value {
            if !self.op().can_omit(&value) {
                value.encode_to(&mut msg.payload)?;
                self.op_mut().replace(value);
            }
        }
        Ok(())
    }
}

impl<U: Operator> Field<U> for Mandatory<U> {
    type Item = U::Item;

    fn op(&self) -> &U {
        &self.0.op
    }

    fn op_mut(&mut self) -> &mut U {
        &mut self.0.op
    }

    fn encode_to_message(&mut self, value: Self::Item, msg: &mut Message) -> Result<()> {
        if !self.op().can_omit(&value) {
            value.encode_to(&mut msg.payload)?;
            self.op_mut().replace(value);
        }
        Ok(())
    }
}
