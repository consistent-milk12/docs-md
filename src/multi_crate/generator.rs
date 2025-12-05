//! Multi-crate documentation generator.
//!
//! This module provides [`MultiCrateGenerator`] which orchestrates
//! documentation generation across multiple crates with cross-crate linking.

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::path::Path;
use std::sync::Arc;

use fs_err as fs;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rustdoc_types::{Id, Item, ItemEnum, StructKind};

use crate::Args;
use crate::error::Error;
use crate::generator::LinkResolver;
use crate::generator::breadcrumbs::BreadcrumbGenerator;
use crate::generator::impls::is_blanket_impl;
use crate::generator::render_shared::{
    append_docs, impl_sort_key, render_constant_definition, render_enum_definition,
    render_enum_variants_docs, render_function_definition, render_impl_items,
    render_macro_heading, render_struct_definition, render_struct_fields, render_trait_definition,
    render_trait_item, render_type_alias_definition,
};
use crate::multi_crate::context::SingleCrateView;
use crate::multi_crate::search::SearchIndexGenerator;
use crate::multi_crate::summary::SummaryGenerator;
use crate::multi_crate::{CrateCollection, MultiCrateContext};
use crate::types::TypeRenderer;

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
    #[must_use]
    pub fn new(crates: &'a CrateCollection, args: &'a Args) -> Self {
        let ctx = MultiCrateContext::new(crates, args);
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
    pub fn generate(&self) -> Result<(), Error> {
        // Create output directory
        fs::create_dir_all(&self.args.output).map_err(Error::CreateDir)?;

        // Pre-create crate directories to avoid race conditions in parallel generation
        for crate_name in self.ctx.crates().names() {
            let crate_dir = self.args.output.join(crate_name);
            fs::create_dir_all(&crate_dir).map_err(Error::CreateDir)?;
        }

        // Count total modules across all crates for progress bar
        let total_modules: usize = self
            .ctx
            .crates()
            .iter()
            .filter_map(|(name, _)| self.ctx.single_crate_view(name))
            .map(|view| view.count_modules() + 1)
            .sum();

        let progress = Arc::new(Self::create_progress_bar(total_modules)?);

        // Generate crates in parallel
        self.ctx
            .crates()
            .names()
            .par_iter()
            .try_for_each(|crate_name| {
                let view = self
                    .ctx
                    .single_crate_view(crate_name)
                    .ok_or_else(|| Error::ItemNotFound(crate_name.to_string()))?;

                self.generate_crate(&view, &progress)
            })?;

        // Generate SUMMARY.md if requested (sequential - single file)
        if self.args.mdbook {
            progress.set_message("Generating SUMMARY.md...");
            let summary_gen = SummaryGenerator::new(self.ctx.crates(), &self.args.output);
            summary_gen.generate()?;
        }

        // Generate search index if requested (sequential - single file)
        if self.args.search_index {
            progress.set_message("Generating search_index.json...");

            // Collect the IDs of all rendered items to filter the search index
            let rendered_items = self.collect_rendered_items();

            let search_gen =
                SearchIndexGenerator::new(self.ctx.crates(), self.args.include_private, rendered_items);
            search_gen
                .write(&self.args.output)
                .map_err(Error::FileWrite)?;
        }

        progress.finish_with_message("Done!");
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
                result.insert(crate_name.to_string(), ids);
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
                            let target_exists = use_item.id.as_ref().map_or(false, |target_id| {
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
    fn generate_crate(
        &self,
        view: &SingleCrateView,
        progress: &Arc<ProgressBar>,
    ) -> Result<(), Error> {
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
        fs::write(&index_path, content).map_err(Error::FileWrite)?;
        progress.inc(1);

        // Generate submodules
        if let ItemEnum::Module(module) = &root_item.inner {
            for item_id in &module.items {
                if let Some(item) = view.krate().index.get(item_id)
                    && let ItemEnum::Module(_) = &item.inner
                    && view.should_include_item(item)
                {
                    Self::generate_module(view, item, &crate_dir, vec![], Arc::clone(progress))?;
                }
            }
        }

        Ok(())
    }

    /// Generate a module directory with index.md and child modules.
    fn generate_module(
        view: &SingleCrateView,
        item: &Item,
        parent_dir: &Path,
        module_path: Vec<String>,
        progress: Arc<ProgressBar>,
    ) -> Result<(), Error> {
        let name = item.name.as_deref().unwrap_or("unnamed");

        // Create module directory
        let module_dir = parent_dir.join(name);
        fs::create_dir_all(&module_dir).map_err(Error::CreateDir)?;

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
        fs::write(&file_path_on_disk, content).map_err(Error::FileWrite)?;
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
                        Arc::clone(&progress),
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

        let style = ProgressStyle::with_template(
            "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}",
        )
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
    fn new(view: &'a SingleCrateView<'a>, file_path: &'a str, is_root: bool) -> Self {
        Self {
            view,
            file_path,
            is_root,
            type_renderer: TypeRenderer::new(view.krate()),
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
    fn render_module_contents(
        &self,
        md: &mut String,
        module: &rustdoc_types::Module,
        _parent: &Item,
    ) {
        let krate = self.view.krate();

        // Collect items by category (with IDs for impl block rendering)
        let mut modules: Vec<&Item> = Vec::new();
        let mut structs: Vec<(&Id, &Item)> = Vec::new();
        let mut enums: Vec<(&Id, &Item)> = Vec::new();
        let mut traits: Vec<&Item> = Vec::new();
        let mut functions: Vec<&Item> = Vec::new();
        let mut types: Vec<&Item> = Vec::new();
        let mut constants: Vec<&Item> = Vec::new();
        let mut macros: Vec<&Item> = Vec::new();

        for item_id in &module.items {
            if let Some(item) = krate.index.get(item_id) {
                if !self.view.should_include_item(item) {
                    continue;
                }

                match &item.inner {
                    ItemEnum::Module(_) => modules.push(item),

                    ItemEnum::Struct(_) => structs.push((item_id, item)),

                    ItemEnum::Enum(_) => enums.push((item_id, item)),

                    ItemEnum::Trait(_) => traits.push(item),

                    ItemEnum::Function(_) => functions.push(item),

                    ItemEnum::TypeAlias(_) => types.push(item),

                    ItemEnum::Constant { .. } => constants.push(item),

                    ItemEnum::Macro(_) => macros.push(item),

                    // Handle re-exports: pub use other::Item;
                    // Skip glob re-exports (pub use foo::*) as they create duplicates
                    ItemEnum::Use(use_item) if !use_item.is_glob => {
                        // Try to resolve target item: first by ID, then by path
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

                                ItemEnum::Trait(_) => traits.push(item),

                                ItemEnum::Function(_) => functions.push(item),

                                ItemEnum::TypeAlias(_) => types.push(item),

                                ItemEnum::Constant { .. } => constants.push(item),

                                ItemEnum::Macro(_) => macros.push(item),

                                _ => {},
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
                _ = writeln!(md, "{parent_crate} = {{ version = \"*\", features = [\"derive\"] }}");
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
                _ = writeln!(
                    md,
                    "*This crate has no public items to document.*"
                );
            }
            return;
        }

        // Render sections with full detail
        Self::render_modules_section(md, &modules);
        self.render_structs_section(md, &structs);
        self.render_enums_section(md, &enums);
        self.render_traits_section(md, &traits);
        self.render_functions_section(md, &functions);
        self.render_type_aliases_section(md, &types);
        self.render_constants_section(md, &constants);
        self.render_macros_section(md, &macros);
    }

    /// Render modules section (links to subdirectories).
    fn render_modules_section(md: &mut String, modules: &[&Item]) {
        if modules.is_empty() {
            return;
        }

        _ = writeln!(md, "## Modules\n");

        for item in modules {
            let (name, summary) = Self::get_item_name_and_summary(item);
            _ = writeln!(md, "- [`{name}`]({name}/index.md) - {summary}");
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
    fn render_traits_section(&self, md: &mut String, traits: &[&Item]) {
        if traits.is_empty() {
            return;
        }

        _ = writeln!(md, "## Traits\n");

        for item in traits {
            self.render_trait(md, item);
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
    fn get_item_name_and_summary(item: &Item) -> (String, String) {
        if let ItemEnum::Use(use_item) = &item.inner {
            // For re-exports, the name is always from the Use item
            let name = use_item.name.clone();
            let docs = item.docs.as_deref().unwrap_or("");
            let summary = docs.lines().next().unwrap_or("").to_string();
            (name, summary)
        } else {
            let name = item.name.clone().unwrap_or_else(|| "unnamed".to_string());
            let docs = item.docs.as_deref().unwrap_or("");
            let summary = docs.lines().next().unwrap_or("").to_string();
            (name, summary)
        }
    }

    /// Render a struct definition to markdown.
    fn render_struct(&self, md: &mut String, item_id: Id, item: &Item) {
        let krate = self.view.krate();

        // Handle re-exports: use the target item for rendering
        // source_crate is set when this is an external re-export
        let (name, actual_item, actual_id, source_crate): (&str, &Item, Id, Option<&str>) =
            if let ItemEnum::Use(use_item) = &item.inner {
                let name = use_item.name.as_str();

                if let Some(ref target_id) = use_item.id {
                    // Has ID - try local crate first, then search all crates
                    let target = krate.index.get(target_id).or_else(|| {
                        self.view
                            .lookup_item_across_crates(target_id)
                            .map(|(_, item)| item)
                    });

                    if let Some(target) = target {
                        (name, target, *target_id, None)
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
            // Struct definition (heading + code block)
            render_struct_definition(md, name, s, krate, &self.type_renderer);

            // Add re-export annotation for external re-exports (insert after heading)
            if let Some(src_crate) = source_crate {
                // Insert the annotation before the code block
                // The shared function already wrote "### `name`\n\n```rust\n..."
                // We need to insert before the code block - but since the function already
                // wrote the full definition, we'll add it after the code block instead
                _ = writeln!(md, "*Re-exported from `{src_crate}`*\n");
            }

            // Documentation
            append_docs(md, self.view.process_docs(actual_item, self.file_path));

            // Fields documentation
            if let StructKind::Plain { fields, .. } = &s.kind {
                render_struct_fields(md, fields, krate, &self.type_renderer, |field| {
                    self.view.process_docs(field, self.file_path)
                });
            }

            // Impl blocks
            self.render_impl_blocks(md, actual_id);
        }
    }

    /// Render an enum definition to markdown.
    fn render_enum(&self, md: &mut String, item_id: Id, item: &Item) {
        let krate = self.view.krate();

        // Handle re-exports: use the target item for rendering
        // source_crate is set when this is an external re-export
        let (name, actual_item, actual_id, source_crate): (&str, &Item, Id, Option<&str>) =
            if let ItemEnum::Use(use_item) = &item.inner {
                let name = use_item.name.as_str();

                if let Some(ref target_id) = use_item.id {
                    // Has ID - try local crate first, then search all crates
                    let target = krate.index.get(target_id).or_else(|| {
                        self.view
                            .lookup_item_across_crates(target_id)
                            .map(|(_, item)| item)
                    });

                    if let Some(target) = target {
                        (name, target, *target_id, None)
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
            // Enum definition (heading + code block with variants)
            render_enum_definition(md, name, e, krate, &self.type_renderer);

            // Add re-export annotation for external re-exports
            if let Some(src_crate) = source_crate {
                _ = writeln!(md, "*Re-exported from `{src_crate}`*\n");
            }

            // Documentation
            append_docs(md, self.view.process_docs(actual_item, self.file_path));

            // Variants documentation
            render_enum_variants_docs(md, &e.variants, krate, |variant| {
                self.view.process_docs(variant, self.file_path)
            });

            // Impl blocks
            self.render_impl_blocks(md, actual_id);
        }
    }

    /// Render a trait definition to markdown.
    fn render_trait(&self, md: &mut String, item: &Item) {
        let krate = self.view.krate();
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::Trait(t) = &item.inner {
            // Trait definition (heading + code block)
            render_trait_definition(md, name, t, &self.type_renderer);

            // Documentation
            append_docs(md, self.view.process_docs(item, self.file_path));

            // Required methods section
            if !t.items.is_empty() {
                md.push_str("#### Required Methods\n\n");

                for method_id in &t.items {
                    if let Some(method) = krate.index.get(method_id) {
                        render_trait_item(md, method, &self.type_renderer, |m| {
                            self.view.process_docs(m, self.file_path)
                        });
                    }
                }
            }
        }
    }

    /// Render a function definition to markdown.
    fn render_function(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::Function(f) = &item.inner {
            render_function_definition(md, name, f, &self.type_renderer);
        }

        append_docs(md, self.view.process_docs(item, self.file_path));
    }

    /// Render a constant definition to markdown.
    fn render_constant(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::Constant { type_, const_ } = &item.inner {
            render_constant_definition(md, name, type_, const_, &self.type_renderer);
        }

        append_docs(md, self.view.process_docs(item, self.file_path));
    }

    /// Render a type alias to markdown.
    fn render_type_alias(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::TypeAlias(ta) = &item.inner {
            render_type_alias_definition(md, name, ta, &self.type_renderer);
        }

        append_docs(md, self.view.process_docs(item, self.file_path));
    }

    /// Render a macro to markdown.
    fn render_macro(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");
        render_macro_heading(md, name);
        append_docs(md, self.view.process_docs(item, self.file_path));
    }

    /// Render impl blocks for a type, including cross-crate impls.
    fn render_impl_blocks(&self, md: &mut String, item_id: Id) {
        // Get all impls including cross-crate ones
        let impls = self.view.get_all_impls(item_id);

        if impls.is_empty() {
            return;
        }

        let krate = self.view.krate();

        // Partition into inherent vs trait impls
        let (inherent, trait_impls): (Vec<&rustdoc_types::Impl>, Vec<&rustdoc_types::Impl>) =
            impls.into_iter().partition(|i| i.trait_.is_none());

        // Filter out synthetic impls and blanket impls (From, Into, Any, etc.)
        let inherent: Vec<_> = inherent.into_iter().filter(|i| !i.is_synthetic).collect();
        let mut trait_impls: Vec<_> = trait_impls
            .into_iter()
            .filter(|i| !i.is_synthetic && !is_blanket_impl(i))
            .collect();

        // Sort trait impls by trait name + generics for deterministic output
        trait_impls.sort_by(|a, b| {
            let key_a = impl_sort_key(a, &self.type_renderer);
            let key_b = impl_sort_key(b, &self.type_renderer);
            key_a.cmp(&key_b)
        });

        // Deduplicate trait impls with same key (can happen with cross-crate impls)
        trait_impls.dedup_by(|a, b| {
            impl_sort_key(a, &self.type_renderer) == impl_sort_key(b, &self.type_renderer)
        });

        // Render inherent implementations
        if !inherent.is_empty() {
            _ = write!(md, "#### Implementations\n\n");

            for impl_block in inherent {
                render_impl_items(
                    md,
                    impl_block,
                    krate,
                    &self.type_renderer,
                    None::<fn(&Item) -> Option<String>>,
                    Some(|id: rustdoc_types::Id| {
                        LinkResolver::create_link(self.view, id, self.file_path)
                    }),
                );
            }
        }

        // Render trait implementations
        if !trait_impls.is_empty() {
            _ = write!(md, "#### Trait Implementations\n\n");

            for impl_block in trait_impls {
                if let Some(trait_path) = &impl_block.trait_ {
                    let trait_name = trait_path
                        .path
                        .split("::")
                        .last()
                        .unwrap_or(&trait_path.path);
                    let generics = self.type_renderer.render_generics(&impl_block.generics.params);

                    _ = writeln!(md, "##### `impl {trait_name}{generics}`\n");
                }

                render_impl_items(
                    md,
                    impl_block,
                    krate,
                    &self.type_renderer,
                    None::<fn(&Item) -> Option<String>>,
                    Some(|id: rustdoc_types::Id| {
                        LinkResolver::create_link(self.view, id, self.file_path)
                    }),
                );
            }
        }
    }
}
