//! Markdown documentation generator for rustdoc JSON.
//!
//! This is the core module that transforms rustdoc JSON data into markdown files.
//! It handles the complete generation pipeline: traversing modules, rendering
//! different item types, and creating cross-reference links.
//!
//! # Architecture
//!
//! The generation process follows these steps:
//!
//! 1. **Setup**: Create output directory, build path and impl maps
//! 2. **Link Registry**: Build a registry mapping item IDs to file paths
//! 3. **Generation**: Recursively traverse modules and write markdown files
//!
//! # Module Structure
//!
//! - [`context`] - Shared state for generation (crate data, maps, config)
//! - [`module`] - Module-level markdown rendering
//! - [`items`] - Individual item rendering (structs, enums, traits, etc.)
//! - [`impls`] - Implementation block rendering
//! - [`flat`] - Flat output format generator
//! - [`nested`] - Nested output format generator
//!
//! # Output Formats
//!
//! Two output formats are supported:
//!
//! - **Flat**: All files in one directory (`module.md`, `parent__child.md`)
//! - **Nested**: Directory hierarchy (`module/index.md`, `parent/child/index.md`)
//!
//! # Usage
//!
//! ```ignore
//! use docs_md::generator::Generator;
//!
//! let generator = Generator::new(&krate, &args)?;
//! generator.generate()?;
//! ```

pub mod breadcrumbs;
mod capture;
mod context;
pub mod doc_links;
mod flat;
pub mod impls;
mod items;
pub mod module;
mod nested;
pub mod render_shared;

pub use breadcrumbs::BreadcrumbGenerator;
pub use capture::MarkdownCapture;
pub use context::{GeneratorContext, ItemAccess, ItemFilter, LinkResolver, RenderContext};
pub use doc_links::{
    DocLinkProcessor, convert_html_links, convert_path_reference_links, strip_duplicate_title,
    strip_reference_definitions,
};
use flat::FlatGenerator;
use fs_err as fs;
use indicatif::{ProgressBar, ProgressStyle};
pub use module::ModuleRenderer;
use nested::NestedGenerator;
use rustdoc_types::{Crate, Item, ItemEnum};
use tracing::{debug, info, instrument};

use crate::error::Error;
use crate::{Args, CliOutputFormat};

/// Main documentation generator.
///
/// This struct orchestrates the entire documentation generation process,
/// coordinating between the context, format-specific generators, and
/// progress reporting.
///
/// # Example
///
/// ```ignore
/// let generator = Generator::new(&krate, &args)?;
/// generator.generate()?;
/// ```
pub struct Generator<'a> {
    /// Shared context containing crate data, maps, and configuration.
    ctx: GeneratorContext<'a>,

    /// CLI arguments containing output path and format options.
    args: &'a Args,

    /// The root module item of the crate.
    root_item: &'a Item,
}

impl<'a> Generator<'a> {
    /// Create a new generator for the given crate and arguments.
    ///
    /// This initializes the shared context including:
    /// - Path map (item ID → module path)
    /// - Impl map (type ID → impl blocks)
    /// - Link registry for cross-references
    ///
    /// # Arguments
    ///
    /// * `krate` - The parsed rustdoc JSON crate
    /// * `args` - CLI arguments containing output path, format, and options
    ///
    /// # Errors
    ///
    /// Returns an error if the root item cannot be found in the crate index.
    pub fn new(krate: &'a Crate, args: &'a Args) -> Result<Self, Error> {
        let root_item = krate
            .index
            .get(&krate.root)
            .ok_or_else(|| Error::ItemNotFound(krate.root.0.to_string()))?;

        let ctx = GeneratorContext::new(krate, args);

        Ok(Self {
            ctx,
            args,
            root_item,
        })
    }

