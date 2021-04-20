use fefix::{AppVersion, Dictionary};
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let path = project_root().join("src").join("generated_fix44.rs");
    let file = &mut File::create(path)?;
    let fix_dictionary = Dictionary::from_version(AppVersion::Fix44);
    let rust_code = fefix::codegen::fields(fix_dictionary, "fefix");
    file.write_all(rust_code.as_bytes())?;
    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
