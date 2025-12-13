//! Multi-crate documentation generator.
//!
//! This module provides [`MultiCrateGenerator`] which orchestrates
//! documentation generation across multiple crates with cross-crate linking.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;

use fs_err as FsErr;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, StructKind};
use tracing::{debug, info, info_span, instrument};

use crate::error::Error;
use crate::generator::breadcrumbs::BreadcrumbGenerator;
use crate::generator::config::RenderConfig;
use crate::generator::impls::ImplUtils;
use crate::generator::quick_ref::{QuickRefEntry, QuickRefGenerator, extract_summary};
use crate::generator::render_shared::{CategorizedTraitItems, RendererInternals, TraitRenderer};
use crate::generator::toc::{TocEntry, TocGenerator};
use crate::generator::{ItemAccess, ItemFilter, LinkResolver};
use crate::multi_crate::context::SingleCrateView;
use crate::multi_crate::search::SearchIndexGenerator;
use crate::multi_crate::summary::SummaryGenerator;
use crate::multi_crate::{CrateCollection, MultiCrateContext};
use crate::types::TypeRenderer;
use crate::linker::ImplContext;
use crate::utils::PathUtils;
use crate::{AnchorUtils, Args};

// Note: CategorizedItems for multi-crate rendering is defined in generator/module.rs
// and is used there for the single-crate rendering path. The struct below was written
// for potential future use in multi-crate rendering but is not yet integrated.

/// Categorized module items for rendering.
///
/// Collects items by category during module traversal, eliminating the need
/// for 8 separate vector parameters in TOC/QuickRef generation.
///
/// Note: This is currently unused - kept for potential future multi-crate
/// rendering improvements.
#[allow(dead_code)]
struct CategorizedItems<'a> {
    modules: Vec<&'a Item>,
    structs: Vec<(&'a Id, &'a Item)>,
    enums: Vec<(&'a Id, &'a Item)>,
    unions: Vec<(&'a Id, &'a Item)>,
    traits: Vec<(&'a Id, &'a Item)>,
    functions: Vec<&'a Item>,
    types: Vec<&'a Item>,
    constants: Vec<&'a Item>,
    statics: Vec<&'a Item>,
    macros: Vec<&'a Item>,
}

#[allow(dead_code)]
impl<'a> CategorizedItems<'a> {
    /// Create empty categorized items collection.
    const fn new() -> Self {
        Self {
            modules: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            unions: Vec::new(),
            traits: Vec::new(),
            functions: Vec::new(),
            types: Vec::new(),
            constants: Vec::new(),
            statics: Vec::new(),
            macros: Vec::new(),
        }
    }

    /// Check if all categories are empty.
    const fn is_empty(&self) -> bool {
        self.modules.is_empty()
            && self.structs.is_empty()
            && self.enums.is_empty()
            && self.unions.is_empty()
            && self.traits.is_empty()
            && self.functions.is_empty()
            && self.types.is_empty()
            && self.constants.is_empty()
            && self.statics.is_empty()
            && self.macros.is_empty()
    }

    /// Add an item to the appropriate category based on its type.
    fn add_item(&mut self, id: &'a Id, item: &'a Item) {
        match &item.inner {
            ItemEnum::Module(_) => self.modules.push(item),
            ItemEnum::Struct(_) => self.structs.push((id, item)),
            ItemEnum::Enum(_) => self.enums.push((id, item)),
            ItemEnum::Union(_) => self.unions.push((id, item)),
            ItemEnum::Trait(_) => self.traits.push((id, item)),
            ItemEnum::Function(_) => self.functions.push(item),
            ItemEnum::TypeAlias(_) => self.types.push(item),
            ItemEnum::Constant { .. } => self.constants.push(item),
            ItemEnum::Static(_) => self.statics.push(item),
            ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => self.macros.push(item),
            _ => {},
        }
    }

    /// Add an item by category based on target item type (for re-exports).
    ///
    /// The `id` is the Use item's ID, and `target` is the resolved target item
    /// used to determine the category. The `use_item` is stored (not target)
    /// because `get_item_name` handles Use items specially.
    fn add_reexport(&mut self, id: &'a Id, use_item: &'a Item, target: &Item) {
        match &target.inner {
            ItemEnum::Module(_) => self.modules.push(use_item),
            ItemEnum::Struct(_) => self.structs.push((id, use_item)),
            ItemEnum::Enum(_) => self.enums.push((id, use_item)),
            ItemEnum::Union(_) => self.unions.push((id, use_item)),
            ItemEnum::Trait(_) => self.traits.push((id, use_item)),
            ItemEnum::Function(_) => self.functions.push(use_item),
            ItemEnum::TypeAlias(_) => self.types.push(use_item),
            ItemEnum::Constant { .. } => self.constants.push(use_item),
            ItemEnum::Static(_) => self.statics.push(use_item),
            ItemEnum::Macro(_) | ItemEnum::ProcMacro(_) => self.macros.push(use_item),
            _ => {},
        }
    }

    /// Build TOC entries from categorized items.
    ///
    /// Preserves the standard rustdoc section order:
    /// Modules → Structs → Enums → Unions → Traits → Functions → Type Aliases → Constants → Statics → Macros
    fn build_toc_entries(&self) -> Vec<TocEntry> {
        let mut entries = Vec::new();

        // Build sections in standard rustdoc order
        // Note: We use direct method calls instead of an array literal because
        // Rust cannot coerce &Vec<T> to &[T] inside array initializers.

        // Modules (simple items)
        if let Some(e) = Self::build_section(&self.modules, "Modules", "modules", false) {
            entries.push(e);
        }

        // Structs (items with IDs)
        if let Some(e) = Self::build_section_with_ids(&self.structs, "Structs", "structs") {
            entries.push(e);
        }

        // Enums (items with IDs)
        if let Some(e) = Self::build_section_with_ids(&self.enums, "Enums", "enums") {
            entries.push(e);
        }

        // Unions (items with IDs)
        if let Some(e) = Self::build_section_with_ids(&self.unions, "Unions", "unions") {
            entries.push(e);
        }

        // Traits (items with IDs)
        if let Some(e) = Self::build_section_with_ids(&self.traits, "Traits", "traits") {
            entries.push(e);
        }

        // Functions (simple items)
        if let Some(e) = Self::build_section(&self.functions, "Functions", "functions", false) {
            entries.push(e);
        }

        // Type Aliases (simple items)
        if let Some(e) = Self::build_section(&self.types, "Type Aliases", "type-aliases", false) {
            entries.push(e);
        }

        // Constants (simple items)
        if let Some(e) = Self::build_section(&self.constants, "Constants", "constants", false) {
            entries.push(e);
        }

        // Statics (simple items)
        if let Some(e) = Self::build_section(&self.statics, "Statics", "statics", false) {
            entries.push(e);
        }

        // Macros (simple items, with ! suffix)
        if let Some(e) = Self::build_section(&self.macros, "Macros", "macros", true) {
            entries.push(e);
        }

        entries
    }

    /// Build a TOC section for items without IDs.
    ///
    /// Uses `slugify_anchor()` for anchor generation to match heading anchors.
    fn build_section(
        items: &[&Item],
        section: &str,
        anchor: &str,
        is_macro: bool,
    ) -> Option<TocEntry> {
        if items.is_empty() {
            return None;
        }

        let children: Vec<TocEntry> = items
            .iter()
            .map(|item| {
                let name = Self::get_item_name(item);
                let display = if is_macro {
                    format!("`{name}!`")
                } else {
                    format!("`{name}`")
                };

                // Use slugify_anchor for consistent anchor generation
                // e.g., "my_type" → "my-type" to match heading anchors
                TocEntry::new(display, AnchorUtils::slugify_anchor(name))
            })
            .collect();

        Some(TocEntry::with_children(section, anchor, children))
    }

