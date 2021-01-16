use fasters::prelude::*;

fn main() {
    let fix_v42 = Dictionary::from_version(fasters::app::Version::Fix42);
    let code = fasters::codegen(&fix_v42);
    println!("{}", code);
}
