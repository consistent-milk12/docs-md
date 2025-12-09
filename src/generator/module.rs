//! Module markdown rendering for documentation generation.
//!
//! This module provides the [`ModuleRenderer`] struct which handles rendering
//! a Rust module's documentation to markdown format, including all its items
//! organized by type.

use std::collections::HashSet;
use std::fmt::Write;

use rustdoc_types::{Id, Item, ItemEnum};

use crate::generator::context::RenderContext;
use crate::generator::items::ItemRenderer;
use crate::generator::quick_ref::{QuickRefEntry, QuickRefGenerator, extract_summary};
use crate::generator::toc::{TocEntry, TocGenerator};
use crate::linker::slugify_anchor;

/// Renders a module to markdown.
///
/// This struct handles the complete rendering of a module's documentation page,
/// including:
/// - Title (Crate or Module heading)
/// - Module-level documentation
/// - Sections for each item type (Modules, Structs, Enums, etc.)
///
/// The renderer is generic over [`RenderContext`], allowing it to work with
/// both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.
pub struct ModuleRenderer<'a> {
    /// Reference to the render context (either single-crate or multi-crate).
    ctx: &'a dyn RenderContext,

    /// Path of the current file being generated (for relative link calculation).
    current_file: &'a str,

    /// Whether this is the crate root module.
    is_root: bool,
}

impl<'a> ModuleRenderer<'a> {
    /// Create a new module renderer.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Render context (implements `RenderContext` trait)
    /// * `current_file` - Path of this file (for relative link calculation)
    /// * `is_root` - True if this is the crate root module
    pub fn new(ctx: &'a dyn RenderContext, current_file: &'a str, is_root: bool) -> Self {
        Self {
            ctx,
            current_file,
            is_root,
        }
    }

    /// Process documentation string to resolve intra-doc links.
    ///
    /// Delegates to the render context's `process_docs` method, which handles
    /// both single-crate and multi-crate link resolution.
    fn process_docs(&self, item: &Item) -> Option<String> {
        self.ctx.process_docs(item, self.current_file)
    }

    /// Generate the complete markdown content for a module.
    ///
    /// # Output Structure
    ///
    /// ```markdown
    /// # Crate `name` (or Module `name`)
    ///
    /// [module documentation]
    ///
    /// ## Contents (if items exceed threshold)
    /// - [Structs](#structs)
    ///   - [`Parser`](#parser)
    ///
    /// ## Modules
    /// - [submodule](link) - first line of docs
    ///
    /// ## Structs
    /// ### `StructName`
    /// [struct definition and docs]
    ///
    /// ## Enums
    /// ...
    /// ```
    #[must_use]
    pub fn render(&self, item: &Item) -> String {
        let mut md = String::new();

        let name = item.name.as_deref().unwrap_or("crate");

        // === Title Section ===
        if self.is_root {
            _ = write!(md, "# Crate `{name}`\n\n");
            if let Some(version) = self.ctx.crate_version() {
                _ = write!(md, "**Version:** {version}\n\n");
            }
        } else {
            _ = write!(md, "# Module `{name}`\n\n");
        }

        // === Documentation Section ===
        if let Some(docs) = self.process_docs(item) {
            md.push_str(&docs);
            md.push_str("\n\n");
        }

        // === Module Contents ===
        if let ItemEnum::Module(module) = &item.inner {
            let categorized = self.categorize_items(&module.items);
            let config = self.ctx.render_config();

            // === Table of Contents (if above threshold) ===
            let toc_gen = TocGenerator::new(config.toc_threshold);
            let toc_entries = Self::build_toc_entries(&categorized);
            if let Some(toc) = toc_gen.generate(&toc_entries) {
                md.push_str(&toc);
            }

            // === Quick Reference (if enabled) ===
            if config.quick_reference {
                let quick_ref_entries = self.build_quick_ref_entries(&categorized);
                if !quick_ref_entries.is_empty() {
                    let quick_ref_gen = QuickRefGenerator::new();
                    md.push_str(&quick_ref_gen.generate(&quick_ref_entries));
                }
            }

            self.render_all_sections(&mut md, &categorized);
        }

        md
    }

