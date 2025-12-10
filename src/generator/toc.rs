//! Table of Contents generation for module documentation.
//!
//! This module provides [`TocGenerator`] which generates a markdown table of contents
//! for modules that exceed a configurable item threshold. The TOC provides navigation
//! to major sections (Types, Traits, Functions, etc.) with nested links to individual items.
//!
//! # Example Output
//!
//! ```markdown
//! ## Contents
//!
//! - [Structs](#structs)
//!   - [`Parser`](#parser)
//!   - [`Config`](#config)
//! - [Enums](#enums)
//!   - [`Error`](#error)
//! - [Functions](#functions)
//! ```

use std::fmt::Write;

/// An entry in the table of contents.
///
/// Each entry represents either a section heading (like "Structs") or an
/// individual item (like a specific struct name). Entries can have children
/// for nested navigation.
#[derive(Debug, Clone)]
pub struct TocEntry {
    /// Display title for this entry.
    pub title: String,

    /// Anchor link target (without the `#` prefix).
    pub anchor: String,

    /// Child entries for nested navigation.
    pub children: Vec<Self>,
}

impl TocEntry {
    /// Create a new TOC entry.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the entry
    /// * `anchor` - Anchor link target (without `#`)
    #[must_use]
    pub fn new(title: impl Into<String>, anchor: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            anchor: anchor.into(),
            children: Vec::new(),
        }
    }

    /// Create a new TOC entry with children.
    ///
    /// # Arguments
    ///
    /// * `title` - Display title for the entry
    /// * `anchor` - Anchor link target (without `#`)
    /// * `children` - Child entries for nested items
    #[must_use]
    pub fn with_children(
        title: impl Into<String>,
        anchor: impl Into<String>,
        children: Vec<Self>,
    ) -> Self {
        Self {
            title: title.into(),
            anchor: anchor.into(),
            children,
        }
    }

    /// Count total items in this entry and all descendants.
    #[must_use]
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(Self::count).sum::<usize>()
    }
}

/// Generator for markdown table of contents.
///
/// The generator only produces output when the total number of items
/// exceeds the configured threshold. This prevents cluttering small
/// modules with unnecessary navigation.
#[derive(Debug, Clone)]
pub struct TocGenerator {
    /// Minimum items required to generate a TOC.
    threshold: usize,
}

impl TocGenerator {
    /// Create a new TOC generator with the given threshold.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Minimum number of items required to generate a TOC
    #[must_use]
    pub const fn new(threshold: usize) -> Self {
        Self { threshold }
    }

    /// Generate a markdown table of contents from the given entries.
    ///
    /// Returns `None` if the total item count is below the threshold.
    ///
    /// # Arguments
    ///
    /// * `entries` - Top-level TOC entries (typically section headings)
    ///
    /// # Returns
    ///
    /// A formatted markdown string with the TOC, or `None` if below threshold.
    #[must_use]
    pub fn generate(&self, entries: &[TocEntry]) -> Option<String> {
        // Calculate total items
        let total: usize = entries.iter().map(TocEntry::count).sum();

        if total < self.threshold {
            return None;
        }

        let mut md = String::new();
        _ = write!(md, "## Contents\n\n");

        for entry in entries {
            Self::render_entry(&mut md, entry, 0);
        }

        md.push('\n');
        Some(md)
    }

    /// Render a single TOC entry with proper indentation.
    fn render_entry(md: &mut String, entry: &TocEntry, depth: usize) {
        let indent = "  ".repeat(depth);
        _ = writeln!(md, "{indent}- [{}](#{})", entry.title, entry.anchor);

        for child in &entry.children {
            Self::render_entry(md, child, depth + 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toc_entry_new() {
        let entry = TocEntry::new("Structs", "structs");
        assert_eq!(entry.title, "Structs");
        assert_eq!(entry.anchor, "structs");
        assert!(entry.children.is_empty());
    }

    #[test]
    fn toc_entry_with_children() {
        let children = vec![
            TocEntry::new("`Parser`", "parser"),
            TocEntry::new("`Config`", "config"),
        ];
        let entry = TocEntry::with_children("Structs", "structs", children);

        assert_eq!(entry.title, "Structs");
        assert_eq!(entry.children.len(), 2);
        assert_eq!(entry.children[0].title, "`Parser`");
    }

    #[test]
    fn toc_entry_count() {
        let entry = TocEntry::new("Single", "single");
        assert_eq!(entry.count(), 1);

        let with_children = TocEntry::with_children(
            "Parent",
            "parent",
            vec![
                TocEntry::new("Child1", "child1"),
                TocEntry::new("Child2", "child2"),
            ],
        );
        assert_eq!(with_children.count(), 3);
    }

    #[test]
    fn toc_generator_below_threshold() {
        let generator = TocGenerator::new(10);
        let entries = vec![
            TocEntry::new("Structs", "structs"),
            TocEntry::new("Enums", "enums"),
        ];

        assert!(generator.generate(&entries).is_none());
    }

    #[test]
    fn toc_generator_at_threshold() {
        let generator = TocGenerator::new(3);
        let entries = vec![
            TocEntry::new("Structs", "structs"),
            TocEntry::new("Enums", "enums"),
            TocEntry::new("Functions", "functions"),
        ];

        let result = generator.generate(&entries);
        assert!(result.is_some());
    }

    #[test]
    fn toc_generator_output_format() {
        let generator = TocGenerator::new(1);
        let entries = vec![TocEntry::with_children(
            "Structs",
            "structs",
            vec![
                TocEntry::new("`Parser`", "parser"),
                TocEntry::new("`Config`", "config"),
            ],
        )];

        let result = generator.generate(&entries).unwrap();

        assert!(result.contains("## Contents"));
        assert!(result.contains("- [Structs](#structs)"));
        assert!(result.contains("  - [`Parser`](#parser)"));
        assert!(result.contains("  - [`Config`](#config)"));
    }

    #[test]
    fn toc_generator_nested_indentation() {
        let generator = TocGenerator::new(1);
        let entries = vec![TocEntry::with_children(
            "Level0",
            "l0",
            vec![TocEntry::with_children(
                "Level1",
                "l1",
                vec![TocEntry::new("Level2", "l2")],
            )],
        )];

        let result = generator.generate(&entries).unwrap();

        assert!(result.contains("- [Level0](#l0)"));
        assert!(result.contains("  - [Level1](#l1)"));
        assert!(result.contains("    - [Level2](#l2)"));
    }
}
