//! `fasters` is a Rust library that implements the FAST 1.2 protocol.

#![allow(dead_code)]

mod codec;
mod err;
mod fixml;
mod settings;
mod template;

pub use crate::err::Result;
pub use crate::template::Template;

use std::io;

pub trait Codec<R, W> {
    type Message;

    fn new_stdio() -> Self;

    fn write_message(&mut self, msg: Self::Message) -> io::Result<()>;

    fn read_message(&self) -> Result<Option<Self::Message>>;
}
