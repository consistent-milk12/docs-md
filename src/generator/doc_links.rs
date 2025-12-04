//! Intra-doc link processing for documentation generation.
//!
//! This module provides [`DocLinkProcessor`] which transforms rustdoc
//! intra-doc link syntax into proper markdown links.

use crate::linker::LinkRegistry;
use regex::Regex;
use rustdoc_types::{Crate, Id, ItemKind};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Regex for HTML-style rustdoc links, used by standalone processing.
static HTML_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\((struct|enum|trait|fn|type|macro|constant)\.([A-Za-z_][A-Za-z0-9_]*)\.html(?:#([a-z]+)\.([A-Za-z_][A-Za-z0-9_]*))?\)"
    ).unwrap()
});

/// Regex for path-style reference links, used by standalone processing.
static PATH_REF_LINK_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)\]\[([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)+)\]").unwrap()
});

/// Convert HTML-style rustdoc links to markdown anchors.
///
/// This is a standalone function that can be used without creating a full
/// [`DocLinkProcessor`]. It transforms links like:
/// - `(enum.NumberPrefix.html)` → `(#numberprefix)`
/// - `(struct.Foo.html#method.bar)` → `(#bar)`
///
/// This is useful for multi-crate documentation where the full processor
/// context may not be available.
pub fn convert_html_links(docs: &str) -> String {
    let mut result = String::with_capacity(docs.len());
    let mut last_end = 0;

    for caps in HTML_LINK_RE.captures_iter(docs) {
        let full_match = caps.get(0).unwrap();
        let match_start = full_match.start();
        let match_end = full_match.end();

        // Append everything before this match
        result.push_str(&docs[last_end..match_start]);
        last_end = match_end;

        // Extract components
        let item_name = &caps[2]; // Name

        // Check if there's an anchor (group 3 and 4)
        let anchor = if let (Some(_anchor_type), Some(anchor_name)) = (caps.get(3), caps.get(4)) {
            // For method.foo, variant.Bar, etc. -> just use the name part
            anchor_name.as_str().to_lowercase()
        } else {
            // No anchor, link to the item itself
            item_name.to_lowercase()
        };

        // Convert to markdown anchor on current page
        result.push_str(&format!("(#{anchor})"));
    }

    result.push_str(&docs[last_end..]);
    result
}

/// Strip duplicate title from documentation.
///
/// Some crate/module docs start with `# title` which duplicates the generated
/// `# Crate \`name\`` or `# Module \`name\`` heading. This function detects
/// and removes such duplicate titles.
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
///
/// # Example
///
/// ```ignore
/// let docs = "# my_crate\n\nThis is the crate docs.";
/// let stripped = strip_duplicate_title(docs, "my_crate");
/// assert_eq!(stripped, "This is the crate docs.");
/// ```
pub fn strip_duplicate_title<'a>(docs: &'a str, item_name: &str) -> &'a str {
    let Some(first_line) = docs.lines().next() else {
        return docs;
    };

    // Check if line starts with "# "
    let Some(title) = first_line.strip_prefix("# ") else {
        return docs;
    };

    let title = title.trim();

    // Normalize names for comparison (handle crate-name vs crate_name)
    let normalized_title = title.replace('-', "_").to_lowercase();
    let normalized_name = item_name.replace('-', "_").to_lowercase();

    if normalized_title == normalized_name {
        // Skip the first line and any following blank lines
        let rest = &docs[first_line.len()..];
        rest.trim_start_matches('\n')
    } else {
        docs
    }
}

