extern crate fasters;
use fasters::internals::{codegen, Dictionary, Version};
use std::fs;

const PATH: &str = "src/generated_fix44.rs";

fn main() {
    fs::File::create(PATH).unwrap();
    let dictionary = Dictionary::new(Version::Fix44);
    let code = codegen(dictionary);
    fs::write(PATH, code).unwrap();
}
