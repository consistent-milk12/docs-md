//! Item rendering for documentation generation.
//!
//! This module provides the [`ItemRenderer`] struct which handles rendering
//! individual Rust items (structs, enums, traits, functions, macros, constants,
//! and type aliases) to markdown format.

use crate::generator::context::GeneratorContext;
use crate::generator::impls::ImplRenderer;
use crate::types::TypeRenderer;
use rustdoc_types::{Id, Item, ItemEnum, StructKind, Visibility};
use std::fmt::Write;

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
pub struct ItemRenderer<'a> {
    /// Reference to the shared generator context.
    ctx: &'a GeneratorContext<'a>,
}

impl<'a> ItemRenderer<'a> {
    /// Create a new item renderer with the given context.
    pub fn new(ctx: &'a GeneratorContext<'a>) -> Self {
        Self { ctx }
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
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        if let ItemEnum::Struct(s) = &item.inner {
            // === Signature Section ===
            let generics = type_renderer.render_generics(&s.generics.params);
            let where_clause = type_renderer.render_where_clause(&s.generics.where_predicates);

            _ = write!(md, "### `{name}{generics}`\n\n");

            // === Definition Code Block ===
            md.push_str("```rust\n");
            match &s.kind {
                StructKind::Unit => {
                    _ = writeln!(md, "struct {name}{generics}{where_clause};");
                }

                StructKind::Tuple(fields) => {
                    let field_types: Vec<String> = fields
                        .iter()
                        .filter_map(|id| id.as_ref())
                        .filter_map(|id| krate.index.get(id))
                        .filter_map(|item| {
                            if let ItemEnum::StructField(ty) = &item.inner {
                                Some(type_renderer.render_type(ty))
                            } else {
                                None
                            }
                        })
                        .collect();
                    _ = writeln!(
                        md,
                        "struct {}{}({}){};",
                        name,
                        generics,
                        field_types.join(", "),
                        where_clause
                    );
                }

                StructKind::Plain { fields, .. } => {
                    _ = writeln!(md, "struct {name}{generics}{where_clause} {{");

                    for field_id in fields {
                        if let Some(field) = krate.index.get(field_id) {
                            let field_name = field.name.as_deref().unwrap_or("_");
                            if let ItemEnum::StructField(ty) = &field.inner {
                                let vis = match &field.visibility {
                                    Visibility::Public => "pub ",
                                    _ => "",
                                };
                                _ = writeln!(
                                    md,
                                    "    {}{}: {},",
                                    vis,
                                    field_name,
                                    type_renderer.render_type(ty)
                                );
                            }
                        }
                    }
                    md.push_str("}\n");
                }
            }
            md.push_str("```\n\n");
        }

        // === Documentation Section ===
        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
        }

        // === Fields Documentation ===
        if let ItemEnum::Struct(s) = &item.inner
            && let StructKind::Plain { fields, .. } = &s.kind
        {
            self.render_struct_fields(md, fields);
        }

