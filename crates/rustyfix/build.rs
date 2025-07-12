#![allow(dead_code)]

use rustyfix_codegen as codegen;
use rustyfix_dictionary::Dictionary;
use std::env::var;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "fix40")]
    codegen(Dictionary::fix40()?, "fix40.rs")?;
    #[cfg(feature = "fix41")]
    codegen(Dictionary::fix41()?, "fix41.rs")?;
    #[cfg(feature = "fix42")]
    codegen(Dictionary::fix42()?, "fix42.rs")?;
    #[cfg(feature = "fix43")]
    codegen(Dictionary::fix43()?, "fix43.rs")?;
    // FIX 4.4 is always enabled.
    codegen(Dictionary::fix44()?, "fix44.rs")?;
    #[cfg(feature = "fix50")]
    codegen(Dictionary::fix50()?, "fix50.rs")?;
    #[cfg(feature = "fix50sp1")]
    codegen(Dictionary::fix50sp1()?, "fix50sp1.rs")?;
    #[cfg(feature = "fix50sp2")]
    codegen(Dictionary::fix50sp2()?, "fix50sp2.rs")?;
    #[cfg(feature = "fixt11")]
    codegen(Dictionary::fixt11()?, "fixt11.rs")?;
    Ok(())
}

fn codegen(fix_dictionary: Dictionary, filename: &str) -> io::Result<()> {
    // All generated code must go in `OUT_DIR`. We avoid writing directly to
    // `src/` to avoid compilation issues on `crates.io`, which disallows
    // writing.
    let dir = PathBuf::from(var("OUT_DIR").unwrap());
    let codegen_settings = &mut codegen::Settings::default();
    codegen_settings.rustyfix_crate_name = "crate".to_string();
    let code = codegen::gen_definitions(&fix_dictionary, codegen_settings);
    let path = dir.join(filename);
    let file = &mut File::create(path)?;
    file.write_all(code.as_bytes())?;
    Ok(())
}
