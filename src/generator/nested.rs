//! Nested format documentation generation.
//!
//! This module provides the [`NestedGenerator`] struct which generates markdown
//! documentation with a nested directory structure that mirrors the Rust module
//! hierarchy.

use crate::error::Error;
use crate::generator::context::GeneratorContext;
use crate::generator::module::ModuleRenderer;
use fs_err as fs;
use indicatif::ProgressBar;
use rustdoc_types::{Item, ItemEnum};
use std::path::Path;

/// Generates documentation with nested directory structure.
///
/// Each module gets its own directory with an `index.md` file.
/// This mirrors the Rust module hierarchy in the filesystem.
///
/// # Output Structure
///
/// ```text
/// output/
/// ├── index.md                  # Crate root
/// ├── module_a/
/// │   ├── index.md              # module_a docs
/// │   └── child/
/// │       └── index.md          # module_a::child docs
/// └── module_b/
///     └── index.md              # module_b docs
/// ```
pub struct NestedGenerator<'a> {
    /// Shared generator context.
    ctx: &'a GeneratorContext<'a>,

    /// Output directory path.
    output_dir: &'a Path,

    /// Progress bar for user feedback.
    progress: &'a ProgressBar,
}

impl<'a> NestedGenerator<'a> {
    /// Create a new nested generator.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Shared generator context
    /// * `output_dir` - Directory to write markdown files to
    /// * `progress` - Progress bar for user feedback
    pub fn new(
        ctx: &'a GeneratorContext<'a>,
        output_dir: &'a Path,
        progress: &'a ProgressBar,
    ) -> Self {
        Self {
            ctx,
            output_dir,
            progress,
        }
    }

    /// Generate all documentation files in nested format.
    ///
    /// Generates `index.md` for the root module in the output directory,
    /// then recursively creates subdirectories for each submodule.
    pub fn generate(&self, root: &Item) -> Result<(), Error> {
        // Generate root module as index.md in the output directory
        let renderer = ModuleRenderer::new(self.ctx, "index.md", true);
        let index_content = renderer.render(root);

        let index_path = self.output_dir.join("index.md");
        fs::write(&index_path, index_content).map_err(Error::FileWrite)?;
        self.progress.inc(1);

        // Generate each top-level submodule (creates directories and recurses)
        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                if let Some(item) = self.ctx.krate.index.get(item_id)
                    && let ItemEnum::Module(_) = &item.inner
                    && self.ctx.should_include_item(item)
                {
                    self.generate_module(item, self.output_dir, "")?;
                }
            }
        }

        Ok(())
    }

    /// Generate a single module directory with index.md and child modules.
    ///
    /// Creates a directory for the module and an `index.md` file inside it.
    /// Recursively creates subdirectories for child modules.
    ///
    /// # Arguments
    ///
    /// * `item` - The module item to generate
    /// * `parent_dir` - Parent directory to create module directory in
    /// * `path_prefix` - Accumulated path for link resolution (e.g., "parent/child")
    fn generate_module(
        &self,
        item: &Item,
        parent_dir: &Path,
        path_prefix: &str,
    ) -> Result<(), Error> {
        let name = item.name.as_deref().unwrap_or("unnamed");

        // Create directory for this module
        let module_dir = parent_dir.join(name);
        fs::create_dir_all(&module_dir).map_err(Error::CreateDir)?;

        // Compute the file path for link resolution
        let current_file = if path_prefix.is_empty() {
            format!("{name}/index.md")
        } else {
            format!("{path_prefix}/{name}/index.md")
        };

        let renderer = ModuleRenderer::new(self.ctx, &current_file, false);
        let content = renderer.render(item);

        // Write index.md inside the module directory
        let file_path = module_dir.join("index.md");
        fs::write(&file_path, content).map_err(Error::FileWrite)?;
        self.progress.inc(1);

        // Update the path prefix for child modules
        let new_prefix = if path_prefix.is_empty() {
            name.to_string()
        } else {
            format!("{path_prefix}/{name}")
        };

        // Recursively generate child modules as subdirectories
        if let ItemEnum::Module(module) = &item.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = self.ctx.krate.index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && self.ctx.should_include_item(sub_item)
                {
                    self.generate_module(sub_item, &module_dir, &new_prefix)?;
                }
            }
        }

        Ok(())
    }
}