/// Convert path-style reference links to markdown anchors.
///
/// This is a standalone function that converts rustdoc-style reference links like:
/// - `[`ProgressTracker`][crate::style::ProgressTracker]` → `[`ProgressTracker`](#progresstracker)`
///
/// Without access to item_links, it falls back to using the last segment of the
/// path as an anchor on the current page.
pub fn convert_path_reference_links(docs: &str) -> String {
    let mut result = String::with_capacity(docs.len());
    let mut last_end = 0;

    for caps in PATH_REF_LINK_RE.captures_iter(docs) {
        let full_match = caps.get(0).unwrap();
        let match_start = full_match.start();
        let match_end = full_match.end();

        // Append everything before this match
        result.push_str(&docs[last_end..match_start]);
        last_end = match_end;

        let display_text = &caps[1];
        let rust_path = &caps[2]; // e.g., "crate::style::ProgressTracker"

        // Create an anchor from the last segment of the path
        let anchor = rust_path
            .split("::")
            .last()
            .unwrap_or(rust_path)
            .to_lowercase();
        result.push_str(&format!("[{display_text}](#{anchor})"));
    }

    result.push_str(&docs[last_end..]);
    result
}

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
///
/// # External Crate Links
///
/// Items from external crates are linked to docs.rs when possible.
pub struct DocLinkProcessor<'a> {
    /// The crate being documented (for looking up items).
    krate: &'a Crate,

    /// Registry mapping IDs to file paths.
    link_registry: &'a LinkRegistry,

    /// The current file path (for relative link calculation).
    current_file: &'a str,

    /// Regex for backtick code links: [`Name`] not followed by ( or [
    backtick_link_re: Regex,

    /// Regex for reference-style links: [display text][`ref`]
    reference_link_re: Regex,

    /// Regex for markdown reference definitions: [`Name`]: path
    reference_def_re: Regex,

    /// Regex for plain links without backticks: [name] not followed by ( or [
    /// Matches snake_case identifiers like [fields], [enter], etc.
    plain_link_re: Regex,

    /// Regex for method links: [`Type::method`] or [Type::method]
    /// Captures the type and method separately for anchor generation.
    method_link_re: Regex,

    /// Regex for HTML-style rustdoc links in existing markdown links.
    /// Matches patterns like `(enum.Name.html)` or `(struct.Foo.html#method.bar)`.
    /// These come from raw doc comments that already contain rustdoc HTML links.
    html_link_re: Regex,

    /// Regex for reference-style links without backticks: [text][crate::path::Item]
    /// These are rustdoc-style references that use Rust paths as the reference ID.
    path_reference_link_re: Regex,
}

impl<'a> DocLinkProcessor<'a> {
    /// Create a new processor for the given context.
    pub fn new(
        krate: &'a Crate,
        link_registry: &'a LinkRegistry,
        current_file: &'a str,
    ) -> Self {
        Self {
            krate,
            link_registry,
            current_file,
            // Match [`text`] - we'll filter out existing markdown links in processing
            backtick_link_re: Regex::new(r"\[`([^`]+)`\]").unwrap(),
            // Match reference-style links: [display text][`ref`]
            // Group 1: display text, Group 2: reference key (inside backticks)
            reference_link_re: Regex::new(r"\[([^\]]+)\]\[`([^`]+)`\]").unwrap(),
            // Match reference definitions like: [`Name`]: path::to::item
            // These appear at the end of doc blocks and should be stripped after processing
            reference_def_re: Regex::new(r"(?m)^\s*\[`[^`]+`\]:\s*\S+\s*$").unwrap(),
            // Match plain links [name] where name is a snake_case identifier
            // We'll check in code if it's followed by ( or [ to avoid matching existing links
            plain_link_re: Regex::new(r"\[([a-z][a-z0-9_]*)\]").unwrap(),
            // Match method links like [`Type::method`] or [`mod::Type::method`]
            // Group 1: full path (Type::method), which we split on last ::
            method_link_re: Regex::new(r"\[`([A-Z][A-Za-z0-9_]*(?:::[a-z][a-z0-9_]*)+)`\]").unwrap(),
            // Match HTML-style rustdoc links in markdown: (type.Name.html) or (type.Name.html#anchor)
            // Group 1: type prefix (struct, enum, trait, fn, type, macro, constant)
            // Group 2: item name
            // Group 3: optional anchor (method.foo, variant.Bar, etc.)
            html_link_re: Regex::new(
                r"\((struct|enum|trait|fn|type|macro|constant)\.([A-Za-z_][A-Za-z0-9_]*)\.html(?:#([a-z]+)\.([A-Za-z_][A-Za-z0-9_]*))?\)"
            ).unwrap(),
            // Match reference-style links with Rust paths: [text][crate::path::Item]
            // Group 1: display text, Group 2: full Rust path (crate::module::Item)
            path_reference_link_re: Regex::new(
                r"\[([^\]]+)\]\[([a-zA-Z_][a-zA-Z0-9_]*(?:::[a-zA-Z_][a-zA-Z0-9_]*)+)\]"
            ).unwrap(),
        }
    }

