//! Implementation block rendering for documentation generation.
//!
//! This module provides the [`ImplRenderer`] struct which handles rendering
//! impl blocks (both inherent and trait implementations) to markdown format.

use std::borrow::Cow;
use std::fmt::Write;

use rustdoc_types::{Id, Impl, Item};

use crate::generator::context::RenderContext;
use crate::generator::render_shared::{impl_sort_key, render_impl_items};
use crate::types::TypeRenderer;

/// Blanket trait implementations to filter from output.
///
/// These are automatically derived by the compiler and add noise to documentation
/// without providing useful information. Users who want them can use `--include-blanket-impls`.
const BLANKET_TRAITS: &[&str] = &[
    // Identity/conversion traits (blanket impls)
    "From",
    "Into",
    "TryFrom",
    "TryInto",
    // Reflection
    "Any",
    // Borrowing (trivial impls)
    "Borrow",
    "BorrowMut",
    // Ownership
    "ToOwned",
    "CloneToUninit",
];

/// Check if an impl block is for a blanket trait that should be filtered.
///
/// Returns `true` if the impl is for one of the commonly auto-derived traits
/// that add noise to documentation (From, Into, Any, Borrow, etc.).
#[must_use]
pub fn is_blanket_impl(impl_block: &Impl) -> bool {
    let Some(trait_ref) = &impl_block.trait_ else {
        return false;
    };

    // Extract the trait name (last segment of the path)
    let trait_name = trait_ref
        .path
        .split("::")
        .last()
        .unwrap_or(&trait_ref.path);

    BLANKET_TRAITS.contains(&trait_name)
}

/// Renders impl blocks to markdown.
///
/// This struct handles:
/// - Inherent implementations (`impl MyType { ... }`)
/// - Trait implementations (`impl Trait for MyType { ... }`)
/// - Method signatures within impl blocks
/// - Associated types and constants
///
/// The renderer is generic over [`RenderContext`], allowing it to work with
/// both single-crate (`GeneratorContext`) and multi-crate (`SingleCrateView`) modes.
pub struct ImplRenderer<'a> {
    /// Reference to the render context (either single-crate or multi-crate).
    ctx: &'a dyn RenderContext,

    /// Path of the current file being generated (for relative link calculation).
    current_file: &'a str,

    /// Cached type renderer to avoid repeated construction.
    type_renderer: TypeRenderer<'a>,
}

impl<'a> ImplRenderer<'a> {
    /// Create a new impl renderer with the given context.
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

    /// Render impl blocks for a given type.
    ///
    /// This method looks up all impl blocks for a type and renders them
    /// in two sections:
    ///
    /// 1. **Implementations** - Inherent impls (methods defined directly on the type)
    /// 2. **Trait Implementations** - Trait impls (`impl Trait for Type`)
    ///
    /// # Impl Block Categories
    ///
    /// - **Inherent**: `impl MyType { fn method(&self) {} }`
    /// - **Trait**: `impl Clone for MyType { ... }`
    /// - **Synthetic**: Auto-derived by compiler (Send, Sync) - skipped
    pub fn render_impl_blocks(&self, md: &mut String, item_id: Id) {
        let Some(impls) = self.ctx.get_impls(&item_id) else {
            return;
        };

        // Partition impls into trait and inherent
        let (mut trait_impls, inherent_impls): (Vec<_>, Vec<_>) =
            impls.iter().partition(|i| i.trait_.is_some());

        // Sort trait impls by trait name + generics for deterministic output
        trait_impls.sort_by(|a: &&&Impl, b: &&&Impl| {
            let key_a = impl_sort_key(a, &self.type_renderer);
            let key_b = impl_sort_key(b, &self.type_renderer);
            key_a.cmp(&key_b)
        });

        // Deduplicate trait impls with the same signature
        trait_impls.dedup_by(|a, b| {
            impl_sort_key(a, &self.type_renderer) == impl_sort_key(b, &self.type_renderer)
        });

        // === Inherent Implementations ===
        if !inherent_impls.is_empty() {
            md.push_str("#### Implementations\n\n");
            for impl_block in inherent_impls {
                self.render_impl_methods(md, impl_block);
            }
        }

        // === Trait Implementations ===
        // Filter out blanket impls (From, Into, Any, etc.) unless explicitly included
        let filtered_trait_impls: Vec<_> = if self.ctx.include_blanket_impls() {
            trait_impls
        } else {
            trait_impls
                .into_iter()
                .filter(|i| !is_blanket_impl(i))
                .collect()
        };

        if !filtered_trait_impls.is_empty() {
            md.push_str("#### Trait Implementations\n\n");
            for impl_block in filtered_trait_impls {
                self.render_trait_impl(md, impl_block);
            }
        }
    }

