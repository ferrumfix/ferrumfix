use rustyfix::{codegen, Dictionary};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let quickfix_spec_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("coinbase_quickfix.xml");

    let spec_content = std::fs::read_to_string(&quickfix_spec_path)
        .expect("Failed to read Coinbase QuickFIX specification file");

    let fix_dictionary = Dictionary::from_quickfix_spec(&spec_content)
        .expect("Failed to parse Coinbase QuickFIX specification");

    let rust_code = {
        let mut settings = codegen::Settings::default();
        settings.derives_for_allowed_values = vec![
            "Debug".to_string(),
            "Copy".to_string(),
            "Clone".to_string(),
            "PartialEq".to_string(),
            "Eq".to_string(),
            "Hash".to_string(),
            "::rustyfix::FieldType".to_string(),
        ];
        codegen::gen_definitions(&fix_dictionary, &settings)
    };

    let mut file = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("generated_coinbase.rs");
        File::create(path)?
    };
    file.write_all(rust_code.as_bytes())?;

    Ok(())
}
