//! Type rendering utilities for converting rustdoc types to string representations.
//!
//! This module provides the [`TypeRenderer`] struct to convert the complex type
//! structures from rustdoc JSON into human-readable Rust type syntax. These
//! rendered strings are used in the generated markdown documentation.
//!
//! # Overview
//!
//! Rustdoc JSON represents types as a tree structure (the `Type` enum). The
//! [`TypeRenderer`] walks that tree and produces the string representation
//! you'd write in code.
//!
//! # Usage
//!
//! ```ignore
//! let renderer = TypeRenderer::new(&krate);
//! let type_string = renderer.render_type(&some_type);
//! let generics = renderer.render_generics(&generic_params);
//! ```
//!
//! # Example Transformations
//!
//! | Rustdoc Type | Rendered String |
//! |--------------|-----------------|
//! | `Type::Primitive("u32")` | `"u32"` |
//! | `Type::BorrowedRef { lifetime: Some("'a"), is_mutable: true, type_: ... }` | `"&'a mut T"` |
//! | `Type::ResolvedPath { path: "Vec", args: ... }` | `"Vec<T>"` |
//!
//! # Performance
//!
//! Uses `Cow<str>` to avoid allocations for simple types like primitives,
//! generics, and inferred types. Complex types that require string building
//! return owned strings.

use std::borrow::Cow;

use rustdoc_types::{
    AssocItemConstraint, AssocItemConstraintKind, Crate, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Id, Term, TraitBoundModifier, Type,
};

use crate::generator::render_shared::sanitize_path;

/// Type renderer for converting rustdoc types to Rust syntax strings.
///
/// This struct holds a reference to the crate context and provides methods
/// to render various type constructs into their string representations.
///
/// # Design Note
///
/// The `krate` field is currently unused because the `Type` enum is self-contained.
/// However, it is retained for:
/// - **Future-proofing**: May need to look up items in `krate.index` for enhanced rendering
/// - **API consistency**: Signals that the renderer is bound to a specific crate context
/// - **Type safety**: Prevents accidentally mixing renderers across different crates
///
/// # Example
///
/// ```ignore
/// let renderer = TypeRenderer::new(&krate);
/// let type_str = renderer.render_type(&some_type);
/// let generics = renderer.render_generics(&params);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct TypeRenderer<'a> {
    /// Reference to the crate for looking up type information.
    ///
    /// Reserved for future use (e.g., resolving paths, getting item metadata).
    #[expect(dead_code, reason = "TODO: Reserved for future use.")]
    krate: &'a Crate,
}

impl<'a> TypeRenderer<'a> {
    /// Create a new type renderer with the given crate context.
    ///
    /// # Arguments
    ///
    /// * `krate` - The parsed rustdoc crate containing type definitions
    #[must_use]
    pub const fn new(krate: &'a Crate) -> Self {
        Self { krate }
    }

    /// Get the ID of a resolved type, if available.
    ///
    /// Returns `Some(Id)` for resolved path types (named types like structs,
    /// enums, traits), `None` for primitives and other type variants.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to extract the ID from
    #[must_use]
    pub const fn get_type_id(&self, ty: &Type) -> Option<Id> {
        match ty {
            Type::ResolvedPath(path) => Some(path.id),

            _ => None,
        }
    }

