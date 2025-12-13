*[syn](../../index.md) / [gen](../index.md) / [visit](index.md)*

---

# Module `visit`

Syntax tree traversal to walk a shared borrow of a syntax tree.

Each method of the [`Visit`](#visit) trait is a hook that can be overridden to
customize the behavior when visiting the corresponding type of node. By
default, every method recursively visits the substructure of the input
by invoking the right visitor method of each of its fields.

```rust
use syn::{Attribute, BinOp, Expr, ExprBinary};

pub trait Visit<'ast> {
    /* ... */

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        visit_expr_binary(self, node);
    }

    /* ... */
    fn visit_attribute(&mut self, node: &'ast Attribute);
    fn visit_expr(&mut self, node: &'ast Expr);
    fn visit_bin_op(&mut self, node: &'ast BinOp);
}

pub fn visit_expr_binary<'ast, V>(v: &mut V, node: &'ast ExprBinary)
where
    V: Visit<'ast> + ?Sized,
{
    for attr in &node.attrs {
        v.visit_attribute(attr);
    }
    v.visit_expr(&*node.left);
    v.visit_bin_op(&node.op);
    v.visit_expr(&*node.right);
}

/* ... */
```

<br>

# Example

This visitor will print the name of every freestanding function in the
syntax tree, including nested functions.

```rust
// [dependencies]
// quote = "1.0"
// syn = { version = "2.0", features = ["full", "visit"] }

use quote::quote;
use syn::visit::{self, Visit};
use syn::{File, ItemFn};

struct FnVisitor;

impl<'ast> Visit<'ast> for FnVisitor {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        println!("Function with name={}", node.sig.ident);

        // Delegate to the default impl to visit any nested functions.
        visit::visit_item_fn(self, node);
    }
}

fn main() {
    let code = quote! {
        pub fn f() {
            fn g() {}
        }
    };

    let syntax_tree: File = syn::parse2(code).unwrap();
    FnVisitor.visit_file(&syntax_tree);
}
```

The `'ast` lifetime on the input references means that the syntax tree
outlives the complete recursive visit call, so the visitor is allowed to
hold on to references into the syntax tree.

```rust
use quote::quote;
use syn::visit::{self, Visit};
use syn::{File, ItemFn};

struct FnVisitor<'ast> {
    functions: Vec<&'ast ItemFn>,
}

impl<'ast> Visit<'ast> for FnVisitor<'ast> {
    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(node);
        visit::visit_item_fn(self, node);
    }
}

fn main() {
    let code = quote! {
        pub fn f() {
            fn g() {}
        }
    };

    let syntax_tree: File = syn::parse2(code).unwrap();
    let mut visitor = FnVisitor { functions: Vec::new() };
    visitor.visit_file(&syntax_tree);
    for f in visitor.functions {
        println!("Function with name={}", f.sig.ident);
    }
}
```

## Contents

- [Traits](#traits)
  - [`Visit`](#visit)
- [Functions](#functions)
  - [`visit_abi`](#visit-abi)
  - [`visit_angle_bracketed_generic_arguments`](#visit-angle-bracketed-generic-arguments)
  - [`visit_arm`](#visit-arm)
  - [`visit_assoc_const`](#visit-assoc-const)
  - [`visit_assoc_type`](#visit-assoc-type)
  - [`visit_attr_style`](#visit-attr-style)
  - [`visit_attribute`](#visit-attribute)
  - [`visit_bare_fn_arg`](#visit-bare-fn-arg)
  - [`visit_bare_variadic`](#visit-bare-variadic)
  - [`visit_bin_op`](#visit-bin-op)
  - [`visit_block`](#visit-block)
  - [`visit_bound_lifetimes`](#visit-bound-lifetimes)
  - [`visit_captured_param`](#visit-captured-param)
  - [`visit_const_param`](#visit-const-param)
  - [`visit_constraint`](#visit-constraint)
  - [`visit_data`](#visit-data)
  - [`visit_data_enum`](#visit-data-enum)
  - [`visit_data_struct`](#visit-data-struct)
  - [`visit_data_union`](#visit-data-union)
  - [`visit_derive_input`](#visit-derive-input)
  - [`visit_expr`](#visit-expr)
  - [`visit_expr_array`](#visit-expr-array)
  - [`visit_expr_assign`](#visit-expr-assign)
  - [`visit_expr_async`](#visit-expr-async)
  - [`visit_expr_await`](#visit-expr-await)
  - [`visit_expr_binary`](#visit-expr-binary)
  - [`visit_expr_block`](#visit-expr-block)
  - [`visit_expr_break`](#visit-expr-break)
  - [`visit_expr_call`](#visit-expr-call)
  - [`visit_expr_cast`](#visit-expr-cast)
  - [`visit_expr_closure`](#visit-expr-closure)
  - [`visit_expr_const`](#visit-expr-const)
  - [`visit_expr_continue`](#visit-expr-continue)
  - [`visit_expr_field`](#visit-expr-field)
  - [`visit_expr_for_loop`](#visit-expr-for-loop)
  - [`visit_expr_group`](#visit-expr-group)
  - [`visit_expr_if`](#visit-expr-if)
  - [`visit_expr_index`](#visit-expr-index)
  - [`visit_expr_infer`](#visit-expr-infer)
  - [`visit_expr_let`](#visit-expr-let)
  - [`visit_expr_lit`](#visit-expr-lit)
  - [`visit_expr_loop`](#visit-expr-loop)
  - [`visit_expr_macro`](#visit-expr-macro)
  - [`visit_expr_match`](#visit-expr-match)
  - [`visit_expr_method_call`](#visit-expr-method-call)
  - [`visit_expr_paren`](#visit-expr-paren)
  - [`visit_expr_path`](#visit-expr-path)
  - [`visit_expr_range`](#visit-expr-range)
  - [`visit_expr_raw_addr`](#visit-expr-raw-addr)
  - [`visit_expr_reference`](#visit-expr-reference)
  - [`visit_expr_repeat`](#visit-expr-repeat)
  - [`visit_expr_return`](#visit-expr-return)
  - [`visit_expr_struct`](#visit-expr-struct)
  - [`visit_expr_try`](#visit-expr-try)
  - [`visit_expr_try_block`](#visit-expr-try-block)
  - [`visit_expr_tuple`](#visit-expr-tuple)
  - [`visit_expr_unary`](#visit-expr-unary)
  - [`visit_expr_unsafe`](#visit-expr-unsafe)
  - [`visit_expr_while`](#visit-expr-while)
  - [`visit_expr_yield`](#visit-expr-yield)
  - [`visit_field`](#visit-field)
  - [`visit_field_mutability`](#visit-field-mutability)
  - [`visit_field_pat`](#visit-field-pat)
  - [`visit_field_value`](#visit-field-value)
  - [`visit_fields`](#visit-fields)
  - [`visit_fields_named`](#visit-fields-named)
  - [`visit_fields_unnamed`](#visit-fields-unnamed)
  - [`visit_file`](#visit-file)
  - [`visit_fn_arg`](#visit-fn-arg)
  - [`visit_foreign_item`](#visit-foreign-item)
  - [`visit_foreign_item_fn`](#visit-foreign-item-fn)
  - [`visit_foreign_item_macro`](#visit-foreign-item-macro)
  - [`visit_foreign_item_static`](#visit-foreign-item-static)
  - [`visit_foreign_item_type`](#visit-foreign-item-type)
  - [`visit_generic_argument`](#visit-generic-argument)
  - [`visit_generic_param`](#visit-generic-param)
  - [`visit_generics`](#visit-generics)
  - [`visit_ident`](#visit-ident)
  - [`visit_impl_item`](#visit-impl-item)
  - [`visit_impl_item_const`](#visit-impl-item-const)
  - [`visit_impl_item_fn`](#visit-impl-item-fn)
  - [`visit_impl_item_macro`](#visit-impl-item-macro)
  - [`visit_impl_item_type`](#visit-impl-item-type)
  - [`visit_impl_restriction`](#visit-impl-restriction)
  - [`visit_index`](#visit-index)
  - [`visit_item`](#visit-item)
  - [`visit_item_const`](#visit-item-const)
  - [`visit_item_enum`](#visit-item-enum)
  - [`visit_item_extern_crate`](#visit-item-extern-crate)
  - [`visit_item_fn`](#visit-item-fn)
  - [`visit_item_foreign_mod`](#visit-item-foreign-mod)
  - [`visit_item_impl`](#visit-item-impl)
  - [`visit_item_macro`](#visit-item-macro)
  - [`visit_item_mod`](#visit-item-mod)
  - [`visit_item_static`](#visit-item-static)
  - [`visit_item_struct`](#visit-item-struct)
  - [`visit_item_trait`](#visit-item-trait)
  - [`visit_item_trait_alias`](#visit-item-trait-alias)
  - [`visit_item_type`](#visit-item-type)
  - [`visit_item_union`](#visit-item-union)
  - [`visit_item_use`](#visit-item-use)
  - [`visit_label`](#visit-label)
  - [`visit_lifetime`](#visit-lifetime)
  - [`visit_lifetime_param`](#visit-lifetime-param)
  - [`visit_lit`](#visit-lit)
  - [`visit_lit_bool`](#visit-lit-bool)
  - [`visit_lit_byte`](#visit-lit-byte)
  - [`visit_lit_byte_str`](#visit-lit-byte-str)
  - [`visit_lit_cstr`](#visit-lit-cstr)
  - [`visit_lit_char`](#visit-lit-char)
  - [`visit_lit_float`](#visit-lit-float)
  - [`visit_lit_int`](#visit-lit-int)
  - [`visit_lit_str`](#visit-lit-str)
  - [`visit_local`](#visit-local)
  - [`visit_local_init`](#visit-local-init)
  - [`visit_macro`](#visit-macro)
  - [`visit_macro_delimiter`](#visit-macro-delimiter)
  - [`visit_member`](#visit-member)
  - [`visit_meta`](#visit-meta)
  - [`visit_meta_list`](#visit-meta-list)
  - [`visit_meta_name_value`](#visit-meta-name-value)
  - [`visit_parenthesized_generic_arguments`](#visit-parenthesized-generic-arguments)
  - [`visit_pat`](#visit-pat)
  - [`visit_pat_ident`](#visit-pat-ident)
  - [`visit_pat_or`](#visit-pat-or)
  - [`visit_pat_paren`](#visit-pat-paren)
  - [`visit_pat_reference`](#visit-pat-reference)
  - [`visit_pat_rest`](#visit-pat-rest)
  - [`visit_pat_slice`](#visit-pat-slice)
  - [`visit_pat_struct`](#visit-pat-struct)
  - [`visit_pat_tuple`](#visit-pat-tuple)
  - [`visit_pat_tuple_struct`](#visit-pat-tuple-struct)
  - [`visit_pat_type`](#visit-pat-type)
  - [`visit_pat_wild`](#visit-pat-wild)
  - [`visit_path`](#visit-path)
  - [`visit_path_arguments`](#visit-path-arguments)
  - [`visit_path_segment`](#visit-path-segment)
  - [`visit_pointer_mutability`](#visit-pointer-mutability)
  - [`visit_precise_capture`](#visit-precise-capture)
  - [`visit_predicate_lifetime`](#visit-predicate-lifetime)
  - [`visit_predicate_type`](#visit-predicate-type)
  - [`visit_qself`](#visit-qself)
  - [`visit_range_limits`](#visit-range-limits)
  - [`visit_receiver`](#visit-receiver)
  - [`visit_return_type`](#visit-return-type)
  - [`visit_signature`](#visit-signature)
  - [`visit_span`](#visit-span)
  - [`visit_static_mutability`](#visit-static-mutability)
  - [`visit_stmt`](#visit-stmt)
  - [`visit_stmt_macro`](#visit-stmt-macro)
  - [`visit_trait_bound`](#visit-trait-bound)
  - [`visit_trait_bound_modifier`](#visit-trait-bound-modifier)
  - [`visit_trait_item`](#visit-trait-item)
  - [`visit_trait_item_const`](#visit-trait-item-const)
  - [`visit_trait_item_fn`](#visit-trait-item-fn)
  - [`visit_trait_item_macro`](#visit-trait-item-macro)
  - [`visit_trait_item_type`](#visit-trait-item-type)
  - [`visit_type`](#visit-type)
  - [`visit_type_array`](#visit-type-array)
  - [`visit_type_bare_fn`](#visit-type-bare-fn)
  - [`visit_type_group`](#visit-type-group)
  - [`visit_type_impl_trait`](#visit-type-impl-trait)
  - [`visit_type_infer`](#visit-type-infer)
  - [`visit_type_macro`](#visit-type-macro)
  - [`visit_type_never`](#visit-type-never)
  - [`visit_type_param`](#visit-type-param)
  - [`visit_type_param_bound`](#visit-type-param-bound)
  - [`visit_type_paren`](#visit-type-paren)
  - [`visit_type_path`](#visit-type-path)
  - [`visit_type_ptr`](#visit-type-ptr)
  - [`visit_type_reference`](#visit-type-reference)
  - [`visit_type_slice`](#visit-type-slice)
  - [`visit_type_trait_object`](#visit-type-trait-object)
  - [`visit_type_tuple`](#visit-type-tuple)
  - [`visit_un_op`](#visit-un-op)
  - [`visit_use_glob`](#visit-use-glob)
  - [`visit_use_group`](#visit-use-group)
  - [`visit_use_name`](#visit-use-name)
  - [`visit_use_path`](#visit-use-path)
  - [`visit_use_rename`](#visit-use-rename)
  - [`visit_use_tree`](#visit-use-tree)
  - [`visit_variadic`](#visit-variadic)
  - [`visit_variant`](#visit-variant)
  - [`visit_vis_restricted`](#visit-vis-restricted)
  - [`visit_visibility`](#visit-visibility)
  - [`visit_where_clause`](#visit-where-clause)
  - [`visit_where_predicate`](#visit-where-predicate)
- [Macros](#macros)
  - [`full!`](#full)
  - [`skip!`](#skip)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Visit`](#visit) | trait | Syntax tree traversal to walk a shared borrow of a syntax tree. |
| [`visit_abi`](#visit-abi) | fn |  |
| [`visit_angle_bracketed_generic_arguments`](#visit-angle-bracketed-generic-arguments) | fn |  |
| [`visit_arm`](#visit-arm) | fn |  |
| [`visit_assoc_const`](#visit-assoc-const) | fn |  |
| [`visit_assoc_type`](#visit-assoc-type) | fn |  |
| [`visit_attr_style`](#visit-attr-style) | fn |  |
| [`visit_attribute`](#visit-attribute) | fn |  |
| [`visit_bare_fn_arg`](#visit-bare-fn-arg) | fn |  |
| [`visit_bare_variadic`](#visit-bare-variadic) | fn |  |
| [`visit_bin_op`](#visit-bin-op) | fn |  |
| [`visit_block`](#visit-block) | fn |  |
| [`visit_bound_lifetimes`](#visit-bound-lifetimes) | fn |  |
| [`visit_captured_param`](#visit-captured-param) | fn |  |
| [`visit_const_param`](#visit-const-param) | fn |  |
| [`visit_constraint`](#visit-constraint) | fn |  |
| [`visit_data`](#visit-data) | fn |  |
| [`visit_data_enum`](#visit-data-enum) | fn |  |
| [`visit_data_struct`](#visit-data-struct) | fn |  |
| [`visit_data_union`](#visit-data-union) | fn |  |
| [`visit_derive_input`](#visit-derive-input) | fn |  |
| [`visit_expr`](#visit-expr) | fn |  |
| [`visit_expr_array`](#visit-expr-array) | fn |  |
| [`visit_expr_assign`](#visit-expr-assign) | fn |  |
| [`visit_expr_async`](#visit-expr-async) | fn |  |
| [`visit_expr_await`](#visit-expr-await) | fn |  |
| [`visit_expr_binary`](#visit-expr-binary) | fn |  |
| [`visit_expr_block`](#visit-expr-block) | fn |  |
| [`visit_expr_break`](#visit-expr-break) | fn |  |
| [`visit_expr_call`](#visit-expr-call) | fn |  |
| [`visit_expr_cast`](#visit-expr-cast) | fn |  |
| [`visit_expr_closure`](#visit-expr-closure) | fn |  |
| [`visit_expr_const`](#visit-expr-const) | fn |  |
| [`visit_expr_continue`](#visit-expr-continue) | fn |  |
| [`visit_expr_field`](#visit-expr-field) | fn |  |
| [`visit_expr_for_loop`](#visit-expr-for-loop) | fn |  |
| [`visit_expr_group`](#visit-expr-group) | fn |  |
| [`visit_expr_if`](#visit-expr-if) | fn |  |
| [`visit_expr_index`](#visit-expr-index) | fn |  |
| [`visit_expr_infer`](#visit-expr-infer) | fn |  |
| [`visit_expr_let`](#visit-expr-let) | fn |  |
| [`visit_expr_lit`](#visit-expr-lit) | fn |  |
| [`visit_expr_loop`](#visit-expr-loop) | fn |  |
| [`visit_expr_macro`](#visit-expr-macro) | fn |  |
| [`visit_expr_match`](#visit-expr-match) | fn |  |
| [`visit_expr_method_call`](#visit-expr-method-call) | fn |  |
| [`visit_expr_paren`](#visit-expr-paren) | fn |  |
| [`visit_expr_path`](#visit-expr-path) | fn |  |
| [`visit_expr_range`](#visit-expr-range) | fn |  |
| [`visit_expr_raw_addr`](#visit-expr-raw-addr) | fn |  |
| [`visit_expr_reference`](#visit-expr-reference) | fn |  |
| [`visit_expr_repeat`](#visit-expr-repeat) | fn |  |
| [`visit_expr_return`](#visit-expr-return) | fn |  |
| [`visit_expr_struct`](#visit-expr-struct) | fn |  |
| [`visit_expr_try`](#visit-expr-try) | fn |  |
| [`visit_expr_try_block`](#visit-expr-try-block) | fn |  |
| [`visit_expr_tuple`](#visit-expr-tuple) | fn |  |
| [`visit_expr_unary`](#visit-expr-unary) | fn |  |
| [`visit_expr_unsafe`](#visit-expr-unsafe) | fn |  |
| [`visit_expr_while`](#visit-expr-while) | fn |  |
| [`visit_expr_yield`](#visit-expr-yield) | fn |  |
| [`visit_field`](#visit-field) | fn |  |
| [`visit_field_mutability`](#visit-field-mutability) | fn |  |
| [`visit_field_pat`](#visit-field-pat) | fn |  |
| [`visit_field_value`](#visit-field-value) | fn |  |
| [`visit_fields`](#visit-fields) | fn |  |
| [`visit_fields_named`](#visit-fields-named) | fn |  |
| [`visit_fields_unnamed`](#visit-fields-unnamed) | fn |  |
| [`visit_file`](#visit-file) | fn |  |
| [`visit_fn_arg`](#visit-fn-arg) | fn |  |
| [`visit_foreign_item`](#visit-foreign-item) | fn |  |
| [`visit_foreign_item_fn`](#visit-foreign-item-fn) | fn |  |
| [`visit_foreign_item_macro`](#visit-foreign-item-macro) | fn |  |
| [`visit_foreign_item_static`](#visit-foreign-item-static) | fn |  |
| [`visit_foreign_item_type`](#visit-foreign-item-type) | fn |  |
| [`visit_generic_argument`](#visit-generic-argument) | fn |  |
| [`visit_generic_param`](#visit-generic-param) | fn |  |
| [`visit_generics`](#visit-generics) | fn |  |
| [`visit_ident`](#visit-ident) | fn |  |
| [`visit_impl_item`](#visit-impl-item) | fn |  |
| [`visit_impl_item_const`](#visit-impl-item-const) | fn |  |
| [`visit_impl_item_fn`](#visit-impl-item-fn) | fn |  |
| [`visit_impl_item_macro`](#visit-impl-item-macro) | fn |  |
| [`visit_impl_item_type`](#visit-impl-item-type) | fn |  |
| [`visit_impl_restriction`](#visit-impl-restriction) | fn |  |
| [`visit_index`](#visit-index) | fn |  |
| [`visit_item`](#visit-item) | fn |  |
| [`visit_item_const`](#visit-item-const) | fn |  |
| [`visit_item_enum`](#visit-item-enum) | fn |  |
| [`visit_item_extern_crate`](#visit-item-extern-crate) | fn |  |
| [`visit_item_fn`](#visit-item-fn) | fn |  |
| [`visit_item_foreign_mod`](#visit-item-foreign-mod) | fn |  |
| [`visit_item_impl`](#visit-item-impl) | fn |  |
| [`visit_item_macro`](#visit-item-macro) | fn |  |
| [`visit_item_mod`](#visit-item-mod) | fn |  |
| [`visit_item_static`](#visit-item-static) | fn |  |
| [`visit_item_struct`](#visit-item-struct) | fn |  |
| [`visit_item_trait`](#visit-item-trait) | fn |  |
| [`visit_item_trait_alias`](#visit-item-trait-alias) | fn |  |
| [`visit_item_type`](#visit-item-type) | fn |  |
| [`visit_item_union`](#visit-item-union) | fn |  |
| [`visit_item_use`](#visit-item-use) | fn |  |
| [`visit_label`](#visit-label) | fn |  |
| [`visit_lifetime`](#visit-lifetime) | fn |  |
| [`visit_lifetime_param`](#visit-lifetime-param) | fn |  |
| [`visit_lit`](#visit-lit) | fn |  |
| [`visit_lit_bool`](#visit-lit-bool) | fn |  |
| [`visit_lit_byte`](#visit-lit-byte) | fn |  |
| [`visit_lit_byte_str`](#visit-lit-byte-str) | fn |  |
| [`visit_lit_cstr`](#visit-lit-cstr) | fn |  |
| [`visit_lit_char`](#visit-lit-char) | fn |  |
| [`visit_lit_float`](#visit-lit-float) | fn |  |
| [`visit_lit_int`](#visit-lit-int) | fn |  |
| [`visit_lit_str`](#visit-lit-str) | fn |  |
| [`visit_local`](#visit-local) | fn |  |
| [`visit_local_init`](#visit-local-init) | fn |  |
| [`visit_macro`](#visit-macro) | fn |  |
| [`visit_macro_delimiter`](#visit-macro-delimiter) | fn |  |
| [`visit_member`](#visit-member) | fn |  |
| [`visit_meta`](#visit-meta) | fn |  |
| [`visit_meta_list`](#visit-meta-list) | fn |  |
| [`visit_meta_name_value`](#visit-meta-name-value) | fn |  |
| [`visit_parenthesized_generic_arguments`](#visit-parenthesized-generic-arguments) | fn |  |
| [`visit_pat`](#visit-pat) | fn |  |
| [`visit_pat_ident`](#visit-pat-ident) | fn |  |
| [`visit_pat_or`](#visit-pat-or) | fn |  |
| [`visit_pat_paren`](#visit-pat-paren) | fn |  |
| [`visit_pat_reference`](#visit-pat-reference) | fn |  |
| [`visit_pat_rest`](#visit-pat-rest) | fn |  |
| [`visit_pat_slice`](#visit-pat-slice) | fn |  |
| [`visit_pat_struct`](#visit-pat-struct) | fn |  |
| [`visit_pat_tuple`](#visit-pat-tuple) | fn |  |
| [`visit_pat_tuple_struct`](#visit-pat-tuple-struct) | fn |  |
| [`visit_pat_type`](#visit-pat-type) | fn |  |
| [`visit_pat_wild`](#visit-pat-wild) | fn |  |
| [`visit_path`](#visit-path) | fn |  |
| [`visit_path_arguments`](#visit-path-arguments) | fn |  |
| [`visit_path_segment`](#visit-path-segment) | fn |  |
| [`visit_pointer_mutability`](#visit-pointer-mutability) | fn |  |
| [`visit_precise_capture`](#visit-precise-capture) | fn |  |
| [`visit_predicate_lifetime`](#visit-predicate-lifetime) | fn |  |
| [`visit_predicate_type`](#visit-predicate-type) | fn |  |
| [`visit_qself`](#visit-qself) | fn |  |
| [`visit_range_limits`](#visit-range-limits) | fn |  |
| [`visit_receiver`](#visit-receiver) | fn |  |
| [`visit_return_type`](#visit-return-type) | fn |  |
| [`visit_signature`](#visit-signature) | fn |  |
| [`visit_span`](#visit-span) | fn |  |
| [`visit_static_mutability`](#visit-static-mutability) | fn |  |
| [`visit_stmt`](#visit-stmt) | fn |  |
| [`visit_stmt_macro`](#visit-stmt-macro) | fn |  |
| [`visit_trait_bound`](#visit-trait-bound) | fn |  |
| [`visit_trait_bound_modifier`](#visit-trait-bound-modifier) | fn |  |
| [`visit_trait_item`](#visit-trait-item) | fn |  |
| [`visit_trait_item_const`](#visit-trait-item-const) | fn |  |
| [`visit_trait_item_fn`](#visit-trait-item-fn) | fn |  |
| [`visit_trait_item_macro`](#visit-trait-item-macro) | fn |  |
| [`visit_trait_item_type`](#visit-trait-item-type) | fn |  |
| [`visit_type`](#visit-type) | fn |  |
| [`visit_type_array`](#visit-type-array) | fn |  |
| [`visit_type_bare_fn`](#visit-type-bare-fn) | fn |  |
| [`visit_type_group`](#visit-type-group) | fn |  |
| [`visit_type_impl_trait`](#visit-type-impl-trait) | fn |  |
| [`visit_type_infer`](#visit-type-infer) | fn |  |
| [`visit_type_macro`](#visit-type-macro) | fn |  |
| [`visit_type_never`](#visit-type-never) | fn |  |
| [`visit_type_param`](#visit-type-param) | fn |  |
| [`visit_type_param_bound`](#visit-type-param-bound) | fn |  |
| [`visit_type_paren`](#visit-type-paren) | fn |  |
| [`visit_type_path`](#visit-type-path) | fn |  |
| [`visit_type_ptr`](#visit-type-ptr) | fn |  |
| [`visit_type_reference`](#visit-type-reference) | fn |  |
| [`visit_type_slice`](#visit-type-slice) | fn |  |
| [`visit_type_trait_object`](#visit-type-trait-object) | fn |  |
| [`visit_type_tuple`](#visit-type-tuple) | fn |  |
| [`visit_un_op`](#visit-un-op) | fn |  |
| [`visit_use_glob`](#visit-use-glob) | fn |  |
| [`visit_use_group`](#visit-use-group) | fn |  |
| [`visit_use_name`](#visit-use-name) | fn |  |
| [`visit_use_path`](#visit-use-path) | fn |  |
| [`visit_use_rename`](#visit-use-rename) | fn |  |
| [`visit_use_tree`](#visit-use-tree) | fn |  |
| [`visit_variadic`](#visit-variadic) | fn |  |
| [`visit_variant`](#visit-variant) | fn |  |
| [`visit_vis_restricted`](#visit-vis-restricted) | fn |  |
| [`visit_visibility`](#visit-visibility) | fn |  |
| [`visit_where_clause`](#visit-where-clause) | fn |  |
| [`visit_where_predicate`](#visit-where-predicate) | fn |  |
| [`full!`](#full) | macro |  |
| [`skip!`](#skip) | macro |  |

## Traits

### `Visit<'ast>`

```rust
trait Visit<'ast> { ... }
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:28-945`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L28-L945)*

Syntax tree traversal to walk a shared borrow of a syntax tree.

See the [module documentation] for details.


#### Provided Methods

- `fn visit_abi(&mut self, i: &'ast crate::Abi)`

- `fn visit_angle_bracketed_generic_arguments(&mut self, i: &'ast crate::AngleBracketedGenericArguments)`

- `fn visit_arm(&mut self, i: &'ast crate::Arm)`

- `fn visit_assoc_const(&mut self, i: &'ast crate::AssocConst)`

- `fn visit_assoc_type(&mut self, i: &'ast crate::AssocType)`

- `fn visit_attr_style(&mut self, i: &'ast crate::AttrStyle)`

- `fn visit_attribute(&mut self, i: &'ast crate::Attribute)`

- `fn visit_bare_fn_arg(&mut self, i: &'ast crate::BareFnArg)`

- `fn visit_bare_variadic(&mut self, i: &'ast crate::BareVariadic)`

- `fn visit_bin_op(&mut self, i: &'ast crate::BinOp)`

- `fn visit_block(&mut self, i: &'ast crate::Block)`

- `fn visit_bound_lifetimes(&mut self, i: &'ast crate::BoundLifetimes)`

- `fn visit_captured_param(&mut self, i: &'ast crate::CapturedParam)`

- `fn visit_const_param(&mut self, i: &'ast crate::ConstParam)`

- `fn visit_constraint(&mut self, i: &'ast crate::Constraint)`

- `fn visit_data(&mut self, i: &'ast crate::Data)`

- `fn visit_data_enum(&mut self, i: &'ast crate::DataEnum)`

- `fn visit_data_struct(&mut self, i: &'ast crate::DataStruct)`

- `fn visit_data_union(&mut self, i: &'ast crate::DataUnion)`

- `fn visit_derive_input(&mut self, i: &'ast crate::DeriveInput)`

- `fn visit_expr(&mut self, i: &'ast crate::Expr)`

- `fn visit_expr_array(&mut self, i: &'ast crate::ExprArray)`

- `fn visit_expr_assign(&mut self, i: &'ast crate::ExprAssign)`

- `fn visit_expr_async(&mut self, i: &'ast crate::ExprAsync)`

- `fn visit_expr_await(&mut self, i: &'ast crate::ExprAwait)`

- `fn visit_expr_binary(&mut self, i: &'ast crate::ExprBinary)`

- `fn visit_expr_block(&mut self, i: &'ast crate::ExprBlock)`

- `fn visit_expr_break(&mut self, i: &'ast crate::ExprBreak)`

- `fn visit_expr_call(&mut self, i: &'ast crate::ExprCall)`

- `fn visit_expr_cast(&mut self, i: &'ast crate::ExprCast)`

- `fn visit_expr_closure(&mut self, i: &'ast crate::ExprClosure)`

- `fn visit_expr_const(&mut self, i: &'ast crate::ExprConst)`

- `fn visit_expr_continue(&mut self, i: &'ast crate::ExprContinue)`

- `fn visit_expr_field(&mut self, i: &'ast crate::ExprField)`

- `fn visit_expr_for_loop(&mut self, i: &'ast crate::ExprForLoop)`

- `fn visit_expr_group(&mut self, i: &'ast crate::ExprGroup)`

- `fn visit_expr_if(&mut self, i: &'ast crate::ExprIf)`

- `fn visit_expr_index(&mut self, i: &'ast crate::ExprIndex)`

- `fn visit_expr_infer(&mut self, i: &'ast crate::ExprInfer)`

- `fn visit_expr_let(&mut self, i: &'ast crate::ExprLet)`

- `fn visit_expr_lit(&mut self, i: &'ast crate::ExprLit)`

- `fn visit_expr_loop(&mut self, i: &'ast crate::ExprLoop)`

- `fn visit_expr_macro(&mut self, i: &'ast crate::ExprMacro)`

- `fn visit_expr_match(&mut self, i: &'ast crate::ExprMatch)`

- `fn visit_expr_method_call(&mut self, i: &'ast crate::ExprMethodCall)`

- `fn visit_expr_paren(&mut self, i: &'ast crate::ExprParen)`

- `fn visit_expr_path(&mut self, i: &'ast crate::ExprPath)`

- `fn visit_expr_range(&mut self, i: &'ast crate::ExprRange)`

- `fn visit_expr_raw_addr(&mut self, i: &'ast crate::ExprRawAddr)`

- `fn visit_expr_reference(&mut self, i: &'ast crate::ExprReference)`

- `fn visit_expr_repeat(&mut self, i: &'ast crate::ExprRepeat)`

- `fn visit_expr_return(&mut self, i: &'ast crate::ExprReturn)`

- `fn visit_expr_struct(&mut self, i: &'ast crate::ExprStruct)`

- `fn visit_expr_try(&mut self, i: &'ast crate::ExprTry)`

- `fn visit_expr_try_block(&mut self, i: &'ast crate::ExprTryBlock)`

- `fn visit_expr_tuple(&mut self, i: &'ast crate::ExprTuple)`

- `fn visit_expr_unary(&mut self, i: &'ast crate::ExprUnary)`

- `fn visit_expr_unsafe(&mut self, i: &'ast crate::ExprUnsafe)`

- `fn visit_expr_while(&mut self, i: &'ast crate::ExprWhile)`

- `fn visit_expr_yield(&mut self, i: &'ast crate::ExprYield)`

- `fn visit_field(&mut self, i: &'ast crate::Field)`

- `fn visit_field_mutability(&mut self, i: &'ast crate::FieldMutability)`

- `fn visit_field_pat(&mut self, i: &'ast crate::FieldPat)`

- `fn visit_field_value(&mut self, i: &'ast crate::FieldValue)`

- `fn visit_fields(&mut self, i: &'ast crate::Fields)`

- `fn visit_fields_named(&mut self, i: &'ast crate::FieldsNamed)`

- `fn visit_fields_unnamed(&mut self, i: &'ast crate::FieldsUnnamed)`

- `fn visit_file(&mut self, i: &'ast crate::File)`

- `fn visit_fn_arg(&mut self, i: &'ast crate::FnArg)`

- `fn visit_foreign_item(&mut self, i: &'ast crate::ForeignItem)`

- `fn visit_foreign_item_fn(&mut self, i: &'ast crate::ForeignItemFn)`

- `fn visit_foreign_item_macro(&mut self, i: &'ast crate::ForeignItemMacro)`

- `fn visit_foreign_item_static(&mut self, i: &'ast crate::ForeignItemStatic)`

- `fn visit_foreign_item_type(&mut self, i: &'ast crate::ForeignItemType)`

- `fn visit_generic_argument(&mut self, i: &'ast crate::GenericArgument)`

- `fn visit_generic_param(&mut self, i: &'ast crate::GenericParam)`

- `fn visit_generics(&mut self, i: &'ast crate::Generics)`

- `fn visit_ident(&mut self, i: &'ast proc_macro2::Ident)`

- `fn visit_impl_item(&mut self, i: &'ast crate::ImplItem)`

- `fn visit_impl_item_const(&mut self, i: &'ast crate::ImplItemConst)`

- `fn visit_impl_item_fn(&mut self, i: &'ast crate::ImplItemFn)`

- `fn visit_impl_item_macro(&mut self, i: &'ast crate::ImplItemMacro)`

- `fn visit_impl_item_type(&mut self, i: &'ast crate::ImplItemType)`

- `fn visit_impl_restriction(&mut self, i: &'ast crate::ImplRestriction)`

- `fn visit_index(&mut self, i: &'ast crate::Index)`

- `fn visit_item(&mut self, i: &'ast crate::Item)`

- `fn visit_item_const(&mut self, i: &'ast crate::ItemConst)`

- `fn visit_item_enum(&mut self, i: &'ast crate::ItemEnum)`

- `fn visit_item_extern_crate(&mut self, i: &'ast crate::ItemExternCrate)`

- `fn visit_item_fn(&mut self, i: &'ast crate::ItemFn)`

- `fn visit_item_foreign_mod(&mut self, i: &'ast crate::ItemForeignMod)`

- `fn visit_item_impl(&mut self, i: &'ast crate::ItemImpl)`

- `fn visit_item_macro(&mut self, i: &'ast crate::ItemMacro)`

- `fn visit_item_mod(&mut self, i: &'ast crate::ItemMod)`

- `fn visit_item_static(&mut self, i: &'ast crate::ItemStatic)`

- `fn visit_item_struct(&mut self, i: &'ast crate::ItemStruct)`

- `fn visit_item_trait(&mut self, i: &'ast crate::ItemTrait)`

- `fn visit_item_trait_alias(&mut self, i: &'ast crate::ItemTraitAlias)`

- `fn visit_item_type(&mut self, i: &'ast crate::ItemType)`

- `fn visit_item_union(&mut self, i: &'ast crate::ItemUnion)`

- `fn visit_item_use(&mut self, i: &'ast crate::ItemUse)`

- `fn visit_label(&mut self, i: &'ast crate::Label)`

- `fn visit_lifetime(&mut self, i: &'ast crate::Lifetime)`

- `fn visit_lifetime_param(&mut self, i: &'ast crate::LifetimeParam)`

- `fn visit_lit(&mut self, i: &'ast crate::Lit)`

- `fn visit_lit_bool(&mut self, i: &'ast crate::LitBool)`

- `fn visit_lit_byte(&mut self, i: &'ast crate::LitByte)`

- `fn visit_lit_byte_str(&mut self, i: &'ast crate::LitByteStr)`

- `fn visit_lit_cstr(&mut self, i: &'ast crate::LitCStr)`

- `fn visit_lit_char(&mut self, i: &'ast crate::LitChar)`

- `fn visit_lit_float(&mut self, i: &'ast crate::LitFloat)`

- `fn visit_lit_int(&mut self, i: &'ast crate::LitInt)`

- `fn visit_lit_str(&mut self, i: &'ast crate::LitStr)`

- `fn visit_local(&mut self, i: &'ast crate::Local)`

- `fn visit_local_init(&mut self, i: &'ast crate::LocalInit)`

- `fn visit_macro(&mut self, i: &'ast crate::Macro)`

- `fn visit_macro_delimiter(&mut self, i: &'ast crate::MacroDelimiter)`

- `fn visit_member(&mut self, i: &'ast crate::Member)`

- `fn visit_meta(&mut self, i: &'ast crate::Meta)`

- `fn visit_meta_list(&mut self, i: &'ast crate::MetaList)`

- `fn visit_meta_name_value(&mut self, i: &'ast crate::MetaNameValue)`

- `fn visit_parenthesized_generic_arguments(&mut self, i: &'ast crate::ParenthesizedGenericArguments)`

- `fn visit_pat(&mut self, i: &'ast crate::Pat)`

- `fn visit_pat_ident(&mut self, i: &'ast crate::PatIdent)`

- `fn visit_pat_or(&mut self, i: &'ast crate::PatOr)`

- `fn visit_pat_paren(&mut self, i: &'ast crate::PatParen)`

- `fn visit_pat_reference(&mut self, i: &'ast crate::PatReference)`

- `fn visit_pat_rest(&mut self, i: &'ast crate::PatRest)`

- `fn visit_pat_slice(&mut self, i: &'ast crate::PatSlice)`

- `fn visit_pat_struct(&mut self, i: &'ast crate::PatStruct)`

- `fn visit_pat_tuple(&mut self, i: &'ast crate::PatTuple)`

- `fn visit_pat_tuple_struct(&mut self, i: &'ast crate::PatTupleStruct)`

- `fn visit_pat_type(&mut self, i: &'ast crate::PatType)`

- `fn visit_pat_wild(&mut self, i: &'ast crate::PatWild)`

- `fn visit_path(&mut self, i: &'ast crate::Path)`

- `fn visit_path_arguments(&mut self, i: &'ast crate::PathArguments)`

- `fn visit_path_segment(&mut self, i: &'ast crate::PathSegment)`

- `fn visit_pointer_mutability(&mut self, i: &'ast crate::PointerMutability)`

- `fn visit_precise_capture(&mut self, i: &'ast crate::PreciseCapture)`

- `fn visit_predicate_lifetime(&mut self, i: &'ast crate::PredicateLifetime)`

- `fn visit_predicate_type(&mut self, i: &'ast crate::PredicateType)`

- `fn visit_qself(&mut self, i: &'ast crate::QSelf)`

- `fn visit_range_limits(&mut self, i: &'ast crate::RangeLimits)`

- `fn visit_receiver(&mut self, i: &'ast crate::Receiver)`

- `fn visit_return_type(&mut self, i: &'ast crate::ReturnType)`

- `fn visit_signature(&mut self, i: &'ast crate::Signature)`

- `fn visit_span(&mut self, i: &proc_macro2::Span)`

- `fn visit_static_mutability(&mut self, i: &'ast crate::StaticMutability)`

- `fn visit_stmt(&mut self, i: &'ast crate::Stmt)`

- `fn visit_stmt_macro(&mut self, i: &'ast crate::StmtMacro)`

- `fn visit_token_stream(&mut self, i: &'ast proc_macro2::TokenStream)`

- `fn visit_trait_bound(&mut self, i: &'ast crate::TraitBound)`

- `fn visit_trait_bound_modifier(&mut self, i: &'ast crate::TraitBoundModifier)`

- `fn visit_trait_item(&mut self, i: &'ast crate::TraitItem)`

- `fn visit_trait_item_const(&mut self, i: &'ast crate::TraitItemConst)`

- `fn visit_trait_item_fn(&mut self, i: &'ast crate::TraitItemFn)`

- `fn visit_trait_item_macro(&mut self, i: &'ast crate::TraitItemMacro)`

- `fn visit_trait_item_type(&mut self, i: &'ast crate::TraitItemType)`

- `fn visit_type(&mut self, i: &'ast crate::Type)`

- `fn visit_type_array(&mut self, i: &'ast crate::TypeArray)`

- `fn visit_type_bare_fn(&mut self, i: &'ast crate::TypeBareFn)`

- `fn visit_type_group(&mut self, i: &'ast crate::TypeGroup)`

- `fn visit_type_impl_trait(&mut self, i: &'ast crate::TypeImplTrait)`

- `fn visit_type_infer(&mut self, i: &'ast crate::TypeInfer)`

- `fn visit_type_macro(&mut self, i: &'ast crate::TypeMacro)`

- `fn visit_type_never(&mut self, i: &'ast crate::TypeNever)`

- `fn visit_type_param(&mut self, i: &'ast crate::TypeParam)`

- `fn visit_type_param_bound(&mut self, i: &'ast crate::TypeParamBound)`

- `fn visit_type_paren(&mut self, i: &'ast crate::TypeParen)`

- `fn visit_type_path(&mut self, i: &'ast crate::TypePath)`

- `fn visit_type_ptr(&mut self, i: &'ast crate::TypePtr)`

- `fn visit_type_reference(&mut self, i: &'ast crate::TypeReference)`

- `fn visit_type_slice(&mut self, i: &'ast crate::TypeSlice)`

- `fn visit_type_trait_object(&mut self, i: &'ast crate::TypeTraitObject)`

- `fn visit_type_tuple(&mut self, i: &'ast crate::TypeTuple)`

- `fn visit_un_op(&mut self, i: &'ast crate::UnOp)`

- `fn visit_use_glob(&mut self, i: &'ast crate::UseGlob)`

- `fn visit_use_group(&mut self, i: &'ast crate::UseGroup)`

- `fn visit_use_name(&mut self, i: &'ast crate::UseName)`

- `fn visit_use_path(&mut self, i: &'ast crate::UsePath)`

- `fn visit_use_rename(&mut self, i: &'ast crate::UseRename)`

- `fn visit_use_tree(&mut self, i: &'ast crate::UseTree)`

- `fn visit_variadic(&mut self, i: &'ast crate::Variadic)`

- `fn visit_variant(&mut self, i: &'ast crate::Variant)`

- `fn visit_vis_restricted(&mut self, i: &'ast crate::VisRestricted)`

- `fn visit_visibility(&mut self, i: &'ast crate::Visibility)`

- `fn visit_where_clause(&mut self, i: &'ast crate::WhereClause)`

- `fn visit_where_predicate(&mut self, i: &'ast crate::WherePredicate)`

## Functions

### `visit_abi`

```rust
fn visit_abi<'ast, V>(v: &mut V, node: &'ast crate::Abi)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:948-956`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L948-L956)*

### `visit_angle_bracketed_generic_arguments`

```rust
fn visit_angle_bracketed_generic_arguments<'ast, V>(v: &mut V, node: &'ast crate::AngleBracketedGenericArguments)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:959-973`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L959-L973)*

### `visit_arm`

```rust
fn visit_arm<'ast, V>(v: &mut V, node: &'ast crate::Arm)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:976-991`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L976-L991)*

### `visit_assoc_const`

```rust
fn visit_assoc_const<'ast, V>(v: &mut V, node: &'ast crate::AssocConst)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:994-1004`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L994-L1004)*

### `visit_assoc_type`

```rust
fn visit_assoc_type<'ast, V>(v: &mut V, node: &'ast crate::AssocType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1007-1017`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1007-L1017)*

### `visit_attr_style`

```rust
fn visit_attr_style<'ast, V>(v: &mut V, node: &'ast crate::AttrStyle)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1020-1030`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1020-L1030)*

### `visit_attribute`

```rust
fn visit_attribute<'ast, V>(v: &mut V, node: &'ast crate::Attribute)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1033-1041`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1033-L1041)*

### `visit_bare_fn_arg`

```rust
fn visit_bare_fn_arg<'ast, V>(v: &mut V, node: &'ast crate::BareFnArg)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1044-1056`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1044-L1056)*

### `visit_bare_variadic`

```rust
fn visit_bare_variadic<'ast, V>(v: &mut V, node: &'ast crate::BareVariadic)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1059-1072`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1059-L1072)*

### `visit_bin_op`

```rust
fn visit_bin_op<'ast, V>(v: &mut V, node: &'ast crate::BinOp)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1075-1165`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1075-L1165)*

### `visit_block`

```rust
fn visit_block<'ast, V>(v: &mut V, node: &'ast crate::Block)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1168-1176`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1168-L1176)*

### `visit_bound_lifetimes`

```rust
fn visit_bound_lifetimes<'ast, V>(v: &mut V, node: &'ast crate::BoundLifetimes)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1179-1190`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1179-L1190)*

### `visit_captured_param`

```rust
fn visit_captured_param<'ast, V>(v: &mut V, node: &'ast crate::CapturedParam)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1193-1205`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1193-L1205)*

### `visit_const_param`

```rust
fn visit_const_param<'ast, V>(v: &mut V, node: &'ast crate::ConstParam)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1208-1223`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1208-L1223)*

### `visit_constraint`

```rust
fn visit_constraint<'ast, V>(v: &mut V, node: &'ast crate::Constraint)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1226-1239`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1226-L1239)*

### `visit_data`

```rust
fn visit_data<'ast, V>(v: &mut V, node: &'ast crate::Data)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1242-1257`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1242-L1257)*

### `visit_data_enum`

```rust
fn visit_data_enum<'ast, V>(v: &mut V, node: &'ast crate::DataEnum)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1260-1270`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1260-L1270)*

### `visit_data_struct`

```rust
fn visit_data_struct<'ast, V>(v: &mut V, node: &'ast crate::DataStruct)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1273-1280`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1273-L1280)*

### `visit_data_union`

```rust
fn visit_data_union<'ast, V>(v: &mut V, node: &'ast crate::DataUnion)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1283-1289`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1283-L1289)*

### `visit_derive_input`

```rust
fn visit_derive_input<'ast, V>(v: &mut V, node: &'ast crate::DeriveInput)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1292-1303`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1292-L1303)*

### `visit_expr`

```rust
fn visit_expr<'ast, V>(v: &mut V, node: &'ast crate::Expr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1306-1432`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1306-L1432)*

### `visit_expr_array`

```rust
fn visit_expr_array<'ast, V>(v: &mut V, node: &'ast crate::ExprArray)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1435-1447`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1435-L1447)*

### `visit_expr_assign`

```rust
fn visit_expr_assign<'ast, V>(v: &mut V, node: &'ast crate::ExprAssign)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1450-1460`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1450-L1460)*

### `visit_expr_async`

```rust
fn visit_expr_async<'ast, V>(v: &mut V, node: &'ast crate::ExprAsync)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1463-1473`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1463-L1473)*

### `visit_expr_await`

```rust
fn visit_expr_await<'ast, V>(v: &mut V, node: &'ast crate::ExprAwait)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1476-1486`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1476-L1486)*

### `visit_expr_binary`

```rust
fn visit_expr_binary<'ast, V>(v: &mut V, node: &'ast crate::ExprBinary)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1489-1499`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1489-L1499)*

### `visit_expr_block`

```rust
fn visit_expr_block<'ast, V>(v: &mut V, node: &'ast crate::ExprBlock)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1502-1513`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1502-L1513)*

### `visit_expr_break`

```rust
fn visit_expr_break<'ast, V>(v: &mut V, node: &'ast crate::ExprBreak)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1516-1530`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1516-L1530)*

### `visit_expr_call`

```rust
fn visit_expr_call<'ast, V>(v: &mut V, node: &'ast crate::ExprCall)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1533-1546`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1533-L1546)*

### `visit_expr_cast`

```rust
fn visit_expr_cast<'ast, V>(v: &mut V, node: &'ast crate::ExprCast)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1549-1559`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1549-L1559)*

### `visit_expr_closure`

```rust
fn visit_expr_closure<'ast, V>(v: &mut V, node: &'ast crate::ExprClosure)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1562-1584`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1562-L1584)*

### `visit_expr_const`

```rust
fn visit_expr_const<'ast, V>(v: &mut V, node: &'ast crate::ExprConst)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1587-1596`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1587-L1596)*

### `visit_expr_continue`

```rust
fn visit_expr_continue<'ast, V>(v: &mut V, node: &'ast crate::ExprContinue)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1599-1610`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1599-L1610)*

### `visit_expr_field`

```rust
fn visit_expr_field<'ast, V>(v: &mut V, node: &'ast crate::ExprField)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1613-1623`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1613-L1623)*

### `visit_expr_for_loop`

```rust
fn visit_expr_for_loop<'ast, V>(v: &mut V, node: &'ast crate::ExprForLoop)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1626-1641`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1626-L1641)*

### `visit_expr_group`

```rust
fn visit_expr_group<'ast, V>(v: &mut V, node: &'ast crate::ExprGroup)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1644-1653`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1644-L1653)*

### `visit_expr_if`

```rust
fn visit_expr_if<'ast, V>(v: &mut V, node: &'ast crate::ExprIf)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1656-1670`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1656-L1670)*

### `visit_expr_index`

```rust
fn visit_expr_index<'ast, V>(v: &mut V, node: &'ast crate::ExprIndex)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1673-1683`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1673-L1683)*

### `visit_expr_infer`

```rust
fn visit_expr_infer<'ast, V>(v: &mut V, node: &'ast crate::ExprInfer)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1686-1694`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1686-L1694)*

### `visit_expr_let`

```rust
fn visit_expr_let<'ast, V>(v: &mut V, node: &'ast crate::ExprLet)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1697-1708`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1697-L1708)*

### `visit_expr_lit`

```rust
fn visit_expr_lit<'ast, V>(v: &mut V, node: &'ast crate::ExprLit)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1711-1719`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1711-L1719)*

### `visit_expr_loop`

```rust
fn visit_expr_loop<'ast, V>(v: &mut V, node: &'ast crate::ExprLoop)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1722-1734`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1722-L1734)*

### `visit_expr_macro`

```rust
fn visit_expr_macro<'ast, V>(v: &mut V, node: &'ast crate::ExprMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1737-1745`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1737-L1745)*

### `visit_expr_match`

```rust
fn visit_expr_match<'ast, V>(v: &mut V, node: &'ast crate::ExprMatch)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1748-1761`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1748-L1761)*

### `visit_expr_method_call`

```rust
fn visit_expr_method_call<'ast, V>(v: &mut V, node: &'ast crate::ExprMethodCall)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1764-1782`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1764-L1782)*

### `visit_expr_paren`

```rust
fn visit_expr_paren<'ast, V>(v: &mut V, node: &'ast crate::ExprParen)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1785-1794`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1785-L1794)*

### `visit_expr_path`

```rust
fn visit_expr_path<'ast, V>(v: &mut V, node: &'ast crate::ExprPath)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1797-1808`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1797-L1808)*

### `visit_expr_range`

```rust
fn visit_expr_range<'ast, V>(v: &mut V, node: &'ast crate::ExprRange)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1811-1825`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1811-L1825)*

### `visit_expr_raw_addr`

```rust
fn visit_expr_raw_addr<'ast, V>(v: &mut V, node: &'ast crate::ExprRawAddr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1828-1839`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1828-L1839)*

### `visit_expr_reference`

```rust
fn visit_expr_reference<'ast, V>(v: &mut V, node: &'ast crate::ExprReference)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1842-1852`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1842-L1852)*

### `visit_expr_repeat`

```rust
fn visit_expr_repeat<'ast, V>(v: &mut V, node: &'ast crate::ExprRepeat)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1855-1866`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1855-L1866)*

### `visit_expr_return`

```rust
fn visit_expr_return<'ast, V>(v: &mut V, node: &'ast crate::ExprReturn)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1869-1880`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1869-L1880)*

### `visit_expr_struct`

```rust
fn visit_expr_struct<'ast, V>(v: &mut V, node: &'ast crate::ExprStruct)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1883-1903`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1883-L1903)*

### `visit_expr_try`

```rust
fn visit_expr_try<'ast, V>(v: &mut V, node: &'ast crate::ExprTry)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1906-1915`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1906-L1915)*

### `visit_expr_try_block`

```rust
fn visit_expr_try_block<'ast, V>(v: &mut V, node: &'ast crate::ExprTryBlock)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1918-1927`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1918-L1927)*

### `visit_expr_tuple`

```rust
fn visit_expr_tuple<'ast, V>(v: &mut V, node: &'ast crate::ExprTuple)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1930-1942`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1930-L1942)*

### `visit_expr_unary`

```rust
fn visit_expr_unary<'ast, V>(v: &mut V, node: &'ast crate::ExprUnary)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1945-1954`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1945-L1954)*

### `visit_expr_unsafe`

```rust
fn visit_expr_unsafe<'ast, V>(v: &mut V, node: &'ast crate::ExprUnsafe)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1957-1966`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1957-L1966)*

### `visit_expr_while`

```rust
fn visit_expr_while<'ast, V>(v: &mut V, node: &'ast crate::ExprWhile)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1969-1982`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1969-L1982)*

### `visit_expr_yield`

```rust
fn visit_expr_yield<'ast, V>(v: &mut V, node: &'ast crate::ExprYield)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1985-1996`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1985-L1996)*

### `visit_field`

```rust
fn visit_field<'ast, V>(v: &mut V, node: &'ast crate::Field)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:1999-2013`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L1999-L2013)*

### `visit_field_mutability`

```rust
fn visit_field_mutability<'ast, V>(v: &mut V, node: &'ast crate::FieldMutability)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2016-2023`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2016-L2023)*

### `visit_field_pat`

```rust
fn visit_field_pat<'ast, V>(v: &mut V, node: &'ast crate::FieldPat)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2026-2036`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2026-L2036)*

### `visit_field_value`

```rust
fn visit_field_value<'ast, V>(v: &mut V, node: &'ast crate::FieldValue)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2039-2049`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2039-L2049)*

### `visit_fields`

```rust
fn visit_fields<'ast, V>(v: &mut V, node: &'ast crate::Fields)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2052-2065`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2052-L2065)*

### `visit_fields_named`

```rust
fn visit_fields_named<'ast, V>(v: &mut V, node: &'ast crate::FieldsNamed)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2068-2077`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2068-L2077)*

### `visit_fields_unnamed`

```rust
fn visit_fields_unnamed<'ast, V>(v: &mut V, node: &'ast crate::FieldsUnnamed)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2080-2089`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2080-L2089)*

### `visit_file`

```rust
fn visit_file<'ast, V>(v: &mut V, node: &'ast crate::File)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2092-2103`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2092-L2103)*

### `visit_fn_arg`

```rust
fn visit_fn_arg<'ast, V>(v: &mut V, node: &'ast crate::FnArg)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2106-2118`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2106-L2118)*

### `visit_foreign_item`

```rust
fn visit_foreign_item<'ast, V>(v: &mut V, node: &'ast crate::ForeignItem)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2121-2142`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2121-L2142)*

### `visit_foreign_item_fn`

```rust
fn visit_foreign_item_fn<'ast, V>(v: &mut V, node: &'ast crate::ForeignItemFn)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2145-2155`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2145-L2155)*

### `visit_foreign_item_macro`

```rust
fn visit_foreign_item_macro<'ast, V>(v: &mut V, node: &'ast crate::ForeignItemMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2158-2167`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2158-L2167)*

### `visit_foreign_item_static`

```rust
fn visit_foreign_item_static<'ast, V>(v: &mut V, node: &'ast crate::ForeignItemStatic)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2170-2187`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2170-L2187)*

### `visit_foreign_item_type`

```rust
fn visit_foreign_item_type<'ast, V>(v: &mut V, node: &'ast crate::ForeignItemType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2190-2202`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2190-L2202)*

### `visit_generic_argument`

```rust
fn visit_generic_argument<'ast, V>(v: &mut V, node: &'ast crate::GenericArgument)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2205-2229`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2205-L2229)*

### `visit_generic_param`

```rust
fn visit_generic_param<'ast, V>(v: &mut V, node: &'ast crate::GenericParam)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2232-2247`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2232-L2247)*

### `visit_generics`

```rust
fn visit_generics<'ast, V>(v: &mut V, node: &'ast crate::Generics)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2250-2263`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2250-L2263)*

### `visit_ident`

```rust
fn visit_ident<'ast, V>(v: &mut V, node: &'ast proc_macro2::Ident)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2264-2269`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2264-L2269)*

### `visit_impl_item`

```rust
fn visit_impl_item<'ast, V>(v: &mut V, node: &'ast crate::ImplItem)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2272-2293`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2272-L2293)*

### `visit_impl_item_const`

```rust
fn visit_impl_item_const<'ast, V>(v: &mut V, node: &'ast crate::ImplItemConst)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2296-2313`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2296-L2313)*

### `visit_impl_item_fn`

```rust
fn visit_impl_item_fn<'ast, V>(v: &mut V, node: &'ast crate::ImplItemFn)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2316-2327`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2316-L2327)*

### `visit_impl_item_macro`

```rust
fn visit_impl_item_macro<'ast, V>(v: &mut V, node: &'ast crate::ImplItemMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2330-2339`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2330-L2339)*

### `visit_impl_item_type`

```rust
fn visit_impl_item_type<'ast, V>(v: &mut V, node: &'ast crate::ImplItemType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2342-2357`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2342-L2357)*

### `visit_impl_restriction`

```rust
fn visit_impl_restriction<'ast, V>(v: &mut V, node: &'ast crate::ImplRestriction)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2360-2365`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2360-L2365)*

### `visit_index`

```rust
fn visit_index<'ast, V>(v: &mut V, node: &'ast crate::Index)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2368-2374`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2368-L2374)*

### `visit_item`

```rust
fn visit_item<'ast, V>(v: &mut V, node: &'ast crate::Item)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2377-2431`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2377-L2431)*

### `visit_item_const`

```rust
fn visit_item_const<'ast, V>(v: &mut V, node: &'ast crate::ItemConst)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2434-2450`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2434-L2450)*

### `visit_item_enum`

```rust
fn visit_item_enum<'ast, V>(v: &mut V, node: &'ast crate::ItemEnum)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2453-2469`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2453-L2469)*

### `visit_item_extern_crate`

```rust
fn visit_item_extern_crate<'ast, V>(v: &mut V, node: &'ast crate::ItemExternCrate)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2472-2488`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2472-L2488)*

### `visit_item_fn`

```rust
fn visit_item_fn<'ast, V>(v: &mut V, node: &'ast crate::ItemFn)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2491-2501`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2491-L2501)*

### `visit_item_foreign_mod`

```rust
fn visit_item_foreign_mod<'ast, V>(v: &mut V, node: &'ast crate::ItemForeignMod)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2504-2517`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2504-L2517)*

### `visit_item_impl`

```rust
fn visit_item_impl<'ast, V>(v: &mut V, node: &'ast crate::ItemImpl)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2520-2541`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2520-L2541)*

### `visit_item_macro`

```rust
fn visit_item_macro<'ast, V>(v: &mut V, node: &'ast crate::ItemMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2544-2556`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2544-L2556)*

### `visit_item_mod`

```rust
fn visit_item_mod<'ast, V>(v: &mut V, node: &'ast crate::ItemMod)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2559-2577`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2559-L2577)*

### `visit_item_static`

```rust
fn visit_item_static<'ast, V>(v: &mut V, node: &'ast crate::ItemStatic)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2580-2596`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2580-L2596)*

### `visit_item_struct`

```rust
fn visit_item_struct<'ast, V>(v: &mut V, node: &'ast crate::ItemStruct)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2599-2612`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2599-L2612)*

### `visit_item_trait`

```rust
fn visit_item_trait<'ast, V>(v: &mut V, node: &'ast crate::ItemTrait)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2615-2640`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2615-L2640)*

### `visit_item_trait_alias`

```rust
fn visit_item_trait_alias<'ast, V>(v: &mut V, node: &'ast crate::ItemTraitAlias)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2643-2660`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2643-L2660)*

### `visit_item_type`

```rust
fn visit_item_type<'ast, V>(v: &mut V, node: &'ast crate::ItemType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2663-2677`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2663-L2677)*

### `visit_item_union`

```rust
fn visit_item_union<'ast, V>(v: &mut V, node: &'ast crate::ItemUnion)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2680-2692`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2680-L2692)*

### `visit_item_use`

```rust
fn visit_item_use<'ast, V>(v: &mut V, node: &'ast crate::ItemUse)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2695-2707`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2695-L2707)*

### `visit_label`

```rust
fn visit_label<'ast, V>(v: &mut V, node: &'ast crate::Label)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2710-2716`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2710-L2716)*

### `visit_lifetime`

```rust
fn visit_lifetime<'ast, V>(v: &mut V, node: &'ast crate::Lifetime)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2717-2723`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2717-L2723)*

### `visit_lifetime_param`

```rust
fn visit_lifetime_param<'ast, V>(v: &mut V, node: &'ast crate::LifetimeParam)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2726-2739`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2726-L2739)*

### `visit_lit`

```rust
fn visit_lit<'ast, V>(v: &mut V, node: &'ast crate::Lit)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2740-2773`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2740-L2773)*

### `visit_lit_bool`

```rust
fn visit_lit_bool<'ast, V>(v: &mut V, node: &'ast crate::LitBool)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2774-2780`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2774-L2780)*

### `visit_lit_byte`

```rust
fn visit_lit_byte<'ast, V>(v: &mut V, node: &'ast crate::LitByte)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2781-2784`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2781-L2784)*

### `visit_lit_byte_str`

```rust
fn visit_lit_byte_str<'ast, V>(v: &mut V, node: &'ast crate::LitByteStr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2785-2788`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2785-L2788)*

### `visit_lit_cstr`

```rust
fn visit_lit_cstr<'ast, V>(v: &mut V, node: &'ast crate::LitCStr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2789-2792`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2789-L2792)*

### `visit_lit_char`

```rust
fn visit_lit_char<'ast, V>(v: &mut V, node: &'ast crate::LitChar)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2793-2796`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2793-L2796)*

### `visit_lit_float`

```rust
fn visit_lit_float<'ast, V>(v: &mut V, node: &'ast crate::LitFloat)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2797-2800`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2797-L2800)*

### `visit_lit_int`

```rust
fn visit_lit_int<'ast, V>(v: &mut V, node: &'ast crate::LitInt)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2801-2804`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2801-L2804)*

### `visit_lit_str`

```rust
fn visit_lit_str<'ast, V>(v: &mut V, node: &'ast crate::LitStr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2805-2808`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2805-L2808)*

### `visit_local`

```rust
fn visit_local<'ast, V>(v: &mut V, node: &'ast crate::Local)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2811-2824`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2811-L2824)*

### `visit_local_init`

```rust
fn visit_local_init<'ast, V>(v: &mut V, node: &'ast crate::LocalInit)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2827-2837`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2827-L2837)*

### `visit_macro`

```rust
fn visit_macro<'ast, V>(v: &mut V, node: &'ast crate::Macro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2840-2848`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2840-L2848)*

### `visit_macro_delimiter`

```rust
fn visit_macro_delimiter<'ast, V>(v: &mut V, node: &'ast crate::MacroDelimiter)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2851-2866`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2851-L2866)*

### `visit_member`

```rust
fn visit_member<'ast, V>(v: &mut V, node: &'ast crate::Member)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2869-2881`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2869-L2881)*

### `visit_meta`

```rust
fn visit_meta<'ast, V>(v: &mut V, node: &'ast crate::Meta)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2884-2899`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2884-L2899)*

### `visit_meta_list`

```rust
fn visit_meta_list<'ast, V>(v: &mut V, node: &'ast crate::MetaList)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2902-2909`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2902-L2909)*

### `visit_meta_name_value`

```rust
fn visit_meta_name_value<'ast, V>(v: &mut V, node: &'ast crate::MetaNameValue)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2912-2919`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2912-L2919)*

### `visit_parenthesized_generic_arguments`

```rust
fn visit_parenthesized_generic_arguments<'ast, V>(v: &mut V, node: &'ast crate::ParenthesizedGenericArguments)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2922-2935`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2922-L2935)*

### `visit_pat`

```rust
fn visit_pat<'ast, V>(v: &mut V, node: &'ast crate::Pat)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2938-2995`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2938-L2995)*

### `visit_pat_ident`

```rust
fn visit_pat_ident<'ast, V>(v: &mut V, node: &'ast crate::PatIdent)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:2998-3012`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L2998-L3012)*

### `visit_pat_or`

```rust
fn visit_pat_or<'ast, V>(v: &mut V, node: &'ast crate::PatOr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3015-3027`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3015-L3027)*

### `visit_pat_paren`

```rust
fn visit_pat_paren<'ast, V>(v: &mut V, node: &'ast crate::PatParen)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3030-3039`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3030-L3039)*

### `visit_pat_reference`

```rust
fn visit_pat_reference<'ast, V>(v: &mut V, node: &'ast crate::PatReference)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3042-3052`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3042-L3052)*

### `visit_pat_rest`

```rust
fn visit_pat_rest<'ast, V>(v: &mut V, node: &'ast crate::PatRest)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3055-3063`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3055-L3063)*

### `visit_pat_slice`

```rust
fn visit_pat_slice<'ast, V>(v: &mut V, node: &'ast crate::PatSlice)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3066-3078`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3066-L3078)*

### `visit_pat_struct`

```rust
fn visit_pat_struct<'ast, V>(v: &mut V, node: &'ast crate::PatStruct)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3081-3100`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3081-L3100)*

### `visit_pat_tuple`

```rust
fn visit_pat_tuple<'ast, V>(v: &mut V, node: &'ast crate::PatTuple)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3103-3115`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3103-L3115)*

### `visit_pat_tuple_struct`

```rust
fn visit_pat_tuple_struct<'ast, V>(v: &mut V, node: &'ast crate::PatTupleStruct)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3118-3134`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3118-L3134)*

### `visit_pat_type`

```rust
fn visit_pat_type<'ast, V>(v: &mut V, node: &'ast crate::PatType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3137-3147`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3137-L3147)*

### `visit_pat_wild`

```rust
fn visit_pat_wild<'ast, V>(v: &mut V, node: &'ast crate::PatWild)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3150-3158`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3150-L3158)*

### `visit_path`

```rust
fn visit_path<'ast, V>(v: &mut V, node: &'ast crate::Path)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3161-3170`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3161-L3170)*

### `visit_path_arguments`

```rust
fn visit_path_arguments<'ast, V>(v: &mut V, node: &'ast crate::PathArguments)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3173-3186`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3173-L3186)*

### `visit_path_segment`

```rust
fn visit_path_segment<'ast, V>(v: &mut V, node: &'ast crate::PathSegment)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3189-3195`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3189-L3195)*

### `visit_pointer_mutability`

```rust
fn visit_pointer_mutability<'ast, V>(v: &mut V, node: &'ast crate::PointerMutability)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3198-3210`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3198-L3210)*

### `visit_precise_capture`

```rust
fn visit_precise_capture<'ast, V>(v: &mut V, node: &'ast crate::PreciseCapture)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3213-3224`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3213-L3224)*

### `visit_predicate_lifetime`

```rust
fn visit_predicate_lifetime<'ast, V>(v: &mut V, node: &'ast crate::PredicateLifetime)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3227-3237`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3227-L3237)*

### `visit_predicate_type`

```rust
fn visit_predicate_type<'ast, V>(v: &mut V, node: &'ast crate::PredicateType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3240-3253`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3240-L3253)*

### `visit_qself`

```rust
fn visit_qself<'ast, V>(v: &mut V, node: &'ast crate::QSelf)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3256-3265`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3256-L3265)*

### `visit_range_limits`

```rust
fn visit_range_limits<'ast, V>(v: &mut V, node: &'ast crate::RangeLimits)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3268-3280`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3268-L3280)*

### `visit_receiver`

```rust
fn visit_receiver<'ast, V>(v: &mut V, node: &'ast crate::Receiver)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3283-3300`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3283-L3300)*

### `visit_return_type`

```rust
fn visit_return_type<'ast, V>(v: &mut V, node: &'ast crate::ReturnType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3303-3314`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3303-L3314)*

### `visit_signature`

```rust
fn visit_signature<'ast, V>(v: &mut V, node: &'ast crate::Signature)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3317-3339`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3317-L3339)*

### `visit_span`

```rust
fn visit_span<'ast, V>(v: &mut V, node: &proc_macro2::Span)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3340-3343`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3340-L3343)*

### `visit_static_mutability`

```rust
fn visit_static_mutability<'ast, V>(v: &mut V, node: &'ast crate::StaticMutability)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3346-3356`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3346-L3356)*

### `visit_stmt`

```rust
fn visit_stmt<'ast, V>(v: &mut V, node: &'ast crate::Stmt)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3359-3378`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3359-L3378)*

### `visit_stmt_macro`

```rust
fn visit_stmt_macro<'ast, V>(v: &mut V, node: &'ast crate::StmtMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3381-3390`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3381-L3390)*

### `visit_trait_bound`

```rust
fn visit_trait_bound<'ast, V>(v: &mut V, node: &'ast crate::TraitBound)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3393-3403`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3393-L3403)*

### `visit_trait_bound_modifier`

```rust
fn visit_trait_bound_modifier<'ast, V>(v: &mut V, node: &'ast crate::TraitBoundModifier)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3406-3419`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3406-L3419)*

### `visit_trait_item`

```rust
fn visit_trait_item<'ast, V>(v: &mut V, node: &'ast crate::TraitItem)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3422-3443`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3422-L3443)*

### `visit_trait_item_const`

```rust
fn visit_trait_item_const<'ast, V>(v: &mut V, node: &'ast crate::TraitItemConst)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3446-3463`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3446-L3463)*

### `visit_trait_item_fn`

```rust
fn visit_trait_item_fn<'ast, V>(v: &mut V, node: &'ast crate::TraitItemFn)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3466-3478`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3466-L3478)*

### `visit_trait_item_macro`

```rust
fn visit_trait_item_macro<'ast, V>(v: &mut V, node: &'ast crate::TraitItemMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3481-3490`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3481-L3490)*

### `visit_trait_item_type`

```rust
fn visit_trait_item_type<'ast, V>(v: &mut V, node: &'ast crate::TraitItemType)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3493-3513`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3493-L3513)*

### `visit_type`

```rust
fn visit_type<'ast, V>(v: &mut V, node: &'ast crate::Type)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3516-3567`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3516-L3567)*

### `visit_type_array`

```rust
fn visit_type_array<'ast, V>(v: &mut V, node: &'ast crate::TypeArray)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3570-3578`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3570-L3578)*

### `visit_type_bare_fn`

```rust
fn visit_type_bare_fn<'ast, V>(v: &mut V, node: &'ast crate::TypeBareFn)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3581-3602`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3581-L3602)*

### `visit_type_group`

```rust
fn visit_type_group<'ast, V>(v: &mut V, node: &'ast crate::TypeGroup)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3605-3611`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3605-L3611)*

### `visit_type_impl_trait`

```rust
fn visit_type_impl_trait<'ast, V>(v: &mut V, node: &'ast crate::TypeImplTrait)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3614-3623`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3614-L3623)*

### `visit_type_infer`

```rust
fn visit_type_infer<'ast, V>(v: &mut V, node: &'ast crate::TypeInfer)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3626-3631`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3626-L3631)*

### `visit_type_macro`

```rust
fn visit_type_macro<'ast, V>(v: &mut V, node: &'ast crate::TypeMacro)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3634-3639`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3634-L3639)*

### `visit_type_never`

```rust
fn visit_type_never<'ast, V>(v: &mut V, node: &'ast crate::TypeNever)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3642-3647`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3642-L3647)*

### `visit_type_param`

```rust
fn visit_type_param<'ast, V>(v: &mut V, node: &'ast crate::TypeParam)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3650-3667`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3650-L3667)*

### `visit_type_param_bound`

```rust
fn visit_type_param_bound<'ast, V>(v: &mut V, node: &'ast crate::TypeParamBound)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3670-3688`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3670-L3688)*

### `visit_type_paren`

```rust
fn visit_type_paren<'ast, V>(v: &mut V, node: &'ast crate::TypeParen)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3691-3697`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3691-L3697)*

### `visit_type_path`

```rust
fn visit_type_path<'ast, V>(v: &mut V, node: &'ast crate::TypePath)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3700-3708`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3700-L3708)*

### `visit_type_ptr`

```rust
fn visit_type_ptr<'ast, V>(v: &mut V, node: &'ast crate::TypePtr)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3711-3719`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3711-L3719)*

### `visit_type_reference`

```rust
fn visit_type_reference<'ast, V>(v: &mut V, node: &'ast crate::TypeReference)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3722-3732`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3722-L3732)*

### `visit_type_slice`

```rust
fn visit_type_slice<'ast, V>(v: &mut V, node: &'ast crate::TypeSlice)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3735-3741`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3735-L3741)*

### `visit_type_trait_object`

```rust
fn visit_type_trait_object<'ast, V>(v: &mut V, node: &'ast crate::TypeTraitObject)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3744-3753`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3744-L3753)*

### `visit_type_tuple`

```rust
fn visit_type_tuple<'ast, V>(v: &mut V, node: &'ast crate::TypeTuple)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3756-3765`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3756-L3765)*

### `visit_un_op`

```rust
fn visit_un_op<'ast, V>(v: &mut V, node: &'ast crate::UnOp)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3768-3783`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3768-L3783)*

### `visit_use_glob`

```rust
fn visit_use_glob<'ast, V>(v: &mut V, node: &'ast crate::UseGlob)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3786-3791`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3786-L3791)*

### `visit_use_group`

```rust
fn visit_use_group<'ast, V>(v: &mut V, node: &'ast crate::UseGroup)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3794-3803`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3794-L3803)*

### `visit_use_name`

```rust
fn visit_use_name<'ast, V>(v: &mut V, node: &'ast crate::UseName)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3806-3811`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3806-L3811)*

### `visit_use_path`

```rust
fn visit_use_path<'ast, V>(v: &mut V, node: &'ast crate::UsePath)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3814-3821`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3814-L3821)*

### `visit_use_rename`

```rust
fn visit_use_rename<'ast, V>(v: &mut V, node: &'ast crate::UseRename)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3824-3831`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3824-L3831)*

### `visit_use_tree`

```rust
fn visit_use_tree<'ast, V>(v: &mut V, node: &'ast crate::UseTree)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3834-3855`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3834-L3855)*

### `visit_variadic`

```rust
fn visit_variadic<'ast, V>(v: &mut V, node: &'ast crate::Variadic)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3858-3871`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3858-L3871)*

### `visit_variant`

```rust
fn visit_variant<'ast, V>(v: &mut V, node: &'ast crate::Variant)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3874-3887`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3874-L3887)*

### `visit_vis_restricted`

```rust
fn visit_vis_restricted<'ast, V>(v: &mut V, node: &'ast crate::VisRestricted)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3890-3898`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3890-L3898)*

### `visit_visibility`

```rust
fn visit_visibility<'ast, V>(v: &mut V, node: &'ast crate::Visibility)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3901-3914`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3901-L3914)*

### `visit_where_clause`

```rust
fn visit_where_clause<'ast, V>(v: &mut V, node: &'ast crate::WhereClause)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3917-3926`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3917-L3926)*

### `visit_where_predicate`

```rust
fn visit_where_predicate<'ast, V>(v: &mut V, node: &'ast crate::WherePredicate)
where
    V: Visit<'ast> + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit.rs:3929-3941`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L3929-L3941)*

## Macros

### `full!`

*Defined in [`syn-2.0.111/src/gen/visit.rs:9-13`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L9-L13)*

### `skip!`

*Defined in [`syn-2.0.111/src/gen/visit.rs:20-22`](../../../../.source_1765633015/syn-2.0.111/src/gen/visit.rs#L20-L22)*

