use fefix::{AppVersion, Dictionary};

fn main() {
    let dict = Dictionary::from_version(AppVersion::Fix42);
    let code = fefix::codegen(&dict);
    println!("{}", code);
}
