//! Source code parsing for enhanced documentation.
//!
//! This module provides functionality to collect and parse Rust source code
//! from dependencies, extracting information not available in rustdoc JSON:
//!
//! - Function bodies and implementation details
//! - Private items (functions, structs, etc.)
//! - Constant and static values
//! - Macro definitions
//! - Test examples
//!
//! # Architecture
//!
//! The source parsing system has four main components:
//!
//! 1. [`SourceCollector`] - Collects dependency sources to `.source_*/`
//! 2. [`SourceLocator`] - Finds crate sources in the Cargo registry
//! 3. [`SourceParser`] - Parses Rust source files using `syn`
//! 4. [`types`] - Data structures for parsed source information
//!
//! # Workflow
//!
//! ```no_run
//! use cargo_docs_md::source::{SourceCollector, CollectOptions};
//!
//! // Collect dependency sources to .source_{timestamp}/
//! let collector = SourceCollector::new()?;
//! let result = collector.collect(&CollectOptions::default())?;
//! println!("Collected {} crates to {}", result.crates_collected, result.output_dir.display());
//! # Ok::<(), cargo_docs_md::error::Error>(())
//! ```
//!
//! # Feature Flag
//!
//! This module requires the `source-parsing` feature:
//!
//! ```toml
//! cargo-docs-md = { version = "0.1", features = ["source-parsing"] }
//! ```

use std::fs;
use std::path::{Path, PathBuf};
use std::result::Result;

mod collector;
mod integration;
mod locator;
mod parser;
pub mod types;

pub use collector::{
    CollectOptions, CollectedCrate, CollectionResult, SourceCollector, SourceManifest,
};
pub use locator::SourceLocator;
pub use parser::SourceParser;
pub use types::{
    ConstInfo, CrateSource, EnumInfo, FieldInfo, FunctionInfo, ImplInfo, MacroInfo, PrivateItem,
    StaticInfo, StructInfo, TraitInfo, TypeAliasInfo, VariantInfo,
};

/// Find the most recent `.source_*` directory in the given root.
///
/// Scans for directories matching `.source_*` pattern and returns
/// the one with the highest timestamp (most recent).
///
/// # Arguments
///
/// * `root` - Directory to search in (typically workspace or project root)
///
/// # Returns
///
/// Path to the most recent `.source_*` directory, or `None` if not found.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use cargo_docs_md::source::find_source_dir;
///
/// if let Some(source_dir) = find_source_dir(Path::new(".")) {
///     println!("Found source directory: {}", source_dir.display());
/// }
/// ```
#[must_use]
pub fn find_source_dir(root: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(root).ok()?;

    let mut source_dirs: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.is_dir()
                && p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with(".source_"))
        })
        .collect();

    // Sort by timestamp descending (higher timestamp = more recent)
    // Directory names are `.source_{timestamp}` so lexicographic sort works
    source_dirs.sort_by(|a, b| {
        let ts_a = extract_source_timestamp(a);
        let ts_b = extract_source_timestamp(b);
        ts_b.cmp(&ts_a)
    });

    source_dirs.into_iter().next()
}

/// Extract timestamp from a `.source_*` directory path.
///
/// Returns the numeric timestamp suffix, or 0 if parsing fails.
fn extract_source_timestamp(path: &Path) -> u64 {
    path.file_name()
        .and_then(|n| n.to_str())
        .and_then(|s| s.strip_prefix(".source_"))
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fs as StdFs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn find_source_dir_returns_none_for_empty_dir() {
        let temp = TempDir::new().unwrap();
        let result = find_source_dir(temp.path());
        assert!(result.is_none());
    }

    #[test]
    fn find_source_dir_finds_single_source_dir() {
        let temp = TempDir::new().unwrap();
        StdFs::create_dir(temp.path().join(".source_12345")).unwrap();

        let result = find_source_dir(temp.path());
        assert!(result.is_some());
        assert!(result.unwrap().ends_with(".source_12345"));
    }

    #[test]
    fn find_source_dir_returns_most_recent() {
        let temp = TempDir::new().unwrap();
        StdFs::create_dir(temp.path().join(".source_10000")).unwrap();
        StdFs::create_dir(temp.path().join(".source_99999")).unwrap();
        StdFs::create_dir(temp.path().join(".source_50000")).unwrap();

        let result = find_source_dir(temp.path());
        assert!(result.is_some());
        assert!(result.unwrap().ends_with(".source_99999"));
    }

    #[test]
    fn find_source_dir_ignores_non_source_dirs() {
        let temp = TempDir::new().unwrap();
        StdFs::create_dir(temp.path().join("source_12345")).unwrap(); // No dot
        StdFs::create_dir(temp.path().join(".sourcecode")).unwrap(); // No underscore
        StdFs::create_dir(temp.path().join("src")).unwrap();

        let result = find_source_dir(temp.path());
        assert!(result.is_none());
    }

    #[test]
    fn extract_source_timestamp_parses_valid() {
        let path = PathBuf::from("/project/.source_1733660400");
        assert_eq!(extract_source_timestamp(&path), 1_733_660_400);
    }

    #[test]
    fn extract_source_timestamp_returns_zero_for_invalid() {
        assert_eq!(
            extract_source_timestamp(&PathBuf::from("/project/.source_abc")),
            0
        );
        assert_eq!(
            extract_source_timestamp(&PathBuf::from("/project/source_123")),
            0
        );
    }
}
