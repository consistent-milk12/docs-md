//! Unified link registry for cross-crate documentation.
//!
//! This module provides [`UnifiedLinkRegistry`] which maps item IDs across
//! multiple crates to their documentation file paths, enabling cross-crate
//! linking in the generated markdown.

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use compact_str::CompactString;
use rustdoc_types::{Crate, Id, ItemEnum, ItemKind, Visibility};
use tracing::{debug, instrument, trace};

use super::{CrateCollection, RUST_PATH_SEP};
use crate::linker::{LinkRegistry, slugify_anchor};

/// Compact string type for memory-efficient storage.
/// Strings ≤24 bytes are stored inline (no heap allocation).
/// Most crate names, item names, and short paths fit inline.
type Str = CompactString;

/// Key type for registry lookups: `(crate_name, item_id)`.
///
/// Uses `CompactString` for memory efficiency - most crate names are short
/// and stored inline without heap allocation.
type RegistryKey = (Str, Id);

/// Borrowed key for zero-allocation lookups.
///
/// Must hash identically to `RegistryKey` tuple of `(CompactString, Id)`.
#[derive(PartialEq, Eq)]
struct BorrowedKey<'a>(&'a str, Id);

impl Hash for BorrowedKey<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash exactly like a tuple (CompactString, Id) would:
        // CompactString hashes as its byte content, same as &str
        self.0.hash(state);
        self.1.hash(state);
    }
}

/// Allow comparing `BorrowedKey` with `RegistryKey`.
fn keys_match(stored: &RegistryKey, borrowed: &BorrowedKey<'_>) -> bool {
    stored.0 == borrowed.0 && stored.1 == borrowed.1
}

/// Registry mapping item IDs to documentation paths across multiple crates.
///
/// Unlike [`LinkRegistry`] which handles a single crate, this registry
/// spans multiple crates and supports cross-crate link resolution with
/// disambiguation based on local/primary crate preference.
///
/// # Path Format
///
/// All paths use the nested format: `{crate_name}/{module_path}/index.md`
///
/// Examples:
/// - `tracing/index.md` (crate root)
/// - `tracing/span/index.md` (module)
/// - `tracing_core/subscriber/index.md` (cross-crate reference)
///
/// # Link Resolution Priority
///
/// When resolving ambiguous names:
/// 1. Items in the current crate (where the link appears)
/// 2. Items in the primary crate (if specified via `--primary-crate`)
/// 3. Items with the shortest qualified path
///
/// # Performance
///
/// Uses `hashbrown` with raw entry API for zero-allocation lookups.
/// This avoids allocating a `String` for the crate name on every lookup.
#[derive(Debug, Default)]
pub struct UnifiedLinkRegistry {
    /// Maps `(crate_name, item_id)` to the file path within output.
    /// Uses hashbrown for `raw_entry` API (zero-alloc lookups).
    item_paths: hashbrown::HashMap<RegistryKey, Str>,

    /// Maps `(crate_name, item_id)` to the item's display name.
    /// Uses hashbrown for `raw_entry` API (zero-alloc lookups).
    item_names: hashbrown::HashMap<RegistryKey, Str>,

    /// Maps short names to all `(crate_name, item_id, item_kind)` tuples.
    /// Used for disambiguating links like `Span` that exist in multiple crates.
    /// The `ItemKind` enables preferring modules over macros with the same name.
    name_index: HashMap<Str, Vec<(Str, Id, ItemKind)>>,

    /// Maps `(crate_name, reexport_id)` to the original source path.
    /// Used for resolving external re-exports where `use_item.id` is `None`
    /// but `use_item.source` provides the canonical path.
    /// Example: `("tracing", id_123)` -> `"tracing_core::field::Visit"`
    re_export_sources: hashbrown::HashMap<RegistryKey, Str>,

    /// The primary crate name for preferential resolution.
    primary_crate: Option<Str>,
}

