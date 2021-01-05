//! Support for FIX-related session protocols (OSI Layer 5).

mod classic;
mod fixp;

pub use classic::{Processor, ProcessorBuilder};
