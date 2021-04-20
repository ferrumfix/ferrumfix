mod src;

use src as fefix_barebones;

use fefix_barebones::codegen;
use fefix_barebones::AppVersion;
use rayon::prelude::*;
use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let generated_path = project_root().join("src").join("fields");
    create_dir_all(generated_path.as_path())?;
    fefix_barebones::AppVersion::ALL
        .par_iter()
        .copied()
        .try_for_each::<_, std::io::Result<()>>(|app_version| {
            codegen(app_version, generated_path.clone())
        })?;
    Ok(())
}

fn codegen(app_version: AppVersion, dir: PathBuf) -> io::Result<()> {
    let fix_dictionary = fefix_barebones::dictionary::Dictionary::from_version(app_version);
    let mut filename: String = app_version
        .name()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();
    filename.push_str(".rs");
    let code = codegen::fields(fix_dictionary, "crate");
    let path = dir.join(filename);
    let mut file = File::create(path)?;
    file.write_all(code.as_bytes())?;
    println!("{}", code);
    Ok(())
}

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
