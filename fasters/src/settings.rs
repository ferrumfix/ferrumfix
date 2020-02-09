use std::default::Default;

/// Code generation settings.
pub struct Settings {
    /// The target programming language.
    pub lang: ProgrammingLanguage,
    /// String case policy. All fields and names will be transformed accordingly.
    pub string_case: StringCase,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            lang: ProgrammingLanguage::Rust,
            string_case: StringCase::Unchanged,
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
