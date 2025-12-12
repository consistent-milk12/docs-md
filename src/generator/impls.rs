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
use std::collections::HashSet;
use std::fmt::Write;

use rustdoc_types::{
    AssocItemConstraintKind, Crate, GenericArg, GenericArgs, GenericBound, GenericParamDefKind, Id,
    Impl, Item, ItemEnum, Term, Type, WherePredicate,
};

use crate::generator::context::RenderContext;
use crate::generator::render_shared::{RendererInternals, RendererUtils};
use crate::linker::ImplContext;
use crate::types::TypeRenderer;
use crate::utils::PathUtils;

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

/// Utility functions to work with impl's and generics
pub struct ImplUtils;

impl ImplUtils {
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
        let trait_name = PathUtils::short_name(&trait_ref.path);

        TRIVIAL_DERIVE_TRAITS.contains(&trait_name)
    }

    /// Get the description for a trivial derive trait.
    ///
    /// Returns `None` if the trait is not in the trivial derives list.
    #[must_use]
    pub fn get_trivial_derive_description(trait_name: &str) -> Option<&'static str> {
        // Extract just the trait name if it's a full path
        let name = PathUtils::short_name(trait_name);

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
                    .is_some_and(|args| Self::generic_args_contain_generic(args))
            },

            // References, raw pointers, and arrays: check the inner type
            Type::BorrowedRef { type_, .. }
            | Type::RawPointer { type_, .. }
            | Type::Array { type_, .. } => Self::is_generic_type(type_),

            // Slices: check the element type
            Type::Slice(inner) => Self::is_generic_type(inner),

            // Tuples: check all element types
            Type::Tuple(types) => types.iter().any(Self::is_generic_type),

            // Qualified paths like `<T as Trait>::Item`
            Type::QualifiedPath { self_type, .. } => Self::is_generic_type(self_type),

            // Primitives and other types are not generic
            _ => false,
        }
    }

    /// Check if generic args contain any generic type parameters.
    fn generic_args_contain_generic(args: &GenericArgs) -> bool {
        match args {
            GenericArgs::AngleBracketed { args, .. } => args.iter().any(|arg| match arg {
                GenericArg::Type(ty) => Self::is_generic_type(ty),

                GenericArg::Const(_) | GenericArg::Lifetime(_) | GenericArg::Infer => false,
            }),

            GenericArgs::Parenthesized { inputs, output } => {
                inputs.iter().any(Self::is_generic_type)
                    || output.as_ref().is_some_and(Self::is_generic_type)
            },

            GenericArgs::ReturnTypeNotation => false,
        }
    }

    // Contains AI generated docs - Start (STILL NOT PROPERLY CHECKED)
    // TODO: Clean this up later to reduce it down to the necessary details,
    // various information related to rustdoc JSON and rustdoc_types crate
    // needs to be checked.

    /// Extract generic parameter names that appear in the impl signature (for_ and trait).
    ///
    /// This extracts generics from only the visible parts of the impl header:
    /// 1. The `for_` type: `impl<T> Trait for Foo<T>` → extracts `T` from `Foo<T>`
    /// 2. The trait path: `impl<U> Trait<U> for Foo` → extracts `U` from `Trait<U>`
    ///
    /// Unlike [`extract_impl_visible_generics`], this does NOT extract from where clauses.
    /// Use this for rendering purposes when deciding which generic params to show in
    /// `impl<...>`. Generics that only appear in where clauses should not be shown in
    /// the impl header since the where clause itself is not rendered.
    ///
    /// # Examples
    ///
    /// ```text
    /// impl<T: Sized> IntoEither for T
    ///   └─ for_ type is Generic("T") → extracts ["T"]
    ///   └─ When rendered for `Either<L, R>`, for_ becomes `Either<L, R>`
    ///   └─ Extracts ["L", "R"], NOT ["T"]
    ///   └─ T is only in where clause, so `impl<T>` should NOT be shown
    ///
    /// impl<T> Clone for Wrapper<T>
    ///   └─ for_ type has `Wrapper<T>` → extracts ["T"]
    ///   └─ Result: render as `impl<T>`
    ///
    /// impl<T, U> Trait<U> for Foo<T>
    ///   └─ for_ type has `Foo<T>` → extracts ["T"]
    ///   └─ trait path has `Trait<U>` → extracts ["U"]
    ///   └─ Result: render as `impl<T, U>` (both visible)
    /// ```
    #[must_use]
    pub fn extract_impl_signature_generics(impl_block: &Impl) -> HashSet<String> {
        let mut names = HashSet::new();

        // Extract from for_ type (the type being implemented for)
        Self::extract_type_generics_into(&impl_block.for_, &mut names);

        // Extract from trait path generic arguments
        if let Some(trait_ref) = &impl_block.trait_
            && let Some(args) = &trait_ref.args
        {
            Self::extract_generic_args_into(args, &mut names);
        }

        names
    }

    /// Extract all generic parameter names visible in an impl block's signature.
    ///
    /// # Why This Is Needed
    ///
    /// Rustdoc JSON stores impl generics in two places that can have DIFFERENT names:
    /// - `impl_block.generics.params` - the declared params (e.g., `T` from `impl<T>`)
    /// - `impl_block.for_` - the actual type (e.g., `StyledObject<D>`)
    ///
    /// For blanket impls, these names often differ! The compiler transforms:
    /// ```text
    /// impl<D: Display> ToString for StyledObject<D>
    /// ```
    /// Into JSON where `generics.params` has `T` but `for_` has `StyledObject<D>`.
    /// If we blindly use `generics.params`, we'd render `impl<T> ToString for StyledObject<D>`.
    ///
    /// # Solution
    ///
    /// Extract generics from what's VISIBLE in the signature:
    /// 1. The `for_` type: `Foo<T>` → extracts `T`
    /// 2. The trait path: `Iterator<Item = U>` → extracts `U`
    /// 3. Where clauses: `where T: Clone` → extracts `T`
    ///
    /// Then filter `generics.params` to only include names that appear in visible locations.
    ///
    /// **Note:** For rendering the impl header, use [`extract_impl_signature_generics`] instead,
    /// which excludes where clause generics.
    ///
    /// # Examples
    ///
    /// ```text
    /// impl<T> Clone for Wrapper<T>
    ///   └─ for_ type has `Wrapper<T>` → extracts ["T"]
    ///   └─ Result: render as `impl<T>`
    ///
    /// impl<T, U> Trait<U> for Foo<T> where T: Clone
    ///   └─ for_ type has `Foo<T>` → extracts ["T"]
    ///   └─ trait path has `Trait<U>` → extracts ["U"]
    ///   └─ where clause has `T: Clone` → extracts ["T"]
    ///   └─ Result: render as `impl<T, U>` (both visible)
    ///
    /// impl<D: Display> ToString for StyledObject<D>
    ///   └─ for_ type has `StyledObject<D>` → extracts ["D"]
    ///   └─ Result: render as `impl<D>` (not `impl<T>`)
    /// ```
    #[must_use]
    pub fn extract_impl_visible_generics(impl_block: &Impl) -> HashSet<String> {
        // Start with signature generics (for_ type and trait path)
        let mut names = Self::extract_impl_signature_generics(impl_block);

        // Also extract from where clause predicates
        for pred in &impl_block.generics.where_predicates {
            match pred {
                // `where T: Clone + Iterator<Item = U>` extracts both T and U
                WherePredicate::BoundPredicate { type_, bounds, .. } => {
                    Self::extract_type_generics_into(type_, &mut names);

                    for bound in bounds {
                        Self::extract_bound_generics_into(bound, &mut names);
                    }
                },

                // `where <T as Trait>::Assoc = U` extracts both T and U
                WherePredicate::EqPredicate { lhs, rhs, .. } => {
                    Self::extract_type_generics_into(lhs, &mut names);

                    Self::extract_term_generics_into(rhs, &mut names);
                },

                // Lifetime predicates like `where 'a: 'b` don't have type params
                WherePredicate::LifetimePredicate { .. } => {},
            }
        }

        names
    }

    /// Extract generic parameter names from a type into a set.
    ///
    /// Recursively traverses the type structure to find all `Type::Generic` variants.
    /// This handles all the ways generics can be nested in complex types.
    ///
    /// # Type Tree Visualization
    ///
    /// ```text
    /// HashMap<K, Vec<T>>
    /// └─ ResolvedPath("HashMap")
    ///    └─ args: [K, Vec<T>]
    ///             │  └─ ResolvedPath("Vec")
    ///             │     └─ args: [T]
    ///             │              └─ Generic("T") ← extracted!
    ///             └─ Generic("K") ← extracted!
    /// ```
    fn extract_type_generics_into(ty: &Type, names: &mut HashSet<String>) {
        match ty {
            // Base case: A bare generic parameter like `T`, `U`, `Item`
            // Example: In `fn foo<T>(x: T)`, the parameter type is Generic("T")
            Type::Generic(name) => {
                names.insert(name.clone());
            },

            // Named types like `Vec<T>`, `HashMap<K, V>`, `Option<T>`
            // The generics are inside the `args` field
            // Example: `Vec<T>` → ResolvedPath { path: "Vec", args: [Generic("T")] }
            Type::ResolvedPath(path) => {
                if let Some(args) = &path.args {
                    Self::extract_generic_args_into(args, names);
                }
            },

            // References and pointers: `&T`, `&mut T`, `*const T`, `*mut T`
            // Example: `&Vec<T>` → BorrowedRef { type_: ResolvedPath("Vec<T>") }
            Type::BorrowedRef { type_, .. } | Type::RawPointer { type_, .. } => {
                Self::extract_type_generics_into(type_, names);
            },

            // Slices and arrays: `[T]`, `[T; N]`
            // Example: `[T]` → Slice(Generic("T"))
            // Example: `[u8; 32]` → Array { type_: Primitive("u8"), len: 32 }
            Type::Slice(t) | Type::Array { type_: t, .. } => {
                Self::extract_type_generics_into(t, names);
            },

            // Tuples: `(T, U, V)`
            // Example: `(T, u32, V)` → Tuple([Generic("T"), Primitive, Generic("V")])
            Type::Tuple(types) => {
                for t in types {
                    Self::extract_type_generics_into(t, names);
                }
            },

            // Qualified paths: `<T as Trait>::Item`, `<I as Iterator>::Item`
            // These appear in associated type definitions
            // Example: `<I as Iterator>::Item` → QualifiedPath {
            //            self_type: Generic("I"),
            //            trait_: Some(Path("Iterator")),
            //            name: "Item"
            //          }
            Type::QualifiedPath {
                self_type, trait_, ..
            } => {
                // Extract from the self type (e.g., `I` in `<I as Iterator>::Item`)
                Self::extract_type_generics_into(self_type, names);

                // Also check trait generic args (e.g., `<T as Trait<U>>::Item`)
                if let Some(tr) = trait_
                    && let Some(args) = &tr.args
                {
                    Self::extract_generic_args_into(args, names);
                }
            },

            // Function pointers: `fn(T) -> U`, `fn(&T, V) -> W`
            // Example: `fn(T) -> U` → FunctionPointer {
            //            inputs: [("_", Generic("T"))],
            //            output: Some(Generic("U"))
            //          }
            Type::FunctionPointer(fp) => {
                // Function pointer inputs are (name, type) tuples - we only need the type
                for (_, input_ty) in &fp.sig.inputs {
                    Self::extract_type_generics_into(input_ty, names);
                }

                if let Some(output) = &fp.sig.output {
                    Self::extract_type_generics_into(output, names);
                }
            },

            // impl Trait: `impl Iterator<Item = T>`, `impl Clone + Send`
            // Example: `impl Iterator<Item = T>` → ImplTrait([TraitBound(Iterator<Item=T>)])
            Type::ImplTrait(bounds) => {
                for bound in bounds {
                    Self::extract_bound_generics_into(bound, names);
                }
            },

            // dyn Trait: `dyn Iterator<Item = T>`, `dyn Fn(T) -> U`
            // Example: `dyn Iterator<Item = T>` → DynTrait with trait bounds
            Type::DynTrait(dyn_trait) => {
                for poly in &dyn_trait.traits {
                    if let Some(args) = &poly.trait_.args {
                        Self::extract_generic_args_into(args, names);
                    }
                }
            },

            // Leaf types with no generics
            // - Primitive: `i32`, `bool`, `str`, etc.
            // - Infer: `_` (the compiler infers this)
            // - Pat: pattern types (experimental)
            Type::Primitive(_) | Type::Infer | Type::Pat { .. } => {},
        }
    }

    /// Extract generic parameter names from generic arguments.
    ///
    /// Generic arguments come in three forms:
    ///
    /// # Forms
    ///
    /// ```text
    /// AngleBracketed: Vec<T, U>           → args: [T, U], constraints: []
    ///                 Iterator<Item = T>  → args: [], constraints: [Item = T]
    ///
    /// Parenthesized:  Fn(T, U) -> V       → inputs: [T, U], output: Some(V)
    ///                 FnOnce(T)           → inputs: [T], output: None
    ///
    /// ReturnTypeNotation: method(..)      → (no generics to extract)
    /// ```
    fn extract_generic_args_into(args: &GenericArgs, names: &mut HashSet<String>) {
        match args {
            // Angle-bracketed args: `<T, U, Item = V>`
            //
            // Two parts:
            // - `args`: positional type args like `T` and `U`
            // - `constraints`: associated type bindings like `Item = V`
            //
            // Example: `Iterator<Item = T>` has constraints with `Item = T`
            // Example: `HashMap<K, V>` has args [K, V] with no constraints
            GenericArgs::AngleBracketed { args, constraints } => {
                // Extract from positional type arguments: <T, U, ...>
                for arg in args {
                    // Only Type args have generics - Lifetime('a) and Const(N) don't
                    if let GenericArg::Type(ty) = arg {
                        Self::extract_type_generics_into(ty, names);
                    }
                }

                // Extract from associated type constraints: <Item = T, Output = U>
                // These can be either:
                // - Equality: `Item = T` (term is a Type or Const)
                // - Constraint: `Item: Clone` (has trait bounds)
                for constraint in constraints {
                    match &constraint.binding {
                        // `Item = T` → extract T from the right-hand side
                        AssocItemConstraintKind::Equality(term) => {
                            Self::extract_term_generics_into(term, names);
                        },

                        // `Item: Iterator<Item = U>` → extract from bounds
                        AssocItemConstraintKind::Constraint(bounds) => {
                            for bound in bounds {
                                Self::extract_bound_generics_into(bound, names);
                            }
                        },
                    }
                }
            },

            // Parenthesized args: `(T, U) -> V` (used for Fn traits)
            //
            // Example: `Fn(i32, T) -> U` → inputs: [i32, T], output: Some(U)
            // Example: `FnMut(&T)` → inputs: [&T], output: None (returns ())
            GenericArgs::Parenthesized { inputs, output } => {
                for input in inputs {
                    Self::extract_type_generics_into(input, names);
                }

                if let Some(out) = output {
                    Self::extract_type_generics_into(out, names);
                }
            },

            // Return type notation: `method(..)` - experimental, no generics
            GenericArgs::ReturnTypeNotation => {},
        }
    }

    /// Extract generic parameter names from a type or const term.
    ///
    /// A `Term` appears in associated type bindings like `Iterator<Item = T>`.
    /// - `Term::Type(T)` - the `T` in `Item = T`
    /// - `Term::Constant(N)` - const generics like `Item = 42`
    fn extract_term_generics_into(term: &Term, names: &mut HashSet<String>) {
        match term {
            // Type term: `Item = T` → extract T
            Term::Type(ty) => Self::extract_type_generics_into(ty, names),

            // Const term: `Item = 42` → no type generics to extract
            Term::Constant(_) => {},
        }
    }

    /// Extract generic parameter names from a generic bound.
    ///
    /// Bounds appear in where clauses: `where T: Clone + Iterator<Item = U>`
    ///
    /// We only care about `TraitBound` (not `Outlives` like `'a: 'b`).
    /// From the trait bound, we extract any generic args on the trait itself.
    ///
    /// # Example
    ///
    /// ```text
    /// where T: Iterator<Item = U>
    ///          └─ TraitBound { trait_: Path("Iterator"), args: [Item = U] }
    ///                                                              └─ extracts U
    /// ```
    fn extract_bound_generics_into(bound: &GenericBound, names: &mut HashSet<String>) {
        // Only TraitBound has generic args - Outlives is for lifetimes only
        if let GenericBound::TraitBound { trait_, .. } = bound
            && let Some(args) = &trait_.args
        {
            Self::extract_generic_args_into(args, names);
        }
    }

    /// Check if an impl has associated types referencing generics NOT visible in the signature.
    ///
    /// # The Problem
    ///
    /// Some impl blocks have generics that only appear in associated types, not in the
    /// visible signature. When rendered, this produces confusing output:
    ///
    /// ```text
    /// impl DoubleEndedIterator for Foo    ← No `I` visible!
    ///     type Item = <I as Iterator>::Item   ← Where does `I` come from?
    /// ```
    ///
    /// # Detection Strategy
    ///
    /// 1. Extract "visible" generics from: `for_` type, trait path, where clause
    /// 2. Extract "declared" generics from: `impl_block.generics.params`
    /// 3. Compute "hidden" = declared - visible
    /// 4. Check if any associated type references a hidden generic
    ///
    /// # Examples
    ///
    /// ```text
    /// // FILTER THIS - `I` is hidden but used in associated type
    /// impl<I: Iterator> DoubleEndedIterator for Foo {
    ///     type Item = <I as Iterator>::Item;
    /// }
    /// visible = {}        (Foo has no generics)
    /// declared = {I}      (from impl<I>)
    /// hidden = {I}        (declared - visible)
    /// assoc type uses I   → FILTER!
    ///
    /// // KEEP THIS - `T` is visible in for_ type
    /// impl<T> Iterator for Wrapper<T> {
    ///     type Item = T;
    /// }
    /// visible = {T}       (from Wrapper<T>)
    /// declared = {T}      (from impl<T>)
    /// hidden = {}         (all visible)
    /// → KEEP!
    ///
    /// // KEEP THIS - `U` is visible in trait path
    /// impl<T, U> Trait<U> for Foo<T> {
    ///     type Output = (T, U);
    /// }
    /// visible = {T, U}    (T from Foo<T>, U from Trait<U>)
    /// declared = {T, U}   (from impl<T, U>)
    /// hidden = {}         (all visible)
    /// → KEEP!
    /// ```
    #[must_use]
    pub fn has_hidden_generic_refs(impl_block: &Impl, krate: &Crate) -> bool {
        // ┌─────────────────────────────────────────────────────────────────────┐
        // │ Step 1: Get generics that appear in the VISIBLE signature           │
        // │                                                                     │
        // │ These come from: for_ type, trait path, where clause                │
        // │ Example: `impl<T> Trait for Foo<T> where T: Clone`                  │
        // │          visible = {T} (appears in Foo<T> and where clause)         │
        // └─────────────────────────────────────────────────────────────────────┘
        let visible_generics = Self::extract_impl_visible_generics(impl_block);

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │ Step 2: Get generics that are DECLARED in impl<...>                 │
        // │                                                                     │
        // │ Only type parameters, not lifetimes ('a) or const generics (N)      │
        // │ Example: `impl<'a, T, const N: usize>` → declared = {T}             │
        // └─────────────────────────────────────────────────────────────────────┘
        let impl_generics: HashSet<_> = impl_block
            .generics
            .params
            .iter()
            .filter_map(|p| match &p.kind {
                // Only type parameters can be "hidden" in our sense
                GenericParamDefKind::Type { .. } => Some(p.name.clone()),

                // Const and lifetime params don't cause the issue we're filtering
                GenericParamDefKind::Const { .. } | GenericParamDefKind::Lifetime { .. } => None,
            })
            .collect();

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │ Step 3: Compute hidden = declared - visible                         │
        // │                                                                     │
        // │ These are params declared in impl<...> but not appearing anywhere   │
        // │ in the visible signature (for_, trait path, where clause)           │
        // └─────────────────────────────────────────────────────────────────────┘
        let hidden_generics: HashSet<_> = impl_generics
            .difference(&visible_generics)
            .cloned()
            .collect();

        // If nothing is hidden, the impl is fine - no filtering needed
        if hidden_generics.is_empty() {
            return false;
        }

        // ┌─────────────────────────────────────────────────────────────────────┐
        // │ Step 4: Check if any associated type USES a hidden generic          │
        // │                                                                     │
        // │ Having hidden generics is only a problem if they're actually used.  │
        // │ We check each associated type definition in the impl block.         │
        // │                                                                     │
        // │ Example: `type Item = <I as Iterator>::Item` uses hidden `I`        │
        // └─────────────────────────────────────────────────────────────────────┘
        for item_id in &impl_block.items {
            // Look up the item in the crate's index
            let Some(item) = krate.index.get(item_id) else {
                continue;
            };

            // We only care about associated types with a concrete definition
            // (not `type Item;` without a `= ...` part)
            if let ItemEnum::AssocType {
                type_: Some(ty), ..
            } = &item.inner
            {
                // Extract all generics referenced in the associated type's definition
                let mut referenced = HashSet::new();
                Self::extract_type_generics_into(ty, &mut referenced);

                // If any referenced generic is hidden, this impl should be filtered
                if referenced.iter().any(|name| hidden_generics.contains(name)) {
                    return true;
                }
            }
        }

        // No hidden generics are used in associated types - impl is okay to render
        false
    }

    // Contains AI generated docs - End
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
            let key_a = RendererInternals::impl_sort_key(a, &self.type_renderer);
            let key_b = RendererInternals::impl_sort_key(b, &self.type_renderer);

            key_a.cmp(&key_b)
        });

        // Deduplicate trait impls with the same signature
        trait_impls.dedup_by(|a, b| {
            RendererInternals::impl_sort_key(a, &self.type_renderer)
                == RendererInternals::impl_sort_key(b, &self.type_renderer)
        });

        // === Inherent Implementations ===
        if !inherent_impls.is_empty() {
            _ = write!(md, "#### Implementations\n\n");

            for impl_block in inherent_impls {
                self.render_impl_methods(md, impl_block);
            }
        }

        // === Trait Implementations ===
        // Filter out:
        // 1. Blanket impls (From, Into, Any, etc.) unless explicitly included
        // 2. Impls with hidden generic references (Issue 5 fix)
        //    e.g., `type Item = <I as Iterator>::Item` where I is not visible
        let krate = self.ctx.krate();
        let filtered_trait_impls: Vec<_> = if self.ctx.include_blanket_impls() {
            trait_impls
                .into_iter()
                .filter(|i| !ImplUtils::has_hidden_generic_refs(i, krate))
                .collect()
        } else {
            trait_impls
                .into_iter()
                .filter(|i| {
                    !ImplUtils::is_blanket_impl(i) && !ImplUtils::has_hidden_generic_refs(i, krate)
                })
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
                .partition(|i| ImplUtils::is_trivial_derive_impl(i));

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

        _ = write!(
            md,
            "{}",
            RendererInternals::render_collapsible_start(&summary)
        );

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

                let description = ImplUtils::get_trivial_derive_description(trait_name)
                    .unwrap_or("Derived trait");

                _ = writeln!(md, "| `{trait_name}` | {description} |");
            }
        }

        _ = write!(md, "{}", RendererInternals::render_collapsible_end());
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
                let mut name = RendererUtils::sanitize_path(&t.path).into_owned();

                if let Some(args) = &t.args {
                    _ = write!(name, "{}", &self.render_generic_args_for_impl(args));
                }

                name
            })
            .unwrap_or_default();

        // Extract generics that appear in the signature (for_ type and trait path).
        // This fixes the generic parameter mismatch issue where `impl_block.generics.params`
        // might have different names (e.g., T) than what's used in for_ type (e.g., D).
        //
        // IMPORTANT: We use `extract_impl_signature_generics` which excludes where clause
        // generics. For blanket impls like `impl<T: Sized> IntoEither for T`, when rendered
        // for `Either<L, R>`, the `T` only appears in the where clause `where T: Sized`.
        // We don't want to show `impl<T> IntoEither for Either<L, R>` since T doesn't appear
        // in the visible header - it would be confusing to users.
        let signature_generics = ImplUtils::extract_impl_signature_generics(impl_block);

        let generics = if signature_generics.is_empty() {
            String::new()
        } else {
            // Filter impl params to only include those that appear in for_ type or trait path
            let filtered: Vec<_> = impl_block
                .generics
                .params
                .iter()
                .filter(|p| signature_generics.contains(&p.name))
                .cloned()
                .collect();

            if filtered.is_empty() {
                String::new()
            } else {
                self.type_renderer.render_generics(&filtered)
            }
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

        // Determine impl context for anchor generation:
        // - For trait impls, include the trait name to avoid duplicate anchors
        //   when multiple traits define the same associated type (e.g., Output)
        // - For inherent impls, use simple anchors
        let impl_ctx = impl_block.trait_.as_ref().map_or(
            ImplContext::Inherent,
            |t| ImplContext::Trait(PathUtils::short_name(&t.path)),
        );

        RendererInternals::render_impl_items(
            md,
            impl_block,
            krate,
            &self.type_renderer,
            &Some(|item: &Item| self.process_docs(item)),
            &Some(|id: Id| self.ctx.create_link(id, self.current_file)),
            Some(type_name.as_ref()),
            impl_ctx,
        );
    }

    /// Render generic arguments for impl block signatures.
    ///
    /// This handles the different forms of generic arguments:
    /// - **Angle bracketed**: `<T, U, Item = V>` (most common)
    /// - **Parenthesized**: `(A, B) -> C` (for Fn traits)
    /// - **Return type notation**: `(..)` (experimental)
    fn render_generic_args_for_impl(&self, args: &GenericArgs) -> String {
        match args {
            GenericArgs::AngleBracketed { args, constraints } => {
                let mut parts: Vec<Cow<str>> = args
                    .iter()
                    .map(|a| match a {
                        GenericArg::Lifetime(lt) => Cow::Borrowed(lt.as_str()),

                        GenericArg::Type(ty) => self.type_renderer.render_type(ty),

                        GenericArg::Const(c) => {
                            Cow::Borrowed(c.value.as_deref().unwrap_or(&c.expr))
                        },

                        GenericArg::Infer => Cow::Borrowed("_"),
                    })
                    .collect();

                parts.extend(constraints.iter().map(|c| {
                    let constraint_args = c
                        .args
                        .as_ref()
                        .map(|a| self.render_generic_args_for_impl(a))
                        .unwrap_or_default();

                    match &c.binding {
                        AssocItemConstraintKind::Equality(term) => {
                            let term_str = match term {
                                Term::Type(ty) => self.type_renderer.render_type(ty),

                                Term::Constant(c) => {
                                    Cow::Borrowed(c.value.as_deref().unwrap_or(&c.expr))
                                },
                            };

                            Cow::Owned(format!("{}{constraint_args} = {term_str}", c.name))
                        },

                        AssocItemConstraintKind::Constraint(bounds) => {
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

            GenericArgs::Parenthesized { inputs, output } => {
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

            GenericArgs::ReturnTypeNotation => " (..)".to_string(),
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
                    ImplUtils::get_trivial_derive_description(trait_name).is_some(),
                    "Missing description for trait: {trait_name}"
                );
            }
        }

        #[test]
        fn get_description_for_clone() {
            assert_eq!(
                ImplUtils::get_trivial_derive_description("Clone"),
                Some("Returns a copy of the value")
            );
        }

        #[test]
        fn get_description_for_debug() {
            assert_eq!(
                ImplUtils::get_trivial_derive_description("Debug"),
                Some("Formats the value for debugging")
            );
        }

        #[test]
        fn get_description_for_full_path() {
            // Should extract trait name from full path
            assert_eq!(
                ImplUtils::get_trivial_derive_description("std::clone::Clone"),
                Some("Returns a copy of the value")
            );
        }

        #[test]
        fn get_description_for_unknown_trait() {
            assert_eq!(ImplUtils::get_trivial_derive_description("Display"), None);
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
