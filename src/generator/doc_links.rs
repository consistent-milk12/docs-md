//! Intra-doc link processing for documentation generation.
//!
//! This module provides [`DocLinkProcessor`] which transforms rustdoc
//! intra-doc link syntax into proper markdown links.
//!
//! # Processing Pipeline
//! The processor applies transformations in this order:
//! 1. Strip markdown reference definitions
//! 2. Unhide rustdoc hidden lines in code blocks
//! 3. Process reference-style links `[text][`ref`]`
//! 4. Process path reference links `[text][crate::path]`
//! 5. Process method links `[Type::method]`
//! 6. Process backtick links `[`Name`]`
//! 7. Process plain links `[name]`
//! 8. Convert HTML-style rustdoc links
//! 9. Clean up blank lines
//!
//! Links inside code blocks are protected from transformation.

use crate::linker::LinkRegistry;
use regex::Regex;
use rustdoc_types::{Crate, Id, ItemKind};
use std::collections::HashMap;
use std::sync::LazyLock;

// =============================================================================
// Static Regex Patterns (compiled once, reused everywhere)
// =============================================================================

/// Regex for HTML-style rustdoc links.
/// Matches: `(struct.Name.html)` or `(enum.Name.html#method.foo)`
static HTML_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(concat!(
        r"\((struct|enum|trait|fn|type|macro|constant|mod)\.",
        r"([A-Za-z_][A-Za-z0-9_]*)\.html",
        r"(?:#([a-z]+)\.([A-Za-z_][A-Za-z0-9_]*))?\)",
    ))
    .unwrap()
});

/// Regex for path-style reference links.
///
/// Matches: `[display][crate::path::Item]`
///
/// Used for rustdoc's reference-style intra-doc links where the display text
/// differs from the path reference.
///
/// # Capture Groups
/// - Group 1: Display text (anything except `]`)
/// - Group 2: Rust path with `::` separators (e.g., `crate::module::Item`)
///
/// # Pattern Breakdown
/// ```text
/// \[([^\]]+)\]              # [display text] - capture non-] chars
/// \[                        # Opening bracket for reference
/// ([a-zA-Z_][a-zA-Z0-9_]*   # First path segment (valid Rust identifier)
/// (?:::[a-zA-Z_][a-zA-Z0-9_]*)+  # One or more ::segment pairs (requires at least one ::)
/// )\]                       # Close capture and bracket
/// ```
///
/// # Note
/// The pattern requires at least one `::` separator, so it won't match
/// single identifiers like `[text][Name]`.
static PATH_REF_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)\]\[([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)+)\]").unwrap()
});

