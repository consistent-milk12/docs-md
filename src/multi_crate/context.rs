//! Multi-crate generation context.
//!
//! This module provides [`MultiCrateContext`] which holds shared state
//! during multi-crate documentation generation, and [`SingleCrateView`]
//! which provides a single-crate interface for existing rendering code.

use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use std::sync::LazyLock;

use regex::Regex;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Visibility};
use tracing::{debug, instrument, trace};

use crate::Args;
use crate::generator::config::RenderConfig;
use crate::generator::doc_links::{
    convert_html_links, convert_path_reference_links, strip_duplicate_title,
    strip_reference_definitions, unhide_code_lines,
};
use crate::generator::render_shared::SourcePathConfig;
use crate::generator::{ItemAccess, ItemFilter, LinkResolver};
use crate::linker::{LinkRegistry, item_has_anchor, slugify_anchor};
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

    /// Rendering configuration options.
    config: RenderConfig,

    /// Pre-computed cross-crate impl blocks.
    ///
    /// Maps target crate name -> type name -> impl blocks from other crates.
    /// This is computed once during construction rather than per-view.
    cross_crate_impls: HashMap<String, HashMap<String, Vec<&'a Impl>>>,

    /// Base source path configuration for transforming cargo registry paths.
    ///
    /// `None` if source locations are disabled or no `.source_*` dir detected.
    source_path_config: Option<SourcePathConfig>,
}

impl<'a> MultiCrateContext<'a> {
    /// Create a new multi-crate context.
    ///
    /// Builds the unified link registry and pre-computes cross-crate impls.
    ///
    /// # Arguments
    ///
    /// * `crates` - Collection of parsed crates
    /// * `args` - CLI arguments
    /// * `config` - Rendering configuration options
    #[must_use]
    #[instrument(skip(crates, args, config), fields(crate_count = crates.names().len()))]
    pub fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self {
        debug!("Creating multi-crate context");

        let primary = args.primary_crate.as_deref();
        let registry = UnifiedLinkRegistry::build(crates, primary);

        // Pre-compute cross-crate impls for all crates
        debug!("Building cross-crate impl map");
        let cross_crate_impls = Self::build_cross_crate_impls(crates);

        // Build source path config if source_locations is enabled and we have a source_dir
        let source_path_config = if config.include_source.source_locations {
            config
                .include_source
                .source_dir
                .as_ref()
                .map(|dir| SourcePathConfig::new(dir, ""))
        } else {
            None
        };

        debug!(
            cross_crate_impl_count = cross_crate_impls.values().map(HashMap::len).sum::<usize>(),
            "Multi-crate context created"
        );

        Self {
            crates,
            registry,
            args,
            config,
            cross_crate_impls,
            source_path_config,
        }
    }

    /// Set the source directory for path transformation.
    ///
    /// This can be called after construction if a `.source_*` directory
    /// is detected or specified via CLI. Only has effect if `source_locations`
    /// is enabled in the config.
    pub fn set_source_dir(&mut self, source_dir: &Path) {
        if self.config.include_source.source_locations {
            self.source_path_config = Some(SourcePathConfig::new(source_dir, ""));
        }
    }

    /// Get source path config for a specific file.
    ///
    /// Returns `None` if source locations are disabled or no source dir configured.
    #[must_use]
    pub fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig> {
        self.source_path_config
            .as_ref()
            .map(|base| base.with_depth(current_file))
    }

    /// Build the cross-crate impl map for all crates.
    ///
    /// Scans all crates once and groups impl blocks by their target crate
    /// and type name. This avoids O(n*m) scanning per view creation.
    fn build_cross_crate_impls(
        crates: &'a CrateCollection,
    ) -> HashMap<String, HashMap<String, Vec<&'a Impl>>> {
        let mut result: HashMap<String, HashMap<String, Vec<&'a Impl>>> = HashMap::new();

        // Initialize empty maps for all crates
        for crate_name in crates.names() {
            result.insert(crate_name.clone(), HashMap::new());
        }

        // Scan all crates for impl blocks
        for (source_crate, krate) in crates.iter() {
            for item in krate.index.values() {
                if let ItemEnum::Impl(impl_block) = &item.inner {
                    // Skip synthetic impls
                    if impl_block.is_synthetic {
                        continue;
                    }

                    // Get the target type path
                    if let Some(type_path) = Self::get_impl_target_path(impl_block) {
                        // Extract the target crate name (first segment)
                        if let Some(target_crate) = type_path.split("::").next() {
                            // Skip if targeting same crate (not cross-crate)
                            if target_crate == source_crate {
                                continue;
                            }

                            // Only add if target crate is in our collection
                            if let Some(type_map) = result.get_mut(target_crate) {
                                // Extract the type name (last segment)
                                let type_name = type_path
                                    .split("::")
                                    .last()
                                    .unwrap_or(&type_path)
                                    .to_string();

                                type_map.entry(type_name).or_default().push(impl_block);
                            }
                        }
                    }
                }
            }
        }

        result
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
        // Use get_with_name to get the crate name with the collection's lifetime
        let (name, krate) = self.crates.get_with_name(crate_name)?;

