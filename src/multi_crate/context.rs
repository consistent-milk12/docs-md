//! Multi-crate generation context.
//!
//! This module provides [`MultiCrateContext`] which holds shared state
//! during multi-crate documentation generation, and [`SingleCrateView`]
//! which provides a single-crate interface for existing rendering code.

use std::collections::HashMap;
use std::fmt::Write;
use std::sync::LazyLock;

use regex::Regex;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Visibility};

use crate::Args;
use crate::generator::RenderContext;
use crate::generator::doc_links::{
    convert_html_links, convert_path_reference_links, strip_duplicate_title,
    strip_reference_definitions, unhide_code_lines,
};
use crate::linker::{LinkRegistry, slugify_anchor};
use crate::multi_crate::{CrateCollection, UnifiedLinkRegistry};

/// Regex for backtick code links: [`Name`] not followed by ( or [
static BACKTICK_LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[`([^`]+)`\]").unwrap());

/// Regex for plain links [name] where name is `snake_case`
static PLAIN_LINK_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\[([a-z][a-z0-9_]*)\]").unwrap());

/// Shared context for multi-crate documentation generation.
///
/// Holds references to all crates, the unified link registry, and
/// CLI configuration. Used by [`MultiCrateGenerator`] to coordinate
/// generation across crates.
///
/// [`MultiCrateGenerator`]: crate::multi_crate::MultiCrateGenerator
pub struct MultiCrateContext<'a> {
    /// All crates being documented.
    crates: &'a CrateCollection,

    /// Unified link registry for cross-crate resolution.
    registry: UnifiedLinkRegistry,

    /// CLI arguments.
    args: &'a Args,
}

impl<'a> MultiCrateContext<'a> {
    /// Create a new multi-crate context.
    ///
    /// Builds the unified link registry from all crates.
    #[must_use]
    pub fn new(crates: &'a CrateCollection, args: &'a Args) -> Self {
        let primary = args.primary_crate.as_deref();
        let registry = UnifiedLinkRegistry::build(crates, primary);

        Self {
            crates,
            registry,
            args,
        }
    }

    /// Get the crate collection.
    #[must_use]
    pub const fn crates(&self) -> &CrateCollection {
        self.crates
    }

    /// Get the unified link registry.
    #[must_use]
    pub const fn registry(&self) -> &UnifiedLinkRegistry {
        &self.registry
    }

    /// Get CLI arguments.
    #[must_use]
    pub const fn args(&self) -> &Args {
        self.args
    }

    /// Create a single-crate view for rendering one crate.
    ///
    /// This bridges multi-crate mode to existing single-crate rendering
    /// code by providing a compatible interface that uses the unified
    /// registry for cross-crate link resolution.
    #[must_use]
    pub fn single_crate_view(&'a self, crate_name: &str) -> Option<SingleCrateView<'a>> {
        let krate = self.crates.get(crate_name)?;

        Some(SingleCrateView::new(
            crate_name,
            krate,
            &self.registry,
            self.args,
            self,
        ))
    }

    /// Find an item across all crates by ID.
    ///
    /// Searches through all crates in the collection to find an item with
    /// the given ID. This is useful for resolving re-exports that point to
    /// items in external crates.
    ///
    /// # Returns
    ///
    /// A tuple of `(crate_name, item)` if found, or `None` if the item
    /// doesn't exist in any crate.
    #[must_use]
    pub fn find_item(&self, id: &Id) -> Option<(&str, &Item)> {
        for (crate_name, krate) in self.crates.iter() {
            if let Some(item) = krate.index.get(id) {
                return Some((crate_name, item));
            }
        }
        None
    }

    /// Find cross-crate impl blocks for types in the target crate.
    ///
    /// Scans all other crates for impl blocks that target types from the
    /// specified crate. Returns a map from type name to impl blocks.
    ///
    /// This is used to show trait implementations from dependency crates
    /// on types from the target crate.
    #[must_use]
    pub fn find_cross_crate_impls(&self, target_crate: &str) -> HashMap<String, Vec<&Impl>> {
        let mut result: HashMap<String, Vec<&Impl>> = HashMap::new();

        for (source_crate, krate) in self.crates.iter() {
            // Skip the target crate itself
            if source_crate == target_crate {
                continue;
            }

            // Find impls in this crate that target types from target_crate
            for item in krate.index.values() {
                if let ItemEnum::Impl(impl_block) = &item.inner {
                    // Skip synthetic impls
                    if impl_block.is_synthetic {
                        continue;
                    }

                    // Check if this impl targets a type from target_crate
                    if let Some(type_path) = Self::get_impl_target_path(impl_block) {
                        // Check if the path starts with the target crate name
                        if let Some(first_segment) = type_path.split("::").next()
                            && first_segment == target_crate
                        {
                            // Extract the type name (last segment)
                            let type_name = type_path
                                .split("::")
                                .last()
                                .unwrap_or(&type_path)
                                .to_string();

                            result.entry(type_name).or_default().push(impl_block);
                        }
                    }
                }
            }
        }

        result
    }

    /// Get the target type path for an impl block.
    fn get_impl_target_path(impl_block: &Impl) -> Option<String> {
        use rustdoc_types::Type;

        match &impl_block.for_ {
            Type::ResolvedPath(path) => Some(path.path.clone()),
            _ => None,
        }
    }
}

/// View of a single crate within multi-crate context.
///
/// Provides an interface similar to [`GeneratorContext`] but uses
/// [`UnifiedLinkRegistry`] for cross-crate link resolution. This
/// allows existing rendering code to work with minimal changes.
///
/// [`GeneratorContext`]: crate::generator::GeneratorContext
pub struct SingleCrateView<'a> {
    /// Name of this crate.
    crate_name: String,

    /// The crate being rendered.
    krate: &'a Crate,

    /// Unified registry for link resolution.
    registry: &'a UnifiedLinkRegistry,

    /// CLI arguments.
    args: &'a Args,

    /// Reference to the parent multi-crate context for cross-crate lookups.
    ctx: &'a MultiCrateContext<'a>,

    /// Map from item ID to module path components.
    path_map: HashMap<Id, Vec<String>>,

    /// Map from type ID to impl blocks (local crate only).
    impl_map: HashMap<Id, Vec<&'a Impl>>,

    /// Map from type name to impl blocks from other crates.
    /// These are trait implementations from dependency crates.
    cross_crate_impls: HashMap<String, Vec<&'a Impl>>,

    /// Map from type name to type ID for cross-crate impl lookup.
    type_name_to_id: HashMap<String, Id>,
}

impl<'a> SingleCrateView<'a> {
    /// Create a new single-crate view.
    fn new(
        crate_name: &str,
        krate: &'a Crate,
        registry: &'a UnifiedLinkRegistry,
        args: &'a Args,
        ctx: &'a MultiCrateContext<'a>,
    ) -> Self {
        let mut view = Self {
            crate_name: crate_name.to_owned(),
            krate,
            registry,
            args,
            ctx,
            path_map: HashMap::new(),
            impl_map: HashMap::new(),
            cross_crate_impls: HashMap::new(),
            type_name_to_id: HashMap::new(),
        };

        view.build_path_map();
        view.build_impl_map();
        view.build_type_name_map();

        // Get cross-crate impls from the context
        view.cross_crate_impls = ctx.find_cross_crate_impls(crate_name);

        view
    }

    /// Build the path map for all items.
    fn build_path_map(&mut self) {
        self.path_map.clear();

        let Some(root) = self.krate.index.get(&self.krate.root) else {
            return;
        };

        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                self.register_item_path(*item_id, vec![]);
            }
        }
    }

    /// Recursively register item paths.
    fn register_item_path(&mut self, id: Id, parent_path: Vec<String>) {
        let Some(item) = self.krate.index.get(&id) else {
            return;
        };

        let name = item.name.clone().unwrap_or_default();

        if let ItemEnum::Module(module) = &item.inner {
            let mut path = parent_path;
            path.push(name);

            self.path_map.insert(id, path.clone());

            for child_id in &module.items {
                self.register_item_path(*child_id, path.clone());
            }
        } else {
            self.path_map.insert(id, parent_path);
        }
    }

    /// Build the impl map for all types.
    fn build_impl_map(&mut self) {
        self.impl_map.clear();

        for item in self.krate.index.values() {
            if let ItemEnum::Impl(impl_block) = &item.inner
                && let Some(target_id) = Self::get_impl_target_id(impl_block)
            {
                self.impl_map.entry(target_id).or_default().push(impl_block);
            }
        }

        // Sort impl blocks for deterministic output
        for impls in self.impl_map.values_mut() {
            impls.sort_by_key(|i| Self::impl_sort_key(i));
        }
    }

    /// Build a map from type name to type ID.
    ///
    /// This is used to look up cross-crate impls by type name.
    fn build_type_name_map(&mut self) {
        self.type_name_to_id.clear();

        for (id, item) in &self.krate.index {
            if let Some(name) = &item.name {
                // Only include types that can have impls
                match &item.inner {
                    ItemEnum::Struct(_) | ItemEnum::Enum(_) | ItemEnum::Union(_) => {
                        self.type_name_to_id.insert(name.clone(), *id);
                    },
                    _ => {},
                }
            }
        }
    }

    /// Get the target type ID for an impl block.
    const fn get_impl_target_id(impl_block: &Impl) -> Option<Id> {
        use rustdoc_types::Type;

        match &impl_block.for_ {
            Type::ResolvedPath(path) => Some(path.id),
            _ => None,
        }
    }

    /// Generate a sort key for impl blocks.
    fn impl_sort_key(impl_block: &Impl) -> (u8, String) {
        // Extract trait name from the path (last segment)
        let trait_name: String = impl_block
            .trait_
            .as_ref()
            .and_then(|p| p.path.split("::").last())
            .unwrap_or("")
            .to_string();

        let priority = if impl_block.trait_.is_none() {
            0 // Inherent impls first
        } else if trait_name.starts_with("From") || trait_name.starts_with("Into") {
            1 // Conversion traits
        } else if trait_name.starts_with("De") || trait_name.starts_with("Se") {
            3 // Serde traits last
        } else {
            2 // Other traits
        };

        (priority, trait_name)
    }

    /// Get the crate name.
    #[must_use]
    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    /// Get the crate being rendered.
    #[must_use]
    pub const fn krate(&self) -> &Crate {
        self.krate
    }

    /// Get the unified registry.
    #[must_use]
    pub const fn registry(&self) -> &UnifiedLinkRegistry {
        self.registry
    }

    /// Get CLI arguments.
    #[must_use]
    pub const fn args(&self) -> &Args {
        self.args
    }

    /// Get module path for an item.
    #[must_use]
    pub fn get_path(&self, id: Id) -> Option<&Vec<String>> {
        self.path_map.get(&id)
    }

    /// Get impl blocks for a type (local crate only).
    #[must_use]
    pub fn get_impls(&self, id: Id) -> Option<&Vec<&'a Impl>> {
        self.impl_map.get(&id)
    }

    /// Get all impl blocks for a type, including cross-crate impls.
    ///
    /// This method merges local impls (from this crate) with impls from
    /// other crates that implement traits for this type.
    #[must_use]
    pub fn get_all_impls(&self, id: Id) -> Vec<&'a Impl> {
        let mut result = Vec::new();

        // Add local impls
        if let Some(local_impls) = self.impl_map.get(&id) {
            result.extend(local_impls.iter().copied());
        }

        // Add cross-crate impls by looking up the type name
        if let Some(item) = self.krate.index.get(&id)
            && let Some(type_name) = &item.name
            && let Some(external_impls) = self.cross_crate_impls.get(type_name)
        {
            result.extend(external_impls.iter().copied());
        }

        result
    }

    /// Check if an item should be included based on visibility.
    #[must_use]
    pub const fn should_include_item(&self, item: &rustdoc_types::Item) -> bool {
        if self.args.include_private {
            return true;
        }

        matches!(item.visibility, Visibility::Public)
    }

    /// Count modules for progress reporting.
    #[must_use]
    pub fn count_modules(&self) -> usize {
        self.krate
            .index
            .values()
            .filter(|item| matches!(&item.inner, ItemEnum::Module(_)))
            .count()
    }

    /// Create a markdown link using the unified registry.
    #[must_use]
    pub fn create_link(&self, to_crate: &str, to_id: Id, from_path: &str) -> Option<String> {
        self.registry
            .create_link(&self.crate_name, from_path, to_crate, to_id)
    }

    /// Resolve a name to a crate and ID.
    #[must_use]
    pub fn resolve_name(&self, name: &str) -> Option<(String, Id)> {
        self.registry.resolve_name(name, &self.crate_name)
    }

    /// Look up an item across all crates by ID.
    ///
    /// This is useful for resolving re-exports that point to items in
    /// external crates. First checks the local crate, then searches
    /// all other crates in the collection.
    ///
    /// # Returns
    ///
    /// A tuple of `(crate_name, item)` if found, or `None` if the item
    /// doesn't exist in any crate.
    #[must_use]
    pub fn lookup_item_across_crates(&self, id: &Id) -> Option<(&str, &Item)> {
        // First check local crate (fast path)
        if let Some(item) = self.krate.index.get(id) {
            return Some((&self.crate_name, item));
        }

        // Fall back to searching all crates
        self.ctx.find_item(id)
    }

    /// Resolve a path like `regex_automata::Regex` to an item.
    ///
    /// This is used for external re-exports where `use_item.id` is `None`
    /// but the source path is available.
    ///
    /// # Returns
    ///
    /// A tuple of `(source_crate, item, item_id)` if found.
    #[must_use]
    pub fn resolve_external_path(&self, path: &str) -> Option<(&str, &Item, Id)> {
        let (source_crate, id) = self.registry.resolve_path(path)?;
        let (crate_name, item) = self.ctx.find_item(&id)?;

        // Verify the crate matches
        if crate_name == source_crate {
            Some((crate_name, item, id))
        } else {
            None
        }
    }

    /// Process backtick links like `[`Span`]` to markdown links.
    fn process_backtick_links(
        &self,
        docs: &str,
        item_links: &HashMap<String, Id>,
        current_file: &str,
    ) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in BACKTICK_LINK_RE.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Check if followed by ( or [ (already a link)
            let next_char = docs[match_end..].chars().next();
            if matches!(next_char, Some('(' | '[')) {
                continue;
            }

            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            let link_text = &caps[1];

            // Try to resolve the link
            if let Some(resolved) = self.resolve_link(link_text, item_links, current_file) {
                result.push_str(&resolved);
            } else {
                // Couldn't resolve - convert to plain inline code
                result.push_str(&format!("`{link_text}`"));
            }
        }

        result.push_str(&docs[last_end..]);
        result
    }

    /// Process plain links like `[enter]` to markdown links.
    ///
    /// Skips matches that are:
    /// - Inside inline code (backticks)
    /// - Already markdown links (followed by `(` or `[`)
    fn process_plain_links(docs: &str) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;

        for caps in PLAIN_LINK_RE.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Check if followed by ( or [ (already a link)
            let next_char = docs[match_end..].chars().next();
            if matches!(next_char, Some('(' | '[')) {
                continue;
            }

            // Check if inside inline code (count backticks before match)
            let before = &docs[..match_start];
            let backtick_count = before.chars().filter(|&c| c == '`').count();
            if backtick_count % 2 == 1 {
                // Odd number of backticks means we're inside inline code
                continue;
            }

            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            let link_text = &caps[1];

            // Plain links usually refer to items on the same page
            let anchor = slugify_anchor(link_text);
            _ = write!(result, "[{link_text}](#{anchor})");
        }

        result.push_str(&docs[last_end..]);
        result
    }

    /// Resolve a link text to a markdown link using the registry.
    #[expect(clippy::unnecessary_wraps, reason = "wrong clippy?")]
    fn resolve_link(
        &self,
        link_text: &str,
        item_links: &HashMap<String, Id>,
        current_file: &str,
    ) -> Option<String> {
        // First, try the item's links map (most accurate for current crate)
        if let Some(id) = item_links.get(link_text) {
            // Only look up in current crate - IDs are crate-local
            if let Some(path) = self.registry.get_path(&self.crate_name, *id) {
                let display_name = self
                    .registry
                    .get_name(&self.crate_name, *id)
                    .map_or(link_text, String::as_str);

                // Compute relative path from current file
                let relative = Self::compute_relative_link(current_file, path, display_name);
                return Some(relative);
            }
            // ID not in current crate - fall through to name-based resolution
        }

        // Try resolving by name in the registry (prefer current crate)
        if let Some((resolved_crate, id)) = self.registry.resolve_name(link_text, &self.crate_name)
            && let Some(path) = self.registry.get_path(&resolved_crate, id)
        {
            if resolved_crate == self.crate_name {
                // Same crate - compute relative path
                let relative = Self::compute_relative_link(current_file, path, link_text);
                return Some(relative);
            }
            // Cross-crate link - only use if this looks like an intentional external reference
            // Skip if this is a simple name that's probably meant to be a local anchor
            if !Self::looks_like_external_reference(link_text) {
                // Fall through to anchor fallback
            } else {
                // Compute relative path to other crate
                // current_file includes crate prefix, so subtract 1 from depth
                // e.g., "regex_syntax/hir/index.md" has depth 2, but relative to crate root is 1
                let current_without_crate = current_file
                    .find('/')
                    .map_or(current_file, |idx| &current_file[idx + 1..]);
                let from_depth = current_without_crate.matches('/').count();
                let prefix = "../".repeat(from_depth + 1); // +1 to go up to docs root
                let full_path = format!("{prefix}{resolved_crate}/{path}");

                return Some(format!("[`{link_text}`]({full_path})"));
            }
        }

        // Couldn't resolve - check if this looks like a method/path reference
        if link_text.contains("::") {
            // For Type::method or path::Item that we can't resolve,
            // just return as inline code without a broken anchor
            return None;
        }

        // Try as an anchor on the current page (only for simple names)
        let anchor = slugify_anchor(link_text);

        Some(format!("[`{link_text}`](#{anchor})"))
    }

    /// Compute a relative link from current file to target path.
    ///
    /// Note: `current_file` includes the crate prefix (e.g., `regex_syntax/hir/index.md`)
    /// but `target_path` from the registry does NOT (e.g., `hir/index.md`).
    /// We need to strip the crate prefix from `current_file` for comparison.
    fn compute_relative_link(current_file: &str, target_path: &str, display_name: &str) -> String {
        use crate::linker::LinkRegistry;

        // Strip crate prefix from current_file (everything before first /)
        let current_without_crate = current_file
            .find('/')
            .map_or(current_file, |idx| &current_file[idx + 1..]);

        // Same file - use anchor
        if current_without_crate == target_path {
            let anchor = slugify_anchor(display_name);
            return format!("[`{display_name}`](#{anchor})");
        }

        // Different file - compute relative path
        let relative = LinkRegistry::compute_relative_path(current_without_crate, target_path);
        format!("[`{display_name}`]({relative})")
    }

    /// Check if a link text looks like an intentional external crate reference.
    ///
    /// Simple names like "Wide", "Error", "Default" are often meant to be
    /// local anchors or type aliases, not cross-crate links.
    fn looks_like_external_reference(link_text: &str) -> bool {
        // Contains :: - explicit path reference
        if link_text.contains("::") {
            return true;
        }

        // Known external crate names or patterns
        let external_patterns = [
            "std::", "core::", "alloc::",
            "_crate", "_derive", "_impl",
        ];

        for pattern in external_patterns {
            if link_text.contains(pattern) {
                return true;
            }
        }

        // Single PascalCase words are usually local items, not external
        // (External items would be referenced with full paths)
        false
    }
}

