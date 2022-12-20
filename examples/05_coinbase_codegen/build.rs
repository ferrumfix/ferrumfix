use fefix::{codegen, Dictionary};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let fix_dictionary = coinbase_fix_dictionary()?;
    let rust_code = {
        let settings = codegen::Settings::default();
        codegen::gen_definitions(&fix_dictionary, &settings)
    };
    let mut file = {
        let path = project_root().join("src").join("gdax.rs");
        File::create(path)?
    };
    file.write_all(rust_code.as_bytes())?;
    Ok(())
}

fn coinbase_fix_dictionary() -> io::Result<Dictionary> {
    let quickfix_spec = {
        let path = project_root().join("src").join("coinbase_quickfix.xml");
        std::fs::read_to_string(path)?
    };
    Ok(Dictionary::from_quickfix_spec(&quickfix_spec).expect("Invalid specs."))
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