    /// Build a TOC section for items with IDs.
    ///
    /// Uses `slugify_anchor()` for anchor generation to match heading anchors.
    fn build_section_with_ids(
        items: &[(&Id, &Item)],
        section: &str,
        anchor: &str,
    ) -> Option<TocEntry> {
        if items.is_empty() {
            return None;
        }

        let children: Vec<TocEntry> = items
            .iter()
            .map(|(_, item)| {
                let name = Self::get_item_name(item);
                // Use slugify_anchor for consistent anchor generation
                TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
            })
            .collect();

        Some(TocEntry::with_children(section, anchor, children))
    }

    /// Build Quick Reference entries from categorized items.
    ///
    /// Preserves the standard rustdoc section order:
    /// Modules → Structs → Enums → Unions → Traits → Functions → Type Aliases → Constants → Statics → Macros
    fn build_quick_ref_entries(&self) -> Vec<QuickRefEntry> {
        let mut entries = Vec::new();

        // Add entries in standard rustdoc order
        Self::add_quick_ref_entries(&mut entries, &self.modules, "mod", false);
        Self::add_quick_ref_entries_with_ids(&mut entries, &self.structs, "struct");
        Self::add_quick_ref_entries_with_ids(&mut entries, &self.enums, "enum");
        Self::add_quick_ref_entries_with_ids(&mut entries, &self.unions, "union");
        Self::add_quick_ref_entries_with_ids(&mut entries, &self.traits, "trait");
        Self::add_quick_ref_entries(&mut entries, &self.functions, "fn", false);
        Self::add_quick_ref_entries(&mut entries, &self.types, "type", false);
        Self::add_quick_ref_entries(&mut entries, &self.constants, "const", false);
        Self::add_quick_ref_entries(&mut entries, &self.statics, "static", false);
        Self::add_quick_ref_entries(&mut entries, &self.macros, "macro", true);

        entries
    }

    /// Add quick ref entries for items without IDs.
    ///
    /// Uses `slugify_anchor()` for anchor generation to match heading anchors.
    fn add_quick_ref_entries(
        entries: &mut Vec<QuickRefEntry>,
        items: &[&Item],
        kind: &'static str,
        is_macro: bool,
    ) {
        for item in items {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());

            let display_name = if is_macro {
                format!("{name}!")
            } else {
                name.to_string()
            };

            // Use slugify_anchor for consistent anchor generation
            entries.push(QuickRefEntry::new(
                display_name,
                kind,
                AnchorUtils::slugify_anchor(name),
                summary,
            ));
        }
    }

    /// Add quick ref entries for items with IDs.
    ///
    /// Uses `slugify_anchor()` for anchor generation to match heading anchors.
    fn add_quick_ref_entries_with_ids(
        entries: &mut Vec<QuickRefEntry>,
        items: &[(&Id, &Item)],
        kind: &'static str,
    ) {
        for (_, item) in items {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());

            // Use slugify_anchor for consistent anchor generation
            entries.push(QuickRefEntry::new(
                name,
                kind,
                AnchorUtils::slugify_anchor(name),
                summary,
            ));
        }
    }

    /// Get the display name for an item, handling re-exports.
    fn get_item_name(item: &Item) -> &str {
        if let ItemEnum::Use(use_item) = &item.inner {
            &use_item.name
        } else {
            item.name.as_deref().unwrap_or("unnamed")
        }
    }

    /// Expand a glob re-export into this collection.
    ///
    /// Iterates through items in the target module and adds them to the
    /// appropriate category vectors. Uses `seen_items` to avoid duplicates.
    ///
    /// # Arguments
    ///
    /// * `use_item` - The glob Use item to expand
    /// * `krate` - The crate containing the target module
    /// * `view` - The single-crate view for visibility filtering
    /// * `seen_items` - Set of already-processed item IDs (mutated)
    fn expand_glob_reexport(
        &mut self,
        use_item: &rustdoc_types::Use,
        krate: &'a Crate,
        view: &SingleCrateView<'_>,
        seen_items: &mut HashSet<Id>,
    ) {
        let Some(target_id) = &use_item.id else {
            return;
        };
        let Some(target_module) = krate.index.get(target_id) else {
            return;
        };
        let ItemEnum::Module(module) = &target_module.inner else {
            return;
        };

        for child_id in &module.items {
            if !seen_items.insert(*child_id) {
                continue; // Already processed
            }

            let Some(child) = krate.index.get(child_id) else {
                continue;
            };

            if !view.should_include_item(child) {
                continue;
            }

            // Add the child item to the appropriate category
            self.add_item(child_id, child);
        }
    }
}

/// Generator for multi-crate documentation.
///
/// Produces a directory structure with one subdirectory per crate,
/// each containing nested markdown files with cross-crate linking.
///
/// # Output Structure
///
/// ```text
/// output/
/// ├── tracing/
/// │   ├── index.md
/// │   └── span/
/// │       └── index.md
/// ├── tracing_core/
/// │   ├── index.md
/// │   └── subscriber/
/// │       └── index.md
/// └── SUMMARY.md        # If --mdbook enabled
/// ```
pub struct MultiCrateGenerator<'a> {
    /// Multi-crate context with unified registry.
    ctx: MultiCrateContext<'a>,

    /// CLI arguments.
    args: &'a Args,
}

impl<'a> MultiCrateGenerator<'a> {
    /// Create a new multi-crate generator.
    ///
    /// # Arguments
    ///
    /// * `crates` - Collection of parsed crates
    /// * `args` - CLI arguments
    /// * `config` - Rendering configuration options
    #[must_use]
    pub fn new(crates: &'a CrateCollection, args: &'a Args, config: RenderConfig) -> Self {
        let ctx = MultiCrateContext::new(crates, args, config);
        Self { ctx, args }
    }

