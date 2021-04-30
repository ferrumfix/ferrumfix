//! *FIX Adapted for Streaming*
//! ([FAST](https://www.fixtrading.org/standards/fast/)) support.

use crate::Dictionary;
use std::collections::HashMap;
use template::Template;

mod codec;
mod codegen;
mod decimal;
mod dtf;
mod errors;
mod field_operators;
mod template;

pub use codec::{Codec, PresenceMap};
pub use codegen::template_struct as codegen_template_struct;
pub use decimal::Decimal;
pub use errors::{DynamicError, Error, ReportableError, StaticError};
pub use field_operators::*;
pub use template::*;

#[derive(Clone, Debug)]
pub struct Fast {
    dict: Dictionary,
    templates: HashMap<String, Template>,
}

impl Fast {
    /// Builds a new `TagValue` encoding device with an empty FIX dictionary.
    pub fn new() -> Self {
        let dict = Dictionary::empty();
        Fast {
            dict,
            templates: HashMap::new(),
        }
    }

    pub fn with_template(mut self, template: Template) -> Self {
        self.templates.insert(template.name().to_string(), template);
        self
    }
}
