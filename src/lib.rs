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
pub mod utils;

pub use crate::generator::{Generator, MarkdownCapture, RenderConfig, SourceConfig};
pub use crate::linker::{AnchorUtils, LinkRegistry};
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

    // === RenderConfig toggles ===
    /// Minimum number of items before generating a table of contents.
    ///
    /// Modules with fewer items than this threshold won't have a TOC.
    /// Default: 10
    #[arg(long, default_value_t = 10)]
    pub toc_threshold: usize,

    /// Disable quick reference tables at the top of modules.
    #[arg(long, default_value_t = false)]
    pub no_quick_reference: bool,

    /// Disable grouping impl blocks by category (Derive, Conversion, etc.).
    #[arg(long, default_value_t = false)]
    pub no_group_impls: bool,

    /// Hide trivial derive implementations (Clone, Copy, Debug, etc.).
    #[arg(long, default_value_t = false)]
    pub hide_trivial_derives: bool,

    /// Disable method-level anchors for deep linking.
    #[arg(long, default_value_t = false)]
    pub no_method_anchors: bool,

    /// Include source file locations for items.
    #[arg(long, default_value_t = false)]
    pub source_locations: bool,

    /// Include full method documentation instead of first-line summaries.
    ///
    /// By default, only the first paragraph of method docs is shown in impl blocks.
    /// Enable this to include the complete documentation for each method.
    #[arg(long, default_value_t = false)]
    pub full_method_docs: bool,

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

    /// Only copy `src/` directory and `Cargo.toml`.
    ///
    /// By default, the entire crate directory is copied to ensure all source
    /// files are available (including `build.rs`, modules outside `src/`, etc.).
    /// Enable this flag to minimize disk usage at the cost of potentially
    /// missing some source files.
    #[arg(long, default_value_t = false)]
    pub minimal_sources: bool,

    /// Do not add `.source_*` pattern to `.gitignore`.
    ///
    /// By default, the tool appends `.source_*` to `.gitignore` to prevent
    /// accidentally committing collected source files. Enable this flag to
    /// skip this modification (useful for projects with strict `.gitignore`
    /// management).
    #[arg(long, default_value_t = false)]
    pub no_gitignore: bool,
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

    /// Skip generating mdBook SUMMARY.md file.
    ///
    /// Only valid with `--dir` for multi-crate documentation.
    /// By default, a `SUMMARY.md` file is created in the output directory
    /// that can be used as the entry point for an mdBook documentation site.
    #[arg(long, requires = "dir", default_value_t = false)]
    pub no_mdbook: bool,

    /// Skip generating `search_index.json` file.
    ///
    /// Only valid with `--dir` for multi-crate documentation.
    /// By default, a `search_index.json` file is created containing all
    /// documented items, which can be used with client-side search libraries
    /// like Fuse.js, Lunr.js, or `FlexSearch`.
    #[arg(long, requires = "dir", default_value_t = false)]
    pub no_search_index: bool,

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

    // === RenderConfig toggles ===
    /// Minimum number of items before generating a table of contents.
    ///
    /// Modules with fewer items than this threshold won't have a TOC.
    /// Default: 10
    #[arg(long, default_value_t = 10)]
    pub toc_threshold: usize,

    /// Disable quick reference tables at the top of modules.
    #[arg(long, default_value_t = false)]
    pub no_quick_reference: bool,

    /// Disable grouping impl blocks by category (Derive, Conversion, etc.).
    #[arg(long, default_value_t = false)]
    pub no_group_impls: bool,

    /// Hide trivial derive implementations (Clone, Copy, Debug, etc.).
    #[arg(long, default_value_t = false)]
    pub hide_trivial_derives: bool,

    /// Disable method-level anchors for deep linking.
    #[arg(long, default_value_t = false)]
    pub no_method_anchors: bool,

    /// Include source file locations for items.
    #[arg(long, default_value_t = false)]
    pub source_locations: bool,

    /// Include full method documentation instead of first-line summaries.
    ///
    /// By default, only the first paragraph of method docs is shown in impl blocks.
    /// Enable this to include the complete documentation for each method.
    #[arg(long, default_value_t = false)]
    pub full_method_docs: bool,
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