    /// Generate documentation for all crates.
    ///
    /// Creates the output directory structure, generates docs for each crate
    /// in parallel using rayon, and optionally generates SUMMARY.md for
    /// mdBook compatibility.
    ///
    /// # Errors
    ///
    /// Returns an error if any file operation fails.
    #[instrument(skip(self), fields(
        crate_count = self.ctx.crates().names().len(),
        output = %self.args.output.display(),
        mdbook = self.args.mdbook,
        search_index = self.args.search_index
    ))]
    pub fn generate(&self) -> Result<(), Error> {
        info!("Starting multi-crate documentation generation");

        // Create output directory
        FsErr::create_dir_all(&self.args.output).map_err(Error::CreateDir)?;

        debug!(path = %self.args.output.display(), "Created output directory");

        // Pre-create crate directories to avoid race conditions in parallel generation
        for crate_name in self.ctx.crates().names() {
            let crate_dir = self.args.output.join(crate_name);
            FsErr::create_dir_all(&crate_dir).map_err(Error::CreateDir)?;
        }

        // Count total modules across all crates for progress bar
        let total_modules: usize = self
            .ctx
            .crates()
            .iter()
            .filter_map(|(name, _)| self.ctx.single_crate_view(name))
            .map(|view| view.count_modules() + 1)
            .sum();

        debug!(total_modules, "Total modules to generate");
        let progress = Arc::new(Self::create_progress_bar(total_modules)?);

        // Generate crates in parallel
        self.ctx
            .crates()
            .names()
            .par_iter()
            .try_for_each(|crate_name| {
                let span = info_span!("generate_crate", crate_name);
                let _guard = span.enter();

                let view = self
                    .ctx
                    .single_crate_view(crate_name)
                    .ok_or_else(|| Error::ItemNotFound((*crate_name).clone()))?;

                self.generate_crate(&view, &progress)
            })?;

        // Generate SUMMARY.md if requested (sequential - single file)
        if self.args.mdbook {
            info!("Generating SUMMARY.md for mdBook");
            progress.set_message("Generating SUMMARY.md...");
            let summary_gen = SummaryGenerator::new(
                self.ctx.crates(),
                &self.args.output,
                !self.args.exclude_private,
            );
            summary_gen.generate()?;
        }

        // Generate search index if requested (sequential - single file)
        if self.args.search_index {
            info!("Generating search_index.json");
            progress.set_message("Generating search_index.json...");

            // Collect the IDs of all rendered items to filter the search index
            let rendered_items = self.collect_rendered_items();

            let search_gen = SearchIndexGenerator::new(
                self.ctx.crates(),
                !self.args.exclude_private,
                rendered_items,
            );
            search_gen
                .write(&self.args.output)
                .map_err(Error::FileWrite)?;
        }

        progress.finish_with_message("Done!");
        info!("Multi-crate documentation generation complete");
        Ok(())
    }

    /// Collect the IDs of all items that would be rendered.
    ///
    /// This walks the module tree for each crate using the same visibility
    /// rules as rendering, collecting the IDs of items that will have
    /// documentation generated for them.
    fn collect_rendered_items(&self) -> HashMap<String, HashSet<Id>> {
        let mut result = HashMap::new();

        for crate_name in self.ctx.crates().names() {
            if let Some(view) = self.ctx.single_crate_view(crate_name) {
                let mut ids = HashSet::new();
                Self::collect_crate_items(&view, &mut ids);
                result.insert(crate_name.clone(), ids);
            }
        }

        result
    }

    /// Collect rendered item IDs for a single crate.
    fn collect_crate_items(view: &SingleCrateView, ids: &mut HashSet<Id>) {
        let krate = view.krate();

        // Get root item
        let Some(root_item) = krate.index.get(&krate.root) else {
            return;
        };

        // Collect root module items
        Self::collect_module_items(view, root_item, ids);
    }

    /// Recursively collect rendered item IDs from a module.
    fn collect_module_items(view: &SingleCrateView, item: &Item, ids: &mut HashSet<Id>) {
        let krate = view.krate();

        if let ItemEnum::Module(module) = &item.inner {
            for item_id in &module.items {
                if let Some(child) = krate.index.get(item_id) {
                    if !view.should_include_item(child) {
                        continue;
                    }

                    match &child.inner {
                        // Documentable items - add their IDs
                        ItemEnum::Struct(_)
                        | ItemEnum::Enum(_)
                        | ItemEnum::Trait(_)
                        | ItemEnum::Function(_)
                        | ItemEnum::TypeAlias(_)
                        | ItemEnum::Constant { .. }
                        | ItemEnum::Macro(_) => {
                            ids.insert(*item_id);
                        },

                        // Modules - add ID and recurse
                        ItemEnum::Module(_) => {
                            ids.insert(*item_id);
                            Self::collect_module_items(view, child, ids);
                        },

                        // Re-exports - add the Use item ID (not the target)
                        ItemEnum::Use(use_item) if !use_item.is_glob => {
                            // Verify target exists (same logic as rendering)
                            let target_exists =
                                use_item.id.as_ref().is_some_and(|target_id| {
                                    krate.index.contains_key(target_id)
                                        || view.lookup_item_across_crates(target_id).is_some()
                                }) || view.resolve_external_path(&use_item.source).is_some();

                            if target_exists {
                                ids.insert(*item_id);
                            }
                        },

                        _ => {},
                    }
                }
            }
        }
    }

    /// Generate documentation for a single crate.
    #[instrument(skip(self, view, progress), fields(crate_name = %view.crate_name()))]
    fn generate_crate(
        &self,
        view: &SingleCrateView,
        progress: &Arc<ProgressBar>,
    ) -> Result<(), Error> {
        debug!("Starting crate generation");

        let crate_name = view.crate_name();
        let crate_dir = self.args.output.join(crate_name);

        // Crate directory already created in generate() to avoid race conditions

        // Get root item
        let root_item = view
            .krate()
            .index
            .get(&view.krate().root)
            .ok_or_else(|| Error::ItemNotFound(view.krate().root.0.to_string()))?;

        // Generate root index.md
        let file_path = format!("{crate_name}/index.md");
        let renderer = MultiCrateModuleRenderer::new(view, &file_path, true);
        let content = renderer.render(root_item);

        let index_path = crate_dir.join("index.md");
        FsErr::write(&index_path, content).map_err(Error::FileWrite)?;
        progress.inc(1);

        // Generate submodules
        if let ItemEnum::Module(module) = &root_item.inner {
            for item_id in &module.items {
                if let Some(item) = view.krate().index.get(item_id)
                    && let ItemEnum::Module(_) = &item.inner
                    && view.should_include_item(item)
                {
                    Self::generate_module(view, item, &crate_dir, vec![], &Arc::clone(progress))?;
                }
            }
        }

        debug!("Crate generation complete");
        Ok(())
    }

    /// Generate a module directory with index.md and child modules.
    fn generate_module(
        view: &SingleCrateView,
        item: &Item,
        parent_dir: &Path,
        module_path: Vec<String>,
        progress: &Arc<ProgressBar>,
    ) -> Result<(), Error> {
        let name = item.name.as_deref().unwrap_or("unnamed");

        // Create module directory
        let module_dir = parent_dir.join(name);
        FsErr::create_dir_all(&module_dir).map_err(Error::CreateDir)?;

        // Build module path for file and breadcrumbs
        let mut current_path = module_path;
        current_path.push(name.to_string());

        // File path relative to output root (includes crate name)
        let file_path = format!("{}/{}/index.md", view.crate_name(), current_path.join("/"));

        // Generate breadcrumbs
        let breadcrumb_gen = BreadcrumbGenerator::new(&current_path, view.crate_name());
        let breadcrumbs = breadcrumb_gen.generate();

        // Generate module content
        let renderer = MultiCrateModuleRenderer::new(view, &file_path, false);
        let module_content = renderer.render(item);

        // Combine breadcrumbs + content
        let content = format!("{breadcrumbs}{module_content}");

        // Write index.md
        let file_path_on_disk = module_dir.join("index.md");
        FsErr::write(&file_path_on_disk, content).map_err(Error::FileWrite)?;
        progress.inc(1);

        // Recurse into child modules
        if let ItemEnum::Module(module) = &item.inner {
            for sub_id in &module.items {
                if let Some(sub_item) = view.krate().index.get(sub_id)
                    && let ItemEnum::Module(_) = &sub_item.inner
                    && view.should_include_item(sub_item)
                {
                    Self::generate_module(
                        view,
                        sub_item,
                        &module_dir,
                        current_path.clone(),
                        &Arc::clone(progress),
                    )?;
                }
            }
        }

        Ok(())
    }

    /// Create a progress bar.
    ///
    /// # Errors
    ///
    /// Returns an error if the progress bar template is invalid.
    fn create_progress_bar(total: usize) -> Result<ProgressBar, Error> {
        let progress = ProgressBar::new(total as u64);

        let style =
            ProgressStyle::with_template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .map_err(Error::ProgressBarTemplate)?
                .progress_chars("=>-");

        progress.set_style(style);
        Ok(progress)
    }
}

/// Module renderer for multi-crate context.
///
/// Wraps the standard module rendering with multi-crate link resolution.
///
/// This renderer handles special cases that aren't covered by the standard
/// `ModuleRenderer`, particularly re-exports (`pub use`) which need to
/// resolve items across crate boundaries.
struct MultiCrateModuleRenderer<'a> {
    /// Single-crate view for this crate (implements `RenderContext`).
    view: &'a SingleCrateView<'a>,

    /// Current file path for link resolution.
    file_path: &'a str,

    /// Whether this is the crate root.
    is_root: bool,

    /// Cached type renderer to avoid repeated construction.
    type_renderer: TypeRenderer<'a>,
}

impl<'a> MultiCrateModuleRenderer<'a> {
    /// Create a new multi-crate module renderer.
    const fn new(view: &'a SingleCrateView<'a>, file_path: &'a str, is_root: bool) -> Self {
        Self {
            view,
            file_path,
            is_root,
            type_renderer: TypeRenderer::new(view.krate()),
        }
    }

