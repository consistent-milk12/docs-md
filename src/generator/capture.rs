//! In-memory markdown capture for testing.
//!
//! This module provides [`MarkdownCapture`] for capturing generated markdown
//! in memory instead of writing to disk, enabling snapshot testing.

use std::collections::HashMap;

/// Captures generated markdown in memory for testing.
///
/// Instead of writing files to disk, this struct stores all generated
/// markdown content in a `HashMap`, keyed by relative file path. This
/// enables snapshot testing and verification of output without filesystem
/// side effects.
#[derive(Debug, Default)]
pub struct MarkdownCapture {
    /// Maps file paths (relative to output directory) to their generated content.
    files: HashMap<String, String>,
}

impl MarkdownCapture {
    /// Create a new empty capture.
    #[must_use]
    pub fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    /// Add a file to the capture.
    ///
    /// # Arguments
    /// * `path` - Relative path of the file (e.g., "index.md" or "span/index.md")
    /// * `content` - The markdown content for this file
    pub fn insert(&mut self, path: String, content: String) {
        self.files.insert(path, content);
    }

    /// Get the content of a specific file.
    #[must_use]
    pub fn get(&self, path: &str) -> Option<&String> {
        self.files.get(path)
    }

    /// Get all file paths in sorted order.
    #[must_use]
    pub fn paths(&self) -> Vec<&String> {
        let mut paths: Vec<_> = self.files.keys().collect();
        paths.sort();
        paths
    }

    /// Get the number of captured files.
    #[must_use]
    pub fn len(&self) -> usize {
        self.files.len()
    }

    /// Check if the capture is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Convert all captured files to a single string for snapshot testing.
    ///
    /// Files are sorted by path and separated with clear headers.
    #[must_use]
    pub fn to_snapshot_string(&self) -> String {
        use std::fmt::Write;

        let mut result = String::new();
        let mut paths: Vec<_> = self.files.keys().collect();
        paths.sort();

        for path in paths {
            _ = writeln!(result, "=== {path} ===");
            result.push_str(&self.files[path]);
            if !self.files[path].ends_with('\n') {
                result.push('\n');
            }
            result.push('\n');
        }

        result
    }

    /// Consume self and return the underlying `HashMap`.
    #[must_use]
    pub fn into_inner(self) -> HashMap<String, String> {
        self.files
    }
}
