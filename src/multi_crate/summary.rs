//! mdBook SUMMARY.md generator.
//!
//! This module provides [`SummaryGenerator`] which creates a SUMMARY.md file
//! compatible with mdBook for multi-crate documentation.

use crate::error::Error;
use crate::multi_crate::CrateCollection;
use fs_err as fs;
use rustdoc_types::ItemEnum;
use std::fmt::Write;
use std::path::Path;

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
}

impl<'a> SummaryGenerator<'a> {
    /// Create a new summary generator.
    ///
    /// # Arguments
    ///
    /// * `crates` - Collection of parsed crates
    /// * `output_dir` - Directory to write SUMMARY.md
    #[must_use]
    pub fn new(crates: &'a CrateCollection, output_dir: &'a Path) -> Self {
        Self { crates, output_dir }
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
        fs::write(&summary_path, content).map_err(Error::FileWrite)?;

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
        // Collect and sort modules alphabetically
        let mut modules: Vec<_> = items
            .iter()
            .filter_map(|id| krate.index.get(id))
            .filter(|item| matches!(&item.inner, ItemEnum::Module(_)))
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
        let generator = SummaryGenerator::new(&crates, &temp_dir);

        // Empty crates should produce minimal output
        assert!(generator.crates.is_empty());
    }
}