impl UnifiedLinkRegistry {
    /// Build a unified registry from a collection of crates.
    ///
    /// # Arguments
    ///
    /// * `crates` - Collection of parsed crates
    /// * `primary_crate` - Optional primary crate for disambiguation
    ///
    /// # Returns
    ///
    /// A populated registry ready for link resolution.
    #[must_use]
    #[instrument(skip(crates), fields(crate_count = crates.names().len()))]
    pub fn build(crates: &CrateCollection, primary_crate: Option<&str>) -> Self {
        debug!(?primary_crate, "Building unified link registry");

        let mut registry = Self {
            primary_crate: primary_crate.map(Str::from),
            ..Default::default()
        };

        // Register all items from each crate
        for (crate_name, krate) in crates.iter() {
            trace!(crate_name, "Registering crate items");
            registry.register_crate(crate_name, krate);
        }

        debug!(
            item_count = registry.item_paths.len(),
            name_count = registry.name_index.len(),
            "Registry build complete"
        );

        registry
    }

    /// Register all items from a single crate.
    fn register_crate(&mut self, crate_name: &str, krate: &Crate) {
        // Get root module
        let Some(root) = krate.index.get(&krate.root) else {
            return;
        };

        // Register root module at index.md (no crate prefix in path)
        self.register_item(crate_name, krate.root, crate_name, "index.md", ItemKind::Module);

        // Strategy 1: Use the `paths` field to register all items by their canonical path
        // This catches items that are re-exported or in private modules
        self.register_from_paths(crate_name, krate);

        // Strategy 2: Process all items in root module recursively
        // This ensures we have correct paths for the generated markdown structure
        if let ItemEnum::Module(module) = &root.inner {
            for item_id in &module.items {
                if let Some(item) = krate.index.get(item_id) {
                    self.register_item_recursive(krate, crate_name, *item_id, item, "");
                }
            }
        }
    }

    /// Register items using the `paths` field from rustdoc JSON.
    ///
    /// The `paths` field contains canonical paths for all items, including
    /// those in private modules that are re-exported publicly. Since we only
    /// generate docs for public modules, items in private modules are
    /// documented at their public re-export location (typically root).
    fn register_from_paths(&mut self, crate_name: &str, krate: &Crate) {
        for (id, path_info) in &krate.paths {
            // Only register items from this crate
            if path_info.crate_id != 0 {
                continue;
            }

            // Get the item name (last segment of path)
            let Some(name) = path_info.path.last() else {
                continue;
            };

            // Skip modules - they're handled by recursive traversal
            if path_info.kind == rustdoc_types::ItemKind::Module {
                continue;
            }

            // Items from paths are typically in private modules that get re-exported
            // at the crate root. Register them at index.md since that's where
            // public re-exports are documented.
            // The recursive traversal will overwrite with correct paths for items
            // that ARE in public modules.
            self.register_item(crate_name, *id, name, "index.md", path_info.kind);
        }
    }

    /// Convert `ItemEnum` to `ItemKind` for the name index.
    fn item_enum_to_kind(inner: &ItemEnum) -> ItemKind {
        match inner {
            ItemEnum::Module(_) => ItemKind::Module,
            ItemEnum::Struct(_) => ItemKind::Struct,
            ItemEnum::Enum(_) => ItemKind::Enum,
            ItemEnum::Trait(_) => ItemKind::Trait,
            ItemEnum::Function(_) => ItemKind::Function,
            ItemEnum::Constant { .. } => ItemKind::Constant,
            ItemEnum::TypeAlias(_) => ItemKind::TypeAlias,
            ItemEnum::Macro(_) => ItemKind::Macro,
            ItemEnum::Use(_) => ItemKind::Use,
            _ => ItemKind::Use, // Fallback for other types
        }
    }

