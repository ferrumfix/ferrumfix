use fefix::{codegen, AppVersion, Dictionary};

fn main() {
    let dict = Dictionary::from_version(AppVersion::Fix42);
    let code = codegen::fields(dict, "fefix");
    println!("{}", code);
}
