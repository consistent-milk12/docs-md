//! Implementation block rendering for documentation generation.
//!
//! This module provides the [`ImplRenderer`] struct which handles rendering
//! impl blocks (both inherent and trait implementations) to markdown format.

use crate::generator::context::GeneratorContext;
use crate::types::{render_generic_bound, render_generics, render_type};
use rustdoc_types::{Id, Impl, ItemEnum};
use std::fmt::Write;

/// Renders impl blocks to markdown.
///
/// This struct handles:
/// - Inherent implementations (`impl MyType { ... }`)
/// - Trait implementations (`impl Trait for MyType { ... }`)
/// - Method signatures within impl blocks
/// - Associated types and constants
pub struct ImplRenderer<'a> {
    /// Reference to the shared generator context.
    ctx: &'a GeneratorContext<'a>,
}

impl<'a> ImplRenderer<'a> {
    /// Create a new impl renderer with the given context.
    pub fn new(ctx: &'a GeneratorContext<'a>) -> Self {
        Self { ctx }
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
        let Some(impls) = self.ctx.impl_map.get(&item_id) else {
            return;
        };

        // Partition impls into trait and inherent
        let (trait_impls, inherent_impls): (Vec<_>, Vec<_>) =
            impls.iter().partition(|i| i.trait_.is_some());

        // === Inherent Implementations ===
        if !inherent_impls.is_empty() {
            md.push_str("#### Implementations\n\n");
            for impl_block in inherent_impls {
                self.render_impl_methods(md, impl_block);
            }
        }

        // === Trait Implementations ===
        if !trait_impls.is_empty() {
            md.push_str("#### Trait Implementations\n\n");
            for impl_block in trait_impls {
                self.render_trait_impl(md, impl_block);
            }
        }
    }

    /// Render a single trait implementation block.
    fn render_trait_impl(&self, md: &mut String, impl_block: &Impl) {
        let krate = self.ctx.krate;

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

        let generics = render_generics(&impl_block.generics.params, krate);
        let for_type = render_type(&impl_block.for_, krate);

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
    fn render_impl_methods(&self, md: &mut String, impl_block: &Impl) {
        let krate = self.ctx.krate;

        for item_id in &impl_block.items {
            if let Some(item) = krate.index.get(item_id) {
                match &item.inner {
                    ItemEnum::Function(f) => {
                        self.render_impl_function(md, item, f);
                    }

                    ItemEnum::AssocConst { type_, .. } => {
                        let name = item.name.as_deref().unwrap_or("_");
                        _ = write!(md, "- `const {}: {}`\n\n", name, render_type(type_, krate));
                    }

                    ItemEnum::AssocType { type_, .. } => {
                        let name = item.name.as_deref().unwrap_or("_");

                        if let Some(ty) = type_ {
                            _ = write!(md, "- `type {} = {}`\n\n", name, render_type(ty, krate));
                        } else {
                            _ = write!(md, "- `type {name}`\n\n");
                        }
                    }

                    _ => {}
                }
            }
        }
    }

    /// Render a function/method within an impl block.
    fn render_impl_function(
        &self,
        md: &mut String,
        item: &rustdoc_types::Item,
        f: &rustdoc_types::Function,
    ) {
        let krate = self.ctx.krate;
        let name = item.name.as_deref().unwrap_or("_");
        let generics = render_generics(&f.generics.params, krate);

        let params: Vec<String> = f
            .sig
            .inputs
            .iter()
            .map(|(param_name, ty)| format!("{}: {}", param_name, render_type(ty, krate)))
            .collect();

        let ret = f
            .sig
            .output
            .as_ref()
            .map(|ty| format!(" -> {}", render_type(ty, krate)))
            .unwrap_or_default();

        let is_async = if f.header.is_async { "async " } else { "" };
        let is_const = if f.header.is_const { "const " } else { "" };
        let is_unsafe = if f.header.is_unsafe { "unsafe " } else { "" };

        _ = write!(
            md,
            "- `{}{}{}fn {}{}({}){}`",
            is_const,
            is_async,
            is_unsafe,
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

    /// Render generic arguments for impl block signatures.
    ///
    /// This handles the different forms of generic arguments:
    /// - **Angle bracketed**: `<T, U, Item = V>` (most common)
    /// - **Parenthesized**: `(A, B) -> C` (for Fn traits)
    /// - **Return type notation**: `(..)` (experimental)
    fn render_generic_args_for_impl(&self, args: &rustdoc_types::GenericArgs) -> String {
        let krate = self.ctx.krate;

        match args {
            rustdoc_types::GenericArgs::AngleBracketed { args, constraints } => {
                let mut parts: Vec<String> = args
                    .iter()
                    .map(|a| match a {
                        rustdoc_types::GenericArg::Lifetime(lt) => lt.clone(),
                        rustdoc_types::GenericArg::Type(ty) => render_type(ty, krate),
                        rustdoc_types::GenericArg::Const(c) => {
                            c.value.clone().unwrap_or_else(|| c.expr.clone())
                        }
                        rustdoc_types::GenericArg::Infer => "_".to_string(),
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
                                rustdoc_types::Term::Type(ty) => render_type(ty, krate),
                                rustdoc_types::Term::Constant(c) => {
                                    c.value.clone().unwrap_or_else(|| c.expr.clone())
                                }
                            };
                            format!("{}{} = {}", c.name, constraint_args, term_str)
                        }
                        rustdoc_types::AssocItemConstraintKind::Constraint(bounds) => {
                            let bound_strs: Vec<String> = bounds
                                .iter()
                                .map(|b| render_generic_bound(b, krate))
                                .collect();
                            format!("{}{}: {}", c.name, constraint_args, bound_strs.join(" + "))
                        }
                    }
                }));

                if parts.is_empty() {
                    String::new()
                } else {
                    format!("<{}>", parts.join(", "))
                }
            }

            rustdoc_types::GenericArgs::Parenthesized { inputs, output } => {
                let input_strs: Vec<String> =
                    inputs.iter().map(|t| render_type(t, krate)).collect();
                let ret = output
                    .as_ref()
                    .map(|t| format!(" -> {}", render_type(t, krate)))
                    .unwrap_or_default();
                format!("({}){}", input_strs.join(", "), ret)
            }

            rustdoc_types::GenericArgs::ReturnTypeNotation => " (..)".to_string(),
        }
    }
}
