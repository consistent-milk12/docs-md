//! Shared context for documentation generation.
//!
//! This module provides the [`GeneratorContext`] struct which holds all shared
//! state needed during markdown generation, including the crate data, lookup
//! maps, and configuration options.

use crate::Args;
use crate::linker::LinkRegistry;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Visibility};
use std::collections::HashMap;

/// Shared context containing all data needed for documentation generation.
///
/// This struct is passed to all rendering components and provides:
/// - Access to the parsed crate data
/// - Path mapping for "defined in" information
/// - Impl block lookup for rendering implementations
/// - Link registry for cross-references
/// - CLI configuration options
pub struct GeneratorContext<'a> {
    /// The parsed rustdoc JSON crate.
    pub krate: &'a Crate,

    /// Maps item IDs to their full module paths.
    ///
    /// Used for generating "defined in" information.
    /// For `std::collections::HashMap`, the path would be `["std", "collections", "HashMap"]`.
    pub path_map: HashMap<Id, Vec<String>>,

    /// Maps type IDs to all impl blocks for that type.
    ///
    /// Used for rendering "Implementations" and "Trait Implementations" sections.
    pub impl_map: HashMap<Id, Vec<&'a Impl>>,

    /// Registry for creating cross-reference links between items.
    pub link_registry: LinkRegistry,

    /// CLI arguments containing output path, format, and options.
    pub args: &'a Args,
}

impl<'a> GeneratorContext<'a> {
    /// Create a new generator context from crate data and CLI arguments.
    ///
    /// Builds the path map, impl map, and link registry needed for generation.
    ///
    /// # Arguments
    ///
    /// * `krate` - The parsed rustdoc JSON crate
    /// * `args` - CLI arguments containing output path, format, and options
    #[must_use]
    pub fn new(krate: &'a Crate, args: &'a Args) -> Self {
        use crate::CliOutputFormat;

        let path_map = Self::build_path_map(krate);
        let impl_map = Self::build_impl_map(krate);
        let is_flat = matches!(args.format, CliOutputFormat::Flat);
        let link_registry = LinkRegistry::build(krate, is_flat);

        Self {
            krate,
            path_map,
            impl_map,
            link_registry,
            args,
        }
    }

    /// Build a map from item ID to its full module path.
    ///
    /// The rustdoc JSON `paths` field contains this information for all
    /// items that can be linked to. We copy it into a `HashMap` for fast lookup.
    fn build_path_map(krate: &Crate) -> HashMap<Id, Vec<String>> {
        krate
            .paths
            .iter()
            .map(|(id, path_info)| (*id, path_info.path.clone()))
            .collect()
    }

    /// Build a map from type ID to all impl blocks for that type.
    ///
    /// This enables rendering the "Implementations" and "Trait Implementations"
    /// sections for structs, enums, and other types.
    ///
    /// Rustdoc JSON stores impl blocks as separate items in the index.
    /// Each impl has a `for_` field indicating what type it implements for.
    /// We walk all items, find impls, and group them by their target type ID.
    fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>> {
        let mut map: HashMap<Id, Vec<&'a Impl>> = HashMap::new();

        for item in krate.index.values() {
            if let ItemEnum::Impl(impl_block) = &item.inner
                && let Some(type_id) = Self::get_type_id(&impl_block.for_)
            {
                map.entry(type_id).or_default().push(impl_block);
            }
        }

        map
    }

    /// Extract the item ID from a Type if it's a resolved path.
    ///
    /// Only `ResolvedPath` types (named types like `Vec`, `String`, `MyStruct`)
    /// have associated IDs. Other types (primitives, references, etc.) return None.
    fn get_type_id(ty: &rustdoc_types::Type) -> Option<Id> {
        match ty {
            rustdoc_types::Type::ResolvedPath(path) => Some(path.id),
            _ => None,
        }
    }

    /// Check if an item should be included based on visibility settings.
    ///
    /// By default, only public items are included. If `--include-private`
    /// is set, all items are included regardless of visibility.
    ///
    /// # Visibility Levels
    ///
    /// - `Public` - Always included
    /// - `Crate`, `Restricted`, `Default` - Only with `--include-private`
    #[must_use]
    pub fn should_include_item(&self, item: &Item) -> bool {
        match &item.visibility {
            Visibility::Public => true,
            _ => self.args.include_private,
        }
    }

    /// Count the total number of modules that will be generated.
    ///
    /// Used to initialize the progress bar with the correct total.
    /// Respects the `--include-private` flag when counting.
    #[must_use]
    pub fn count_modules(&self, item: &Item) -> usize {
        let mut count = 0;

        if let ItemEnum::Module(module) = &item.inner {
            for item_id in &module.items {
                if let Some(child) = self.krate.index.get(item_id)
                    && let ItemEnum::Module(_) = &child.inner
                    && self.should_include_item(child)
                {
                    count += 1;
                    count += self.count_modules(child);
                }
            }
        }

        count
    }
}
