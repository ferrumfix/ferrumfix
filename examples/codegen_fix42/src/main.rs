use fasters::Dictionary;
use fasters::app::Version;

fn main() {
    let dict = Dictionary::from_version(Version::Fix42);
    let code = fasters::codegen(&dict);
    println!("{}", code);
}
