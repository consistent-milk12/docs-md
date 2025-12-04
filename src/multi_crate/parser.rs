//! Multi-crate JSON parser.
//!
//! This module provides [`MultiCrateParser`] which scans a directory for
//! rustdoc JSON files and parses them into a [`CrateCollection`].

use crate::error::Error;
use crate::multi_crate::CrateCollection;
use crate::parser::Parser;
use rustdoc_types::ItemEnum;
use std::path::Path;
use walkdir::WalkDir;

/// Parser for multiple rustdoc JSON files in a directory.
///
/// Discovers JSON files and parses each one, extracting the crate name
/// from the root module item.
///
/// # Example
///
/// ```ignore
/// let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
/// println!("Found {} crates", crates.len());
/// ```
pub struct MultiCrateParser;

impl MultiCrateParser {
    /// Parse all rustdoc JSON files in a directory.
    ///
    /// Scans the top level of the directory for `*.json` files and
    /// attempts to parse each one as rustdoc JSON. Files that aren't
    /// valid rustdoc JSON (e.g., search indices) are silently skipped.
    ///
    /// # Arguments
    ///
    /// * `dir` - Path to directory containing JSON files
    ///
    /// # Returns
    ///
    /// A `CrateCollection` containing all successfully parsed crates.
    ///
    /// # Errors
    ///
    /// - [`Error::InvalidDirectory`] if the path is invalid
    /// - [`Error::NoJsonFiles`] if no valid JSON files found
    /// - [`Error::DuplicateCrate`] if multiple files define the same crate
    /// - [`Error::NoCrateName`] if a JSON file has no root module
    pub fn parse_directory(dir: &Path) -> Result<CrateCollection, Error> {
        // Validate directory exists and is readable
        if !dir.is_dir() {
            return Err(Error::InvalidDirectory(dir.display().to_string()));
        }

        let mut collection = CrateCollection::new();

        // Walk only the top level (max_depth 1 = directory itself + immediate children)
        for entry in WalkDir::new(dir).max_depth(1).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            // Skip non-JSON files
            if path.extension().is_some_and(|ext| ext != "json") {
                continue;
            }

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Try to parse as rustdoc JSON
            let krate = match Parser::parse_json(path) {
                Ok(k) => k,
                Err(_) => {
                    // Not a valid rustdoc JSON file, skip it
                    // (could be search-index.json or other artifacts)
                    continue;
                }
            };

            // Validate it's actually rustdoc JSON by checking for root module
            if krate.index.get(&krate.root).is_none() {
                continue;
            }

            // Extract crate name from root item
            let crate_name = Self::extract_crate_name(&krate, path)?;

            // Check for duplicates
            if collection.contains(&crate_name) {
                return Err(Error::DuplicateCrate(crate_name));
            }

            collection.insert(crate_name, krate);
        }

        // Ensure we found at least one crate
        if collection.is_empty() {
            return Err(Error::NoJsonFiles(dir.to_path_buf()));
        }

        Ok(collection)
    }

    /// Extract the crate name from a parsed Crate.
    ///
    /// The crate name is stored in the root item's `name` field.
    fn extract_crate_name(krate: &rustdoc_types::Crate, path: &Path) -> Result<String, Error> {
        let root_item = krate
            .index
            .get(&krate.root)
            .ok_or_else(|| Error::NoCrateName(path.to_path_buf()))?;

        // The root should be a Module
        if !matches!(&root_item.inner, ItemEnum::Module(_)) {
            return Err(Error::NoCrateName(path.to_path_buf()));
        }

        root_item
            .name
            .clone()
            .ok_or_else(|| Error::NoCrateName(path.to_path_buf()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_directory() {
        let result = MultiCrateParser::parse_directory(Path::new("/nonexistent/path"));
        assert!(matches!(result, Err(Error::InvalidDirectory(_))));
    }

    #[test]
    fn test_empty_directory() {
        // Create a temp directory with no JSON files
        let temp_dir = std::env::temp_dir().join("docs_md_test_empty");
        let _ = std::fs::create_dir_all(&temp_dir);

        let result = MultiCrateParser::parse_directory(&temp_dir);

        // Should fail with NoJsonFiles
        assert!(matches!(result, Err(Error::NoJsonFiles(_))));

        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
