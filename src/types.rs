//! Type rendering utilities for converting rustdoc types to string representations.
//!
//! This module provides functions to convert the complex type structures from
//! rustdoc JSON into human-readable Rust type syntax. These rendered strings
//! are used in the generated markdown documentation.
//!
//! # Overview
//!
//! Rustdoc JSON represents types as a tree structure (the `Type` enum). This module
//! walks that tree and produces the string representation you'd write in code.
//!
//! # Key Functions
//!
//! - [`render_type`] - Main entry point for type rendering
//! - [`render_generics`] - Render generic parameter lists (`<T, U>`)
//! - [`render_where_clause`] - Render where clauses
//! - [`render_generic_bound`] - Render trait bounds (`T: Clone + Send`)
//!
//! # Example Transformations
//!
//! | Rustdoc Type | Rendered String |
//! |--------------|-----------------|
//! | `Type::Primitive("u32")` | `"u32"` |
//! | `Type::BorrowedRef { lifetime: Some("'a"), is_mutable: true, type_: ... }` | `"&'a mut T"` |
//! | `Type::ResolvedPath { path: "Vec", args: ... }` | `"Vec<T>"` |

use rustdoc_types::{
    AssocItemConstraint, AssocItemConstraintKind, Crate, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Term, TraitBoundModifier, Type,
};

/// Render a rustdoc `Type` to its Rust syntax string representation.
///
/// This is the main entry point for type rendering. It handles all variants
/// of the `Type` enum and recursively renders nested types.
///
/// # Arguments
///
/// * `ty` - The type to render
/// * `krate` - The crate context (needed for looking up path information)
///
/// # Returns
///
/// A string representing the type in Rust syntax.
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
pub fn render_type(ty: &Type, krate: &Crate) -> String {
    match ty {
        // Named type path like `Vec<T>` or `std::collections::HashMap<K, V>`
        Type::ResolvedPath(path) => {
            let mut result = path.path.clone();
            // Append generic arguments if present
            if let Some(args) = &path.args {
                result.push_str(&render_generic_args(args, krate));
            }
            result
        }

        // Trait object: `dyn Trait + OtherTrait`
        Type::DynTrait(dyn_trait) => {
            let traits: Vec<String> = dyn_trait
                .traits
                .iter()
                .map(|pt| {
                    let mut s = pt.trait_.path.clone();
                    if let Some(args) = &pt.trait_.args {
                        s.push_str(&render_generic_args(args, krate));
                    }
                    s
                })
                .collect();
            format!("dyn {}", traits.join(" + "))
        }

        // Generic type parameter: `T`, `U`, etc.
        Type::Generic(name) | Type::Primitive(name) => name.clone(),

        // Primitive type: `u32`, `bool`, `str`, etc.
        // Function pointer: `fn(A, B) -> C`
        Type::FunctionPointer(fp) => {
            // Render parameter types (we ignore parameter names for brevity)
            let params: Vec<String> = fp
                .sig
                .inputs
                .iter()
                .map(|(_, t)| render_type(t, krate))
                .collect();
            // Render return type if present
            let ret = if let Some(output) = &fp.sig.output {
                format!(" -> {}", render_type(output, krate))
            } else {
                String::new()
            };
            format!("fn({}){}", params.join(", "), ret)
        }

        // Tuple type: `(A, B, C)` or unit `()`
        Type::Tuple(types) => {
            let inner: Vec<String> = types.iter().map(|t| render_type(t, krate)).collect();
            format!("({})", inner.join(", "))
        }

        // Slice type: `[T]`
        Type::Slice(inner) => format!("[{}]", render_type(inner, krate)),

        // Array type: `[T; N]` where N is a const expression
        Type::Array { type_, len } => format!("[{}; {}]", render_type(type_, krate), len),

        // Pattern type (rare): just render the underlying type
        Type::Pat { type_, .. } => render_type(type_, krate),

        // Impl trait in return position: `impl Iterator<Item = T>`
        Type::ImplTrait(bounds) => {
            let bound_strs: Vec<String> = bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!("impl {}", bound_strs.join(" + "))
        }

        // Inferred type: `_` (placeholder in turbofish)
        Type::Infer => "_".to_string(),

        // Raw pointer: `*const T` or `*mut T`
        Type::RawPointer { is_mutable, type_ } => {
            let mutability = if *is_mutable { "mut" } else { "const" };
            format!("*{} {}", mutability, render_type(type_, krate))
        }

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
            format!("&{}{}{}", lt, mutability, render_type(type_, krate))
        }

        // Qualified path: `<T as Trait>::Item` or `T::Item`
        Type::QualifiedPath {
            name,
            self_type,
            trait_,
            ..
        } => {
            let self_ty = render_type(self_type, krate);
            if let Some(trait_path) = trait_ {
                // Full qualification: <Self as Trait>::Item
                format!("<{} as {}>::{}", self_ty, trait_path.path, name)
            } else {
                // Simple associated type: Self::Item
                format!("{self_ty}::{name}")
            }
        }
    }
}

