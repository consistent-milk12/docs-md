//! Flat format documentation generation.
//!
//! This module provides the [`FlatGenerator`] struct which generates markdown
//! documentation with a flat file structure where all files are in a single
//! directory and module hierarchy is encoded in filenames.

use crate::error::Error;
use crate::generator::context::GeneratorContext;
use crate::generator::module::ModuleRenderer;
use fs_err as fs;
use indicatif::ProgressBar;
use rustdoc_types::{Item, ItemEnum};
use std::path::Path;

/// Generates documentation with flat file structure.
///
/// All markdown files are placed in a single directory. Module hierarchy
/// is encoded in filenames using double underscores as separators.
///
/// # Output Structure
///
/// ```text
/// output/
/// ├── index.md              # Crate root
/// ├── module_a.md           # Top-level module
/// ├── module_b.md           # Top-level module
/// ├── module_a__child.md    # Nested module (module_a::child)
/// └── module_a__child__deep.md  # Deeply nested
/// ```
pub struct FlatGenerator<'a> {
    /// Shared generator context.
    ctx: &'a GeneratorContext<'a>,

    /// Output directory path.
    output_dir: &'a Path,

    /// Progress bar for user feedback.
    progress: &'a ProgressBar,
}

impl<'a> FlatGenerator<'a> {
    /// Create a new flat generator.
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

    /// Generate all documentation files in flat format.
    ///
    /// Generates `index.md` for the root module, then recursively generates
    /// files for all submodules with flattened names.
    pub fn generate(&self, root: &Item) -> Result<(), Error> {
        // Generate root module as index.md
        let renderer = ModuleRenderer::new(self.ctx, "index.md", true);
        let index_content = renderer.render(root);

        let index_path = self.output_dir.join("index.md");
        fs::write(&index_path, index_content).map_err(Error::FileWrite)?;
        self.progress.inc(1);

        // Generate each top-level submodule
        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                if let Some(item) = self.ctx.krate.index.get(item_id)
                    && let ItemEnum::Module(_) = &item.inner
                    && self.ctx.should_include_item(item)
                {
                    self.generate_module(item)?;
                }
            }
        }

        Ok(())
    }

    /// Generate a single module file and its children.
    ///
    /// Creates `{module_name}.md` in the output directory and recursively
    /// generates child modules with flattened names (e.g., `parent__child.md`).
    fn generate_module(&self, item: &Item) -> Result<(), Error> {
        let name = item.name.as_deref().unwrap_or("unnamed");
        let current_file = format!("{name}.md");

        let renderer = ModuleRenderer::new(self.ctx, &current_file, false);
        let content = renderer.render(item);

        let file_path = self.output_dir.join(&current_file);
        fs::write(&file_path, content).map_err(Error::FileWrite)?;
        self.progress.inc(1);

        // Recursively generate nested modules with flattened names
        if let ItemEnum::Module(module) = &item.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = self.ctx.krate.index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && self.ctx.should_include_item(sub_item)
                {
                    let sub_name = sub_item.name.as_deref().unwrap_or("unnamed");
                    let flat_name = format!("{name}__{sub_name}");
                    self.generate_module_recursive(sub_item, &flat_name)?;
                }
            }
        }

        Ok(())
    }

    /// Recursively generate nested module files with flattened names.
    ///
    /// # Arguments
    ///
    /// * `item` - The module item to generate
    /// * `prefix` - Accumulated path prefix (e.g., "`parent__child`")
    fn generate_module_recursive(&self, item: &Item, prefix: &str) -> Result<(), Error> {
        let current_file = format!("{prefix}.md");

        let renderer = ModuleRenderer::new(self.ctx, &current_file, false);
        let content = renderer.render(item);

        let file_path = self.output_dir.join(&current_file);
        fs::write(&file_path, content).map_err(Error::FileWrite)?;
        self.progress.inc(1);

        // Continue recursively for child modules
        if let ItemEnum::Module(module) = &item.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = self.ctx.krate.index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && self.ctx.should_include_item(sub_item)
                {
                    let sub_name = sub_item.name.as_deref().unwrap_or("unnamed");
                    let new_prefix = format!("{prefix}__{sub_name}");

                    self.generate_module_recursive(sub_item, &new_prefix)?;
                }
            }
        }

        Ok(())
    }
}