    /// Process a doc string and resolve all intra-doc links.
    ///
    /// Uses the item's `links` map to resolve link text to IDs,
    /// then uses `LinkRegistry` to convert IDs to relative paths.
    /// Finally strips any markdown reference definitions.
    pub fn process(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        // Step 1: Strip reference definitions FIRST (before link processing mangles them)
        let stripped = self.strip_reference_definitions(docs);

        // Step 2: Strip rustdoc hidden lines (# prefix) from code blocks
        let no_hidden = self.strip_hidden_code_lines(&stripped);

        // Step 3: Process reference-style links [text][`ref`] BEFORE simple links
        // This prevents [text][`ref`] from becoming [text][`ref`](url)
        let ref_resolved = self.process_reference_links(&no_hidden, item_links);

        // Step 3b: Process path reference links [text][crate::path::Item]
        let path_ref_resolved = self.process_path_reference_links(&ref_resolved, item_links);

        // Step 4: Process method links [`Type::method`] with anchor generation
        let method_resolved = self.process_method_links(&path_ref_resolved, item_links);

        // Step 5: Resolve simple backtick links [`Name`] to markdown links
        let backtick_resolved = self.process_backtick_links(&method_resolved, item_links);

        // Step 6: Process plain links [name] (snake_case identifiers)
        let plain_resolved = self.process_plain_links(&backtick_resolved, item_links);

        // Step 7: Convert HTML-style rustdoc links to markdown anchors
        let html_resolved = self.process_html_links(&plain_resolved);

        // Step 8: Clean up extra blank lines that may result from stripping
        self.clean_blank_lines(&html_resolved)
    }

    /// Unhide rustdoc hidden lines in code blocks.
    ///
    /// In rustdoc, lines starting with `# ` inside code blocks are hidden
    /// in the rendered output but still compiled. We remove the `# ` prefix
    /// to show the full working example as normal code.
    fn strip_hidden_code_lines(&self, docs: &str) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut in_code_block = false;

        for line in docs.lines() {
            let trimmed = line.trim_start();

            // Track code block boundaries
            if trimmed.starts_with("```") {
                in_code_block = !in_code_block;
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // Inside code blocks, remove the # prefix from hidden lines
            if in_code_block {
                // Get the leading whitespace
                let leading_ws = &line[..line.len() - trimmed.len()];

                if trimmed == "#" {
                    // Line is just "#" - convert to empty line
                    result.push('\n');
                    continue;
                } else if let Some(rest) = trimmed.strip_prefix("# ") {
                    // Line starts with "# " - remove prefix, keep content
                    result.push_str(leading_ws);
                    result.push_str(rest);
                    result.push('\n');
                    continue;
                }
            }

            result.push_str(line);
            result.push('\n');
        }

        // Remove trailing newline if original didn't have one
        if !docs.ends_with('\n') && result.ends_with('\n') {
            result.pop();
        }

        result
    }

    /// Process reference-style links like `[display text][`Span`]`.
    ///
    /// Converts them to inline links: `[display text](url)`
    fn process_reference_links(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in self.reference_link_re.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Append everything before this match
            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            let display_text = &caps[1];
            let ref_key = &caps[2];

            // Try to resolve the reference key to a URL
            if let Some(url) = self.resolve_to_url(ref_key, item_links) {
                // Convert to inline link: [display text](url)
                result.push_str(&format!("[{display_text}]({url})"));
            } else {
                // Keep original if we can't resolve
                result.push_str(full_match.as_str());
            }
        }

