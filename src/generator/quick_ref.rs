//! Quick Reference table generation for module documentation.
//!
//! This module provides [`QuickRefGenerator`] which generates a markdown table
//! summarizing all public items in a module at a glance. The table shows item name,
//! kind, and first-sentence description.
//!
//! # Example Output
//!
//! ```markdown
//! ## Quick Reference
//!
//! | Item | Kind | Description |
//! |------|------|-------------|
//! | [`Parser`](#parser) | struct | JSON parser with streaming support |
//! | [`Value`](#value) | enum | Dynamic JSON value type |
//! | [`parse`](#parse) | fn | Parses a JSON string into a Value |
//! ```

use std::fmt::Write;

/// An entry in the quick reference table.
///
/// Each entry represents a single public item with its name, kind,
/// anchor link, and first-sentence summary.
#[derive(Debug, Clone)]
pub struct QuickRefEntry {
    /// Display name for this entry.
    pub name: String,

    /// Item kind (struct, enum, trait, fn, etc.).
    pub kind: &'static str,

    /// Anchor link target (without the `#` prefix).
    pub anchor: String,

    /// First-sentence summary from doc comment.
    pub summary: String,
}

impl QuickRefEntry {
    /// Create a new quick reference entry.
    ///
    /// # Arguments
    ///
    /// * `name` - Display name for the entry
    /// * `kind` - Item kind (struct, enum, fn, etc.)
    /// * `anchor` - Anchor link target (without `#`)
    /// * `summary` - First-sentence summary
    #[must_use]
    pub fn new(
        name: impl Into<String>,
        kind: &'static str,
        anchor: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            kind,
            anchor: anchor.into(),
            summary: summary.into(),
        }
    }
}

/// Generator for markdown quick reference tables.
///
/// The generator creates a table summarizing all items with links,
/// kinds, and first-sentence descriptions.
#[derive(Debug, Clone, Default)]
pub struct QuickRefGenerator;

impl QuickRefGenerator {
    /// Create a new quick reference generator.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    /// Generate a markdown quick reference table from the given entries.
    ///
    /// Returns an empty string if there are no entries.
    ///
    /// # Arguments
    ///
    /// * `entries` - Quick reference entries to include in the table
    ///
    /// # Returns
    ///
    /// A formatted markdown table string.
    #[must_use]
    pub fn generate(&self, entries: &[QuickRefEntry]) -> String {
        if entries.is_empty() {
            return String::new();
        }

        let mut md = String::new();
        md.push_str("## Quick Reference\n\n");
        md.push_str("| Item | Kind | Description |\n");
        md.push_str("|------|------|-------------|\n");

        for entry in entries {
            // Escape pipe characters in summary to prevent table breakage
            let escaped_summary = entry.summary.replace('|', "\\|");
            _ = writeln!(
                md,
                "| [`{}`](#{}) | {} | {} |",
                entry.name, entry.anchor, entry.kind, escaped_summary
            );
        }

        md.push('\n');
        md
    }
}

/// Extract the first sentence from a documentation string.
///
/// This function finds the first non-empty line and extracts the first
/// sentence (text up to the first `. ` or end of line).
///
/// # Arguments
///
/// * `docs` - Optional documentation string
///
/// # Returns
///
/// The first sentence, or an empty string if no docs.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(extract_summary(Some("A parser. With more.")), "A parser");
/// assert_eq!(extract_summary(Some("Single sentence")), "Single sentence");
/// assert_eq!(extract_summary(None), "");
/// ```
#[must_use]
pub fn extract_summary(docs: Option<&str>) -> String {
    let Some(docs) = docs else {
        return String::new();
    };

    // Find first non-empty line
    let first_line = docs
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("")
        .trim();

    if first_line.is_empty() {
        return String::new();
    }

    // Extract first sentence (up to ". " or end of line)
    // Handle common abbreviations that shouldn't end sentences
    let sentence = extract_first_sentence(first_line);

    // Clean up any trailing punctuation except period
    sentence.trim_end_matches([',', ';', ':']).to_string()
}

