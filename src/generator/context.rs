//! Shared context for documentation generation.
//!
//! This module provides the [`GeneratorContext`] struct which holds all shared
//! state needed during markdown generation, including the crate data, lookup
//! maps, and configuration options.
//!
//! # Trait Hierarchy
//!
//! The rendering context is split into focused traits for better abstraction:
//!
//! - [`ItemAccess`] - Core data access (crate, items, impls)
//! - [`ItemFilter`] - Visibility and filtering logic
//! - [`LinkResolver`] - Link creation and documentation processing
//! - [`RenderContext`] - Combined super-trait for convenience
//!
//! This allows components to depend only on the traits they need, improving
//! testability and reducing coupling.

use std::collections::HashMap;
use std::path::Path;

use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Visibility};

use crate::Args;
use crate::generator::config::RenderConfig;
use crate::generator::doc_links::{DocLinkProcessor, strip_duplicate_title};
use crate::generator::render_shared::SourcePathConfig;
use crate::linker::LinkRegistry;

// =============================================================================
// Focused Traits
// =============================================================================

/// Core data access for crate documentation.
///
/// Provides read-only access to the crate structure, items, and impl blocks.
pub trait ItemAccess {
    /// Get the crate being documented.
    fn krate(&self) -> &Crate;

    /// Get the crate name.
    fn crate_name(&self) -> &str;

    /// Get an item by its ID.
    fn get_item(&self, id: &Id) -> Option<&Item>;

    /// Get impl blocks for a type.
    fn get_impls(&self, id: &Id) -> Option<&[&Impl]>;

    /// Get the crate version for display in headers.
    fn crate_version(&self) -> Option<&str>;

    /// Get the rendering configuration.
    fn render_config(&self) -> &RenderConfig;

    /// Get source path config for a specific file.
    ///
    /// Returns `None` if source locations are disabled or no source dir configured.
    /// The returned config has the correct depth for the given file path.
    fn source_path_config_for_file(&self, _current_file: &str) -> Option<SourcePathConfig> {
        None
    }
}

/// Item visibility and filtering logic.
///
/// Determines which items should be included in the generated documentation.
pub trait ItemFilter {
    /// Check if an item should be included based on visibility.
    fn should_include_item(&self, item: &Item) -> bool;

    /// Whether private items should be included.
    fn include_private(&self) -> bool;

    /// Whether blanket trait implementations should be included.
    ///
    /// When `false` (default), impls like `From`, `Into`, `Any`, `Borrow` are filtered.
    fn include_blanket_impls(&self) -> bool;
}

/// Link creation and documentation processing.
///
/// Handles intra-doc link resolution and markdown link generation.
pub trait LinkResolver {
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

// =============================================================================
// Combined Trait
// =============================================================================

/// Combined rendering context trait.
///
/// This trait combines [`ItemAccess`], [`ItemFilter`], and [`LinkResolver`]
/// for components that need full access to the rendering context.
///
/// Most renderers should use this trait for convenience, but components
/// with limited requirements can depend on individual sub-traits.
pub trait RenderContext: ItemAccess + ItemFilter + LinkResolver {}

/// Shared context containing all data needed for documentation generation.
///
/// This struct is passed to all rendering components and provides:
/// - Access to the parsed crate data
/// - Impl block lookup for rendering implementations
/// - Link registry for cross-references
/// - CLI configuration options
pub struct GeneratorContext<'a> {
    /// The parsed rustdoc JSON crate.
    pub krate: &'a Crate,

    /// The crate name (extracted from root module).
    crate_name: String,

    /// Maps type IDs to all impl blocks for that type.
    ///
    /// Used for rendering "Implementations" and "Trait Implementations" sections.
    pub impl_map: HashMap<Id, Vec<&'a Impl>>,

    /// Registry for creating cross-reference links between items.
    pub link_registry: LinkRegistry,

    /// CLI arguments containing output path, format, and options.
    pub args: &'a Args,

    /// Rendering configuration options.
    pub config: RenderConfig,

    /// Pre-built index mapping item names to their IDs for fast lookup.
    ///
    /// Built once at construction time from `krate.paths` and shared across
    /// all `DocLinkProcessor` instances for efficiency.
    path_name_index: HashMap<&'a str, Vec<Id>>,