    /// Generate markdown documentation.
    ///
    /// This is the main entry point for documentation generation. It:
    ///
    /// 1. Creates the output directory
    /// 2. Sets up a progress bar
    /// 3. Dispatches to the format-specific generator (flat or nested)
    ///
    /// # Errors
    ///
    /// Returns an error if any file operation fails.
    #[instrument(skip(self), fields(
        crate_name = %self.ctx.crate_name(),
        format = ?self.args.format,
        output = %self.args.output.display()
    ))]
    pub fn generate(&self) -> Result<(), Error> {
        info!("Starting single-crate documentation generation");

        // Ensure the output directory exists
        fs::create_dir_all(&self.args.output).map_err(Error::CreateDir)?;
        debug!(path = %self.args.output.display(), "Created output directory");

        // Set up progress bar
        let total_modules = self.ctx.count_modules(self.root_item) + 1;
        debug!(total_modules, "Counted modules for progress tracking");
        let progress = Self::create_progress_bar(total_modules)?;

        // Dispatch to format-specific generator
        match self.args.format {
            CliOutputFormat::Flat => {
                debug!("Using flat output format");
                let generator = FlatGenerator::new(&self.ctx, &self.args.output, &progress);
                generator.generate(self.root_item)?;
            },
            CliOutputFormat::Nested => {
                debug!("Using nested output format");
                let generator = NestedGenerator::new(&self.ctx, &self.args.output, &progress);
                generator.generate(self.root_item)?;
            },
        }

        progress.finish_with_message("done");
        info!("Single-crate documentation generation complete");
        Ok(())
    }

    /// Create a progress bar for user feedback.
    ///
    /// # Errors
    ///
    /// Returns an error if the progress bar template is invalid.
    fn create_progress_bar(total: usize) -> Result<ProgressBar, Error> {
        let progress = ProgressBar::new(total as u64);
        let style = ProgressStyle::with_template(
            "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} modules",
        )
        .map_err(Error::ProgressBarTemplate)?
        .progress_chars("=>-");
        progress.set_style(style);
        Ok(progress)
    }

    /// Generate documentation to memory instead of disk.
    ///
    /// This function mirrors `generate()` but captures all output in a
    /// `MarkdownCapture` struct instead of writing to the filesystem.
    /// Useful for testing and programmatic access to generated docs.
    ///
    /// # Arguments
    ///
    /// * `krate` - The parsed rustdoc JSON crate
    /// * `format` - Output format (Flat or Nested)
    /// * `include_private` - Whether to include private items
    ///
    /// # Returns
    ///
    /// A `MarkdownCapture` containing all generated markdown files.
    ///
    /// # Errors
    ///
    /// Returns an error if the root item cannot be found in the crate index.
    pub fn generate_to_capture(
        krate: &Crate,
        format: CliOutputFormat,
        include_private: bool,
    ) -> Result<MarkdownCapture, Error> {
        // Create a mock Args for the context
        let args = Args {
            path: None,
            dir: None,
            mdbook: false,
            search_index: false,
            primary_crate: None,
            output: std::path::PathBuf::new(),
            format,
            exclude_private: !include_private,
            include_blanket_impls: false,
        };

        let root_item = krate
            .index
            .get(&krate.root)
            .ok_or_else(|| Error::ItemNotFound(krate.root.0.to_string()))?;

        let ctx = GeneratorContext::new(krate, &args);
        let mut capture = MarkdownCapture::new();

        match format {
            CliOutputFormat::Flat => {
                Self::generate_flat_to_capture(&ctx, root_item, &mut capture)?;
            },
            CliOutputFormat::Nested => {
                Self::generate_nested_to_capture(&ctx, root_item, "", &mut capture)?;
            },
        }

        Ok(capture)
    }

    /// Generate flat structure to capture.
    fn generate_flat_to_capture(
        ctx: &GeneratorContext,
        root: &Item,
        capture: &mut MarkdownCapture,
    ) -> Result<(), Error> {
        // Generate root module
        let renderer = module::ModuleRenderer::new(ctx, "index.md", true);
        capture.insert("index.md".to_string(), renderer.render(root));

        // Generate submodules
        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                if let Some(item) = ctx.krate.index.get(item_id)
                    && let ItemEnum::Module(_) = &item.inner
                    && ctx.should_include_item(item)
                {
                    Self::generate_flat_recursive_capture(ctx, item, "", capture)?;
                }
            }
        }

        Ok(())
    }

    /// Recursive flat generation to capture.
    fn generate_flat_recursive_capture(
        ctx: &GeneratorContext,
        item: &Item,
        prefix: &str,
        capture: &mut MarkdownCapture,
    ) -> Result<(), Error> {
        let name = item.name.as_deref().unwrap_or("unnamed");
        let current_file = if prefix.is_empty() {
            format!("{name}.md")
        } else {
            format!("{prefix}__{name}.md")
        };

        let renderer = module::ModuleRenderer::new(ctx, &current_file, false);
        let content = renderer.render(item);
        capture.insert(current_file, content);

        let new_prefix = if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{prefix}__{name}")
        };

        if let ItemEnum::Module(module) = &item.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = ctx.krate.index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && ctx.should_include_item(sub_item)
                {
                    Self::generate_flat_recursive_capture(ctx, sub_item, &new_prefix, capture)?;
                }
            }
        }

        Ok(())
    }

    /// Generate nested structure to capture.
    fn generate_nested_to_capture(
        ctx: &GeneratorContext,
        root: &Item,
        path_prefix: &str,
        capture: &mut MarkdownCapture,
    ) -> Result<(), Error> {
        let name = root.name.as_deref().unwrap_or("unnamed");
        let is_root = path_prefix.is_empty()
            && name
                == ctx.krate.index[&ctx.krate.root]
                    .name
                    .as_deref()
                    .unwrap_or("");

        let current_file = if path_prefix.is_empty() {
            if is_root {
                "index.md".to_string()
            } else {
                format!("{name}/index.md")
            }
        } else {
            format!("{path_prefix}/{name}/index.md")
        };

        let renderer = module::ModuleRenderer::new(ctx, &current_file, is_root);
        capture.insert(current_file.clone(), renderer.render(root));

        let new_prefix = if path_prefix.is_empty() {
            if is_root {
                String::new()
            } else {
                name.to_string()
            }
        } else {
            format!("{path_prefix}/{name}")
        };

        if let ItemEnum::Module(module) = &root.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = ctx.krate.index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && ctx.should_include_item(sub_item)
                {
                    Self::generate_nested_to_capture(ctx, sub_item, &new_prefix, capture)?;
                }
            }
        }

        Ok(())
    }

    /// Convenience method to generate documentation in one call.
    ///
    /// Creates a `Generator` and runs it immediately. For more control
    /// over the generation process, use `new()` and `generate()` separately.
    ///
    /// # Arguments
    ///
    /// * `krate` - The parsed rustdoc JSON crate
    /// * `args` - CLI arguments containing output path, format, and options
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an error if any file operation fails.
    ///
    /// # Errors
    ///
    /// Returns an error if the root item cannot be found or if file operations fail.
    pub fn run(krate: &'a Crate, args: &'a Args) -> Result<(), Error> {
        let generator = Self::new(krate, args)?;
        generator.generate()
    }
}
