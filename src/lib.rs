//! docs-md library interface for testing and reuse.
//!
//! This module exposes the core functionality of docs-md as a library,
//! allowing integration tests and external tools to use the markdown
//! generation capabilities programmatically.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};

pub mod error;
pub mod generator;
pub mod linker;
#[cfg(feature = "trace")]
pub mod logger;
pub mod multi_crate;
pub mod parser;
#[cfg(feature = "source-parsing")]
pub mod source;
pub mod types;

pub use crate::generator::{Generator, MarkdownCapture, RenderConfig, SourceConfig};
pub use crate::linker::{LinkRegistry, slugify_anchor};
#[cfg(feature = "trace")]
use crate::logger::LogLevel;
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

/// Cargo wrapper for subcommand invocation.
///
/// When invoked as `cargo docs-md`, cargo passes "docs-md" as the first argument.
/// This wrapper handles that by making `docs-md` a subcommand that contains the real CLI.
#[derive(Parser, Debug)]
#[command(name = "cargo", bin_name = "cargo")]
pub enum Cargo {
    /// Generate per-module markdown from rustdoc JSON
    #[command(name = "docs-md")]
    DocsMd(Cli),
}

/// Top-level CLI for docs-md.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Generate per-module markdown from rustdoc JSON",
    args_conflicts_with_subcommands = true
)]
pub struct Cli {
    #[command(subcommand)]
    /// Subcommand to run
    pub command: Option<Command>,

    #[command(flatten)]
    /// Generation options (used when no subcommand is specified)
    pub args: GenerateArgs,

    /// Logging verbosity level
    ///
    /// Controls the amount of diagnostic output. Use for debugging link
    /// resolution issues or understanding the generation process.
    #[cfg(feature = "trace")]
    #[arg(long, value_enum, default_value = "off")]
    pub log_level: LogLevel,

    /// Enable logging to a file instead of stderr
    ///
    /// When set, logs are written to this file path instead of stderr.
    /// Useful for capturing debug output without cluttering terminal.
    #[cfg(feature = "trace")]
    #[arg(long)]
    pub log_file: Option<PathBuf>,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Build rustdoc JSON and generate markdown in one step.
    ///
    /// This runs `cargo +nightly doc` with JSON output, then generates
    /// markdown documentation from the result. Requires nightly toolchain.
    ///
    /// Example: `cargo docs-md docs --primary-crate my_crate`
    Docs(DocsArgs),

    /// Collect dependency sources to a local directory.
    ///
    /// Copies source code from `~/.cargo/registry/src/` into a local
    /// `.source_{timestamp}/` directory for parsing and documentation.
    ///
    /// Example: `cargo docs-md collect-sources --include-dev`
    #[cfg(feature = "source-parsing")]
    CollectSources(CollectSourcesArgs),
}

/// Arguments for the `docs` subcommand (build + generate).
#[derive(Parser, Debug)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Hm.. Cache lining optimization? Seems unnecessary for a CLI args struct."
)]
pub struct DocsArgs {
    /// Output directory for generated markdown files.
    ///
    /// Defaults to `generated_docs/` in the current directory.
    #[arg(short, long, default_value = "generated_docs")]
    pub output: PathBuf,

    /// Output format (flat or nested).
    #[arg(short, long, value_enum, default_value_t = CliOutputFormat::Nested)]
    pub format: CliOutputFormat,

    /// Primary crate name for preferential link resolution.
    ///
    /// If not specified, attempts to detect from Cargo.toml.
    #[arg(long)]
    pub primary_crate: Option<String>,

    /// Exclude private (non-public) items from the output.
    ///
    /// By default, all items are documented including private ones.
    /// Enable this to only include public items.
    #[arg(long, default_value_t = false)]
    pub exclude_private: bool,

    /// Include blanket trait implementations in the output.
    #[arg(long, default_value_t = false)]
    pub include_blanket_impls: bool,

    /// Skip generating mdBook SUMMARY.md file.
    #[arg(long, default_value_t = false)]
    pub no_mdbook: bool,

    /// Skip generating `search_index.json` file.
    #[arg(long, default_value_t = false)]
    pub no_search_index: bool,

    /// Run cargo clean before building (full rebuild).
    #[arg(long, default_value_t = false)]
    pub clean: bool,

    /// Additional arguments to pass to cargo doc.
    ///
    /// Example: `docs-md docs -- --all-features`
    #[arg(last = true)]
    pub cargo_args: Vec<String>,
}

/// Arguments for the `collect-sources` subcommand.
#[cfg(feature = "source-parsing")]
#[derive(Parser, Debug)]
pub struct CollectSourcesArgs {
    /// Output directory for collected sources.
    ///
    /// If not specified, creates `.source_{timestamp}/` in the workspace root.
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Include dev-dependencies in collection.
    ///
    /// By default, only regular dependencies are collected.
    #[arg(long, default_value_t = false)]
    pub include_dev: bool,

    /// Dry run - show what would be collected without copying.
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,

    /// Path to Cargo.toml (defaults to current directory).
    #[arg(long)]
    pub manifest_path: Option<PathBuf>,
}

/// Command-line arguments for direct generation (no subcommand).
///
/// The tool accepts input from two mutually exclusive sources:
/// 1. A local rustdoc JSON file (`--path`)
/// 2. A directory of rustdoc JSON files (`--dir`)
#[derive(Parser, Debug, Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Hm.. Cache lining optimization? Seems unnecessary for a CLI args struct."
)]
pub struct GenerateArgs {
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
    /// Defaults to `generated_docs/` in the current directory.
    #[arg(short, long, default_value = "generated_docs")]
    pub output: PathBuf,

    /// Output format (flat or nested).
    ///
    /// - `flat`: All files in one directory
    /// - `nested`: Directory hierarchy mirroring modules (default)
    #[arg(short, long, value_enum, default_value_t = CliOutputFormat::Nested)]
    pub format: CliOutputFormat,

    /// Exclude private (non-public) items from the output.
    ///
    /// By default, all items are documented including `pub(crate)`,
    /// `pub(super)`, and private items. Enable this to only include
    /// public items.
    #[arg(long, default_value_t = false)]
    pub exclude_private: bool,

    /// Include blanket trait implementations in the output.
    ///
    /// By default, blanket impls like `From`, `Into`, `TryFrom`, `TryInto`,
    /// `Any`, `Borrow`, `BorrowMut`, and `ToOwned` are filtered out to reduce
    /// noise. Enable this to include them in the documentation.
    #[arg(long, default_value_t = false)]
    pub include_blanket_impls: bool,
}

/// Backwards-compatible type alias for existing code.
pub type Args = GenerateArgs;

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
