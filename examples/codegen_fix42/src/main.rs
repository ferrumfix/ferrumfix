use fasters::prelude::*;

fn main() {
    let dict = Dictionary::from_version(fasters::app::Version::Fix42);
    let code = fasters::codegen(&dict);
    println!("{}", code);
}