    /// Process documentation for an item.
    ///
    /// Delegates to the view's `process_docs` method which handles:
    /// - Stripping duplicate titles
    /// - Converting doc links to markdown links
    /// - Processing code blocks
    fn process_docs(&self, item: &Item) -> Option<String> {
        self.view.process_docs(item, self.file_path)
    }

    /// Render source location if enabled in config.
    ///
    /// Returns the source location string if `source_locations` is enabled,
    /// otherwise returns an empty string. Uses the source path config to
    /// generate clickable links to the `.source_*` directory when available.
    fn maybe_render_source_location(&self, item: &Item) -> String {
        if self.view.render_config().include_source.source_locations {
            let source_config = self.view.source_path_config_for_file(self.file_path);

            RendererInternals::render_source_location(item.span.as_ref(), source_config.as_ref())
        } else {
            String::new()
        }
    }

    /// Render a module item to markdown.
    fn render(&self, item: &Item) -> String {
        let mut md = String::new();

        // Module title
        let name = item.name.as_deref().unwrap_or("unnamed");
        if self.is_root {
            _ = writeln!(md, "# Crate `{name}`\n");
        } else {
            _ = writeln!(md, "# Module `{name}`\n");
        }

        // Module documentation - use RenderContext trait method
        if let Some(docs) = self.view.process_docs(item, self.file_path) {
            _ = writeln!(md, "{docs}\n");
        }

        // Module contents
        if let ItemEnum::Module(module) = &item.inner {
            self.render_module_contents(&mut md, module, item);
        }

        md
    }

