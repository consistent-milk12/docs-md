//! Type rendering utilities for converting rustdoc types to string representations.

use rustdoc_types::{
    AssocItemConstraint, AssocItemConstraintKind, Crate, GenericArg, GenericArgs, GenericBound,
    GenericParamDef, GenericParamDefKind, Term, TraitBoundModifier, Type,
};

/// Render a Type to its string representation
pub fn render_type(ty: &Type, krate: &Crate) -> String {
    match ty {
        Type::ResolvedPath(path) => {
            let mut result = path.path.clone();
            if let Some(args) = &path.args {
                result.push_str(&render_generic_args(args, krate));
            }
            result
        }
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
        Type::Generic(name) => name.clone(),
        Type::Primitive(name) => name.clone(),
        Type::FunctionPointer(fp) => {
            let params: Vec<String> = fp
                .sig
                .inputs
                .iter()
                .map(|(_, t)| render_type(t, krate))
                .collect();
            let ret = if let Some(output) = &fp.sig.output {
                format!(" -> {}", render_type(output, krate))
            } else {
                String::new()
            };
            format!("fn({}){}", params.join(", "), ret)
        }
        Type::Tuple(types) => {
            let inner: Vec<String> = types.iter().map(|t| render_type(t, krate)).collect();
            format!("({})", inner.join(", "))
        }
        Type::Slice(inner) => format!("[{}]", render_type(inner, krate)),
        Type::Array { type_, len } => format!("[{}; {}]", render_type(type_, krate), len),
        Type::Pat { type_, .. } => render_type(type_, krate),
        Type::ImplTrait(bounds) => {
            let bound_strs: Vec<String> = bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!("impl {}", bound_strs.join(" + "))
        }
        Type::Infer => "_".to_string(),
        Type::RawPointer { is_mutable, type_ } => {
            let mutability = if *is_mutable { "mut" } else { "const" };
            format!("*{} {}", mutability, render_type(type_, krate))
        }
        Type::BorrowedRef {
            lifetime,
            is_mutable,
            type_,
        } => {
            let lt = lifetime
                .as_ref()
                .map(|l| format!("{} ", l))
                .unwrap_or_default();
            let mutability = if *is_mutable { "mut " } else { "" };
            format!("&{}{}{}", lt, mutability, render_type(type_, krate))
        }
        Type::QualifiedPath {
            name,
            self_type,
            trait_,
            ..
        } => {
            let self_ty = render_type(self_type, krate);
            if let Some(trait_path) = trait_ {
                format!("<{} as {}>::{}", self_ty, trait_path.path, name)
            } else {
                format!("{}::{}", self_ty, name)
            }
        }
    }
}

/// Render generic arguments
fn render_generic_args(args: &GenericArgs, krate: &Crate) -> String {
    match args {
        GenericArgs::AngleBracketed { args, constraints } => {
            let mut parts: Vec<String> =
                args.iter().map(|a| render_generic_arg(a, krate)).collect();
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
        GenericArgs::Parenthesized { inputs, output } => {
            let input_strs: Vec<String> = inputs.iter().map(|t| render_type(t, krate)).collect();
            let ret = output
                .as_ref()
                .map(|t| format!(" -> {}", render_type(t, krate)))
                .unwrap_or_default();
            format!("({}){}", input_strs.join(", "), ret)
        }
        GenericArgs::ReturnTypeNotation => " (..)".to_string(),
    }
}

/// Render a single generic argument
fn render_generic_arg(arg: &GenericArg, krate: &Crate) -> String {
    match arg {
        GenericArg::Lifetime(lt) => lt.clone(),
        GenericArg::Type(ty) => render_type(ty, krate),
        GenericArg::Const(c) => c.value.clone().unwrap_or_else(|| c.expr.clone()),
        GenericArg::Infer => "_".to_string(),
    }
}

/// Render an associated item constraint (e.g., `Item = u32`)
fn render_assoc_item_constraint(constraint: &AssocItemConstraint, krate: &Crate) -> String {
    let args = constraint
        .args
        .as_ref()
        .map(|a| render_generic_args(a, krate))
        .unwrap_or_default();
    match &constraint.binding {
        AssocItemConstraintKind::Equality(term) => {
            format!("{}{} = {}", constraint.name, args, render_term(term, krate))
        }
        AssocItemConstraintKind::Constraint(bounds) => {
            let bound_strs: Vec<String> = bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!("{}{}: {}", constraint.name, args, bound_strs.join(" + "))
        }
    }
}

/// Render a term (type or constant)
fn render_term(term: &Term, krate: &Crate) -> String {
    match term {
        Term::Type(ty) => render_type(ty, krate),
        Term::Constant(c) => c.value.clone().unwrap_or_else(|| c.expr.clone()),
    }
}

/// Render a generic bound
pub fn render_generic_bound(bound: &GenericBound, krate: &Crate) -> String {
    match bound {
        GenericBound::TraitBound {
            trait_, modifier, ..
        } => {
            let modifier_str = match modifier {
                TraitBoundModifier::None => "",
                TraitBoundModifier::Maybe => "?",
                TraitBoundModifier::MaybeConst => "~const ",
            };
            let mut result = format!("{}{}", modifier_str, trait_.path);
            if let Some(args) = &trait_.args {
                result.push_str(&render_generic_args(args, krate));
            }
            result
        }
        GenericBound::Outlives(lt) => lt.clone(),
        GenericBound::Use(_) => String::new(),
    }
}

/// Render generic parameter definitions
pub fn render_generics(generics: &[GenericParamDef], krate: &Crate) -> String {
    if generics.is_empty() {
        return String::new();
    }

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

/// Render a single generic parameter definition
fn render_generic_param_def(param: &GenericParamDef, krate: &Crate) -> Option<String> {
    match &param.kind {
        GenericParamDefKind::Lifetime { outlives } => {
            let mut result = param.name.clone();
            if !outlives.is_empty() {
                result.push_str(": ");
                result.push_str(&outlives.join(" + "));
            }
            Some(result)
        }
        GenericParamDefKind::Type {
            bounds,
            is_synthetic,
            ..
        } => {
            if *is_synthetic {
                return None;
            }
            let mut result = param.name.clone();
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
        GenericParamDefKind::Const { type_, .. } => Some(format!(
            "const {}: {}",
            param.name,
            render_type(type_, krate)
        )),
    }
}

/// Render where clause predicates
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

    format!("\nwhere\n    {}", clauses.join(",\n    "))
}

fn render_where_predicate(pred: &rustdoc_types::WherePredicate, krate: &Crate) -> String {
    match pred {
        rustdoc_types::WherePredicate::BoundPredicate { type_, bounds, .. } => {
            let bound_strs: Vec<String> = bounds
                .iter()
                .map(|b| render_generic_bound(b, krate))
                .collect();
            format!("{}: {}", render_type(type_, krate), bound_strs.join(" + "))
        }
        rustdoc_types::WherePredicate::LifetimePredicate { lifetime, outlives } => {
            format!("{}: {}", lifetime, outlives.join(" + "))
        }
        rustdoc_types::WherePredicate::EqPredicate { lhs, rhs } => {
            format!("{} = {}", render_type(lhs, krate), render_term(rhs, krate))
        }
    }
}
