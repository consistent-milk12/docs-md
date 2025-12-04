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

use clap::Parser;
use docs_md as Internals;
use miette::{IntoDiagnostic, Result};

use Internals::Args;
use Internals::generator::Generator;
use Internals::parser::Parser as InternalParser;

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
        InternalParser::parse_json(path)?
    } else if let Some(crate_name) = &args.crate_name {
        // Crate name provided - attempt to fetch from docs.rs
        // NOTE: This path is currently non-functional (docs.rs doesn't serve JSON)
        eprintln!("Fetching {crate_name} from docs.rs...");

        let json_content = fetch_from_docs_rs(crate_name, args.crate_version.as_deref())?;
        InternalParser::parse_json_string(&json_content)?
    } else {
        // This branch should never execute - clap ensures one of the two is provided
        unreachable!("clap should ensure either path or crate_name is provided");
    };

    // Generate markdown files from the parsed crate documentation.
    // This is the main work: traversing the module tree and writing .md files.
    Generator::run(&krate, &args)?;

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
