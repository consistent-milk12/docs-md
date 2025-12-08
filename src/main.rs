//! docs-md: Generate per-module markdown documentation from rustdoc JSON.
//!
//! This tool takes rustdoc JSON output (generated via `cargo doc --output-format json`)
//! and produces a directory of markdown files, one per module, with proper cross-linking
//! between items.
//!
//! # Usage
//!
//! ```bash
//! # One-step: build and generate docs
//! cargo docs-md docs
//!
//! # From local JSON file
//! cargo docs-md --path target/doc/my_crate.json -o generated_docs/
//!
//! # With nested directory structure
//! cargo docs-md --path target/doc/my_crate.json -o generated_docs/ --format nested
//! ```
//!
//! # Output Formats
//!
//! - **Flat**: All markdown files in a single directory with `module__submodule.md` naming
//! - **Nested**: Directory hierarchy mirroring module structure with `module/index.md` files

use std::path::PathBuf;
use std::process::Command;

use Internals::generator::Generator;
use Internals::multi_crate::{MultiCrateGenerator, MultiCrateParser};
use Internals::parser::Parser as InternalParser;
use Internals::{Cargo, Command as CliCommand, DocsArgs, GenerateArgs};
use clap::Parser;
use cargo_docs_md as Internals;
#[cfg(feature = "trace")]
use cargo_docs_md::logger::Logger;
use miette::{IntoDiagnostic, Result, miette};

/// Entry point for the docs-md CLI tool.
///
/// # Workflow
///
/// 1. Parse command-line arguments
/// 2. Handle subcommand if present (`docs` runs cargo doc first)
/// 3. Load rustdoc JSON from file or directory
/// 4. Generate markdown documentation
/// 5. Report success
///
/// # Errors
///
/// Returns an error if:
/// - The input JSON file cannot be read or parsed
/// - The output directory cannot be created
/// - Any file write operation fails
/// - (for `docs` subcommand) cargo doc fails or nightly is missing
fn main() -> Result<()> {
    // Enable miette's fancy panic hook for better error display
    miette::set_panic_hook();

    // Parse CLI arguments (clap handles validation and help text)
    // Cargo wrapper handles `cargo docs-md` invocation
    let Cargo::DocsMd(cli) = Cargo::parse();

    #[cfg(feature = "trace")]
    Logger::init_logging(cli.log_level, cli.log_file.as_ref())?;

    // Handle subcommands
    if let Some(command) = cli.command {
        return match command {
            CliCommand::Docs(args) => run_docs_command(args),
        };
    }

    // No subcommand: use the flattened args for direct generation
    run_generate(&cli.args)
}

/// Run the `docs` subcommand: build rustdoc JSON and generate markdown.
fn run_docs_command(args: DocsArgs) -> Result<()> {
    // Check for nightly toolchain
    check_nightly_toolchain()?;

    // Optionally run cargo clean
    if args.clean {
        eprintln!("Running cargo clean...");
        let status = Command::new("cargo")
            .arg("clean")
            .status()
            .into_diagnostic()?;

        if !status.success() {
            return Err(miette!("cargo clean failed"));
        }
    }

    // Detect primary crate from Cargo.toml if not specified
    let primary_crate = args.primary_crate.or_else(detect_crate_name);

    // Build rustdoc JSON
    eprintln!("Building rustdoc JSON (this may take a while)...");
    let mut cargo_cmd = Command::new("cargo");
    cargo_cmd.arg("+nightly").arg("doc");

    // Add --document-private-items unless exclude_private is set
    if args.exclude_private {
        cargo_cmd.env("RUSTDOCFLAGS", "-Z unstable-options --output-format json");
    } else {
        cargo_cmd.env(
            "RUSTDOCFLAGS",
            "-Z unstable-options --output-format json --document-private-items",
        );
    }

    // Add any extra cargo args
    for arg in &args.cargo_args {
        cargo_cmd.arg(arg);
    }

    let status = cargo_cmd.status().into_diagnostic()?;

    if !status.success() {
        return Err(miette!(
            "cargo doc failed. Make sure nightly toolchain is installed:\n  rustup toolchain install nightly"
        ));
    }

    eprintln!("Rustdoc JSON generated in target/doc/");

    // Now generate markdown from target/doc/
    let generate_args = GenerateArgs {
        path: None,
        dir: Some(PathBuf::from("target/doc")),
        mdbook: !args.no_mdbook,
        search_index: !args.no_search_index,
        primary_crate,
        output: args.output,
        format: args.format,
        exclude_private: args.exclude_private,
        include_blanket_impls: args.include_blanket_impls,
    };

    run_generate(&generate_args)
}

/// Run the generation logic (shared by direct invocation and `docs` subcommand).
fn run_generate(args: &GenerateArgs) -> Result<()> {
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
            crates
                .names()
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );

        // Generate documentation for all crates
        let generator = MultiCrateGenerator::new(&crates, args);
        generator.generate()?;

        // Success message
        println!(
            "Documentation generated successfully in '{}'",
            args.output.display()
        );

        return Ok(());
    }

    // Single-crate mode: load from file
    let path = args
        .path
        .as_ref()
        .expect("clap ensures path or dir is provided");
    let krate = InternalParser::parse_file(path)?;

    // Generate markdown files
    Generator::run(&krate, args)?;

    println!(
        "Documentation generated successfully in '{}'",
        args.output.display()
    );

    Ok(())
}

/// Check that the nightly toolchain is installed.
fn check_nightly_toolchain() -> Result<()> {
    let output = Command::new("rustup")
        .args(["toolchain", "list"])
        .output()
        .into_diagnostic()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stdout.lines().any(|line| line.starts_with("nightly")) {
        return Err(miette!(
            "Rust nightly toolchain is not installed.\n\n\
             rustdoc JSON output requires the nightly toolchain.\n\
             Install it with:\n\n  \
             rustup toolchain install nightly"
        ));
    }

    Ok(())
}

/// Try to detect the crate name from Cargo.toml.
fn detect_crate_name() -> Option<String> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").ok()?;

    // Simple parsing - look for name = "..." in [package] section
    let mut in_package = false;

    for line in cargo_toml.lines() {
        let trimmed = line.trim();

        if trimmed == "[package]" {
            in_package = true;
            continue;
        }

        if trimmed.starts_with('[') {
            in_package = false;
            continue;
        }

        if in_package
            && trimmed.starts_with("name")
            && let Some(name) = trimmed
                .split('=')
                .nth(1)
                .map(|s| s.trim().trim_matches('"').trim_matches('\''))
        {
            eprintln!("Detected primary crate: {name}");

            return Some(name.to_string());
        }
    }

    None
}
