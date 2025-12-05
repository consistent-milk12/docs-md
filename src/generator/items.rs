//! Item rendering for documentation generation.
//!
//! This module provides the [`ItemRenderer`] struct which handles rendering
//! individual Rust items (structs, enums, traits, functions, macros, constants,
//! and type aliases) to markdown format.

use rustdoc_types::{Id, Item, ItemEnum, StructKind};

use crate::generator::context::RenderContext;
use crate::generator::impls::ImplRenderer;
use crate::generator::render_shared::{
    append_docs, render_constant_definition, render_enum_definition, render_enum_variants_docs,
    render_function_definition, render_macro_heading, render_struct_definition,
    render_struct_fields, render_trait_definition, render_trait_item, render_type_alias_definition,
};
use crate::types::TypeRenderer;

/// Renders individual Rust items to markdown.
///
/// This struct provides methods for rendering each type of documentable item:
/// - Structs with fields and implementations
/// - Enums with variants and implementations
/// - Traits with methods and associated types
/// - Functions with signatures
/// - Macros
/// - Constants
/// - Type aliases
///
/// The renderer is generic over [`RenderContext`], allowing it to work with
/// both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.
pub struct ItemRenderer<'a> {
    /// Reference to the render context (either single-crate or multi-crate).
    ctx: &'a dyn RenderContext,

    /// Path of the current file being generated (for relative link calculation).
    current_file: &'a str,

    /// Type renderer for converting rustdoc types to strings.
    /// Stored once to avoid redundant construction in each render method.
    type_renderer: TypeRenderer<'a>,
}

impl<'a> ItemRenderer<'a> {
    /// Create a new item renderer with the given context.
    ///
    /// # Arguments
    ///
    /// * `ctx` - Render context (implements `RenderContext` trait)
    /// * `current_file` - Path of the current file (for relative link calculation)
    pub fn new(ctx: &'a dyn RenderContext, current_file: &'a str) -> Self {
        let type_renderer = TypeRenderer::new(ctx.krate());
        Self {
            ctx,
            current_file,
            type_renderer,
        }
    }

    /// Process documentation string to resolve intra-doc links.
    ///
    /// Delegates to the render context's `process_docs` method, which handles
    /// both single-crate and multi-crate link resolution.
    fn process_docs(&self, item: &Item) -> Option<String> {
        self.ctx.process_docs(item, self.current_file)
    }

    /// Render a struct definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with struct name and generics
    /// - Rust code block showing the struct definition
    /// - Documentation from doc comments
    /// - Fields section (for structs with documented fields)
    /// - Implementations section (inherent and trait impls)
    ///
    /// # Struct Kinds
    ///
    /// Rust has three kinds of structs, each rendered differently:
    /// - **Unit**: `struct Foo;`
    /// - **Tuple**: `struct Foo(T, U);`
    /// - **Plain** (named fields): `struct Foo { field: T }`
    pub fn render_struct(&self, md: &mut String, item_id: Id, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");
        let krate = self.ctx.krate();

        if let ItemEnum::Struct(s) = &item.inner {
            // Struct definition (heading + code block)
            render_struct_definition(md, name, s, krate, &self.type_renderer);
        }

        // Documentation
        append_docs(md, self.process_docs(item));

        // Fields documentation
        if let ItemEnum::Struct(s) = &item.inner
            && let StructKind::Plain { fields, .. } = &s.kind
        {
            render_struct_fields(md, fields, krate, &self.type_renderer, |field| {
                self.process_docs(field)
            });
        }

        // Implementations
        let impl_renderer = ImplRenderer::new(self.ctx, self.current_file);
        impl_renderer.render_impl_blocks(md, item_id);
    }

    /// Render an enum definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with enum name and generics
    /// - Rust code block showing the enum definition with all variants
    /// - Documentation from doc comments
    /// - Variants section (for variants with documentation)
    /// - Implementations section (inherent and trait impls)
    ///
    /// # Variant Kinds
    ///
    /// Rust enums support three variant kinds:
    /// - **Plain**: `Variant` (no data)
    /// - **Tuple**: `Variant(T, U)` (positional data)
    /// - **Struct**: `Variant { field: T }` (named fields)
    pub fn render_enum(&self, md: &mut String, item_id: Id, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");
        let krate = self.ctx.krate();

        if let ItemEnum::Enum(e) = &item.inner {
            // Enum definition (heading + code block with variants)
            render_enum_definition(md, name, e, krate, &self.type_renderer);
        }

        // Documentation
        append_docs(md, self.process_docs(item));

        // Variants documentation
        if let ItemEnum::Enum(e) = &item.inner {
            render_enum_variants_docs(md, &e.variants, krate, |variant| {
                self.process_docs(variant)
            });
        }

        // Implementations
        let impl_renderer = ImplRenderer::new(self.ctx, self.current_file);
        impl_renderer.render_impl_blocks(md, item_id);
    }

