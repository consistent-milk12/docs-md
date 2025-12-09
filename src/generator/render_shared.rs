//! Shared rendering functions for documentation generation.
//!
//! This module contains standalone rendering functions that can be used by both
//! single-crate ([`ItemRenderer`](super::ItemRenderer)) and multi-crate
//! ([`MultiCrateModuleRenderer`](crate::multi_crate::generator)) renderers.
//!
//! These functions handle the core markdown generation logic without being tied
//! to a specific rendering context, avoiding code duplication between the two modes.

use std::borrow::Cow;
use std::fmt::Write;

use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, StructKind, VariantKind, Visibility};

use crate::generator::context::RenderContext;
use crate::linker::method_anchor;
use crate::types::TypeRenderer;

/// Categorized trait items for structured rendering.
#[derive(Default)]
pub struct CategorizedTraitItems<'a> {
    /// Required methods (no default body)
    pub required_methods: Vec<&'a Item>,

    /// Provided methods (have default body)
    pub provided_methods: Vec<&'a Item>,

    /// Associated types
    pub associated_types: Vec<&'a Item>,

    /// Associated constants
    pub associated_consts: Vec<&'a Item>,
}

impl<'a> CategorizedTraitItems<'a> {
    /// Categorize trait items into required/provided methods, types and constants.
    #[must_use]
    pub fn categorize_trait_items(trait_items: &[Id], krate: &'a Crate) -> Self {
        let mut result = CategorizedTraitItems::default();

        for item_id in trait_items {
            let Some(item) = krate.index.get(item_id) else {
                continue;
            };

            match &item.inner {
                ItemEnum::Function(f) => {
                    if f.has_body {
                        result.provided_methods.push(item);
                    } else {
                        result.required_methods.push(item);
                    }
                },

                ItemEnum::AssocType { .. } => {
                    result.associated_types.push(item);
                },

                ItemEnum::AssocConst { .. } => {
                    result.associated_consts.push(item);
                },

                _ => {},
            }
        }

        result
    }
}

/// Sanitize trait paths by removing macro artifacts.
///
/// Rustdoc JSON can contain `$crate::` prefixes from macro expansions
/// which leak implementation details into documentation. This function
/// removes these artifacts for cleaner output.
///
/// Uses `Cow<str>` to avoid allocation when no changes are needed.
///
/// # Examples
///
/// ```
/// use cargo_docs_md::generator::render_shared::sanitize_path;
///
/// assert_eq!(sanitize_path("$crate::clone::Clone"), "clone::Clone");
/// assert_eq!(sanitize_path("std::fmt::Debug"), "std::fmt::Debug");
/// ```
#[must_use]
pub fn sanitize_path(path: &str) -> Cow<'_, str> {
    if path.contains("$crate::") {
        Cow::Owned(path.replace("$crate::", ""))
    } else {
        Cow::Borrowed(path)
    }
}

/// Sanitize self parameter in function signatures.
///
/// Converts verbose self type annotations to idiomatic Rust syntax:
/// - `self: &Self` → `&self`
/// - `self: &mut Self` → `&mut self`
/// - `self: Self` → `self`
///
/// Uses `Cow<str>` to avoid allocation when no changes are needed.
///
/// # Examples
///
/// ```
/// use cargo_docs_md::generator::render_shared::sanitize_self_param;
///
/// assert_eq!(sanitize_self_param("self: &Self"), "&self");
/// assert_eq!(sanitize_self_param("self: &mut Self"), "&mut self");
/// assert_eq!(sanitize_self_param("self: Self"), "self");
/// assert_eq!(sanitize_self_param("x: i32"), "x: i32");
/// ```
#[must_use]
pub fn sanitize_self_param(param: &str) -> Cow<'_, str> {
    match param {
        "self: &Self" => Cow::Borrowed("&self"),
        "self: &mut Self" => Cow::Borrowed("&mut self"),
        "self: Self" => Cow::Borrowed("self"),
        _ => Cow::Borrowed(param),
    }
}