    /// Render a single trait implementation block.
    fn render_trait_impl(&self, md: &mut String, impl_block: &Impl) {
        // Skip synthetic impls (auto-traits like Send, Sync, Unpin)
        if impl_block.is_synthetic {
            return;
        }

        // Build the trait name with generic args
        let trait_name = impl_block
            .trait_
            .as_ref()
            .map(|t| {
                let mut name = t.path.clone();
                if let Some(args) = &t.args {
                    name.push_str(&self.render_generic_args_for_impl(args));
                }
                name
            })
            .unwrap_or_default();

        let generics = self
            .type_renderer
            .render_generics(&impl_block.generics.params);
        let for_type = self.type_renderer.render_type(&impl_block.for_);

        let unsafe_str = if impl_block.is_unsafe { "unsafe " } else { "" };
        let negative_str = if impl_block.is_negative { "!" } else { "" };

        _ = write!(
            md,
            "##### `{unsafe_str}impl{generics} {negative_str}{trait_name} for {for_type}`\n\n"
        );

        self.render_impl_methods(md, impl_block);
    }

    /// Render the items (methods, constants, types) within an impl block.
    ///
    /// Each item is rendered as a bullet point. Items can be:
    /// - **Functions/Methods**: Full signature with modifiers
    /// - **Associated Constants**: `const NAME: Type`
    /// - **Associated Types**: `type Name = Type`
    ///
    /// For methods, the first line of documentation is included as a brief summary.
    /// Type links are added for resolvable types in method signatures.
    fn render_impl_methods(&self, md: &mut String, impl_block: &Impl) {
        let krate = self.ctx.krate();
        render_impl_items(
            md,
            impl_block,
            krate,
            &self.type_renderer,
            Some(|item: &Item| self.process_docs(item)),
            Some(|id: rustdoc_types::Id| self.ctx.create_link(id, self.current_file)),
        );
    }

    /// Render generic arguments for impl block signatures.
    ///
    /// This handles the different forms of generic arguments:
    /// - **Angle bracketed**: `<T, U, Item = V>` (most common)
    /// - **Parenthesized**: `(A, B) -> C` (for Fn traits)
    /// - **Return type notation**: `(..)` (experimental)
    fn render_generic_args_for_impl(&self, args: &rustdoc_types::GenericArgs) -> String {
        match args {
            rustdoc_types::GenericArgs::AngleBracketed { args, constraints } => {
                let mut parts: Vec<Cow<str>> = args
                    .iter()
                    .map(|a| match a {
                        rustdoc_types::GenericArg::Lifetime(lt) => Cow::Borrowed(lt.as_str()),
                        rustdoc_types::GenericArg::Type(ty) => self.type_renderer.render_type(ty),
                        rustdoc_types::GenericArg::Const(c) => Cow::Borrowed(
                            c.value.as_deref().unwrap_or(&c.expr),
                        ),
                        rustdoc_types::GenericArg::Infer => Cow::Borrowed("_"),
                    })
                    .collect();

                parts.extend(constraints.iter().map(|c| {
                    let constraint_args = c
                        .args
                        .as_ref()
                        .map(|a| self.render_generic_args_for_impl(a))
                        .unwrap_or_default();

                    match &c.binding {
                        rustdoc_types::AssocItemConstraintKind::Equality(term) => {
                            let term_str = match term {
                                rustdoc_types::Term::Type(ty) => {
                                    self.type_renderer.render_type(ty)
                                },
                                rustdoc_types::Term::Constant(c) => Cow::Borrowed(
                                    c.value.as_deref().unwrap_or(&c.expr),
                                ),
                            };
                            Cow::Owned(format!("{}{constraint_args} = {term_str}", c.name))
                        },
                        rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                            let bound_strs: Vec<Cow<str>> = bounds
                                .iter()
                                .map(|b| self.type_renderer.render_generic_bound(b))
                                .collect();
                            Cow::Owned(format!(
                                "{}{constraint_args}: {}",
                                c.name,
                                bound_strs.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(" + ")
                            ))
                        },
                    }
                }));

                if parts.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<{}>",
                        parts.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(", ")
                    )
                }
            },

            rustdoc_types::GenericArgs::Parenthesized { inputs, output } => {
                let input_strs: Vec<Cow<str>> = inputs
                    .iter()
                    .map(|t| self.type_renderer.render_type(t))
                    .collect();
                let ret = output
                    .as_ref()
                    .map(|t| format!(" -> {}", self.type_renderer.render_type(t)))
                    .unwrap_or_default();
                format!(
                    "({}){ret}",
                    input_strs.iter().map(|s| s.as_ref()).collect::<Vec<_>>().join(", ")
                )
            },

            rustdoc_types::GenericArgs::ReturnTypeNotation => " (..)".to_string(),
        }
    }
}
