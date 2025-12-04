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

use Internals::Args;
use Internals::generator::Generator;
use Internals::multi_crate::{MultiCrateGenerator, MultiCrateParser};
use Internals::parser::Parser as InternalParser;
use clap::Parser;
use docs_md as Internals;
use miette::Result;

/// Entry point for the docs-md CLI tool.
///
/// # Workflow
///
/// 1. Parse command-line arguments
/// 2. Load rustdoc JSON from file or directory
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

    // Handle multi-crate mode (--dir) separately from single-crate mode
    if let Some(dir) = &args.dir {
        // Multi-crate mode: scan directory for JSON files
        eprintln!(
            "Scanning directory for rustdoc JSON files: {}",
            dir.display()
        );

        let crates = MultiCrateParser::parse_directory(dir)?;
        eprintln!(
            "Found {} crates: {}",
            crates.len(),
            crates.names().join(", ")
        );

        // Generate documentation for all crates
        let generator = MultiCrateGenerator::new(&crates, &args);
        generator.generate()?;

        // Success message
        println!(
            "Multi-crate documentation generated successfully in '{}'",
            args.output.display()
        );

        return Ok(());
    }

    // Single-crate mode: load from file
    // The `krate` variable holds the entire crate's documentation structure,
    // including all modules, items, and their relationships.
    let path = args
        .path
        .as_ref()
        .expect("clap ensures path or dir is provided");
    let krate = InternalParser::parse_json(path)?;

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