    /// Base source path configuration for transforming cargo registry paths.
    ///
    /// `None` if source locations are disabled or no `.source_*` dir detected.
    /// The `depth` field is set to 0; use `source_path_config_for_file()` to
    /// get a config with the correct depth for a specific file.
    source_path_config: Option<SourcePathConfig>,
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
    /// * `config` - Rendering configuration options
    #[must_use]
    pub fn new(krate: &'a Crate, args: &'a Args, config: RenderConfig) -> Self {
        use crate::CliOutputFormat;

        // Extract crate name from root module
        let crate_name = krate
            .index
            .get(&krate.root)
            .and_then(|item| item.name.clone())
            .unwrap_or_else(|| "unnamed".to_string());

        let impl_map = Self::build_impl_map(krate);
        let is_flat = matches!(args.format, CliOutputFormat::Flat);
        let link_registry = LinkRegistry::build(krate, is_flat, !args.exclude_private);
        let path_name_index = Self::build_path_name_index(krate);

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

        Self {
            krate,
            crate_name,
            impl_map,
            link_registry,
            args,
            config,
            path_name_index,
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

    /// Build a map from type ID to all impl blocks for that type.
    ///
    /// This enables rendering the "Implementations" and "Trait Implementations"
    /// sections for structs, enums, and other types.
    ///
    /// Uses the `impls` field on Struct/Enum/Union items directly rather than
    /// scanning all items and checking the `for_` field. This provides clearer
    /// semantics and leverages `rustdoc_types` structured data.
    fn build_impl_map(krate: &'a Crate) -> HashMap<Id, Vec<&'a Impl>> {
        let mut map: HashMap<Id, Vec<&'a Impl>> = HashMap::new();

        // Iterate over all types that can have impl blocks and collect their impls
        for (type_id, item) in &krate.index {
            let impl_ids: &[Id] = match &item.inner {
                ItemEnum::Struct(s) => &s.impls,
                ItemEnum::Enum(e) => &e.impls,
                ItemEnum::Union(u) => &u.impls,
                _ => continue,
            };

            // Look up each impl block and add to the map
            for impl_id in impl_ids {
                if let Some(impl_item) = krate.index.get(impl_id)
                    && let ItemEnum::Impl(impl_block) = &impl_item.inner
                {
                    map.entry(*type_id).or_default().push(impl_block);
                }
            }
        }

        // Sort impl blocks within each type for deterministic output
        for impls in map.values_mut() {
            impls.sort_by(|a, b| Self::impl_sort_key(a).cmp(&Self::impl_sort_key(b)));
        }

        map
    }

    /// Generate a sort key for an impl block.
    ///
    /// Inherent impls (no trait) sort before trait impls.
    /// Trait impls are sorted by trait name.
    fn impl_sort_key(impl_block: &Impl) -> (u8, String) {
        impl_block
            .trait_
            .as_ref()
            .map_or_else(|| (0, String::new()), |path| (1, path.path.clone()))
    }

    /// Check if an item should be included based on visibility settings.
    ///
    /// By default, all items are included. If `--exclude-private`
    /// is set, only public items are included.
    ///
    /// # Visibility Levels
    ///
    /// - `Public` - Always included
    /// - `Crate`, `Restricted`, `Default` - Included by default, excluded with `--exclude-private`
    #[must_use]
    pub const fn should_include_item(&self, item: &Item) -> bool {
        match &item.visibility {
            Visibility::Public => true,
            _ => !self.args.exclude_private,
        }
    }

    /// Count the total number of modules that will be generated.
    ///
    /// Used to initialize the progress bar with the correct total.
    /// Respects the `--exclude-private` flag when counting.
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

    /// Build an index mapping item names to their IDs for fast lookup.
    ///
    /// This index is built once at context construction time and shared
    /// across all `DocLinkProcessor` instances, eliminating redundant
    /// index building for each item with documentation.
    fn build_path_name_index(krate: &'a Crate) -> HashMap<&'a str, Vec<Id>> {
        let mut index: HashMap<&'a str, Vec<Id>> = HashMap::new();

        for (id, path_info) in &krate.paths {
            if let Some(name) = path_info.path.last() {
                index.entry(name.as_str()).or_default().push(*id);
            }
        }

        // Sort each Vec by full path for deterministic resolution order
        // Using direct Vec<String> comparison (lexicographic) instead of joining
        for ids in index.values_mut() {
            ids.sort_by(|a, b| {
                let path_a = krate.paths.get(a).map(|p| &p.path);
                let path_b = krate.paths.get(b).map(|p| &p.path);
                path_a.cmp(&path_b)
            });
        }

        index
    }
}

impl ItemAccess for GeneratorContext<'_> {
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

    fn crate_version(&self) -> Option<&str> {
        self.krate.crate_version.as_deref()
    }

    fn render_config(&self) -> &RenderConfig {
        &self.config
    }

    fn source_path_config_for_file(&self, current_file: &str) -> Option<SourcePathConfig> {
        self.source_path_config
            .as_ref()
            .map(|base| base.with_depth(current_file))
    }
}

impl ItemFilter for GeneratorContext<'_> {
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

impl LinkResolver for GeneratorContext<'_> {
    fn link_registry(&self) -> Option<&LinkRegistry> {
        Some(&self.link_registry)
    }

    fn process_docs(&self, item: &Item, current_file: &str) -> Option<String> {
        let docs = item.docs.as_ref()?;
        let name = item.name.as_deref().unwrap_or("");

        // Strip duplicate title if docs start with "# name"
        let docs = strip_duplicate_title(docs, name);

        // Use pre-built index for efficiency (avoids rebuilding for each item)
        let processor = DocLinkProcessor::with_index(
            self.krate,
            &self.link_registry,
            current_file,
            &self.path_name_index,
        );
        Some(processor.process(docs, &item.links))
    }

    fn create_link(&self, id: Id, current_file: &str) -> Option<String> {
        self.link_registry.create_link(id, current_file)
    }
}

// Blanket implementation: any type that implements all three sub-traits
// automatically implements RenderContext
impl<T: ItemAccess + ItemFilter + LinkResolver> RenderContext for T {}
