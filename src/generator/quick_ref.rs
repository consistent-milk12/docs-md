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
        _ = write!(
            md,
            "## Quick Reference\n\n\
             | Item | Kind | Description |\n\
             |------|------|-------------|\n"
        );

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
/// This function handles sentences that span multiple lines by joining
/// consecutive non-empty lines until a sentence boundary is found.
/// A blank line always terminates the paragraph.
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
/// assert_eq!(extract_summary(Some("A parser. With more.")), "A parser.");
/// assert_eq!(extract_summary(Some("Single sentence")), "Single sentence");
/// assert_eq!(extract_summary(None), "");
/// // Handles wrapped sentences:
/// assert_eq!(
///     extract_summary(Some("A long sentence that\nspans multiple lines. More.")),
///     "A long sentence that spans multiple lines."
/// );
/// ```
#[must_use]
pub fn extract_summary(docs: Option<&str>) -> String {
    let Some(docs) = docs else {
        return String::new();
    };

    // Collect lines until we find a sentence end or hit a blank line
    let mut collected = String::new();
    let mut found_content = false;

    for line in docs.lines() {
        let trimmed = line.trim();

        // Blank line terminates the first paragraph
        if trimmed.is_empty() {
            if found_content {
                break;
            }
            continue;
        }

        found_content = true;

        // Add space between lines (unless at start)
        if !collected.is_empty() {
            collected.push(' ');
        }
        collected.push_str(trimmed);

        // Check if we've found a sentence end
        if let Some(sentence) = try_extract_sentence(&collected) {
            return sentence.trim_end_matches([',', ';', ':']).to_string();
        }
    }

    if collected.is_empty() {
        return String::new();
    }

    // No sentence boundary found - return collected text
    collected.trim_end_matches([',', ';', ':']).to_string()
}

/// Common abbreviations that shouldn't end sentences.
const ABBREVIATIONS: &[&str] = &[
    "e.g.", "i.e.", "etc.", "vs.", "cf.", "Dr.", "Mr.", "Mrs.", "Ms.", "Jr.", "Sr.", "Inc.",
    "Ltd.", "Corp.", "viz.", "approx.", "dept.", "est.", "fig.", "no.", "vol.",
];

/// Try to extract a complete first sentence from text.
///
/// Returns `Some(sentence)` if a sentence boundary (`. ` not part of abbreviation
/// or version number) is found, otherwise `None`.
fn try_extract_sentence(text: &str) -> Option<String> {
    let mut search_start = 0;

    loop {
        // Find next ". " pattern
        let pos = text[search_start..].find(". ")?;
        let absolute_pos = search_start + pos;

        // Check if this is part of an abbreviation
        let prefix = &text[..=absolute_pos];
        let is_abbreviation = ABBREVIATIONS.iter().any(|abbr| prefix.ends_with(abbr));

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
        return Some(text[..=absolute_pos].to_string());
    }
}

/// Extract the first sentence from a line of text.
///
/// Handles edge cases like:
/// - "e.g." and "i.e." abbreviations
/// - Version numbers like "1.0"
/// - URLs with dots
#[cfg(test)]
fn extract_first_sentence(text: &str) -> &str {
    let mut search_start = 0;

    loop {
        // Find next ". " pattern
        let Some(pos) = text[search_start..].find(". ") else {
            // No more ". " found, return the whole text
            return text;
        };

        let absolute_pos = search_start + pos;

        // Check if this is part of an abbreviation
        let prefix = &text[..=absolute_pos];
        let is_abbreviation = ABBREVIATIONS.iter().any(|abbr| prefix.ends_with(abbr));

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
        return &text[..=absolute_pos]; // Include the period
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

    // =========================================================================
    // Multiline sentence extraction tests
    // =========================================================================

    #[test]
    fn extract_summary_wrapped_sentence() {
        assert_eq!(
            extract_summary(Some(
                "A long sentence that\nspans multiple lines. More text."
            )),
            "A long sentence that spans multiple lines."
        );
    }

    #[test]
    fn extract_summary_wrapped_no_sentence_end() {
        assert_eq!(
            extract_summary(Some("A sentence that\nspans lines")),
            "A sentence that spans lines"
        );
    }

    #[test]
    fn extract_summary_blank_line_terminates() {
        assert_eq!(
            extract_summary(Some("First paragraph\n\nSecond paragraph")),
            "First paragraph"
        );
    }

    #[test]
    fn extract_summary_wrapped_with_abbreviation() {
        assert_eq!(
            extract_summary(Some("This is e.g. an\nexample sentence. More.")),
            "This is e.g. an example sentence."
        );
    }

    #[test]
    fn extract_summary_three_lines() {
        assert_eq!(
            extract_summary(Some("Line one\nline two\nline three. Done.")),
            "Line one line two line three."
        );
    }

    #[test]
    fn extract_summary_preserves_single_sentence_behavior() {
        // Ensure single-line behavior is unchanged
        assert_eq!(extract_summary(Some("Short. More.")), "Short.");
    }
}
