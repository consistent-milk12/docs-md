//! Source code parsing for enhanced documentation.
//!
//! This module provides functionality to collect and parse Rust source code
//! from dependencies, extracting information not available in rustdoc JSON:
//!
//! - Function bodies and implementation details
//! - Private items (functions, structs, etc.)
//! - Constant and static values
//! - Macro definitions
//! - Test examples
//!
//! # Architecture
//!
//! The source parsing system has four main components:
//!
//! 1. [`SourceCollector`] - Collects dependency sources to `.source_*/`
//! 2. [`SourceLocator`] - Finds crate sources in the Cargo registry
//! 3. [`SourceParser`] - Parses Rust source files using `syn`
//! 4. [`types`] - Data structures for parsed source information
//!
//! # Workflow
//!
//! ```no_run
//! use cargo_docs_md::source::{SourceCollector, CollectOptions};
//!
//! // Collect dependency sources to .source_{timestamp}/
//! let collector = SourceCollector::new()?;
//! let result = collector.collect(&CollectOptions::default())?;
//! println!("Collected {} crates to {}", result.crates_collected, result.output_dir.display());
//! # Ok::<(), cargo_docs_md::error::Error>(())
//! ```
//!
//! # Feature Flag
//!
//! This module requires the `source-parsing` feature:
//!
//! ```toml
//! cargo-docs-md = { version = "0.1", features = ["source-parsing"] }
//! ```

mod collector;
mod integration;
mod locator;
mod parser;
pub mod types;

pub use collector::{
    CollectOptions, CollectedCrate, CollectionResult, SourceCollector, SourceManifest,
};
pub use locator::SourceLocator;
pub use parser::SourceParser;
pub use types::{
    ConstInfo, CrateSource, EnumInfo, FieldInfo, FunctionInfo, ImplInfo, MacroInfo, PrivateItem,
    StaticInfo, StructInfo, TraitInfo, TypeAliasInfo, VariantInfo,
};
