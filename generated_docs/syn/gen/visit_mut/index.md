*[syn](../../index.md) / [gen](../index.md) / [visit_mut](index.md)*

---

# Module `visit_mut`

Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
place.

Each method of the [`VisitMut`](#visitmut) trait is a hook that can be overridden
to customize the behavior when mutating the corresponding type of node.
By default, every method recursively visits the substructure of the
input by invoking the right visitor method of each of its fields.

```rust
use syn::{Attribute, BinOp, Expr, ExprBinary};

pub trait VisitMut {
    /* ... */

    fn visit_expr_binary_mut(&mut self, node: &mut ExprBinary) {
        visit_expr_binary_mut(self, node);
    }

    /* ... */
    fn visit_attribute_mut(&mut self, node: &mut Attribute);
    fn visit_expr_mut(&mut self, node: &mut Expr);
    fn visit_bin_op_mut(&mut self, node: &mut BinOp);
}

pub fn visit_expr_binary_mut<V>(v: &mut V, node: &mut ExprBinary)
where
    V: VisitMut + ?Sized,
{
    for attr in &mut node.attrs {
        v.visit_attribute_mut(attr);
    }
    v.visit_expr_mut(&mut *node.left);
    v.visit_bin_op_mut(&mut node.op);
    v.visit_expr_mut(&mut *node.right);
}

/* ... */
```

<br>

# Example

This mut visitor replace occurrences of u256 suffixed integer literals
like `999u256` with a macro invocation `bigint::u256!(999)`.

```rust
// [dependencies]
// quote = "1.0"
// syn = { version = "2.0", features = ["full", "visit-mut"] }

use quote::quote;
use syn::visit_mut::{self, VisitMut};
use syn::{parse_quote, Expr, File, Lit, LitInt};

struct BigintReplace;

impl VisitMut for BigintReplace {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Lit(expr) = &node {
            if let Lit::Int(int) = &expr.lit {
                if int.suffix() == "u256" {
                    let digits = int.base10_digits();
                    let unsuffixed: LitInt = syn::parse_str(digits).unwrap();
                    *node = parse_quote!(bigint::u256!(#unsuffixed));
                    return;
                }
            }
        }

        // Delegate to the default impl to visit nested expressions.
        visit_mut::visit_expr_mut(self, node);
    }
}

fn main() {
    let code = quote! {
        fn main() {
            let _ = 999u256;
        }
    };

    let mut syntax_tree: File = syn::parse2(code).unwrap();
    BigintReplace.visit_file_mut(&mut syntax_tree);
    println!("{}", quote!(#syntax_tree));
}
```

## Contents

- [Traits](#traits)
  - [`VisitMut`](#visitmut)
- [Functions](#functions)
  - [`visit_abi_mut`](#visit_abi_mut)
  - [`visit_angle_bracketed_generic_arguments_mut`](#visit_angle_bracketed_generic_arguments_mut)
  - [`visit_arm_mut`](#visit_arm_mut)
  - [`visit_assoc_const_mut`](#visit_assoc_const_mut)
  - [`visit_assoc_type_mut`](#visit_assoc_type_mut)
  - [`visit_attr_style_mut`](#visit_attr_style_mut)
  - [`visit_attribute_mut`](#visit_attribute_mut)
  - [`visit_bare_fn_arg_mut`](#visit_bare_fn_arg_mut)
  - [`visit_bare_variadic_mut`](#visit_bare_variadic_mut)
  - [`visit_bin_op_mut`](#visit_bin_op_mut)
  - [`visit_block_mut`](#visit_block_mut)
  - [`visit_bound_lifetimes_mut`](#visit_bound_lifetimes_mut)
  - [`visit_captured_param_mut`](#visit_captured_param_mut)
  - [`visit_const_param_mut`](#visit_const_param_mut)
  - [`visit_constraint_mut`](#visit_constraint_mut)
  - [`visit_data_mut`](#visit_data_mut)
  - [`visit_data_enum_mut`](#visit_data_enum_mut)
  - [`visit_data_struct_mut`](#visit_data_struct_mut)
  - [`visit_data_union_mut`](#visit_data_union_mut)
  - [`visit_derive_input_mut`](#visit_derive_input_mut)
  - [`visit_expr_mut`](#visit_expr_mut)
  - [`visit_expr_array_mut`](#visit_expr_array_mut)
  - [`visit_expr_assign_mut`](#visit_expr_assign_mut)
  - [`visit_expr_async_mut`](#visit_expr_async_mut)
  - [`visit_expr_await_mut`](#visit_expr_await_mut)
  - [`visit_expr_binary_mut`](#visit_expr_binary_mut)
  - [`visit_expr_block_mut`](#visit_expr_block_mut)
  - [`visit_expr_break_mut`](#visit_expr_break_mut)
  - [`visit_expr_call_mut`](#visit_expr_call_mut)
  - [`visit_expr_cast_mut`](#visit_expr_cast_mut)
  - [`visit_expr_closure_mut`](#visit_expr_closure_mut)
  - [`visit_expr_const_mut`](#visit_expr_const_mut)
  - [`visit_expr_continue_mut`](#visit_expr_continue_mut)
  - [`visit_expr_field_mut`](#visit_expr_field_mut)
  - [`visit_expr_for_loop_mut`](#visit_expr_for_loop_mut)
  - [`visit_expr_group_mut`](#visit_expr_group_mut)
  - [`visit_expr_if_mut`](#visit_expr_if_mut)
  - [`visit_expr_index_mut`](#visit_expr_index_mut)
  - [`visit_expr_infer_mut`](#visit_expr_infer_mut)
  - [`visit_expr_let_mut`](#visit_expr_let_mut)
  - [`visit_expr_lit_mut`](#visit_expr_lit_mut)
  - [`visit_expr_loop_mut`](#visit_expr_loop_mut)
  - [`visit_expr_macro_mut`](#visit_expr_macro_mut)
  - [`visit_expr_match_mut`](#visit_expr_match_mut)
  - [`visit_expr_method_call_mut`](#visit_expr_method_call_mut)
  - [`visit_expr_paren_mut`](#visit_expr_paren_mut)
  - [`visit_expr_path_mut`](#visit_expr_path_mut)
  - [`visit_expr_range_mut`](#visit_expr_range_mut)
  - [`visit_expr_raw_addr_mut`](#visit_expr_raw_addr_mut)
  - [`visit_expr_reference_mut`](#visit_expr_reference_mut)
  - [`visit_expr_repeat_mut`](#visit_expr_repeat_mut)
  - [`visit_expr_return_mut`](#visit_expr_return_mut)
  - [`visit_expr_struct_mut`](#visit_expr_struct_mut)
  - [`visit_expr_try_mut`](#visit_expr_try_mut)
  - [`visit_expr_try_block_mut`](#visit_expr_try_block_mut)
  - [`visit_expr_tuple_mut`](#visit_expr_tuple_mut)
  - [`visit_expr_unary_mut`](#visit_expr_unary_mut)
  - [`visit_expr_unsafe_mut`](#visit_expr_unsafe_mut)
  - [`visit_expr_while_mut`](#visit_expr_while_mut)
  - [`visit_expr_yield_mut`](#visit_expr_yield_mut)
  - [`visit_field_mut`](#visit_field_mut)
  - [`visit_field_mutability_mut`](#visit_field_mutability_mut)
  - [`visit_field_pat_mut`](#visit_field_pat_mut)
  - [`visit_field_value_mut`](#visit_field_value_mut)
  - [`visit_fields_mut`](#visit_fields_mut)
  - [`visit_fields_named_mut`](#visit_fields_named_mut)
  - [`visit_fields_unnamed_mut`](#visit_fields_unnamed_mut)
  - [`visit_file_mut`](#visit_file_mut)
  - [`visit_fn_arg_mut`](#visit_fn_arg_mut)
  - [`visit_foreign_item_mut`](#visit_foreign_item_mut)
  - [`visit_foreign_item_fn_mut`](#visit_foreign_item_fn_mut)
  - [`visit_foreign_item_macro_mut`](#visit_foreign_item_macro_mut)
  - [`visit_foreign_item_static_mut`](#visit_foreign_item_static_mut)
  - [`visit_foreign_item_type_mut`](#visit_foreign_item_type_mut)
  - [`visit_generic_argument_mut`](#visit_generic_argument_mut)
  - [`visit_generic_param_mut`](#visit_generic_param_mut)
  - [`visit_generics_mut`](#visit_generics_mut)
  - [`visit_ident_mut`](#visit_ident_mut)
  - [`visit_impl_item_mut`](#visit_impl_item_mut)
  - [`visit_impl_item_const_mut`](#visit_impl_item_const_mut)
  - [`visit_impl_item_fn_mut`](#visit_impl_item_fn_mut)
  - [`visit_impl_item_macro_mut`](#visit_impl_item_macro_mut)
  - [`visit_impl_item_type_mut`](#visit_impl_item_type_mut)
  - [`visit_impl_restriction_mut`](#visit_impl_restriction_mut)
  - [`visit_index_mut`](#visit_index_mut)
  - [`visit_item_mut`](#visit_item_mut)
  - [`visit_item_const_mut`](#visit_item_const_mut)
  - [`visit_item_enum_mut`](#visit_item_enum_mut)
  - [`visit_item_extern_crate_mut`](#visit_item_extern_crate_mut)
  - [`visit_item_fn_mut`](#visit_item_fn_mut)
  - [`visit_item_foreign_mod_mut`](#visit_item_foreign_mod_mut)
  - [`visit_item_impl_mut`](#visit_item_impl_mut)
  - [`visit_item_macro_mut`](#visit_item_macro_mut)
  - [`visit_item_mod_mut`](#visit_item_mod_mut)
  - [`visit_item_static_mut`](#visit_item_static_mut)
  - [`visit_item_struct_mut`](#visit_item_struct_mut)
  - [`visit_item_trait_mut`](#visit_item_trait_mut)
  - [`visit_item_trait_alias_mut`](#visit_item_trait_alias_mut)
  - [`visit_item_type_mut`](#visit_item_type_mut)
  - [`visit_item_union_mut`](#visit_item_union_mut)
  - [`visit_item_use_mut`](#visit_item_use_mut)
  - [`visit_label_mut`](#visit_label_mut)
  - [`visit_lifetime_mut`](#visit_lifetime_mut)
  - [`visit_lifetime_param_mut`](#visit_lifetime_param_mut)
  - [`visit_lit_mut`](#visit_lit_mut)
  - [`visit_lit_bool_mut`](#visit_lit_bool_mut)
  - [`visit_lit_byte_mut`](#visit_lit_byte_mut)
  - [`visit_lit_byte_str_mut`](#visit_lit_byte_str_mut)
  - [`visit_lit_cstr_mut`](#visit_lit_cstr_mut)
  - [`visit_lit_char_mut`](#visit_lit_char_mut)
  - [`visit_lit_float_mut`](#visit_lit_float_mut)
  - [`visit_lit_int_mut`](#visit_lit_int_mut)
  - [`visit_lit_str_mut`](#visit_lit_str_mut)
  - [`visit_local_mut`](#visit_local_mut)
  - [`visit_local_init_mut`](#visit_local_init_mut)
  - [`visit_macro_mut`](#visit_macro_mut)
  - [`visit_macro_delimiter_mut`](#visit_macro_delimiter_mut)
  - [`visit_member_mut`](#visit_member_mut)
  - [`visit_meta_mut`](#visit_meta_mut)
  - [`visit_meta_list_mut`](#visit_meta_list_mut)
  - [`visit_meta_name_value_mut`](#visit_meta_name_value_mut)
  - [`visit_parenthesized_generic_arguments_mut`](#visit_parenthesized_generic_arguments_mut)
  - [`visit_pat_mut`](#visit_pat_mut)
  - [`visit_pat_ident_mut`](#visit_pat_ident_mut)
  - [`visit_pat_or_mut`](#visit_pat_or_mut)
  - [`visit_pat_paren_mut`](#visit_pat_paren_mut)
  - [`visit_pat_reference_mut`](#visit_pat_reference_mut)
  - [`visit_pat_rest_mut`](#visit_pat_rest_mut)
  - [`visit_pat_slice_mut`](#visit_pat_slice_mut)
  - [`visit_pat_struct_mut`](#visit_pat_struct_mut)
  - [`visit_pat_tuple_mut`](#visit_pat_tuple_mut)
  - [`visit_pat_tuple_struct_mut`](#visit_pat_tuple_struct_mut)
  - [`visit_pat_type_mut`](#visit_pat_type_mut)
  - [`visit_pat_wild_mut`](#visit_pat_wild_mut)
  - [`visit_path_mut`](#visit_path_mut)
  - [`visit_path_arguments_mut`](#visit_path_arguments_mut)
  - [`visit_path_segment_mut`](#visit_path_segment_mut)
  - [`visit_pointer_mutability_mut`](#visit_pointer_mutability_mut)
  - [`visit_precise_capture_mut`](#visit_precise_capture_mut)
  - [`visit_predicate_lifetime_mut`](#visit_predicate_lifetime_mut)
  - [`visit_predicate_type_mut`](#visit_predicate_type_mut)
  - [`visit_qself_mut`](#visit_qself_mut)
  - [`visit_range_limits_mut`](#visit_range_limits_mut)
  - [`visit_receiver_mut`](#visit_receiver_mut)
  - [`visit_return_type_mut`](#visit_return_type_mut)
  - [`visit_signature_mut`](#visit_signature_mut)
  - [`visit_span_mut`](#visit_span_mut)
  - [`visit_static_mutability_mut`](#visit_static_mutability_mut)
  - [`visit_stmt_mut`](#visit_stmt_mut)
  - [`visit_stmt_macro_mut`](#visit_stmt_macro_mut)
  - [`visit_trait_bound_mut`](#visit_trait_bound_mut)
  - [`visit_trait_bound_modifier_mut`](#visit_trait_bound_modifier_mut)
  - [`visit_trait_item_mut`](#visit_trait_item_mut)
  - [`visit_trait_item_const_mut`](#visit_trait_item_const_mut)
  - [`visit_trait_item_fn_mut`](#visit_trait_item_fn_mut)
  - [`visit_trait_item_macro_mut`](#visit_trait_item_macro_mut)
  - [`visit_trait_item_type_mut`](#visit_trait_item_type_mut)
  - [`visit_type_mut`](#visit_type_mut)
  - [`visit_type_array_mut`](#visit_type_array_mut)
  - [`visit_type_bare_fn_mut`](#visit_type_bare_fn_mut)
  - [`visit_type_group_mut`](#visit_type_group_mut)
  - [`visit_type_impl_trait_mut`](#visit_type_impl_trait_mut)
  - [`visit_type_infer_mut`](#visit_type_infer_mut)
  - [`visit_type_macro_mut`](#visit_type_macro_mut)
  - [`visit_type_never_mut`](#visit_type_never_mut)
  - [`visit_type_param_mut`](#visit_type_param_mut)
  - [`visit_type_param_bound_mut`](#visit_type_param_bound_mut)
  - [`visit_type_paren_mut`](#visit_type_paren_mut)
  - [`visit_type_path_mut`](#visit_type_path_mut)
  - [`visit_type_ptr_mut`](#visit_type_ptr_mut)
  - [`visit_type_reference_mut`](#visit_type_reference_mut)
  - [`visit_type_slice_mut`](#visit_type_slice_mut)
  - [`visit_type_trait_object_mut`](#visit_type_trait_object_mut)
  - [`visit_type_tuple_mut`](#visit_type_tuple_mut)
  - [`visit_un_op_mut`](#visit_un_op_mut)
  - [`visit_use_glob_mut`](#visit_use_glob_mut)
  - [`visit_use_group_mut`](#visit_use_group_mut)
  - [`visit_use_name_mut`](#visit_use_name_mut)
  - [`visit_use_path_mut`](#visit_use_path_mut)
  - [`visit_use_rename_mut`](#visit_use_rename_mut)
  - [`visit_use_tree_mut`](#visit_use_tree_mut)
  - [`visit_variadic_mut`](#visit_variadic_mut)
  - [`visit_variant_mut`](#visit_variant_mut)
  - [`visit_vis_restricted_mut`](#visit_vis_restricted_mut)
  - [`visit_visibility_mut`](#visit_visibility_mut)
  - [`visit_where_clause_mut`](#visit_where_clause_mut)
  - [`visit_where_predicate_mut`](#visit_where_predicate_mut)
- [Macros](#macros)
  - [`full!`](#full)
  - [`skip!`](#skip)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VisitMut`](#visitmut) | trait | Syntax tree traversal to mutate an exclusive borrow of a syntax tree in place. |
| [`visit_abi_mut`](#visit_abi_mut) | fn |  |
| [`visit_angle_bracketed_generic_arguments_mut`](#visit_angle_bracketed_generic_arguments_mut) | fn |  |
| [`visit_arm_mut`](#visit_arm_mut) | fn |  |
| [`visit_assoc_const_mut`](#visit_assoc_const_mut) | fn |  |
| [`visit_assoc_type_mut`](#visit_assoc_type_mut) | fn |  |
| [`visit_attr_style_mut`](#visit_attr_style_mut) | fn |  |
| [`visit_attribute_mut`](#visit_attribute_mut) | fn |  |
| [`visit_bare_fn_arg_mut`](#visit_bare_fn_arg_mut) | fn |  |
| [`visit_bare_variadic_mut`](#visit_bare_variadic_mut) | fn |  |
| [`visit_bin_op_mut`](#visit_bin_op_mut) | fn |  |
| [`visit_block_mut`](#visit_block_mut) | fn |  |
| [`visit_bound_lifetimes_mut`](#visit_bound_lifetimes_mut) | fn |  |
| [`visit_captured_param_mut`](#visit_captured_param_mut) | fn |  |
| [`visit_const_param_mut`](#visit_const_param_mut) | fn |  |
| [`visit_constraint_mut`](#visit_constraint_mut) | fn |  |
| [`visit_data_mut`](#visit_data_mut) | fn |  |
| [`visit_data_enum_mut`](#visit_data_enum_mut) | fn |  |
| [`visit_data_struct_mut`](#visit_data_struct_mut) | fn |  |
| [`visit_data_union_mut`](#visit_data_union_mut) | fn |  |
| [`visit_derive_input_mut`](#visit_derive_input_mut) | fn |  |
| [`visit_expr_mut`](#visit_expr_mut) | fn |  |
| [`visit_expr_array_mut`](#visit_expr_array_mut) | fn |  |
| [`visit_expr_assign_mut`](#visit_expr_assign_mut) | fn |  |
| [`visit_expr_async_mut`](#visit_expr_async_mut) | fn |  |
| [`visit_expr_await_mut`](#visit_expr_await_mut) | fn |  |
| [`visit_expr_binary_mut`](#visit_expr_binary_mut) | fn |  |
| [`visit_expr_block_mut`](#visit_expr_block_mut) | fn |  |
| [`visit_expr_break_mut`](#visit_expr_break_mut) | fn |  |
| [`visit_expr_call_mut`](#visit_expr_call_mut) | fn |  |
| [`visit_expr_cast_mut`](#visit_expr_cast_mut) | fn |  |
| [`visit_expr_closure_mut`](#visit_expr_closure_mut) | fn |  |
| [`visit_expr_const_mut`](#visit_expr_const_mut) | fn |  |
| [`visit_expr_continue_mut`](#visit_expr_continue_mut) | fn |  |
| [`visit_expr_field_mut`](#visit_expr_field_mut) | fn |  |
| [`visit_expr_for_loop_mut`](#visit_expr_for_loop_mut) | fn |  |
| [`visit_expr_group_mut`](#visit_expr_group_mut) | fn |  |
| [`visit_expr_if_mut`](#visit_expr_if_mut) | fn |  |
| [`visit_expr_index_mut`](#visit_expr_index_mut) | fn |  |
| [`visit_expr_infer_mut`](#visit_expr_infer_mut) | fn |  |
| [`visit_expr_let_mut`](#visit_expr_let_mut) | fn |  |
| [`visit_expr_lit_mut`](#visit_expr_lit_mut) | fn |  |
| [`visit_expr_loop_mut`](#visit_expr_loop_mut) | fn |  |
| [`visit_expr_macro_mut`](#visit_expr_macro_mut) | fn |  |
| [`visit_expr_match_mut`](#visit_expr_match_mut) | fn |  |
| [`visit_expr_method_call_mut`](#visit_expr_method_call_mut) | fn |  |
| [`visit_expr_paren_mut`](#visit_expr_paren_mut) | fn |  |
| [`visit_expr_path_mut`](#visit_expr_path_mut) | fn |  |
| [`visit_expr_range_mut`](#visit_expr_range_mut) | fn |  |
| [`visit_expr_raw_addr_mut`](#visit_expr_raw_addr_mut) | fn |  |
| [`visit_expr_reference_mut`](#visit_expr_reference_mut) | fn |  |
| [`visit_expr_repeat_mut`](#visit_expr_repeat_mut) | fn |  |
| [`visit_expr_return_mut`](#visit_expr_return_mut) | fn |  |
| [`visit_expr_struct_mut`](#visit_expr_struct_mut) | fn |  |
| [`visit_expr_try_mut`](#visit_expr_try_mut) | fn |  |
| [`visit_expr_try_block_mut`](#visit_expr_try_block_mut) | fn |  |
| [`visit_expr_tuple_mut`](#visit_expr_tuple_mut) | fn |  |
| [`visit_expr_unary_mut`](#visit_expr_unary_mut) | fn |  |
| [`visit_expr_unsafe_mut`](#visit_expr_unsafe_mut) | fn |  |
| [`visit_expr_while_mut`](#visit_expr_while_mut) | fn |  |
| [`visit_expr_yield_mut`](#visit_expr_yield_mut) | fn |  |
| [`visit_field_mut`](#visit_field_mut) | fn |  |
| [`visit_field_mutability_mut`](#visit_field_mutability_mut) | fn |  |
| [`visit_field_pat_mut`](#visit_field_pat_mut) | fn |  |
| [`visit_field_value_mut`](#visit_field_value_mut) | fn |  |
| [`visit_fields_mut`](#visit_fields_mut) | fn |  |
| [`visit_fields_named_mut`](#visit_fields_named_mut) | fn |  |
| [`visit_fields_unnamed_mut`](#visit_fields_unnamed_mut) | fn |  |
| [`visit_file_mut`](#visit_file_mut) | fn |  |
| [`visit_fn_arg_mut`](#visit_fn_arg_mut) | fn |  |
| [`visit_foreign_item_mut`](#visit_foreign_item_mut) | fn |  |
| [`visit_foreign_item_fn_mut`](#visit_foreign_item_fn_mut) | fn |  |
| [`visit_foreign_item_macro_mut`](#visit_foreign_item_macro_mut) | fn |  |
| [`visit_foreign_item_static_mut`](#visit_foreign_item_static_mut) | fn |  |
| [`visit_foreign_item_type_mut`](#visit_foreign_item_type_mut) | fn |  |
| [`visit_generic_argument_mut`](#visit_generic_argument_mut) | fn |  |
| [`visit_generic_param_mut`](#visit_generic_param_mut) | fn |  |
| [`visit_generics_mut`](#visit_generics_mut) | fn |  |
| [`visit_ident_mut`](#visit_ident_mut) | fn |  |
| [`visit_impl_item_mut`](#visit_impl_item_mut) | fn |  |
| [`visit_impl_item_const_mut`](#visit_impl_item_const_mut) | fn |  |
| [`visit_impl_item_fn_mut`](#visit_impl_item_fn_mut) | fn |  |
| [`visit_impl_item_macro_mut`](#visit_impl_item_macro_mut) | fn |  |
| [`visit_impl_item_type_mut`](#visit_impl_item_type_mut) | fn |  |
| [`visit_impl_restriction_mut`](#visit_impl_restriction_mut) | fn |  |
| [`visit_index_mut`](#visit_index_mut) | fn |  |
| [`visit_item_mut`](#visit_item_mut) | fn |  |
| [`visit_item_const_mut`](#visit_item_const_mut) | fn |  |
| [`visit_item_enum_mut`](#visit_item_enum_mut) | fn |  |
| [`visit_item_extern_crate_mut`](#visit_item_extern_crate_mut) | fn |  |
| [`visit_item_fn_mut`](#visit_item_fn_mut) | fn |  |
| [`visit_item_foreign_mod_mut`](#visit_item_foreign_mod_mut) | fn |  |
| [`visit_item_impl_mut`](#visit_item_impl_mut) | fn |  |
| [`visit_item_macro_mut`](#visit_item_macro_mut) | fn |  |
| [`visit_item_mod_mut`](#visit_item_mod_mut) | fn |  |
| [`visit_item_static_mut`](#visit_item_static_mut) | fn |  |
| [`visit_item_struct_mut`](#visit_item_struct_mut) | fn |  |
| [`visit_item_trait_mut`](#visit_item_trait_mut) | fn |  |
| [`visit_item_trait_alias_mut`](#visit_item_trait_alias_mut) | fn |  |
| [`visit_item_type_mut`](#visit_item_type_mut) | fn |  |
| [`visit_item_union_mut`](#visit_item_union_mut) | fn |  |
| [`visit_item_use_mut`](#visit_item_use_mut) | fn |  |
| [`visit_label_mut`](#visit_label_mut) | fn |  |
| [`visit_lifetime_mut`](#visit_lifetime_mut) | fn |  |
| [`visit_lifetime_param_mut`](#visit_lifetime_param_mut) | fn |  |
| [`visit_lit_mut`](#visit_lit_mut) | fn |  |
| [`visit_lit_bool_mut`](#visit_lit_bool_mut) | fn |  |
| [`visit_lit_byte_mut`](#visit_lit_byte_mut) | fn |  |
| [`visit_lit_byte_str_mut`](#visit_lit_byte_str_mut) | fn |  |
| [`visit_lit_cstr_mut`](#visit_lit_cstr_mut) | fn |  |
| [`visit_lit_char_mut`](#visit_lit_char_mut) | fn |  |
| [`visit_lit_float_mut`](#visit_lit_float_mut) | fn |  |
| [`visit_lit_int_mut`](#visit_lit_int_mut) | fn |  |
| [`visit_lit_str_mut`](#visit_lit_str_mut) | fn |  |
| [`visit_local_mut`](#visit_local_mut) | fn |  |
| [`visit_local_init_mut`](#visit_local_init_mut) | fn |  |
| [`visit_macro_mut`](#visit_macro_mut) | fn |  |
| [`visit_macro_delimiter_mut`](#visit_macro_delimiter_mut) | fn |  |
| [`visit_member_mut`](#visit_member_mut) | fn |  |
| [`visit_meta_mut`](#visit_meta_mut) | fn |  |
| [`visit_meta_list_mut`](#visit_meta_list_mut) | fn |  |
| [`visit_meta_name_value_mut`](#visit_meta_name_value_mut) | fn |  |
| [`visit_parenthesized_generic_arguments_mut`](#visit_parenthesized_generic_arguments_mut) | fn |  |
| [`visit_pat_mut`](#visit_pat_mut) | fn |  |
| [`visit_pat_ident_mut`](#visit_pat_ident_mut) | fn |  |
| [`visit_pat_or_mut`](#visit_pat_or_mut) | fn |  |
| [`visit_pat_paren_mut`](#visit_pat_paren_mut) | fn |  |
| [`visit_pat_reference_mut`](#visit_pat_reference_mut) | fn |  |
| [`visit_pat_rest_mut`](#visit_pat_rest_mut) | fn |  |
| [`visit_pat_slice_mut`](#visit_pat_slice_mut) | fn |  |
| [`visit_pat_struct_mut`](#visit_pat_struct_mut) | fn |  |
| [`visit_pat_tuple_mut`](#visit_pat_tuple_mut) | fn |  |
| [`visit_pat_tuple_struct_mut`](#visit_pat_tuple_struct_mut) | fn |  |
| [`visit_pat_type_mut`](#visit_pat_type_mut) | fn |  |
| [`visit_pat_wild_mut`](#visit_pat_wild_mut) | fn |  |
| [`visit_path_mut`](#visit_path_mut) | fn |  |
| [`visit_path_arguments_mut`](#visit_path_arguments_mut) | fn |  |
| [`visit_path_segment_mut`](#visit_path_segment_mut) | fn |  |
| [`visit_pointer_mutability_mut`](#visit_pointer_mutability_mut) | fn |  |
| [`visit_precise_capture_mut`](#visit_precise_capture_mut) | fn |  |
| [`visit_predicate_lifetime_mut`](#visit_predicate_lifetime_mut) | fn |  |
| [`visit_predicate_type_mut`](#visit_predicate_type_mut) | fn |  |
| [`visit_qself_mut`](#visit_qself_mut) | fn |  |
| [`visit_range_limits_mut`](#visit_range_limits_mut) | fn |  |
| [`visit_receiver_mut`](#visit_receiver_mut) | fn |  |
| [`visit_return_type_mut`](#visit_return_type_mut) | fn |  |
| [`visit_signature_mut`](#visit_signature_mut) | fn |  |
| [`visit_span_mut`](#visit_span_mut) | fn |  |
| [`visit_static_mutability_mut`](#visit_static_mutability_mut) | fn |  |
| [`visit_stmt_mut`](#visit_stmt_mut) | fn |  |
| [`visit_stmt_macro_mut`](#visit_stmt_macro_mut) | fn |  |
| [`visit_trait_bound_mut`](#visit_trait_bound_mut) | fn |  |
| [`visit_trait_bound_modifier_mut`](#visit_trait_bound_modifier_mut) | fn |  |
| [`visit_trait_item_mut`](#visit_trait_item_mut) | fn |  |
| [`visit_trait_item_const_mut`](#visit_trait_item_const_mut) | fn |  |
| [`visit_trait_item_fn_mut`](#visit_trait_item_fn_mut) | fn |  |
| [`visit_trait_item_macro_mut`](#visit_trait_item_macro_mut) | fn |  |
| [`visit_trait_item_type_mut`](#visit_trait_item_type_mut) | fn |  |
| [`visit_type_mut`](#visit_type_mut) | fn |  |
| [`visit_type_array_mut`](#visit_type_array_mut) | fn |  |
| [`visit_type_bare_fn_mut`](#visit_type_bare_fn_mut) | fn |  |
| [`visit_type_group_mut`](#visit_type_group_mut) | fn |  |
| [`visit_type_impl_trait_mut`](#visit_type_impl_trait_mut) | fn |  |
| [`visit_type_infer_mut`](#visit_type_infer_mut) | fn |  |
| [`visit_type_macro_mut`](#visit_type_macro_mut) | fn |  |
| [`visit_type_never_mut`](#visit_type_never_mut) | fn |  |
| [`visit_type_param_mut`](#visit_type_param_mut) | fn |  |
| [`visit_type_param_bound_mut`](#visit_type_param_bound_mut) | fn |  |
| [`visit_type_paren_mut`](#visit_type_paren_mut) | fn |  |
| [`visit_type_path_mut`](#visit_type_path_mut) | fn |  |
| [`visit_type_ptr_mut`](#visit_type_ptr_mut) | fn |  |
| [`visit_type_reference_mut`](#visit_type_reference_mut) | fn |  |
| [`visit_type_slice_mut`](#visit_type_slice_mut) | fn |  |
| [`visit_type_trait_object_mut`](#visit_type_trait_object_mut) | fn |  |
| [`visit_type_tuple_mut`](#visit_type_tuple_mut) | fn |  |
| [`visit_un_op_mut`](#visit_un_op_mut) | fn |  |
| [`visit_use_glob_mut`](#visit_use_glob_mut) | fn |  |
| [`visit_use_group_mut`](#visit_use_group_mut) | fn |  |
| [`visit_use_name_mut`](#visit_use_name_mut) | fn |  |
| [`visit_use_path_mut`](#visit_use_path_mut) | fn |  |
| [`visit_use_rename_mut`](#visit_use_rename_mut) | fn |  |
| [`visit_use_tree_mut`](#visit_use_tree_mut) | fn |  |
| [`visit_variadic_mut`](#visit_variadic_mut) | fn |  |
| [`visit_variant_mut`](#visit_variant_mut) | fn |  |
| [`visit_vis_restricted_mut`](#visit_vis_restricted_mut) | fn |  |
| [`visit_visibility_mut`](#visit_visibility_mut) | fn |  |
| [`visit_where_clause_mut`](#visit_where_clause_mut) | fn |  |
| [`visit_where_predicate_mut`](#visit_where_predicate_mut) | fn |  |
| [`full!`](#full) | macro |  |
| [`skip!`](#skip) | macro |  |

## Traits

### `VisitMut`

```rust
trait VisitMut { ... }
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:29-953`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L29-L953)*

Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
place.

See the [module documentation] for details.


#### Provided Methods

- `fn visit_abi_mut(&mut self, i: &mut crate::Abi)`

- `fn visit_angle_bracketed_generic_arguments_mut(&mut self, i: &mut crate::AngleBracketedGenericArguments)`

- `fn visit_arm_mut(&mut self, i: &mut crate::Arm)`

- `fn visit_assoc_const_mut(&mut self, i: &mut crate::AssocConst)`

- `fn visit_assoc_type_mut(&mut self, i: &mut crate::AssocType)`

- `fn visit_attr_style_mut(&mut self, i: &mut crate::AttrStyle)`

- `fn visit_attribute_mut(&mut self, i: &mut crate::Attribute)`

- `fn visit_attributes_mut(&mut self, i: &mut Vec<crate::Attribute>)`

- `fn visit_bare_fn_arg_mut(&mut self, i: &mut crate::BareFnArg)`

- `fn visit_bare_variadic_mut(&mut self, i: &mut crate::BareVariadic)`

- `fn visit_bin_op_mut(&mut self, i: &mut crate::BinOp)`

- `fn visit_block_mut(&mut self, i: &mut crate::Block)`

- `fn visit_bound_lifetimes_mut(&mut self, i: &mut crate::BoundLifetimes)`

- `fn visit_captured_param_mut(&mut self, i: &mut crate::CapturedParam)`

- `fn visit_const_param_mut(&mut self, i: &mut crate::ConstParam)`

- `fn visit_constraint_mut(&mut self, i: &mut crate::Constraint)`

- `fn visit_data_mut(&mut self, i: &mut crate::Data)`

- `fn visit_data_enum_mut(&mut self, i: &mut crate::DataEnum)`

- `fn visit_data_struct_mut(&mut self, i: &mut crate::DataStruct)`

- `fn visit_data_union_mut(&mut self, i: &mut crate::DataUnion)`

- `fn visit_derive_input_mut(&mut self, i: &mut crate::DeriveInput)`

- `fn visit_expr_mut(&mut self, i: &mut crate::Expr)`

- `fn visit_expr_array_mut(&mut self, i: &mut crate::ExprArray)`

- `fn visit_expr_assign_mut(&mut self, i: &mut crate::ExprAssign)`

- `fn visit_expr_async_mut(&mut self, i: &mut crate::ExprAsync)`

- `fn visit_expr_await_mut(&mut self, i: &mut crate::ExprAwait)`

- `fn visit_expr_binary_mut(&mut self, i: &mut crate::ExprBinary)`

- `fn visit_expr_block_mut(&mut self, i: &mut crate::ExprBlock)`

- `fn visit_expr_break_mut(&mut self, i: &mut crate::ExprBreak)`

- `fn visit_expr_call_mut(&mut self, i: &mut crate::ExprCall)`

- `fn visit_expr_cast_mut(&mut self, i: &mut crate::ExprCast)`

- `fn visit_expr_closure_mut(&mut self, i: &mut crate::ExprClosure)`

- `fn visit_expr_const_mut(&mut self, i: &mut crate::ExprConst)`

- `fn visit_expr_continue_mut(&mut self, i: &mut crate::ExprContinue)`

- `fn visit_expr_field_mut(&mut self, i: &mut crate::ExprField)`

- `fn visit_expr_for_loop_mut(&mut self, i: &mut crate::ExprForLoop)`

- `fn visit_expr_group_mut(&mut self, i: &mut crate::ExprGroup)`

- `fn visit_expr_if_mut(&mut self, i: &mut crate::ExprIf)`

- `fn visit_expr_index_mut(&mut self, i: &mut crate::ExprIndex)`

- `fn visit_expr_infer_mut(&mut self, i: &mut crate::ExprInfer)`

- `fn visit_expr_let_mut(&mut self, i: &mut crate::ExprLet)`

- `fn visit_expr_lit_mut(&mut self, i: &mut crate::ExprLit)`

- `fn visit_expr_loop_mut(&mut self, i: &mut crate::ExprLoop)`

- `fn visit_expr_macro_mut(&mut self, i: &mut crate::ExprMacro)`

- `fn visit_expr_match_mut(&mut self, i: &mut crate::ExprMatch)`

- `fn visit_expr_method_call_mut(&mut self, i: &mut crate::ExprMethodCall)`

- `fn visit_expr_paren_mut(&mut self, i: &mut crate::ExprParen)`

- `fn visit_expr_path_mut(&mut self, i: &mut crate::ExprPath)`

- `fn visit_expr_range_mut(&mut self, i: &mut crate::ExprRange)`

- `fn visit_expr_raw_addr_mut(&mut self, i: &mut crate::ExprRawAddr)`

- `fn visit_expr_reference_mut(&mut self, i: &mut crate::ExprReference)`

- `fn visit_expr_repeat_mut(&mut self, i: &mut crate::ExprRepeat)`

- `fn visit_expr_return_mut(&mut self, i: &mut crate::ExprReturn)`

- `fn visit_expr_struct_mut(&mut self, i: &mut crate::ExprStruct)`

- `fn visit_expr_try_mut(&mut self, i: &mut crate::ExprTry)`

- `fn visit_expr_try_block_mut(&mut self, i: &mut crate::ExprTryBlock)`

- `fn visit_expr_tuple_mut(&mut self, i: &mut crate::ExprTuple)`

- `fn visit_expr_unary_mut(&mut self, i: &mut crate::ExprUnary)`

- `fn visit_expr_unsafe_mut(&mut self, i: &mut crate::ExprUnsafe)`

- `fn visit_expr_while_mut(&mut self, i: &mut crate::ExprWhile)`

- `fn visit_expr_yield_mut(&mut self, i: &mut crate::ExprYield)`

- `fn visit_field_mut(&mut self, i: &mut crate::Field)`

- `fn visit_field_mutability_mut(&mut self, i: &mut crate::FieldMutability)`

- `fn visit_field_pat_mut(&mut self, i: &mut crate::FieldPat)`

- `fn visit_field_value_mut(&mut self, i: &mut crate::FieldValue)`

- `fn visit_fields_mut(&mut self, i: &mut crate::Fields)`

- `fn visit_fields_named_mut(&mut self, i: &mut crate::FieldsNamed)`

- `fn visit_fields_unnamed_mut(&mut self, i: &mut crate::FieldsUnnamed)`

- `fn visit_file_mut(&mut self, i: &mut crate::File)`

- `fn visit_fn_arg_mut(&mut self, i: &mut crate::FnArg)`

- `fn visit_foreign_item_mut(&mut self, i: &mut crate::ForeignItem)`

- `fn visit_foreign_item_fn_mut(&mut self, i: &mut crate::ForeignItemFn)`

- `fn visit_foreign_item_macro_mut(&mut self, i: &mut crate::ForeignItemMacro)`

- `fn visit_foreign_item_static_mut(&mut self, i: &mut crate::ForeignItemStatic)`

- `fn visit_foreign_item_type_mut(&mut self, i: &mut crate::ForeignItemType)`

- `fn visit_generic_argument_mut(&mut self, i: &mut crate::GenericArgument)`

- `fn visit_generic_param_mut(&mut self, i: &mut crate::GenericParam)`

- `fn visit_generics_mut(&mut self, i: &mut crate::Generics)`

- `fn visit_ident_mut(&mut self, i: &mut proc_macro2::Ident)`

- `fn visit_impl_item_mut(&mut self, i: &mut crate::ImplItem)`

- `fn visit_impl_item_const_mut(&mut self, i: &mut crate::ImplItemConst)`

- `fn visit_impl_item_fn_mut(&mut self, i: &mut crate::ImplItemFn)`

- `fn visit_impl_item_macro_mut(&mut self, i: &mut crate::ImplItemMacro)`

- `fn visit_impl_item_type_mut(&mut self, i: &mut crate::ImplItemType)`

- `fn visit_impl_restriction_mut(&mut self, i: &mut crate::ImplRestriction)`

- `fn visit_index_mut(&mut self, i: &mut crate::Index)`

- `fn visit_item_mut(&mut self, i: &mut crate::Item)`

- `fn visit_item_const_mut(&mut self, i: &mut crate::ItemConst)`

- `fn visit_item_enum_mut(&mut self, i: &mut crate::ItemEnum)`

- `fn visit_item_extern_crate_mut(&mut self, i: &mut crate::ItemExternCrate)`

- `fn visit_item_fn_mut(&mut self, i: &mut crate::ItemFn)`

- `fn visit_item_foreign_mod_mut(&mut self, i: &mut crate::ItemForeignMod)`

- `fn visit_item_impl_mut(&mut self, i: &mut crate::ItemImpl)`

- `fn visit_item_macro_mut(&mut self, i: &mut crate::ItemMacro)`

- `fn visit_item_mod_mut(&mut self, i: &mut crate::ItemMod)`

- `fn visit_item_static_mut(&mut self, i: &mut crate::ItemStatic)`

- `fn visit_item_struct_mut(&mut self, i: &mut crate::ItemStruct)`

- `fn visit_item_trait_mut(&mut self, i: &mut crate::ItemTrait)`

- `fn visit_item_trait_alias_mut(&mut self, i: &mut crate::ItemTraitAlias)`

- `fn visit_item_type_mut(&mut self, i: &mut crate::ItemType)`

- `fn visit_item_union_mut(&mut self, i: &mut crate::ItemUnion)`

- `fn visit_item_use_mut(&mut self, i: &mut crate::ItemUse)`

- `fn visit_label_mut(&mut self, i: &mut crate::Label)`

- `fn visit_lifetime_mut(&mut self, i: &mut crate::Lifetime)`

- `fn visit_lifetime_param_mut(&mut self, i: &mut crate::LifetimeParam)`

- `fn visit_lit_mut(&mut self, i: &mut crate::Lit)`

- `fn visit_lit_bool_mut(&mut self, i: &mut crate::LitBool)`

- `fn visit_lit_byte_mut(&mut self, i: &mut crate::LitByte)`

- `fn visit_lit_byte_str_mut(&mut self, i: &mut crate::LitByteStr)`

- `fn visit_lit_cstr_mut(&mut self, i: &mut crate::LitCStr)`

- `fn visit_lit_char_mut(&mut self, i: &mut crate::LitChar)`

- `fn visit_lit_float_mut(&mut self, i: &mut crate::LitFloat)`

- `fn visit_lit_int_mut(&mut self, i: &mut crate::LitInt)`

- `fn visit_lit_str_mut(&mut self, i: &mut crate::LitStr)`

- `fn visit_local_mut(&mut self, i: &mut crate::Local)`

- `fn visit_local_init_mut(&mut self, i: &mut crate::LocalInit)`

- `fn visit_macro_mut(&mut self, i: &mut crate::Macro)`

- `fn visit_macro_delimiter_mut(&mut self, i: &mut crate::MacroDelimiter)`

- `fn visit_member_mut(&mut self, i: &mut crate::Member)`

- `fn visit_meta_mut(&mut self, i: &mut crate::Meta)`

- `fn visit_meta_list_mut(&mut self, i: &mut crate::MetaList)`

- `fn visit_meta_name_value_mut(&mut self, i: &mut crate::MetaNameValue)`

- `fn visit_parenthesized_generic_arguments_mut(&mut self, i: &mut crate::ParenthesizedGenericArguments)`

- `fn visit_pat_mut(&mut self, i: &mut crate::Pat)`

- `fn visit_pat_ident_mut(&mut self, i: &mut crate::PatIdent)`

- `fn visit_pat_or_mut(&mut self, i: &mut crate::PatOr)`

- `fn visit_pat_paren_mut(&mut self, i: &mut crate::PatParen)`

- `fn visit_pat_reference_mut(&mut self, i: &mut crate::PatReference)`

- `fn visit_pat_rest_mut(&mut self, i: &mut crate::PatRest)`

- `fn visit_pat_slice_mut(&mut self, i: &mut crate::PatSlice)`

- `fn visit_pat_struct_mut(&mut self, i: &mut crate::PatStruct)`

- `fn visit_pat_tuple_mut(&mut self, i: &mut crate::PatTuple)`

- `fn visit_pat_tuple_struct_mut(&mut self, i: &mut crate::PatTupleStruct)`

- `fn visit_pat_type_mut(&mut self, i: &mut crate::PatType)`

- `fn visit_pat_wild_mut(&mut self, i: &mut crate::PatWild)`

- `fn visit_path_mut(&mut self, i: &mut crate::Path)`

- `fn visit_path_arguments_mut(&mut self, i: &mut crate::PathArguments)`

- `fn visit_path_segment_mut(&mut self, i: &mut crate::PathSegment)`

- `fn visit_pointer_mutability_mut(&mut self, i: &mut crate::PointerMutability)`

- `fn visit_precise_capture_mut(&mut self, i: &mut crate::PreciseCapture)`

- `fn visit_predicate_lifetime_mut(&mut self, i: &mut crate::PredicateLifetime)`

- `fn visit_predicate_type_mut(&mut self, i: &mut crate::PredicateType)`

- `fn visit_qself_mut(&mut self, i: &mut crate::QSelf)`

- `fn visit_range_limits_mut(&mut self, i: &mut crate::RangeLimits)`

- `fn visit_receiver_mut(&mut self, i: &mut crate::Receiver)`

- `fn visit_return_type_mut(&mut self, i: &mut crate::ReturnType)`

- `fn visit_signature_mut(&mut self, i: &mut crate::Signature)`

- `fn visit_span_mut(&mut self, i: &mut proc_macro2::Span)`

- `fn visit_static_mutability_mut(&mut self, i: &mut crate::StaticMutability)`

- `fn visit_stmt_mut(&mut self, i: &mut crate::Stmt)`

- `fn visit_stmt_macro_mut(&mut self, i: &mut crate::StmtMacro)`

- `fn visit_token_stream_mut(&mut self, i: &mut proc_macro2::TokenStream)`

- `fn visit_trait_bound_mut(&mut self, i: &mut crate::TraitBound)`

- `fn visit_trait_bound_modifier_mut(&mut self, i: &mut crate::TraitBoundModifier)`

- `fn visit_trait_item_mut(&mut self, i: &mut crate::TraitItem)`

- `fn visit_trait_item_const_mut(&mut self, i: &mut crate::TraitItemConst)`

- `fn visit_trait_item_fn_mut(&mut self, i: &mut crate::TraitItemFn)`

- `fn visit_trait_item_macro_mut(&mut self, i: &mut crate::TraitItemMacro)`

- `fn visit_trait_item_type_mut(&mut self, i: &mut crate::TraitItemType)`

- `fn visit_type_mut(&mut self, i: &mut crate::Type)`

- `fn visit_type_array_mut(&mut self, i: &mut crate::TypeArray)`

- `fn visit_type_bare_fn_mut(&mut self, i: &mut crate::TypeBareFn)`

- `fn visit_type_group_mut(&mut self, i: &mut crate::TypeGroup)`

- `fn visit_type_impl_trait_mut(&mut self, i: &mut crate::TypeImplTrait)`

- `fn visit_type_infer_mut(&mut self, i: &mut crate::TypeInfer)`

- `fn visit_type_macro_mut(&mut self, i: &mut crate::TypeMacro)`

- `fn visit_type_never_mut(&mut self, i: &mut crate::TypeNever)`

- `fn visit_type_param_mut(&mut self, i: &mut crate::TypeParam)`

- `fn visit_type_param_bound_mut(&mut self, i: &mut crate::TypeParamBound)`

- `fn visit_type_paren_mut(&mut self, i: &mut crate::TypeParen)`

- `fn visit_type_path_mut(&mut self, i: &mut crate::TypePath)`

- `fn visit_type_ptr_mut(&mut self, i: &mut crate::TypePtr)`

- `fn visit_type_reference_mut(&mut self, i: &mut crate::TypeReference)`

- `fn visit_type_slice_mut(&mut self, i: &mut crate::TypeSlice)`

- `fn visit_type_trait_object_mut(&mut self, i: &mut crate::TypeTraitObject)`

- `fn visit_type_tuple_mut(&mut self, i: &mut crate::TypeTuple)`

- `fn visit_un_op_mut(&mut self, i: &mut crate::UnOp)`

- `fn visit_use_glob_mut(&mut self, i: &mut crate::UseGlob)`

- `fn visit_use_group_mut(&mut self, i: &mut crate::UseGroup)`

- `fn visit_use_name_mut(&mut self, i: &mut crate::UseName)`

- `fn visit_use_path_mut(&mut self, i: &mut crate::UsePath)`

- `fn visit_use_rename_mut(&mut self, i: &mut crate::UseRename)`

- `fn visit_use_tree_mut(&mut self, i: &mut crate::UseTree)`

- `fn visit_variadic_mut(&mut self, i: &mut crate::Variadic)`

- `fn visit_variant_mut(&mut self, i: &mut crate::Variant)`

- `fn visit_vis_restricted_mut(&mut self, i: &mut crate::VisRestricted)`

- `fn visit_visibility_mut(&mut self, i: &mut crate::Visibility)`

- `fn visit_where_clause_mut(&mut self, i: &mut crate::WhereClause)`

- `fn visit_where_predicate_mut(&mut self, i: &mut crate::WherePredicate)`

## Functions

### `visit_abi_mut`

```rust
fn visit_abi_mut<V>(v: &mut V, node: &mut crate::Abi)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:956-964`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L956-L964)*

### `visit_angle_bracketed_generic_arguments_mut`

```rust
fn visit_angle_bracketed_generic_arguments_mut<V>(v: &mut V, node: &mut crate::AngleBracketedGenericArguments)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:967-981`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L967-L981)*

### `visit_arm_mut`

```rust
fn visit_arm_mut<V>(v: &mut V, node: &mut crate::Arm)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:984-997`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L984-L997)*

### `visit_assoc_const_mut`

```rust
fn visit_assoc_const_mut<V>(v: &mut V, node: &mut crate::AssocConst)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1000-1010`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1000-L1010)*

### `visit_assoc_type_mut`

```rust
fn visit_assoc_type_mut<V>(v: &mut V, node: &mut crate::AssocType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1013-1023`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1013-L1023)*

### `visit_attr_style_mut`

```rust
fn visit_attr_style_mut<V>(v: &mut V, node: &mut crate::AttrStyle)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1026-1036`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1026-L1036)*

### `visit_attribute_mut`

```rust
fn visit_attribute_mut<V>(v: &mut V, node: &mut crate::Attribute)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1039-1047`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1039-L1047)*

### `visit_bare_fn_arg_mut`

```rust
fn visit_bare_fn_arg_mut<V>(v: &mut V, node: &mut crate::BareFnArg)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1050-1060`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1050-L1060)*

### `visit_bare_variadic_mut`

```rust
fn visit_bare_variadic_mut<V>(v: &mut V, node: &mut crate::BareVariadic)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1063-1074`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1063-L1074)*

### `visit_bin_op_mut`

```rust
fn visit_bin_op_mut<V>(v: &mut V, node: &mut crate::BinOp)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1077-1167`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1077-L1167)*

### `visit_block_mut`

```rust
fn visit_block_mut<V>(v: &mut V, node: &mut crate::Block)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1170-1178`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1170-L1178)*

### `visit_bound_lifetimes_mut`

```rust
fn visit_bound_lifetimes_mut<V>(v: &mut V, node: &mut crate::BoundLifetimes)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1181-1192`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1181-L1192)*

### `visit_captured_param_mut`

```rust
fn visit_captured_param_mut<V>(v: &mut V, node: &mut crate::CapturedParam)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1195-1207`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1195-L1207)*

### `visit_const_param_mut`

```rust
fn visit_const_param_mut<V>(v: &mut V, node: &mut crate::ConstParam)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1210-1223`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1210-L1223)*

### `visit_constraint_mut`

```rust
fn visit_constraint_mut<V>(v: &mut V, node: &mut crate::Constraint)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1226-1239`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1226-L1239)*

### `visit_data_mut`

```rust
fn visit_data_mut<V>(v: &mut V, node: &mut crate::Data)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1242-1257`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1242-L1257)*

### `visit_data_enum_mut`

```rust
fn visit_data_enum_mut<V>(v: &mut V, node: &mut crate::DataEnum)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1260-1270`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1260-L1270)*

### `visit_data_struct_mut`

```rust
fn visit_data_struct_mut<V>(v: &mut V, node: &mut crate::DataStruct)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1273-1280`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1273-L1280)*

### `visit_data_union_mut`

```rust
fn visit_data_union_mut<V>(v: &mut V, node: &mut crate::DataUnion)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1283-1289`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1283-L1289)*

### `visit_derive_input_mut`

```rust
fn visit_derive_input_mut<V>(v: &mut V, node: &mut crate::DeriveInput)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1292-1301`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1292-L1301)*

### `visit_expr_mut`

```rust
fn visit_expr_mut<V>(v: &mut V, node: &mut crate::Expr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1304-1430`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1304-L1430)*

### `visit_expr_array_mut`

```rust
fn visit_expr_array_mut<V>(v: &mut V, node: &mut crate::ExprArray)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1433-1443`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1433-L1443)*

### `visit_expr_assign_mut`

```rust
fn visit_expr_assign_mut<V>(v: &mut V, node: &mut crate::ExprAssign)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1446-1454`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1446-L1454)*

### `visit_expr_async_mut`

```rust
fn visit_expr_async_mut<V>(v: &mut V, node: &mut crate::ExprAsync)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1457-1465`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1457-L1465)*

### `visit_expr_await_mut`

```rust
fn visit_expr_await_mut<V>(v: &mut V, node: &mut crate::ExprAwait)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1468-1476`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1468-L1476)*

### `visit_expr_binary_mut`

```rust
fn visit_expr_binary_mut<V>(v: &mut V, node: &mut crate::ExprBinary)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1479-1487`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1479-L1487)*

### `visit_expr_block_mut`

```rust
fn visit_expr_block_mut<V>(v: &mut V, node: &mut crate::ExprBlock)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1490-1499`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1490-L1499)*

### `visit_expr_break_mut`

```rust
fn visit_expr_break_mut<V>(v: &mut V, node: &mut crate::ExprBreak)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1502-1514`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1502-L1514)*

### `visit_expr_call_mut`

```rust
fn visit_expr_call_mut<V>(v: &mut V, node: &mut crate::ExprCall)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1517-1528`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1517-L1528)*

### `visit_expr_cast_mut`

```rust
fn visit_expr_cast_mut<V>(v: &mut V, node: &mut crate::ExprCast)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1531-1539`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1531-L1539)*

### `visit_expr_closure_mut`

```rust
fn visit_expr_closure_mut<V>(v: &mut V, node: &mut crate::ExprClosure)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1542-1562`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1542-L1562)*

### `visit_expr_const_mut`

```rust
fn visit_expr_const_mut<V>(v: &mut V, node: &mut crate::ExprConst)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1565-1572`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1565-L1572)*

### `visit_expr_continue_mut`

```rust
fn visit_expr_continue_mut<V>(v: &mut V, node: &mut crate::ExprContinue)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1575-1584`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1575-L1584)*

### `visit_expr_field_mut`

```rust
fn visit_expr_field_mut<V>(v: &mut V, node: &mut crate::ExprField)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1587-1595`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1587-L1595)*

### `visit_expr_for_loop_mut`

```rust
fn visit_expr_for_loop_mut<V>(v: &mut V, node: &mut crate::ExprForLoop)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1598-1611`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1598-L1611)*

### `visit_expr_group_mut`

```rust
fn visit_expr_group_mut<V>(v: &mut V, node: &mut crate::ExprGroup)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1614-1621`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1614-L1621)*

### `visit_expr_if_mut`

```rust
fn visit_expr_if_mut<V>(v: &mut V, node: &mut crate::ExprIf)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1624-1636`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1624-L1636)*

### `visit_expr_index_mut`

```rust
fn visit_expr_index_mut<V>(v: &mut V, node: &mut crate::ExprIndex)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1639-1647`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1639-L1647)*

### `visit_expr_infer_mut`

```rust
fn visit_expr_infer_mut<V>(v: &mut V, node: &mut crate::ExprInfer)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1650-1656`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1650-L1656)*

### `visit_expr_let_mut`

```rust
fn visit_expr_let_mut<V>(v: &mut V, node: &mut crate::ExprLet)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1659-1668`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1659-L1668)*

### `visit_expr_lit_mut`

```rust
fn visit_expr_lit_mut<V>(v: &mut V, node: &mut crate::ExprLit)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1671-1677`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1671-L1677)*

### `visit_expr_loop_mut`

```rust
fn visit_expr_loop_mut<V>(v: &mut V, node: &mut crate::ExprLoop)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1680-1690`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1680-L1690)*

### `visit_expr_macro_mut`

```rust
fn visit_expr_macro_mut<V>(v: &mut V, node: &mut crate::ExprMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1693-1699`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1693-L1699)*

### `visit_expr_match_mut`

```rust
fn visit_expr_match_mut<V>(v: &mut V, node: &mut crate::ExprMatch)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1702-1713`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1702-L1713)*

### `visit_expr_method_call_mut`

```rust
fn visit_expr_method_call_mut<V>(v: &mut V, node: &mut crate::ExprMethodCall)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1716-1732`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1716-L1732)*

### `visit_expr_paren_mut`

```rust
fn visit_expr_paren_mut<V>(v: &mut V, node: &mut crate::ExprParen)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1735-1742`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1735-L1742)*

### `visit_expr_path_mut`

```rust
fn visit_expr_path_mut<V>(v: &mut V, node: &mut crate::ExprPath)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1745-1754`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1745-L1754)*

### `visit_expr_range_mut`

```rust
fn visit_expr_range_mut<V>(v: &mut V, node: &mut crate::ExprRange)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1757-1769`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1757-L1769)*

### `visit_expr_raw_addr_mut`

```rust
fn visit_expr_raw_addr_mut<V>(v: &mut V, node: &mut crate::ExprRawAddr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1772-1781`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1772-L1781)*

### `visit_expr_reference_mut`

```rust
fn visit_expr_reference_mut<V>(v: &mut V, node: &mut crate::ExprReference)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1784-1792`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1784-L1792)*

### `visit_expr_repeat_mut`

```rust
fn visit_expr_repeat_mut<V>(v: &mut V, node: &mut crate::ExprRepeat)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1795-1804`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1795-L1804)*

### `visit_expr_return_mut`

```rust
fn visit_expr_return_mut<V>(v: &mut V, node: &mut crate::ExprReturn)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1807-1816`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1807-L1816)*

### `visit_expr_struct_mut`

```rust
fn visit_expr_struct_mut<V>(v: &mut V, node: &mut crate::ExprStruct)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1819-1837`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1819-L1837)*

### `visit_expr_try_mut`

```rust
fn visit_expr_try_mut<V>(v: &mut V, node: &mut crate::ExprTry)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1840-1847`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1840-L1847)*

### `visit_expr_try_block_mut`

```rust
fn visit_expr_try_block_mut<V>(v: &mut V, node: &mut crate::ExprTryBlock)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1850-1857`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1850-L1857)*

### `visit_expr_tuple_mut`

```rust
fn visit_expr_tuple_mut<V>(v: &mut V, node: &mut crate::ExprTuple)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1860-1870`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1860-L1870)*

### `visit_expr_unary_mut`

```rust
fn visit_expr_unary_mut<V>(v: &mut V, node: &mut crate::ExprUnary)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1873-1880`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1873-L1880)*

### `visit_expr_unsafe_mut`

```rust
fn visit_expr_unsafe_mut<V>(v: &mut V, node: &mut crate::ExprUnsafe)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1883-1890`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1883-L1890)*

### `visit_expr_while_mut`

```rust
fn visit_expr_while_mut<V>(v: &mut V, node: &mut crate::ExprWhile)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1893-1904`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1893-L1904)*

### `visit_expr_yield_mut`

```rust
fn visit_expr_yield_mut<V>(v: &mut V, node: &mut crate::ExprYield)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1907-1916`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1907-L1916)*

### `visit_field_mut`

```rust
fn visit_field_mut<V>(v: &mut V, node: &mut crate::Field)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1919-1931`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1919-L1931)*

### `visit_field_mutability_mut`

```rust
fn visit_field_mutability_mut<V>(v: &mut V, node: &mut crate::FieldMutability)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1934-1941`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1934-L1941)*

### `visit_field_pat_mut`

```rust
fn visit_field_pat_mut<V>(v: &mut V, node: &mut crate::FieldPat)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1944-1952`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1944-L1952)*

### `visit_field_value_mut`

```rust
fn visit_field_value_mut<V>(v: &mut V, node: &mut crate::FieldValue)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1955-1963`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1955-L1963)*

### `visit_fields_mut`

```rust
fn visit_fields_mut<V>(v: &mut V, node: &mut crate::Fields)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1966-1979`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1966-L1979)*

### `visit_fields_named_mut`

```rust
fn visit_fields_named_mut<V>(v: &mut V, node: &mut crate::FieldsNamed)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1982-1991`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1982-L1991)*

### `visit_fields_unnamed_mut`

```rust
fn visit_fields_unnamed_mut<V>(v: &mut V, node: &mut crate::FieldsUnnamed)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:1994-2003`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L1994-L2003)*

### `visit_file_mut`

```rust
fn visit_file_mut<V>(v: &mut V, node: &mut crate::File)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2006-2015`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2006-L2015)*

### `visit_fn_arg_mut`

```rust
fn visit_fn_arg_mut<V>(v: &mut V, node: &mut crate::FnArg)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2018-2030`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2018-L2030)*

### `visit_foreign_item_mut`

```rust
fn visit_foreign_item_mut<V>(v: &mut V, node: &mut crate::ForeignItem)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2033-2054`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2033-L2054)*

### `visit_foreign_item_fn_mut`

```rust
fn visit_foreign_item_fn_mut<V>(v: &mut V, node: &mut crate::ForeignItemFn)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2057-2065`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2057-L2065)*

### `visit_foreign_item_macro_mut`

```rust
fn visit_foreign_item_macro_mut<V>(v: &mut V, node: &mut crate::ForeignItemMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2068-2075`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2068-L2075)*

### `visit_foreign_item_static_mut`

```rust
fn visit_foreign_item_static_mut<V>(v: &mut V, node: &mut crate::ForeignItemStatic)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2078-2090`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2078-L2090)*

### `visit_foreign_item_type_mut`

```rust
fn visit_foreign_item_type_mut<V>(v: &mut V, node: &mut crate::ForeignItemType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2093-2103`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2093-L2103)*

### `visit_generic_argument_mut`

```rust
fn visit_generic_argument_mut<V>(v: &mut V, node: &mut crate::GenericArgument)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2106-2130`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2106-L2130)*

### `visit_generic_param_mut`

```rust
fn visit_generic_param_mut<V>(v: &mut V, node: &mut crate::GenericParam)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2133-2148`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2133-L2148)*

### `visit_generics_mut`

```rust
fn visit_generics_mut<V>(v: &mut V, node: &mut crate::Generics)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2151-2164`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2151-L2164)*

### `visit_ident_mut`

```rust
fn visit_ident_mut<V>(v: &mut V, node: &mut proc_macro2::Ident)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2165-2172`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2165-L2172)*

### `visit_impl_item_mut`

```rust
fn visit_impl_item_mut<V>(v: &mut V, node: &mut crate::ImplItem)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2175-2196`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2175-L2196)*

### `visit_impl_item_const_mut`

```rust
fn visit_impl_item_const_mut<V>(v: &mut V, node: &mut crate::ImplItemConst)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2199-2214`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2199-L2214)*

### `visit_impl_item_fn_mut`

```rust
fn visit_impl_item_fn_mut<V>(v: &mut V, node: &mut crate::ImplItemFn)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2217-2226`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2217-L2226)*

### `visit_impl_item_macro_mut`

```rust
fn visit_impl_item_macro_mut<V>(v: &mut V, node: &mut crate::ImplItemMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2229-2236`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2229-L2236)*

### `visit_impl_item_type_mut`

```rust
fn visit_impl_item_type_mut<V>(v: &mut V, node: &mut crate::ImplItemType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2239-2252`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2239-L2252)*

### `visit_impl_restriction_mut`

```rust
fn visit_impl_restriction_mut<V>(v: &mut V, node: &mut crate::ImplRestriction)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2255-2260`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2255-L2260)*

### `visit_index_mut`

```rust
fn visit_index_mut<V>(v: &mut V, node: &mut crate::Index)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2263-2269`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2263-L2269)*

### `visit_item_mut`

```rust
fn visit_item_mut<V>(v: &mut V, node: &mut crate::Item)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2272-2326`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2272-L2326)*

### `visit_item_const_mut`

```rust
fn visit_item_const_mut<V>(v: &mut V, node: &mut crate::ItemConst)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2329-2343`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2329-L2343)*

### `visit_item_enum_mut`

```rust
fn visit_item_enum_mut<V>(v: &mut V, node: &mut crate::ItemEnum)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2346-2360`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2346-L2360)*

### `visit_item_extern_crate_mut`

```rust
fn visit_item_extern_crate_mut<V>(v: &mut V, node: &mut crate::ItemExternCrate)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2363-2377`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2363-L2377)*

### `visit_item_fn_mut`

```rust
fn visit_item_fn_mut<V>(v: &mut V, node: &mut crate::ItemFn)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2380-2388`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2380-L2388)*

### `visit_item_foreign_mod_mut`

```rust
fn visit_item_foreign_mod_mut<V>(v: &mut V, node: &mut crate::ItemForeignMod)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2391-2402`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2391-L2402)*

### `visit_item_impl_mut`

```rust
fn visit_item_impl_mut<V>(v: &mut V, node: &mut crate::ItemImpl)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2405-2424`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2405-L2424)*

### `visit_item_macro_mut`

```rust
fn visit_item_macro_mut<V>(v: &mut V, node: &mut crate::ItemMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2427-2437`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2427-L2437)*

### `visit_item_mod_mut`

```rust
fn visit_item_mod_mut<V>(v: &mut V, node: &mut crate::ItemMod)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2440-2456`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2440-L2456)*

### `visit_item_static_mut`

```rust
fn visit_item_static_mut<V>(v: &mut V, node: &mut crate::ItemStatic)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2459-2473`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2459-L2473)*

### `visit_item_struct_mut`

```rust
fn visit_item_struct_mut<V>(v: &mut V, node: &mut crate::ItemStruct)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2476-2487`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2476-L2487)*

### `visit_item_trait_mut`

```rust
fn visit_item_trait_mut<V>(v: &mut V, node: &mut crate::ItemTrait)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2490-2513`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2490-L2513)*

### `visit_item_trait_alias_mut`

```rust
fn visit_item_trait_alias_mut<V>(v: &mut V, node: &mut crate::ItemTraitAlias)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2516-2531`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2516-L2531)*

### `visit_item_type_mut`

```rust
fn visit_item_type_mut<V>(v: &mut V, node: &mut crate::ItemType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2534-2546`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2534-L2546)*

### `visit_item_union_mut`

```rust
fn visit_item_union_mut<V>(v: &mut V, node: &mut crate::ItemUnion)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2549-2559`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2549-L2559)*

### `visit_item_use_mut`

```rust
fn visit_item_use_mut<V>(v: &mut V, node: &mut crate::ItemUse)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2562-2572`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2562-L2572)*

### `visit_label_mut`

```rust
fn visit_label_mut<V>(v: &mut V, node: &mut crate::Label)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2575-2581`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2575-L2581)*

### `visit_lifetime_mut`

```rust
fn visit_lifetime_mut<V>(v: &mut V, node: &mut crate::Lifetime)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2582-2588`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2582-L2588)*

### `visit_lifetime_param_mut`

```rust
fn visit_lifetime_param_mut<V>(v: &mut V, node: &mut crate::LifetimeParam)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2591-2602`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2591-L2602)*

### `visit_lit_mut`

```rust
fn visit_lit_mut<V>(v: &mut V, node: &mut crate::Lit)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2603-2636`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2603-L2636)*

### `visit_lit_bool_mut`

```rust
fn visit_lit_bool_mut<V>(v: &mut V, node: &mut crate::LitBool)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2637-2643`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2637-L2643)*

### `visit_lit_byte_mut`

```rust
fn visit_lit_byte_mut<V>(v: &mut V, node: &mut crate::LitByte)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2644-2647`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2644-L2647)*

### `visit_lit_byte_str_mut`

```rust
fn visit_lit_byte_str_mut<V>(v: &mut V, node: &mut crate::LitByteStr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2648-2651`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2648-L2651)*

### `visit_lit_cstr_mut`

```rust
fn visit_lit_cstr_mut<V>(v: &mut V, node: &mut crate::LitCStr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2652-2655`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2652-L2655)*

### `visit_lit_char_mut`

```rust
fn visit_lit_char_mut<V>(v: &mut V, node: &mut crate::LitChar)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2656-2659`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2656-L2659)*

### `visit_lit_float_mut`

```rust
fn visit_lit_float_mut<V>(v: &mut V, node: &mut crate::LitFloat)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2660-2663`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2660-L2663)*

### `visit_lit_int_mut`

```rust
fn visit_lit_int_mut<V>(v: &mut V, node: &mut crate::LitInt)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2664-2667`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2664-L2667)*

### `visit_lit_str_mut`

```rust
fn visit_lit_str_mut<V>(v: &mut V, node: &mut crate::LitStr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2668-2671`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2668-L2671)*

### `visit_local_mut`

```rust
fn visit_local_mut<V>(v: &mut V, node: &mut crate::Local)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2674-2685`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2674-L2685)*

### `visit_local_init_mut`

```rust
fn visit_local_init_mut<V>(v: &mut V, node: &mut crate::LocalInit)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2688-2698`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2688-L2698)*

### `visit_macro_mut`

```rust
fn visit_macro_mut<V>(v: &mut V, node: &mut crate::Macro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2701-2709`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2701-L2709)*

### `visit_macro_delimiter_mut`

```rust
fn visit_macro_delimiter_mut<V>(v: &mut V, node: &mut crate::MacroDelimiter)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2712-2727`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2712-L2727)*

### `visit_member_mut`

```rust
fn visit_member_mut<V>(v: &mut V, node: &mut crate::Member)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2730-2742`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2730-L2742)*

### `visit_meta_mut`

```rust
fn visit_meta_mut<V>(v: &mut V, node: &mut crate::Meta)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2745-2760`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2745-L2760)*

### `visit_meta_list_mut`

```rust
fn visit_meta_list_mut<V>(v: &mut V, node: &mut crate::MetaList)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2763-2770`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2763-L2770)*

### `visit_meta_name_value_mut`

```rust
fn visit_meta_name_value_mut<V>(v: &mut V, node: &mut crate::MetaNameValue)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2773-2780`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2773-L2780)*

### `visit_parenthesized_generic_arguments_mut`

```rust
fn visit_parenthesized_generic_arguments_mut<V>(v: &mut V, node: &mut crate::ParenthesizedGenericArguments)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2783-2796`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2783-L2796)*

### `visit_pat_mut`

```rust
fn visit_pat_mut<V>(v: &mut V, node: &mut crate::Pat)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2799-2856`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2799-L2856)*

### `visit_pat_ident_mut`

```rust
fn visit_pat_ident_mut<V>(v: &mut V, node: &mut crate::PatIdent)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2859-2871`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2859-L2871)*

### `visit_pat_or_mut`

```rust
fn visit_pat_or_mut<V>(v: &mut V, node: &mut crate::PatOr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2874-2884`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2874-L2884)*

### `visit_pat_paren_mut`

```rust
fn visit_pat_paren_mut<V>(v: &mut V, node: &mut crate::PatParen)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2887-2894`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2887-L2894)*

### `visit_pat_reference_mut`

```rust
fn visit_pat_reference_mut<V>(v: &mut V, node: &mut crate::PatReference)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2897-2905`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2897-L2905)*

### `visit_pat_rest_mut`

```rust
fn visit_pat_rest_mut<V>(v: &mut V, node: &mut crate::PatRest)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2908-2914`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2908-L2914)*

### `visit_pat_slice_mut`

```rust
fn visit_pat_slice_mut<V>(v: &mut V, node: &mut crate::PatSlice)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2917-2927`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2917-L2927)*

### `visit_pat_struct_mut`

```rust
fn visit_pat_struct_mut<V>(v: &mut V, node: &mut crate::PatStruct)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2930-2947`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2930-L2947)*

### `visit_pat_tuple_mut`

```rust
fn visit_pat_tuple_mut<V>(v: &mut V, node: &mut crate::PatTuple)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2950-2960`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2950-L2960)*

### `visit_pat_tuple_struct_mut`

```rust
fn visit_pat_tuple_struct_mut<V>(v: &mut V, node: &mut crate::PatTupleStruct)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2963-2977`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2963-L2977)*

### `visit_pat_type_mut`

```rust
fn visit_pat_type_mut<V>(v: &mut V, node: &mut crate::PatType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2980-2988`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2980-L2988)*

### `visit_pat_wild_mut`

```rust
fn visit_pat_wild_mut<V>(v: &mut V, node: &mut crate::PatWild)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:2991-2997`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L2991-L2997)*

### `visit_path_mut`

```rust
fn visit_path_mut<V>(v: &mut V, node: &mut crate::Path)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3000-3009`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3000-L3009)*

### `visit_path_arguments_mut`

```rust
fn visit_path_arguments_mut<V>(v: &mut V, node: &mut crate::PathArguments)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3012-3025`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3012-L3025)*

### `visit_path_segment_mut`

```rust
fn visit_path_segment_mut<V>(v: &mut V, node: &mut crate::PathSegment)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3028-3034`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3028-L3034)*

### `visit_pointer_mutability_mut`

```rust
fn visit_pointer_mutability_mut<V>(v: &mut V, node: &mut crate::PointerMutability)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3037-3049`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3037-L3049)*

### `visit_precise_capture_mut`

```rust
fn visit_precise_capture_mut<V>(v: &mut V, node: &mut crate::PreciseCapture)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3052-3063`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3052-L3063)*

### `visit_predicate_lifetime_mut`

```rust
fn visit_predicate_lifetime_mut<V>(v: &mut V, node: &mut crate::PredicateLifetime)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3066-3076`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3066-L3076)*

### `visit_predicate_type_mut`

```rust
fn visit_predicate_type_mut<V>(v: &mut V, node: &mut crate::PredicateType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3079-3092`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3079-L3092)*

### `visit_qself_mut`

```rust
fn visit_qself_mut<V>(v: &mut V, node: &mut crate::QSelf)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3095-3104`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3095-L3104)*

### `visit_range_limits_mut`

```rust
fn visit_range_limits_mut<V>(v: &mut V, node: &mut crate::RangeLimits)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3107-3119`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3107-L3119)*

### `visit_receiver_mut`

```rust
fn visit_receiver_mut<V>(v: &mut V, node: &mut crate::Receiver)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3122-3137`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3122-L3137)*

### `visit_return_type_mut`

```rust
fn visit_return_type_mut<V>(v: &mut V, node: &mut crate::ReturnType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3140-3151`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3140-L3151)*

### `visit_signature_mut`

```rust
fn visit_signature_mut<V>(v: &mut V, node: &mut crate::Signature)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3154-3176`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3154-L3176)*

### `visit_span_mut`

```rust
fn visit_span_mut<V>(v: &mut V, node: &mut proc_macro2::Span)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3177-3180`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3177-L3180)*

### `visit_static_mutability_mut`

```rust
fn visit_static_mutability_mut<V>(v: &mut V, node: &mut crate::StaticMutability)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3183-3193`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3183-L3193)*

### `visit_stmt_mut`

```rust
fn visit_stmt_mut<V>(v: &mut V, node: &mut crate::Stmt)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3196-3215`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3196-L3215)*

### `visit_stmt_macro_mut`

```rust
fn visit_stmt_macro_mut<V>(v: &mut V, node: &mut crate::StmtMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3218-3225`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3218-L3225)*

### `visit_trait_bound_mut`

```rust
fn visit_trait_bound_mut<V>(v: &mut V, node: &mut crate::TraitBound)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3228-3238`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3228-L3238)*

### `visit_trait_bound_modifier_mut`

```rust
fn visit_trait_bound_modifier_mut<V>(v: &mut V, node: &mut crate::TraitBoundModifier)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3241-3251`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3241-L3251)*

### `visit_trait_item_mut`

```rust
fn visit_trait_item_mut<V>(v: &mut V, node: &mut crate::TraitItem)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3254-3275`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3254-L3275)*

### `visit_trait_item_const_mut`

```rust
fn visit_trait_item_const_mut<V>(v: &mut V, node: &mut crate::TraitItemConst)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3278-3293`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3278-L3293)*

### `visit_trait_item_fn_mut`

```rust
fn visit_trait_item_fn_mut<V>(v: &mut V, node: &mut crate::TraitItemFn)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3296-3306`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3296-L3306)*

### `visit_trait_item_macro_mut`

```rust
fn visit_trait_item_macro_mut<V>(v: &mut V, node: &mut crate::TraitItemMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3309-3316`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3309-L3316)*

### `visit_trait_item_type_mut`

```rust
fn visit_trait_item_type_mut<V>(v: &mut V, node: &mut crate::TraitItemType)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3319-3337`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3319-L3337)*

### `visit_type_mut`

```rust
fn visit_type_mut<V>(v: &mut V, node: &mut crate::Type)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3340-3391`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3340-L3391)*

### `visit_type_array_mut`

```rust
fn visit_type_array_mut<V>(v: &mut V, node: &mut crate::TypeArray)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3394-3402`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3394-L3402)*

### `visit_type_bare_fn_mut`

```rust
fn visit_type_bare_fn_mut<V>(v: &mut V, node: &mut crate::TypeBareFn)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3405-3426`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3405-L3426)*

### `visit_type_group_mut`

```rust
fn visit_type_group_mut<V>(v: &mut V, node: &mut crate::TypeGroup)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3429-3435`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3429-L3435)*

### `visit_type_impl_trait_mut`

```rust
fn visit_type_impl_trait_mut<V>(v: &mut V, node: &mut crate::TypeImplTrait)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3438-3447`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3438-L3447)*

### `visit_type_infer_mut`

```rust
fn visit_type_infer_mut<V>(v: &mut V, node: &mut crate::TypeInfer)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3450-3455`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3450-L3455)*

### `visit_type_macro_mut`

```rust
fn visit_type_macro_mut<V>(v: &mut V, node: &mut crate::TypeMacro)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3458-3463`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3458-L3463)*

### `visit_type_never_mut`

```rust
fn visit_type_never_mut<V>(v: &mut V, node: &mut crate::TypeNever)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3466-3471`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3466-L3471)*

### `visit_type_param_mut`

```rust
fn visit_type_param_mut<V>(v: &mut V, node: &mut crate::TypeParam)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3474-3489`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3474-L3489)*

### `visit_type_param_bound_mut`

```rust
fn visit_type_param_bound_mut<V>(v: &mut V, node: &mut crate::TypeParamBound)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3492-3510`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3492-L3510)*

### `visit_type_paren_mut`

```rust
fn visit_type_paren_mut<V>(v: &mut V, node: &mut crate::TypeParen)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3513-3519`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3513-L3519)*

### `visit_type_path_mut`

```rust
fn visit_type_path_mut<V>(v: &mut V, node: &mut crate::TypePath)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3522-3530`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3522-L3530)*

### `visit_type_ptr_mut`

```rust
fn visit_type_ptr_mut<V>(v: &mut V, node: &mut crate::TypePtr)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3533-3541`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3533-L3541)*

### `visit_type_reference_mut`

```rust
fn visit_type_reference_mut<V>(v: &mut V, node: &mut crate::TypeReference)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3544-3554`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3544-L3554)*

### `visit_type_slice_mut`

```rust
fn visit_type_slice_mut<V>(v: &mut V, node: &mut crate::TypeSlice)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3557-3563`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3557-L3563)*

### `visit_type_trait_object_mut`

```rust
fn visit_type_trait_object_mut<V>(v: &mut V, node: &mut crate::TypeTraitObject)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3566-3575`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3566-L3575)*

### `visit_type_tuple_mut`

```rust
fn visit_type_tuple_mut<V>(v: &mut V, node: &mut crate::TypeTuple)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3578-3587`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3578-L3587)*

### `visit_un_op_mut`

```rust
fn visit_un_op_mut<V>(v: &mut V, node: &mut crate::UnOp)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3590-3605`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3590-L3605)*

### `visit_use_glob_mut`

```rust
fn visit_use_glob_mut<V>(v: &mut V, node: &mut crate::UseGlob)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3608-3613`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3608-L3613)*

### `visit_use_group_mut`

```rust
fn visit_use_group_mut<V>(v: &mut V, node: &mut crate::UseGroup)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3616-3625`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3616-L3625)*

### `visit_use_name_mut`

```rust
fn visit_use_name_mut<V>(v: &mut V, node: &mut crate::UseName)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3628-3633`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3628-L3633)*

### `visit_use_path_mut`

```rust
fn visit_use_path_mut<V>(v: &mut V, node: &mut crate::UsePath)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3636-3643`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3636-L3643)*

### `visit_use_rename_mut`

```rust
fn visit_use_rename_mut<V>(v: &mut V, node: &mut crate::UseRename)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3646-3653`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3646-L3653)*

### `visit_use_tree_mut`

```rust
fn visit_use_tree_mut<V>(v: &mut V, node: &mut crate::UseTree)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3656-3677`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3656-L3677)*

### `visit_variadic_mut`

```rust
fn visit_variadic_mut<V>(v: &mut V, node: &mut crate::Variadic)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3680-3691`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3680-L3691)*

### `visit_variant_mut`

```rust
fn visit_variant_mut<V>(v: &mut V, node: &mut crate::Variant)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3694-3705`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3694-L3705)*

### `visit_vis_restricted_mut`

```rust
fn visit_vis_restricted_mut<V>(v: &mut V, node: &mut crate::VisRestricted)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3708-3716`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3708-L3716)*

### `visit_visibility_mut`

```rust
fn visit_visibility_mut<V>(v: &mut V, node: &mut crate::Visibility)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3719-3732`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3719-L3732)*

### `visit_where_clause_mut`

```rust
fn visit_where_clause_mut<V>(v: &mut V, node: &mut crate::WhereClause)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3735-3744`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3735-L3744)*

### `visit_where_predicate_mut`

```rust
fn visit_where_predicate_mut<V>(v: &mut V, node: &mut crate::WherePredicate)
where
    V: VisitMut + ?Sized
```

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:3747-3759`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L3747-L3759)*

## Macros

### `full!`

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:9-13`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L9-L13)*

### `skip!`

*Defined in [`syn-2.0.111/src/gen/visit_mut.rs:20-22`](../../../../.source_1765210505/syn-2.0.111/src/gen/visit_mut.rs#L20-L22)*