impl RenderContext for SingleCrateView<'_> {
    fn krate(&self) -> &Crate {
        self.krate
    }

    fn crate_name(&self) -> &str {
        &self.crate_name
    }

    fn get_item(&self, id: &Id) -> Option<&Item> {
        self.krate.index.get(id)
    }

    fn get_impls(&self, id: &Id) -> Option<&[&Impl]> {
        self.impl_map.get(id).map(Vec::as_slice)
    }

    fn should_include_item(&self, item: &Item) -> bool {
        match &item.visibility {
            Visibility::Public => true,
            _ => self.args.include_private,
        }
    }

    fn include_private(&self) -> bool {
        self.args.include_private
    }

    fn crate_version(&self) -> Option<&str> {
        self.krate.crate_version.as_deref()
    }

    fn link_registry(&self) -> Option<&LinkRegistry> {
        // Multi-crate mode uses UnifiedLinkRegistry instead
        None
    }

    fn process_docs(&self, item: &Item, current_file: &str) -> Option<String> {
        let docs = item.docs.as_ref()?;
        let name = item.name.as_deref().unwrap_or("");

        // Strip duplicate title if docs start with "# name"
        let docs = strip_duplicate_title(docs, name);

        // Strip reference definitions first to prevent mangled output
        let stripped = strip_reference_definitions(docs);

        // Unhide rustdoc hidden lines and add `rust` to bare code fences
        let unhidden = unhide_code_lines(&stripped);

        // Convert HTML and path reference links
        let html_processed = convert_html_links(&unhidden);
        let path_processed = convert_path_reference_links(&html_processed);

        // Process backtick links [`Name`]
        let backtick_processed =
            self.process_backtick_links(&path_processed, &item.links, current_file);

        // Process plain links [name]
        let plain_processed = Self::process_plain_links(&backtick_processed);

        Some(plain_processed)
    }

    fn create_link(&self, id: Id, current_file: &str) -> Option<String> {
        // Look up path in the unified registry
        let path = self.registry.get_path(&self.crate_name, id)?;

        // Get the item name for display
        let display_name = self
            .registry
            .get_name(&self.crate_name, id)
            .cloned()
            .unwrap_or_else(|| "item".to_string());

        // Compute relative path from current file
        let from_depth = current_file.matches('/').count();

        let relative_path = if from_depth == 0 {
            path.clone()
        } else {
            // Go up to crate root, then down to target
            let prefix = "../".repeat(from_depth);

            format!("{prefix}{path}")
        };

        Some(format!("[`{display_name}`]({relative_path})"))
    }
}
