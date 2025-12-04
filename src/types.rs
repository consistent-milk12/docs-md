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

use rustdoc_types::{
    AssocItemConstraint, AssocItemConstraintKind, Crate, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Term, TraitBoundModifier, Type,
};

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
    pub fn new(krate: &'a Crate) -> Self {
        Self { krate }
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
    pub fn render_type(&self, ty: &Type) -> String {
        match ty {
            // Named type path like `Vec<T>` or `std::collections::HashMap<K, V>`
            Type::ResolvedPath(path) => {
                let mut result = path.path.clone();
                // Append generic arguments if present
                if let Some(args) = &path.args {
                    result.push_str(&self.render_generic_args(args));
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
                            s.push_str(&self.render_generic_args(args));
                        }
                        s
                    })
                    .collect();
                format!("dyn {}", traits.join(" + "))
            }

            // Generic type parameter: `T`, `U`, etc.
            // Primitive type: `u32`, `bool`, `str`, etc.
            Type::Generic(name) | Type::Primitive(name) => name.clone(),

            // Function pointer: `fn(A, B) -> C`
            Type::FunctionPointer(fp) => {
                // Render parameter types (we ignore parameter names for brevity)
                let params: Vec<String> = fp
                    .sig
                    .inputs
                    .iter()
                    .map(|(_, t)| self.render_type(t))
                    .collect();
                // Render return type if present
                let ret = if let Some(output) = &fp.sig.output {
                    format!(" -> {}", self.render_type(output))
                } else {
                    String::new()
                };
                format!("fn({}){}", params.join(", "), ret)
            }

            // Tuple type: `(A, B, C)` or unit `()`
            Type::Tuple(types) => {
                let inner: Vec<String> = types.iter().map(|t| self.render_type(t)).collect();
                format!("({})", inner.join(", "))
            }

            // Slice type: `[T]`
            Type::Slice(inner) => format!("[{}]", self.render_type(inner)),

            // Array type: `[T; N]` where N is a const expression
            Type::Array { type_, len } => format!("[{}; {len}]", self.render_type(type_)),

            // Pattern type (rare): just render the underlying type
            Type::Pat { type_, .. } => self.render_type(type_),

            // Impl trait in return position: `impl Iterator<Item = T>`
            Type::ImplTrait(bounds) => {
                let bound_strs: Vec<String> = bounds
                    .iter()
                    .map(|b| self.render_generic_bound(b))
                    .collect();
                format!("impl {}", bound_strs.join(" + "))
            }

            // Inferred type: `_` (placeholder in turbofish)
            Type::Infer => "_".to_string(),

            // Raw pointer: `*const T` or `*mut T`
            Type::RawPointer { is_mutable, type_ } => {
                let mutability = if *is_mutable { "mut" } else { "const" };
                format!("*{mutability} {}", self.render_type(type_))
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
                format!("&{lt}{mutability}{}", self.render_type(type_))
            }

            // Qualified path: `<T as Trait>::Item` or `T::Item`
            Type::QualifiedPath {
                name,
                self_type,
                trait_,
                ..
            } => {
                let self_ty = self.render_type(self_type);
                if let Some(trait_path) = trait_ {
                    // Full qualification: <Self as Trait>::Item
                    format!("<{self_ty} as {}>::{name}", trait_path.path)
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
    fn render_generic_args(self, args: &GenericArgs) -> String {
        match args {
            // Standard angle bracket syntax: `<T, U, Item = V>`
            GenericArgs::AngleBracketed { args, constraints } => {
                // Collect all generic arguments (types, lifetimes, consts)
                let mut parts: Vec<String> =
                    args.iter().map(|a| self.render_generic_arg(a)).collect();

                // Add associated type constraints (e.g., `Item = u32`)
                parts.extend(
                    constraints
                        .iter()
                        .map(|c| self.render_assoc_item_constraint(c)),
                );

                if parts.is_empty() {
                    String::new()
                } else {
                    format!("<{}>", parts.join(", "))
                }
            }

            // Parenthesized syntax for Fn traits: `Fn(A, B) -> C`
            GenericArgs::Parenthesized { inputs, output } => {
                let input_strs: Vec<String> = inputs.iter().map(|t| self.render_type(t)).collect();
                let ret = output
                    .as_ref()
                    .map(|t| format!(" -> {}", self.render_type(t)))
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
    fn render_generic_arg(self, arg: &GenericArg) -> String {
        match arg {
            GenericArg::Lifetime(lt) => lt.clone(),
            GenericArg::Type(ty) => self.render_type(ty),
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
            }
            // Bound constraint: `Item: SomeTrait + OtherTrait`
            AssocItemConstraintKind::Constraint(bounds) => {
                let bound_strs: Vec<String> = bounds
                    .iter()
                    .map(|b| self.render_generic_bound(b))
                    .collect();
                format!("{}{args}: {}", constraint.name, bound_strs.join(" + "))
            }
        }
    }

    /// Render a term, which is either a type or a constant.
    ///
    /// Used in associated type constraints like `Item = u32`.
    fn render_term(self, term: &Term) -> String {
        match term {
            Term::Type(ty) => self.render_type(ty),
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
    pub fn render_generic_bound(&self, bound: &GenericBound) -> String {
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
                let mut result = format!("{modifier_str}{}", trait_.path);
                if let Some(args) = &trait_.args {
                    result.push_str(&self.render_generic_args(args));
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
                        .map(|b| self.render_generic_bound(b))
                        .collect();
                    result.push_str(": ");
                    result.push_str(&bound_strs.join(" + "));
                }
                Some(result)
            }

            // Const parameter: `const N: usize`
            GenericParamDefKind::Const { type_, .. } => {
                Some(format!("const {}: {}", param.name, self.render_type(type_)))
            }
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
                let bound_strs: Vec<String> = bounds
                    .iter()
                    .map(|b| self.render_generic_bound(b))
                    .collect();
                format!("{}: {}", self.render_type(type_), bound_strs.join(" + "))
            }

            // Lifetime predicate: `'a: 'b + 'c`
            rustdoc_types::WherePredicate::LifetimePredicate { lifetime, outlives } => {
                format!("{lifetime}: {}", outlives.join(" + "))
            }

            // Equality predicate: `<T as Trait>::Item = SomeType`
            rustdoc_types::WherePredicate::EqPredicate { lhs, rhs } => {
                format!("{} = {}", self.render_type(lhs), self.render_term(rhs))
            }
        }
    }
}
