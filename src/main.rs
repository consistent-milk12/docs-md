//! docs-md: Generate per-module markdown documentation from rustdoc JSON.
//!
//! This tool takes rustdoc JSON output (generated via `cargo doc --output-format json`)
//! and produces a directory of markdown files, one per module, with proper cross-linking
//! between items.
//!
//! # Usage
//!
//! ```bash
//! # From local JSON file
//! docs-md --path target/doc/my_crate.json -o docs/
//!
//! # With nested directory structure
//! docs-md --path target/doc/my_crate.json -o docs/ --format nested
//! ```
//!
//! # Output Formats
//!
//! - **Flat**: All markdown files in a single directory with `module__submodule.md` naming
//! - **Nested**: Directory hierarchy mirroring module structure with `module/index.md` files

use clap::{Parser, ValueEnum};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

mod error;
mod generator;
mod linker;
mod parser;
mod types;

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
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Flat)]
    pub format: OutputFormat,

    /// Include private (non-public) items in the output.
    ///
    /// By default, only public items are documented. Enable this
    /// to also include `pub(crate)`, `pub(super)`, and private items.
    #[arg(long, default_value_t = false)]
    pub include_private: bool,
}

/// Entry point for the docs-md CLI tool.
///
/// # Workflow
///
/// 1. Parse command-line arguments
/// 2. Load rustdoc JSON from file or fetch from docs.rs
/// 3. Generate markdown documentation
/// 4. Report success
///
/// # Errors
///
/// Returns an error if:
/// - The input JSON file cannot be read or parsed
/// - The output directory cannot be created
/// - Any file write operation fails
fn main() -> Result<()> {
    // Enable miette's fancy panic hook for better error display
    miette::set_panic_hook();

    // Parse CLI arguments (clap handles validation and help text)
    let args = Args::parse();

    // Load the rustdoc JSON from the appropriate source.
    // The `krate` variable holds the entire crate's documentation structure,
    // including all modules, items, and their relationships.
    let krate = if let Some(path) = &args.path {
        // Local file path provided - parse directly
        parser::parse_json(path)?
    } else if let Some(crate_name) = &args.crate_name {
        // Crate name provided - attempt to fetch from docs.rs
        // NOTE: This path is currently non-functional (docs.rs doesn't serve JSON)
        eprintln!("Fetching {crate_name} from docs.rs...");
        let json_content = fetch_from_docs_rs(crate_name, args.crate_version.as_deref())?;
        parser::parse_json_string(&json_content)?
    } else {
        // This branch should never execute - clap ensures one of the two is provided
        unreachable!("clap should ensure either path or crate_name is provided");
    };

    // Generate markdown files from the parsed crate documentation.
    // This is the main work: traversing the module tree and writing .md files.
    generator::generate(&krate, &args)?;

    // Success message to the user
    println!(
        "Documentation generated successfully in '{}'",
        args.output.display()
    );

    Ok(())
}

/// Attempt to fetch rustdoc JSON from docs.rs (currently non-functional).
///
/// **Important**: This function exists for future compatibility, but docs.rs
/// does not currently serve rustdoc JSON files. Users should generate JSON
/// locally using `cargo doc --output-format json`.
///
/// # Arguments
///
/// * `crate_name` - The name of the crate on crates.io (e.g., "serde")
/// * `version` - Optional version string. Defaults to "latest" if None.
///
/// # URL Pattern
///
/// Attempts to fetch from: `https://docs.rs/{crate}/{version}/{crate}.json`
///
/// Note: Crate names with hyphens are converted to underscores for the filename
/// (e.g., "my-crate" becomes "`my_crate.json`").
///
/// # Errors
///
/// Returns an error if:
/// - The HTTP request fails (network error, 404, etc.)
/// - The response body cannot be read
fn fetch_from_docs_rs(crate_name: &str, version: Option<&str>) -> Result<String> {
    // Default to "latest" if no version specified
    let version = version.unwrap_or("latest");

    // Build the URL for the rustdoc JSON file.
    // Crate names use hyphens (my-crate) but file names use underscores (my_crate.json)
    let url = format!(
        "https://docs.rs/{}/{}/{}.json",
        crate_name,
        version,
        crate_name.replace('-', "_") // Convert hyphens to underscores for filename
    );

    eprintln!("Trying URL: {url}");

    // Make HTTP GET request using ureq (blocking HTTP client)
    let response = ureq::get(&url)
        .call()
        .into_diagnostic()
        .map_err(|e| miette::miette!("Failed to fetch from docs.rs: {}", e))?;

    // Read the response body as a string (the JSON content)
    let body = response
        .into_body()
        .read_to_string()
        .into_diagnostic()
        .map_err(|e| miette::miette!("Failed to read response body: {}", e))?;

    Ok(body)
}
