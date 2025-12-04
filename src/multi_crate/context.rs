//! Multi-crate generation context.
//!
//! This module provides [`MultiCrateContext`] which holds shared state
//! during multi-crate documentation generation, and [`SingleCrateView`]
//! which provides a single-crate interface for existing rendering code.

use crate::multi_crate::{CrateCollection, UnifiedLinkRegistry};
use crate::Args;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Visibility};
use std::collections::HashMap;

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
    pub fn crates(&self) -> &CrateCollection {
        self.crates
    }

    /// Get the unified link registry.
    #[must_use]
    pub fn registry(&self) -> &UnifiedLinkRegistry {
        &self.registry
    }

    /// Get CLI arguments.
    #[must_use]
    pub fn args(&self) -> &Args {
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
            crate_name.to_string(),
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

    /// Map from type ID to impl blocks.
    impl_map: HashMap<Id, Vec<&'a Impl>>,
}

impl<'a> SingleCrateView<'a> {
    /// Create a new single-crate view.
    fn new(
        crate_name: String,
        krate: &'a Crate,
        registry: &'a UnifiedLinkRegistry,
        args: &'a Args,
        ctx: &'a MultiCrateContext<'a>,
    ) -> Self {
        let mut view = Self {
            crate_name,
            krate,
            registry,
            args,
            ctx,
            path_map: HashMap::new(),
            impl_map: HashMap::new(),
        };

        view.build_path_map();
        view.build_impl_map();

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
            let mut path = parent_path.clone();
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
            if let ItemEnum::Impl(impl_block) = &item.inner {
                if let Some(target_id) = Self::get_impl_target_id(impl_block) {
                    self.impl_map.entry(target_id).or_default().push(impl_block);
                }
            }
        }

        // Sort impl blocks for deterministic output
        for impls in self.impl_map.values_mut() {
            impls.sort_by_key(|i| Self::impl_sort_key(i));
        }
    }

    /// Get the target type ID for an impl block.
    fn get_impl_target_id(impl_block: &Impl) -> Option<Id> {
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
    pub fn krate(&self) -> &Crate {
        self.krate
    }

    /// Get the unified registry.
    #[must_use]
    pub fn registry(&self) -> &UnifiedLinkRegistry {
        self.registry
    }

    /// Get CLI arguments.
    #[must_use]
    pub fn args(&self) -> &Args {
        self.args
    }

    /// Get module path for an item.
    #[must_use]
    pub fn get_path(&self, id: Id) -> Option<&Vec<String>> {
        self.path_map.get(&id)
    }

    /// Get impl blocks for a type.
    #[must_use]
    pub fn get_impls(&self, id: Id) -> Option<&Vec<&'a Impl>> {
        self.impl_map.get(&id)
    }

    /// Check if an item should be included based on visibility.
    #[must_use]
    pub fn should_include_item(&self, item: &rustdoc_types::Item) -> bool {
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
        self.registry.create_link(&self.crate_name, from_path, to_crate, to_id)
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
}
