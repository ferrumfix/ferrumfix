use fefix::backend::Version;
use fefix::Dictionary;

fn main() {
    let dict = Dictionary::from_version(Version::Fix42);
    let code = fefix::codegen(&dict);
    println!("{}", code);
}
