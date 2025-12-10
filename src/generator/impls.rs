//! Implementation block rendering for documentation generation.
//!
//! This module provides the [`ImplRenderer`] struct which handles rendering
//! impl blocks (both inherent and trait implementations) to markdown format.
//!
//! # Path Types in `rustdoc_types::Impl`
//!
//! When working with impl blocks, there are two different path representations:
//!
//! - **`Impl.trait_: Option<Path>`** - The trait being implemented
//!   - `Path.path` is a `String` like `"Clone"` or `"std::fmt::Debug"`
//!   - Use for display/rendering in documentation output
//!   - Extract trait name with `path.rsplit("::").next()`
//!
//! - **`ItemSummary.path: Vec<String>`** - Structured path from `krate.paths`
//!   - Example: `["std", "clone", "Clone"]`
//!   - Use for lookups and resolution via item IDs
//!
//! The `Path.path` string is already formatted for display, while
//! `ItemSummary.path` is structured for programmatic manipulation.

use std::borrow::Cow;
use std::fmt::Write;

use rustdoc_types::{Id, Impl, Item, Type};

use crate::generator::context::RenderContext;
use crate::generator::render_shared::{
    impl_sort_key, render_collapsible_end, render_collapsible_start, render_impl_items,
    sanitize_path,
};
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

/// Trivial derive trait implementations that can be collapsed.
///
/// These are traits commonly derived via `#[derive(...)]` that have standard,
/// predictable implementations. When `RenderConfig.hide_trivial_derives` is enabled,
/// these are grouped into a collapsible `<details>` block with a summary table.
///
/// The list includes:
/// - **Cloning**: `Clone`, `Copy`
/// - **Formatting**: `Debug`
/// - **Default values**: `Default`
/// - **Equality**: `PartialEq`, `Eq`
/// - **Hashing**: `Hash`
/// - **Ordering**: `PartialOrd`, `Ord`
pub const TRIVIAL_DERIVE_TRAITS: &[&str] = &[
    // Cloning traits
    "Clone",
    "Copy",
    // Formatting
    "Debug",
    // Default values
    "Default",
    // Equality comparison
    "PartialEq",
    "Eq",
    // Hashing
    "Hash",
    // Ordering comparison
    "PartialOrd",
    "Ord",
];

/// Short descriptions for trivial derive traits, used in summary tables.
///
/// Maps trait names to brief descriptions for the collapsible summary table.
pub const TRIVIAL_DERIVE_DESCRIPTIONS: &[(&str, &str)] = &[
    ("Clone", "Returns a copy of the value"),
    ("Copy", "Marker trait for types with copy semantics"),
    ("Debug", "Formats the value for debugging"),
    ("Default", "Returns the default value"),
    ("PartialEq", "Compares for equality"),
    ("Eq", "Marker for total equality"),
    ("Hash", "Feeds this value into a Hasher"),
    ("PartialOrd", "Partial ordering comparison"),
    ("Ord", "Total ordering comparison"),
];

/// Check if an impl block is for a trivial derive trait.
///
/// Returns `true` if the impl is for one of the commonly derived traits
/// `(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)`.
///
/// # Examples
///
/// ```
/// use rustdoc_types::Impl;
/// // For an impl block with trait "Clone", returns true
/// // For an impl block with trait "Display", returns false
/// ```
#[must_use]
pub fn is_trivial_derive_impl(impl_block: &Impl) -> bool {
    let Some(trait_ref) = &impl_block.trait_ else {
        return false;
    };

    // Extract the trait name (last segment of the path)
    // Using rsplit().next() is more efficient than split().last()
    let trait_name = trait_ref
        .path
        .rsplit("::")
        .next()
        .unwrap_or(&trait_ref.path);

    TRIVIAL_DERIVE_TRAITS.contains(&trait_name)
}

/// Get the description for a trivial derive trait.
///
/// Returns `None` if the trait is not in the trivial derives list.
#[must_use]
pub fn get_trivial_derive_description(trait_name: &str) -> Option<&'static str> {
    // Extract just the trait name if it's a full path
    // Using rsplit().next() is more efficient than split().last()
    let name = trait_name.rsplit("::").next().unwrap_or(trait_name);

    TRIVIAL_DERIVE_DESCRIPTIONS
        .iter()
        .find(|(t, _)| *t == name)
        .map(|(_, desc)| *desc)
}

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
    // Using rsplit().next() is more efficient than split().last()
    let trait_name = trait_ref
        .path
        .rsplit("::")
        .next()
        .unwrap_or(&trait_ref.path);

    BLANKET_TRAITS.contains(&trait_name)
}