/// Extract the first sentence from a line of text.
///
/// Handles edge cases like:
/// - "e.g." and "i.e." abbreviations
/// - Version numbers like "1.0"
/// - URLs with dots
fn extract_first_sentence(text: &str) -> &str {
    // Common abbreviations that shouldn't end sentences
    const ABBREVIATIONS: &[&str] = &[
        "e.g.", "i.e.", "etc.", "vs.", "cf.", "Dr.", "Mr.", "Mrs.", "Ms.", "Jr.", "Sr.", "Inc.",
        "Ltd.", "Corp.", "viz.", "approx.", "dept.", "est.", "fig.", "no.", "vol.",
    ];

    let mut search_start = 0;

    loop {
        // Find next ". " pattern
        let Some(pos) = text[search_start..].find(". ") else {
            // No more ". " found, return the whole text
            return text;
        };

        let absolute_pos = search_start + pos;

        // Check if this is part of an abbreviation
        // The text up to and including the period (at absolute_pos) should end with the abbreviation
        let prefix = &text[..=absolute_pos];
        let is_abbreviation = ABBREVIATIONS
            .iter()
            .any(|abbr| prefix.ends_with(abbr));

        // Check if this looks like a version number (digit before and after the dot)
        let is_version = absolute_pos > 0
            && text[..absolute_pos]
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_digit())
            && text[absolute_pos + 2..]
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit());

        if is_abbreviation || is_version {
            // Skip this occurrence and continue searching
            search_start = absolute_pos + 2;
            continue;
        }

        // Found a real sentence end
        return &text[..absolute_pos + 1]; // Include the period
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // QuickRefEntry tests
    // =========================================================================

    #[test]
    fn quick_ref_entry_new() {
        let entry = QuickRefEntry::new("Parser", "struct", "parser", "A JSON parser");
        assert_eq!(entry.name, "Parser");
        assert_eq!(entry.kind, "struct");
        assert_eq!(entry.anchor, "parser");
        assert_eq!(entry.summary, "A JSON parser");
    }

    // =========================================================================
    // QuickRefGenerator tests
    // =========================================================================

    #[test]
    fn quick_ref_empty_entries() {
        let generator = QuickRefGenerator::new();
        let result = generator.generate(&[]);
        assert!(result.is_empty());
    }

    #[test]
    fn quick_ref_single_entry() {
        let generator = QuickRefGenerator::new();
        let entries = vec![QuickRefEntry::new("Parser", "struct", "parser", "A parser")];

        let result = generator.generate(&entries);

        assert!(result.contains("## Quick Reference"));
        assert!(result.contains("| Item | Kind | Description |"));
        assert!(result.contains("| [`Parser`](#parser) | struct | A parser |"));
    }

    #[test]
    fn quick_ref_multiple_entries() {
        let generator = QuickRefGenerator::new();
        let entries = vec![
            QuickRefEntry::new("Parser", "struct", "parser", "A parser"),
            QuickRefEntry::new("Value", "enum", "value", "A value type"),
            QuickRefEntry::new("parse", "fn", "parse", "Parse a string"),
        ];

        let result = generator.generate(&entries);

        assert!(result.contains("| [`Parser`](#parser) | struct | A parser |"));
        assert!(result.contains("| [`Value`](#value) | enum | A value type |"));
        assert!(result.contains("| [`parse`](#parse) | fn | Parse a string |"));
    }

    #[test]
    fn quick_ref_escapes_pipe_in_summary() {
        let generator = QuickRefGenerator::new();
        let entries = vec![QuickRefEntry::new(
            "Choice",
            "enum",
            "choice",
            "Either A | B",
        )];

        let result = generator.generate(&entries);

        assert!(result.contains(r"Either A \| B"));
    }

    // =========================================================================
    // extract_summary tests
    // =========================================================================

    #[test]
    fn extract_summary_none() {
        assert_eq!(extract_summary(None), "");
    }

    #[test]
    fn extract_summary_empty() {
        assert_eq!(extract_summary(Some("")), "");
        assert_eq!(extract_summary(Some("   ")), "");
        assert_eq!(extract_summary(Some("\n\n")), "");
    }

    #[test]
    fn extract_summary_single_line() {
        assert_eq!(extract_summary(Some("A simple parser")), "A simple parser");
    }

    #[test]
    fn extract_summary_first_sentence() {
        assert_eq!(
            extract_summary(Some("A parser. With more details.")),
            "A parser."
        );
    }

    #[test]
    fn extract_summary_multiline() {
        assert_eq!(
            extract_summary(Some("First line.\nSecond line.")),
            "First line."
        );
    }

    #[test]
    fn extract_summary_leading_whitespace() {
        assert_eq!(extract_summary(Some("\n  First line")), "First line");
    }

    #[test]
    fn extract_summary_preserves_eg_abbreviation() {
        assert_eq!(
            extract_summary(Some("Use e.g. this method. Then do more.")),
            "Use e.g. this method."
        );
    }

    #[test]
    fn extract_summary_preserves_version_numbers() {
        assert_eq!(
            extract_summary(Some("Version 1.0 is here. More info.")),
            "Version 1.0 is here."
        );
    }

    #[test]
    fn extract_summary_strips_trailing_punctuation() {
        assert_eq!(extract_summary(Some("A value,")), "A value");
        assert_eq!(extract_summary(Some("A value;")), "A value");
        assert_eq!(extract_summary(Some("A value:")), "A value");
    }

    #[test]
    fn extract_summary_keeps_trailing_period() {
        assert_eq!(
            extract_summary(Some("A complete sentence.")),
            "A complete sentence."
        );
    }

    // =========================================================================
    // extract_first_sentence tests
    // =========================================================================

    #[test]
    fn first_sentence_no_period() {
        assert_eq!(extract_first_sentence("No period here"), "No period here");
    }

    #[test]
    fn first_sentence_single_sentence() {
        assert_eq!(
            extract_first_sentence("One sentence. Two sentences."),
            "One sentence."
        );
    }

    #[test]
    fn first_sentence_ie_abbreviation() {
        assert_eq!(
            extract_first_sentence("That is i.e. an example. More text."),
            "That is i.e. an example."
        );
    }

    #[test]
    fn first_sentence_version_number() {
        assert_eq!(
            extract_first_sentence("Supports version 2.0 and up. Details follow."),
            "Supports version 2.0 and up."
        );
    }
}