/// Render a struct definition code block to markdown.
///
/// Produces a heading with the struct name and generics, followed by a Rust
/// code block showing the struct definition.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `name` - The struct name (may differ from item.name for re-exports)
/// * `s` - The struct data from rustdoc
/// * `krate` - The crate containing field definitions
/// * `type_renderer` - Type renderer for generics and field types
pub fn render_struct_definition(
    md: &mut String,
    name: &str,
    s: &rustdoc_types::Struct,
    krate: &Crate,
    type_renderer: &TypeRenderer,
) {
    let generics = type_renderer.render_generics(&s.generics.params);
    let where_clause = type_renderer.render_where_clause(&s.generics.where_predicates);

    _ = write!(md, "### `{name}{generics}`\n\n");

    md.push_str("```rust\n");
    match &s.kind {
        StructKind::Unit => {
            _ = writeln!(md, "struct {name}{generics}{where_clause};");
        },

        StructKind::Tuple(fields) => {
            let field_types: Vec<Cow<str>> = fields
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
        },

        StructKind::Plain {
            fields,
            has_stripped_fields,
        } => {
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

            if *has_stripped_fields {
                _ = writeln!(md, "    // [REDACTED: Private Fields]");
            }

            md.push_str("}\n");
        },
    }
    md.push_str("```\n\n");
}

/// Render documented struct fields to markdown.
///
/// Produces a "Fields" section with each documented field as a bullet point
/// showing the field name, type, and documentation.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `fields` - Field IDs from the struct
/// * `krate` - Crate containing field definitions
/// * `type_renderer` - Type renderer for field types
/// * `process_docs` - Closure to process documentation with intra-doc link resolution
pub fn render_struct_fields<F>(
    md: &mut String,
    fields: &[Id],
    krate: &Crate,
    type_renderer: &TypeRenderer,
    process_docs: F,
) where
    F: Fn(&Item) -> Option<String>,
{
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

                if let Some(docs) = process_docs(field) {
                    _ = write!(md, "\n\n  {}", docs.replace('\n', "\n  "));
                }

                md.push_str("\n\n");
            }
        }
    }
}

/// Render an enum definition code block to markdown.
///
/// Produces a heading with the enum name and generics, followed by a Rust
/// code block showing the enum definition with all variants.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `name` - The enum name (may differ from item.name for re-exports)
/// * `e` - The enum data from rustdoc
/// * `krate` - The crate containing variant definitions
/// * `type_renderer` - Type renderer for generics and variant types
pub fn render_enum_definition(
    md: &mut String,
    name: &str,
    e: &rustdoc_types::Enum,
    krate: &Crate,
    type_renderer: &TypeRenderer,
) {
    let generics = type_renderer.render_generics(&e.generics.params);
    let where_clause = type_renderer.render_where_clause(&e.generics.where_predicates);

    _ = write!(md, "### `{name}{generics}`\n\n");

    md.push_str("```rust\n");
    _ = writeln!(md, "enum {name}{generics}{where_clause} {{");

    for variant_id in &e.variants {
        if let Some(variant) = krate.index.get(variant_id) {
            render_enum_variant(md, variant, krate, type_renderer);
        }
    }
    md.push_str("}\n");
    md.push_str("```\n\n");
}

/// Render a single enum variant within the definition code block.
///
/// Handles all three variant kinds: plain, tuple, and struct variants.
pub fn render_enum_variant(
    md: &mut String,
    variant: &Item,
    krate: &Crate,
    type_renderer: &TypeRenderer,
) {
    let variant_name = variant.name.as_deref().unwrap_or("_");

    if let ItemEnum::Variant(v) = &variant.inner {
        match &v.kind {
            VariantKind::Plain => {
                _ = writeln!(md, "    {variant_name},");
            },

            VariantKind::Tuple(fields) => {
                let field_types: Vec<Cow<str>> = fields
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
            },

            VariantKind::Struct { fields, .. } => {
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
            },
        }
    }
}

/// Render documented enum variants to markdown.
///
/// Produces a "Variants" section with each documented variant as a bullet point.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `variants` - Variant IDs from the enum
/// * `krate` - Crate containing variant definitions
/// * `process_docs` - Closure to process documentation with intra-doc link resolution
pub fn render_enum_variants_docs<F>(
    md: &mut String,
    variants: &[Id],
    krate: &Crate,
    process_docs: F,
) where
    F: Fn(&Item) -> Option<String>,
{
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

            if let Some(docs) = process_docs(variant) {
                _ = write!(md, "\n\n  {}", docs.replace('\n', "\n  "));
            }

            md.push_str("\n\n");
        }
    }
}

/// Unit struct to ornagize trait related functions.
pub struct TraitRenderer;