/// Regex for backtick code links.
///
/// Matches: `` [`Name`] `` (the most common intra-doc link format)
///
/// This is the primary pattern for rustdoc intra-doc links. The backticks
/// indicate the link should be rendered as inline code.
///
/// # Capture Groups
/// - Group 1: The link text inside backticks (e.g., `Name`, `path::Item`)
///
/// # Pattern Breakdown
/// ```text
/// \[`        # Literal "[`" - opening bracket and backtick
/// ([^`]+)    # Capture: one or more non-backtick characters
/// `\]        # Literal "`]" - closing backtick and bracket
/// ```
///
/// # Processing Note
/// The code checks if the match is followed by `(` to avoid double-processing
/// already-converted markdown links like `` [`Name`](url) ``.
static BACKTICK_LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[`([^`]+)`\]").unwrap());

/// Regex for reference-style links with backticks.
///
/// Matches: `` [display text][`ref`] ``
///
/// This pattern handles rustdoc reference-style links where custom display
/// text links to a backtick-wrapped reference.
///
/// # Capture Groups
/// - Group 1: Display text (what the user sees)
/// - Group 2: Reference text inside backticks (the actual link target)
///
/// # Pattern Breakdown
/// ```text
/// \[([^\]]+)\]   # [display text] - capture anything except ]
/// \[`            # Opening "[`" for the reference
/// ([^`]+)        # Capture: reference name (non-backtick chars)
/// `\]            # Closing "`]"
/// ```
///
/// # Example
/// `` [custom text][`HashMap`] `` renders as "custom text" linking to `HashMap`.
static REFERENCE_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([^\]]+)\]\[`([^`]+)`\]").unwrap());

/// Regex for markdown reference definitions.
///
/// Matches: `` [`Name`]: path::to::item `` at line start
///
/// These are markdown reference definition lines that rustdoc uses internally.
/// We strip these from output since intra-doc links are resolved directly.
///
/// # Pattern Breakdown
/// ```text
/// (?m)       # Multi-line mode: ^ and $ match line boundaries
/// ^          # Start of line
/// \s*        # Optional leading whitespace
/// \[`[^`]+`\]  # Backtick link syntax (not captured)
/// :          # Literal colon separator
/// \s*        # Optional whitespace after colon
/// \S+        # The target path (non-whitespace chars)
/// \s*        # Optional trailing whitespace
/// $          # End of line
/// ```
///
/// # Note
/// This pattern doesn't capture groups because it's used with `replace_all`
/// to remove entire lines.
static REFERENCE_DEF_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^\s*\[`[^`]+`\]:\s*\S+\s*$").unwrap());

/// Regex for plain identifier links.
///
/// Matches: `[name]` where name is a valid Rust identifier
///
/// This handles the simplest intra-doc link format without backticks.
/// Used less frequently than backtick links but still valid rustdoc syntax.
///
/// # Capture Groups
/// - Group 1: The identifier name
///
/// # Pattern Breakdown
/// ```text
/// \[                      # Opening bracket
/// ([a-zA-Z_]              # Capture start: letter or underscore (Rust identifier rules)
/// [a-zA-Z0-9_]*)          # Followed by alphanumeric or underscore
/// \]                      # Closing bracket
/// ```
///
/// # Processing Note
/// The code checks if the match is followed by `(` or `[` to avoid
/// false positives on existing markdown links or reference-style links.
/// Also only processes if the identifier exists in `item_links`.
static PLAIN_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([a-zA-Z_][a-zA-Z0-9_]*)\]").unwrap());

/// Regex for method/associated item links.
///
/// Matches: `` [`Type::method`] `` or `` [`mod::Type::CONST`] ``
///
/// Handles links to methods, associated functions, constants, and other
/// items accessed via `::` path notation. This includes both type-level
/// paths (`Type::method`) and module-level paths (`mod::Type::CONST`).
///
/// # Capture Groups
/// - Group 1: The full path including `::` separators
///
/// # Pattern Breakdown
/// ```text
/// \[`                              # Opening "[`"
/// (                                # Start capture group
///   [A-Za-z_][A-Za-z0-9_]*         # First segment (Rust identifier)
///   (?:::[A-Za-z_][A-Za-z0-9_]*)+  # One or more ::segment pairs
/// )                                # End capture group
/// `\]                              # Closing "`]"
/// ```
///
/// # Examples Matched
/// - `` [`HashMap::new`] `` - associated function
/// - `` [`Option::Some`] `` - enum variant
/// - `` [`Iterator::next`] `` - trait method
/// - `` [`std::vec::Vec`] `` - fully qualified path
///
/// # Processing Note
/// The last segment after `::` is used as the anchor (lowercased).
/// The type path before the last `::` is used to find the target file.
static METHOD_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[`([A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)+)`\]").unwrap()
});

// =============================================================================
// Standalone Functions
// =============================================================================

/// Convert HTML-style rustdoc links to markdown anchors.
///
/// Transforms links like:
/// - `(enum.NumberPrefix.html)` -> `(#numberprefix)`
/// - `(struct.Foo.html#method.bar)` -> `(#bar)`
///
/// This is useful for multi-crate documentation where the full processor
/// context may not be available.
#[must_use]
pub fn convert_html_links(docs: &str) -> String {
    replace_with_regex(docs, &HTML_LINK_RE, |caps| {
        let item_name = &caps[2];

        let anchor = caps
            .get(4)
            .map_or_else(|| item_name.to_lowercase(), |m| m.as_str().to_lowercase());

        format!("(#{anchor})")
    })
}

