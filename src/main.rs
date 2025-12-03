use clap::{Parser, ValueEnum};
use miette::{IntoDiagnostic, Result};
use std::path::PathBuf;

mod error;
mod generator;
mod linker;
mod parser;
mod types;

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
    /// One markdown file per module, flat directory
    #[default]
    Flat,
    /// Nested directory structure mirroring module hierarchy
    Nested,
}

#[derive(Parser, Debug)]
#[command(name = "docs-md")]
#[command(
    author,
    version,
    about = "Generate per-module markdown from rustdoc JSON"
)]
pub struct Args {
    /// Path to rustdoc JSON file (mutually exclusive with --crate-name)
    #[arg(
        short,
        long,
        required_unless_present = "crate_name",
        conflicts_with = "crate_name"
    )]
    pub path: Option<PathBuf>,

    /// Fetch crate from docs.rs (mutually exclusive with --path)
    #[arg(short, long, required_unless_present = "path", conflicts_with = "path")]
    pub crate_name: Option<String>,

    /// Crate version to fetch from docs.rs (defaults to latest)
    #[arg(long, requires = "crate_name")]
    pub crate_version: Option<String>,

    /// Output directory
    #[arg(short, long, default_value = "docs")]
    pub output: PathBuf,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Flat)]
    pub format: OutputFormat,

    /// Include private items
    #[arg(long, default_value_t = false)]
    pub include_private: bool,
}

fn main() -> Result<()> {
    miette::set_panic_hook();

    let args = Args::parse();

    // Get the rustdoc JSON (either from file or docs.rs)
    let krate = if let Some(path) = &args.path {
        parser::parse_json(path)?
    } else if let Some(crate_name) = &args.crate_name {
        eprintln!("Fetching {} from docs.rs...", crate_name);
        let json_content = fetch_from_docs_rs(crate_name, args.crate_version.as_deref())?;
        parser::parse_json_string(&json_content)?
    } else {
        unreachable!("clap should ensure either path or crate_name is provided");
    };

    // Generate markdown files
    generator::generate(&krate, &args)?;

    println!(
        "Documentation generated successfully in '{}'",
        args.output.display()
    );

    Ok(())
}

/// Fetch rustdoc JSON from docs.rs
fn fetch_from_docs_rs(crate_name: &str, version: Option<&str>) -> Result<String> {
    // Build the URL for the rustdoc JSON
    // docs.rs provides JSON at: https://docs.rs/crate/{name}/{version}/target.json
    // But the actual rustdoc JSON is at: https://docs.rs/crate/{name}/{version}/builds.json
    // then we need to find the right build and download it
    //
    // Actually, docs.rs provides the JSON at:
    // https://docs.rs/{crate}/{version}/{crate}/index.json (not quite right either)
    //
    // The correct URL pattern for rustdoc JSON is:
    // https://docs.rs/{crate}/{version}/{crate}.json
    // But this requires the exact version. For "latest", we need to resolve it first.

    let version = version.unwrap_or("latest");

    // First, try to get the JSON directly
    let url = format!(
        "https://docs.rs/{}/{}/{}.json",
        crate_name,
        version,
        crate_name.replace('-', "_")
    );

    eprintln!("Trying URL: {}", url);

    let response = ureq::get(&url)
        .call()
        .into_diagnostic()
        .map_err(|e| miette::miette!("Failed to fetch from docs.rs: {}", e))?;

    let body = response
        .into_body()
        .read_to_string()
        .into_diagnostic()
        .map_err(|e| miette::miette!("Failed to read response body: {}", e))?;

    Ok(body)
}