    /// Render module contents (items, types, functions, etc.).
    #[expect(
        clippy::too_many_lines,
        reason = "Inherently complex. Will probably grow more."
    )]
    fn render_module_contents(
        &self,
        md: &mut String,
        module: &rustdoc_types::Module,
        _parent: &Item,
    ) {
        let krate = self.view.krate();
        let mut seen_items: HashSet<Id> = HashSet::new();

        // Collect items by category (with IDs for impl block rendering)
        let mut modules: Vec<&Item> = Vec::new();
        let mut structs: Vec<(&Id, &Item)> = Vec::new();
        let mut enums: Vec<(&Id, &Item)> = Vec::new();
        let mut traits: Vec<(&Id, &Item)> = Vec::new();
        let mut functions: Vec<&Item> = Vec::new();
        let mut types: Vec<&Item> = Vec::new();
        let mut constants: Vec<&Item> = Vec::new();
        let mut macros: Vec<&Item> = Vec::new();

        for item_id in &module.items {
            // Skip if already processed (from glob expansion)
            if !seen_items.insert(*item_id) {
                continue;
            }

            if let Some(item) = krate.index.get(item_id) {
                if !self.view.should_include_item(item) {
                    continue;
                }

                match &item.inner {
                    ItemEnum::Module(_) => modules.push(item),

                    ItemEnum::Struct(_) => structs.push((item_id, item)),

                    ItemEnum::Enum(_) => enums.push((item_id, item)),

                    ItemEnum::Trait(_) => traits.push((item_id, item)),

                    ItemEnum::Function(_) => functions.push(item),

                    ItemEnum::TypeAlias(_) => types.push(item),

                    ItemEnum::Constant { .. } => constants.push(item),

                    ItemEnum::Macro(_) => macros.push(item),

                    // Handle re-exports
                    ItemEnum::Use(use_item) => {
                        if use_item.is_glob {
                            // Glob re-export: expand target module's items
                            self.expand_glob_reexport(
                                &mut modules,
                                &mut structs,
                                &mut enums,
                                &mut traits,
                                &mut functions,
                                &mut types,
                                &mut constants,
                                &mut macros,
                                use_item,
                                &mut seen_items,
                            );
                        } else {
                            // Specific re-export: resolve target and categorize
                            let target_item = use_item.id.as_ref().map_or_else(
                                || {
                                    // No ID (external re-export) - resolve by path
                                    self.view
                                        .resolve_external_path(&use_item.source)
                                        .map(|(_, item, _)| item)
                                },
                                |target_id| {
                                    // Has ID - try local crate first, then search all crates
                                    krate.index.get(target_id).or_else(|| {
                                        self.view
                                            .lookup_item_across_crates(target_id)
                                            .map(|(_, item)| item)
                                    })
                                },
                            );

                            if let Some(target_item) = target_item {
                                match &target_item.inner {
                                    ItemEnum::Module(_) => modules.push(item),

                                    ItemEnum::Struct(_) => structs.push((item_id, item)),

                                    ItemEnum::Enum(_) => enums.push((item_id, item)),

                                    ItemEnum::Trait(_) => traits.push((item_id, item)),

                                    ItemEnum::Function(_) => functions.push(item),

                                    ItemEnum::TypeAlias(_) => types.push(item),

                                    ItemEnum::Constant { .. } => constants.push(item),

                                    ItemEnum::Macro(_) => macros.push(item),

                                    _ => {},
                                }
                            }
                        }
                    },
                    _ => {},
                }
            }
        }

        // Check if crate/module is empty
        let is_empty = modules.is_empty()
            && structs.is_empty()
            && enums.is_empty()
            && traits.is_empty()
            && functions.is_empty()
            && types.is_empty()
            && constants.is_empty()
            && macros.is_empty();

        if is_empty && self.is_root {
            // Empty crate - likely a proc-macro or re-export crate
            let crate_name = self.view.crate_name();
            if crate_name.ends_with("_derive") || crate_name.ends_with("-derive") {
                // Derive macro crate - try to link to parent crate
                let parent_crate = crate_name
                    .strip_suffix("_derive")
                    .or_else(|| crate_name.strip_suffix("-derive"))
                    .unwrap_or(crate_name);

                _ = writeln!(md, "## Overview\n");
                _ = writeln!(
                    md,
                    "This is a **procedural macro crate** that provides derive macros."
                );
                _ = writeln!(md);
                _ = writeln!(
                    md,
                    "The macros from this crate are typically re-exported from the parent crate \
                     [`{parent_crate}`](../{parent_crate}/index.md) for convenience. \
                     You should generally depend on the parent crate rather than this one directly."
                );
                _ = writeln!(md);
                _ = writeln!(md, "### Usage\n");
                _ = writeln!(md, "```toml");
                _ = writeln!(md, "[dependencies]");
                _ = writeln!(
                    md,
                    "{parent_crate} = {{ version = \"*\", features = [\"derive\"] }}"
                );
                _ = writeln!(md, "```");
            } else if crate_name.ends_with("_impl") || crate_name.ends_with("-impl") {
                let parent_crate = crate_name
                    .strip_suffix("_impl")
                    .or_else(|| crate_name.strip_suffix("-impl"))
                    .unwrap_or(crate_name);

                _ = writeln!(md, "## Overview\n");
                _ = writeln!(
                    md,
                    "This is an **implementation detail crate** with no public API."
                );
                _ = writeln!(md);
                _ = writeln!(
                    md,
                    "The functionality from this crate is re-exported through \
                     [`{parent_crate}`](../{parent_crate}/index.md). \
                     You should depend on the parent crate instead."
                );
            } else {
                _ = writeln!(md, "*This crate has no public items to document.*");
            }

            return;
        }

        // === Table of Contents (if above threshold) ===
        let config = self.view.render_config();
        let toc_gen = TocGenerator::new(config.toc_threshold);
        let toc_entries = Self::build_toc_entries(
            &modules, &structs, &enums, &traits, &functions, &types, &constants, &macros,
        );

        if let Some(toc) = toc_gen.generate(&toc_entries) {
            _ = write!(md, "{}", &toc);
        }

        // === Quick Reference (if enabled) ===
        if config.quick_reference {
            let quick_ref_entries = Self::build_quick_ref_entries(
                &modules, &structs, &enums, &traits, &functions, &types, &constants, &macros,
            );

            if !quick_ref_entries.is_empty() {
                let quick_ref_gen = QuickRefGenerator::new();

                _ = write!(md, "{}", &quick_ref_gen.generate(&quick_ref_entries));
            }
        }

        // Render sections with full detail
        Self::render_modules_section(md, &modules, self.view.krate());
        self.render_structs_section(md, &structs);
        self.render_enums_section(md, &enums);
        self.render_traits_section(md, &traits);
        self.render_functions_section(md, &functions);
        self.render_type_aliases_section(md, &types);
        self.render_constants_section(md, &constants);
        self.render_macros_section(md, &macros);
    }

    /// Render modules section (links to subdirectories).
    fn render_modules_section(md: &mut String, modules: &[&Item], krate: &Crate) {
        if modules.is_empty() {
            return;
        }

        _ = writeln!(md, "## Modules\n");

        for item in modules {
            let (name, summary) = Self::get_item_name_and_summary_with_fallback(item, Some(krate));
            if summary.is_empty() {
                _ = writeln!(md, "- [`{name}`]({name}/index.md)");
            } else {
                _ = writeln!(md, "- [`{name}`]({name}/index.md) — {summary}");
            }
        }

        _ = writeln!(md);
    }

    /// Render structs section with full detail.
    fn render_structs_section(&self, md: &mut String, structs: &[(&Id, &Item)]) {
        if structs.is_empty() {
            return;
        }

        _ = writeln!(md, "## Structs\n");

        for (item_id, item) in structs {
            self.render_struct(md, **item_id, item);
        }
    }

    /// Render enums section with full detail.
    fn render_enums_section(&self, md: &mut String, enums: &[(&Id, &Item)]) {
        if enums.is_empty() {
            return;
        }

        _ = writeln!(md, "## Enums\n");

        for (item_id, item) in enums {
            self.render_enum(md, **item_id, item);
        }
    }

    /// Render traits section with full detail.
    fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)]) {
        if traits.is_empty() {
            return;
        }

        _ = writeln!(md, "## Traits\n");

        for (item_id, item) in traits {
            self.render_trait(md, **item_id, item);
        }
    }

    /// Render functions section with full detail.
    fn render_functions_section(&self, md: &mut String, functions: &[&Item]) {
        if functions.is_empty() {
            return;
        }

        _ = writeln!(md, "## Functions\n");

        for item in functions {
            self.render_function(md, item);
        }
    }

    /// Render type aliases section with full detail.
    fn render_type_aliases_section(&self, md: &mut String, types: &[&Item]) {
        if types.is_empty() {
            return;
        }

        _ = writeln!(md, "## Type Aliases\n");

        for item in types {
            self.render_type_alias(md, item);
        }
    }

    /// Render constants section with full detail.
    fn render_constants_section(&self, md: &mut String, constants: &[&Item]) {
        if constants.is_empty() {
            return;
        }

        _ = writeln!(md, "## Constants\n");

        for item in constants {
            self.render_constant(md, item);
        }
    }

    /// Render macros section with full detail.
    fn render_macros_section(&self, md: &mut String, macros: &[&Item]) {
        if macros.is_empty() {
            return;
        }

        _ = writeln!(md, "## Macros\n");

        for item in macros {
            self.render_macro(md, item);
        }
    }

    /// Get name and summary for an item, handling re-exports.
    ///
    /// For re-exports (Use items), if the Use item has no docs, falls back to
    /// the target item's docs when a crate reference is provided.
    #[expect(
        dead_code,
        reason = "Kept for API consistency, use _with_fallback for docs"
    )]
    fn get_item_name_and_summary(item: &Item) -> (String, String) {
        Self::get_item_name_and_summary_with_fallback(item, None)
    }

    /// Get name and summary for an item with optional fallback lookup.
    ///
    /// When `krate` is provided and the item is a re-export without docs,
    /// looks up the target item's docs as a fallback.
    fn get_item_name_and_summary_with_fallback(
        item: &Item,
        krate: Option<&Crate>,
    ) -> (String, String) {
        if let ItemEnum::Use(use_item) = &item.inner {
            // For re-exports, the name is always from the Use item
            let name = use_item.name.clone();

            // First try the Use item's own docs
            let summary = if let Some(docs) = &item.docs
                && !docs.trim().is_empty()
            {
                docs.lines().next().unwrap_or("").to_string()
            } else if let Some(krate) = krate
                && let Some(target_id) = &use_item.id
                && let Some(target_item) = krate.index.get(target_id)
                && let Some(target_docs) = &target_item.docs
            {
                // Fall back to target item's docs
                target_docs.lines().next().unwrap_or("").to_string()
            } else {
                String::new()
            };

            (name, summary)
        } else {
            let name = item.name.clone().unwrap_or_else(|| "unnamed".to_string());
            let docs = item.docs.as_deref().unwrap_or("");
            let summary = docs.lines().next().unwrap_or("").to_string();

            (name, summary)
        }
    }

    /// Get the display name for an item, handling re-exports.
    ///
    /// For `Use` items (re-exports), the name is stored in `use_item.name`.
    /// For all other items, the name is in `item.name`.
    fn get_item_name(item: &Item) -> &str {
        if let ItemEnum::Use(use_item) = &item.inner {
            &use_item.name
        } else {
            item.name.as_deref().unwrap_or("unnamed")
        }
    }

    /// Build TOC entries from categorized module items.
    ///
    /// Uses `slugify_anchor()` for anchor generation to match heading anchors.
    #[expect(
        clippy::too_many_arguments,
        reason = "Matches categorization structure"
    )]
    fn build_toc_entries(
        modules: &[&Item],
        structs: &[(&Id, &Item)],
        enums: &[(&Id, &Item)],
        traits: &[(&Id, &Item)],
        functions: &[&Item],
        types: &[&Item],
        constants: &[&Item],
        macros: &[&Item],
    ) -> Vec<TocEntry> {
        let mut entries = Vec::new();

        // Modules section
        if !modules.is_empty() {
            let children: Vec<TocEntry> = modules
                .iter()
                .map(|item| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children("Modules", "modules", children));
        }

        // Structs section
        if !structs.is_empty() {
            let children: Vec<TocEntry> = structs
                .iter()
                .map(|(_, item)| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();
            entries.push(TocEntry::with_children("Structs", "structs", children));
        }

        // Enums section
        if !enums.is_empty() {
            let children: Vec<TocEntry> = enums
                .iter()
                .map(|(_, item)| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children("Enums", "enums", children));
        }

        // Traits section
        if !traits.is_empty() {
            let children: Vec<TocEntry> = traits
                .iter()
                .map(|(_, item)| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children("Traits", "traits", children));
        }

        // Functions section
        if !functions.is_empty() {
            let children: Vec<TocEntry> = functions
                .iter()
                .map(|item| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children("Functions", "functions", children));
        }

        // Type Aliases section
        if !types.is_empty() {
            let children: Vec<TocEntry> = types
                .iter()
                .map(|item| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children(
                "Type Aliases",
                "type-aliases",
                children,
            ));
        }

        // Constants section
        if !constants.is_empty() {
            let children: Vec<TocEntry> = constants
                .iter()
                .map(|item| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children("Constants", "constants", children));
        }

        // Macros section
        if !macros.is_empty() {
            let children: Vec<TocEntry> = macros
                .iter()
                .map(|item| {
                    let name = Self::get_item_name(item);
                    TocEntry::new(format!("`{name}!`"), AnchorUtils::slugify_anchor(name))
                })
                .collect();

            entries.push(TocEntry::with_children("Macros", "macros", children));
        }

        entries
    }

    /// Build Quick Reference entries from categorized module items.
    ///
    /// Uses `slugify_anchor()` for anchor generation to match heading anchors.
    #[expect(
        clippy::too_many_arguments,
        reason = "Matches categorization structure"
    )]
    fn build_quick_ref_entries(
        modules: &[&Item],
        structs: &[(&Id, &Item)],
        enums: &[(&Id, &Item)],
        traits: &[(&Id, &Item)],
        functions: &[&Item],
        types: &[&Item],
        constants: &[&Item],
        macros: &[&Item],
    ) -> Vec<QuickRefEntry> {
        let mut entries = Vec::new();

        // Modules
        for item in modules {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "mod", anchor, summary));
        }

        // Structs
        for (_, item) in structs {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "struct", anchor, summary));
        }

        // Enums
        for (_, item) in enums {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "enum", anchor, summary));
        }

        // Traits
        for (_, item) in traits {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "trait", anchor, summary));
        }

        // Functions
        for item in functions {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "fn", anchor, summary));
        }

        // Type aliases
        for item in types {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "type", anchor, summary));
        }

        // Constants
        for item in constants {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(name, "const", anchor, summary));
        }

        // Macros
        for item in macros {
            let name = Self::get_item_name(item);
            let summary = extract_summary(item.docs.as_deref());
            let anchor = AnchorUtils::slugify_anchor(name);

            entries.push(QuickRefEntry::new(
                format!("{name}!"),
                "macro",
                anchor,
                summary,
            ));
        }

        entries
    }

    /// Render a struct definition to markdown.
    fn render_struct(&self, md: &mut String, item_id: Id, item: &Item) {
        let current_krate = self.view.krate();

        // Handle re-exports: use the target item for rendering
        // source_crate_name is set when the target is from another crate
        let (name, actual_item, actual_id, source_crate_name): (&str, &Item, Id, Option<&str>) =
            if let ItemEnum::Use(use_item) = &item.inner {
                let name = use_item.name.as_str();

                if let Some(ref target_id) = use_item.id {
                    // Has ID - try local crate first, then search all crates
                    if let Some(target) = current_krate.index.get(target_id) {
                        // Found in local crate
                        (name, target, *target_id, None)
                    } else if let Some((src_crate, target)) =
                        self.view.lookup_item_across_crates(target_id)
                    {
                        // Found in another crate - capture the source crate name
                        let is_external = src_crate != self.view.crate_name();
                        (
                            name,
                            target,
                            *target_id,
                            if is_external { Some(src_crate) } else { None },
                        )
                    } else {
                        return;
                    }
                } else {
                    // No ID - try to resolve by path (external re-export)
                    if let Some((src_crate, target, target_id)) =
                        self.view.resolve_external_path(&use_item.source)
                    {
                        (name, target, target_id, Some(src_crate))
                    } else {
                        return;
                    }
                }
            } else {
                (
                    item.name.as_deref().unwrap_or("unnamed"),
                    item,
                    item_id,
                    None,
                )
            };

        if let ItemEnum::Struct(s) = &actual_item.inner {
            // Get the appropriate crate context for rendering
            // Use source crate for field lookups if this is a cross-crate re-export
            let render_krate = source_crate_name
                .and_then(|name| self.view.get_crate(name))
                .unwrap_or(current_krate);

            // Create TypeRenderer for the appropriate crate
            let type_renderer = if source_crate_name.is_some() {
                TypeRenderer::new(render_krate)
            } else {
                // Use cached renderer for local items
                self.type_renderer
            };

            // Struct definition (heading + code block)
            RendererInternals::render_struct_definition(md, name, s, render_krate, &type_renderer);

            // Source location (if enabled)
            _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

            // Add re-export annotation for external re-exports
            if let Some(src_crate) = source_crate_name {
                _ = writeln!(md, "*Re-exported from `{src_crate}`*\n");
            }

            // Documentation
            RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));

            // Fields documentation - use source crate for field type lookups
            if let StructKind::Plain { fields, .. } = &s.kind {
                RendererInternals::render_struct_fields(
                    md,
                    fields,
                    render_krate,
                    &type_renderer,
                    |field| self.view.process_docs(field, self.file_path),
                );
            }

            // Impl blocks - pass source crate for cross-crate re-exports
            self.render_impl_blocks(md, actual_id, source_crate_name);
        }
    }

    /// Render an enum definition to markdown.
    fn render_enum(&self, md: &mut String, item_id: Id, item: &Item) {
        let current_krate = self.view.krate();

        // Handle re-exports: use the target item for rendering
        // source_crate_name is set when the target is from another crate
        let (name, actual_item, actual_id, source_crate_name): (&str, &Item, Id, Option<&str>) =
            if let ItemEnum::Use(use_item) = &item.inner {
                let name = use_item.name.as_str();

                if let Some(ref target_id) = use_item.id {
                    // Has ID - try local crate first, then search all crates
                    if let Some(target) = current_krate.index.get(target_id) {
                        // Found in local crate
                        (name, target, *target_id, None)
                    } else if let Some((src_crate, target)) =
                        self.view.lookup_item_across_crates(target_id)
                    {
                        // Found in another crate - capture the source crate name
                        let is_external = src_crate != self.view.crate_name();
                        (
                            name,
                            target,
                            *target_id,
                            if is_external { Some(src_crate) } else { None },
                        )
                    } else {
                        return;
                    }
                } else {
                    // No ID - try to resolve by path (external re-export)
                    if let Some((src_crate, target, target_id)) =
                        self.view.resolve_external_path(&use_item.source)
                    {
                        (name, target, target_id, Some(src_crate))
                    } else {
                        return;
                    }
                }
            } else {
                (
                    item.name.as_deref().unwrap_or("unnamed"),
                    item,
                    item_id,
                    None,
                )
            };

        if let ItemEnum::Enum(e) = &actual_item.inner {
            // Get the appropriate crate context for rendering
            // Use source crate for variant lookups if this is a cross-crate re-export
            let render_krate = source_crate_name
                .and_then(|name| self.view.get_crate(name))
                .unwrap_or(current_krate);

            // Create TypeRenderer for the appropriate crate
            let type_renderer = if source_crate_name.is_some() {
                TypeRenderer::new(render_krate)
            } else {
                // Use cached renderer for local items
                self.type_renderer
            };

            // Enum definition (heading + code block with variants)
            RendererInternals::render_enum_definition(md, name, e, render_krate, &type_renderer);

            // Source location (if enabled)
            _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

            // Add re-export annotation for external re-exports
            if let Some(src_crate) = source_crate_name {
                _ = writeln!(md, "*Re-exported from `{src_crate}`*\n");
            }

            // Documentation
            RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));

            // Variants documentation - use source crate for variant type lookups
            RendererInternals::render_enum_variants_docs(
                md,
                &e.variants,
                render_krate,
                |variant| self.view.process_docs(variant, self.file_path),
            );

            // Impl blocks - pass source crate for cross-crate re-exports
            self.render_impl_blocks(md, actual_id, source_crate_name);
        }
    }

    /// Render a trait definition to markdown.
    fn render_trait(&self, md: &mut String, item_id: Id, item: &Item) {
        let current_krate = self.view.krate();

        // Handle re-exports: resolve to actual trait item
        let (name, actual_item, actual_id): (&str, &Item, Id) = if let ItemEnum::Use(use_item) =
            &item.inner
        {
            let name = use_item.name.as_str();

            if let Some(ref target_id) = use_item.id {
                // Has ID - try local crate first, then search all crates
                if let Some(target) = current_krate.index.get(target_id) {
                    (name, target, *target_id)
                } else if let Some((_, target)) = self.view.lookup_item_across_crates(target_id) {
                    (name, target, *target_id)
                } else {
                    return;
                }
            } else {
                // No ID - try to resolve by path (external re-export)
                if let Some((_, target, target_id)) =
                    self.view.resolve_external_path(&use_item.source)
                {
                    (name, target, target_id)
                } else {
                    return;
                }
            }
        } else {
            (item.name.as_deref().unwrap_or("unnamed"), item, item_id)
        };

        if let ItemEnum::Trait(t) = &actual_item.inner {
            // Trait definition (heading + code block)
            TraitRenderer::render_trait_definition(md, name, t, &self.type_renderer);

            // Source location (if enabled)
            _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

            // Documentation
            RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));

            // Categorize trait items
            let items = CategorizedTraitItems::categorize_trait_items(&t.items, current_krate);

            // Associated Types
            if !items.associated_types.is_empty() {
                _ = writeln!(md, "#### Associated Types\n");

                for type_item in &items.associated_types {
                    TraitRenderer::render_trait_item(md, type_item, &self.type_renderer, |m| {
                        self.view.process_docs(m, self.file_path)
                    });
                }
            }

            // Associated Constants
            if !items.associated_consts.is_empty() {
                _ = writeln!(md, "#### Associated Constants\n");

                for const_item in &items.associated_consts {
                    TraitRenderer::render_trait_item(md, const_item, &self.type_renderer, |m| {
                        self.view.process_docs(m, self.file_path)
                    });
                }
            }

            // Required Methods
            if !items.required_methods.is_empty() {
                _ = writeln!(md, "#### Required Methods\n");

                for method in &items.required_methods {
                    TraitRenderer::render_trait_item(md, method, &self.type_renderer, |m| {
                        self.view.process_docs(m, self.file_path)
                    });
                }
            }

            // Provided Methods
            if !items.provided_methods.is_empty() {
                _ = writeln!(md, "#### Provided Methods\n");

                for method in &items.provided_methods {
                    TraitRenderer::render_trait_item(md, method, &self.type_renderer, |m| {
                        self.view.process_docs(m, self.file_path)
                    });
                }
            }

            // Implementors section
            self.render_trait_implementors(md, actual_id);
        }
    }

    /// Render the implementors section for a trait.
    ///
    /// Uses `Trait.implementations` field for direct lookup instead of scanning
    /// all items in the crate index, providing O(k) performance where k is the
    /// number of implementors.
    fn render_trait_implementors(&self, md: &mut String, trait_id: Id) {
        let krate = self.view.krate();

        // Get the trait item to access its implementations list
        let Some(trait_item) = krate.index.get(&trait_id) else {
            return;
        };
        let ItemEnum::Trait(trait_data) = &trait_item.inner else {
            return;
        };

        let mut implementors: BTreeSet<String> = BTreeSet::new();

        // Use Trait.implementations for direct lookup instead of scanning all items
        for impl_id in &trait_data.implementations {
            let Some(impl_item) = krate.index.get(impl_id) else {
                continue;
            };
            let ItemEnum::Impl(impl_block) = &impl_item.inner else {
                continue;
            };

            let for_type = self.type_renderer.render_type(&impl_block.for_);

            let entry = self
                .type_renderer
                .get_type_id(&impl_block.for_)
                .and_then(|type_id| LinkResolver::create_link(self.view, type_id, self.file_path))
                .unwrap_or_else(|| format!("`{for_type}`"));

            implementors.insert(entry);
        }

        if !implementors.is_empty() {
            _ = write!(md, "#### Implementors\n\n");
            for implementor in implementors {
                _ = writeln!(md, "- {implementor}");
            }
            md.push('\n');
        }
    }

    /// Render a function definition to markdown.
    /// FIX: Handle re-exports: Resolve to actual function item.
    fn render_function(&self, md: &mut String, item: &Item) {
        let Some((name, actual_item)) = self.resolve_reexport(item) else {
            return;
        };

        let ItemEnum::Function(f) = &actual_item.inner else {
            return;
        };

        RendererInternals::render_function_definition(md, name, f, &self.type_renderer);
        _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

        RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));
    }

    /// Render a constant definition to markdown.
    /// Handles re-exports by resolving to the actual constant item.
    fn render_constant(&self, md: &mut String, item: &Item) {
        let Some((name, actual_item)) = self.resolve_reexport(item) else {
            return;
        };

        let ItemEnum::Constant { type_, const_ } = &actual_item.inner else {
            return; // Not a constant after resolution
        };

        RendererInternals::render_constant_definition(md, name, type_, const_, &self.type_renderer);
        _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

        RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));
    }

    /// Render a type alias to markdown.
    /// Handles re-exports by resolving to the actual type alias item.
    fn render_type_alias(&self, md: &mut String, item: &Item) {
        let Some((name, actual_item)) = self.resolve_reexport(item) else {
            return;
        };

        let ItemEnum::TypeAlias(ta) = &actual_item.inner else {
            return; // Not a type alias after resolution
        };

        RendererInternals::render_type_alias_definition(md, name, ta, &self.type_renderer);
        _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

        RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));
    }

    /// Render a macro to markdown.
    /// Handles re-exports by resolving to the actual macro item.
    fn render_macro(&self, md: &mut String, item: &Item) {
        let Some((name, actual_item)) = self.resolve_reexport(item) else {
            return;
        };

        // Verify it's actually a macro after resolution
        if !matches!(
            &actual_item.inner,
            ItemEnum::Macro(_) | ItemEnum::ProcMacro(_)
        ) {
            return;
        }

        RendererInternals::render_macro_heading(md, name);
        _ = write!(md, "{}", self.maybe_render_source_location(actual_item));

        RendererInternals::append_docs(md, self.view.process_docs(actual_item, self.file_path));
    }

    fn resolve_reexport<'b>(&'b self, item: &'b Item) -> Option<(&'b str, &'b Item)> {
        let ItemEnum::Use(use_item) = &item.inner else {
            // Not a re-export - return the item itself
            return Some((item.name.as_deref().unwrap_or("unnamed"), item));
        };

        let name = use_item.name.as_str();
        let current_crate = self.view.krate();

        if let Some(ref target_id) = use_item.id {
            // Has ID - try local crate first, then search all crates
            if let Some(target) = current_crate.index.get(target_id) {
                return Some((name, target));
            }

            // Cross crate search if not in local crate
            if let Some((_, target)) = self.view.lookup_item_across_crates(target_id) {
                return Some((name, target));
            }
        } else if let Some((_, target, _)) = self.view.resolve_external_path(&use_item.source) {
            // No ID - try to resolve by path (external re-export)
            return Some((name, target));
        }

        // Cannot resolve
        #[cfg(feature = "trace")]
        tracing::error!(
            "Cannot resolve link. Logging it for investigation: {:?}",
            item.clone()
        );

        None
    }

    /// Expand a glob re-export into the category vectors.
    #[allow(clippy::too_many_arguments)]
    fn expand_glob_reexport<'b>(
        &self,
        modules: &mut Vec<&'b Item>,
        structs: &mut Vec<(&'b Id, &'b Item)>,
        enums: &mut Vec<(&'b Id, &'b Item)>,
        traits: &mut Vec<(&'b Id, &'b Item)>,
        functions: &mut Vec<&'b Item>,
        types: &mut Vec<&'b Item>,
        constants: &mut Vec<&'b Item>,
        macros: &mut Vec<&'b Item>,
        use_item: &rustdoc_types::Use,
        seen_items: &mut HashSet<Id>,
    ) where
        'a: 'b,
    {
        let krate = self.view.krate();

        let Some(target_id) = &use_item.id else {
            return;
        };
        let Some(target_module) = krate.index.get(target_id) else {
            return;
        };
        let ItemEnum::Module(module) = &target_module.inner else {
            return;
        };

        for child_id in &module.items {
            if !seen_items.insert(*child_id) {
                continue; // Already processed
            }

            let Some(child) = krate.index.get(child_id) else {
                continue;
            };

            if !self.view.should_include_item(child) {
                continue;
            }

            match &child.inner {
                ItemEnum::Module(_) => modules.push(child),

                ItemEnum::Struct(_) => structs.push((child_id, child)),

                ItemEnum::Enum(_) => enums.push((child_id, child)),

                ItemEnum::Trait(_) => traits.push((child_id, child)),

                ItemEnum::Function(_) => functions.push(child),

                ItemEnum::TypeAlias(_) => types.push(child),

                ItemEnum::Constant { .. } => constants.push(child),

                ItemEnum::Macro(_) => macros.push(child),
                _ => {},
            }
        }
    }

    /// Render impl blocks for a type, including cross-crate impls.
    ///
    /// # Arguments
    ///
    /// * `md` - The markdown output buffer
    /// * `item_id` - The ID of the item to render impl blocks for
    /// * `source_crate_name` - Optional source crate name for cross-crate re-exports.
    ///   When provided, impls are looked up from the source crate.
    fn render_impl_blocks(&self, md: &mut String, item_id: Id, source_crate_name: Option<&str>) {
        let current_krate = self.view.krate();

        // Get the appropriate crate for impl lookups
        let render_krate = source_crate_name
            .and_then(|name| self.view.get_crate(name))
            .unwrap_or(current_krate);

        // Create TypeRenderer for the appropriate crate
        let type_renderer = if source_crate_name.is_some() {
            TypeRenderer::new(render_krate)
        } else {
            self.type_renderer
        };

        // Get impls - for cross-crate items, look up from source crate
        let impls = if source_crate_name.is_some() {
            // For cross-crate re-exports, get impls from the source crate
            self.view.get_impls_from_crate(item_id, render_krate)
        } else {
            // For local items, use the normal lookup
            self.view.get_all_impls(item_id)
        };

        if impls.is_empty() {
            return;
        }

        // Partition into inherent vs trait impls
        let (inherent, trait_impls): (Vec<&Impl>, Vec<&Impl>) = impls
            .into_iter()
            .partition(|i: &&Impl| -> bool { i.trait_.is_none() });

        // Filter out synthetic impls
        let inherent: Vec<_> = inherent.into_iter().filter(|i| !i.is_synthetic).collect();

        // Filter out synthetic impls and optionally blanket impls
        // Respect the --include-blanket-impls flag
        let include_blanket = self.view.include_blanket_impls();
        let mut trait_impls: Vec<_> = trait_impls
            .into_iter()
            .filter(|i| !i.is_synthetic)
            .filter(|i| include_blanket || !ImplUtils::is_blanket_impl(i))
            .collect();

        // Sort trait impls by trait name + generics for deterministic output
        trait_impls.sort_by(|a, b| {
            let key_a = RendererInternals::impl_sort_key(a, &type_renderer);
            let key_b = RendererInternals::impl_sort_key(b, &type_renderer);
            key_a.cmp(&key_b)
        });

        // Deduplicate trait impls with same key (can happen with cross-crate impls)
        trait_impls.dedup_by(|a, b| {
            RendererInternals::impl_sort_key(a, &type_renderer)
                == RendererInternals::impl_sort_key(b, &type_renderer)
        });

        // Render inherent implementations
        if !inherent.is_empty() {
            _ = write!(md, "#### Implementations\n\n");

            for impl_block in inherent {
                // Extract type name for method anchor generation
                let type_name = type_renderer.render_type(&impl_block.for_);

                RendererInternals::render_impl_items(
                    md,
                    impl_block,
                    render_krate,
                    &type_renderer,
                    &Some(|item: &Item| self.process_docs(item)),
                    &Some(|id: Id| LinkResolver::create_link(self.view, id, self.file_path)),
                    Some(type_name.as_ref()),
                    ImplContext::Inherent,
                    self.view.render_config().full_method_docs,
                );
            }
        }

        // Render trait implementations
        if !trait_impls.is_empty() {
            _ = write!(md, "#### Trait Implementations\n\n");

            for impl_block in trait_impls {
                // Extract type name for method anchor generation
                let for_type = type_renderer.render_type(&impl_block.for_);

                // Determine impl context for anchor generation:
                // - For trait impls, include the trait name to avoid duplicate anchors
                let impl_ctx = impl_block.trait_.as_ref().map_or(
                    ImplContext::Inherent,
                    |t| ImplContext::Trait(PathUtils::short_name(&t.path)),
                );

                if let Some(trait_path) = &impl_block.trait_ {
                    // Build trait name with generic args
                    let trait_name = trait_path
                        .path
                        .split("::")
                        .last()
                        .unwrap_or(&trait_path.path);

                    // Only show generic parameters that appear in the signature (for_ type
                    // or trait path). For blanket impls like `impl<T: Sized> IntoEither for T`,
                    // when rendered for `Either<L, R>`, the `T` only appears in the where clause.
                    // We don't want to show orphaned generics like `impl<T> IntoEither for Either<L, R>`.
                    let signature_generics = ImplUtils::extract_impl_signature_generics(impl_block);
                    let generics = if signature_generics.is_empty() {
                        String::new()
                    } else {
                        // Filter to params that appear in the signature
                        let filtered: Vec<_> = impl_block
                            .generics
                            .params
                            .iter()
                            .filter(|p| signature_generics.contains(&p.name))
                            .cloned()
                            .collect();
                        if filtered.is_empty() {
                            String::new()
                        } else {
                            type_renderer.render_generics(&filtered)
                        }
                    };

                    // Include unsafe/negative markers like single-crate mode
                    let unsafe_str = if impl_block.is_unsafe { "unsafe " } else { "" };
                    let negative_str = if impl_block.is_negative { "!" } else { "" };

                    _ = writeln!(
                        md,
                        "##### `{unsafe_str}impl{generics} {negative_str}{trait_name} for {for_type}`\n"
                    );
                }

                RendererInternals::render_impl_items(
                    md,
                    impl_block,
                    render_krate,
                    &type_renderer,
                    &Some(|item: &Item| self.process_docs(item)),
                    &Some(|id: Id| LinkResolver::create_link(self.view, id, self.file_path)),
                    Some(for_type.as_ref()),
                    impl_ctx,
                    self.view.render_config().full_method_docs,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod get_item_name_tests {
        use rustdoc_types::{Item, ItemEnum, Use, Visibility};

        use super::*;

        /// Create a regular item with the given name.
        fn make_item(name: Option<&str>) -> Item {
            Item {
                id: Id(0),
                crate_id: 0,
                name: name.map(ToString::to_string),
                span: None,
                visibility: Visibility::Public,
                docs: None,
                links: std::collections::HashMap::new(),
                attrs: Vec::new(),
                deprecation: None,
                inner: ItemEnum::Module(rustdoc_types::Module {
                    is_crate: false,
                    items: Vec::new(),
                    is_stripped: false,
                }),
            }
        }

        /// Create a Use item (re-export) with the given name.
        fn make_use_item(use_name: &str) -> Item {
            Item {
                id: Id(0),
                crate_id: 0,
                name: None, // Use items don't have item.name set
                span: None,
                visibility: Visibility::Public,
                docs: None,
                links: std::collections::HashMap::new(),
                attrs: Vec::new(),
                deprecation: None,
                inner: ItemEnum::Use(Use {
                    source: "some::path".to_string(),
                    name: use_name.to_string(),
                    id: None,
                    is_glob: false,
                }),
            }
        }

        #[test]
        fn regular_item_with_name() {
            let item = make_item(Some("MyStruct"));

            assert_eq!(MultiCrateModuleRenderer::get_item_name(&item), "MyStruct");
        }

        #[test]
        fn regular_item_without_name_returns_unnamed() {
            let item = make_item(None);

            assert_eq!(MultiCrateModuleRenderer::get_item_name(&item), "unnamed");
        }

        #[test]
        fn use_item_returns_reexport_name() {
            let item = make_use_item("Parser");

            assert_eq!(MultiCrateModuleRenderer::get_item_name(&item), "Parser");
        }

        #[test]
        fn use_item_ignores_item_name() {
            // Even if item.name were somehow set, we should use use_item.name
            let mut item = make_use_item("CorrectName");
            item.name = Some("WrongName".to_string());

            assert_eq!(
                MultiCrateModuleRenderer::get_item_name(&item),
                "CorrectName"
            );
        }
    }
}