    /// Recursively register an item and its children.
    fn register_item_recursive(
        &mut self,
        krate: &Crate,
        crate_name: &str,
        item_id: Id,
        item: &rustdoc_types::Item,
        parent_path: &str,
    ) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        match &item.inner {
            // Modules get their own directory with index.md
            ItemEnum::Module(module) => {
                // Build module path (handle empty parent for root-level modules)
                let module_path = if parent_path.is_empty() {
                    name.to_string()
                } else {
                    format!("{parent_path}/{name}")
                };
                let file_path = format!("{module_path}/index.md");

                self.register_item(crate_name, item_id, name, &file_path, ItemKind::Module);

                // Recurse into child items
                for child_id in &module.items {
                    if let Some(child) = krate.index.get(child_id) {
                        self.register_item_recursive(
                            krate,
                            crate_name,
                            *child_id,
                            child,
                            &module_path,
                        );
                    }
                }
            },

            // Types and functions are documented in their parent's index.md
            ItemEnum::Struct(_)
            | ItemEnum::Enum(_)
            | ItemEnum::Trait(_)
            | ItemEnum::Function(_)
            | ItemEnum::Constant { .. }
            | ItemEnum::TypeAlias(_)
            | ItemEnum::Macro(_) => {
                // Handle root-level items (parent_path is empty)
                let file_path = if parent_path.is_empty() {
                    "index.md".to_string()
                } else {
                    format!("{parent_path}/index.md")
                };
                let kind = Self::item_enum_to_kind(&item.inner);
                self.register_item(crate_name, item_id, name, &file_path, kind);
            },

            // Re-exports (pub use) should be registered under this crate's namespace
            // This allows links to resolve within the current crate rather than cross-crate
            ItemEnum::Use(use_item) => {
                let file_path = if parent_path.is_empty() {
                    "index.md".to_string()
                } else {
                    format!("{parent_path}/index.md")
                };

                if use_item.is_glob {
                    // Register items from glob re-export target
                    if let Some(target_id) = &use_item.id
                        && let Some(target_module) = krate.index.get(target_id)
                        && let ItemEnum::Module(module) = &target_module.inner
                    {
                        for child_id in &module.items {
                            if let Some(child) = krate.index.get(child_id) {
                                // Check visibility
                                if !matches!(child.visibility, Visibility::Public) {
                                    continue;
                                }
                                let child_name = child.name.as_deref().unwrap_or("unnamed");
                                let child_kind = Self::item_enum_to_kind(&child.inner);
                                self.register_item(crate_name, *child_id, child_name, &file_path, child_kind);
                            }
                        }
                    }
                } else {
                    // Specific re-export - try to get kind from target, fallback to Use
                    let export_name = &use_item.name;
                    let kind = use_item
                        .id
                        .and_then(|id| krate.index.get(&id))
                        .map_or(ItemKind::Use, |target| Self::item_enum_to_kind(&target.inner));
                    self.register_item(crate_name, item_id, export_name, &file_path, kind);

                    // For ALL re-exports, store the source path so we can
                    // resolve to the original definition (which has a heading)
                    if !use_item.source.is_empty() {
                        let key = (Str::from(crate_name), item_id);
                        self.re_export_sources.insert(key, Str::from(use_item.source.as_str()));
                    }
                }
            },

            _ => {},
        }
    }

    /// Register a single item in the registry.
    fn register_item(&mut self, crate_name: &str, id: Id, name: &str, path: &str, kind: ItemKind) {
        let key = (Str::from(crate_name), id);

        self.item_paths.insert(key.clone(), Str::from(path));
        self.item_names.insert(key, Str::from(name));

        // Add to name index for disambiguation (includes kind for preference logic)
        self.name_index
            .entry(Str::from(name))
            .or_default()
            .push((Str::from(crate_name), id, kind));
    }

    /// Get the file path for an item in a specific crate.
    ///
    /// Uses raw entry API for zero-allocation lookup.
    #[must_use]
    #[instrument(skip(self), level = "trace")]
    pub fn get_path(&self, crate_name: &str, id: Id) -> Option<&Str> {
        use std::hash::BuildHasher;
        let borrowed = BorrowedKey(crate_name, id);
        let hash = self.item_paths.hasher().hash_one(&borrowed);
        let result = self
            .item_paths
            .raw_entry()
            .from_hash(hash, |k| keys_match(k, &borrowed))
            .map(|(_, v)| v);

        trace!(found = result.is_some(), "Path lookup");
        result
    }

    /// Get the display name for an item.
    ///
    /// Uses raw entry API for zero-allocation lookup.
    #[must_use]
    pub fn get_name(&self, crate_name: &str, id: Id) -> Option<&Str> {
        use std::hash::BuildHasher;
        let borrowed = BorrowedKey(crate_name, id);
        let hash = self.item_names.hasher().hash_one(&borrowed);
        self.item_names
            .raw_entry()
            .from_hash(hash, |k| keys_match(k, &borrowed))
            .map(|(_, v)| v)
    }

    /// Get the original source path for an external re-export.
    ///
    /// Returns `Some("crate::path::Item")` if this item is a re-export
    /// from another crate, `None` otherwise.
    #[must_use]
    pub fn get_re_export_source(&self, crate_name: &str, id: Id) -> Option<&Str> {
        use std::hash::BuildHasher;
        let borrowed = BorrowedKey(crate_name, id);
        let hash = self.re_export_sources.hasher().hash_one(&borrowed);
        self.re_export_sources
            .raw_entry()
            .from_hash(hash, |k| keys_match(k, &borrowed))
            .map(|(_, v)| v)
    }

    /// Resolve through re-export chain to find the canonical item.
    ///
    /// If the item is an external re-export, follows the source path
    /// to find the original crate and ID. Returns the original if found,
    /// otherwise returns `None`.
    ///
    /// # Arguments
    ///
    /// * `crate_name` - The crate where the re-export appears
    /// * `id` - The ID of the re-export Use item
    ///
    /// # Returns
    ///
    /// `Some((original_crate, original_id))` if the re-export chain can be resolved,
    /// `None` if there's no re-export source or the original can't be found.
    #[must_use]
    pub fn resolve_reexport(&self, crate_name: &str, id: Id) -> Option<(Str, Id)> {
        let source = self.get_re_export_source(crate_name, id)?;
        self.resolve_path(source)
    }

    /// Resolve an item name to its crate and ID.
    ///
    /// Uses disambiguation priority:
    /// 1. Current crate (modules preferred over macros)
    /// 2. Primary crate (if set, modules preferred)
    /// 3. First module match, then first non-module match
    #[must_use]
    #[instrument(skip(self), level = "trace")]
    pub fn resolve_name(&self, name: &str, current_crate: &str) -> Option<(Str, Id)> {
        let candidates = self.name_index.get(name)?;

        if candidates.is_empty() {
            trace!("No candidates found");
            return None;
        }

        // Priority 1: Current crate - prefer modules over macros
        let current_crate_candidates: Vec<_> = candidates
            .iter()
            .filter(|(crate_name, _, _)| crate_name == current_crate)
            .collect();

        if !current_crate_candidates.is_empty() {
            // Prefer module if available
            if let Some((crate_name, id, _)) = current_crate_candidates
                .iter()
                .find(|(_, _, kind)| *kind == ItemKind::Module)
            {
                trace!(resolved_crate = %crate_name, "Resolved to current crate (module)");
                return Some(((*crate_name).clone(), *id));
            }
            // Otherwise take first match from current crate
            let (crate_name, id, _) = current_crate_candidates[0];
            trace!(resolved_crate = %crate_name, "Resolved to current crate");
            return Some((crate_name.clone(), *id));
        }

        // Priority 2: Primary crate - prefer modules
        if let Some(primary) = &self.primary_crate {
            let primary_candidates: Vec<_> = candidates
                .iter()
                .filter(|(crate_name, _, _)| crate_name == primary)
                .collect();

            if !primary_candidates.is_empty() {
                // Prefer module if available
                if let Some((crate_name, id, _)) = primary_candidates
                    .iter()
                    .find(|(_, _, kind)| *kind == ItemKind::Module)
                {
                    trace!(resolved_crate = %crate_name, "Resolved to primary crate (module)");
                    return Some(((*crate_name).clone(), *id));
                }
                // Otherwise take first match from primary crate
                let (crate_name, id, _) = primary_candidates[0];
                trace!(resolved_crate = %crate_name, "Resolved to primary crate");
                return Some((crate_name.clone(), *id));
            }
        }

        // Priority 3: Prefer any module, then first match
        if let Some((crate_name, id, _)) = candidates
            .iter()
            .find(|(_, _, kind)| *kind == ItemKind::Module)
        {
            trace!(resolved_crate = %crate_name, "Resolved to module");
            return Some((crate_name.clone(), *id));
        }

        let result = candidates.first().map(|(c, id, _)| (c.clone(), *id));
        trace!(
            resolved_crate = ?result.as_ref().map(|(c, _)| c),
            "Resolved to first match"
        );
        result
    }

    /// Resolve a full path like `regex_automata::Regex` to its crate and ID.
    ///
    /// This is used for resolving external re-exports where `use_item.id` is `None`
    /// but the source path is available.
    ///
    /// # Arguments
    ///
    /// * `path` - Full path like `regex_automata::Regex` or `tracing_core::span::Span`
    ///
    /// # Returns
    ///
    /// The (`crate_name`, `item_id`) if found in the registry.
    #[must_use]
    pub fn resolve_path(&self, path: &str) -> Option<(Str, Id)> {
        let segments: Vec<&str> = path.split(RUST_PATH_SEP).collect();

        if segments.is_empty() {
            return None;
        }

        // First segment is the crate name
        let target_crate = segments[0];

        // Last segment is the item name
        let item_name = segments.last()?;

        // Look up in name_index and filter by crate
        let candidates = self.name_index.get(*item_name)?;

        for (crate_name, id, _kind) in candidates {
            if crate_name == target_crate {
                return Some((crate_name.clone(), *id));
            }
        }

        None
    }

    /// Create a markdown link from one file to another across crates.
    ///
    /// # Arguments
    ///
    /// * `from_crate` - The crate where the link appears
    /// * `from_path` - The file path where the link appears
    /// * `to_crate` - The target crate
    /// * `to_id` - The target item's ID
    ///
    /// # Returns
    ///
    /// A formatted markdown link like `[`Name`](relative/path.md)`,
    /// or `None` if the target item isn't registered.
    #[must_use]
    pub fn create_link(
        &self,
        from_crate: &str,
        from_path: &str,
        to_crate: &str,
        to_id: Id,
    ) -> Option<String> {
        let target_path = self.get_path(to_crate, to_id)?;
        let name = self.get_name(to_crate, to_id)?;

        // Build full paths including crate directory
        let from_full = format!("{from_crate}/{from_path}");
        let to_full = format!("{to_crate}/{target_path}");

        // Compute relative path
        let relative = Self::compute_cross_crate_path(&from_full, &to_full);

        // Check if same file - use anchor instead
        if from_full == to_full {
            let anchor = slugify_anchor(name);
            return Some(format!("[`{name}`](#{anchor})"));
        }

        Some(format!("[`{name}`]({relative})"))
    }

    /// Compute relative path between files potentially in different crates.
    ///
    /// # Examples
    ///
    /// - `tracing/span/index.md` to `tracing_core/subscriber/index.md`
    ///   = `../../tracing_core/subscriber/index.md`
    /// - `tracing/index.md` to `tracing/span/index.md`
    ///   = `span/index.md`
    #[must_use]
    pub fn compute_cross_crate_path(from: &str, to: &str) -> String {
        // Delegate to the single-crate implementation - it handles
        // the path computation correctly regardless of crate boundaries
        LinkRegistry::compute_relative_path(from, to)
    }

    /// Get an anchor string for an item within its page.
    ///
    /// # Arguments
    ///
    /// * `crate_name` - The crate containing the item
    /// * `id` - The item's ID
    ///
    /// # Returns
    ///
    /// An anchor like `#span` or `#enter` for linking to specific items.
    #[must_use]
    pub fn get_anchor(&self, crate_name: &str, id: Id) -> Option<String> {
        let name = self.get_name(crate_name, id)?;
        Some(format!("#{}", slugify_anchor(name)))
    }

    /// Check if an item exists in the registry.
    ///
    /// Uses raw entry API for zero-allocation lookup.
    #[must_use]
    pub fn contains(&self, crate_name: &str, id: Id) -> bool {
        use std::hash::BuildHasher;
        let borrowed = BorrowedKey(crate_name, id);
        let hash = self.item_paths.hasher().hash_one(&borrowed);
        self.item_paths
            .raw_entry()
            .from_hash(hash, |k| keys_match(k, &borrowed))
            .is_some()
    }

    /// Get the number of registered items.
    #[must_use]
    pub fn len(&self) -> usize {
        self.item_paths.len()
    }

    /// Check if the registry is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.item_paths.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::DefaultHashBuilder;

    use super::*;

    #[test]
    fn test_cross_crate_path_same_crate() {
        assert_eq!(
            UnifiedLinkRegistry::compute_cross_crate_path(
                "tracing/index.md",
                "tracing/span/index.md"
            ),
            "span/index.md"
        );
    }

    #[test]
    fn test_cross_crate_path_different_crates() {
        assert_eq!(
            UnifiedLinkRegistry::compute_cross_crate_path(
                "tracing/span/index.md",
                "tracing_core/subscriber/index.md"
            ),
            "../../tracing_core/subscriber/index.md"
        );
    }

    #[test]
    fn test_cross_crate_path_to_root() {
        assert_eq!(
            UnifiedLinkRegistry::compute_cross_crate_path(
                "tracing/span/enter/index.md",
                "tracing/index.md"
            ),
            "../../index.md"
        );
    }

    /// Verify that `BorrowedKey` and `RegistryKey` hash identically.
    #[test]
    fn test_borrowed_key_hash_compatibility() {
        use std::hash::BuildHasher;

        // Use a fixed hasher (same instance for both)
        let hasher = DefaultHashBuilder::default();
        let id = Id(42);

        // Create owned key (how it's stored in the HashMap)
        let owned: RegistryKey = (Str::from("test_crate"), id);

        // Create borrowed key (how we look up)
        let borrowed = BorrowedKey("test_crate", id);

        // Hashes must be equal for raw_entry lookup to work
        // Using the SAME hasher instance is critical
        let owned_hash = hasher.hash_one(&owned);
        let borrowed_hash = hasher.hash_one(&borrowed);

        assert_eq!(
            owned_hash, borrowed_hash,
            "BorrowedKey hash must equal RegistryKey hash"
        );
    }

    /// Test that `raw_entry` lookup works correctly.
    #[test]
    fn test_raw_entry_lookup() {
        let mut registry = UnifiedLinkRegistry::default();
        let id = Id(123);

        // Insert using owned key
        registry.register_item("my_crate", id, "MyType", "module/index.md", ItemKind::Struct);

        // Lookup using borrowed key (zero-allocation)
        assert!(registry.contains("my_crate", id));
        assert_eq!(
            registry.get_path("my_crate", id),
            Some(&Str::from("module/index.md"))
        );
        assert_eq!(
            registry.get_name("my_crate", id),
            Some(&Str::from("MyType"))
        );

        // Non-existent lookups
        assert!(!registry.contains("other_crate", id));
        assert!(registry.get_path("other_crate", id).is_none());
    }
}