/// Check if a type is generic (contains a type parameter like `T`).
///
/// This is used to determine whether to show generic parameters in impl blocks.
/// For blanket impls like `impl<T> Trait for T`, we only show the generics if
/// the `for_` type is actually generic. When the impl is instantiated for a
/// concrete type like `TocEntry`, we hide the generics.
///
/// # Examples
///
/// ```text
/// // Generic type - returns true
/// is_generic_type(&Type::Generic("T")) == true
///
/// // Concrete type - returns false
/// is_generic_type(&Type::ResolvedPath { name: "TocEntry", .. }) == false
///
/// // Container with generic - returns true
/// is_generic_type(&Type::ResolvedPath {
///     name: "Vec",
///     args: Some(GenericArgs::AngleBracketed { args: [Type::Generic("T")] })
/// }) == true
/// ```
#[must_use]
pub fn is_generic_type(ty: &Type) -> bool {
    match ty {
        // Direct generic parameter like `T`
        Type::Generic(_) => true,

        // Check nested types for generic parameters
        Type::ResolvedPath(path) => {
            // Check generic args of the path
            path.args
                .as_ref()
                .is_some_and(|args| generic_args_contain_generic(args))
        },

        // References, raw pointers, and arrays: check the inner type
        Type::BorrowedRef { type_, .. }
        | Type::RawPointer { type_, .. }
        | Type::Array { type_, .. } => is_generic_type(type_),

        // Slices: check the element type
        Type::Slice(inner) => is_generic_type(inner),

        // Tuples: check all element types
        Type::Tuple(types) => types.iter().any(is_generic_type),

        // Qualified paths like `<T as Trait>::Item`
        Type::QualifiedPath { self_type, .. } => is_generic_type(self_type),

        // Primitives and other types are not generic
        _ => false,
    }
}