/// Strip duplicate title from documentation.
///
/// Some crate/module docs start with `# title` which duplicates the generated
/// `# Crate \`name\`` or `# Module \`name\`` heading.
///
/// # Arguments
///
/// * `docs` - The documentation string to process
/// * `item_name` - The name of the crate or module being documented
///
/// # Returns
///
/// The docs with the leading title removed if it matches the item name,
/// otherwise the original docs unchanged.
#[must_use]
pub fn strip_duplicate_title<'a>(docs: &'a str, item_name: &str) -> &'a str {
    let Some(first_line) = docs.lines().next() else {
        return docs;
    };

    let Some(title) = first_line.strip_prefix("# ") else {
        return docs;
    };

    // Normalize names for comparison (handle crate-name vs crate_name)
    let normalized_title = title.trim().replace('-', "_").to_lowercase();
    let normalized_name = item_name.replace('-', "_").to_lowercase();

    if normalized_title == normalized_name {
        // Skip the first line and any following blank lines
        docs[first_line.len()..].trim_start_matches('\n')
    } else {
        docs
    }
}

/// Strip markdown reference definition lines.
///
/// Removes lines like `[`Name`]: path::to::item` which are no longer needed
/// after intra-doc links are processed.
pub fn strip_reference_definitions(docs: &str) -> String {
    REFERENCE_DEF_RE.replace_all(docs, "").to_string()
}

/// Convert path-style reference links to markdown anchors.
///
/// Transforms: `[``ProgressTracker``][crate::style::ProgressTracker]`
/// Into: `[``ProgressTracker``](#progresstracker)`
///
/// Uses the last segment of the path as an anchor on the current page.
#[must_use]
pub fn convert_path_reference_links(docs: &str) -> String {
    replace_with_regex(docs, &PATH_REF_LINK_RE, |caps| {
        let display_text = &caps[1];
        let rust_path = &caps[2];
        let anchor = rust_path
            .split("::")
            .last()
            .unwrap_or(rust_path)
            .to_lowercase();
        format!("[{display_text}](#{anchor})")
    })
}

// =============================================================================
// DocLinkProcessor
// =============================================================================

/// Processes doc comments to resolve intra-doc links to markdown links.
///
/// Rustdoc JSON includes a `links` field on each Item that maps intra-doc
/// link text to item IDs. This processor uses that map along with the
/// `LinkRegistry` to convert these to relative markdown links.
///
/// # Supported Patterns
///
/// - `` [`Name`] `` - Backtick code links (most common)
/// - `` [`path::to::Item`] `` - Qualified path links
/// - `` [`Type::method`] `` - Method/associated item links
/// - `[name]` - Plain identifier links
/// - `[text][`ref`]` - Reference-style links
/// - `[text][crate::path]` - Path reference links
///
/// # External Crate Links
///
/// Items from external crates are linked to docs.rs when possible.
///
/// # Code Block Protection
///
/// Links inside fenced code blocks (```) are not processed.
pub struct DocLinkProcessor<'a> {
    /// The crate being documented (for looking up items).
    krate: &'a Crate,

    /// Registry mapping IDs to file paths.
    link_registry: &'a LinkRegistry,

    /// The current file path (for relative link calculation).
    current_file: &'a str,

    /// Index mapping item names to their IDs for fast lookup.
    /// Built from `krate.paths` at construction time.
    path_name_index: HashMap<&'a str, Vec<Id>>,
}

impl<'a> DocLinkProcessor<'a> {
    /// Create a new processor for the given context.
    #[must_use]
    pub fn new(krate: &'a Crate, link_registry: &'a LinkRegistry, current_file: &'a str) -> Self {
        // Build path name index for O(1) lookups
        let mut path_name_index: HashMap<&'a str, Vec<Id>> = HashMap::new();
        for (id, path_info) in &krate.paths {
            if let Some(name) = path_info.path.last() {
                path_name_index.entry(name.as_str()).or_default().push(*id);
            }
        }

        Self {
            krate,
            link_registry,
            current_file,
            path_name_index,
        }
    }

