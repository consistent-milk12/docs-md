//! Item rendering for documentation generation.
//!
//! This module provides the [`ItemRenderer`] struct which handles rendering
//! individual Rust items (structs, enums, traits, functions, macros, constants,
//! and type aliases) to markdown format.

use std::fmt::Write;

use rustdoc_types::{Id, Item, ItemEnum, StructKind};

use crate::generator::context::RenderContext;
use crate::generator::impls::ImplRenderer;
use crate::generator::render_shared::{
    CategorizedTraitItems, TraitRenderer, append_docs, render_constant_definition,
    render_enum_definition, render_enum_variants_docs, render_function_definition,
    render_macro_heading, render_source_location, render_static_definition,
    render_struct_definition, render_struct_fields, render_type_alias_definition,
    render_union_definition, render_union_fields,
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

    /// Render source location if enabled in config.
    ///
    /// Returns the source location string if `source_locations` is enabled,
    /// otherwise returns an empty string. Uses the source path config to
    /// generate clickable links to the `.source_*` directory when available.
    fn maybe_render_source_location(&self, item: &Item) -> String {
        if self.ctx.render_config().include_source.source_locations {
            let source_config = self.ctx.source_path_config_for_file(self.current_file);
            render_source_location(item.span.as_ref(), source_config.as_ref())
        } else {
            String::new()
        }
    }

    /// Resolve a Use item to its target, returning the alias name and target item.
    ///
    /// For non-Use items, returns the item's own name and itself.
    /// For Use items, resolves the target and returns the alias name with the target.
    fn resolve_use_target<'b>(&self, item: &'b Item) -> Option<(&'b str, &'b Item)>
    where
        'a: 'b,
    {
        if let ItemEnum::Use(use_item) = &item.inner {
            // Get alias name from the Use item
            let name = use_item.name.as_str();

            // Try to resolve target by ID
            if let Some(target_id) = &use_item.id
                && let Some(target) = self.ctx.get_item(target_id)
            {
                return Some((name, target));
            }
            // Can't resolve - return None
            None
        } else {
            // Not a Use item - return the item itself
            Some((item.name.as_deref().unwrap_or("unnamed"), item))
        }
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
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a struct.
    pub fn render_struct(&self, md: &mut String, item_id: Id, item: &Item) {
        let krate = self.ctx.krate();

        // Resolve Use items to their target, getting the display name and actual item
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return; // Couldn't resolve Use target
        };

        // Get the actual ID for impl lookup (use target ID for Use items)
        let actual_id = if let ItemEnum::Use(use_item) = &item.inner {
            use_item.id.unwrap_or(item_id)
        } else {
            item_id
        };

        if let ItemEnum::Struct(s) = &actual_item.inner {
            // Struct definition (heading + code block)
            render_struct_definition(md, name, s, krate, &self.type_renderer);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        // Documentation (from actual item, not the Use item)
        append_docs(md, self.process_docs(actual_item));

        // Fields documentation
        if let ItemEnum::Struct(s) = &actual_item.inner
            && let StructKind::Plain { fields, .. } = &s.kind
        {
            render_struct_fields(md, fields, krate, &self.type_renderer, |field| {
                self.process_docs(field)
            });
        }

        // Implementations
        let impl_renderer = ImplRenderer::new(self.ctx, self.current_file);
        impl_renderer.render_impl_blocks(md, actual_id);
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
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to an enum.
    pub fn render_enum(&self, md: &mut String, item_id: Id, item: &Item) {
        let krate = self.ctx.krate();

        // Resolve Use items to their target, getting the display name and actual item
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return; // Couldn't resolve Use target
        };

        // Get the actual ID for impl lookup (use target ID for Use items)
        let actual_id = if let ItemEnum::Use(use_item) = &item.inner {
            use_item.id.unwrap_or(item_id)
        } else {
            item_id
        };

        if let ItemEnum::Enum(e) = &actual_item.inner {
            // Enum definition (heading + code block with variants)
            render_enum_definition(md, name, e, krate, &self.type_renderer);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        // Documentation (from actual item, not the Use item)
        append_docs(md, self.process_docs(actual_item));

        // Variants documentation
        if let ItemEnum::Enum(e) = &actual_item.inner {
            render_enum_variants_docs(md, &e.variants, krate, |variant| self.process_docs(variant));
        }

        // Implementations
        let impl_renderer = ImplRenderer::new(self.ctx, self.current_file);
        impl_renderer.render_impl_blocks(md, actual_id);
    }

    /// Render a trait definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with trait name and generics
    /// - Rust code block showing trait signature with supertraits
    /// - Documentation from doc comments
    /// - Associated Types section (if any)
    /// - Associated Constants section (if any)
    /// - Required Methods section (methods without default impl)
    /// - Provided Methods section (methods with default impl)
    /// - Implementors section (types that implement this trait)
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a trait.
    pub fn render_trait(&self, md: &mut String, item_id: Id, item: &Item) {
        // Resolve Use items to their target
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        // Get actual ID for implementation
        let actual_id = if let ItemEnum::Use(use_item) = &item.inner {
            use_item.id.unwrap_or(item_id)
        } else {
            item_id
        };

        let krate = self.ctx.krate();

        if let ItemEnum::Trait(t) = &actual_item.inner {
            // Trait definition (heading + code block)
            TraitRenderer::render_trait_definition(md, name, t, &self.type_renderer);

            // Source location (if enabled)
            _ = writeln!(md, "{}", &self.maybe_render_source_location(actual_item));

            // Documentation
            append_docs(md, self.process_docs(actual_item));

            // Categorize trait items
            let items = CategorizedTraitItems::categorize_trait_items(&t.items, krate);

            // Associated Types
            if !items.associated_types.is_empty() {
                _ = writeln!(md, "#### Associated Types\n");

                for type_item in &items.associated_types {
                    TraitRenderer::render_trait_item(md, type_item, &self.type_renderer, |m| {
                        self.process_docs(m)
                    });
                }
            }

            // Associated Constants
            if !items.associated_consts.is_empty() {
                _ = writeln!(md, "#### Associated Constants\n");

                for const_item in &items.associated_consts {
                    TraitRenderer::render_trait_item(md, const_item, &self.type_renderer, |m| {
                        self.process_docs(m)
                    });
                }
            }

            // Required Methods
            if !items.required_methods.is_empty() {
                _ = writeln!(md, "#### Required Methods\n");

                for required_method in &items.required_methods {
                    TraitRenderer::render_trait_item(
                        md,
                        required_method,
                        &self.type_renderer,
                        |m| self.process_docs(m),
                    );
                }
            }

            // Provided Methods
            if !items.provided_methods.is_empty() {
                _ = writeln!(md, "#### Provided Methods \n");

                for provided_method in &items.provided_methods {
                    TraitRenderer::render_trait_item(
                        md,
                        provided_method,
                        &self.type_renderer,
                        |m| self.process_docs(m),
                    );
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
        use std::collections::BTreeSet;

        let krate = self.ctx.krate();

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
                .and_then(|type_id| self.ctx.create_link(type_id, self.current_file))
                .unwrap_or_else(|| format!("`{for_type}`"));

            implementors.insert(entry);
        }

        if !implementors.is_empty() {
            _ = writeln!(md, "#### Implementors\n");

            for implementor in implementors {
                _ = writeln!(md, "- {implementor}");
            }

            _ = writeln!(md);
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
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a function.
    pub fn render_function(&self, md: &mut String, item: &Item) {
        // Resolve Use items to their target
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        if let ItemEnum::Function(f) = &actual_item.inner {
            render_function_definition(md, name, f, &self.type_renderer);

            // Add type links for function signature types
            self.render_function_type_links(md, f);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        // Documentation
        append_docs(md, self.process_docs(actual_item));
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
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a macro.
    pub fn render_macro(&self, md: &mut String, item: &Item) {
        // Resolve Use items to their target
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        render_macro_heading(md, name);

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        append_docs(md, self.process_docs(actual_item));
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
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a constant.
    pub fn render_constant(&self, md: &mut String, item: &Item) {
        // Resolve Use items to their target
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        if let ItemEnum::Constant { type_, const_ } = &actual_item.inner {
            render_constant_definition(md, name, type_, const_, &self.type_renderer);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        append_docs(md, self.process_docs(actual_item));
    }

    /// Render a type alias to markdown.
    ///
    /// Produces a section with:
    /// - Heading with alias name and generics
    /// - Rust code block showing `type Name<T> = TargetType;`
    /// - Documentation from doc comments
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a type alias.
    pub fn render_type_alias(&self, md: &mut String, item: &Item) {
        // Resolve Use items to their target
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        if let ItemEnum::TypeAlias(ta) = &actual_item.inner {
            render_type_alias_definition(md, name, ta, &self.type_renderer);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        append_docs(md, self.process_docs(actual_item));
    }

    /// Render a union definition to markdown.
    ///
    /// Produces a section with:
    /// - Heading with union name and generics
    /// - Rust code block showing the union definition with all fields
    /// - Documentation from doc comments
    /// - Fields section (for unions with documented fields)
    /// - Implementations section (inherent and trait impls)
    ///
    /// # Union Fields
    ///
    /// Unlike structs, all union fields share the same memory location.
    /// Only one field can be active at a time, and accessing fields
    /// requires `unsafe` code.
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a union.
    pub fn render_union(&self, md: &mut String, item_id: Id, item: &Item) {
        let krate = self.ctx.krate();

        // Resolve Use items to their target, getting the display name and actual item
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        // Get the actual ID for impl lookup (use target ID for Use items)
        let actual_id = if let ItemEnum::Use(use_item) = &item.inner {
            use_item.id.unwrap_or(item_id)
        } else {
            item_id
        };

        if let ItemEnum::Union(u) = &actual_item.inner {
            // Union definition (heading + code block)
            render_union_definition(md, name, u, krate, &self.type_renderer);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        // Documentation (from actual item, not the Use item)
        append_docs(md, self.process_docs(actual_item));

        // Fields documentation
        if let ItemEnum::Union(u) = &actual_item.inner {
            render_union_fields(md, &u.fields, krate, &self.type_renderer, |field| {
                self.process_docs(field)
            });
        }

        // Implementations
        let impl_renderer = ImplRenderer::new(self.ctx, self.current_file);
        impl_renderer.render_impl_blocks(md, actual_id);
    }

    /// Render a static variable to markdown.
    ///
    /// Produces a section with:
    /// - Heading with static name
    /// - Rust code block showing `static [mut] NAME: Type = expr;`
    /// - Documentation from doc comments
    ///
    /// # Static Modifiers
    ///
    /// The signature includes applicable modifiers:
    /// - `static mut` - Mutable static (access requires unsafe)
    /// - `unsafe static` - Unsafe static in extern block
    ///
    /// # Re-exports
    ///
    /// Also handles `pub use` re-exports where the item is a Use pointing to a static.
    pub fn render_static(&self, md: &mut String, item: &Item) {
        // Resolve Use items to their target
        let Some((name, actual_item)) = self.resolve_use_target(item) else {
            return;
        };

        if let ItemEnum::Static(s) = &actual_item.inner {
            render_static_definition(md, name, s, &self.type_renderer);
        }

        // Source location (if enabled)
        _ = write!(md, "{}", &self.maybe_render_source_location(actual_item));

        append_docs(md, self.process_docs(actual_item));
    }
}