        Some(SingleCrateView::new(
            name,
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

    /// Get pre-computed cross-crate impl blocks for a target crate.
    ///
    /// Returns a map from type name to impl blocks from other crates.
    /// This data is pre-computed during context construction for efficiency.
    ///
    /// # Returns
    ///
    /// Reference to the type-name -> impl-blocks map, or `None` if the
    /// crate is not in the collection.
    #[must_use]
    pub fn get_cross_crate_impls(
        &self,
        target_crate: &str,
    ) -> Option<&HashMap<String, Vec<&'a Impl>>> {
        self.cross_crate_impls.get(target_crate)
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
    /// Name of this crate (borrowed from the context).
    crate_name: &'a str,

    /// The crate being rendered.
    krate: &'a Crate,

    /// Unified registry for link resolution.
    registry: &'a UnifiedLinkRegistry,

    /// CLI arguments.
    args: &'a Args,

    /// Reference to the parent multi-crate context for cross-crate lookups.
    ctx: &'a MultiCrateContext<'a>,

    /// Map from type ID to impl blocks (local crate only).
    impl_map: HashMap<Id, Vec<&'a Impl>>,

    /// Reference to pre-computed cross-crate impl blocks from context.
    /// Maps type name to impl blocks from other crates.
    cross_crate_impls: Option<&'a HashMap<String, Vec<&'a Impl>>>,

    /// Map from type name to type ID for cross-crate impl lookup.
    type_name_to_id: HashMap<String, Id>,
}

impl<'a> SingleCrateView<'a> {
    /// Create a new single-crate view.
    fn new(
        crate_name: &'a str,
        krate: &'a Crate,
        registry: &'a UnifiedLinkRegistry,
        args: &'a Args,
        ctx: &'a MultiCrateContext<'a>,
    ) -> Self {
        // Get reference to pre-computed cross-crate impls
        let cross_crate_impls = ctx.get_cross_crate_impls(crate_name);

        let mut view = Self {
            crate_name,
            krate,
            registry,
            args,
            ctx,
            impl_map: HashMap::new(),
            cross_crate_impls,
            type_name_to_id: HashMap::new(),
        };

        view.build_impl_map();
        view.build_type_name_map();

        view
    }

    /// Build the impl map for all types.
    ///
    /// Uses the `impls` field on Struct/Enum/Union items directly rather than
    /// scanning all items and checking the `for_` field. This provides clearer
    /// semantics and leverages `rustdoc_types` structured data.
    fn build_impl_map(&mut self) {
        self.impl_map.clear();

        // Iterate over all types that can have impl blocks and collect their impls
        for (type_id, item) in &self.krate.index {
            let impl_ids: &[Id] = match &item.inner {
                ItemEnum::Struct(s) => &s.impls,
                ItemEnum::Enum(e) => &e.impls,
                ItemEnum::Union(u) => &u.impls,
                _ => continue,
            };

            // Look up each impl block and add to the map
            for impl_id in impl_ids {
                if let Some(impl_item) = self.krate.index.get(impl_id)
                    && let ItemEnum::Impl(impl_block) = &impl_item.inner
                {
                    self.impl_map.entry(*type_id).or_default().push(impl_block);
                }
            }
        }

        // Sort impl blocks for deterministic output
        for impls in self.impl_map.values_mut() {
            impls.sort_by_key(|i| Self::impl_sort_key(i));
            // Deduplicate impls with the same sort key
            impls.dedup_by(|a, b| Self::impl_sort_key(a) == Self::impl_sort_key(b));
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

    /// Generate a sort key for impl blocks.
    fn impl_sort_key(impl_block: &Impl) -> (u8, String) {
        // Extract trait name from the path (last segment)
        // Using rsplit().next() is more efficient than split().last()
        let trait_name: String = impl_block
            .trait_
            .as_ref()
            .and_then(|p| p.path.rsplit("::").next())
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
    pub const fn crate_name(&self) -> &str {
        self.crate_name
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
            && let Some(cross_crate_map) = self.cross_crate_impls
            && let Some(external_impls) = cross_crate_map.get(type_name)
        {
            result.extend(external_impls.iter().copied());
        }

        result
    }

    /// Get impl blocks for a type from a specific crate.
    ///
    /// This is used for cross-crate re-exports where we need to look up
    /// impl blocks from the source crate rather than the current crate.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the type to get impls for
    /// * `source_krate` - The crate to look up impls from
    ///
    /// # Returns
    ///
    /// A vector of impl blocks found in the source crate for the given type ID.
    #[must_use]
    pub fn get_impls_from_crate(&self, id: Id, source_krate: &'a Crate) -> Vec<&'a Impl> {
        let mut result = Vec::new();

        // Scan the source crate for impl blocks targeting this ID
        for item in source_krate.index.values() {
            if let ItemEnum::Impl(impl_block) = &item.inner {
                // Check if this impl targets our type using existing helper
                if let Some(target_id) = Self::get_impl_target_id_from_type(&impl_block.for_)
                    && target_id == id
                {
                    result.push(impl_block);
                }
            }
        }

        // Also include cross-crate impls if this is our current crate
        if std::ptr::eq(source_krate, self.krate)
            && let Some(item) = self.krate.index.get(&id)
            && let Some(type_name) = &item.name
            && let Some(cross_crate_map) = self.cross_crate_impls
            && let Some(external_impls) = cross_crate_map.get(type_name)
        {
            result.extend(external_impls.iter().copied());
        }

        result
    }

    /// Extract the target ID from a Type (for impl block matching).
    const fn get_impl_target_id_from_type(ty: &rustdoc_types::Type) -> Option<Id> {
        use rustdoc_types::Type;

        match ty {
            Type::ResolvedPath(path) => Some(path.id),
            _ => None,
        }
    }

    /// Check if an item should be included based on visibility.
    #[must_use]
    pub const fn should_include_item(&self, item: &rustdoc_types::Item) -> bool {
        if self.args.exclude_private {
            return matches!(item.visibility, Visibility::Public);
        }

        true
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
            .create_link(self.crate_name, from_path, to_crate, to_id)
    }

    /// Resolve a name to a crate and ID.
    #[must_use]
    pub fn resolve_name(&self, name: &str) -> Option<(String, Id)> {
        self.registry
            .resolve_name(name, self.crate_name)
            .map(|(s, id)| (s.to_string(), id))
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
    #[instrument(skip(self), fields(crate_name = %self.crate_name), level = "trace")]
    pub fn lookup_item_across_crates(&self, id: &Id) -> Option<(&str, &Item)> {
        // First check local crate (fast path)
        if let Some(item) = self.krate.index.get(id) {
            trace!(found_in = "local", "Item found in local crate");
            return Some((self.crate_name, item));
        }

        // Fall back to searching all crates
        trace!("Item not in local crate, searching all crates");
        let result = self.ctx.find_item(id);

        if let Some((crate_name, _)) = &result {
            debug!(found_in = %crate_name, "Item found in external crate");
        } else {
            trace!("Item not found in any crate");
        }

        result
    }

    /// Get a crate by name from the collection.
    ///
    /// This is useful for getting the source crate context when rendering
    /// re-exported items from other crates.
    ///
    /// # Returns
    ///
    /// The crate if found, or `None` if no crate with that name exists.
    #[must_use]
    pub fn get_crate(&self, name: &str) -> Option<&Crate> {
        self.ctx.crates.get(name)
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
    #[tracing::instrument(skip(self, docs, item_links), level = "trace", fields(file = %current_file))]
    fn process_backtick_links(
        &self,
        docs: &str,
        item_links: &HashMap<String, Id>,
        current_file: &str,
    ) -> String {
        let mut result = String::with_capacity(docs.len());
        let mut last_end = 0;
        let mut resolved_count = 0;
        let mut unresolved_count = 0;

        for caps in BACKTICK_LINK_RE.captures_iter(docs) {
            let full_match = caps.get(0).unwrap();
            let match_start = full_match.start();
            let match_end = full_match.end();

            // Check if followed by ( or [ (already a link)
            let next_char = docs[match_end..].chars().next();
            if matches!(next_char, Some('(' | '[')) {
                tracing::trace!(
                    link = %full_match.as_str(),
                    "Skipping - already has link target"
                );
                continue;
            }

            result.push_str(&docs[last_end..match_start]);
            last_end = match_end;

            let link_text = &caps[1];

            // The item_links keys may have backticks (e.g., "`Visit`") or not ("Visit")
            // Try the backtick-wrapped version first since that's what rustdoc typically uses
            let backtick_key = format!("`{link_text}`");

            // Try to resolve the link (try backtick version first, then plain)
            if let Some(resolved) = self
                .resolve_link(&backtick_key, item_links, current_file)
                .or_else(|| self.resolve_link(link_text, item_links, current_file))
            {
                tracing::trace!(
                    link_text = %link_text,
                    resolved = %resolved,
                    "Resolved backtick link"
                );
                resolved_count += 1;
                result.push_str(&resolved);
            } else {
                tracing::trace!(
                    link_text = %link_text,
                    "Could not resolve backtick link, keeping as inline code"
                );
                unresolved_count += 1;
                // Couldn't resolve - convert to plain inline code
                _ = write!(result, "`{link_text}`");
            }
        }

        result.push_str(&docs[last_end..]);

        if resolved_count > 0 || unresolved_count > 0 {
            tracing::trace!(
                resolved = resolved_count,
                unresolved = unresolved_count,
                "Finished processing backtick links"
            );
        }

        result
    }

    /// Process plain links like `[enter]` to markdown links.
    ///
    /// Uses the registry to resolve links to proper paths. If the item exists
    /// in the registry, creates a link to its location. If on the current page
    /// and has a heading anchor, uses an anchor link.
    ///
    /// Skips matches that are:
    /// - Inside inline code (backticks)
    /// - Already markdown links (followed by `(` or `[`)
    fn process_plain_links(&self, docs: &str, current_file: &str) -> String {
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

            // Try to resolve via registry
            if let Some(link) = self.resolve_plain_link(link_text, current_file) {
                result.push_str(&link);
            } else {
                // Unresolved - keep as plain text
                _ = write!(result, "[{link_text}]");
            }
        }

        result.push_str(&docs[last_end..]);
        result
    }

    /// Resolve a plain link `[name]` to a markdown link.
    ///
    /// Returns `Some(markdown_link)` if the item can be resolved,
    /// `None` if it should remain as plain text.
    #[expect(clippy::similar_names)]
    #[tracing::instrument(skip(self), level = "trace")]
    fn resolve_plain_link(&self, link_text: &str, current_file: &str) -> Option<String> {
        // Try to find the item in the registry
        let (resolved_crate, id) = self.registry.resolve_name(link_text, self.crate_name)?;

        tracing::trace!(
            resolved_crate = %resolved_crate,
            id = ?id,
            "Found item in registry"
        );

        // Check if this is an external re-export and try to follow it
        let (target_crate, target_id) = self
            .registry
            .resolve_reexport(&resolved_crate, id)
            .unwrap_or_else(|| (resolved_crate.clone(), id));

        let followed_reexport = target_crate != resolved_crate || target_id != id;
        if followed_reexport {
            tracing::trace!(
                original_crate = %resolved_crate,
                original_id = ?id,
                target_crate = %target_crate,
                target_id = ?target_id,
                "Followed re-export chain to original item"
            );
        }

        // Get the crate data for the target (might be different from current crate)
        let target_krate = self.ctx.crates.get(&target_crate)?;

        // Get the item's path info from the target crate
        let path_info = target_krate.paths.get(&target_id)?;

        // Get the file path for this item
        let target_path = self.registry.get_path(&target_crate, target_id)?;

        // Strip crate prefix from current_file for comparison
        let current_local = Self::strip_crate_prefix(current_file);

        // Check if same file (accounting for cross-crate)
        let is_same_file = target_crate == self.crate_name && target_path == current_local;

        if is_same_file {
            // Item is on the current page
            if item_has_anchor(path_info.kind) {
                // Has a heading - create anchor link
                let anchor = slugify_anchor(link_text);
                tracing::trace!(
                    anchor = %anchor,
                    kind = ?path_info.kind,
                    "Creating same-page anchor link"
                );
                Some(format!("[{link_text}](#{anchor})"))
            } else {
                // No heading - link to page without anchor
                tracing::trace!(
                    kind = ?path_info.kind,
                    "Item on same page but no heading - linking to page"
                );
                Some(format!("[{link_text}]()"))
            }
        } else {
            // Item is in a different file (possibly different crate)
            tracing::trace!(
                target_crate = %target_crate,
                target_path = %target_path,
                "Creating cross-file link"
            );
            let relative =
                self.build_markdown_link(current_file, &target_crate, target_path, link_text, None);
            Some(relative)
        }
    }

    /// Resolve a link text to a markdown link using the registry.
    ///
    /// This function attempts to convert rustdoc link syntax into valid markdown
    /// links that work in the generated documentation.
    ///
    /// # Arguments
    /// * `link_text` - The raw link target from rustdoc (e.g., "`crate::config::ConfigBuilder::method`")
    /// * `item_links` - Map of link texts to Item IDs from rustdoc's `links` field
    /// * `current_file` - The markdown file being generated (e.g., "ureq/index.md")
    ///
    /// # Returns
    /// * `Some(markdown_link)` - A formatted markdown link like `[`text`](path.md#anchor)`
    /// * `None` - If the link cannot be resolved (will be rendered as inline code)
    ///
    /// # Examples
    ///
    /// ```text
    /// Input:  link_text = "crate::config::ConfigBuilder::http_status_as_error"
    ///         current_file = "ureq/index.md"
    /// Output: Some("[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)")
    ///
    /// Input:  link_text = "ConfigBuilder"
    ///         current_file = "ureq/agent/index.md"
    /// Output: Some("[`ConfigBuilder`](../config/index.md#configbuilder)")
    ///
    /// Input:  link_text = "std::io::Error"  (external crate, not in registry)
    ///         current_file = "ureq/index.md"
    /// Output: None  (rendered as `std::io::Error` inline code)
    /// ```
    #[instrument(skip(self, item_links), fields(crate_name = %self.crate_name))]
    fn resolve_link(
        &self,
        link_text: &str,
        item_links: &HashMap<String, Id>,
        current_file: &str,
    ) -> Option<String> {
        // ─────────────────────────────────────────────────────────────────────
        // Strategy 1: Try the item's links map (most accurate)
        // ─────────────────────────────────────────────────────────────────────
        // Rustdoc provides a `links` map on each item that maps link text to
        // the resolved Item ID. This is the most reliable source because rustdoc
        // has already done the name resolution.
        //
        // Example item_links map:
        //   {
        //     "ConfigBuilder" => Id(123),
        //     "crate::config::ConfigBuilder" => Id(123),
        //     "Agent" => Id(456)
        //   }
        tracing::trace!(
            strategy = "item_links",
            "Attempting resolution via item links map"
        );
        if let Some(id) = item_links.get(link_text) {
            // We have an ID! Now convert it to a markdown path.
            // Example: Id(123) → "config/index.md" → "[`ConfigBuilder`](config/index.md)"
            tracing::debug!(strategy = "item_links", ?id, "Found ID in item links");

            // Strip backticks from display name if present (rustdoc uses `Name` as keys)
            let display_name = link_text.trim_matches('`');
            if let Some(link) = self.build_link_to_id(*id, current_file, display_name, None) {
                tracing::debug!(strategy = "item_links", link = %link, "Successfully resolved");

                return Some(link);
            }

            tracing::trace!(strategy = "item_links", "ID Found but couldn't build link");
        }

        // ─────────────────────────────────────────────────────────────────────
        // Strategy 2: Try resolving by name in the registry
        // ─────────────────────────────────────────────────────────────────────
        // If the item_links map didn't have this link (can happen with re-exports
        // or manually written links), try looking up the name directly in our
        // cross-crate registry.
        //
        // Example:
        //   link_text = "Agent"
        //   registry.resolve_name("Agent", "ureq") → Some(("ureq", Id(456)))
        tracing::trace!(
            strategy = "registry_name",
            "Attempting resolution via registry name lookup"
        );
        if let Some((resolved_crate, id)) = self.registry.resolve_name(link_text, self.crate_name) {
            // Only use this if:
            // 1. Same crate (internal link), OR
            // 2. Explicitly looks like an external reference (contains "::")
            //
            // This prevents accidental cross-crate linking for common names like "Error"
            if resolved_crate == self.crate_name || Self::looks_like_external_reference(link_text) {
                // Use build_link_to_id to follow re-exports to the original definition
                if let Some(link) = self.build_link_to_id(id, current_file, link_text, None) {
                    return Some(link);
                }
            }
        }

        // ─────────────────────────────────────────────────────────────────────
        // Strategy 3: Try crate:: prefixed paths
        // ─────────────────────────────────────────────────────────────────────
        // Handle explicit crate-relative paths like "crate::config::ConfigBuilder::method"
        // These are common in rustdoc comments and need special parsing.
        //
        // Example:
        //   link_text = "crate::config::ConfigBuilder::http_status_as_error"
        //   → strip prefix → "config::ConfigBuilder::http_status_as_error"
        //   → resolve_crate_path() handles the rest
        if let Some(path_without_crate) = link_text.strip_prefix("crate::")
            && let Some(link) = self.resolve_crate_path(path_without_crate, link_text, current_file)
        {
            return Some(link);
        }

        // ─────────────────────────────────────────────────────────────────────
        // Give up on qualified paths we can't resolve
        // ─────────────────────────────────────────────────────────────────────
        // If it has "::" and we still haven't resolved it, it's probably an
        // external crate we don't have (like std, serde, tokio, etc.)
        // Return None so it renders as inline code: `std::io::Error`
        if link_text.contains("::") {
            return None;
        }

        // ─────────────────────────────────────────────────────────────────────
        // Fallback: anchor on current page (only if item has a heading)
        // ─────────────────────────────────────────────────────────────────────
        // For simple names without ::, check if the item exists and has a heading.
        // Only structs, enums, traits, functions, etc. get headings.
        // Methods, fields, and variants don't have headings (they're bullet points).
        if let Some((_, id)) = self.registry.resolve_name(link_text, self.crate_name)
            && let Some(path_info) = self.krate.paths.get(&id)
        {
            if item_has_anchor(path_info.kind) {
                return Some(format!("[`{link_text}`](#{})", slugify_anchor(link_text)));
            }

            // Item exists but no anchor - link to page without anchor
            return Some(format!("[`{link_text}`]()"));
        }
        // Unknown item - return None (renders as inline code)
        None
    }

    /// Build a link to an item by ID.
    ///
    /// This is the simplest path when we already have a resolved Item ID from
    /// rustdoc's links map. We just need to look up the file path in our registry.
    ///
    /// # Arguments
    /// * `id` - The rustdoc Item ID to link to
    /// * `current_file` - Source file for relative path computation
    /// * `display_name` - Text to show in the link
    /// * `anchor` - Optional anchor (e.g., method name)
    ///
    /// # Example Transformation
    ///
    /// ```text
    /// Input:
    ///   id = Id(123)  (rustdoc's internal ID for ConfigBuilder)
    ///   current_file = "ureq/agent/index.md"
    ///   display_name = "ConfigBuilder"
    ///   anchor = None
    ///
    /// Step 1: Look up ID in registry
    ///   registry.get_path("ureq", Id(123)) → Some("config/index.md")
    ///
    /// Step 2: Build markdown link
    ///   build_markdown_link("ureq/agent/index.md", "ureq", "config/index.md", "ConfigBuilder", None)
    ///   → "[`ConfigBuilder`](../config/index.md)"
    ///
    /// Output: Some("[`ConfigBuilder`](../config/index.md)")
    /// ```
    #[expect(clippy::too_many_lines)]
    #[tracing::instrument(skip(self), level = "trace")]
    fn build_link_to_id(
        &self,
        id: Id,
        current_file: &str,
        display_name: &str,
        anchor: Option<&str>,
    ) -> Option<String> {
        // First: Check if this is a re-export and follow to the original definition
        // Re-exports don't have headings - we need to link to where the item is defined
        //
        // Method 1: Check our re_export_sources registry
        if let Some((original_crate, original_id)) =
            self.registry.resolve_reexport(self.crate_name, id)
        {
            tracing::trace!(
                original_crate = %original_crate,
                original_id = ?original_id,
                "Following re-export via registry to original definition"
            );

            if let Some(target_path) = self.registry.get_path(&original_crate, original_id) {
                return Some(self.build_markdown_link(
                    current_file,
                    &original_crate,
                    target_path,
                    display_name,
                    anchor,
                ));
            }
        }

        // Method 2: Check if the item itself is a Use item in the index
        if let Some(item) = self.krate.index.get(&id)
            && let ItemEnum::Use(use_item) = &item.inner
        {
            tracing::trace!(
                source = %use_item.source,
                target_id = ?use_item.id,
                "Found Use item in index"
            );

            // Method 2a: If the Use item has a target ID, look up via paths
            // This handles cases where source is relative (e.g., "self::event::Event")
            // but the ID points to the actual item in another crate
            if let Some(target_id) = use_item.id
                && let Some(path_info) = self.krate.paths.get(&target_id)
                && let Some(external_crate) = path_info.path.first()
            {
                tracing::trace!(
                    external_crate = %external_crate,
                    path = ?path_info.path,
                    "Following Use item target ID to external crate"
                );

                // Try to find the item in the external crate by name
                let item_name = path_info.path.last().unwrap_or(&path_info.path[0]);

                if let Some((resolved_crate, resolved_id)) =
                    self.registry.resolve_name(item_name, external_crate)
                    && let Some(target_path) = self.registry.get_path(&resolved_crate, resolved_id)
                {
                    return Some(self.build_markdown_link(
                        current_file,
                        &resolved_crate,
                        target_path,
                        display_name,
                        anchor,
                    ));
                }
            }

            // Method 2b: Try to resolve the source path directly
            if !use_item.source.is_empty()
                && let Some((original_crate, original_id)) =
                    self.registry.resolve_path(&use_item.source)
                && let Some(target_path) = self.registry.get_path(&original_crate, original_id)
            {
                return Some(self.build_markdown_link(
                    current_file,
                    &original_crate,
                    target_path,
                    display_name,
                    anchor,
                ));
            }
        }

        // Strategy 1: Try to find the ID in the current crate
        if let Some(target_path) = self.registry.get_path(self.crate_name, id) {
            tracing::trace!(
                strategy = "current_crate",
                crate_name = %self.crate_name,
                target_path = %target_path,
                "Found ID in current crate registry"
            );
            return Some(self.build_markdown_link(
                current_file,
                self.crate_name,
                target_path,
                display_name,
                anchor,
            ));
        }

        tracing::trace!(
            strategy = "current_crate",
            crate_name = %self.crate_name,
            "ID not found in current crate, checking paths for external reference"
        );

        // Strategy 2: ID not in current crate - check if it's an external item via paths
        // The paths map can contain IDs from other crates (for re-exports/cross-refs)
        if let Some(path_info) = self.krate.paths.get(&id) {
            // path_info.path is like ["tracing_core", "field", "Visit"]
            // First element is the crate name
            let path_str = path_info.path.join("::");
            tracing::trace!(
                strategy = "external_paths",
                path = %path_str,
                kind = ?path_info.kind,
                "Found path info for external item"
            );

            if let Some(external_crate) = path_info.path.first() {
                // Strategy 2a: Try direct ID lookup in external crate
                if let Some(target_path) = self.registry.get_path(external_crate, id) {
                    tracing::trace!(
                        strategy = "external_direct_id",
                        external_crate = %external_crate,
                        target_path = %target_path,
                        "Found external item by direct ID lookup"
                    );

                    return Some(self.build_markdown_link(
                        current_file,
                        external_crate,
                        target_path,
                        display_name,
                        anchor,
                    ));
                }

                // Strategy 2b: External crate uses different ID - try name-based lookup
                // This handles cross-crate references where IDs are crate-local
                let item_name = path_info.path.last()?;
                tracing::trace!(
                    strategy = "external_name_lookup",
                    external_crate = %external_crate,
                    item_name = %item_name,
                    "Attempting name-based lookup in external crate"
                );

                if let Some((resolved_crate, resolved_id)) =
                    self.registry.resolve_name(item_name, external_crate)
                {
                    tracing::trace!(
                        strategy = "external_name_lookup",
                        resolved_crate = %resolved_crate,
                        resolved_id = ?resolved_id,
                        "Name resolved to crate and ID"
                    );

                    if let Some(target_path) = self.registry.get_path(&resolved_crate, resolved_id)
                    {
                        tracing::debug!(
                            strategy = "external_name_lookup",
                            resolved_crate = %resolved_crate,
                            target_path = %target_path,
                            "Successfully resolved external item"
                        );
                        return Some(self.build_markdown_link(
                            current_file,
                            &resolved_crate,
                            target_path,
                            display_name,
                            anchor,
                        ));
                    }

                    tracing::trace!(
                        strategy = "external_name_lookup",
                        resolved_crate = %resolved_crate,
                        resolved_id = ?resolved_id,
                        "Name resolved but no path found in registry"
                    );
                } else {
                    tracing::trace!(
                        strategy = "external_name_lookup",
                        external_crate = %external_crate,
                        item_name = %item_name,
                        "Name not found in external crate registry"
                    );
                }
            }
        } else {
            tracing::trace!(strategy = "external_paths", "No path info found for ID");
        }

        tracing::trace!("All strategies exhausted, returning None");
        None
    }

    /// Resolve `crate::path::Item` or `crate::path::Item::method` patterns.
    ///
    /// This handles the common rustdoc pattern where docs reference items using
    /// crate-relative paths. The tricky part is distinguishing between:
    /// - `crate::module::Type` (link to Type, no anchor)
    /// - `crate::module::Type::method` (link to Type with #method anchor)
    /// - `crate::module::Type::Variant` (link to Type with #Variant anchor)
    ///
    /// # Arguments
    /// * `path_without_crate` - The path after stripping "`crate::`" prefix
    /// * `display_name` - Full original text for display (includes "`crate::`")
    /// * `current_file` - Source file for relative path computation
    ///
    /// # Example Transformation
    ///
    /// ```text
    /// Input:
    ///   path_without_crate = "config::ConfigBuilder::http_status_as_error"
    ///   display_name = "crate::config::ConfigBuilder::http_status_as_error"
    ///   current_file = "ureq/index.md"
    ///
    /// Step 1: Split into type path and anchor
    ///   split_type_and_anchor("config::ConfigBuilder::http_status_as_error")
    ///   → ("config::ConfigBuilder", Some("http_status_as_error"))
    ///   (lowercase "http_status_as_error" indicates a method)
    ///
    /// Step 2: Extract the type name (last segment of type path)
    ///   "config::ConfigBuilder".rsplit("::").next() → "ConfigBuilder"
    ///
    /// Step 3: Resolve type name in registry
    ///   registry.resolve_name("ConfigBuilder", "ureq") → Some(("ureq", Id(123)))
    ///   registry.get_path("ureq", Id(123)) → Some("config/index.md")
    ///
    /// Step 4: Build markdown link with anchor
    ///   build_markdown_link("ureq/index.md", "ureq", "config/index.md",
    ///                       "crate::config::ConfigBuilder::http_status_as_error",
    ///                       Some("http_status_as_error"))
    ///   → "[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)"
    ///
    /// Output: Some("[`crate::config::ConfigBuilder::http_status_as_error`](config/index.md#http_status_as_error)")
    /// ```
    fn resolve_crate_path(
        &self,
        path_without_crate: &str,
        display_name: &str,
        current_file: &str,
    ) -> Option<String> {
        // Step 1: Separate the type path from any method/variant anchor
        // "config::ConfigBuilder::method" → ("config::ConfigBuilder", Some("method"))
        let (type_path, anchor) = Self::split_type_and_anchor(path_without_crate);

        // Step 2: Get just the type name (we'll search for this in the registry)
        // "config::ConfigBuilder" → "ConfigBuilder"
        let type_name = type_path.rsplit("::").next()?;

        // Step 3: Look up the type in our cross-crate registry
        // This finds which crate owns "ConfigBuilder" and what file it's in
        let (resolved_crate, id) = self.registry.resolve_name(type_name, self.crate_name)?;
        let target_path = self.registry.get_path(&resolved_crate, id)?;

        // Step 4: Build the final markdown link
        Some(self.build_markdown_link(
            current_file,
            &resolved_crate,
            target_path,
            display_name,
            anchor,
        ))
    }

    /// Split `config::ConfigBuilder::method` into (`config::ConfigBuilder`, Some("method")).
    ///
    /// Detects methods (lowercase) and enum variants (`Type::Variant` pattern).
    ///
    /// # Detection Rules
    ///
    /// 1. **Methods/fields**: Last segment starts with lowercase
    ///    - `Type::method` → (Type, method)
    ///    - `mod::Type::field_name` → (`mod::Type`, `field_name`)
    ///
    /// 2. **Enum variants**: Two consecutive uppercase segments
    ///    - `Option::Some` → (Option, Some)
    ///    - `mod::Error::IoError` → (`mod::Error`, `IoError`)
    ///
    /// 3. **Nested types**: Uppercase but no uppercase predecessor
    ///    - `mod::OuterType::InnerType` → (`mod::OuterType::InnerType`, None)
    ///
    /// # Examples
    ///
    /// ```text
    /// "ConfigBuilder::http_status_as_error"
    ///   Last segment "http_status_as_error" starts lowercase → method
    ///   → ("ConfigBuilder", Some("http_status_as_error"))
    ///
    /// "config::ConfigBuilder::new"
    ///   Last segment "new" starts lowercase → method
    ///   → ("config::ConfigBuilder", Some("new"))
    ///
    /// "Option::Some"
    ///   "Option" uppercase, "Some" uppercase → enum variant
    ///   → ("Option", Some("Some"))
    ///
    /// "error::Error::Io"
    ///   "Error" uppercase, "Io" uppercase → enum variant
    ///   → ("error::Error", Some("Io"))
    ///
    /// "config::ConfigBuilder"
    ///   "config" lowercase, "ConfigBuilder" uppercase → not a variant
    ///   → ("config::ConfigBuilder", None)
    ///
    /// "Vec"
    ///   No "::" separator
    ///   → ("Vec", None)
    /// ```
    fn split_type_and_anchor(path: &str) -> (&str, Option<&str>) {
        // Find the last "::" separator
        // "config::ConfigBuilder::method" → sep_pos = 21 (before "method")
        let Some(sep_pos) = path.rfind("::") else {
            // No separator, just a simple name like "Vec"
            return (path, None);
        };

        // Split into: rest = "config::ConfigBuilder", last = "method"
        let last = &path[sep_pos + 2..]; // Skip the "::"
        let rest = &path[..sep_pos];

        // ─────────────────────────────────────────────────────────────────────
        // Rule 1: Lowercase last segment = method/field
        // ─────────────────────────────────────────────────────────────────────
        // Methods and fields in Rust are snake_case by convention
        if last.starts_with(|c: char| c.is_lowercase()) {
            return (rest, Some(last));
        }

        // ─────────────────────────────────────────────────────────────────────
        // Rule 2: Check for enum variant (Type::Variant pattern)
        // ─────────────────────────────────────────────────────────────────────
        // Both the type and variant are uppercase (PascalCase)

        // Check if there's another "::" before this one
        // "error::Error::Io" → prev_sep at position of "Error", prev = "Error"
        if let Some(prev_sep) = rest.rfind("::") {
            let prev = &rest[prev_sep + 2..]; // The segment before "last"

            // Both uppercase = likely Type::Variant
            // "Error" uppercase + "Io" uppercase → enum variant
            if prev.starts_with(|c: char| c.is_uppercase())
                && last.starts_with(|c: char| c.is_uppercase())
            {
                return (rest, Some(last));
            }
        } else if rest.starts_with(|c: char| c.is_uppercase())
            && last.starts_with(|c: char| c.is_uppercase())
        {
            // Simple case: "Option::Some" with no module prefix
            // "Option" uppercase + "Some" uppercase → enum variant
            return (rest, Some(last));
        }

        // ─────────────────────────────────────────────────────────────────────
        // No anchor detected
        // ─────────────────────────────────────────────────────────────────────
        // This is something like "mod::Type" where Type is not a variant
        (path, None)
    }

    /// Build a markdown link, handling same-crate and cross-crate cases.
    ///
    /// This is the core function that computes relative paths between markdown
    /// files and formats the final link.
    ///
    /// # Arguments
    /// * `current_file` - The file we're generating (e.g., "ureq/agent/index.md")
    /// * `target_crate` - The crate containing the target item
    /// * `target_path` - Path to target within its crate (e.g., "config/index.md")
    /// * `display_name` - Text to show in the link
    /// * `anchor` - Optional anchor suffix (e.g., "`method_name`")
    ///
    /// # Path Computation Examples
    ///
    /// ## Same Crate Examples
    ///
    /// ```text
    /// Example 1: Link from index to nested module
    ///    current_file = "ureq/index.md"
    ///    target_crate = "ureq"
    ///    target_path = "config/index.md"
    ///
    ///    Step 1: Strip crate prefix from current
    ///      "ureq/index.md" -> "index.md"
    ///
    ///    Step 2: Compute relative path
    ///      from "index.md" to "config/index.md"
    ///      -> "config/index.md"
    ///
    ///    Output: "[`display`](config/index.md)"
    ///
    /// Example 2: Link from nested to sibling module
    ///    current_file = "ureq/agent/index.md"
    ///    target_crate = "ureq"
    ///    target_path = "config/index.md"
    ///
    ///    Step 1: Strip crate prefix
    ///      "ureq/agent/index.md" -> "agent/index.md"
    ///
    ///    Step 2: Compute relative path
    ///      from "agent/index.md" to "config/index.md"
    ///      -> "config/index.md"
    ///
    ///    Output: "[`display`][../config/index.md]"
    ///
    /// ## Cross-Crate Examples
    ///
    /// ```text
    /// Example 3: Link from one crate to another
    ///   current_file = "ureq/agent/index.md"
    ///   target_crate = "http"
    ///   target_path  = "status/index.md"
    ///
    ///   Step 1: Strip crate prefix
    ///     "ureq/agent/index.md" → "agent/index.md"
    ///
    ///   Step 2: Count depth (number of '/' in local path)
    ///     "agent/index.md" has 1 slash → depth = 1
    ///
    ///   Step 3: Build cross-crate path
    ///     Go up (depth + 1) levels: "../" * 2 = "../../"
    ///     Then into target crate: "../../http/status/index.md"
    ///
    ///   Output: "[`display`](../../http/status/index.md)"
    ///
    /// Example 4: Cross-crate from root
    ///   current_file = "ureq/index.md"
    ///   target_crate = "http"
    ///   target_path  = "index.md"
    ///
    ///   depth = 0 (no slashes in "index.md")
    ///   prefix = "../" * 1 = "../"
    ///
    ///   Output: "[`display`](../http/index.md)"
    /// ```
    fn build_markdown_link(
        &self,
        current_file: &str,
        target_crate: &str,
        target_path: &str,
        display_name: &str,
        anchor: Option<&str>,
    ) -> String {
        use crate::linker::LinkRegistry;

        // ------------------------------------------------------------------------
        //  Step 1: Get the crate-local portion of the current path
        // ------------------------------------------------------------------------
        // "ureq/agent/index.md" -> "agent/index.md"
        // This is needed because target_path doesn't include the crate prefix
        let current_local = Self::strip_crate_prefix(current_file);

        // ------------------------------------------------------------------------
        //  Step 2: Compute the file path portion of the link
        // ------------------------------------------------------------------------
        let file_link = if target_crate == self.crate_name {
            // ====================================================================
            //  SAME CRATE: Use relative path within the crate
            // ====================================================================
            if current_local == target_path {
                // Same file, we only need an anchor, no file path.
                // Example: linking to a method on the same page
                String::new()
            } else {
                // Different file in same crate - compute relative path
                // "agent/index.md" -> "config/index.md" = "../config/index.md"
                LinkRegistry::compute_relative_path(current_local, target_path)
            }
        } else {
            // ================================================================
            // CROSS-CRATE: Navigate up to docs root, then into target crate
            // ================================================================
            Self::compute_cross_crate_path(current_local, target_crate, target_path)
        };

        // ─────────────────────────────────────────────────────────────────────
        // Step 3: Build the anchor suffix
        // ─────────────────────────────────────────────────────────────────────
        // Convert anchor to slug format (lowercase, hyphens for special chars)
        // "http_status_as_error" → "#http_status_as_error"
        let anchor_suffix = anchor.map_or_else(String::new, |a| format!("#{}", slugify_anchor(a)));

        // ─────────────────────────────────────────────────────────────────────
        // Step 4: Assemble the final markdown link
        // ─────────────────────────────────────────────────────────────────────
        if file_link.is_empty() {
            // Same file - we need an anchor (either explicit or from display name)
            // If no explicit anchor was provided, use the display name as anchor
            let anchor = if anchor.is_some() {
                anchor_suffix
            } else {
                // Turn display name into anchor: "ConfigBuilder" → "#configbuilder"
                format!("#{}", slugify_anchor(display_name))
            };
            format!("[`{display_name}`]({anchor})")
        } else {
            // Different file - include file path and optional anchor
            format!("[`{display_name}`]({file_link}{anchor_suffix})")
        }
    }

    /// Compute a relative path for cross-crate linking.
    ///
    /// Given the local portion of the current file path (without crate prefix),
    /// computes the `../` prefix needed to navigate to another crate's file.
    ///
    /// # Arguments
    /// * `current_local` - Current file path within crate (e.g., "agent/index.md")
    /// * `target_crate` - Name of the target crate
    /// * `target_path` - Path within target crate (e.g., "status/index.md")
    ///
    /// # Examples
    ///
    /// ```text
    /// // From root of one crate to another
    /// compute_cross_crate_path("index.md", "http", "index.md")
    ///   → "../http/index.md"
    ///
    /// // From nested module to another crate
    /// compute_cross_crate_path("agent/index.md", "http", "status/index.md")
    ///   → "../../http/status/index.md"
    ///
    /// // From deeply nested to another crate root
    /// compute_cross_crate_path("a/b/c/index.md", "other", "index.md")
    ///   → "../../../../other/index.md"
    /// ```
    fn compute_cross_crate_path(
        current_local: &str,
        target_crate: &str,
        target_path: &str,
    ) -> String {
        // Count depth: number of '/' in current path
        // "agent/index.md" has 1 slash → depth = 1
        let depth = current_local.matches('/').count();

        // We need to go up:
        // - `depth` levels to get to crate root
        // - +1 more level to get to docs root (above all crates)
        let prefix = "../".repeat(depth + 1);

        // Then descend into the target crate
        format!("{prefix}{target_crate}/{target_path}")
    }

    /// Strip the crate prefix from a file path.
    ///
    /// File paths in our system includes the crate name as the first directory.
    /// This helper removes it to get the crate-local path.
    ///
    /// # Examples
    ///
    /// ```text
    /// "ureq/config/index.md" -> "config/index.md"
    /// "ureq/index.md"        -> "index.md"
    /// "http/status/index.md" -> "status/index.md"
    /// "simple.md"            -> "simple.md" (no slash returns as is)
    /// ```
    #[inline]
    fn strip_crate_prefix(path: &str) -> &str {
        // Find the first '/' which seperates crate name from the rest
        // "ureq/config/index.md"
        //      ^ position = 4
        //
        // Then return everything after it: "config/index.md"
        path.find('/').map_or(path, |i| &path[(i + 1)..])
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
        let external_patterns = ["std::", "core::", "alloc::", "_crate", "_derive", "_impl"];

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

impl ItemAccess for SingleCrateView<'_> {
    fn krate(&self) -> &Crate {
        self.krate
    }

    fn crate_name(&self) -> &str {
        self.crate_name
    }

    fn get_item(&self, id: &Id) -> Option<&Item> {
        self.krate.index.get(id)
    }

    fn get_impls(&self, id: &Id) -> Option<&[&Impl]> {
        self.impl_map.get(id).map(Vec::as_slice)
    }

    fn crate_version(&self) -> Option<&str> {
        self.krate.crate_version.as_deref()
    }

    fn render_config(&self) -> &RenderConfig {
        &self.ctx.config
    }

    fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig> {
        self.ctx.source_path_config_for_file(current_file)
    }
}

impl ItemFilter for SingleCrateView<'_> {
    fn should_include_item(&self, item: &Item) -> bool {
        match &item.visibility {
            Visibility::Public => true,
            _ => !self.args.exclude_private,
        }
    }

    fn include_private(&self) -> bool {
        !self.args.exclude_private
    }

    fn include_blanket_impls(&self) -> bool {
        self.args.include_blanket_impls
    }
}

impl LinkResolver for SingleCrateView<'_> {
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
        let plain_processed = self.process_plain_links(&backtick_processed, current_file);

        Some(plain_processed)
    }

    fn create_link(&self, id: Id, current_file: &str) -> Option<String> {
        use crate::linker::LinkRegistry;

        // Look up path in the unified registry (crate-local, no prefix)
        let target_path = self.registry.get_path(self.crate_name, id)?;

        // Get the item name for display
        let display_name = self
            .registry
            .get_name(self.crate_name, id)
            .map_or("item", |s| s.as_str());

        // Strip crate prefix from current_file to get crate-local path
        // "crate_name/module/index.md" -> "module/index.md"
        let current_local = Self::strip_crate_prefix(current_file);

        // Compute relative path using the same logic as build_markdown_link
        let relative_path = if current_local == target_path.as_str() {
            // Same file - just use anchor
            format!("#{}", slugify_anchor(display_name))
        } else {
            // Different file - compute relative path within crate
            LinkRegistry::compute_relative_path(current_local, target_path)
        };

        Some(format!("[`{display_name}`]({relative_path})"))
    }
}

// SingleCrateView automatically implements RenderContext via blanket impl

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Tests for split_type_and_anchor
    // =========================================================================

    mod split_type_and_anchor {
        use super::*;

        #[test]
        fn simple_type_no_anchor() {
            assert_eq!(SingleCrateView::split_type_and_anchor("Vec"), ("Vec", None));
        }

        #[test]
        fn module_path_no_anchor() {
            // Module prefix + type = no anchor (lowercase then uppercase)
            assert_eq!(
                SingleCrateView::split_type_and_anchor("config::ConfigBuilder"),
                ("config::ConfigBuilder", None)
            );
        }

        #[test]
        fn type_with_method() {
            // Type::method - last segment lowercase = method anchor
            assert_eq!(
                SingleCrateView::split_type_and_anchor("Type::method"),
                ("Type", Some("method"))
            );
        }

        #[test]
        fn type_with_snake_case_method() {
            assert_eq!(
                SingleCrateView::split_type_and_anchor("ConfigBuilder::http_status_as_error"),
                ("ConfigBuilder", Some("http_status_as_error"))
            );
        }

        #[test]
        fn module_type_method() {
            // Full path with method
            assert_eq!(
                SingleCrateView::split_type_and_anchor("config::ConfigBuilder::new"),
                ("config::ConfigBuilder", Some("new"))
            );
        }

        #[test]
        fn enum_variant_simple() {
            // Both uppercase = enum variant
            assert_eq!(
                SingleCrateView::split_type_and_anchor("Option::Some"),
                ("Option", Some("Some"))
            );
        }

        #[test]
        fn enum_variant_with_module() {
            // Module + Type::Variant
            assert_eq!(
                SingleCrateView::split_type_and_anchor("error::Error::Io"),
                ("error::Error", Some("Io"))
            );
        }

        #[test]
        fn result_variant() {
            assert_eq!(
                SingleCrateView::split_type_and_anchor("Result::Ok"),
                ("Result", Some("Ok"))
            );
        }

        #[test]
        fn nested_modules_with_type() {
            // Deep nesting ending in type (no anchor)
            assert_eq!(
                SingleCrateView::split_type_and_anchor("a::b::c::Type"),
                ("a::b::c::Type", None)
            );
        }

        #[test]
        fn nested_modules_with_method() {
            // Deep nesting ending in method
            assert_eq!(
                SingleCrateView::split_type_and_anchor("a::b::Type::method"),
                ("a::b::Type", Some("method"))
            );
        }

        #[test]
        fn associated_type_treated_as_variant() {
            // Iterator::Item - both uppercase, treated as variant (acceptable)
            assert_eq!(
                SingleCrateView::split_type_and_anchor("Iterator::Item"),
                ("Iterator", Some("Item"))
            );
        }

        #[test]
        fn const_associated_item() {
            // Type::CONST - uppercase const, treated as variant
            assert_eq!(
                SingleCrateView::split_type_and_anchor("Type::MAX"),
                ("Type", Some("MAX"))
            );
        }
    }

    // =========================================================================
    // Tests for strip_crate_prefix
    // =========================================================================

    mod strip_crate_prefix {
        use super::*;

        #[test]
        fn strips_crate_from_nested_path() {
            assert_eq!(
                SingleCrateView::strip_crate_prefix("ureq/config/index.md"),
                "config/index.md"
            );
        }

        #[test]
        fn strips_crate_from_root() {
            assert_eq!(
                SingleCrateView::strip_crate_prefix("ureq/index.md"),
                "index.md"
            );
        }

        #[test]
        fn strips_crate_from_deep_path() {
            assert_eq!(
                SingleCrateView::strip_crate_prefix("http/uri/authority/index.md"),
                "uri/authority/index.md"
            );
        }

        #[test]
        fn no_slash_returns_as_is() {
            assert_eq!(
                SingleCrateView::strip_crate_prefix("simple.md"),
                "simple.md"
            );
        }
    }

    // =========================================================================
    // Tests for looks_like_external_reference
    // =========================================================================

    mod looks_like_external_reference {
        use super::*;

        #[test]
        fn qualified_path_is_external() {
            assert!(SingleCrateView::looks_like_external_reference(
                "std::io::Error"
            ));
        }

        #[test]
        fn crate_path_is_external() {
            assert!(SingleCrateView::looks_like_external_reference(
                "regex::Regex"
            ));
        }

        #[test]
        fn std_prefix_is_external() {
            assert!(SingleCrateView::looks_like_external_reference(
                "std::vec::Vec"
            ));
        }

        #[test]
        fn core_prefix_is_external() {
            assert!(SingleCrateView::looks_like_external_reference(
                "core::mem::drop"
            ));
        }

        #[test]
        fn alloc_prefix_is_external() {
            assert!(SingleCrateView::looks_like_external_reference(
                "alloc::string::String"
            ));
        }

        #[test]
        fn simple_name_not_external() {
            assert!(!SingleCrateView::looks_like_external_reference("Error"));
        }

        #[test]
        fn pascal_case_not_external() {
            assert!(!SingleCrateView::looks_like_external_reference(
                "ConfigBuilder"
            ));
        }

        #[test]
        fn derive_suffix_is_external() {
            assert!(SingleCrateView::looks_like_external_reference(
                "serde_derive"
            ));
        }
    }

    // =========================================================================
    // Tests for compute_cross_crate_path (relative path computation)
    // =========================================================================

    mod compute_cross_crate_path {
        use super::*;

        #[test]
        fn from_root_to_root() {
            // From crate root (index.md) to another crate's root
            assert_eq!(
                SingleCrateView::compute_cross_crate_path("index.md", "http", "index.md"),
                "../http/index.md"
            );
        }

        #[test]
        fn from_root_to_nested() {
            // From crate root to nested module in another crate
            assert_eq!(
                SingleCrateView::compute_cross_crate_path("index.md", "http", "status/index.md"),
                "../http/status/index.md"
            );
        }

        #[test]
        fn from_nested_to_root() {
            // From nested module to another crate's root
            // depth = 1 (one '/'), needs "../" * 2 = "../../"
            assert_eq!(
                SingleCrateView::compute_cross_crate_path("agent/index.md", "http", "index.md"),
                "../../http/index.md"
            );
        }

        #[test]
        fn from_nested_to_nested() {
            // From nested module to nested module in another crate
            assert_eq!(
                SingleCrateView::compute_cross_crate_path(
                    "agent/index.md",
                    "http",
                    "status/index.md"
                ),
                "../../http/status/index.md"
            );
        }

        #[test]
        fn from_deeply_nested() {
            // From deeply nested (3 levels) to another crate
            // depth = 3, needs "../" * 4 = "../../../../"
            assert_eq!(
                SingleCrateView::compute_cross_crate_path("a/b/c/index.md", "other", "index.md"),
                "../../../../other/index.md"
            );
        }

        #[test]
        fn to_deeply_nested() {
            // From root to deeply nested in another crate
            assert_eq!(
                SingleCrateView::compute_cross_crate_path("index.md", "target", "x/y/z/index.md"),
                "../target/x/y/z/index.md"
            );
        }

        #[test]
        fn both_deeply_nested() {
            // Both source and target are deeply nested
            assert_eq!(
                SingleCrateView::compute_cross_crate_path("a/b/index.md", "target", "x/y/index.md"),
                "../../../target/x/y/index.md"
            );
        }
    }

    // Note: process_plain_links tests removed - function is now registry-aware
    // and requires a full SingleCrateView context. Behavior is tested via
    // integration tests.
}