    /// Process a doc string and resolve all intra-doc links.
    ///
    /// Uses the item's `links` map to resolve link text to IDs,
    /// then uses `LinkRegistry` to convert IDs to relative paths.
    #[must_use]
    pub fn process(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        // Step 1: Strip reference definitions first
        let stripped = strip_reference_definitions(docs);

        // Step 2: Unhide rustdoc hidden lines in code blocks
        let unhidden = Self::unhide_code_lines(&stripped);

        // Step 3: Process all link types (with code block protection)
        let processed = self.process_links_protected(&unhidden, item_links);

        // Step 4: Clean up blank lines
        Self::clean_blank_lines(&processed)
    }

    /// Process links while protecting code block contents.
    fn process_links_protected(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut current_pos = 0;
        let _bytes = docs.as_bytes();

        // Track code block state
        let mut in_code_block = false;
        let mut fence: Option<&str> = None;

        for line in docs.lines() {
            let line_end = current_pos + line.len();

            // Check for code fence
            let trimmed = line.trim_start();
            if let Some(f) = Self::detect_fence(trimmed) {
                if in_code_block {
                    // Check if this closes the current block
                    if let Some(open_fence) = fence
                        && trimmed.starts_with(open_fence)
                    {
                        in_code_block = false;
                        fence = None;
                    }
                } else {
                    in_code_block = true;
                    fence = Some(f);
                }

                result.push_str(line);
            } else if in_code_block {
                // Inside code block - don't process
                result.push_str(line);
            } else {
                // Outside code block - process links
                let processed = self.process_line(line, item_links);
                result.push_str(&processed);
            }

            // Add newline if not at end
            current_pos = line_end;
            if current_pos < docs.len() {
                result.push('\n');
                current_pos += 1; // Skip the newline character
            }
        }

        result
    }

