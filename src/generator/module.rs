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
use crate::linker::AnchorUtils;

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
            _ = write!(md, "{}", &docs);
            _ = write!(md, "\n\n");
        }

        // === Module Contents ===
        if let ItemEnum::Module(module) = &item.inner {
            let categorized = self.categorize_items(&module.items);
            let config = self.ctx.render_config();

            // === Table of Contents (if above threshold) ===
            let toc_gen = TocGenerator::new(config.toc_threshold);
            let toc_entries = Self::build_toc_entries(&categorized);
            if let Some(toc) = toc_gen.generate(&toc_entries) {
                _ = write!(md, "{}", &toc);
            }

            // === Quick Reference (if enabled) ===
            if config.quick_reference {
                let quick_ref_entries = self.build_quick_ref_entries(&categorized);
                if !quick_ref_entries.is_empty() {
                    let quick_ref_gen = QuickRefGenerator::new();
                    _ = write!(md, "{}", &quick_ref_gen.generate(&quick_ref_entries));
                }
            }

            self.render_all_sections(&mut md, &categorized);
        }

        md
    }

    /// Categorize module items by type for organized rendering.
    ///
    /// Items are categorized into groups for structured documentation.
    /// - Modules (for navigation)
    /// - Types (structs, enums, unions, type aliases)
    /// - Traits
    /// - Functions
    /// - Constants and statics
    /// - Macros
    fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a> {
        let mut items = CategorizedItems::default();
        let mut seen_items: HashSet<&Id> = HashSet::new();

        for item_id in item_ids {
            // Skip: if alredy processed (from glob expansion)
            if !seen_items.insert(item_id) {
                continue;
            }

            if let Some(child) = self.ctx.get_item(item_id) {
                // Skip: This item should not be included.
                if !self.ctx.should_include_item(child) {
                    continue;
                }

                match &child.inner {
                    // Navigation
                    ItemEnum::Module(_) => items.modules.push((item_id, child)),

                    // Types section
                    ItemEnum::Struct(_) => items.structs.push((item_id, child)),
                    ItemEnum::Enum(_) => items.enums.push((item_id, child)),
                    ItemEnum::Union(_) => items.unions.push((item_id, child)),
                    ItemEnum::TypeAlias(_) => items.type_aliases.push(child),

                    // Other sections
                    ItemEnum::Trait(_) => items.traits.push((item_id, child)),
                    ItemEnum::Function(_) => items.functions.push(child),
                    ItemEnum::Constant { .. } => items.constants.push(child),
                    ItemEnum::Static(_) => items.statics.push(child),
                    ItemEnum::Macro(_) => items.macros.push(child),

                    // Handle re-exports
                    ItemEnum::Use(use_item) => {
                        // If its a glob re-export
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
                                ItemEnum::Union(_) => items.unions.push((item_id, child)),
                                ItemEnum::TypeAlias(_) => items.type_aliases.push(child),

                                ItemEnum::Trait(_) => items.traits.push((item_id, child)),
                                ItemEnum::Function(_) => items.functions.push(child),
                                ItemEnum::Constant { .. } => items.constants.push(child),
                                ItemEnum::Static(_) => items.statics.push(child),
                                ItemEnum::Macro(_) => items.macros.push(child),
                                _ => {},
                            }
                        }
                    },

                    _ => {},
                }
            }
        }

        // Sort all categories for deterministic output.
        // TODO: See if we can move to an ordered data structure by insertion.
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
                ItemEnum::Union(_) => items.unions.push((child_id, child)),
                ItemEnum::TypeAlias(_) => items.type_aliases.push(child),

                ItemEnum::Trait(_) => items.traits.push((child_id, child)),
                ItemEnum::Function(_) => items.functions.push(child),
                ItemEnum::Constant { .. } => items.constants.push(child),
                ItemEnum::Static(_) => items.statics.push(child),
                ItemEnum::Macro(_) => items.macros.push(child),

                _ => {},
            }
        }
    }

    /// Render all item sections with horizontal rule separators.
    ///
    /// Sections are rendered in this order:
    /// 1. Modules (navigation, no separator before)
    /// 2. Types (structs, enums, unions, type aliases)
    /// 3. Traits
    /// 4. Functions
    /// 5. Constants
    /// 6. Statics
    /// 7. Macros
    ///
    /// Horizontal rules (`---`) are added between major sections for
    /// visual separation in the rendered output.
    fn render_all_sections(&self, md: &mut String, items: &CategorizedItems) {
        // Track if we've rendered any content (for separator logic)
        // === Modules Section (navigation) ===
        // No separator before modules - they come first
        let mut has_content = if items.modules.is_empty() {
            false
        } else {
            self.render_modules_section(md, &items.modules);
            true
        };

        // === Types Section (structs, enums, unions, type aliases) ===
        if items.has_types() {
            if has_content {
                _ = writeln!(md, "\n---\n");
            }

            self.render_types_section(md, items);
            has_content = true;
        }

        // === Traits Section ===
        if !items.traits.is_empty() {
            if has_content {
                _ = writeln!(md, "\n---\n");
            }

            self.render_traits_section(md, &items.traits);
            has_content = true;
        }

        // === Functions Section ===
        if !items.functions.is_empty() {
            if has_content {
                _ = writeln!(md, "\n---\n");
            }

            self.render_functions_section(md, &items.functions);
            has_content = true;
        }

        // === Constants Section ===
        if !items.constants.is_empty() {
            if has_content {
                _ = writeln!(md, "\n---\n");
            }

            self.render_constants_section(md, &items.constants);
            has_content = true;
        }

        // === Statics Section ===
        if !items.statics.is_empty() {
            if has_content {
                _ = writeln!(md, "\n---\n");
            }

            self.render_statics_section(md, &items.statics);
            has_content = true;
        }

        // === Macros Section ===
        if !items.macros.is_empty() {
            if has_content {
                _ = writeln!(md, "\n---\n");
            }

            self.render_macros_section(md, &items.macros);
        }
    }

    /// Render the Types section (structs, enums, unions, type aliases).
    ///
    /// All type definitions are grouped under a single "Types" heading,
    /// with each item type rendered in subsections:
    ///
    /// ```markdown
    /// ## Types
    ///
    /// ### `MyStruct`
    /// [struct definition]
    ///
    /// ### `MyEnum`
    /// [enum definition]
    ///
    /// ### `MyUnion`
    /// [union definition]
    ///
    /// ### `MyAlias`
    /// [type alias definition]
    /// ```
    fn render_types_section(&self, md: &mut String, items: &CategorizedItems) {
        _ = write!(md, "## Types\n\n");

        let renderer = ItemRenderer::new(self.ctx, self.current_file);

        // Render structs
        for (item_id, struct_item) in &items.structs {
            renderer.render_struct(md, **item_id, struct_item);
        }

        // Render enums
        for (item_id, enum_item) in &items.enums {
            renderer.render_enum(md, **item_id, enum_item);
        }

        // Render unions
        for (item_id, union_item) in &items.unions {
            renderer.render_union(md, **item_id, union_item);
        }

        // Render type aliases
        for alias_item in &items.type_aliases {
            renderer.render_type_alias(md, alias_item);
        }
    }

    /// Render the Statics section.
    fn render_statics_section(&self, md: &mut String, statics: &[&Item]) {
        if statics.is_empty() {
            return;
        }

        _ = write!(md, "## Statics\n\n");

        let renderer = ItemRenderer::new(self.ctx, self.current_file);

        for static_item in statics {
            renderer.render_static(md, static_item);
        }
    }

    /// Build TOC entries from categorized items.
    ///
    /// Creates a hierarchical structure for the table of contents:
    /// - Modules section
    /// - Types section (with children: structs, enums, unions, type aliases)
    /// - Traits section
    /// - Functions section
    /// - Constants section
    /// - Statics section
    /// - Macros section
    fn build_toc_entries(items: &CategorizedItems) -> Vec<TocEntry> {
        let mut entries = Vec::new();

        // Helper to create item entries for items with IDs
        #[expect(clippy::items_after_statements, reason = "Helper function definition")]
        fn item_entries(items: &[(&Id, &Item)]) -> Vec<TocEntry> {
            items
                .iter()
                .filter_map(|(_, item)| {
                    let name = item.name.as_deref()?;
                    Some(TocEntry::new(
                        format!("`{name}`"),
                        AnchorUtils::slugify_anchor(name),
                    ))
                })
                .collect()
        }

        // Helper for items without IDs
        #[expect(clippy::items_after_statements, reason = "Helper function definition")]
        fn simple_item_entries(items: &[&Item]) -> Vec<TocEntry> {
            items
                .iter()
                .filter_map(|item| {
                    let name = item.name.as_deref()?;
                    Some(TocEntry::new(
                        format!("`{name}`"),
                        AnchorUtils::slugify_anchor(name),
                    ))
                })
                .collect()
        }

        // === Modules ===
        if !items.modules.is_empty() {
            entries.push(TocEntry::with_children(
                "Modules",
                "modules",
                item_entries(&items.modules),
            ));
        }

        // === Types (combined section) ===
        if items.has_types() {
            // Collect all type items as children
            let mut type_children = Vec::new();

            // Add structs
            type_children.extend(item_entries(&items.structs));

            // Add enums
            type_children.extend(item_entries(&items.enums));

            // Add unions
            type_children.extend(item_entries(&items.unions));

            // Add type aliases
            type_children.extend(simple_item_entries(&items.type_aliases));

            entries.push(TocEntry::with_children("Types", "types", type_children));
        }

        // === Traits ===
        if !items.traits.is_empty() {
            entries.push(TocEntry::with_children(
                "Traits",
                "traits",
                item_entries(&items.traits),
            ));
        }

        // === Functions ===
        if !items.functions.is_empty() {
            entries.push(TocEntry::with_children(
                "Functions",
                "functions",
                simple_item_entries(&items.functions),
            ));
        }

        // === Constants ===
        if !items.constants.is_empty() {
            entries.push(TocEntry::with_children(
                "Constants",
                "constants",
                simple_item_entries(&items.constants),
            ));
        }

        // === Statics ===
        if !items.statics.is_empty() {
            entries.push(TocEntry::with_children(
                "Statics",
                "statics",
                simple_item_entries(&items.statics),
            ));
        }

        // === Macros ===
        if !items.macros.is_empty() {
            entries.push(TocEntry::with_children(
                "Macros",
                "macros",
                simple_item_entries(&items.macros),
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
                    AnchorUtils::slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        for (id, item) in &items.structs {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "struct",
                    AnchorUtils::slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        for (id, item) in &items.enums {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "enum",
                    AnchorUtils::slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        for (id, item) in &items.traits {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "trait",
                    AnchorUtils::slugify_anchor(name),
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
                    AnchorUtils::slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        for item in &items.macros {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "macro",
                    AnchorUtils::slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        for item in &items.constants {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "const",
                    AnchorUtils::slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        for item in &items.type_aliases {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "type",
                    AnchorUtils::slugify_anchor(name),
                    extract_summary(item.docs.as_deref()),
                ));
            }
        }

        // Add unions (new)
        for (id, item) in &items.unions {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "union",
                    AnchorUtils::slugify_anchor(name),
                    self.get_item_summary(item, **id),
                ));
            }
        }

        // Add statics (new)
        for item in &items.statics {
            if let Some(name) = item.name.as_deref() {
                entries.push(QuickRefEntry::new(
                    name,
                    "static",
                    AnchorUtils::slugify_anchor(name),
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

        _ = writeln!(md, "## Modules\n");

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

            _ = writeln!(md);
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

    /// Render the Traits section.
    fn render_traits_section(&self, md: &mut String, traits: &[(&Id, &Item)]) {
        if traits.is_empty() {
            return;
        }

        _ = writeln!(md, "## Traits\n");

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

        _ = writeln!(md, "## Functions\n");

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

        _ = writeln!(md, "## Macros\n");

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

        _ = writeln!(md, "## Constants\n");

        let renderer = ItemRenderer::new(self.ctx, self.current_file);

        for const_item in constants {
            renderer.render_constant(md, const_item);
        }
    }
}

/// Items categorized by type for organized rendering.
///
/// Items are sorted into buckets by their type so they can be rendered
/// in consistent sections. The structure groups related items:
///
/// - **Types**: Structs, enums, unions, and type aliases
/// - **Traits**: Trait definitions
/// - **Functions**: Standalone functions
/// - **Constants**: Constants and statics
/// - **Macros**: Macro definitions
///
/// This organization improves navigation by grouping related items together.
#[derive(Default)]
struct CategorizedItems<'a> {
    /// Child modules (need ID for linking).
    /// Rendered first for navigation purposes.
    modules: Vec<(&'a Id, &'a Item)>,

    // === Types Section ===
    // These are grouped under a single "Types" heading in the output.
    /// Struct definitions (need ID for impl lookup).
    structs: Vec<(&'a Id, &'a Item)>,

    /// Enum definitions (need ID for impl lookup).
    enums: Vec<(&'a Id, &'a Item)>,

    /// Union definitions (need ID for impl lookup).
    unions: Vec<(&'a Id, &'a Item)>,

    /// Type alias definitions.
    type_aliases: Vec<&'a Item>,

    // === Other Sections ===
    /// Trait definitions (need ID for impl lookup).
    traits: Vec<(&'a Id, &'a Item)>,

    /// Standalone functions.
    functions: Vec<&'a Item>,

    /// Constants.
    constants: Vec<&'a Item>,

    /// Static variables.
    statics: Vec<&'a Item>,

    /// Macro definitions.
    macros: Vec<&'a Item>,
}

impl CategorizedItems<'_> {
    /// Check if the Types section has any items.
    ///
    /// Returns true if there are any structs, enums, unions, or type aliases.
    pub const fn has_types(&self) -> bool {
        !self.structs.is_empty()
            || !self.enums.is_empty()
            || !self.unions.is_empty()
            || !self.type_aliases.is_empty()
    }

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
        self.unions
            .sort_by(|a, b| item_name(a.1).cmp(item_name(b.1)));
        self.traits
            .sort_by(|a, b| item_name(a.1).cmp(item_name(b.1)));

        // Sort items without IDs by name
        self.type_aliases
            .sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.functions
            .sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.constants
            .sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.statics.sort_by(|a, b| item_name(a).cmp(item_name(b)));
        self.macros.sort_by(|a, b| item_name(a).cmp(item_name(b)));
    }
}
