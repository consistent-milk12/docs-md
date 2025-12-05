//! Multi-crate documentation generation.
//!
//! This module provides support for generating documentation from multiple
//! rustdoc JSON files, enabling cross-crate linking and self-contained
//! documentation for entire dependency trees.
//!
//! # Architecture
//!
//! The multi-crate system uses these components:
//!
//! - [`CrateCollection`]: Container for parsed crates with processing order
//! - [`MultiCrateParser`]: Scans directories and parses JSON files
//! - [`UnifiedLinkRegistry`]: Cross-crate link resolution
//! - [`MultiCrateContext`]: Shared state during generation
//! - [`MultiCrateGenerator`]: Orchestrates per-crate generation
//! - [`SummaryGenerator`]: Creates mdBook-compatible SUMMARY.md
//!
//! # Usage
//!
//! ```ignore
//! use docs_md::multi_crate::{MultiCrateParser, MultiCrateGenerator};
//!
//! let crates = MultiCrateParser::parse_directory(Path::new("target/doc"))?;
//! let generator = MultiCrateGenerator::new(&crates, &args);
//! generator.generate()?;
//! ```

/// Rust module path separator (e.g., `serde_json::de::from_str`).
pub const RUST_PATH_SEP: &str = "::";

/// File system path separator for generated documentation.
pub const FILE_PATH_SEP: char = '/';

mod collection;
mod context;
mod generator;
mod parser;
mod registry;
mod search;
mod summary;

pub use collection::CrateCollection;
pub use context::{MultiCrateContext, SingleCrateView};
pub use generator::MultiCrateGenerator;
pub use parser::MultiCrateParser;
pub use registry::UnifiedLinkRegistry;
pub use search::{SearchIndex, SearchIndexGenerator};
pub use summary::SummaryGenerator;
