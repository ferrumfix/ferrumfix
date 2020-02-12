use crate::repo::types::Datatype;
use std::default::Default;

/// Code generation settings.
pub struct Settings {
    /// The target programming language.
    pub lang: ProgrammingLanguage,
    /// String case policy. All fields and names will be transformed accordingly.
    pub string_case: StringCase,
    pub typer: Box<dyn Fn(Datatype) -> &'static str>,
}

fn default_typer(dt: Datatype) -> &'static str {
    match dt.base_type.unwrap_or(dt.name).as_str() {
        "char" => "char",
        "String" | "data" => "String",
        "int" => "i64",
        "float" => "f32",
        _ => panic!("Unexpected base type"),
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            lang: ProgrammingLanguage::Rust,
            string_case: StringCase::Unchanged,
            typer: Box::new(default_typer),
        }
    }
}

/// Programming languages supported by Fasters' for code generation.
pub enum ProgrammingLanguage {
    Rust,
}

pub enum StringCase {
    Unchanged,
    LowerCase,
    CamelCase,
    Custom(Box<dyn Fn(&str) -> &str>),
}
