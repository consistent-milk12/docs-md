//! Error types for docs-md.
//!
//! This module defines all errors that can occur during documentation generation.
//! Errors use the `miette` crate for enhanced diagnostics, providing:
//! - Human-readable error messages
//! - Diagnostic codes for programmatic handling
//! - Helpful suggestions for resolution
//!
//! # Error Categories
//!
//! - **I/O Errors** (`FileRead`, `CreateDir`, `FileWrite`): File system operations
//! - **Parse Errors** (`JsonParse`): Invalid or malformed rustdoc JSON
//! - **Lookup Errors** (`ItemNotFound`): Missing items in the documentation index

use miette::Diagnostic;

/// Errors that can occur during documentation generation.
///
/// Each variant includes:
/// - A human-readable error message
/// - A diagnostic code (e.g., `docs_md::io::read`)
/// - Optional help text for resolution
/// - The underlying source error (where applicable)
#[derive(Debug, Diagnostic, thiserror::Error)]
pub enum Error {
    /// Failed to read a file from disk.
    ///
    /// This typically occurs when:
    /// - The file doesn't exist
    /// - The process lacks read permissions
    /// - The path is invalid
    #[error("Failed to read file")]
    #[diagnostic(
        code(docs_md::io::read),
        help("Check that the file exists and is readable")
    )]
    FileRead(#[source] std::io::Error),

    /// Failed to parse the rustdoc JSON file.
    ///
    /// This can happen when:
    /// - The file is not valid JSON
    /// - The JSON schema doesn't match `rustdoc-types` expectations
    /// - The file is from an incompatible rustdoc version
    #[error("Failed to parse rustdoc JSON")]
    #[diagnostic(
        code(docs_md::parse::json),
        help("Ensure the file is valid rustdoc JSON output (generated with --output-format json)")
    )]
    JsonParse(#[source] serde_json::Error),

    /// Failed to create the output directory.
    ///
    /// This typically occurs when:
    /// - The parent directory doesn't exist
    /// - The process lacks write permissions
    /// - The path is invalid or too long
    #[error("Failed to create output directory")]
    #[diagnostic(code(docs_md::io::mkdir))]
    CreateDir(#[source] std::io::Error),

    /// Failed to write a markdown file.
    ///
    /// This typically occurs when:
    /// - The output directory is not writable
    /// - The disk is full
    /// - The file is locked by another process
    #[error("Failed to write file")]
    #[diagnostic(code(docs_md::io::write))]
    FileWrite(#[source] std::io::Error),

    /// An item ID was not found in the crate's index.
    ///
    /// The rustdoc JSON index should contain all referenced items.
    /// This error indicates data inconsistency, possibly from:
    /// - Corrupted JSON
    /// - Incompatible rustdoc-types version
    /// - Items removed during filtering
    ///
    /// The string contains the ID that was not found.
    #[error("Item not found in index: {0}")]
    #[diagnostic(code(docs_md::parse::item_not_found))]
    ItemNotFound(String),
}
