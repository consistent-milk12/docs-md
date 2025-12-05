//! docs-md library interface for testing and reuse.
//!
//! This module exposes the core functionality of docs-md as a library,
//! allowing integration tests and external tools to use the markdown
//! generation capabilities programmatically.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

pub mod error;
pub mod generator;
pub mod linker;
pub mod multi_crate;
pub mod parser;
pub mod types;

pub use crate::generator::{Generator, MarkdownCapture};
pub use crate::linker::{LinkRegistry, slugify_anchor};
pub use crate::multi_crate::{
    CrateCollection, MultiCrateContext, MultiCrateGenerator, MultiCrateParser, SearchIndex,
    SearchIndexGenerator, UnifiedLinkRegistry,
};

/// Output format for the generated markdown documentation.
///
/// Controls how module files are organized in the output directory.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    /// Flat structure: all files in one directory.
    ///
    /// Module hierarchy is encoded in filenames using double underscores.
    /// Example: `parent__child__grandchild.md`
    #[default]
    Flat,

    /// Nested structure: directories mirror module hierarchy.
    ///
    /// Each module gets its own directory with an `index.md` file.
    /// Example: `parent/child/grandchild/index.md`
    Nested,
}

/// Command-line arguments for docs-md.
///
/// The tool accepts input from two mutually exclusive sources:
/// 1. A local rustdoc JSON file (`--path`)
/// 2. A crate name to fetch from docs.rs (`--crate-name`) - Note: currently non-functional
///    as docs.rs doesn't serve rustdoc JSON files publicly.
#[derive(Parser, Debug)]
#[command(name = "docs-md")]
#[command(
    author,
    version,
    about = "Generate per-module markdown from rustdoc JSON"
)]
pub struct Args {
    /// Path to a local rustdoc JSON file.
    ///
    /// Generate this file with: `cargo doc --output-format json`
    /// The JSON file will be in `target/doc/{crate_name}.json`
    ///
    /// Mutually exclusive with `--dir`.
    #[arg(
        short,
        long,
        required_unless_present_any = ["dir"],
        conflicts_with = "dir"
    )]
    pub path: Option<PathBuf>,

    /// Directory containing multiple rustdoc JSON files.
    ///
    /// Use this for multi-crate documentation generation. The tool will
    /// scan the directory for all `*.json` files (rustdoc format) and
    /// generate documentation for each crate with cross-crate linking.
    ///
    /// Generate JSON files with:
    /// `RUSTDOCFLAGS='-Z unstable-options --output-format json' cargo +nightly doc`
    ///
    /// Mutually exclusive with `--path`.
    #[arg(
        long,
        required_unless_present_any = ["path"],
        conflicts_with = "path"
    )]
    pub dir: Option<PathBuf>,

    /// Generate mdBook-compatible SUMMARY.md file.
    ///
    /// Only valid with `--dir` for multi-crate documentation.
    /// Creates a `SUMMARY.md` file in the output directory that can be
    /// used as the entry point for an mdBook documentation site.
    #[arg(long, requires = "dir", default_value_t = false)]
    pub mdbook: bool,

    /// Generate `search_index.json` for client-side search.
    ///
    /// Only valid with `--dir` for multi-crate documentation.
    /// Creates a `search_index.json` file containing all documented items,
    /// which can be used with client-side search libraries like Fuse.js,
    /// Lunr.js, or `FlexSearch`.
    #[arg(long, requires = "dir", default_value_t = false)]
    pub search_index: bool,

    /// Primary crate name for preferential link resolution.
    ///
    /// When specified with `--dir`, links to items in this crate take
    /// precedence over items with the same name in dependencies.
    /// This helps resolve ambiguous links like `exit` to the intended
    /// crate rather than `std::process::exit`.
    #[arg(long, requires = "dir")]
    pub primary_crate: Option<String>,

    /// Output directory for generated markdown files.
    ///
    /// The directory will be created if it doesn't exist.
    /// Defaults to `docs/` in the current directory.
    #[arg(short, long, default_value = "docs")]
    pub output: PathBuf,

    /// Output format (flat or nested).
    ///
    /// - `flat`: All files in one directory (default)
    /// - `nested`: Directory hierarchy mirroring modules
    #[arg(short, long, value_enum, default_value_t = CliOutputFormat::Flat)]
    pub format: CliOutputFormat,

    /// Include private (non-public) items in the output.
    ///
    /// By default, only public items are documented. Enable this
    /// to also include `pub(crate)`, `pub(super)`, and private items.
    #[arg(long, default_value_t = false)]
    pub include_private: bool,

    /// Include blanket trait implementations in the output.
    ///
    /// By default, blanket impls like `From`, `Into`, `TryFrom`, `TryInto`,
    /// `Any`, `Borrow`, `BorrowMut`, and `ToOwned` are filtered out to reduce
    /// noise. Enable this to include them in the documentation.
    #[arg(long, default_value_t = false)]
    pub include_blanket_impls: bool,
}

/// CLI-compatible output format enum (for clap `ValueEnum` derive).
#[derive(Clone, Copy, Debug, Default, ValueEnum)]
pub enum CliOutputFormat {
    /// Flat structure with double-underscore separators in filenames.
    #[default]
    Flat,

    /// Nested directory structure mirroring the module hierarchy.
    Nested,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(cli: CliOutputFormat) -> Self {
        match cli {
            CliOutputFormat::Flat => Self::Flat,

            CliOutputFormat::Nested => Self::Nested,
        }
    }
}
