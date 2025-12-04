//! Search index generation for multi-crate documentation.
//!
//! This module provides [`SearchIndexGenerator`] which creates a JSON search index
//! containing all documented items across multiple crates. The index can be used
//! with client-side search libraries like Fuse.js, Lunr.js, or FlexSearch.
//!
//! # Output Format
//!
//! The generated `search_index.json` contains:
//!
//! ```json
//! {
//!   "items": [
//!     {
//!       "name": "Span",
//!       "path": "tracing::span::Span",
//!       "kind": "struct",
//!       "crate": "tracing",
//!       "file": "tracing/span/index.md",
//!       "summary": "A handle representing a span..."
//!     }
//!   ]
//! }
//! ```
//!
//! # Usage
//!
//! ```ignore
//! let generator = SearchIndexGenerator::new(&crates);
//! generator.write(Path::new("docs/"))?;
//! ```

use crate::multi_crate::CrateCollection;
use rustdoc_types::{Crate, Id, ItemEnum, Visibility};
use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

/// A single searchable item in the index.
///
/// Contains all metadata needed for search and display in results.
#[derive(Debug, Serialize)]
pub struct SearchEntry {
    /// Item name (e.g., "Span", "spawn", "Error").
    pub name: String,

    /// Full path including crate (e.g., "tracing::span::Span").
    pub path: String,

    /// Item kind for filtering and display.
    ///
    /// One of: "mod", "struct", "enum", "trait", "fn", "type", "const", "macro"
    pub kind: &'static str,

    /// Crate this item belongs to.
    #[serde(rename = "crate")]
    pub crate_name: String,

    /// Relative file path to the markdown documentation.
    pub file: String,

    /// First line of documentation for preview in search results.
    ///
    /// `None` if the item has no documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

/// The complete search index containing all searchable items.
///
/// Serialized to `search_index.json` for client-side consumption.
#[derive(Debug, Serialize)]
pub struct SearchIndex {
    /// All searchable items across all crates.
    pub items: Vec<SearchEntry>,
}

/// Generator for multi-crate search indices.
///
/// Traverses all crates in a [`CrateCollection`] and builds a comprehensive
/// search index of all public items (or all items if `include_private` is set).
///
/// # Example
///
/// ```ignore
/// let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
/// let generator = SearchIndexGenerator::new(&crates, false);
/// generator.write(Path::new("docs/"))?;
/// ```
pub struct SearchIndexGenerator<'a> {
    /// Collection of crates to index.
    crates: &'a CrateCollection,

    /// Whether to include private items in the search index.
    ///
    /// When false (default), only public items are indexed.
    /// When true, all items regardless of visibility are indexed.
    include_private: bool,
}

impl<'a> SearchIndexGenerator<'a> {
    /// Create a new search index generator.
    ///
    /// # Arguments
    ///
    /// * `crates` - Collection of parsed crates to index
    /// * `include_private` - Whether to include non-public items
    #[must_use]
    pub fn new(crates: &'a CrateCollection, include_private: bool) -> Self {
        Self {
            crates,
            include_private,
        }
    }

    /// Generate the complete search index.
    ///
    /// Traverses all crates and collects searchable items including:
    /// - Modules
    /// - Structs
    /// - Enums
    /// - Traits
    /// - Functions
    /// - Type aliases
    /// - Constants
    /// - Macros
    ///
    /// Items are sorted alphabetically by name for consistent output.
    #[must_use]
    pub fn generate(&self) -> SearchIndex {
        let mut items = Vec::new();

        for (crate_name, krate) in self.crates.iter() {
            self.index_crate(&mut items, crate_name, krate);
        }

        // Sort by name for consistent, deterministic output
        items.sort_by(|a, b| a.name.cmp(&b.name));

        SearchIndex { items }
    }

    /// Write the search index to `search_index.json` in the output directory.
    ///
    /// # Arguments
    ///
    /// * `output_dir` - Directory where `search_index.json` will be written
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn write(&self, output_dir: &Path) -> std::io::Result<()> {
        let index = self.generate();
        let json = serde_json::to_string_pretty(&index)?;
        let path = output_dir.join("search_index.json");
        fs_err::write(path, json)?;
        Ok(())
    }