    /// Categorize module items by type for organized rendering.
    fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a> {
        let mut items = CategorizedItems::default();
        let mut seen_items: HashSet<&Id> = HashSet::new();

        for item_id in item_ids {
            // Skip if already processed (from glob expansion)
            if !seen_items.insert(item_id) {
                continue;
            }

            if let Some(child) = self.ctx.get_item(item_id) {
                if !self.ctx.should_include_item(child) {
                    continue;
                }

                match &child.inner {
                    ItemEnum::Module(_) => items.modules.push((item_id, child)),
                    ItemEnum::Struct(_) => items.structs.push((item_id, child)),
                    ItemEnum::Enum(_) => items.enums.push((item_id, child)),
                    ItemEnum::Trait(_) => items.traits.push((item_id, child)),
                    ItemEnum::Function(_) => items.functions.push(child),
                    ItemEnum::Macro(_) => items.macros.push(child),
                    ItemEnum::Constant { .. } => items.constants.push(child),
                    ItemEnum::TypeAlias(_) => items.type_aliases.push(child),

                    // Handle re-exports
                    ItemEnum::Use(use_item) => {
                        if use_item.is_glob {
                            // Glob re-export: expand target module's items
                            self.expand_glob_reexport(&mut items, use_item, &mut seen_items);
                        } else if let Some(target_id) = &use_item.id
                            && let Some(target_item) = self.ctx.get_item(target_id)
                        {
                            // Specific re-export: categorize by target type
                            match &target_item.inner {
                                ItemEnum::Module(_) => items.modules.push((item_id, child)),
                                ItemEnum::Struct(_) => items.structs.push((item_id, child)),
                                ItemEnum::Enum(_) => items.enums.push((item_id, child)),
                                ItemEnum::Trait(_) => items.traits.push((item_id, child)),
                                ItemEnum::Function(_) => items.functions.push(child),
                                ItemEnum::Macro(_) => items.macros.push(child),
                                ItemEnum::Constant { .. } => items.constants.push(child),
                                ItemEnum::TypeAlias(_) => items.type_aliases.push(child),
                                _ => {},
                            }
                        }
                    },
                    _ => {},
                }
            }
        }

        // Sort all categories for deterministic output
        items.sort();

        items
    }

    /// Expand a glob re-export by adding all public items from the target module.
    fn expand_glob_reexport(
        &self,
        items: &mut CategorizedItems<'a>,
        use_item: &rustdoc_types::Use,
        seen_items: &mut HashSet<&'a Id>,
    ) {
        // Get target module ID
        let Some(target_id) = &use_item.id else {
            return;
        };

        // Look up target module
        let Some(target_module) = self.ctx.get_item(target_id) else {
            return;
        };

        // Must be a module
        let ItemEnum::Module(module) = &target_module.inner else {
            return;
        };

        // Add each public item from the target module
        for child_id in &module.items {
            // Skip if already seen (handles explicit + glob overlap)
            if !seen_items.insert(child_id) {
                continue;
            }

            let Some(child) = self.ctx.get_item(child_id) else {
                continue;
            };

            // Respect visibility settings
            if !self.ctx.should_include_item(child) {
                continue;
            }

            // Categorize based on item type
            match &child.inner {
                ItemEnum::Module(_) => items.modules.push((child_id, child)),
                ItemEnum::Struct(_) => items.structs.push((child_id, child)),
                ItemEnum::Enum(_) => items.enums.push((child_id, child)),
                ItemEnum::Trait(_) => items.traits.push((child_id, child)),
                ItemEnum::Function(_) => items.functions.push(child),
                ItemEnum::Macro(_) => items.macros.push(child),
                ItemEnum::Constant { .. } => items.constants.push(child),
                ItemEnum::TypeAlias(_) => items.type_aliases.push(child),
                _ => {},
            }
        }
    }

    /// Render all item sections in the standard order.
    fn render_all_sections(&self, md: &mut String, items: &CategorizedItems) {
        self.render_modules_section(md, &items.modules);
        self.render_structs_section(md, &items.structs);
        self.render_enums_section(md, &items.enums);
        self.render_traits_section(md, &items.traits);
        self.render_functions_section(md, &items.functions);
        self.render_macros_section(md, &items.macros);
        self.render_constants_section(md, &items.constants);
        self.render_type_aliases_section(md, &items.type_aliases);
    }

