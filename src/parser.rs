use crate::error::Error;
use fs_err as fs;
use rustdoc_types::Crate;
use std::path::Path;

/// Parse a rustdoc JSON file into a Crate structure
pub fn parse_json(path: &Path) -> Result<Crate, Error> {
    let content = fs::read_to_string(path).map_err(Error::FileRead)?;
    parse_json_string(&content)
}

/// Parse a rustdoc JSON string into a Crate structure
pub fn parse_json_string(content: &str) -> Result<Crate, Error> {
    let krate: Crate = serde_json::from_str(content).map_err(Error::JsonParse)?;
    Ok(krate)
}
