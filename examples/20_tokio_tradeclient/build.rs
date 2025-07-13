use rustyfix::{codegen, Dictionary};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let fix_dictionary = Dictionary::fix42().expect("Failed to load FIX 4.2 dictionary");
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
            "::strum::EnumIter".to_string(),
            "::strum::IntoStaticStr".to_string(),
        ];
        codegen::gen_definitions(&fix_dictionary, &settings)
    };
    let mut file = {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("generated_fix42.rs");
        File::create(path)?
    };
    file.write_all(rust_code.as_bytes())?;
    Ok(())
}