/// Check if generic args contain any generic type parameters.
fn generic_args_contain_generic(args: &rustdoc_types::GenericArgs) -> bool {
    match args {
        rustdoc_types::GenericArgs::AngleBracketed { args, .. } => {
            args.iter().any(|arg| match arg {
                rustdoc_types::GenericArg::Type(ty) => is_generic_type(ty),
                rustdoc_types::GenericArg::Const(_)
                | rustdoc_types::GenericArg::Lifetime(_)
                | rustdoc_types::GenericArg::Infer => false,
            })
        },
        rustdoc_types::GenericArgs::Parenthesized { inputs, output } => {
            inputs.iter().any(is_generic_type) || output.as_ref().is_some_and(is_generic_type)
        },
        rustdoc_types::GenericArgs::ReturnTypeNotation => false,
    }
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
            _ = write!(md, "#### Implementations\n\n");
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

        if filtered_trait_impls.is_empty() {
            return;
        }

        _ = write!(md, "#### Trait Implementations\n\n");

        // Check if we should collapse trivial derives
        let hide_trivial = self.ctx.render_config().hide_trivial_derives;

        if hide_trivial {
            // Partition into trivial derives and non-trivial impls
            let (trivial_impls, other_impls): (Vec<_>, Vec<_>) = filtered_trait_impls
                .into_iter()
                .partition(|i| is_trivial_derive_impl(i));

            // Render trivial derives in a collapsible block
            if !trivial_impls.is_empty() {
                Self::render_trivial_derives_collapsed(md, &trivial_impls);
            }

            // Render non-trivial trait impls normally
            for impl_block in other_impls {
                self.render_trait_impl(md, impl_block);
            }
        } else {
            // Render all trait impls normally (no collapsing)
            for impl_block in filtered_trait_impls {
                self.render_trait_impl(md, impl_block);
            }
        }
    }

    /// Render trivial derive trait implementations in a collapsible block.
    ///
    /// Groups `Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord`
    /// implementations into a `<details>` block with a summary table.
    fn render_trivial_derives_collapsed(md: &mut String, impls: &[&&Impl]) {
        let count = impls.len();
        let summary = format!("Derived Traits ({count} implementations)");

        _ = write!(md, "{}", render_collapsible_start(&summary));

        // Render as a summary table
        _ = write!(
            md,
            "| Trait | Description |\n\
             | ----- | ----------- |\n"
        );

        for impl_block in impls {
            if let Some(trait_ref) = &impl_block.trait_ {
                let trait_name = trait_ref
                    .path
                    .rsplit("::")
                    .next()
                    .unwrap_or(&trait_ref.path);
                let description =
                    get_trivial_derive_description(trait_name).unwrap_or("Derived trait");
                _ = writeln!(md, "| `{trait_name}` | {description} |");
            }
        }

        _ = write!(md, "{}", render_collapsible_end());
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
                let mut name = sanitize_path(&t.path).into_owned();
                if let Some(args) = &t.args {
                    name.push_str(&self.render_generic_args_for_impl(args));
                }
                name
            })
            .unwrap_or_default();

        // Only show generic parameters if the `for_` type is itself generic.
        // For concrete types like `TocEntry`, we don't want to show `impl<T>` even
        // if the original blanket impl was defined as `impl<T> Trait for T`.
        let generics = if is_generic_type(&impl_block.for_) {
            self.type_renderer
                .render_generics(&impl_block.generics.params)
        } else {
            String::new()
        };
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
    /// Method anchors are generated for deep linking (e.g., `#typename-methodname`).
    fn render_impl_methods(&self, md: &mut String, impl_block: &Impl) {
        let krate = self.ctx.krate();

        // Extract the type name for method anchor generation
        let type_name = self.type_renderer.render_type(&impl_block.for_);

        render_impl_items(
            md,
            impl_block,
            krate,
            &self.type_renderer,
            &Some(|item: &Item| self.process_docs(item)),
            &Some(|id: rustdoc_types::Id| self.ctx.create_link(id, self.current_file)),
            Some(type_name.as_ref()),
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
                        rustdoc_types::GenericArg::Const(c) => {
                            Cow::Borrowed(c.value.as_deref().unwrap_or(&c.expr))
                        },
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
                                rustdoc_types::Term::Type(ty) => self.type_renderer.render_type(ty),
                                rustdoc_types::Term::Constant(c) => {
                                    Cow::Borrowed(c.value.as_deref().unwrap_or(&c.expr))
                                },
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
                                bound_strs
                                    .iter()
                                    .map(AsRef::as_ref)
                                    .collect::<Vec<_>>()
                                    .join(" + ")
                            ))
                        },
                    }
                }));

                if parts.is_empty() {
                    String::new()
                } else {
                    format!(
                        "<{}>",
                        parts
                            .iter()
                            .map(AsRef::as_ref)
                            .collect::<Vec<_>>()
                            .join(", ")
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
                    input_strs
                        .iter()
                        .map(AsRef::as_ref)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            },

            rustdoc_types::GenericArgs::ReturnTypeNotation => " (..)".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod trivial_derive_traits_tests {
        use super::*;

        #[test]
        fn trivial_traits_list_has_nine_entries() {
            assert_eq!(TRIVIAL_DERIVE_TRAITS.len(), 9);
        }

        #[test]
        fn descriptions_match_traits_count() {
            assert_eq!(
                TRIVIAL_DERIVE_TRAITS.len(),
                TRIVIAL_DERIVE_DESCRIPTIONS.len()
            );
        }

        #[test]
        fn all_traits_have_descriptions() {
            for trait_name in TRIVIAL_DERIVE_TRAITS {
                assert!(
                    get_trivial_derive_description(trait_name).is_some(),
                    "Missing description for trait: {trait_name}"
                );
            }
        }

        #[test]
        fn get_description_for_clone() {
            assert_eq!(
                get_trivial_derive_description("Clone"),
                Some("Returns a copy of the value")
            );
        }

        #[test]
        fn get_description_for_debug() {
            assert_eq!(
                get_trivial_derive_description("Debug"),
                Some("Formats the value for debugging")
            );
        }

        #[test]
        fn get_description_for_full_path() {
            // Should extract trait name from full path
            assert_eq!(
                get_trivial_derive_description("std::clone::Clone"),
                Some("Returns a copy of the value")
            );
        }

        #[test]
        fn get_description_for_unknown_trait() {
            assert_eq!(get_trivial_derive_description("Display"), None);
        }

        #[test]
        fn contains_cloning_traits() {
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Clone"));
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Copy"));
        }

        #[test]
        fn contains_equality_traits() {
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"PartialEq"));
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Eq"));
        }

        #[test]
        fn contains_ordering_traits() {
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"PartialOrd"));
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Ord"));
        }

        #[test]
        fn contains_hash_trait() {
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Hash"));
        }

        #[test]
        fn contains_debug_and_default() {
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Debug"));
            assert!(TRIVIAL_DERIVE_TRAITS.contains(&"Default"));
        }

        #[test]
        fn does_not_contain_display() {
            assert!(!TRIVIAL_DERIVE_TRAITS.contains(&"Display"));
        }

        #[test]
        fn does_not_contain_serialize() {
            assert!(!TRIVIAL_DERIVE_TRAITS.contains(&"Serialize"));
            assert!(!TRIVIAL_DERIVE_TRAITS.contains(&"Deserialize"));
        }
    }
}