    /// Render a trait definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with trait name and generics
    /// - Rust code block showing trait signature with supertraits
    /// - Documentation from doc comments
    /// - Required methods section listing all trait items
    ///
    /// # Trait Items
    ///
    /// Traits can contain:
    /// - **Methods**: `fn method(&self) -> T`
    /// - **Associated Types**: `type Item;`
    /// - **Associated Constants**: `const VALUE: T;`
    pub fn render_trait(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::Trait(t) = &item.inner {
            // Trait definition (heading + code block)
            render_trait_definition(md, name, t, &self.type_renderer);
        }

        // Documentation
        append_docs(md, self.process_docs(item));

        // Required methods section
        if let ItemEnum::Trait(t) = &item.inner
            && !t.items.is_empty()
        {
            md.push_str("#### Required Methods\n\n");
            for method_id in &t.items {
                if let Some(method) = self.ctx.krate().index.get(method_id) {
                    render_trait_item(md, method, &self.type_renderer, |m| self.process_docs(m));
                }
            }
        }
    }

    /// Render a standalone function to markdown.
    ///
    /// Produces a section with:
    /// - Heading with function name
    /// - Rust code block showing full signature
    /// - Type links for parameter and return types (when resolvable)
    /// - Documentation from doc comments
    ///
    /// # Function Modifiers
    ///
    /// The signature includes applicable modifiers:
    /// - `const fn` - Compile-time evaluable
    /// - `async fn` - Returns a Future
    /// - `unsafe fn` - Requires unsafe block to call
    pub fn render_function(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::Function(f) = &item.inner {
            render_function_definition(md, name, f, &self.type_renderer);

            // Add type links for function signature types
            self.render_function_type_links(md, f);
        }

        // Documentation
        append_docs(md, self.process_docs(item));
    }

    /// Render linked types used in a function signature.
    ///
    /// Collects all types from parameters and return type, creates links
    /// for resolvable types, and outputs a "**Types:**" line.
    fn render_function_type_links(&self, md: &mut String, f: &rustdoc_types::Function) {
        use std::collections::HashSet;
        use std::fmt::Write;

        let mut all_types = Vec::new();

        // Collect from parameters
        for (_, ty) in &f.sig.inputs {
            all_types.extend(self.type_renderer.collect_linkable_types(ty));
        }

        // Collect from return type
        if let Some(output) = &f.sig.output {
            all_types.extend(self.type_renderer.collect_linkable_types(output));
        }

        // Deduplicate by name (keep first occurrence)
        let mut seen = HashSet::new();
        let unique_types: Vec<_> = all_types
            .into_iter()
            .filter(|(name, _)| seen.insert(name.clone()))
            .collect();

        // Create links for resolvable types
        let links: Vec<String> = unique_types
            .iter()
            .filter_map(|(name, id)| {
                self.ctx.create_link(*id, self.current_file).or_else(|| {
                    // Fallback: check if it's on the current page
                    if let Some(registry) = self.ctx.link_registry()
                        && let Some(path) = registry.get_path(*id)
                        && path == self.current_file
                    {
                        Some(format!("[`{name}`](#{})", name.to_lowercase()))
                    } else {
                        None
                    }
                })
            })
            .collect();

        // Only output if we have links
        if !links.is_empty() {
            _ = writeln!(md, "**Types:** {}\n", links.join(", "));
        }
    }

    /// Render a macro definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with macro name and `!` suffix
    /// - Documentation from doc comments
    ///
    /// Note: We don't show macro rules/implementation since rustdoc JSON
    /// doesn't provide the full macro definition, only metadata.
    pub fn render_macro(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        render_macro_heading(md, name);
        append_docs(md, self.process_docs(item));
    }

    /// Render a constant definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with constant name
    /// - Rust code block showing `const NAME: Type = value;`
    /// - Documentation from doc comments
    ///
    /// The value may be omitted if rustdoc couldn't determine it
    /// (e.g., for complex const expressions).
    pub fn render_constant(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::Constant { type_, const_ } = &item.inner {
            render_constant_definition(md, name, type_, const_, &self.type_renderer);
        }

        append_docs(md, self.process_docs(item));
    }

    /// Render a type alias to markdown.
    ///
    /// Produces a section with:
    /// - Heading with alias name and generics
    /// - Rust code block showing `type Name<T> = TargetType;`
    /// - Documentation from doc comments
    pub fn render_type_alias(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        if let ItemEnum::TypeAlias(ta) = &item.inner {
            render_type_alias_definition(md, name, ta, &self.type_renderer);
        }

        append_docs(md, self.process_docs(item));
    }
}