        // === Implementations Section ===
        let impl_renderer = ImplRenderer::new(self.ctx);
        impl_renderer.render_impl_blocks(md, item_id);
    }

    /// Render documented fields for a plain struct.
    fn render_struct_fields(&self, md: &mut String, fields: &[Id]) {
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        let documented_fields: Vec<_> = fields
            .iter()
            .filter_map(|id| krate.index.get(id))
            .filter(|f| f.docs.is_some())
            .collect();

        if !documented_fields.is_empty() {
            md.push_str("#### Fields\n\n");
            for field in documented_fields {
                let field_name = field.name.as_deref().unwrap_or("_");
                if let ItemEnum::StructField(ty) = &field.inner {
                    _ = write!(
                        md,
                        "- **`{}`**: `{}`",
                        field_name,
                        type_renderer.render_type(ty)
                    );

                    if let Some(docs) = &field.docs {
                        _ = write!(md, "\n\n  {}", docs.replace('\n', "\n  "));
                    }

                    md.push_str("\n\n");
                }
            }
        }
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
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        if let ItemEnum::Enum(e) = &item.inner {
            // === Signature Section ===
            let generics = type_renderer.render_generics(&e.generics.params);
            let where_clause = type_renderer.render_where_clause(&e.generics.where_predicates);

            _ = write!(md, "### `{name}{generics}`\n\n");

            // === Definition Code Block ===
            md.push_str("```rust\n");
            _ = writeln!(md, "enum {name}{generics}{where_clause} {{");

            for variant_id in &e.variants {
                if let Some(variant) = krate.index.get(variant_id) {
                    self.render_enum_variant(md, variant);
                }
            }
            md.push_str("}\n");
            md.push_str("```\n\n");
        }

        // === Documentation Section ===
        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
        }

        // === Variants Documentation ===
        if let ItemEnum::Enum(e) = &item.inner {
            self.render_enum_variants_docs(md, &e.variants);
        }

        // === Implementations Section ===
        let impl_renderer = ImplRenderer::new(self.ctx);
        impl_renderer.render_impl_blocks(md, item_id);
    }

    /// Render a single enum variant in the definition code block.
    fn render_enum_variant(&self, md: &mut String, variant: &Item) {
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);
        let variant_name = variant.name.as_deref().unwrap_or("_");

        if let ItemEnum::Variant(v) = &variant.inner {
            match &v.kind {
                rustdoc_types::VariantKind::Plain => {
                    _ = writeln!(md, "    {variant_name},");
                }

                rustdoc_types::VariantKind::Tuple(fields) => {
                    let field_types: Vec<String> = fields
                        .iter()
                        .filter_map(|id| id.as_ref())
                        .filter_map(|id| krate.index.get(id))
                        .filter_map(|item| {
                            if let ItemEnum::StructField(ty) = &item.inner {
                                Some(type_renderer.render_type(ty))
                            } else {
                                None
                            }
                        })
                        .collect();

                    _ = writeln!(md, "    {}({}),", variant_name, field_types.join(", "));
                }

                rustdoc_types::VariantKind::Struct { fields, .. } => {
                    _ = writeln!(md, "    {variant_name} {{");

                    for field_id in fields {
                        if let Some(field) = krate.index.get(field_id) {
                            let field_name = field.name.as_deref().unwrap_or("_");
                            if let ItemEnum::StructField(ty) = &field.inner {
                                _ = writeln!(
                                    md,
                                    "        {}: {},",
                                    field_name,
                                    type_renderer.render_type(ty)
                                );
                            }
                        }
                    }
                    md.push_str("    },\n");
                }
            }
        }
    }

    /// Render documented variants section for an enum.
    fn render_enum_variants_docs(&self, md: &mut String, variants: &[Id]) {
        let krate = self.ctx.krate;

        let documented_variants: Vec<_> = variants
            .iter()
            .filter_map(|id| krate.index.get(id))
            .filter(|v| v.docs.is_some())
            .collect();

        if !documented_variants.is_empty() {
            md.push_str("#### Variants\n\n");
            for variant in documented_variants {
                let variant_name = variant.name.as_deref().unwrap_or("_");
                _ = write!(md, "- **`{variant_name}`**");

                if let Some(docs) = &variant.docs {
                    _ = write!(md, "\n\n  {}", docs.replace('\n', "\n  "));
                }

                md.push_str("\n\n");
            }
        }
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
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        if let ItemEnum::Trait(t) = &item.inner {
            // === Signature Section ===
            let generics = type_renderer.render_generics(&t.generics.params);
            let where_clause = type_renderer.render_where_clause(&t.generics.where_predicates);

            _ = write!(md, "### `{name}{generics}`\n\n");

            // === Definition Code Block ===
            md.push_str("```rust\n");

            let bounds = if t.bounds.is_empty() {
                String::new()
            } else {
                let bound_strs: Vec<String> = t
                    .bounds
                    .iter()
                    .map(|b| type_renderer.render_generic_bound(b))
                    .collect();
                format!(": {}", bound_strs.join(" + "))
            };

            _ = writeln!(md, "trait {name}{generics}{bounds}{where_clause} {{ ... }}");
            md.push_str("```\n\n");
        }

        // === Documentation Section ===
        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
        }

        // === Required Methods Section ===
        if let ItemEnum::Trait(t) = &item.inner
            && !t.items.is_empty()
        {
            md.push_str("#### Required Methods\n\n");
            for method_id in &t.items {
                if let Some(method) = self.ctx.krate.index.get(method_id) {
                    self.render_trait_item(md, method);
                }
            }
        }
    }

    /// Render a single trait item (method, associated type, or associated constant).
    ///
    /// Each item is rendered as a bullet point with its signature in backticks.
    /// The first line of documentation is included for methods.
    fn render_trait_item(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("_");
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        match &item.inner {
            ItemEnum::Function(f) => {
                let generics = type_renderer.render_generics(&f.generics.params);

                let params: Vec<String> = f
                    .sig
                    .inputs
                    .iter()
                    .map(|(param_name, ty)| {
                        format!("{param_name}: {}", type_renderer.render_type(ty))
                    })
                    .collect();

                let ret = f
                    .sig
                    .output
                    .as_ref()
                    .map(|ty| format!(" -> {}", type_renderer.render_type(ty)))
                    .unwrap_or_default();

                _ = write!(
                    md,
                    "- `fn {}{}({}){}`",
                    name,
                    generics,
                    params.join(", "),
                    ret
                );

                if let Some(docs) = &item.docs
                    && let Some(first_line) = docs.lines().next()
                {
                    _ = write!(md, "\n\n  {first_line}");
                }

                md.push_str("\n\n");
            }

            ItemEnum::AssocType { bounds, type_, .. } => {
                let bounds_str = if bounds.is_empty() {
                    String::new()
                } else {
                    format!(": {}", bounds.len())
                };
                let default_str = type_
                    .as_ref()
                    .map(|ty| format!(" = {}", type_renderer.render_type(ty)))
                    .unwrap_or_default();

                _ = write!(md, "- `type {name}{bounds_str}{default_str}`\n\n");
            }

            ItemEnum::AssocConst { type_, .. } => {
                _ = write!(
                    md,
                    "- `const {name}: {}`\n\n",
                    type_renderer.render_type(type_)
                );
            }

            _ => {
                _ = write!(md, "- `{name}`\n\n");
            }
        }
    }

    /// Render a standalone function to markdown.
    ///
    /// Produces a section with:
    /// - Heading with function name
    /// - Rust code block showing full signature
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
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        if let ItemEnum::Function(f) = &item.inner {
            let generics = type_renderer.render_generics(&f.generics.params);
            let where_clause = type_renderer.render_where_clause(&f.generics.where_predicates);

            let params: Vec<String> = f
                .sig
                .inputs
                .iter()
                .map(|(param_name, ty)| {
                    format!("{param_name}: {}", type_renderer.render_type(ty))
                })
                .collect();

            let ret = f
                .sig
                .output
                .as_ref()
                .map(|ty| format!(" -> {}", type_renderer.render_type(ty)))
                .unwrap_or_default();

            let is_async = if f.header.is_async { "async " } else { "" };
            let is_const = if f.header.is_const { "const " } else { "" };
            let is_unsafe = if f.header.is_unsafe { "unsafe " } else { "" };

            _ = write!(md, "### `{name}`\n\n");
            md.push_str("```rust\n");

            _ = writeln!(
                md,
                "{}{}{}fn {}{}({}){}{}",
                is_const,
                is_async,
                is_unsafe,
                name,
                generics,
                params.join(", "),
                ret,
                where_clause
            );

            md.push_str("```\n\n");
        }

        // === Documentation Section ===
        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
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
    pub fn render_macro(md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");

        _ = write!(md, "### `{name}!`\n\n");

        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
        }
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
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        if let ItemEnum::Constant { type_, const_ } = &item.inner {
            _ = write!(md, "### `{name}`\n\n");

            md.push_str("```rust\n");

            let value = const_
                .value
                .as_ref()
                .map(|v| format!(" = {v}"))
                .unwrap_or_default();

            _ = writeln!(
                md,
                "const {name}: {}{value};",
                type_renderer.render_type(type_)
            );

            md.push_str("```\n\n");
        }

        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
        }
    }

    /// Render a type alias to markdown.
    ///
    /// Produces a section with:
    /// - Heading with alias name and generics
    /// - Rust code block showing `type Name<T> = TargetType;`
    /// - Documentation from doc comments
    pub fn render_type_alias(&self, md: &mut String, item: &Item) {
        let name = item.name.as_deref().unwrap_or("unnamed");
        let krate = self.ctx.krate;
        let type_renderer = TypeRenderer::new(krate);

        if let ItemEnum::TypeAlias(ta) = &item.inner {
            let generics = type_renderer.render_generics(&ta.generics.params);
            let where_clause = type_renderer.render_where_clause(&ta.generics.where_predicates);

            _ = write!(md, "### `{name}{generics}`\n\n");
            md.push_str("```rust\n");

            _ = writeln!(
                md,
                "type {name}{generics}{where_clause} = {};",
                type_renderer.render_type(&ta.type_)
            );
            md.push_str("```\n\n");
        }

        if let Some(docs) = &item.docs {
            md.push_str(docs);
            md.push_str("\n\n");
        }
    }
}
