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

mod context;
mod flat;
mod impls;
mod items;
mod module;
mod nested;

pub use context::GeneratorContext;

use crate::Args;
use crate::CliOutputFormat;
use crate::error::Error;
use flat::FlatGenerator;
use fs_err as fs;
use indicatif::{ProgressBar, ProgressStyle};
use nested::NestedGenerator;
use rustdoc_types::{Crate, Item};

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
    pub fn generate(&self) -> Result<(), Error> {
        // Ensure the output directory exists
        fs::create_dir_all(&self.args.output).map_err(Error::CreateDir)?;

        // Set up progress bar
        let total_modules = self.ctx.count_modules(self.root_item) + 1;
        let progress = Self::create_progress_bar(total_modules);

        // Dispatch to format-specific generator
        match self.args.format {
            CliOutputFormat::Flat => {
                let generator = FlatGenerator::new(&self.ctx, &self.args.output, &progress);
                generator.generate(self.root_item)?;
            }
            CliOutputFormat::Nested => {
                let generator = NestedGenerator::new(&self.ctx, &self.args.output, &progress);
                generator.generate(self.root_item)?;
            }
        }

        progress.finish_with_message("done");
        Ok(())
    }

    /// Create a progress bar for user feedback.
    fn create_progress_bar(total: usize) -> ProgressBar {
        let progress = ProgressBar::new(total as u64);
        progress.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} modules",
            )
            .unwrap()
            .progress_chars("=>-"),
        );
        progress
    }
}

/// Generate markdown documentation from a parsed rustdoc JSON crate.
///
/// This is a convenience function that creates a [`Generator`] and runs it.
/// For more control over the generation process, use [`Generator`] directly.
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
/// # Example
///
/// ```ignore
/// use docs_md::generator::generate;
///
/// generate(&krate, &args)?;
/// ```
pub fn generate(krate: &Crate, args: &Args) -> Result<(), Error> {
    let generator = Generator::new(krate, args)?;
    generator.generate()
}