        // Append any remaining text
        result.push_str(&docs[last_end..]);
        result
    }

    /// Process path-style reference links like `[`ProgressTracker`][crate::style::ProgressTracker]`.
    ///
    /// These are rustdoc-style references without backticks around the reference part.
    /// Converts them to inline links: `[`ProgressTracker`](url)` or anchors.
    fn process_path_reference_links(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in self.path_reference_link_re.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Append everything before this match
            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            let display_text = &caps[1];
            let rust_path = &caps[2]; // e.g., "crate::style::ProgressTracker"

            // Try to resolve the Rust path to a URL
            if let Some(url) = self.resolve_to_url(rust_path, item_links) {
                // Convert to inline link: [display text](url)
                result.push_str(&format!("[{display_text}]({url})"));
            } else {
                // Fallback: create an anchor from the last segment of the path
                // e.g., crate::style::ProgressTracker -> #progresstracker
                let anchor = rust_path
                    .split("::")
                    .last()
                    .unwrap_or(rust_path)
                    .to_lowercase();
                result.push_str(&format!("[{display_text}](#{anchor})"));
            }
        }

        // Append any remaining text
        result.push_str(&docs[last_end..]);
        result
    }

    /// Resolve a link reference to just the URL (without markdown formatting).
    fn resolve_to_url(&self, link_text: &str, item_links: &HashMap<String, Id>) -> Option<String> {
        // Strategy 1: Exact match in item_links
        if let Some(id) = item_links.get(link_text) {
            if let Some(url) = self.get_url_for_id(*id) {
                return Some(url);
            }
        }

        // Strategy 2: Try the short name (last segment of path)
        let short_name = link_text.split("::").last().unwrap_or(link_text);

        for (key, id) in item_links {
            let key_short = key.split("::").last().unwrap_or(key);
            if key_short == short_name {
                if let Some(url) = self.get_url_for_id(*id) {
                    return Some(url);
                }
            }
        }

        // Strategy 3: Try to find in krate.paths
        for (id, path_info) in &self.krate.paths {
            if let Some(last_segment) = path_info.path.last()
                && last_segment == short_name
            {
                if let Some(url) = self.get_url_for_id(*id) {
                    return Some(url);
                }
            }
        }

        None
    }

    /// Get just the URL for an ID (local path or docs.rs).
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
            return self.get_docs_rs_url(path_info);
        }

        None
    }

    /// Get docs.rs URL for an external crate item.
    fn get_docs_rs_url(&self, path_info: &rustdoc_types::ItemSummary) -> Option<String> {
        let path = &path_info.path;
        if path.is_empty() {
            return None;
        }

        let crate_name = &path[0];
        let item_path = path[1..].join("/");

        let type_suffix = match path_info.kind {
            ItemKind::Struct => "struct",
            ItemKind::Enum => "enum",
            ItemKind::Trait => "trait",
            ItemKind::Function => "fn",
            ItemKind::Module => "index",
            ItemKind::Constant => "constant",
            ItemKind::TypeAlias => "type",
            ItemKind::Macro => "macro",
            _ => "index",
        };

        if item_path.is_empty() {
            Some(format!("https://docs.rs/{crate_name}/latest/{crate_name}/"))
        } else {
            Some(format!(
                "https://docs.rs/{crate_name}/latest/{crate_name}/{item_path}/{type_suffix}.{}.html",
                path.last().unwrap_or(crate_name)
            ))
        }
    }

    /// Strip markdown reference definition lines.
    ///
    /// After converting `[`Name`]` to `[`Name`](url)`, the reference
    /// definitions like `[`Name`]: path` are no longer needed.
    fn strip_reference_definitions(&self, docs: &str) -> String {
        self.reference_def_re.replace_all(docs, "").to_string()
    }

    /// Clean up multiple consecutive blank lines.
    fn clean_blank_lines(&self, docs: &str) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut prev_blank = false;

        for line in docs.lines() {
            let is_blank = line.trim().is_empty();
            if is_blank && prev_blank {
                continue; // Skip consecutive blank lines
            }
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(line);
            prev_blank = is_blank;
        }

        // Trim trailing whitespace
        result.trim_end().to_string()
    }

    /// Process `` [`Type`] `` style links.
    fn process_backtick_links(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in self.backtick_link_re.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Append everything before this match
            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            // Check if this is already a markdown link (followed by '(')
            if docs[match_end..].starts_with('(') {
                // Keep original text, it's already a link
                result.push_str(full_match.as_str());
                continue;
            }

            let link_text = &caps[1];

            // Try to resolve the link
            let resolved = self.resolve_link(link_text, item_links);
            result.push_str(&resolved);
        }

        // Append any remaining text after the last match
        result.push_str(&docs[last_end..]);
        result
    }

    /// Process plain links `[name]` (snake_case identifiers without backticks).
    ///
    /// This handles links like `[fields]` or `[enter]` that appear in rustdoc
    /// documentation without backticks.
    fn process_plain_links(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in self.plain_link_re.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Append everything before this match
            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            // Check if this is already a markdown link (followed by '(' or '[')
            let next_char = docs[match_end..].chars().next();
            if matches!(next_char, Some('(') | Some('[')) {
                // Keep original text, it's already a link
                result.push_str(full_match.as_str());
                continue;
            }

            let link_text = &caps[1];

            // Try to resolve the link
            if let Some(id) = item_links.get(link_text)
                && let Some(md_link) = self.create_link_for_id(*id, link_text)
            {
                result.push_str(&md_link);
            } else {
                // Keep original if we can't resolve
                result.push_str(full_match.as_str());
            }
        }

        result.push_str(&docs[last_end..]);
        result
    }

    /// Process method links like `[`Type::method`]` with anchor generation.
    ///
    /// Links like `[`Span::enter`]` should resolve to `span/index.md#enter`
    /// where `#enter` is an anchor to the method on the type's page.
    fn process_method_links(&self, docs: &str, item_links: &HashMap<String, Id>) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in self.method_link_re.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Append everything before this match
            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            // Check if already a markdown link
            if docs[match_end..].starts_with('(') {
                result.push_str(full_match.as_str());
                continue;
            }

            let full_path = &caps[1]; // e.g., "Span::enter"

            // Split into type and method
            if let Some(last_colon_idx) = full_path.rfind("::") {
                let type_part = &full_path[..last_colon_idx];
                let method_part = &full_path[last_colon_idx + 2..];

                // Try to resolve the type
                if let Some(md_link) = self.resolve_method_link(type_part, method_part, item_links) {
                    result.push_str(&md_link);
                } else {
                    // Keep original if we can't resolve
                    result.push_str(full_match.as_str());
                }
            } else {
                // No :: found, keep original
                result.push_str(full_match.as_str());
            }
        }

        result.push_str(&docs[last_end..]);
        result
    }

    /// Convert HTML-style rustdoc links to markdown anchors.
    ///
    /// Transforms links like `(enum.NumberPrefix.html#method.binary)` to `(#binary)`.
    /// These links appear in documentation that was written for rustdoc HTML output.
    fn process_html_links(&self, docs: &str) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in self.html_link_re.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Append everything before this match
            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            // Extract components
            let _item_type = &caps[1]; // struct, enum, trait, etc.
            let item_name = &caps[2];  // Name

            // Check if there's an anchor (group 3 and 4)
            let anchor = if let (Some(_anchor_type), Some(anchor_name)) = (caps.get(3), caps.get(4)) {
                // For method.foo, variant.Bar, etc. -> just use the name part
                anchor_name.as_str().to_lowercase()
            } else {
                // No anchor, link to the item itself
                item_name.to_lowercase()
            };

            // Convert to markdown anchor on current page
            result.push_str(&format!("(#{anchor})"));
        }

        result.push_str(&docs[last_end..]);
        result
    }

    /// Resolve a method link to a markdown link with anchor.
    fn resolve_method_link(
        &self,
        type_name: &str,
        method_name: &str,
        item_links: &HashMap<String, Id>,
    ) -> Option<String> {
        // Try to find the type in item_links
        let type_id = item_links.get(type_name).or_else(|| {
            // Try short name match
            let short_type = type_name.split("::").last().unwrap_or(type_name);
            item_links.iter()
                .find(|(k, _)| k.split("::").last() == Some(short_type))
                .map(|(_, id)| id)
        })?;

        // Get the path to the type's page
        let type_path = self.link_registry.get_path(*type_id)?;
        let relative = LinkRegistry::compute_relative_path(self.current_file, type_path);

        // Build anchor from method name
        let anchor = method_name.to_lowercase();
        let display = format!("{type_name}::{method_name}");

        Some(format!("[`{display}`]({relative}#{anchor})"))
    }

    /// Try to resolve a link text to a markdown link.
    fn resolve_link(&self, link_text: &str, item_links: &HashMap<String, Id>) -> String {
        // Strategy 1: Exact match in item_links
        if let Some(id) = item_links.get(link_text)
            && let Some(md_link) = self.create_link_for_id(*id, link_text)
        {
            return md_link;
        }

        // Strategy 2: Try the short name (last segment of path)
        let short_name = link_text.split("::").last().unwrap_or(link_text);

        for (key, id) in item_links {
            let key_short = key.split("::").last().unwrap_or(key);
            if key_short == short_name {
                if let Some(md_link) = self.create_link_for_id(*id, short_name) {
                    return md_link;
                }
            }
        }

        // Strategy 3: Try to find in krate.paths
        for (id, path_info) in &self.krate.paths {
            if let Some(last_segment) = path_info.path.last()
                && last_segment == short_name
            {
                if let Some(md_link) = self.create_link_for_id(*id, short_name) {
                    return md_link;
                }
            }
        }

        // Fallback: Return original text
        format!("[`{link_text}`]")
    }

    /// Create a markdown link for a given ID.
    ///
    /// First tries to link to local documentation, then falls back to
    /// docs.rs for external crate items.
    fn create_link_for_id(&self, id: Id, display_name: &str) -> Option<String> {
        // Strategy 1: Local link via LinkRegistry
        if let Some(link) = self.link_registry.create_link(id, self.current_file) {
            return Some(link);
        }

        if let Some(path) = self.link_registry.get_path(id) {
            let relative = LinkRegistry::compute_relative_path(self.current_file, path);
            let clean_name = display_name.split("::").last().unwrap_or(display_name);
            return Some(format!("[`{clean_name}`]({relative})"));
        }

        // Strategy 2: External crate link via docs.rs
        if let Some(path_info) = self.krate.paths.get(&id) {
            // External crates have crate_id != 0
            if path_info.crate_id != 0 {
                return self.create_docs_rs_link(path_info, display_name);
            }
        }

        None
    }

    /// Create a docs.rs link for an external crate item.
    fn create_docs_rs_link(
        &self,
        path_info: &rustdoc_types::ItemSummary,
        display_name: &str,
    ) -> Option<String> {
        let path = &path_info.path;
        if path.is_empty() {
            return None;
        }

        let crate_name = &path[0];
        let item_path = path[1..].join("/");
        let clean_name = display_name.split("::").last().unwrap_or(display_name);

        // Determine the item type suffix for docs.rs URLs
        let type_suffix = match path_info.kind {
            ItemKind::Struct => "struct",
            ItemKind::Enum => "enum",
            ItemKind::Trait => "trait",
            ItemKind::Function => "fn",
            ItemKind::Module => "index",
            ItemKind::Constant => "constant",
            ItemKind::TypeAlias => "type",
            ItemKind::Macro => "macro",
            _ => "index",
        };

        // Build docs.rs URL
        // Format: https://docs.rs/crate_name/latest/crate_name/path/type.Name.html
        let url = if item_path.is_empty() {
            format!("https://docs.rs/{crate_name}/latest/{crate_name}/")
        } else {
            format!(
                "https://docs.rs/{crate_name}/latest/{crate_name}/{item_path}/{type_suffix}.{}.html",
                path.last().unwrap_or(crate_name)
            )
        };

        Some(format!("[`{clean_name}`]({url})"))
    }
}