    /// Build TOC entries from categorized items.
    ///
    /// Creates a hierarchical structure for the table of contents, with
    /// section headings as top-level entries and individual items as children.
    fn build_toc_entries(items: &CategorizedItems) -> Vec<TocEntry> {
        let mut entries = Vec::new();

        // Helper to create item entries
        #[expect(
            clippy::items_after_statements,
            reason = "Consider rewriting as a struct"
        )]
        fn item_entries(items: &[(&Id, &Item)]) -> Vec<TocEntry> {
            items
                .iter()
                .filter_map(|(_, item)| {
                    let name = item.name.as_deref()?;
                    Some(TocEntry::new(format!("`{name}`"), slugify_anchor(name)))
                })
                .collect()
        }

        #[expect(
            clippy::items_after_statements,
            reason = "Consider rewriting as a struct"
        )]
        fn simple_item_entries(items: &[&Item]) -> Vec<TocEntry> {
            items
                .iter()
                .filter_map(|item| {
                    let name = item.name.as_deref()?;
                    Some(TocEntry::new(format!("`{name}`"), slugify_anchor(name)))
                })
                .collect()
        }

        // Add sections with their items
        if !items.modules.is_empty() {
            entries.push(TocEntry::with_children(
                "Modules",
                "modules",
                item_entries(&items.modules),
            ));
        }

        if !items.structs.is_empty() {
            entries.push(TocEntry::with_children(
                "Structs",
                "structs",
                item_entries(&items.structs),
            ));
        }

        if !items.enums.is_empty() {
            entries.push(TocEntry::with_children(
                "Enums",
                "enums",
                item_entries(&items.enums),
            ));
        }

        if !items.traits.is_empty() {
            entries.push(TocEntry::with_children(
                "Traits",
                "traits",
                item_entries(&items.traits),
            ));
        }

        if !items.functions.is_empty() {
            entries.push(TocEntry::with_children(
                "Functions",
                "functions",
                simple_item_entries(&items.functions),
            ));
        }

        if !items.macros.is_empty() {
            entries.push(TocEntry::with_children(
                "Macros",
                "macros",
                simple_item_entries(&items.macros),
            ));
        }

        if !items.constants.is_empty() {
            entries.push(TocEntry::with_children(
                "Constants",
                "constants",
                simple_item_entries(&items.constants),
            ));
        }

        if !items.type_aliases.is_empty() {
            entries.push(TocEntry::with_children(
                "Type Aliases",
                "type-aliases",
                simple_item_entries(&items.type_aliases),
            ));
        }

        entries
    }

    /// Build quick reference entries from categorized items.
    ///
    /// Creates a flat list of entries for the quick reference table,
    /// including all item types with their names, kinds, and summaries.
    /// For re-exports, uses the target item's docs when the re-export lacks its own.
    fn build_quick_ref_entries(&self, items: &CategorizedItems) -> Vec<QuickRefEntry> {
        let mut entries = Vec::new();

        // Add entries from items with IDs (supports re-export doc fallback)
        for (id, item) in &items.modules {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "mod",
                    slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        for (id, item) in &items.structs {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "struct",
                    slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        for (id, item) in &items.enums {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "enum",
                    slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        for (id, item) in &items.traits {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "trait",
                    slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        // Simple entries (functions, macros, constants, type aliases)
        // These don't have IDs in categorization, so use direct docs only
        for item in &items.functions {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "fn",
                    slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        for item in &items.macros {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "macro",
                    slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        for item in &items.constants {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "const",
                    slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        for item in &items.type_aliases {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "type",
                    slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        entries
    }

    /// Get summary for an item, with fallback for re-exports.
    ///
    /// For re-exports (`ItemEnum::Use`), if the item has no docs, falls back
    /// to the target item's documentation.
    fn get_item_summary(&self, item: &Item, item_id: Id) -> String {
        // First try the item's own docs
        let summary = extract_summary(item.docs.as_deref());
        if !summary.is_empty() {
            return summary;
        }

        // For re-exports, try to get the target's docs
        if let ItemEnum::Use(use_item) = &item.inner
            && let Some(target_id) = &use_item.id
            && let Some(target_item) = self.ctx.krate().index.get(target_id)
        {
            let target_summary = extract_summary(target_item.docs.as_deref());
            if !target_summary.is_empty() {
                return target_summary;
            }
        }

        // Also check if item_id points to a different item (for non-Use re-exports)
        if let Some(target_item) = self.ctx.krate().index.get(&item_id)
            && target_item.id != item.id
        {
            let target_summary = extract_summary(target_item.docs.as_deref());
            if !target_summary.is_empty() {
                return target_summary;
            }
        }

        String::new()
    }

    /// Render the Modules section with links to submodules.
    fn render_modules_section(&self, md: &mut String, modules: &[(&Id, &Item)]) {
        if modules.is_empty() {
            return;
        }

        md.push_str("## Modules\n\n");
        for (module_id, module_item) in modules {
            let module_name = module_item.name.as_deref().unwrap_or("unnamed");

            if let Some(link) = self.ctx.create_link(**module_id, self.current_file) {
                _ = write!(md, "- {link}");
            } else {
                _ = write!(md, "- **`{module_name}`**");
            }

            // Get summary: try item's own docs, then fall back to target's docs for re-exports
            let summary = self.get_module_summary(module_item, **module_id);
            if !summary.is_empty() {
                _ = write!(md, " — {summary}");
            }
            md.push('\n');
        }
        md.push('\n');
    }

    /// Get summary for a module, with fallback for re-exports.
    fn get_module_summary(&self, item: &Item, item_id: Id) -> String {
        // First try the item's own docs
        if let Some(docs) = &item.docs
            && let Some(first_line) = docs.lines().next()
            && !first_line.trim().is_empty()
        {
            return first_line.to_string();
        }

        // For re-exports, try to get the target's docs
        if let ItemEnum::Use(use_item) = &item.inner
            && let Some(target_id) = &use_item.id
            && let Some(target_item) = self.ctx.krate().index.get(target_id)
            && let Some(docs) = &target_item.docs
            && let Some(first_line) = docs.lines().next()
        {
            return first_line.to_string();
        }

        // Also check if item_id points to a different item (for non-Use re-exports)
        if let Some(target_item) = self.ctx.krate().index.get(&item_id)
            && target_item.id != item.id
            && let Some(docs) = &target_item.docs
            && let Some(first_line) = docs.lines().next()
        {
            return first_line.to_string();
        }

        String::new()
    }

    /// Render the Structs section.
    fn render_structs_section(&self, md: &mut String, structs: &[(&Id, &Item)]) {
        if structs.is_empty() {
            return;
        }

        md.push_str("## Structs\n\n");
        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for (item_id, struct_item) in structs {
            renderer.render_struct(md, **item_id, struct_item);
        }
    }

    /// Render the Enums section.
    fn render_enums_section(&self, md: &mut String, enums: &[(&Id, &Item)]) {
        if enums.is_empty() {
            return;
        }

        md.push_str("## Enums\n\n");
        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for (item_id, enum_item) in enums {
            renderer.render_enum(md, **item_id, enum_item);
        }
    }

    /// Render the Traits section.
    fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)]) {
        if traits.is_empty() {
            return;
        }

        md.push_str("## Traits\n\n");

        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for (item_id, trait_item) in traits {
            renderer.render_trait(md, **item_id, trait_item);
        }
    }

    /// Render the Functions section.
    fn render_functions_section(&self, md: &mut String, functions: &[&Item]) {
        if functions.is_empty() {
            return;
        }

        md.push_str("## Functions\n\n");
        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for func_item in functions {
            renderer.render_function(md, func_item);
        }
    }

    /// Render the Macros section.
    fn render_macros_section(&self, md: &mut String, macros: &[&Item]) {
        if macros.is_empty() {
            return;
        }

        md.push_str("## Macros\n\n");
        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for macro_item in macros {
            renderer.render_macro(md, macro_item);
        }
    }

    /// Render the Constants section.
    fn render_constants_section(&self, md: &mut String, constants: &[&Item]) {
        if constants.is_empty() {
            return;
        }

        md.push_str("## Constants\n\n");
        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for const_item in constants {
            renderer.render_constant(md, const_item);
        }
    }

    /// Render the Type Aliases section.
    fn render_type_aliases_section(&self, md: &mut String, type_aliases: &[&Item]) {
        if type_aliases.is_empty() {
            return;
        }

        md.push_str("## Type Aliases\n\n");
        let renderer = ItemRenderer::new(self.ctx, self.current_file);
        for alias_item in type_aliases {
            renderer.render_type_alias(md, alias_item);
        }
    }
}

/// Items categorized by type for organized rendering.
///
/// Items are sorted into buckets by their type so they can be rendered
/// in consistent sections.
#[derive(Default)]
struct CategorizedItems<'a> {
    /// Child modules (need ID for linking).
    modules: Vec<(&'a Id, &'a Item)>,

    /// Struct definitions (need ID for impl lookup).
    structs: Vec<(&'a Id, &'a Item)>,

    /// Enum definitions (need ID for impl lookup).
    enums: Vec<(&'a Id, &'a Item)>,

    /// Trait definitions (need ID for impl lookup).
    traits: Vec<(&'a Id, &'a Item)>,

    /// Standalone functions.
    functions: Vec<&'a Item>,

    /// Macro definitions.
    macros: Vec<&'a Item>,

    /// Constants and statics.
    constants: Vec<&'a Item>,

    /// Type alias definitions.
    type_aliases: Vec<&'a Item>,
}

impl CategorizedItems<'_> {
    /// Sort all item categories alphabetically by name for deterministic output.
    ///
    /// This ensures consistent ordering regardless of `HashMap` iteration order
    /// in the rustdoc JSON index.
    fn sort(&mut self) {
        // Helper to get item name for sorting
        fn item_name(item: &Item) -> &str {
            item.name.as_deref().unwrap_or("")
        }

        // Sort items with IDs by name
        self.modules
            .sort_by(|a, b| item_name(a.1).cmp(item_name(b.1)));
        self.structs
            .sort_by(|a, b| item_name(a.1).cmp(item_name(b.1)));
        self.enums
            .sort_by(|a, b| item_name(a.1).cmp(item_name(b.1)));
        self.traits
            .sort_by(|a, b| item_name(a.1).cmp(item_name(b.1)));

        // Sort items without IDs by name
        self.functions
            .sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.macros.sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.constants
            .sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.type_aliases
            .sort_by(|a, b| item_name(a).cmp(item_name(b)));
    }
}