impl TraitRenderer {
    /// Render a trait definition code block to markdown.
    ///
    /// Produces a heading with the trait name and generics, followed by a Rust
    /// code block showing the trait signature with supertraits.
    ///
    /// # Arguments
    ///
    /// * `md` - Output markdown string
    /// * `name` - The trait name
    /// * `t` - The trait data from rustdoc
    /// * `type_renderer` - Type renderer for generics and bounds
    pub fn render_trait_definition(
        md: &mut String,
        name: &str,
        t: &rustdoc_types::Trait,
        type_renderer: &TypeRenderer,
    ) {
        let generics = type_renderer.render_generics(&t.generics.params);
        let where_clause = type_renderer.render_where_clause(&t.generics.where_predicates);

        _ = write!(md, "### `{name}{generics}`\n\n");

        md.push_str("```rust\n");

        let bounds = if t.bounds.is_empty() {
            String::new()
        } else {
            let bound_strs: Vec<Cow<str>> = t
                .bounds
                .iter()
                .map(|b| type_renderer.render_generic_bound(b))
                .collect();
            format!(": {}", bound_strs.join(" + "))
        };

        _ = writeln!(md, "trait {name}{generics}{bounds}{where_clause} {{ ... }}");
        md.push_str("```\n\n");
    }

    /// Render a single trait item (method, associated type, or constant).
    ///
    /// Each item is rendered as a bullet point with its signature in backticks.
    /// For methods, the first line of documentation is included.
    ///
    /// # Arguments
    ///
    /// * `md` - Output markdown string
    /// * `item` - The trait item (function, assoc type, or assoc const)
    /// * `type_renderer` - Type renderer for types
    /// * `process_docs` - Closure to process documentation with intra-doc link resolution
    pub fn render_trait_item<F>(
        md: &mut String,
        item: &Item,
        type_renderer: &TypeRenderer,
        process_docs: F,
    ) where
        F: Fn(&Item) -> Option<String>,
    {
        let name = item.name.as_deref().unwrap_or("_");

        match &item.inner {
            ItemEnum::Function(f) => {
                let generics = type_renderer.render_generics(&f.generics.params);

                let params: Vec<String> = f
                    .sig
                    .inputs
                    .iter()
                    .map(|(param_name, ty)| {
                        let raw = format!("{param_name}: {}", type_renderer.render_type(ty));
                        sanitize_self_param(&raw).into_owned()
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

                if let Some(docs) = process_docs(item)
                    && let Some(first_line) = docs.lines().next()
                {
                    _ = write!(md, "\n\n  {first_line}");
                }

                md.push_str("\n\n");
            },

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
            },

            ItemEnum::AssocConst { type_, .. } => {
                _ = write!(
                    md,
                    "- `const {name}: {}`\n\n",
                    type_renderer.render_type(type_)
                );
            },

            _ => {
                _ = write!(md, "- `{name}`\n\n");
            },
        }
    }
}

/// Render a function definition to markdown.
///
/// Produces a heading with the function name, followed by a Rust code block
/// showing the full signature with modifiers (const, async, unsafe).
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `name` - The function name
/// * `f` - The function data from rustdoc
/// * `type_renderer` - Type renderer for parameter and return types
pub fn render_function_definition(
    md: &mut String,
    name: &str,
    f: &rustdoc_types::Function,
    type_renderer: &TypeRenderer,
) {
    let generics = type_renderer.render_generics(&f.generics.params);
    let where_clause = type_renderer.render_where_clause(&f.generics.where_predicates);

    let params: Vec<String> = f
        .sig
        .inputs
        .iter()
        .map(|(param_name, ty)| {
            let raw = format!("{param_name}: {}", type_renderer.render_type(ty));
            sanitize_self_param(&raw).into_owned()
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

/// Render a constant definition to markdown.
///
/// Produces a heading with the constant name, followed by a Rust code block
/// showing `const NAME: Type = value;`.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `name` - The constant name
/// * `type_` - The constant's type
/// * `const_` - The constant data including value
/// * `type_renderer` - Type renderer for the type
pub fn render_constant_definition(
    md: &mut String,
    name: &str,
    type_: &rustdoc_types::Type,
    const_: &rustdoc_types::Constant,
    type_renderer: &TypeRenderer,
) {
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

/// Render a type alias definition to markdown.
///
/// Produces a heading with the alias name and generics, followed by a Rust
/// code block showing `type Name<T> = TargetType;`.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `name` - The type alias name
/// * `ta` - The type alias data from rustdoc
/// * `type_renderer` - Type renderer for generics and the aliased type
pub fn render_type_alias_definition(
    md: &mut String,
    name: &str,
    ta: &rustdoc_types::TypeAlias,
    type_renderer: &TypeRenderer,
) {
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

/// Render a macro definition to markdown.
///
/// Produces a heading with the macro name and `!` suffix.
/// Note: We don't show macro rules since rustdoc JSON doesn't provide them.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `name` - The macro name
pub fn render_macro_heading(md: &mut String, name: &str) {
    _ = write!(md, "### `{name}!`\n\n");
}

/// Render the items within an impl block.
///
/// This renders all methods, associated constants, and associated types
/// within an impl block as bullet points.
///
/// # Arguments
///
/// * `md` - Output markdown string
/// * `impl_block` - The impl block to render items from
/// * `krate` - The crate containing item definitions
/// * `type_renderer` - Type renderer for types
/// * `process_docs` - Optional closure to process documentation
/// * `create_type_link` - Optional closure to create links for types `(id -> Option<markdown_link>)`
/// * `parent_type_name` - Optional type name for generating method anchors
pub fn render_impl_items<F, L>(
    md: &mut String,
    impl_block: &Impl,
    krate: &Crate,
    type_renderer: &TypeRenderer,
    process_docs: &Option<F>,
    create_type_link: &Option<L>,
    parent_type_name: Option<&str>,
) where
    F: Fn(&Item) -> Option<String>,
    L: Fn(rustdoc_types::Id) -> Option<String>,
{
    for item_id in &impl_block.items {
        if let Some(item) = krate.index.get(item_id) {
            let name = item.name.as_deref().unwrap_or("_");

            match &item.inner {
                ItemEnum::Function(f) => {
                    render_impl_function(md, name, f, *type_renderer, parent_type_name);

                    // Add type links if link creator is provided
                    if let Some(link_creator) = create_type_link {
                        render_function_type_links_inline(md, f, *type_renderer, link_creator);
                    }

                    // First line of docs as summary (with blank line before)
                    if let Some(pf) = process_docs
                        && let Some(docs) = pf(item)
                        && let Some(first_line) = docs.lines().next()
                    {
                        _ = write!(md, "\n\n  {first_line}");
                    }
                    md.push_str("\n\n");
                },

                ItemEnum::AssocConst { type_, .. } => {
                    // Add anchor for associated constants if parent type is known
                    if let Some(type_name) = parent_type_name {
                        let anchor = method_anchor(type_name, name);
                        _ = writeln!(
                            md,
                            "- <span id=\"{anchor}\"></span>`const {name}: {}`\n",
                            type_renderer.render_type(type_)
                        );
                    } else {
                        _ = writeln!(
                            md,
                            "- `const {name}: {}`\n",
                            type_renderer.render_type(type_)
                        );
                    }
                },

                ItemEnum::AssocType { type_, .. } => {
                    // Add anchor for associated types if parent type is known
                    let anchor_prefix = parent_type_name
                        .map(|tn| format!("<span id=\"{}\"></span>", method_anchor(tn, name)))
                        .unwrap_or_default();

                    if let Some(ty) = type_ {
                        _ = writeln!(
                            md,
                            "- {anchor_prefix}`type {name} = {}`\n",
                            type_renderer.render_type(ty)
                        );
                    } else {
                        _ = writeln!(md, "- {anchor_prefix}`type {name}`\n");
                    }
                },

                _ => {},
            }
        }
    }
}

/// Render type links for a function signature inline (for impl methods).
///
/// This is a helper that collects types from function signatures and
/// creates links for resolvable types, outputting them on the same line.
fn render_function_type_links_inline<L>(
    md: &mut String,
    f: &rustdoc_types::Function,
    type_renderer: TypeRenderer,
    create_link: &L,
) where
    L: Fn(rustdoc_types::Id) -> Option<String>,
{
    use std::collections::HashSet;

    let mut all_types = Vec::new();

    // Collect from parameters
    for (_, ty) in &f.sig.inputs {
        all_types.extend(type_renderer.collect_linkable_types(ty));
    }

    // Collect from return type
    if let Some(output) = &f.sig.output {
        all_types.extend(type_renderer.collect_linkable_types(output));
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
        .filter_map(|(_, id)| create_link(*id))
        .collect();

    // Output inline if we have links
    if !links.is_empty() {
        _ = write!(md, " — {}", links.join(", "));
    }
}

/// Render a function signature within an impl block.
///
/// Renders as a bullet point with the full signature including modifiers.
/// If `parent_type_name` is provided, includes a hidden anchor for deep linking.
fn render_impl_function(
    md: &mut String,
    name: &str,
    f: &rustdoc_types::Function,
    type_renderer: TypeRenderer,
    parent_type_name: Option<&str>,
) {
    let generics = type_renderer.render_generics(&f.generics.params);

    let params: Vec<String> = f
        .sig
        .inputs
        .iter()
        .map(|(param_name, ty)| {
            let raw = format!("{param_name}: {}", type_renderer.render_type(ty));
            sanitize_self_param(&raw).into_owned()
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

    // Add anchor for deep linking if parent type is known
    let anchor_span = parent_type_name
        .map(|tn| format!("<span id=\"{}\"></span>", method_anchor(tn, name)))
        .unwrap_or_default();

    _ = write!(
        md,
        "- {anchor_span}`{}{}{}fn {}{}({}){}`",
        is_const,
        is_async,
        is_unsafe,
        name,
        generics,
        params.join(", "),
        ret
    );
}

/// Append processed documentation to markdown.
///
/// Helper function to add documentation with consistent formatting.
pub fn append_docs(md: &mut String, docs: Option<String>) {
    if let Some(docs) = docs {
        md.push_str(&docs);
        md.push_str("\n\n");
    }
}

/// Render the opening of a collapsible `<details>` block with a summary.
///
/// Produces HTML that creates a collapsible section in markdown. Use with
/// [`render_collapsible_end`] to close the block.
///
/// # Arguments
///
/// * `summary` - The text to display in the summary line (clickable header)
///
/// # Example
///
/// ```
/// use cargo_docs_md::generator::render_shared::render_collapsible_start;
///
/// let start = render_collapsible_start("Derived Traits (9 implementations)");
/// assert!(start.contains("<details>"));
/// assert!(start.contains("<summary>Derived Traits (9 implementations)</summary>"));
/// ```
#[must_use]
pub fn render_collapsible_start(summary: &str) -> String {
    format!("<details>\n<summary>{summary}</summary>\n\n")
}

/// Render the closing of a collapsible `<details>` block.
///
/// Returns a static string to close a block opened with [`render_collapsible_start`].
///
/// # Example
///
/// ```
/// use cargo_docs_md::generator::render_shared::render_collapsible_end;
///
/// assert_eq!(render_collapsible_end(), "\n</details>\n\n");
/// ```
#[must_use]
pub const fn render_collapsible_end() -> &'static str {
    "\n</details>\n\n"
}

/// Generate a sort key for an impl block for deterministic ordering.
///
/// Combines trait name, generic params, and for-type to create a unique key.
#[must_use]
pub fn impl_sort_key(impl_block: &Impl, type_renderer: &TypeRenderer) -> String {
    let trait_name = impl_block
        .trait_
        .as_ref()
        .map(|t| t.path.clone())
        .unwrap_or_default();
    let for_type = type_renderer.render_type(&impl_block.for_);
    let generics = type_renderer.render_generics(&impl_block.generics.params);
    format!("{trait_name}{generics}::{for_type}")
}

/// Check if a render context can resolve documentation.
///
/// This trait provides a unified way to process docs from different contexts.
pub trait DocsProcessor {
    /// Process documentation for an item, resolving intra-doc links.
    fn process_item_docs(&self, item: &Item) -> Option<String>;
}

impl<T: RenderContext + ?Sized> DocsProcessor for (&T, &str) {
    fn process_item_docs(&self, item: &Item) -> Option<String> {
        self.0.process_docs(item, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod sanitize_path_tests {
        use super::*;

        #[test]
        fn removes_crate_prefix() {
            assert_eq!(sanitize_path("$crate::clone::Clone"), "clone::Clone");
        }

        #[test]
        fn removes_multiple_crate_prefixes() {
            assert_eq!(
                sanitize_path("$crate::foo::$crate::bar::Baz"),
                "foo::bar::Baz"
            );
        }

        #[test]
        fn preserves_normal_paths() {
            assert_eq!(sanitize_path("std::fmt::Debug"), "std::fmt::Debug");
        }

        #[test]
        fn preserves_simple_names() {
            assert_eq!(sanitize_path("Clone"), "Clone");
        }

        #[test]
        fn handles_empty_string() {
            assert_eq!(sanitize_path(""), "");
        }

        #[test]
        fn returns_borrowed_when_no_change() {
            let result = sanitize_path("std::fmt::Debug");
            assert!(matches!(result, Cow::Borrowed(_)));
        }

        #[test]
        fn returns_owned_when_changed() {
            let result = sanitize_path("$crate::Clone");
            assert!(matches!(result, Cow::Owned(_)));
        }
    }

    mod sanitize_self_param_tests {
        use super::*;

        #[test]
        fn converts_ref_self() {
            assert_eq!(sanitize_self_param("self: &Self"), "&self");
        }

        #[test]
        fn converts_mut_ref_self() {
            assert_eq!(sanitize_self_param("self: &mut Self"), "&mut self");
        }

        #[test]
        fn converts_owned_self() {
            assert_eq!(sanitize_self_param("self: Self"), "self");
        }

        #[test]
        fn preserves_regular_params() {
            assert_eq!(sanitize_self_param("x: i32"), "x: i32");
        }

        #[test]
        fn preserves_complex_types() {
            assert_eq!(
                sanitize_self_param("callback: impl Fn()"),
                "callback: impl Fn()"
            );
        }

        #[test]
        fn returns_borrowed_for_all_cases() {
            // All return values should be borrowed (no allocation)
            assert!(matches!(
                sanitize_self_param("self: &Self"),
                Cow::Borrowed(_)
            ));
            assert!(matches!(
                sanitize_self_param("self: &mut Self"),
                Cow::Borrowed(_)
            ));
            assert!(matches!(
                sanitize_self_param("self: Self"),
                Cow::Borrowed(_)
            ));
            assert!(matches!(sanitize_self_param("x: i32"), Cow::Borrowed(_)));
        }
    }

    mod collapsible_tests {
        use super::*;

        #[test]
        fn start_contains_details_tag() {
            let result = render_collapsible_start("Test Summary");
            assert!(result.contains("<details>"));
        }

        #[test]
        fn start_contains_summary_with_text() {
            let result = render_collapsible_start("Derived Traits (9 implementations)");
            assert!(result.contains("<summary>Derived Traits (9 implementations)</summary>"));
        }

        #[test]
        fn start_has_proper_formatting() {
            let result = render_collapsible_start("Test");
            assert_eq!(result, "<details>\n<summary>Test</summary>\n\n");
        }

        #[test]
        fn end_closes_details_tag() {
            let result = render_collapsible_end();
            assert!(result.contains("</details>"));
        }

        #[test]
        fn end_has_proper_formatting() {
            assert_eq!(render_collapsible_end(), "\n</details>\n\n");
        }

        #[test]
        fn start_and_end_pair_correctly() {
            let start = render_collapsible_start("Content");
            let end = render_collapsible_end();
            let full = format!("{start}Some markdown content here{end}");

            assert!(full.starts_with("<details>"));
            assert!(full.ends_with("</details>\n\n"));
            assert!(full.contains("<summary>Content</summary>"));
        }
    }

    mod categorized_trait_items_tests {
        use super::*;
        use rustdoc_types::{Abi, Crate, Function, FunctionHeader, FunctionSignature, Target};
        use std::collections::HashMap;

        fn make_test_crate(items: Vec<(Id, Item)>) -> Crate {
            let mut index: HashMap<Id, Item> = HashMap::new();
            for (id, item) in items {
                index.insert(id, item);
            }

            Crate {
                root: Id(0),
                crate_version: None,
                includes_private: false,
                index,
                paths: HashMap::new(),
                external_crates: HashMap::new(),
                format_version: 0,
                target: Target {
                    triple: String::new(),
                    target_features: vec![],
                },
            }
        }

        fn make_function_item(name: &str, has_body: bool) -> Item {
            Item {
                id: Id(0),
                crate_id: 0,
                name: Some(name.to_string()),
                attrs: vec![],
                visibility: Visibility::Public,
                inner: ItemEnum::Function(Function {
                    sig: FunctionSignature {
                        inputs: vec![],
                        output: None,
                        is_c_variadic: false,
                    },
                    generics: rustdoc_types::Generics {
                        params: vec![],
                        where_predicates: vec![],
                    },
                    header: FunctionHeader {
                        is_const: false,
                        is_async: false,
                        is_unsafe: false,
                        abi: Abi::Rust,
                    },
                    has_body,
                }),
                deprecation: None,
                docs: None,
                span: None,
                links: HashMap::new(),
            }
        }

        fn make_assoc_type_item(name: &str) -> Item {
            Item {
                id: Id(0),
                crate_id: 0,
                name: Some(name.to_string()),
                attrs: vec![],
                visibility: Visibility::Public,
                inner: ItemEnum::AssocType {
                    generics: rustdoc_types::Generics {
                        params: vec![],
                        where_predicates: vec![],
                    },
                    bounds: vec![],
                    type_: None,
                },
                deprecation: None,
                docs: None,
                span: None,
                links: HashMap::new(),
            }
        }

        fn make_assoc_const_item(name: &str) -> Item {
            Item {
                id: Id(0),
                crate_id: 0,
                name: Some(name.to_string()),
                attrs: vec![],
                visibility: Visibility::Public,
                inner: ItemEnum::AssocConst {
                    type_: rustdoc_types::Type::Primitive("i32".to_string()),
                    value: Some("42".to_string()),
                },
                deprecation: None,
                docs: None,
                span: None,
                links: HashMap::new(),
            }
        }

        #[test]
        fn empty_trait_items() {
            let krate = make_test_crate(vec![]);
            let result = CategorizedTraitItems::categorize_trait_items(&[], &krate);

            assert!(result.required_methods.is_empty());
            assert!(result.provided_methods.is_empty());
            assert!(result.associated_types.is_empty());
            assert!(result.associated_consts.is_empty());
        }

        #[test]
        fn categorizes_required_method() {
            let id = Id(1);
            let item = make_function_item("required_fn", false);
            let krate = make_test_crate(vec![(id, item)]);

            let result = CategorizedTraitItems::categorize_trait_items(&[id], &krate);

            assert_eq!(result.required_methods.len(), 1);
            assert_eq!(result.required_methods[0].name.as_deref(), Some("required_fn"));
            assert!(result.provided_methods.is_empty());
        }

        #[test]
        fn categorizes_provided_method() {
            let id = Id(1);
            let item = make_function_item("provided_fn", true);
            let krate = make_test_crate(vec![(id, item)]);

            let result = CategorizedTraitItems::categorize_trait_items(&[id], &krate);

            assert!(result.required_methods.is_empty());
            assert_eq!(result.provided_methods.len(), 1);
            assert_eq!(result.provided_methods[0].name.as_deref(), Some("provided_fn"));
        }

        #[test]
        fn categorizes_associated_type() {
            let id = Id(1);
            let item = make_assoc_type_item("Item");
            let krate = make_test_crate(vec![(id, item)]);

            let result = CategorizedTraitItems::categorize_trait_items(&[id], &krate);

            assert_eq!(result.associated_types.len(), 1);
            assert_eq!(result.associated_types[0].name.as_deref(), Some("Item"));
        }

        #[test]
        fn categorizes_associated_const() {
            let id = Id(1);
            let item = make_assoc_const_item("CONST");
            let krate = make_test_crate(vec![(id, item)]);

            let result = CategorizedTraitItems::categorize_trait_items(&[id], &krate);

            assert_eq!(result.associated_consts.len(), 1);
            assert_eq!(result.associated_consts[0].name.as_deref(), Some("CONST"));
        }

        #[test]
        fn categorizes_mixed_items() {
            let req_id = Id(1);
            let prov_id = Id(2);
            let type_id = Id(3);
            let const_id = Id(4);

            let krate = make_test_crate(vec![
                (req_id, make_function_item("req", false)),
                (prov_id, make_function_item("prov", true)),
                (type_id, make_assoc_type_item("Output")),
                (const_id, make_assoc_const_item("MAX")),
            ]);

            let result = CategorizedTraitItems::categorize_trait_items(
                &[req_id, prov_id, type_id, const_id],
                &krate,
            );

            assert_eq!(result.required_methods.len(), 1);
            assert_eq!(result.provided_methods.len(), 1);
            assert_eq!(result.associated_types.len(), 1);
            assert_eq!(result.associated_consts.len(), 1);
        }

        #[test]
        fn skips_missing_items() {
            let existing_id = Id(1);
            let missing_id = Id(99);
            let krate = make_test_crate(vec![(existing_id, make_function_item("fn", false))]);

            let result =
                CategorizedTraitItems::categorize_trait_items(&[existing_id, missing_id], &krate);

            // Should have one item, missing ID is skipped
            assert_eq!(result.required_methods.len(), 1);
        }
    }
}