    /// Render a rustdoc `Type` to its Rust syntax string representation.
    ///
    /// This is the main entry point for type rendering. It handles all variants
    /// of the `Type` enum and recursively renders nested types.
    ///
    /// # Arguments
    ///
    /// * `ty` - The type to render
    ///
    /// # Returns
    ///
    /// A `Cow<str>` representing the type in Rust syntax. Simple types like
    /// primitives and generics return borrowed strings to avoid allocation.
    ///
    /// # Supported Type Variants
    ///
    /// - Primitives: `u32`, `bool`, `str`, etc.
    /// - References: `&T`, `&'a mut T`
    /// - Pointers: `*const T`, `*mut T`
    /// - Slices and arrays: `[T]`, `[T; N]`
    /// - Tuples: `(A, B, C)`
    /// - Paths: `std::vec::Vec<T>`
    /// - Function pointers: `fn(A, B) -> C`
    /// - Trait objects: `dyn Trait + Send`
    /// - Impl trait: `impl Iterator<Item = T>`
    /// - Qualified paths: `<T as Trait>::Item`
    #[must_use]
    pub fn render_type<'t>(&self, ty: &'t Type) -> Cow<'t, str> {
        match ty {
            // Named type path like `Vec<T>` or `std::collections::HashMap<K, V>`
            Type::ResolvedPath(path) => {
                let sanitized = sanitize_path(&path.path);
                // If no generic args and no sanitization needed, borrow the path directly
                if path.args.is_none() && matches!(sanitized, Cow::Borrowed(_)) {
                    return Cow::Borrowed(&path.path);
                }
                let mut result = sanitized.into_owned();
                // Append generic arguments if present
                if let Some(args) = &path.args {
                    result.push_str(&self.render_generic_args(args));
                }
                Cow::Owned(result)
            },

            // Trait object: `dyn Trait + OtherTrait`
            Type::DynTrait(dyn_trait) => {
                let traits: Vec<String> = dyn_trait
                    .traits
                    .iter()
                    .map(|pt| {
                        let sanitized = sanitize_path(&pt.trait_.path);
                        if pt.trait_.args.is_none() {
                            sanitized.into_owned()
                        } else {
                            let mut s = sanitized.into_owned();
                            if let Some(args) = &pt.trait_.args {
                                s.push_str(&self.render_generic_args(args));
                            }
                            s
                        }
                    })
                    .collect();
                Cow::Owned(format!("dyn {}", traits.join(" + ")))
            },

            // Generic type parameter: `T`, `U`, etc.
            // Primitive type: `u32`, `bool`, `str`, etc.
            // Zero allocation - borrow from input
            Type::Generic(name) | Type::Primitive(name) => Cow::Borrowed(name),

            // Function pointer: `fn(A, B) -> C`
            Type::FunctionPointer(fp) => {
                // Render parameter types (we ignore parameter names for brevity)
                let params: Vec<Cow<str>> = fp
                    .sig
                    .inputs
                    .iter()
                    .map(|(_, t)| self.render_type(t))
                    .collect();
                // Render return type if present
                let ret = fp.sig.output.as_ref().map_or_else(String::new, |output| {
                    format!(" -> {}", self.render_type(output))
                });
                Cow::Owned(format!("fn({}){}", params.join(", "), ret))
            },

            // Tuple type: `(A, B, C)` or unit `()`
            Type::Tuple(types) => {
                if types.is_empty() {
                    return Cow::Borrowed("()");
                }
                let inner: Vec<Cow<str>> = types.iter().map(|t| self.render_type(t)).collect();
                Cow::Owned(format!("({})", inner.join(", ")))
            },

            // Slice type: `[T]`
            Type::Slice(inner) => Cow::Owned(format!("[{}]", self.render_type(inner))),

            // Array type: `[T; N]` where N is a const expression
            Type::Array { type_, len } => {
                Cow::Owned(format!("[{}; {len}]", self.render_type(type_)))
            },

            // Pattern type (rare): just render the underlying type
            Type::Pat { type_, .. } => self.render_type(type_),

            // Impl trait in return position: `impl Iterator<Item = T>`
            Type::ImplTrait(bounds) => {
                let bound_strs: Vec<Cow<str>> = bounds
                    .iter()
                    .map(|b| self.render_generic_bound(b))
                    .collect();
                Cow::Owned(format!("impl {}", bound_strs.join(" + ")))
            },

            // Inferred type: `_` (placeholder in turbofish)
            // Zero allocation - static string
            Type::Infer => Cow::Borrowed("_"),

            // Raw pointer: `*const T` or `*mut T`
            Type::RawPointer { is_mutable, type_ } => {
                let mutability = if *is_mutable { "mut" } else { "const" };
                Cow::Owned(format!("*{mutability} {}", self.render_type(type_)))
            },

            // Reference: `&T`, `&mut T`, `&'a T`, `&'a mut T`
            Type::BorrowedRef {
                lifetime,
                is_mutable,
                type_,
            } => {
                // Optional lifetime annotation
                let lt = lifetime
                    .as_ref()
                    .map(|l| format!("{l} "))
                    .unwrap_or_default();
                // Optional mut keyword
                let mutability = if *is_mutable { "mut " } else { "" };
                Cow::Owned(format!("&{lt}{mutability}{}", self.render_type(type_)))
            },

            // Qualified path: `<T as Trait>::Item` or `T::Item`
            Type::QualifiedPath {
                name,
                self_type,
                trait_,
                ..
            } => {
                let self_ty = self.render_type(self_type);
                Cow::Owned(trait_.as_ref().map_or_else(
                    || format!("{self_ty}::{name}"),
                    |trait_path| {
                        let sanitized_trait = sanitize_path(&trait_path.path);
                        format!("<{self_ty} as {sanitized_trait}>::{name}")
                    },
                ))
            },
        }
    }

    /// Render generic arguments in angle bracket or parenthesized form.
    ///
    /// Handles three syntaxes:
    /// - Angle brackets: `<T, U, Item = V>` (most common)
    /// - Parenthesized: `(A, B) -> C` (for Fn traits)
    /// - Return type notation: `(..)` (experimental)
    fn render_generic_args(self, args: &GenericArgs) -> String {
        match args {
            // Standard angle bracket syntax: `<T, U, Item = V>`
            GenericArgs::AngleBracketed { args, constraints } => {
                // Collect all generic arguments (types, lifetimes, consts)
                let mut parts: Vec<Cow<str>> =
                    args.iter().map(|a| self.render_generic_arg(a)).collect();

                // Add associated type constraints (e.g., `Item = u32`)
                parts.extend(
                    constraints
                        .iter()
                        .map(|c| Cow::Owned(self.render_assoc_item_constraint(c))),
                );

                if parts.is_empty() {
                    String::new()
                } else {
                    format!("<{}>", parts.join(", "))
                }
            },

            // Parenthesized syntax for Fn traits: `Fn(A, B) -> C`
            GenericArgs::Parenthesized { inputs, output } => {
                let input_strs: Vec<Cow<str>> =
                    inputs.iter().map(|t| self.render_type(t)).collect();
                let ret = output
                    .as_ref()
                    .map(|t| format!(" -> {}", self.render_type(t)))
                    .unwrap_or_default();

                format!("({}){}", input_strs.join(", "), ret)
            },

            // Return type notation (experimental feature)
            GenericArgs::ReturnTypeNotation => " (..)".to_string(),
        }
    }

    /// Render a single generic argument.
    ///
    /// Arguments can be:
    /// - Lifetimes: `'a`, `'static`
    /// - Types: `T`, `Vec<u32>`
    /// - Const values: `N`, `{expr}`
    /// - Inferred: `_`
    fn render_generic_arg(self, arg: &GenericArg) -> Cow<'_, str> {
        match arg {
            // Zero allocation - borrow from input
            GenericArg::Lifetime(lt) => Cow::Borrowed(lt),

            GenericArg::Type(ty) => self.render_type(ty),

            // For consts, prefer the computed value; fall back to the expression
            // These are already owned strings in rustdoc_types
            GenericArg::Const(c) => Cow::Borrowed(c.value.as_deref().unwrap_or(&c.expr)),

            // Zero allocation - static string
            GenericArg::Infer => Cow::Borrowed("_"),
        }
    }

    /// Render an associated type constraint.
    ///
    /// These appear in generic bounds:
    /// - Equality: `Item = u32`
    /// - Bound: `Item: Display`
    fn render_assoc_item_constraint(self, constraint: &AssocItemConstraint) -> String {
        // The constraint may have its own generic args (rare)
        let args = constraint
            .args
            .as_ref()
            .map(|a| self.render_generic_args(a))
            .unwrap_or_default();

        match &constraint.binding {
            // Equality constraint: `Item = SomeType`
            AssocItemConstraintKind::Equality(term) => {
                format!("{}{args} = {}", constraint.name, self.render_term(term))
            },

            // Bound constraint: `Item: SomeTrait + OtherTrait`
            AssocItemConstraintKind::Constraint(bounds) => {
                let bound_strs: Vec<Cow<str>> = bounds
                    .iter()
                    .map(|b| self.render_generic_bound(b))
                    .collect();

                format!("{}{args}: {}", constraint.name, bound_strs.join(" + "))
            },
        }
    }

    /// Render a term, which is either a type or a constant.
    ///
    /// Used in associated type constraints like `Item = u32`.
    fn render_term(self, term: &Term) -> Cow<'_, str> {
        match term {
            Term::Type(ty) => self.render_type(ty),

            // For consts, prefer the computed value; fall back to the expression
            Term::Constant(c) => Cow::Borrowed(c.value.as_deref().unwrap_or(&c.expr)),
        }
    }

    /// Render a generic bound (trait bound or lifetime bound).
    ///
    /// # Examples
    ///
    /// - Trait bound: `Clone`, `Iterator<Item = T>`
    /// - Modified bound: `?Sized`, `~const Drop`
    /// - Lifetime bound: `'static`, `'a`
    #[must_use]
    pub fn render_generic_bound<'t>(&self, bound: &'t GenericBound) -> Cow<'t, str> {
        match bound {
            // Trait bound: `Clone`, `?Sized`, `~const Drop`
            GenericBound::TraitBound {
                trait_, modifier, ..
            } => {
                let sanitized = sanitize_path(&trait_.path);
                // Simple case: no modifier, no generic args, no sanitization - borrow directly
                if matches!(modifier, TraitBoundModifier::None)
                    && trait_.args.is_none()
                    && matches!(sanitized, Cow::Borrowed(_))
                {
                    return Cow::Borrowed(&trait_.path);
                }

                // Handle bound modifiers
                let modifier_str = match modifier {
                    TraitBoundModifier::None => "",
                    TraitBoundModifier::Maybe => "?", // ?Sized
                    TraitBoundModifier::MaybeConst => "~const ", // ~const Trait
                };

                // Build the trait path with any generic args
                let mut result = format!("{modifier_str}{sanitized}");

                if let Some(args) = &trait_.args {
                    result.push_str(&self.render_generic_args(args));
                }

                Cow::Owned(result)
            },

            // Lifetime bound: `'static`, `'a`
            // Zero allocation - borrow from input
            GenericBound::Outlives(lt) => Cow::Borrowed(lt),

            // Use bound (experimental) - typically empty in output
            GenericBound::Use(_) => Cow::Borrowed(""),
        }
    }

    /// Render a list of generic parameter definitions.
    ///
    /// Produces the `<T, U, const N: usize>` portion of a signature.
    ///
    /// # Arguments
    ///
    /// * `generics` - The list of generic parameters from rustdoc
    ///
    /// # Returns
    ///
    /// A string like `<T, U>` or empty string if no generics.
    ///
    /// # Filtering
    ///
    /// Synthetic parameters (generated by the compiler for `impl Trait`)
    /// are filtered out since they don't appear in the source.
    #[must_use]
    pub fn render_generics(&self, generics: &[GenericParamDef]) -> String {
        if generics.is_empty() {
            return String::new();
        }

        // Filter out synthetic params and render each remaining one
        let params: Vec<String> = generics
            .iter()
            .filter_map(|p| self.render_generic_param_def(p))
            .collect();

        if params.is_empty() {
            String::new()
        } else {
            format!("<{}>", params.join(", "))
        }
    }

    /// Render a single generic parameter definition.
    ///
    /// Returns `None` for synthetic parameters (compiler-generated).
    ///
    /// # Parameter Kinds
    ///
    /// - Lifetime: `'a`, `'a: 'b + 'c`
    /// - Type: `T`, `T: Clone + Send`
    /// - Const: `const N: usize`
    fn render_generic_param_def(self, param: &GenericParamDef) -> Option<String> {
        match &param.kind {
            // Lifetime parameter: `'a` or `'a: 'b + 'c`
            GenericParamDefKind::Lifetime { outlives } => {
                let mut result = param.name.clone();

                // Add outlives bounds if present
                if !outlives.is_empty() {
                    result.push_str(": ");
                    result.push_str(&outlives.join(" + "));
                }

                Some(result)
            },

            // Type parameter: `T` or `T: Clone + Send`
            GenericParamDefKind::Type {
                bounds,
                is_synthetic,
                ..
            } => {
                // Skip synthetic parameters (compiler-generated for impl Trait)
                if *is_synthetic {
                    return None;
                }

                let mut result = param.name.clone();

                // Add trait bounds if present
                if !bounds.is_empty() {
                    let bound_strs: Vec<Cow<str>> = bounds
                        .iter()
                        .map(|b| self.render_generic_bound(b))
                        .collect();
                    result.push_str(": ");
                    result.push_str(&bound_strs.join(" + "));
                }

                Some(result)
            },

            // Const parameter: `const N: usize`
            GenericParamDefKind::Const { type_, .. } => {
                Some(format!("const {}: {}", param.name, self.render_type(type_)))
            },
        }
    }

    /// Render where clause predicates.
    ///
    /// Produces the `where T: Clone, U: Send` portion of a signature.
    ///
    /// # Arguments
    ///
    /// * `where_predicates` - The list of where predicates from rustdoc
    ///
    /// # Returns
    ///
    /// A formatted where clause string, or empty string if no predicates.
    ///
    /// # Format
    ///
    /// ```text
    /// where
    ///     T: Clone,
    ///     U: Send
    /// ```
    #[must_use]
    pub fn render_where_clause(
        &self,
        where_predicates: &[rustdoc_types::WherePredicate],
    ) -> String {
        if where_predicates.is_empty() {
            return String::new();
        }

        let clauses: Vec<String> = where_predicates
            .iter()
            .map(|p| self.render_where_predicate(p))
            .collect();

        // Format with newline and indentation for readability
        format!("\nwhere\n    {}", clauses.join(",\n    "))
    }

    /// Render a single where predicate.
    ///
    /// # Predicate Types
    ///
    /// - Bound: `T: Clone + Send`
    /// - Lifetime: `'a: 'b + 'c`
    /// - Equality: `<T as Iterator>::Item = u32`
    fn render_where_predicate(self, pred: &rustdoc_types::WherePredicate) -> String {
        match pred {
            // Type bound predicate: `T: Clone + Send`
            rustdoc_types::WherePredicate::BoundPredicate { type_, bounds, .. } => {
                let bound_strs: Vec<Cow<str>> = bounds
                    .iter()
                    .map(|b| self.render_generic_bound(b))
                    .collect();

                format!("{}: {}", self.render_type(type_), bound_strs.join(" + "))
            },

            // Lifetime predicate: `'a: 'b + 'c`
            rustdoc_types::WherePredicate::LifetimePredicate { lifetime, outlives } => {
                format!("{lifetime}: {}", outlives.join(" + "))
            },

            // Equality predicate: `<T as Trait>::Item = SomeType`
            rustdoc_types::WherePredicate::EqPredicate { lhs, rhs } => {
                format!("{} = {}", self.render_type(lhs), self.render_term(rhs))
            },
        }
    }

    /// Collect all linkable type names from a type.
    ///
    /// This extracts type names that could potentially be linked to their definitions.
    /// Returns a set of (`ype_name`, `type_id`) pairs for `ResolvedPath` types.
    ///
    /// # Linkable Types
    ///
    /// - `ResolvedPath` types (e.g., `Vec`, `HashMap`, `MyStruct`)
    /// - Nested types within generics, references, slices, etc.
    ///
    /// # Excluded
    ///
    /// - Primitives (e.g., `u32`, `bool`)
    /// - Generic parameters (e.g., `T`, `U`)
    /// - Inferred types (`_`)
    #[must_use]
    pub fn collect_linkable_types(&self, ty: &Type) -> Vec<(String, rustdoc_types::Id)> {
        let mut result = Vec::new();
        self.collect_types_recursive(ty, &mut result);
        result
    }

    /// Recursively collect linkable types from a type tree.
    fn collect_types_recursive(self, ty: &Type, result: &mut Vec<(String, rustdoc_types::Id)>) {
        match ty {
            Type::ResolvedPath(path) => {
                // Extract the simple name (last segment of the path)
                // Using rsplit().next() is more efficient than split().last()
                let name = path.path.rsplit("::").next().unwrap_or(&path.path);
                result.push((name.to_string(), path.id));

                // Also collect from generic arguments
                if let Some(args) = &path.args {
                    self.collect_from_generic_args(args, result);
                }
            },

            Type::DynTrait(dyn_trait) => {
                for pt in &dyn_trait.traits {
                    let name = pt
                        .trait_
                        .path
                        .rsplit("::")
                        .next()
                        .unwrap_or(&pt.trait_.path);
                    result.push((name.to_string(), pt.trait_.id));

                    if let Some(args) = &pt.trait_.args {
                        self.collect_from_generic_args(args, result);
                    }
                }
            },

            Type::BorrowedRef { type_, .. } | Type::RawPointer { type_, .. } => {
                self.collect_types_recursive(type_, result);
            },

            Type::Slice(inner)
            | Type::Array { type_: inner, .. }
            | Type::Pat { type_: inner, .. } => {
                self.collect_types_recursive(inner, result);
            },

            Type::Tuple(types) => {
                for inner in types {
                    self.collect_types_recursive(inner, result);
                }
            },

            Type::FunctionPointer(fp) => {
                for (_, input_ty) in &fp.sig.inputs {
                    self.collect_types_recursive(input_ty, result);
                }
                if let Some(output) = &fp.sig.output {
                    self.collect_types_recursive(output, result);
                }
            },

            Type::ImplTrait(bounds) => {
                for bound in bounds {
                    if let GenericBound::TraitBound { trait_, .. } = bound {
                        let name = trait_.path.rsplit("::").next().unwrap_or(&trait_.path);
                        result.push((name.to_string(), trait_.id));

                        if let Some(args) = &trait_.args {
                            self.collect_from_generic_args(args, result);
                        }
                    }
                }
            },

            Type::QualifiedPath {
                self_type, trait_, ..
            } => {
                self.collect_types_recursive(self_type, result);
                if let Some(t) = trait_ {
                    let name = t.path.rsplit("::").next().unwrap_or(&t.path);
                    result.push((name.to_string(), t.id));
                }
            },

            // Primitives, generics, and inferred types are not linkable
            Type::Primitive(_) | Type::Generic(_) | Type::Infer => {},
        }
    }

    /// Collect types from generic arguments.
    fn collect_from_generic_args(
        self,
        args: &GenericArgs,
        result: &mut Vec<(String, rustdoc_types::Id)>,
    ) {
        match args {
            GenericArgs::AngleBracketed { args, constraints } => {
                for arg in args {
                    if let GenericArg::Type(ty) = arg {
                        self.collect_types_recursive(ty, result);
                    }
                }
                for constraint in constraints {
                    if let AssocItemConstraintKind::Equality(Term::Type(ty)) = &constraint.binding {
                        self.collect_types_recursive(ty, result);
                    }
                }
            },
            GenericArgs::Parenthesized { inputs, output } => {
                for input in inputs {
                    self.collect_types_recursive(input, result);
                }
                if let Some(output) = output {
                    self.collect_types_recursive(output, result);
                }
            },
            GenericArgs::ReturnTypeNotation => {},
        }
    }
}
