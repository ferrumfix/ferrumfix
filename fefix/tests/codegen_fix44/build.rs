use fefix::Dictionary;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let path = project_root().join("src").join("generated_fix44.rs");
    let mut file = File::create(path)?;
    let fix_dictionary = Dictionary::fix44();
    let rust_code = {
        let settings = fefix::codegen::Settings::default();
        fefix::codegen::gen_definitions(fix_dictionary, &settings)
    };
    file.write_all(rust_code.as_bytes())?;
    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
