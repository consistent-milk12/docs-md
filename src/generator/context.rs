//! Shared context for documentation generation.
//!
//! This module provides the [`GeneratorContext`] struct which holds all shared
//! state needed during markdown generation, including the crate data, lookup
//! maps, and configuration options.
//!
//! The [`RenderContext`] trait defines the interface that renderers use,
//! enabling both single-crate and multi-crate contexts to share rendering code.

use crate::Args;
use crate::generator::doc_links::{DocLinkProcessor, strip_duplicate_title};
use crate::linker::LinkRegistry;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Visibility};
use std::collections::HashMap;

/// Trait defining the interface for rendering context.
///
/// This trait abstracts over [`GeneratorContext`] (single-crate) and
/// [`SingleCrateView`](crate::multi_crate::SingleCrateView) (multi-crate),
/// allowing renderers like [`ModuleRenderer`](super::module::ModuleRenderer)
/// to work with both contexts.
pub trait RenderContext {
    /// Get the crate being documented.
    fn krate(&self) -> &Crate;

    /// Get the crate name.
    fn crate_name(&self) -> &str;

    /// Get an item by its ID.
    fn get_item(&self, id: &Id) -> Option<&Item>;

    /// Get impl blocks for a type.
    fn get_impls(&self, id: &Id) -> Option<&[&Impl]>;

    /// Check if an item should be included based on visibility.
    fn should_include_item(&self, item: &Item) -> bool;

    /// Whether private items should be included.
    fn include_private(&self) -> bool;

    /// Get the crate version for display in headers.
    fn crate_version(&self) -> Option<&str>;

    /// Get the link registry for single-crate mode.
    ///
    /// Returns `None` in multi-crate mode where `UnifiedLinkRegistry` is used instead.
    fn link_registry(&self) -> Option<&LinkRegistry>;

    /// Process documentation string with intra-doc link resolution.
    ///
    /// Transforms `` [`Type`] `` style links in doc comments into proper
    /// markdown links. Also strips duplicate titles and reference definitions.
    ///
    /// # Arguments
    ///
    /// * `item` - The item whose docs to process (provides docs and links map)
    /// * `current_file` - Path of the current file (for relative link calculation)
    fn process_docs(&self, item: &Item, current_file: &str) -> Option<String>;

    /// Create a markdown link to an item.
    ///
    /// # Arguments
    ///
    /// * `id` - The item ID to link to
    /// * `current_file` - Path of the current file (for relative link calculation)
    ///
    /// # Returns
    ///
    /// A markdown link like `[`Name`](path/to/item.md)`, or `None` if the item
    /// cannot be linked.
    fn create_link(&self, id: Id, current_file: &str) -> Option<String>;
}

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

    /// The crate name (extracted from root module).
    crate_name: String,

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

        // Extract crate name from root module
        let crate_name = krate
            .index
            .get(&krate.root)
            .and_then(|item| item.name.clone())
            .unwrap_or_else(|| "unnamed".to_string());

        let path_map = Self::build_path_map(krate);
        let impl_map = Self::build_impl_map(krate);
        let is_flat = matches!(args.format, CliOutputFormat::Flat);
        let link_registry = LinkRegistry::build(krate, is_flat);

        Self {
            krate,
            crate_name,
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

impl RenderContext for GeneratorContext<'_> {
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
        Some(&self.link_registry)
    }

    fn process_docs(&self, item: &Item, current_file: &str) -> Option<String> {
        let docs = item.docs.as_ref()?;
        let name = item.name.as_deref().unwrap_or("");

        // Strip duplicate title if docs start with "# name"
        let docs = strip_duplicate_title(docs, name);

        let processor = DocLinkProcessor::new(self.krate, &self.link_registry, current_file);
        Some(processor.process(docs, &item.links))
    }

    fn create_link(&self, id: Id, current_file: &str) -> Option<String> {
        self.link_registry.create_link(id, current_file)
    }
}
