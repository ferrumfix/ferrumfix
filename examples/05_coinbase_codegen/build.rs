use fefix::codegen;
use fefix::Dictionary;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let generated_path = project_root().join("src").join("gdax.rs");
    let spec = quickfix_spec()?;
    let fix_dictionary = Dictionary::from_quickfix_spec(spec).expect("Invalid specs.");
    let rust_code = codegen::module_with_field_definitions(fix_dictionary, "fefix");
    let mut file = File::create(generated_path)?;
    file.write_all(rust_code.as_bytes())?;
    Ok(())
}

fn quickfix_spec() -> io::Result<String> {
    let path = project_root().join("src").join("coinbase_quickfix.xml");
    std::fs::read_to_string(path)
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