/// Render generic arguments in angle bracket or parenthesized form.
///
/// Handles three syntaxes:
/// - Angle brackets: `<T, U, Item = V>` (most common)
/// - Parenthesized: `(A, B) -> C` (for Fn traits)
/// - Return type notation: `(..)` (experimental)
fn render_generic_args(args: &GenericArgs, krate: &Crate) -> String {
    match args {
        // Standard angle bracket syntax: `<T, U, Item = V>`
        GenericArgs::AngleBracketed { args, constraints } => {
            // Collect all generic arguments (types, lifetimes, consts)
            let mut parts: Vec<String> =
                args.iter().map(|a| render_generic_arg(a, krate)).collect();

            // Add associated type constraints (e.g., `Item = u32`)
            parts.extend(
                constraints
                    .iter()
                    .map(|c| render_assoc_item_constraint(c, krate)),
            );

            if parts.is_empty() {
                String::new()
            } else {
                format!("<{}>", parts.join(", "))
            }
        }

        // Parenthesized syntax for Fn traits: `Fn(A, B) -> C`
        GenericArgs::Parenthesized { inputs, output } => {
            let input_strs: Vec<String> = inputs.iter().map(|t| render_type(t, krate)).collect();
            let ret = output
                .as_ref()
                .map(|t| format!(" -> {}", render_type(t, krate)))
                .unwrap_or_default();
            format!("({}){}", input_strs.join(", "), ret)
        }

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
fn render_generic_arg(arg: &GenericArg, krate: &Crate) -> String {
    match arg {
        GenericArg::Lifetime(lt) => lt.clone(),
        GenericArg::Type(ty) => render_type(ty, krate),
        // For consts, prefer the computed value; fall back to the expression
        GenericArg::Const(c) => c.value.clone().unwrap_or_else(|| c.expr.clone()),
        GenericArg::Infer => "_".to_string(),
    }
}

/// Render an associated type constraint.
///
/// These appear in generic bounds:
/// - Equality: `Item = u32`
/// - Bound: `Item: Display`
fn render_assoc_item_constraint(constraint: &AssocItemConstraint, krate: &Crate) -> String {
    // The constraint may have its own generic args (rare)
    let args = constraint
        .args
        .as_ref()
        .map(|a| render_generic_args(a, krate))
        .unwrap_or_default();

    match &constraint.binding {
        // Equality constraint: `Item = SomeType`
        AssocItemConstraintKind::Equality(term) => {
            format!("{}{} = {}", constraint.name, args, render_term(term, krate))
        }
        // Bound constraint: `Item: SomeTrait + OtherTrait`
        AssocItemConstraintKind::Constraint(bounds) => {
            let bound_strs: Vec<String> = bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!("{}{}: {}", constraint.name, args, bound_strs.join(" + "))
        }
    }
}

/// Render a term, which is either a type or a constant.
///
/// Used in associated type constraints like `Item = u32`.
fn render_term(term: &Term, krate: &Crate) -> String {
    match term {
        Term::Type(ty) => render_type(ty, krate),
        // For consts, prefer the computed value; fall back to the expression
        Term::Constant(c) => c.value.clone().unwrap_or_else(|| c.expr.clone()),
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
pub fn render_generic_bound(bound: &GenericBound, krate: &Crate) -> String {
    match bound {
        // Trait bound: `Clone`, `?Sized`, `~const Drop`
        GenericBound::TraitBound {
            trait_, modifier, ..
        } => {
            // Handle bound modifiers
            let modifier_str = match modifier {
                TraitBoundModifier::None => "",
                TraitBoundModifier::Maybe => "?", // ?Sized
                TraitBoundModifier::MaybeConst => "~const ", // ~const Trait
            };

            // Build the trait path with any generic args
            let mut result = format!("{}{}", modifier_str, trait_.path);
            if let Some(args) = &trait_.args {
                result.push_str(&render_generic_args(args, krate));
            }
            result
        }

        // Lifetime bound: `'static`, `'a`
        GenericBound::Outlives(lt) => lt.clone(),

        // Use bound (experimental) - typically empty in output
        GenericBound::Use(_) => String::new(),
    }
}

/// Render a list of generic parameter definitions.
///
/// Produces the `<T, U, const N: usize>` portion of a signature.
///
/// # Arguments
///
/// * `generics` - The list of generic parameters from rustdoc
/// * `krate` - The crate context
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
pub fn render_generics(generics: &[GenericParamDef], krate: &Crate) -> String {
    if generics.is_empty() {
        return String::new();
    }

    // Filter out synthetic params and render each remaining one
    let params: Vec<String> = generics
        .iter()
        .filter_map(|p| render_generic_param_def(p, krate))
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
fn render_generic_param_def(param: &GenericParamDef, krate: &Crate) -> Option<String> {
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
        }

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
                let bound_strs: Vec<String> = bounds
                    .iter()
                    .map(|b| render_generic_bound(b, krate))
                    .collect();
                result.push_str(": ");
                result.push_str(&bound_strs.join(" + "));
            }
            Some(result)
        }

        // Const parameter: `const N: usize`
        GenericParamDefKind::Const { type_, .. } => Some(format!(
            "const {}: {}",
            param.name,
            render_type(type_, krate)
        )),
    }
}

/// Render where clause predicates.
///
/// Produces the `where T: Clone, U: Send` portion of a signature.
///
/// # Arguments
///
/// * `where_predicates` - The list of where predicates from rustdoc
/// * `krate` - The crate context
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
    where_predicates: &[rustdoc_types::WherePredicate],
    krate: &Crate,
) -> String {
    if where_predicates.is_empty() {
        return String::new();
    }

    let clauses: Vec<String> = where_predicates
        .iter()
        .map(|p| render_where_predicate(p, krate))
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
fn render_where_predicate(pred: &rustdoc_types::WherePredicate, krate: &Crate) -> String {
    match pred {
        // Type bound predicate: `T: Clone + Send`
        rustdoc_types::WherePredicate::BoundPredicate { type_, bounds, .. } => {
            let bound_strs: Vec<String> = bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!("{}: {}", render_type(type_, krate), bound_strs.join(" + "))
        }

        // Lifetime predicate: `'a: 'b + 'c`
        rustdoc_types::WherePredicate::LifetimePredicate { lifetime, outlives } => {
            format!("{}: {}", lifetime, outlives.join(" + "))
        }

        // Equality predicate: `<T as Trait>::Item = SomeType`
        rustdoc_types::WherePredicate::EqPredicate { lhs, rhs } => {
            format!("{} = {}", render_type(lhs, krate), render_term(rhs, krate))
        }
    }
}
