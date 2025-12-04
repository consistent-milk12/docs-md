//! Nested format documentation generation.
//!
//! This module provides the [`NestedGenerator`] struct which generates markdown
//! documentation with a nested directory structure that mirrors the Rust module
//! hierarchy.

use std::path::Path;

use fs_err as fs;
use indicatif::ProgressBar;
use rustdoc_types::{Item, ItemEnum};

use crate::error::Error;
use crate::generator::breadcrumbs::BreadcrumbGenerator;
use crate::generator::context::GeneratorContext;
use crate::generator::module::ModuleRenderer;

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
    pub const fn new(
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
                    self.generate_module(item, self.output_dir, Vec::new())?;
                }
            }
        }

        Ok(())
    }

    /// Generate a single module directory with index.md and child modules.
    ///
    /// Creates a directory for the module and an `index.md` file inside it.
    /// Recursively creates subdirectories for child modules. Now tracks the
    /// full module path for breadcrumb generation.
    ///
    /// # Arguments
    ///
    /// * `item` - The module item to generate
    /// * `parent_dir` - Parent directory to create module directory in
    /// * `module_path` - Accumulated module path segments for breadcrumbs
    fn generate_module(
        &self,
        item: &Item,
        parent_dir: &Path,
        module_path: Vec<String>,
    ) -> Result<(), Error> {
        let name = item.name.as_deref().unwrap_or("unnamed");

        // Create directory for this module
        let module_dir = parent_dir.join(name);
        fs::create_dir_all(&module_dir).map_err(Error::CreateDir)?;

        // Build updated module path for breadcrumbs
        let mut current_path = module_path;
        current_path.push(name.to_string());

        // Compute the file path for link resolution
        let current_file = format!("{}/index.md", current_path.join("/"));

        // Get crate name for breadcrumbs
        let crate_name = self
            .ctx
            .krate
            .index
            .get(&self.ctx.krate.root)
            .and_then(|r| r.name.as_deref())
            .unwrap_or("crate");

        // Generate breadcrumbs
        let breadcrumb_gen = BreadcrumbGenerator::new(&current_path, crate_name);
        let breadcrumbs = breadcrumb_gen.generate();

        // Generate module content
        let renderer = ModuleRenderer::new(self.ctx, &current_file, false);
        let module_content = renderer.render(item);

        // Combine breadcrumbs + content
        let content = format!("{breadcrumbs}{module_content}");

        // Write index.md inside the module directory
        let file_path = module_dir.join("index.md");
        fs::write(&file_path, content).map_err(Error::FileWrite)?;
        self.progress.inc(1);

        // Recursively generate child modules as subdirectories
        if let ItemEnum::Module(module) = &item.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = self.ctx.krate.index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && self.ctx.should_include_item(sub_item)
                {
                    self.generate_module(sub_item, &module_dir, current_path.clone())?;
                }
            }
        }

        Ok(())
    }
}
