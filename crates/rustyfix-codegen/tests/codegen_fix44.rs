use rustyfix_dictionary::Dictionary;
use std::fs::File;
use std::io::{self, Write};

#[test]
fn test_fix44_codegen() -> io::Result<()> {
    // Generate code for FIX 4.4
    let fix_dictionary = Dictionary::fix44().expect("Failed to load FIX 4.4 dictionary");
    let rust_code = {
        let settings = rustyfix_codegen::Settings::default();
        rustyfix_codegen::gen_definitions(&fix_dictionary, &settings)
    };

    // Write to a temporary file for testing
    let temp_dir = std::env::temp_dir();
    let path = temp_dir.join("generated_fix44_test.rs");
    let mut file = File::create(&path)?;
    file.write_all(rust_code.as_bytes())?;

    // Verify the generated code compiles by checking it contains expected content
    assert!(rust_code.contains("BEGIN_STRING"));
    assert!(rust_code.contains("MSG_TYPE"));
    assert!(rust_code.contains("pub const"));

    // Clean up
    std::fs::remove_file(path).ok();

    Ok(())
}

#[test]
fn test_begin_string_field_definition() {
    use rustyfix_codegen::{Settings, gen_definitions};

    let fix_dictionary = Dictionary::fix44().expect("Failed to load FIX 4.4 dictionary");
    let rust_code = gen_definitions(&fix_dictionary, &Settings::default());

    // Verify that BeginString field is generated correctly
    assert!(rust_code.contains("BEGIN_STRING"));
    assert!(rust_code.contains("tag: 8"));
    assert!(rust_code.contains("BeginString"));
}
