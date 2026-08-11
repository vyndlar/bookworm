use std::fs;

// Load library files
use color_eyre::eyre::Error;

use crate::content::library::Library;

// load library.toml
pub fn load() -> Result<Library, Error> {
    println!("Trying to access file...");
    let lib_str = fs::read_to_string("src/library.toml")?;
    println!("Trying to parse toml...");
    let lib: Library = toml::from_str(&lib_str)?;
    println!("Parsed toml.");
    Ok(lib)
}

pub fn save() -> Result<(), Error> {
    Ok(())
}
