//! Module markdown rendering for documentation generation.
//!
//! This module provides the [`ModuleRenderer`] struct which handles rendering
//! a Rust module's documentation to markdown format, including all its items
//! organized by type.

use crate::generator::context::RenderContext;
use crate::generator::items::ItemRenderer;
use rustdoc_types::{Id, Item, ItemEnum};
use std::fmt::Write;

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
    /// * `ctx` - Render context (implements RenderContext trait)
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
            self.render_all_sections(&mut md, &categorized);
        }

        md
    }

    /// Categorize module items by type for organized rendering.
    fn categorize_items(&self, item_ids: &'a [Id]) -> CategorizedItems<'a> {
        let mut items = CategorizedItems::default();

        for item_id in item_ids {
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
                    _ => {}
                }
            }
        }

        items
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

            if let Some(docs) = &module_item.docs
                && let Some(first_line) = docs.lines().next()
            {
                _ = write!(md, " - {first_line}");
            }
            md.push('\n');
        }
        md.push('\n');
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
        for (_item_id, trait_item) in traits {
            renderer.render_trait(md, trait_item);
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