    /// Index all items in a single crate.
    fn index_crate(&self, items: &mut Vec<SearchEntry>, crate_name: &str, krate: &Crate) {
        // Build a map of item ID to module path for accurate path construction
        let path_map = self.build_path_map(krate);

        for (id, item) in &krate.index {
            let Some(name) = &item.name else { continue };

            // Filter by visibility unless include_private is set
            if !self.include_private && !matches!(item.visibility, Visibility::Public) {
                continue;
            }

            // Determine item kind and whether to include
            let kind = match &item.inner {
                ItemEnum::Module(_) => "mod",
                ItemEnum::Struct(_) => "struct",
                ItemEnum::Enum(_) => "enum",
                ItemEnum::Trait(_) => "trait",
                ItemEnum::Function(_) => "fn",
                ItemEnum::TypeAlias(_) => "type",
                ItemEnum::Constant { .. } => "const",
                ItemEnum::Macro(_) => "macro",
                // Skip other item types (impl blocks, fields, variants, etc.)
                _ => continue,
            };

            // Build full path (crate::module::Item)
            let module_path = path_map.get(id).cloned().unwrap_or_default();
            let full_path = if module_path.is_empty() {
                format!("{crate_name}::{name}")
            } else {
                format!("{crate_name}::{module_path}::{name}")
            };

            // Build file path based on module location
            let file = self.compute_file_path(crate_name, &module_path, kind);

            // Extract first line of documentation as summary
            let summary = item
                .docs
                .as_ref()
                .and_then(|d| d.lines().next())
                .map(str::to_string);

            items.push(SearchEntry {
                name: name.clone(),
                path: full_path,
                kind,
                crate_name: crate_name.to_string(),
                file,
                summary,
            });
        }
    }

    /// Build a map from item ID to its module path.
    ///
    /// This allows us to reconstruct the full path for each item.
    fn build_path_map(&self, krate: &Crate) -> HashMap<Id, String> {
        let mut path_map = HashMap::new();

        // Use the crate's paths table which maps IDs to their paths
        for (id, item_summary) in &krate.paths {
            // Skip external items (from other crates)
            if item_summary.crate_id != 0 {
                continue;
            }

            // Build path from components, excluding the item name itself
            let path_components = &item_summary.path;
            if path_components.len() > 1 {
                // Path without the crate name and without the item name
                let module_path = path_components[1..path_components.len() - 1].join("::");
                path_map.insert(*id, module_path);
            } else {
                path_map.insert(*id, String::new());
            }
        }

        path_map
    }

    /// Compute the file path for an item based on its module location.
    fn compute_file_path(&self, crate_name: &str, module_path: &str, kind: &str) -> String {
        if module_path.is_empty() {
            // Root-level item
            format!("{crate_name}/index.md")
        } else if kind == "mod" {
            // Module gets its own directory
            let path = module_path.replace("::", "/");
            format!("{crate_name}/{path}/index.md")
        } else {
            // Item within a module - link to the module's index
            let path = module_path.replace("::", "/");
            format!("{crate_name}/{path}/index.md")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_entry_serialization() {
        let entry = SearchEntry {
            name: "Span".to_string(),
            path: "tracing::span::Span".to_string(),
            kind: "struct",
            crate_name: "tracing".to_string(),
            file: "tracing/span/index.md".to_string(),
            summary: Some("A handle representing a span.".to_string()),
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert!(json.contains("\"name\":\"Span\""));
        assert!(json.contains("\"kind\":\"struct\""));
        assert!(json.contains("\"crate\":\"tracing\""));
    }

    #[test]
    fn test_search_entry_without_summary() {
        let entry = SearchEntry {
            name: "foo".to_string(),
            path: "crate::foo".to_string(),
            kind: "fn",
            crate_name: "crate".to_string(),
            file: "crate/index.md".to_string(),
            summary: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        // summary should be skipped when None
        assert!(!json.contains("summary"));
    }
}
