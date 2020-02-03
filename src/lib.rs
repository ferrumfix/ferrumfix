//! `fasters` is a Rust library that implements the FAST 1.2 protocol.

#![allow(dead_code)]

mod codec;
mod field;
#[cfg(gen)]
mod gen;
mod op;
mod result;

pub use crate::codec::Codec;
pub use crate::field::Field;
pub use crate::op::Operator;
pub use crate::result::Result;

use bitvec::vec::BitVec;
use std::io;

/// Templates are used to represent the structure of the data that is to be
/// encoded. A template represents a logical unit of data as it is transmitted
/// from sender to receiver. In other words, a template is used to represent a
/// specific message type. When planning to encode a data feed a user should
/// begin by converting standard message formats to templates.
///
/// Templates need to be communicated from sender to receiver. The originator of
/// the data is responsible for distributing the templates once they have been
/// defined.
#[derive(Clone, Debug)]
pub struct Template {
    /// Each template is assigned a Template ID that can be used to uniquely
    /// describe the format of an encoded message. A Template ID will be carried
    /// in every encoded message which will provide a link to the correct
    /// template for decoding.
    id: usize,
    number_of_fields: usize,
    presence_map: BitVec,
}

impl Template {
    /// Creates a template with its own id as the first field, but empty
    /// otherwise.
    pub fn with_id(id: usize) -> Self {
        Template {
            id,
            number_of_fields: 0,
            presence_map: BitVec::new(),
        }
    }

    /// Adds a new field specification to this template.
    pub fn register_field<U: Operator>(&mut self, f: &impl Field<U>) {
        unimplemented!()
    }

    // pub fn register_field_at<O: Operator>(self, field: &mut Field<O>, index:
    // usize) -> Self {     unimplemented!()
    // }

    fn next_field_index(&mut self) -> usize {
        self.number_of_fields += 1;
        self.number_of_fields
    }

    fn has_nth_presence_bit(&self, index: usize) -> bool {
        self.presence_map[index]
    }

    fn set_nth_presence_bit(&self, index: bool) {
        unimplemented!()
    }

    fn reset_presence_bits(&mut self) {
        unimplemented!()
    }
}

pub struct Faster<R, W> {
    input: R,
    output: W,
}

impl Faster<io::Stdin, io::Stdout> {
    pub fn stdio() -> Self {
        Faster {
            input: io::stdin(),
            output: io::stdout(),
        }
    }
}

impl<R: io::Read, W: io::Write> Faster<R, W> {
    pub fn io(input: R, output: W) -> Faster<R, W> {
        Faster { input, output }
    }

    pub fn write_message(&mut self, msg: Message) -> io::Result<()> {
        let presence_map: Vec<u8> = msg.presence_map.into();
        self.output.write_all(&presence_map[..])?;
        let template_id = msg.template.id as u32;
        template_id.encode_to_writer(&mut self.output)?;
        msg.payload.encode_to_writer(&mut self.output)
    }

    pub fn read_message(&self, template: &Template) -> Result<Option<Message>> {
        unimplemented!()
    }
}

pub struct Message<'t> {
    pub template: &'t Template,
    presence_map: BitVec,
    payload: Vec<u8>,
}

impl<'t> Message<'t> {
    /// Creates an empty message, compliant to the telected template.
    pub fn with_template(template: &'t Template) -> Self {
        Message {
            template,
            presence_map: BitVec::default(),
            payload: Vec::default(),
        }
    }

    pub fn encode<U, F>(&mut self, field: &mut F, value: F::Item) -> Result<()>
    where
        U: Operator,
        F: Field<U>,
    {
        field.encode_to_message(value, self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::field;

    #[test]
    fn create_template() {
        let mut faster = Faster::stdio();
        let template = Template::with_id(0);
        let order_id = op::None::<u32>::default();
        let mut field_order_id = field::Nullable::new(order_id);
        let mut msg = Message::with_template(&template);
        msg.encode(&mut field_order_id, Some(1337)).unwrap();
        faster.write_message(msg).unwrap();
    }
}