// Bounds check test functions for assembly inspection
// Run: cargo asm cargo_docs_md::iter_zip

/// Test function: iterator zip (no bounds checks in loop).
#[inline(never)]
pub fn iter_zip(a: Vec<i64>, b: Vec<i64>) -> i64 {
    let mut r = 0i64;
    assert!(a.len() == b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        r += x + y;
    }
    r
}

/// Test function: index loop with assert (bounds check elided).
#[inline(never)]
pub fn index_loop(a: Vec<i64>, b: Vec<i64>) -> i64 {
    let mut r = 0i64;
    assert!(a.len() == b.len());
    for i in 0..a.len() {
        r += a[i] + b[i];
    }
    r
}

/// Test function: index loop without assert (bounds check present).
#[inline(never)]
pub fn index_loop_no_assert(a: Vec<i64>, b: Vec<i64>) -> i64 {
    let mut r = 0i64;
    for i in 0..a.len() {
        r += a[i] + b[i];
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    /// Test that GenerateArgs correctly parses `--no-mdbook` flag.
    #[test]
    fn test_generate_args_no_mdbook_flag() {
        // Without flag: no_mdbook should be false
        let args = GenerateArgs::try_parse_from(["test", "--dir", "target/doc"]).unwrap();
        assert!(!args.no_mdbook, "no_mdbook should default to false");

        // With flag: no_mdbook should be true
        let args =
            GenerateArgs::try_parse_from(["test", "--dir", "target/doc", "--no-mdbook"]).unwrap();
        assert!(args.no_mdbook, "no_mdbook should be true when flag is set");
    }

    /// Test that GenerateArgs correctly parses `--no-search-index` flag.
    #[test]
    fn test_generate_args_no_search_index_flag() {
        // Without flag: no_search_index should be false
        let args = GenerateArgs::try_parse_from(["test", "--dir", "target/doc"]).unwrap();
        assert!(
            !args.no_search_index,
            "no_search_index should default to false"
        );

        // With flag: no_search_index should be true
        let args =
            GenerateArgs::try_parse_from(["test", "--dir", "target/doc", "--no-search-index"])
                .unwrap();
        assert!(
            args.no_search_index,
            "no_search_index should be true when flag is set"
        );
    }

    /// Test that DocsArgs correctly parses `--no-mdbook` flag.
    #[test]
    fn test_docs_args_no_mdbook_flag() {
        // Without flag: no_mdbook should be false (mdbook enabled by default)
        let args = DocsArgs::try_parse_from(["test"]).unwrap();
        assert!(
            !args.no_mdbook,
            "no_mdbook should default to false (mdbook enabled)"
        );

        // With flag: no_mdbook should be true
        let args = DocsArgs::try_parse_from(["test", "--no-mdbook"]).unwrap();
        assert!(args.no_mdbook, "no_mdbook should be true when flag is set");
    }

    /// Test that DocsArgs correctly parses `--no-search-index` flag.
    #[test]
    fn test_docs_args_no_search_index_flag() {
        // Without flag: no_search_index should be false (search index enabled by default)
        let args = DocsArgs::try_parse_from(["test"]).unwrap();
        assert!(
            !args.no_search_index,
            "no_search_index should default to false (search index enabled)"
        );

        // With flag: no_search_index should be true
        let args = DocsArgs::try_parse_from(["test", "--no-search-index"]).unwrap();
        assert!(
            args.no_search_index,
            "no_search_index should be true when flag is set"
        );
    }

    /// Test that DocsArgs and GenerateArgs have consistent defaults for mdbook/search behavior.
    #[test]
    fn test_docs_and_generate_args_consistent_defaults() {
        let docs_args = DocsArgs::try_parse_from(["test"]).unwrap();
        let generate_args = GenerateArgs::try_parse_from(["test", "--dir", "target/doc"]).unwrap();

        // Both should have the same default behavior: features enabled (no_* = false)
        assert_eq!(
            docs_args.no_mdbook, generate_args.no_mdbook,
            "DocsArgs and GenerateArgs should have same default for no_mdbook"
        );
        assert_eq!(
            docs_args.no_search_index, generate_args.no_search_index,
            "DocsArgs and GenerateArgs should have same default for no_search_index"
        );
    }

    /// Test that CollectSourcesArgs correctly parses `--minimal-sources` flag.
    #[cfg(feature = "source-parsing")]
    #[test]
    fn test_collect_sources_args_minimal_sources_flag() {
        // Without flag: minimal_sources should be false (full copy)
        let args = CollectSourcesArgs::try_parse_from(["test"]).unwrap();
        assert!(
            !args.minimal_sources,
            "minimal_sources should default to false (full copy)"
        );

        // With flag: minimal_sources should be true
        let args = CollectSourcesArgs::try_parse_from(["test", "--minimal-sources"]).unwrap();
        assert!(
            args.minimal_sources,
            "minimal_sources should be true when flag is set"
        );
    }

    /// Test that CollectSourcesArgs correctly parses `--no-gitignore` flag.
    #[cfg(feature = "source-parsing")]
    #[test]
    fn test_collect_sources_args_no_gitignore_flag() {
        // Without flag: no_gitignore should be false (update gitignore)
        let args = CollectSourcesArgs::try_parse_from(["test"]).unwrap();
        assert!(
            !args.no_gitignore,
            "no_gitignore should default to false (update gitignore)"
        );

        // With flag: no_gitignore should be true
        let args = CollectSourcesArgs::try_parse_from(["test", "--no-gitignore"]).unwrap();
        assert!(
            args.no_gitignore,
            "no_gitignore should be true when flag is set"
        );
    }

    /// Test that GenerateArgs requires either --path or --dir.
    #[test]
    fn test_generate_args_requires_path_or_dir() {
        // Neither provided - should fail
        let result = GenerateArgs::try_parse_from(["test"]);
        assert!(
            result.is_err(),
            "Should require either --path or --dir"
        );

        // Only --path provided - should succeed
        let result = GenerateArgs::try_parse_from(["test", "--path", "file.json"]);
        assert!(result.is_ok(), "Should accept --path alone");

        // Only --dir provided - should succeed
        let result = GenerateArgs::try_parse_from(["test", "--dir", "target/doc"]);
        assert!(result.is_ok(), "Should accept --dir alone");

        // Both provided - should fail (mutually exclusive)
        let result =
            GenerateArgs::try_parse_from(["test", "--path", "file.json", "--dir", "target/doc"]);
        assert!(
            result.is_err(),
            "--path and --dir should be mutually exclusive"
        );
    }

    /// Test that mdbook/search-index flags work correctly with --dir.
    #[test]
    fn test_mdbook_search_index_with_dir() {
        // With --dir, both flags should work
        let result = GenerateArgs::try_parse_from([
            "test",
            "--dir",
            "target/doc",
            "--no-mdbook",
            "--no-search-index",
        ]);
        assert!(
            result.is_ok(),
            "--no-mdbook and --no-search-index should work with --dir"
        );

        // Verify the flags are correctly parsed
        let args = result.unwrap();
        assert!(args.no_mdbook, "--no-mdbook should be true");
        assert!(args.no_search_index, "--no-search-index should be true");
    }

    /// Test GenerateArgs default values for common options.
    #[test]
    fn test_generate_args_defaults() {
        let args = GenerateArgs::try_parse_from(["test", "--dir", "target/doc"]).unwrap();

        assert_eq!(
            args.output,
            PathBuf::from("generated_docs"),
            "output should default to generated_docs"
        );
        assert!(
            matches!(args.format, CliOutputFormat::Nested),
            "format should default to Nested"
        );
        assert!(!args.exclude_private, "exclude_private should default to false");
        assert!(
            !args.include_blanket_impls,
            "include_blanket_impls should default to false"
        );
        assert_eq!(args.toc_threshold, 10, "toc_threshold should default to 10");
        assert!(
            !args.no_quick_reference,
            "no_quick_reference should default to false"
        );
        assert!(!args.no_group_impls, "no_group_impls should default to false");
        assert!(
            !args.hide_trivial_derives,
            "hide_trivial_derives should default to false"
        );
        assert!(
            !args.no_method_anchors,
            "no_method_anchors should default to false"
        );
        assert!(
            !args.source_locations,
            "source_locations should default to false"
        );
        assert!(
            !args.full_method_docs,
            "full_method_docs should default to false"
        );
    }
}
