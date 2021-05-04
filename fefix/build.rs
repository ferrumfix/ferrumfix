// `fefix_core` is part of the FerrumFIX codebase, so all warnings will get
// caught anyway during compilation. Enabling compiler diagnostics would thus
// have the undesirable effect of showing two identical warnings.
#[allow(warnings)]
#[path = "src/fefix_core/mod.rs"]
mod fefix_core;

use fefix_core::{codegen, dict::Dictionary};
use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let dir_buf = project_root().join("src").join("definitions");
    let dir = dir_buf.as_path();
    create_dir_all(dir)?;
    #[cfg(feature = "fix40")]
    codegen(Dictionary::fix40(), "fix40.rs", dir)?;
    #[cfg(feature = "fix41")]
    codegen(Dictionary::fix41(), "fix41.rs", dir)?;
    #[cfg(feature = "fix42")]
    codegen(Dictionary::fix42(), "fix42.rs", dir)?;
    #[cfg(feature = "fix43")]
    codegen(Dictionary::fix43(), "fix43.rs", dir)?;
    #[cfg(feature = "fix44")]
    codegen(Dictionary::fix44(), "fix44.rs", dir)?;
    #[cfg(feature = "fix50")]
    codegen(Dictionary::fix50(), "fix50.rs", dir)?;
    #[cfg(feature = "fix50sp1")]
    codegen(Dictionary::fix50sp1(), "fix50sp1.rs", dir)?;
    #[cfg(feature = "fix50sp2")]
    codegen(Dictionary::fix50sp2(), "fix50sp2.rs", dir)?;
    #[cfg(feature = "fixt11")]
    codegen(Dictionary::fixt11(), "fixt11.rs", dir)?;
    Ok(())
}

fn codegen(fix_dictionary: Dictionary, filename: &str, dir: &Path) -> io::Result<()> {
    let code = codegen::module_with_field_definitions(fix_dictionary, "crate");
    let path = dir.to_path_buf().join(filename);
    let file = &mut File::create(path)?;
    file.write_all(code.as_bytes())?;
    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