    /// Detect a code fence and return the fence string.
    fn detect_fence(trimmed: &str) -> Option<&'static str> {
        if trimmed.starts_with("```") {
            Some("```")
        } else if trimmed.starts_with("~~~") {
            Some("~~~")
        } else {
            None
        }
    }

    /// Process a single line for all link types.
    fn process_line(&self, line: &str, item_links: &HashMap<String, Id>) -> String {
        // Skip lines that look like reference definitions (backup check)
        if line.trim_start().starts_with("[`") && line.contains("]:") {
            return String::new();
        }

        // Process in order of specificity (most specific patterns first)
        let s = self.process_reference_links(line, item_links);
        let s = self.process_path_reference_links(&s, item_links);
        let s = self.process_method_links(&s, item_links);
        let s = self.process_backtick_links(&s, item_links);
        let s = self.process_plain_links(&s, item_links);

        Self::process_html_links(&s)
    }

    /// Unhide rustdoc hidden lines in code blocks.
    ///
    /// Lines starting with `# ` inside code blocks are hidden in rustdoc
    /// but compiled. We remove the prefix to show the full example.
    fn unhide_code_lines(docs: &str) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut in_code_block = false;
        let mut fence: Option<&str> = None;

        for line in docs.lines() {
            let trimmed = line.trim_start();

            // Track code block boundaries
            if let Some(f) = Self::detect_fence(trimmed) {
                if in_code_block && fence.map_or_else(|| false, |open| trimmed.starts_with(open)) {
                    in_code_block = false;
                    fence = None;
                } else if !in_code_block {
                    in_code_block = true;
                    fence = Some(f);
                }

                result.push_str(line);
                result.push('\n');

                continue;
            }

            if in_code_block {
                let leading_ws = &line[..line.len() - trimmed.len()];

                if trimmed == "#" {
                    // Just "#" becomes empty line
                    result.push('\n');
                } else if let Some(rest) = trimmed.strip_prefix("# ") {
                    // "# code" becomes "code"
                    result.push_str(leading_ws);
                    result.push_str(rest);
                    result.push('\n');
                } else {
                    result.push_str(line);
                    result.push('\n');
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        // Remove trailing newline if original didn't have one
        if !docs.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }

        result
    }

    /// Process reference-style links `[display text][`Span`]`.
    fn process_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String {
        replace_with_regex(text, &REFERENCE_LINK_RE, |caps| {
            let display_text = &caps[1];
            let ref_key = &caps[2];

            if let Some(url) = self.resolve_to_url(ref_key, item_links) {
                format!("[{display_text}]({url})")
            } else {
                caps[0].to_string()
            }
        })
    }

    /// Process path reference links `[text][crate::path::Item]`.
    fn process_path_reference_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String {
        replace_with_regex(text, &PATH_REF_LINK_RE, |caps| {
            let display_text = &caps[1];
            let rust_path = &caps[2];

            if let Some(url) = self.resolve_to_url(rust_path, item_links) {
                format!("[{display_text}]({url})")
            } else {
                // Fallback: anchor from last segment
                let anchor = rust_path
                    .split("::")
                    .last()
                    .unwrap_or(rust_path)
                    .to_lowercase();
                format!("[{display_text}](#{anchor})")
            }
        })
    }

    /// Process method links `[``Type::method``]`.
    fn process_method_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String {
        replace_with_regex_checked(text, &METHOD_LINK_RE, |caps, rest| {
            // Skip if already a markdown link
            if rest.starts_with('(') {
                return caps[0].to_string();
            }

            let full_path = &caps[1];
            if let Some(last_sep) = full_path.rfind("::") {
                let type_part = &full_path[..last_sep];
                let method_part = &full_path[last_sep + 2..];

                if let Some(link) = self.resolve_method_link(type_part, method_part, item_links) {
                    return link;
                }
            }
            caps[0].to_string()
        })
    }

    /// Process backtick links `[`Name`]`.
    fn process_backtick_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String {
        replace_with_regex_checked(text, &BACKTICK_LINK_RE, |caps, rest| {
            // Skip if already a markdown link
            if rest.starts_with('(') {
                return caps[0].to_string();
            }

            let link_text = &caps[1];
            self.resolve_link(link_text, item_links)
        })
    }

    /// Process plain links `[name]`.
    fn process_plain_links(&self, text: &str, item_links: &HashMap<String, Id>) -> String {
        replace_with_regex_checked(text, &PLAIN_LINK_RE, |caps, rest| {
            // Skip if already a markdown link
            if matches!(rest.chars().next(), Some('(' | '[')) {
                return caps[0].to_string();
            }

            let link_text = &caps[1];

            // Only process if it's in item_links (avoid false positives)
            if let Some(id) = item_links.get(link_text)
                && let Some(md_link) = self.create_link_for_id(*id, link_text)
            {
                return md_link;
            }
            caps[0].to_string()
        })
    }

    /// Process HTML-style rustdoc links.
    fn process_html_links(text: &str) -> String {
        replace_with_regex(text, &HTML_LINK_RE, |caps| {
            let item_name = &caps[2];
            let anchor = caps
                .get(4)
                .map_or_else(|| item_name.to_lowercase(), |m| m.as_str().to_lowercase());
            format!("(#{anchor})")
        })
    }

    /// Clean up multiple consecutive blank lines.
    fn clean_blank_lines(docs: &str) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut prev_blank = false;

        for line in docs.lines() {
            let is_blank = line.trim().is_empty();
            if is_blank && prev_blank {
                continue;
            }
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(line);
            prev_blank = is_blank;
        }

        result.trim_end().to_string()
    }

    // =========================================================================
    // Resolution Methods
    // =========================================================================

    /// Resolve a link reference to a URL.
    fn resolve_to_url(&self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String> {
        // Strategy 1: Exact match in item_links
        if let Some(id) = item_links.get(link_text)
            && let Some(url) = self.get_url_for_id(*id)
        {
            return Some(url);
        }

        // Strategy 2: Short name match in item_links
        let short_name = link_text.split("::").last().unwrap_or(link_text);

        for (key, id) in item_links {
            if key.split("::").last() == Some(short_name)
                && let Some(url) = self.get_url_for_id(*id)
            {
                return Some(url);
            }
        }

        // Strategy 3: Use path name index
        if let Some(ids) = self.path_name_index.get(short_name) {
            for id in ids {
                if let Some(url) = self.get_url_for_id(*id) {
                    return Some(url);
                }
            }
        }

        None
    }

    /// Get the URL for an ID (local or docs.rs).
    fn get_url_for_id(&self, id: Id) -> Option<String> {
        // Try local first
        if let Some(path) = self.link_registry.get_path(id) {
            let relative = LinkRegistry::compute_relative_path(self.current_file, path);
            return Some(relative);
        }

        // Try docs.rs for external crates
        if let Some(path_info) = self.krate.paths.get(&id)
            && path_info.crate_id != 0
        {
            return Self::get_docs_rs_url(path_info);
        }

        None
    }

    /// Get docs.rs URL for an external crate item.
    fn get_docs_rs_url(path_info: &rustdoc_types::ItemSummary) -> Option<String> {
        let path = &path_info.path;
        if path.is_empty() {
            return None;
        }

        let crate_name = &path[0];

        // Handle module URLs specially
        if path_info.kind == ItemKind::Module {
            if path.len() == 1 {
                return Some(format!("https://docs.rs/{crate_name}/latest/{crate_name}/"));
            }

            let module_path = path[1..].join("/");

            return Some(format!(
                "https://docs.rs/{crate_name}/latest/{crate_name}/{module_path}/index.html"
            ));
        }

        let item_path = path[1..].join("/");
        let type_prefix = match path_info.kind {
            ItemKind::Struct => "struct",
            ItemKind::Enum => "enum",
            ItemKind::Trait => "trait",
            ItemKind::Function => "fn",
            ItemKind::Constant => "constant",
            ItemKind::TypeAlias => "type",
            ItemKind::Macro => "macro",
            _ => "index",
        };

        let item_name = path.last().unwrap_or(crate_name);

        if item_path.is_empty() {
            Some(format!("https://docs.rs/{crate_name}/latest/{crate_name}/"))
        } else {
            // Remove last segment from path for the directory
            let dir_path = if path.len() > 2 {
                path[1..path.len() - 1].join("/")
            } else {
                String::new()
            };

            if dir_path.is_empty() {
                Some(format!(
                    "https://docs.rs/{crate_name}/latest/{crate_name}/{type_prefix}.{item_name}.html"
                ))
            } else {
                Some(format!(
                    "https://docs.rs/{crate_name}/latest/{crate_name}/{dir_path}/{type_prefix}.{item_name}.html"
                ))
            }
        }
    }

    /// Resolve a method link to a markdown link with anchor.
    fn resolve_method_link(
        &self,
        type_name: &str,
        method_name: &str,
        item_links: &HashMap<String, Id>,
    ) -> Option<String> {
        // Try to find the type
        let type_id = item_links.get(type_name).or_else(|| {
            let short_type = type_name.split("::").last().unwrap_or(type_name);
            item_links
                .iter()
                .find(|(k, _)| k.split("::").last() == Some(short_type))
                .map(|(_, id)| id)
        })?;

        let type_path = self.link_registry.get_path(*type_id)?;
        let relative = LinkRegistry::compute_relative_path(self.current_file, type_path);
        let anchor = method_name.to_lowercase();
        let display = format!("{type_name}::{method_name}");

        Some(format!("[`{display}`]({relative}#{anchor})"))
    }

    /// Try to resolve link text to a markdown link.
    fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>) -> String {
        // Strategy 1: Exact match
        if let Some(id) = item_links.get(link_text)
            && let Some(md_link) = self.create_link_for_id(*id, link_text)
        {
            return md_link;
        }

        // Strategy 2: Short name match in item_links
        let short_name = link_text.split("::").last().unwrap_or(link_text);

        for (key, id) in item_links {
            if key.split("::").last() == Some(short_name)
                && let Some(md_link) = self.create_link_for_id(*id, short_name)
            {
                return md_link;
            }
        }

        // Strategy 3: Use path name index
        if let Some(ids) = self.path_name_index.get(short_name) {
            for id in ids {
                if let Some(md_link) = self.create_link_for_id(*id, short_name) {
                    return md_link;
                }
            }
        }

        // Fallback: return original
        format!("[`{link_text}`]")
    }

    /// Create a markdown link for an ID.
    fn create_link_for_id(&self, id: Id, display_name: &str) -> Option<String> {
        // Try local link
        if let Some(link) = self.link_registry.create_link(id, self.current_file) {
            return Some(link);
        }

        if let Some(path) = self.link_registry.get_path(id) {
            let relative = LinkRegistry::compute_relative_path(self.current_file, path);
            let clean_name = display_name.split("::").last().unwrap_or(display_name);
            return Some(format!("[`{clean_name}`]({relative})"));
        }

        // Try docs.rs for external crates
        if let Some(path_info) = self.krate.paths.get(&id)
            && path_info.crate_id != 0
        {
            return Self::create_docs_rs_link(path_info, display_name);
        }

        None
    }

    /// Create a docs.rs link for an external crate item.
    fn create_docs_rs_link(
        path_info: &rustdoc_types::ItemSummary,
        display_name: &str,
    ) -> Option<String> {
        let url = Self::get_docs_rs_url(path_info)?;
        let clean_name = display_name.split("::").last().unwrap_or(display_name);
        Some(format!("[`{clean_name}`]({url})"))
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Replace regex matches using a closure.
fn replace_with_regex<F>(text: &str, re: &Regex, replacer: F) -> String
where
    F: Fn(&regex::Captures<'_>) -> String,
{
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for caps in re.captures_iter(text) {
        let m = caps.get(0).unwrap();
        result.push_str(&text[last_end..m.start()]);
        result.push_str(&replacer(&caps));
        last_end = m.end();
    }

    result.push_str(&text[last_end..]);
    result
}

/// Replace regex matches with access to the text after the match.
fn replace_with_regex_checked<F>(text: &str, re: &Regex, replacer: F) -> String
where
    F: Fn(&regex::Captures<'_>, &str) -> String,
{
    let mut result = String::with_capacity(text.len());
    let mut last_end = 0;

    for caps in re.captures_iter(text) {
        let m = caps.get(0).unwrap();
        result.push_str(&text[last_end..m.start()]);
        let rest = &text[m.end()..];
        result.push_str(&replacer(&caps, rest));
        last_end = m.end();
    }

    result.push_str(&text[last_end..]);
    result
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_html_links() {
        assert_eq!(
            convert_html_links("See (enum.Foo.html) for details"),
            "See (#foo) for details"
        );
        assert_eq!(
            convert_html_links("Call (struct.Bar.html#method.new)"),
            "Call (#new)"
        );
    }

    #[test]
    fn test_strip_duplicate_title() {
        let docs = "# my_crate\n\nThis is the description.";
        assert_eq!(
            strip_duplicate_title(docs, "my_crate"),
            "This is the description."
        );

        // Different title - keep it
        let docs2 = "# Introduction\n\nThis is the description.";
        assert_eq!(strip_duplicate_title(docs2, "my_crate"), docs2);
    }

    #[test]
    fn test_strip_reference_definitions() {
        let docs = "See [`Foo`] for details.\n\n[`Foo`]: crate::Foo";
        let result = strip_reference_definitions(docs);
        assert!(result.contains("See [`Foo`]"));
        assert!(!result.contains("[`Foo`]: crate::Foo"));
    }

    #[test]
    fn test_convert_path_reference_links() {
        let docs = "[`Tracker`][crate::style::Tracker] is useful";
        let result = convert_path_reference_links(docs);
        assert_eq!(result, "[`Tracker`](#tracker) is useful");
    }
}
