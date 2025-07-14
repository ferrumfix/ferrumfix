// `rustysbe` public API.

pub mod codegen;

pub mod sbe {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/sbe.rs"));
}
