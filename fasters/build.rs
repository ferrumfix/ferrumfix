extern crate fasters_internals;

use fasters_internals::{codegen, Dictionary, Version};
use std::fs;

fn main() {
    for version in Version::iter_supported() {
        generate_dictionary(version);
    }
}

fn generate_dictionary(version: Version) {
    let dictionary = Dictionary::new(version);
    let code = codegen(dictionary);
    let filename = format!("src/generated/generated_{}.rs", version.module_name());
    println!("writing to {}, code is {}", filename, code);
    fs::write(filename, code).unwrap();
}
