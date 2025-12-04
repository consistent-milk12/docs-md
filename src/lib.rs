//! docs-md library interface for testing and reuse.
//!
//! This module exposes the core functionality of docs-md as a library,
//! allowing integration tests and external tools to use the markdown
//! generation capabilities programmatically.

#![deny(missing_docs)]
#![warn(clippy::pedantic)]

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

pub mod error;
pub mod generator;
pub mod linker;
pub mod parser;
pub mod types;
pub mod utils;

// pub use crate::error::Error;
// pub use crate::generator::{MarkdownCapture, generate, generate_to_capture};
// pub use crate::linker::LinkRegistry;
// pub use crate::parser::{parse_json, parse_json_string};

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
    /// Mutually exclusive with `--crate-name`.
    #[arg(
        short,
        long,
        required_unless_present = "crate_name",
        conflicts_with = "crate_name"
    )]
    pub path: Option<PathBuf>,

    /// Crate name to fetch from docs.rs (experimental).
    ///
    /// **Note**: This feature is currently non-functional because docs.rs
    /// doesn't expose rustdoc JSON files publicly. Use `--path` instead.
    ///
    /// Mutually exclusive with `--path`.
    #[arg(short, long, required_unless_present = "path", conflicts_with = "path")]
    pub crate_name: Option<String>,

    /// Specific version to fetch from docs.rs.
    ///
    /// Only used with `--crate-name`. Defaults to "latest" if not specified.
    #[arg(long, requires = "crate_name")]
    pub crate_version: Option<String>,

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
            CliOutputFormat::Flat => OutputFormat::Flat,

            CliOutputFormat::Nested => OutputFormat::Nested,
        }
    }
}
