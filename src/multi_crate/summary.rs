//! mdBook SUMMARY.md generator.
//!
//! This module provides [`SummaryGenerator`] which creates a SUMMARY.md file
//! compatible with mdBook for multi-crate documentation.

use std::fmt::Write;
use std::path::Path;

use fs_err as FsErr;
use rustdoc_types::{ItemEnum, Visibility};

use crate::error::Error;
use crate::multi_crate::CrateCollection;

/// Generates mdBook-compatible SUMMARY.md file.
///
/// Creates a table of contents linking all crates and their modules,
/// allowing the documentation to be built as an mdBook site.
///
/// # Output Format
///
/// ```markdown
/// # Summary
///
/// - [tracing](tracing/index.md)
///   - [span](tracing/span/index.md)
///   - [field](tracing/field/index.md)
/// - [tracing_core](tracing_core/index.md)
///   - [subscriber](tracing_core/subscriber/index.md)
/// ```
pub struct SummaryGenerator<'a> {
    /// Collection of crates to document.
    crates: &'a CrateCollection,

    /// Output directory for SUMMARY.md.
    output_dir: &'a Path,

    /// Whether to include private items.
    include_private: bool,
}

impl<'a> SummaryGenerator<'a> {
    /// Create a new summary generator.
    ///
    /// # Arguments
    ///
    /// * `crates` - Collection of parsed crates
    /// * `output_dir` - Directory to write SUMMARY.md
    /// * `include_private` - Whether to include private modules
    #[must_use]
    pub const fn new(
        crates: &'a CrateCollection,
        output_dir: &'a Path,
        include_private: bool,
    ) -> Self {
        Self {
            crates,
            output_dir,
            include_private,
        }
    }

    /// Generate the SUMMARY.md file.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn generate(&self) -> Result<(), Error> {
        let mut content = String::from("# Summary\n\n");

        for (crate_name, krate) in self.crates.iter() {
            // Add crate entry
            _ = writeln!(content, "- [{crate_name}]({crate_name}/index.md)");

            // Add module entries
            if let Some(root) = krate.index.get(&krate.root)
                && let ItemEnum::Module(module) = &root.inner
            {
                self.add_modules(&mut content, krate, &module.items, crate_name, 1);
            }
        }

        let summary_path = self.output_dir.join("SUMMARY.md");
        FsErr::write(&summary_path, content).map_err(Error::FileWrite)?;

        Ok(())
    }

    /// Add module entries recursively.
    fn add_modules(
        &self,
        content: &mut String,
        krate: &rustdoc_types::Crate,
        items: &[rustdoc_types::Id],
        path_prefix: &str,
        indent: usize,
    ) {
        // Collect and sort modules alphabetically, filtering by visibility
        let mut modules: Vec<_> = items
            .iter()
            .filter_map(|id| krate.index.get(id))
            .filter(|item| matches!(&item.inner, ItemEnum::Module(_)))
            .filter(|item| {
                // Include item if include_private is true OR item is public
                self.include_private || matches!(item.visibility, Visibility::Public)
            })
            .collect();

        modules.sort_by_key(|item| item.name.as_deref().unwrap_or(""));

        for item in modules {
            let name = item.name.as_deref().unwrap_or("unnamed");
            let indent_str = "  ".repeat(indent);
            let link_path = format!("{path_prefix}/{name}/index.md");

            _ = writeln!(content, "{indent_str}- [{name}]({link_path})");

            // Recurse into child modules
            if let ItemEnum::Module(module) = &item.inner {
                let child_prefix = format!("{path_prefix}/{name}");

                self.add_modules(content, krate, &module.items, &child_prefix, indent + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_generation() {
        // Basic test that the generator can be constructed
        let crates = CrateCollection::new();
        let temp_dir = std::env::temp_dir();
        let generator = SummaryGenerator::new(&crates, &temp_dir, false);

        // Empty crates should produce minimal output
        assert!(generator.crates.is_empty());
    }

    #[test]
    fn test_summary_respects_include_private() {
        // Verify the flag is stored correctly
        let crates = CrateCollection::new();
        let temp_dir = std::env::temp_dir();

        let public_only = SummaryGenerator::new(&crates, &temp_dir, false);
        assert!(!public_only.include_private);

        let with_private = SummaryGenerator::new(&crates, &temp_dir, true);
        assert!(with_private.include_private);
    }
}
