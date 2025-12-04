//! Intra-doc link processing for documentation generation.
//!
//! This module provides [`DocLinkProcessor`] which transforms rustdoc
//! intra-doc link syntax into proper markdown links.

use crate::linker::LinkRegistry;
use regex::Regex;
use rustdoc_types::{Crate, Id, ItemKind};
use std::collections::HashMap;

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

        // Step 4: Resolve simple backtick links [`Name`] to markdown links
        let resolved = self.process_backtick_links(&ref_resolved, item_links);

        // Step 5: Clean up extra blank lines that may result from stripping
        self.clean_blank_lines(&resolved)
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
