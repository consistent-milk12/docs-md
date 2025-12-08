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

## Traits

### `VisitMut`

```rust
trait VisitMut { ... }
```

Syntax tree traversal to mutate an exclusive borrow of a syntax tree in
place.

See the [module documentation] for details.


#### Required Methods

- `fn visit_abi_mut(self: &mut Self, i: &mut crate::Abi)`

- `fn visit_angle_bracketed_generic_arguments_mut(self: &mut Self, i: &mut crate::AngleBracketedGenericArguments)`

- `fn visit_arm_mut(self: &mut Self, i: &mut crate::Arm)`

- `fn visit_assoc_const_mut(self: &mut Self, i: &mut crate::AssocConst)`

- `fn visit_assoc_type_mut(self: &mut Self, i: &mut crate::AssocType)`

- `fn visit_attr_style_mut(self: &mut Self, i: &mut crate::AttrStyle)`

- `fn visit_attribute_mut(self: &mut Self, i: &mut crate::Attribute)`

- `fn visit_attributes_mut(self: &mut Self, i: &mut Vec<crate::Attribute>)`

- `fn visit_bare_fn_arg_mut(self: &mut Self, i: &mut crate::BareFnArg)`

- `fn visit_bare_variadic_mut(self: &mut Self, i: &mut crate::BareVariadic)`

- `fn visit_bin_op_mut(self: &mut Self, i: &mut crate::BinOp)`

- `fn visit_block_mut(self: &mut Self, i: &mut crate::Block)`

- `fn visit_bound_lifetimes_mut(self: &mut Self, i: &mut crate::BoundLifetimes)`

- `fn visit_captured_param_mut(self: &mut Self, i: &mut crate::CapturedParam)`

- `fn visit_const_param_mut(self: &mut Self, i: &mut crate::ConstParam)`

- `fn visit_constraint_mut(self: &mut Self, i: &mut crate::Constraint)`

- `fn visit_data_mut(self: &mut Self, i: &mut crate::Data)`

- `fn visit_data_enum_mut(self: &mut Self, i: &mut crate::DataEnum)`

- `fn visit_data_struct_mut(self: &mut Self, i: &mut crate::DataStruct)`

- `fn visit_data_union_mut(self: &mut Self, i: &mut crate::DataUnion)`

- `fn visit_derive_input_mut(self: &mut Self, i: &mut crate::DeriveInput)`

- `fn visit_expr_mut(self: &mut Self, i: &mut crate::Expr)`

- `fn visit_expr_array_mut(self: &mut Self, i: &mut crate::ExprArray)`

- `fn visit_expr_assign_mut(self: &mut Self, i: &mut crate::ExprAssign)`

- `fn visit_expr_async_mut(self: &mut Self, i: &mut crate::ExprAsync)`

- `fn visit_expr_await_mut(self: &mut Self, i: &mut crate::ExprAwait)`

- `fn visit_expr_binary_mut(self: &mut Self, i: &mut crate::ExprBinary)`

- `fn visit_expr_block_mut(self: &mut Self, i: &mut crate::ExprBlock)`

- `fn visit_expr_break_mut(self: &mut Self, i: &mut crate::ExprBreak)`

- `fn visit_expr_call_mut(self: &mut Self, i: &mut crate::ExprCall)`

- `fn visit_expr_cast_mut(self: &mut Self, i: &mut crate::ExprCast)`

- `fn visit_expr_closure_mut(self: &mut Self, i: &mut crate::ExprClosure)`

- `fn visit_expr_const_mut(self: &mut Self, i: &mut crate::ExprConst)`

- `fn visit_expr_continue_mut(self: &mut Self, i: &mut crate::ExprContinue)`

- `fn visit_expr_field_mut(self: &mut Self, i: &mut crate::ExprField)`

- `fn visit_expr_for_loop_mut(self: &mut Self, i: &mut crate::ExprForLoop)`

- `fn visit_expr_group_mut(self: &mut Self, i: &mut crate::ExprGroup)`

- `fn visit_expr_if_mut(self: &mut Self, i: &mut crate::ExprIf)`

- `fn visit_expr_index_mut(self: &mut Self, i: &mut crate::ExprIndex)`

- `fn visit_expr_infer_mut(self: &mut Self, i: &mut crate::ExprInfer)`

- `fn visit_expr_let_mut(self: &mut Self, i: &mut crate::ExprLet)`

- `fn visit_expr_lit_mut(self: &mut Self, i: &mut crate::ExprLit)`

- `fn visit_expr_loop_mut(self: &mut Self, i: &mut crate::ExprLoop)`

- `fn visit_expr_macro_mut(self: &mut Self, i: &mut crate::ExprMacro)`

- `fn visit_expr_match_mut(self: &mut Self, i: &mut crate::ExprMatch)`

- `fn visit_expr_method_call_mut(self: &mut Self, i: &mut crate::ExprMethodCall)`

- `fn visit_expr_paren_mut(self: &mut Self, i: &mut crate::ExprParen)`

- `fn visit_expr_path_mut(self: &mut Self, i: &mut crate::ExprPath)`

- `fn visit_expr_range_mut(self: &mut Self, i: &mut crate::ExprRange)`

- `fn visit_expr_raw_addr_mut(self: &mut Self, i: &mut crate::ExprRawAddr)`

- `fn visit_expr_reference_mut(self: &mut Self, i: &mut crate::ExprReference)`

- `fn visit_expr_repeat_mut(self: &mut Self, i: &mut crate::ExprRepeat)`

- `fn visit_expr_return_mut(self: &mut Self, i: &mut crate::ExprReturn)`

- `fn visit_expr_struct_mut(self: &mut Self, i: &mut crate::ExprStruct)`

- `fn visit_expr_try_mut(self: &mut Self, i: &mut crate::ExprTry)`

- `fn visit_expr_try_block_mut(self: &mut Self, i: &mut crate::ExprTryBlock)`

- `fn visit_expr_tuple_mut(self: &mut Self, i: &mut crate::ExprTuple)`

- `fn visit_expr_unary_mut(self: &mut Self, i: &mut crate::ExprUnary)`

- `fn visit_expr_unsafe_mut(self: &mut Self, i: &mut crate::ExprUnsafe)`

- `fn visit_expr_while_mut(self: &mut Self, i: &mut crate::ExprWhile)`

- `fn visit_expr_yield_mut(self: &mut Self, i: &mut crate::ExprYield)`

- `fn visit_field_mut(self: &mut Self, i: &mut crate::Field)`

- `fn visit_field_mutability_mut(self: &mut Self, i: &mut crate::FieldMutability)`

- `fn visit_field_pat_mut(self: &mut Self, i: &mut crate::FieldPat)`

- `fn visit_field_value_mut(self: &mut Self, i: &mut crate::FieldValue)`

- `fn visit_fields_mut(self: &mut Self, i: &mut crate::Fields)`

- `fn visit_fields_named_mut(self: &mut Self, i: &mut crate::FieldsNamed)`

- `fn visit_fields_unnamed_mut(self: &mut Self, i: &mut crate::FieldsUnnamed)`

- `fn visit_file_mut(self: &mut Self, i: &mut crate::File)`

- `fn visit_fn_arg_mut(self: &mut Self, i: &mut crate::FnArg)`

- `fn visit_foreign_item_mut(self: &mut Self, i: &mut crate::ForeignItem)`

- `fn visit_foreign_item_fn_mut(self: &mut Self, i: &mut crate::ForeignItemFn)`

- `fn visit_foreign_item_macro_mut(self: &mut Self, i: &mut crate::ForeignItemMacro)`

- `fn visit_foreign_item_static_mut(self: &mut Self, i: &mut crate::ForeignItemStatic)`

- `fn visit_foreign_item_type_mut(self: &mut Self, i: &mut crate::ForeignItemType)`

- `fn visit_generic_argument_mut(self: &mut Self, i: &mut crate::GenericArgument)`

- `fn visit_generic_param_mut(self: &mut Self, i: &mut crate::GenericParam)`

- `fn visit_generics_mut(self: &mut Self, i: &mut crate::Generics)`

- `fn visit_ident_mut(self: &mut Self, i: &mut proc_macro2::Ident)`

- `fn visit_impl_item_mut(self: &mut Self, i: &mut crate::ImplItem)`

- `fn visit_impl_item_const_mut(self: &mut Self, i: &mut crate::ImplItemConst)`

- `fn visit_impl_item_fn_mut(self: &mut Self, i: &mut crate::ImplItemFn)`

- `fn visit_impl_item_macro_mut(self: &mut Self, i: &mut crate::ImplItemMacro)`

- `fn visit_impl_item_type_mut(self: &mut Self, i: &mut crate::ImplItemType)`

- `fn visit_impl_restriction_mut(self: &mut Self, i: &mut crate::ImplRestriction)`

- `fn visit_index_mut(self: &mut Self, i: &mut crate::Index)`

- `fn visit_item_mut(self: &mut Self, i: &mut crate::Item)`

- `fn visit_item_const_mut(self: &mut Self, i: &mut crate::ItemConst)`

- `fn visit_item_enum_mut(self: &mut Self, i: &mut crate::ItemEnum)`

- `fn visit_item_extern_crate_mut(self: &mut Self, i: &mut crate::ItemExternCrate)`

- `fn visit_item_fn_mut(self: &mut Self, i: &mut crate::ItemFn)`

- `fn visit_item_foreign_mod_mut(self: &mut Self, i: &mut crate::ItemForeignMod)`

- `fn visit_item_impl_mut(self: &mut Self, i: &mut crate::ItemImpl)`

- `fn visit_item_macro_mut(self: &mut Self, i: &mut crate::ItemMacro)`

- `fn visit_item_mod_mut(self: &mut Self, i: &mut crate::ItemMod)`

- `fn visit_item_static_mut(self: &mut Self, i: &mut crate::ItemStatic)`

- `fn visit_item_struct_mut(self: &mut Self, i: &mut crate::ItemStruct)`

- `fn visit_item_trait_mut(self: &mut Self, i: &mut crate::ItemTrait)`

- `fn visit_item_trait_alias_mut(self: &mut Self, i: &mut crate::ItemTraitAlias)`

- `fn visit_item_type_mut(self: &mut Self, i: &mut crate::ItemType)`

- `fn visit_item_union_mut(self: &mut Self, i: &mut crate::ItemUnion)`

- `fn visit_item_use_mut(self: &mut Self, i: &mut crate::ItemUse)`

- `fn visit_label_mut(self: &mut Self, i: &mut crate::Label)`

- `fn visit_lifetime_mut(self: &mut Self, i: &mut crate::Lifetime)`

- `fn visit_lifetime_param_mut(self: &mut Self, i: &mut crate::LifetimeParam)`

- `fn visit_lit_mut(self: &mut Self, i: &mut crate::Lit)`

- `fn visit_lit_bool_mut(self: &mut Self, i: &mut crate::LitBool)`

- `fn visit_lit_byte_mut(self: &mut Self, i: &mut crate::LitByte)`

- `fn visit_lit_byte_str_mut(self: &mut Self, i: &mut crate::LitByteStr)`

- `fn visit_lit_cstr_mut(self: &mut Self, i: &mut crate::LitCStr)`

- `fn visit_lit_char_mut(self: &mut Self, i: &mut crate::LitChar)`

- `fn visit_lit_float_mut(self: &mut Self, i: &mut crate::LitFloat)`

- `fn visit_lit_int_mut(self: &mut Self, i: &mut crate::LitInt)`

- `fn visit_lit_str_mut(self: &mut Self, i: &mut crate::LitStr)`

- `fn visit_local_mut(self: &mut Self, i: &mut crate::Local)`

- `fn visit_local_init_mut(self: &mut Self, i: &mut crate::LocalInit)`

- `fn visit_macro_mut(self: &mut Self, i: &mut crate::Macro)`

- `fn visit_macro_delimiter_mut(self: &mut Self, i: &mut crate::MacroDelimiter)`

- `fn visit_member_mut(self: &mut Self, i: &mut crate::Member)`

- `fn visit_meta_mut(self: &mut Self, i: &mut crate::Meta)`

- `fn visit_meta_list_mut(self: &mut Self, i: &mut crate::MetaList)`

- `fn visit_meta_name_value_mut(self: &mut Self, i: &mut crate::MetaNameValue)`

- `fn visit_parenthesized_generic_arguments_mut(self: &mut Self, i: &mut crate::ParenthesizedGenericArguments)`

- `fn visit_pat_mut(self: &mut Self, i: &mut crate::Pat)`

- `fn visit_pat_ident_mut(self: &mut Self, i: &mut crate::PatIdent)`

- `fn visit_pat_or_mut(self: &mut Self, i: &mut crate::PatOr)`

- `fn visit_pat_paren_mut(self: &mut Self, i: &mut crate::PatParen)`

- `fn visit_pat_reference_mut(self: &mut Self, i: &mut crate::PatReference)`

- `fn visit_pat_rest_mut(self: &mut Self, i: &mut crate::PatRest)`

- `fn visit_pat_slice_mut(self: &mut Self, i: &mut crate::PatSlice)`

- `fn visit_pat_struct_mut(self: &mut Self, i: &mut crate::PatStruct)`

- `fn visit_pat_tuple_mut(self: &mut Self, i: &mut crate::PatTuple)`

- `fn visit_pat_tuple_struct_mut(self: &mut Self, i: &mut crate::PatTupleStruct)`

- `fn visit_pat_type_mut(self: &mut Self, i: &mut crate::PatType)`

- `fn visit_pat_wild_mut(self: &mut Self, i: &mut crate::PatWild)`

- `fn visit_path_mut(self: &mut Self, i: &mut crate::Path)`

- `fn visit_path_arguments_mut(self: &mut Self, i: &mut crate::PathArguments)`

- `fn visit_path_segment_mut(self: &mut Self, i: &mut crate::PathSegment)`

- `fn visit_pointer_mutability_mut(self: &mut Self, i: &mut crate::PointerMutability)`

- `fn visit_precise_capture_mut(self: &mut Self, i: &mut crate::PreciseCapture)`

- `fn visit_predicate_lifetime_mut(self: &mut Self, i: &mut crate::PredicateLifetime)`

- `fn visit_predicate_type_mut(self: &mut Self, i: &mut crate::PredicateType)`

- `fn visit_qself_mut(self: &mut Self, i: &mut crate::QSelf)`

- `fn visit_range_limits_mut(self: &mut Self, i: &mut crate::RangeLimits)`

- `fn visit_receiver_mut(self: &mut Self, i: &mut crate::Receiver)`

- `fn visit_return_type_mut(self: &mut Self, i: &mut crate::ReturnType)`

- `fn visit_signature_mut(self: &mut Self, i: &mut crate::Signature)`

- `fn visit_span_mut(self: &mut Self, i: &mut proc_macro2::Span)`

- `fn visit_static_mutability_mut(self: &mut Self, i: &mut crate::StaticMutability)`

- `fn visit_stmt_mut(self: &mut Self, i: &mut crate::Stmt)`

- `fn visit_stmt_macro_mut(self: &mut Self, i: &mut crate::StmtMacro)`

- `fn visit_token_stream_mut(self: &mut Self, i: &mut proc_macro2::TokenStream)`

- `fn visit_trait_bound_mut(self: &mut Self, i: &mut crate::TraitBound)`

- `fn visit_trait_bound_modifier_mut(self: &mut Self, i: &mut crate::TraitBoundModifier)`

- `fn visit_trait_item_mut(self: &mut Self, i: &mut crate::TraitItem)`

- `fn visit_trait_item_const_mut(self: &mut Self, i: &mut crate::TraitItemConst)`

- `fn visit_trait_item_fn_mut(self: &mut Self, i: &mut crate::TraitItemFn)`

- `fn visit_trait_item_macro_mut(self: &mut Self, i: &mut crate::TraitItemMacro)`

- `fn visit_trait_item_type_mut(self: &mut Self, i: &mut crate::TraitItemType)`

- `fn visit_type_mut(self: &mut Self, i: &mut crate::Type)`

- `fn visit_type_array_mut(self: &mut Self, i: &mut crate::TypeArray)`

- `fn visit_type_bare_fn_mut(self: &mut Self, i: &mut crate::TypeBareFn)`

- `fn visit_type_group_mut(self: &mut Self, i: &mut crate::TypeGroup)`

- `fn visit_type_impl_trait_mut(self: &mut Self, i: &mut crate::TypeImplTrait)`

- `fn visit_type_infer_mut(self: &mut Self, i: &mut crate::TypeInfer)`

- `fn visit_type_macro_mut(self: &mut Self, i: &mut crate::TypeMacro)`

- `fn visit_type_never_mut(self: &mut Self, i: &mut crate::TypeNever)`

- `fn visit_type_param_mut(self: &mut Self, i: &mut crate::TypeParam)`

- `fn visit_type_param_bound_mut(self: &mut Self, i: &mut crate::TypeParamBound)`

- `fn visit_type_paren_mut(self: &mut Self, i: &mut crate::TypeParen)`

- `fn visit_type_path_mut(self: &mut Self, i: &mut crate::TypePath)`

- `fn visit_type_ptr_mut(self: &mut Self, i: &mut crate::TypePtr)`

- `fn visit_type_reference_mut(self: &mut Self, i: &mut crate::TypeReference)`

- `fn visit_type_slice_mut(self: &mut Self, i: &mut crate::TypeSlice)`

- `fn visit_type_trait_object_mut(self: &mut Self, i: &mut crate::TypeTraitObject)`

- `fn visit_type_tuple_mut(self: &mut Self, i: &mut crate::TypeTuple)`

- `fn visit_un_op_mut(self: &mut Self, i: &mut crate::UnOp)`

- `fn visit_use_glob_mut(self: &mut Self, i: &mut crate::UseGlob)`

- `fn visit_use_group_mut(self: &mut Self, i: &mut crate::UseGroup)`

- `fn visit_use_name_mut(self: &mut Self, i: &mut crate::UseName)`

- `fn visit_use_path_mut(self: &mut Self, i: &mut crate::UsePath)`

- `fn visit_use_rename_mut(self: &mut Self, i: &mut crate::UseRename)`

- `fn visit_use_tree_mut(self: &mut Self, i: &mut crate::UseTree)`

- `fn visit_variadic_mut(self: &mut Self, i: &mut crate::Variadic)`

- `fn visit_variant_mut(self: &mut Self, i: &mut crate::Variant)`

- `fn visit_vis_restricted_mut(self: &mut Self, i: &mut crate::VisRestricted)`

- `fn visit_visibility_mut(self: &mut Self, i: &mut crate::Visibility)`

- `fn visit_where_clause_mut(self: &mut Self, i: &mut crate::WhereClause)`

- `fn visit_where_predicate_mut(self: &mut Self, i: &mut crate::WherePredicate)`

## Functions

### `visit_abi_mut`

```rust
fn visit_abi_mut<V>(v: &mut V, node: &mut crate::Abi)
where
    V: VisitMut + ?Sized
```

### `visit_angle_bracketed_generic_arguments_mut`

```rust
fn visit_angle_bracketed_generic_arguments_mut<V>(v: &mut V, node: &mut crate::AngleBracketedGenericArguments)
where
    V: VisitMut + ?Sized
```

### `visit_arm_mut`

```rust
fn visit_arm_mut<V>(v: &mut V, node: &mut crate::Arm)
where
    V: VisitMut + ?Sized
```

### `visit_assoc_const_mut`

```rust
fn visit_assoc_const_mut<V>(v: &mut V, node: &mut crate::AssocConst)
where
    V: VisitMut + ?Sized
```

### `visit_assoc_type_mut`

```rust
fn visit_assoc_type_mut<V>(v: &mut V, node: &mut crate::AssocType)
where
    V: VisitMut + ?Sized
```

### `visit_attr_style_mut`

```rust
fn visit_attr_style_mut<V>(v: &mut V, node: &mut crate::AttrStyle)
where
    V: VisitMut + ?Sized
```

### `visit_attribute_mut`

```rust
fn visit_attribute_mut<V>(v: &mut V, node: &mut crate::Attribute)
where
    V: VisitMut + ?Sized
```

### `visit_bare_fn_arg_mut`

```rust
fn visit_bare_fn_arg_mut<V>(v: &mut V, node: &mut crate::BareFnArg)
where
    V: VisitMut + ?Sized
```

### `visit_bare_variadic_mut`

```rust
fn visit_bare_variadic_mut<V>(v: &mut V, node: &mut crate::BareVariadic)
where
    V: VisitMut + ?Sized
```

### `visit_bin_op_mut`

```rust
fn visit_bin_op_mut<V>(v: &mut V, node: &mut crate::BinOp)
where
    V: VisitMut + ?Sized
```

### `visit_block_mut`

```rust
fn visit_block_mut<V>(v: &mut V, node: &mut crate::Block)
where
    V: VisitMut + ?Sized
```

### `visit_bound_lifetimes_mut`

```rust
fn visit_bound_lifetimes_mut<V>(v: &mut V, node: &mut crate::BoundLifetimes)
where
    V: VisitMut + ?Sized
```

### `visit_captured_param_mut`

```rust
fn visit_captured_param_mut<V>(v: &mut V, node: &mut crate::CapturedParam)
where
    V: VisitMut + ?Sized
```

### `visit_const_param_mut`

```rust
fn visit_const_param_mut<V>(v: &mut V, node: &mut crate::ConstParam)
where
    V: VisitMut + ?Sized
```

### `visit_constraint_mut`

```rust
fn visit_constraint_mut<V>(v: &mut V, node: &mut crate::Constraint)
where
    V: VisitMut + ?Sized
```

### `visit_data_mut`

```rust
fn visit_data_mut<V>(v: &mut V, node: &mut crate::Data)
where
    V: VisitMut + ?Sized
```

### `visit_data_enum_mut`

```rust
fn visit_data_enum_mut<V>(v: &mut V, node: &mut crate::DataEnum)
where
    V: VisitMut + ?Sized
```

### `visit_data_struct_mut`

```rust
fn visit_data_struct_mut<V>(v: &mut V, node: &mut crate::DataStruct)
where
    V: VisitMut + ?Sized
```

### `visit_data_union_mut`

```rust
fn visit_data_union_mut<V>(v: &mut V, node: &mut crate::DataUnion)
where
    V: VisitMut + ?Sized
```

### `visit_derive_input_mut`

```rust
fn visit_derive_input_mut<V>(v: &mut V, node: &mut crate::DeriveInput)
where
    V: VisitMut + ?Sized
```

### `visit_expr_mut`

```rust
fn visit_expr_mut<V>(v: &mut V, node: &mut crate::Expr)
where
    V: VisitMut + ?Sized
```

### `visit_expr_array_mut`

```rust
fn visit_expr_array_mut<V>(v: &mut V, node: &mut crate::ExprArray)
where
    V: VisitMut + ?Sized
```

### `visit_expr_assign_mut`

```rust
fn visit_expr_assign_mut<V>(v: &mut V, node: &mut crate::ExprAssign)
where
    V: VisitMut + ?Sized
```

### `visit_expr_async_mut`

```rust
fn visit_expr_async_mut<V>(v: &mut V, node: &mut crate::ExprAsync)
where
    V: VisitMut + ?Sized
```

### `visit_expr_await_mut`

```rust
fn visit_expr_await_mut<V>(v: &mut V, node: &mut crate::ExprAwait)
where
    V: VisitMut + ?Sized
```

### `visit_expr_binary_mut`

```rust
fn visit_expr_binary_mut<V>(v: &mut V, node: &mut crate::ExprBinary)
where
    V: VisitMut + ?Sized
```

### `visit_expr_block_mut`

```rust
fn visit_expr_block_mut<V>(v: &mut V, node: &mut crate::ExprBlock)
where
    V: VisitMut + ?Sized
```

### `visit_expr_break_mut`

```rust
fn visit_expr_break_mut<V>(v: &mut V, node: &mut crate::ExprBreak)
where
    V: VisitMut + ?Sized
```

### `visit_expr_call_mut`

```rust
fn visit_expr_call_mut<V>(v: &mut V, node: &mut crate::ExprCall)
where
    V: VisitMut + ?Sized
```

### `visit_expr_cast_mut`

```rust
fn visit_expr_cast_mut<V>(v: &mut V, node: &mut crate::ExprCast)
where
    V: VisitMut + ?Sized
```

### `visit_expr_closure_mut`

```rust
fn visit_expr_closure_mut<V>(v: &mut V, node: &mut crate::ExprClosure)
where
    V: VisitMut + ?Sized
```

### `visit_expr_const_mut`

```rust
fn visit_expr_const_mut<V>(v: &mut V, node: &mut crate::ExprConst)
where
    V: VisitMut + ?Sized
```

### `visit_expr_continue_mut`

```rust
fn visit_expr_continue_mut<V>(v: &mut V, node: &mut crate::ExprContinue)
where
    V: VisitMut + ?Sized
```

### `visit_expr_field_mut`

```rust
fn visit_expr_field_mut<V>(v: &mut V, node: &mut crate::ExprField)
where
    V: VisitMut + ?Sized
```

### `visit_expr_for_loop_mut`

```rust
fn visit_expr_for_loop_mut<V>(v: &mut V, node: &mut crate::ExprForLoop)
where
    V: VisitMut + ?Sized
```

### `visit_expr_group_mut`

```rust
fn visit_expr_group_mut<V>(v: &mut V, node: &mut crate::ExprGroup)
where
    V: VisitMut + ?Sized
```

### `visit_expr_if_mut`

```rust
fn visit_expr_if_mut<V>(v: &mut V, node: &mut crate::ExprIf)
where
    V: VisitMut + ?Sized
```

### `visit_expr_index_mut`

```rust
fn visit_expr_index_mut<V>(v: &mut V, node: &mut crate::ExprIndex)
where
    V: VisitMut + ?Sized
```

### `visit_expr_infer_mut`

```rust
fn visit_expr_infer_mut<V>(v: &mut V, node: &mut crate::ExprInfer)
where
    V: VisitMut + ?Sized
```

### `visit_expr_let_mut`

```rust
fn visit_expr_let_mut<V>(v: &mut V, node: &mut crate::ExprLet)
where
    V: VisitMut + ?Sized
```

### `visit_expr_lit_mut`

```rust
fn visit_expr_lit_mut<V>(v: &mut V, node: &mut crate::ExprLit)
where
    V: VisitMut + ?Sized
```

### `visit_expr_loop_mut`

```rust
fn visit_expr_loop_mut<V>(v: &mut V, node: &mut crate::ExprLoop)
where
    V: VisitMut + ?Sized
```

### `visit_expr_macro_mut`

```rust
fn visit_expr_macro_mut<V>(v: &mut V, node: &mut crate::ExprMacro)
where
    V: VisitMut + ?Sized
```

### `visit_expr_match_mut`

```rust
fn visit_expr_match_mut<V>(v: &mut V, node: &mut crate::ExprMatch)
where
    V: VisitMut + ?Sized
```

### `visit_expr_method_call_mut`

```rust
fn visit_expr_method_call_mut<V>(v: &mut V, node: &mut crate::ExprMethodCall)
where
    V: VisitMut + ?Sized
```

### `visit_expr_paren_mut`

```rust
fn visit_expr_paren_mut<V>(v: &mut V, node: &mut crate::ExprParen)
where
    V: VisitMut + ?Sized
```

### `visit_expr_path_mut`

```rust
fn visit_expr_path_mut<V>(v: &mut V, node: &mut crate::ExprPath)
where
    V: VisitMut + ?Sized
```

### `visit_expr_range_mut`

```rust
fn visit_expr_range_mut<V>(v: &mut V, node: &mut crate::ExprRange)
where
    V: VisitMut + ?Sized
```

### `visit_expr_raw_addr_mut`

```rust
fn visit_expr_raw_addr_mut<V>(v: &mut V, node: &mut crate::ExprRawAddr)
where
    V: VisitMut + ?Sized
```

### `visit_expr_reference_mut`

```rust
fn visit_expr_reference_mut<V>(v: &mut V, node: &mut crate::ExprReference)
where
    V: VisitMut + ?Sized
```

### `visit_expr_repeat_mut`

```rust
fn visit_expr_repeat_mut<V>(v: &mut V, node: &mut crate::ExprRepeat)
where
    V: VisitMut + ?Sized
```

### `visit_expr_return_mut`

```rust
fn visit_expr_return_mut<V>(v: &mut V, node: &mut crate::ExprReturn)
where
    V: VisitMut + ?Sized
```

### `visit_expr_struct_mut`

```rust
fn visit_expr_struct_mut<V>(v: &mut V, node: &mut crate::ExprStruct)
where
    V: VisitMut + ?Sized
```

### `visit_expr_try_mut`

```rust
fn visit_expr_try_mut<V>(v: &mut V, node: &mut crate::ExprTry)
where
    V: VisitMut + ?Sized
```

### `visit_expr_try_block_mut`

```rust
fn visit_expr_try_block_mut<V>(v: &mut V, node: &mut crate::ExprTryBlock)
where
    V: VisitMut + ?Sized
```

### `visit_expr_tuple_mut`

```rust
fn visit_expr_tuple_mut<V>(v: &mut V, node: &mut crate::ExprTuple)
where
    V: VisitMut + ?Sized
```

### `visit_expr_unary_mut`

```rust
fn visit_expr_unary_mut<V>(v: &mut V, node: &mut crate::ExprUnary)
where
    V: VisitMut + ?Sized
```

### `visit_expr_unsafe_mut`

```rust
fn visit_expr_unsafe_mut<V>(v: &mut V, node: &mut crate::ExprUnsafe)
where
    V: VisitMut + ?Sized
```

### `visit_expr_while_mut`

```rust
fn visit_expr_while_mut<V>(v: &mut V, node: &mut crate::ExprWhile)
where
    V: VisitMut + ?Sized
```

### `visit_expr_yield_mut`

```rust
fn visit_expr_yield_mut<V>(v: &mut V, node: &mut crate::ExprYield)
where
    V: VisitMut + ?Sized
```

### `visit_field_mut`

```rust
fn visit_field_mut<V>(v: &mut V, node: &mut crate::Field)
where
    V: VisitMut + ?Sized
```

### `visit_field_mutability_mut`

```rust
fn visit_field_mutability_mut<V>(v: &mut V, node: &mut crate::FieldMutability)
where
    V: VisitMut + ?Sized
```

### `visit_field_pat_mut`

```rust
fn visit_field_pat_mut<V>(v: &mut V, node: &mut crate::FieldPat)
where
    V: VisitMut + ?Sized
```

### `visit_field_value_mut`

```rust
fn visit_field_value_mut<V>(v: &mut V, node: &mut crate::FieldValue)
where
    V: VisitMut + ?Sized
```

### `visit_fields_mut`

```rust
fn visit_fields_mut<V>(v: &mut V, node: &mut crate::Fields)
where
    V: VisitMut + ?Sized
```

### `visit_fields_named_mut`

```rust
fn visit_fields_named_mut<V>(v: &mut V, node: &mut crate::FieldsNamed)
where
    V: VisitMut + ?Sized
```

### `visit_fields_unnamed_mut`

```rust
fn visit_fields_unnamed_mut<V>(v: &mut V, node: &mut crate::FieldsUnnamed)
where
    V: VisitMut + ?Sized
```

### `visit_file_mut`

```rust
fn visit_file_mut<V>(v: &mut V, node: &mut crate::File)
where
    V: VisitMut + ?Sized
```

### `visit_fn_arg_mut`

```rust
fn visit_fn_arg_mut<V>(v: &mut V, node: &mut crate::FnArg)
where
    V: VisitMut + ?Sized
```

### `visit_foreign_item_mut`

```rust
fn visit_foreign_item_mut<V>(v: &mut V, node: &mut crate::ForeignItem)
where
    V: VisitMut + ?Sized
```

### `visit_foreign_item_fn_mut`

```rust
fn visit_foreign_item_fn_mut<V>(v: &mut V, node: &mut crate::ForeignItemFn)
where
    V: VisitMut + ?Sized
```

### `visit_foreign_item_macro_mut`

```rust
fn visit_foreign_item_macro_mut<V>(v: &mut V, node: &mut crate::ForeignItemMacro)
where
    V: VisitMut + ?Sized
```

### `visit_foreign_item_static_mut`

```rust
fn visit_foreign_item_static_mut<V>(v: &mut V, node: &mut crate::ForeignItemStatic)
where
    V: VisitMut + ?Sized
```

### `visit_foreign_item_type_mut`

```rust
fn visit_foreign_item_type_mut<V>(v: &mut V, node: &mut crate::ForeignItemType)
where
    V: VisitMut + ?Sized
```

### `visit_generic_argument_mut`

```rust
fn visit_generic_argument_mut<V>(v: &mut V, node: &mut crate::GenericArgument)
where
    V: VisitMut + ?Sized
```

### `visit_generic_param_mut`

```rust
fn visit_generic_param_mut<V>(v: &mut V, node: &mut crate::GenericParam)
where
    V: VisitMut + ?Sized
```

### `visit_generics_mut`

```rust
fn visit_generics_mut<V>(v: &mut V, node: &mut crate::Generics)
where
    V: VisitMut + ?Sized
```

### `visit_ident_mut`

```rust
fn visit_ident_mut<V>(v: &mut V, node: &mut proc_macro2::Ident)
where
    V: VisitMut + ?Sized
```

### `visit_impl_item_mut`

```rust
fn visit_impl_item_mut<V>(v: &mut V, node: &mut crate::ImplItem)
where
    V: VisitMut + ?Sized
```

### `visit_impl_item_const_mut`

```rust
fn visit_impl_item_const_mut<V>(v: &mut V, node: &mut crate::ImplItemConst)
where
    V: VisitMut + ?Sized
```

### `visit_impl_item_fn_mut`

```rust
fn visit_impl_item_fn_mut<V>(v: &mut V, node: &mut crate::ImplItemFn)
where
    V: VisitMut + ?Sized
```

### `visit_impl_item_macro_mut`

```rust
fn visit_impl_item_macro_mut<V>(v: &mut V, node: &mut crate::ImplItemMacro)
where
    V: VisitMut + ?Sized
```

### `visit_impl_item_type_mut`

```rust
fn visit_impl_item_type_mut<V>(v: &mut V, node: &mut crate::ImplItemType)
where
    V: VisitMut + ?Sized
```

### `visit_impl_restriction_mut`

```rust
fn visit_impl_restriction_mut<V>(v: &mut V, node: &mut crate::ImplRestriction)
where
    V: VisitMut + ?Sized
```

### `visit_index_mut`

```rust
fn visit_index_mut<V>(v: &mut V, node: &mut crate::Index)
where
    V: VisitMut + ?Sized
```

### `visit_item_mut`

```rust
fn visit_item_mut<V>(v: &mut V, node: &mut crate::Item)
where
    V: VisitMut + ?Sized
```

### `visit_item_const_mut`

```rust
fn visit_item_const_mut<V>(v: &mut V, node: &mut crate::ItemConst)
where
    V: VisitMut + ?Sized
```

### `visit_item_enum_mut`

```rust
fn visit_item_enum_mut<V>(v: &mut V, node: &mut crate::ItemEnum)
where
    V: VisitMut + ?Sized
```

### `visit_item_extern_crate_mut`

```rust
fn visit_item_extern_crate_mut<V>(v: &mut V, node: &mut crate::ItemExternCrate)
where
    V: VisitMut + ?Sized
```

### `visit_item_fn_mut`

```rust
fn visit_item_fn_mut<V>(v: &mut V, node: &mut crate::ItemFn)
where
    V: VisitMut + ?Sized
```

### `visit_item_foreign_mod_mut`

```rust
fn visit_item_foreign_mod_mut<V>(v: &mut V, node: &mut crate::ItemForeignMod)
where
    V: VisitMut + ?Sized
```

### `visit_item_impl_mut`

```rust
fn visit_item_impl_mut<V>(v: &mut V, node: &mut crate::ItemImpl)
where
    V: VisitMut + ?Sized
```

### `visit_item_macro_mut`

```rust
fn visit_item_macro_mut<V>(v: &mut V, node: &mut crate::ItemMacro)
where
    V: VisitMut + ?Sized
```

### `visit_item_mod_mut`

```rust
fn visit_item_mod_mut<V>(v: &mut V, node: &mut crate::ItemMod)
where
    V: VisitMut + ?Sized
```

### `visit_item_static_mut`

```rust
fn visit_item_static_mut<V>(v: &mut V, node: &mut crate::ItemStatic)
where
    V: VisitMut + ?Sized
```

### `visit_item_struct_mut`

```rust
fn visit_item_struct_mut<V>(v: &mut V, node: &mut crate::ItemStruct)
where
    V: VisitMut + ?Sized
```

### `visit_item_trait_mut`

```rust
fn visit_item_trait_mut<V>(v: &mut V, node: &mut crate::ItemTrait)
where
    V: VisitMut + ?Sized
```

### `visit_item_trait_alias_mut`

```rust
fn visit_item_trait_alias_mut<V>(v: &mut V, node: &mut crate::ItemTraitAlias)
where
    V: VisitMut + ?Sized
```

### `visit_item_type_mut`

```rust
fn visit_item_type_mut<V>(v: &mut V, node: &mut crate::ItemType)
where
    V: VisitMut + ?Sized
```

### `visit_item_union_mut`

```rust
fn visit_item_union_mut<V>(v: &mut V, node: &mut crate::ItemUnion)
where
    V: VisitMut + ?Sized
```

### `visit_item_use_mut`

```rust
fn visit_item_use_mut<V>(v: &mut V, node: &mut crate::ItemUse)
where
    V: VisitMut + ?Sized
```

### `visit_label_mut`

```rust
fn visit_label_mut<V>(v: &mut V, node: &mut crate::Label)
where
    V: VisitMut + ?Sized
```

### `visit_lifetime_mut`

```rust
fn visit_lifetime_mut<V>(v: &mut V, node: &mut crate::Lifetime)
where
    V: VisitMut + ?Sized
```

### `visit_lifetime_param_mut`

```rust
fn visit_lifetime_param_mut<V>(v: &mut V, node: &mut crate::LifetimeParam)
where
    V: VisitMut + ?Sized
```

### `visit_lit_mut`

```rust
fn visit_lit_mut<V>(v: &mut V, node: &mut crate::Lit)
where
    V: VisitMut + ?Sized
```

### `visit_lit_bool_mut`

```rust
fn visit_lit_bool_mut<V>(v: &mut V, node: &mut crate::LitBool)
where
    V: VisitMut + ?Sized
```

### `visit_lit_byte_mut`

```rust
fn visit_lit_byte_mut<V>(v: &mut V, node: &mut crate::LitByte)
where
    V: VisitMut + ?Sized
```

### `visit_lit_byte_str_mut`

```rust
fn visit_lit_byte_str_mut<V>(v: &mut V, node: &mut crate::LitByteStr)
where
    V: VisitMut + ?Sized
```

### `visit_lit_cstr_mut`

```rust
fn visit_lit_cstr_mut<V>(v: &mut V, node: &mut crate::LitCStr)
where
    V: VisitMut + ?Sized
```

### `visit_lit_char_mut`

```rust
fn visit_lit_char_mut<V>(v: &mut V, node: &mut crate::LitChar)
where
    V: VisitMut + ?Sized
```

### `visit_lit_float_mut`

```rust
fn visit_lit_float_mut<V>(v: &mut V, node: &mut crate::LitFloat)
where
    V: VisitMut + ?Sized
```

### `visit_lit_int_mut`

```rust
fn visit_lit_int_mut<V>(v: &mut V, node: &mut crate::LitInt)
where
    V: VisitMut + ?Sized
```

### `visit_lit_str_mut`

```rust
fn visit_lit_str_mut<V>(v: &mut V, node: &mut crate::LitStr)
where
    V: VisitMut + ?Sized
```

### `visit_local_mut`

```rust
fn visit_local_mut<V>(v: &mut V, node: &mut crate::Local)
where
    V: VisitMut + ?Sized
```

### `visit_local_init_mut`

```rust
fn visit_local_init_mut<V>(v: &mut V, node: &mut crate::LocalInit)
where
    V: VisitMut + ?Sized
```

### `visit_macro_mut`

```rust
fn visit_macro_mut<V>(v: &mut V, node: &mut crate::Macro)
where
    V: VisitMut + ?Sized
```

### `visit_macro_delimiter_mut`

```rust
fn visit_macro_delimiter_mut<V>(v: &mut V, node: &mut crate::MacroDelimiter)
where
    V: VisitMut + ?Sized
```

### `visit_member_mut`

```rust
fn visit_member_mut<V>(v: &mut V, node: &mut crate::Member)
where
    V: VisitMut + ?Sized
```

### `visit_meta_mut`

```rust
fn visit_meta_mut<V>(v: &mut V, node: &mut crate::Meta)
where
    V: VisitMut + ?Sized
```

### `visit_meta_list_mut`

```rust
fn visit_meta_list_mut<V>(v: &mut V, node: &mut crate::MetaList)
where
    V: VisitMut + ?Sized
```

### `visit_meta_name_value_mut`

```rust
fn visit_meta_name_value_mut<V>(v: &mut V, node: &mut crate::MetaNameValue)
where
    V: VisitMut + ?Sized
```

### `visit_parenthesized_generic_arguments_mut`

```rust
fn visit_parenthesized_generic_arguments_mut<V>(v: &mut V, node: &mut crate::ParenthesizedGenericArguments)
where
    V: VisitMut + ?Sized
```

### `visit_pat_mut`

```rust
fn visit_pat_mut<V>(v: &mut V, node: &mut crate::Pat)
where
    V: VisitMut + ?Sized
```

### `visit_pat_ident_mut`

```rust
fn visit_pat_ident_mut<V>(v: &mut V, node: &mut crate::PatIdent)
where
    V: VisitMut + ?Sized
```

### `visit_pat_or_mut`

```rust
fn visit_pat_or_mut<V>(v: &mut V, node: &mut crate::PatOr)
where
    V: VisitMut + ?Sized
```

### `visit_pat_paren_mut`

```rust
fn visit_pat_paren_mut<V>(v: &mut V, node: &mut crate::PatParen)
where
    V: VisitMut + ?Sized
```

### `visit_pat_reference_mut`

```rust
fn visit_pat_reference_mut<V>(v: &mut V, node: &mut crate::PatReference)
where
    V: VisitMut + ?Sized
```

### `visit_pat_rest_mut`

```rust
fn visit_pat_rest_mut<V>(v: &mut V, node: &mut crate::PatRest)
where
    V: VisitMut + ?Sized
```

### `visit_pat_slice_mut`

```rust
fn visit_pat_slice_mut<V>(v: &mut V, node: &mut crate::PatSlice)
where
    V: VisitMut + ?Sized
```

### `visit_pat_struct_mut`

```rust
fn visit_pat_struct_mut<V>(v: &mut V, node: &mut crate::PatStruct)
where
    V: VisitMut + ?Sized
```

### `visit_pat_tuple_mut`

```rust
fn visit_pat_tuple_mut<V>(v: &mut V, node: &mut crate::PatTuple)
where
    V: VisitMut + ?Sized
```

### `visit_pat_tuple_struct_mut`

```rust
fn visit_pat_tuple_struct_mut<V>(v: &mut V, node: &mut crate::PatTupleStruct)
where
    V: VisitMut + ?Sized
```

### `visit_pat_type_mut`

```rust
fn visit_pat_type_mut<V>(v: &mut V, node: &mut crate::PatType)
where
    V: VisitMut + ?Sized
```

### `visit_pat_wild_mut`

```rust
fn visit_pat_wild_mut<V>(v: &mut V, node: &mut crate::PatWild)
where
    V: VisitMut + ?Sized
```

### `visit_path_mut`

```rust
fn visit_path_mut<V>(v: &mut V, node: &mut crate::Path)
where
    V: VisitMut + ?Sized
```

### `visit_path_arguments_mut`

```rust
fn visit_path_arguments_mut<V>(v: &mut V, node: &mut crate::PathArguments)
where
    V: VisitMut + ?Sized
```

### `visit_path_segment_mut`

```rust
fn visit_path_segment_mut<V>(v: &mut V, node: &mut crate::PathSegment)
where
    V: VisitMut + ?Sized
```

### `visit_pointer_mutability_mut`

```rust
fn visit_pointer_mutability_mut<V>(v: &mut V, node: &mut crate::PointerMutability)
where
    V: VisitMut + ?Sized
```

### `visit_precise_capture_mut`

```rust
fn visit_precise_capture_mut<V>(v: &mut V, node: &mut crate::PreciseCapture)
where
    V: VisitMut + ?Sized
```

### `visit_predicate_lifetime_mut`

```rust
fn visit_predicate_lifetime_mut<V>(v: &mut V, node: &mut crate::PredicateLifetime)
where
    V: VisitMut + ?Sized
```

### `visit_predicate_type_mut`

```rust
fn visit_predicate_type_mut<V>(v: &mut V, node: &mut crate::PredicateType)
where
    V: VisitMut + ?Sized
```

### `visit_qself_mut`

```rust
fn visit_qself_mut<V>(v: &mut V, node: &mut crate::QSelf)
where
    V: VisitMut + ?Sized
```

### `visit_range_limits_mut`

```rust
fn visit_range_limits_mut<V>(v: &mut V, node: &mut crate::RangeLimits)
where
    V: VisitMut + ?Sized
```

### `visit_receiver_mut`

```rust
fn visit_receiver_mut<V>(v: &mut V, node: &mut crate::Receiver)
where
    V: VisitMut + ?Sized
```

### `visit_return_type_mut`

```rust
fn visit_return_type_mut<V>(v: &mut V, node: &mut crate::ReturnType)
where
    V: VisitMut + ?Sized
```

### `visit_signature_mut`

```rust
fn visit_signature_mut<V>(v: &mut V, node: &mut crate::Signature)
where
    V: VisitMut + ?Sized
```

### `visit_span_mut`

```rust
fn visit_span_mut<V>(v: &mut V, node: &mut proc_macro2::Span)
where
    V: VisitMut + ?Sized
```

### `visit_static_mutability_mut`

```rust
fn visit_static_mutability_mut<V>(v: &mut V, node: &mut crate::StaticMutability)
where
    V: VisitMut + ?Sized
```

### `visit_stmt_mut`

```rust
fn visit_stmt_mut<V>(v: &mut V, node: &mut crate::Stmt)
where
    V: VisitMut + ?Sized
```

### `visit_stmt_macro_mut`

```rust
fn visit_stmt_macro_mut<V>(v: &mut V, node: &mut crate::StmtMacro)
where
    V: VisitMut + ?Sized
```

### `visit_trait_bound_mut`

```rust
fn visit_trait_bound_mut<V>(v: &mut V, node: &mut crate::TraitBound)
where
    V: VisitMut + ?Sized
```

### `visit_trait_bound_modifier_mut`

```rust
fn visit_trait_bound_modifier_mut<V>(v: &mut V, node: &mut crate::TraitBoundModifier)
where
    V: VisitMut + ?Sized
```

### `visit_trait_item_mut`

```rust
fn visit_trait_item_mut<V>(v: &mut V, node: &mut crate::TraitItem)
where
    V: VisitMut + ?Sized
```

### `visit_trait_item_const_mut`

```rust
fn visit_trait_item_const_mut<V>(v: &mut V, node: &mut crate::TraitItemConst)
where
    V: VisitMut + ?Sized
```

### `visit_trait_item_fn_mut`

```rust
fn visit_trait_item_fn_mut<V>(v: &mut V, node: &mut crate::TraitItemFn)
where
    V: VisitMut + ?Sized
```

### `visit_trait_item_macro_mut`

```rust
fn visit_trait_item_macro_mut<V>(v: &mut V, node: &mut crate::TraitItemMacro)
where
    V: VisitMut + ?Sized
```

### `visit_trait_item_type_mut`

```rust
fn visit_trait_item_type_mut<V>(v: &mut V, node: &mut crate::TraitItemType)
where
    V: VisitMut + ?Sized
```

### `visit_type_mut`

```rust
fn visit_type_mut<V>(v: &mut V, node: &mut crate::Type)
where
    V: VisitMut + ?Sized
```

### `visit_type_array_mut`

```rust
fn visit_type_array_mut<V>(v: &mut V, node: &mut crate::TypeArray)
where
    V: VisitMut + ?Sized
```

### `visit_type_bare_fn_mut`

```rust
fn visit_type_bare_fn_mut<V>(v: &mut V, node: &mut crate::TypeBareFn)
where
    V: VisitMut + ?Sized
```

### `visit_type_group_mut`

```rust
fn visit_type_group_mut<V>(v: &mut V, node: &mut crate::TypeGroup)
where
    V: VisitMut + ?Sized
```

### `visit_type_impl_trait_mut`

```rust
fn visit_type_impl_trait_mut<V>(v: &mut V, node: &mut crate::TypeImplTrait)
where
    V: VisitMut + ?Sized
```

### `visit_type_infer_mut`

```rust
fn visit_type_infer_mut<V>(v: &mut V, node: &mut crate::TypeInfer)
where
    V: VisitMut + ?Sized
```

### `visit_type_macro_mut`

```rust
fn visit_type_macro_mut<V>(v: &mut V, node: &mut crate::TypeMacro)
where
    V: VisitMut + ?Sized
```

### `visit_type_never_mut`

```rust
fn visit_type_never_mut<V>(v: &mut V, node: &mut crate::TypeNever)
where
    V: VisitMut + ?Sized
```

### `visit_type_param_mut`

```rust
fn visit_type_param_mut<V>(v: &mut V, node: &mut crate::TypeParam)
where
    V: VisitMut + ?Sized
```

### `visit_type_param_bound_mut`

```rust
fn visit_type_param_bound_mut<V>(v: &mut V, node: &mut crate::TypeParamBound)
where
    V: VisitMut + ?Sized
```

### `visit_type_paren_mut`

```rust
fn visit_type_paren_mut<V>(v: &mut V, node: &mut crate::TypeParen)
where
    V: VisitMut + ?Sized
```

### `visit_type_path_mut`

```rust
fn visit_type_path_mut<V>(v: &mut V, node: &mut crate::TypePath)
where
    V: VisitMut + ?Sized
```

### `visit_type_ptr_mut`

```rust
fn visit_type_ptr_mut<V>(v: &mut V, node: &mut crate::TypePtr)
where
    V: VisitMut + ?Sized
```

### `visit_type_reference_mut`

```rust
fn visit_type_reference_mut<V>(v: &mut V, node: &mut crate::TypeReference)
where
    V: VisitMut + ?Sized
```

### `visit_type_slice_mut`

```rust
fn visit_type_slice_mut<V>(v: &mut V, node: &mut crate::TypeSlice)
where
    V: VisitMut + ?Sized
```

### `visit_type_trait_object_mut`

```rust
fn visit_type_trait_object_mut<V>(v: &mut V, node: &mut crate::TypeTraitObject)
where
    V: VisitMut + ?Sized
```

### `visit_type_tuple_mut`

```rust
fn visit_type_tuple_mut<V>(v: &mut V, node: &mut crate::TypeTuple)
where
    V: VisitMut + ?Sized
```

### `visit_un_op_mut`

```rust
fn visit_un_op_mut<V>(v: &mut V, node: &mut crate::UnOp)
where
    V: VisitMut + ?Sized
```

### `visit_use_glob_mut`

```rust
fn visit_use_glob_mut<V>(v: &mut V, node: &mut crate::UseGlob)
where
    V: VisitMut + ?Sized
```

### `visit_use_group_mut`

```rust
fn visit_use_group_mut<V>(v: &mut V, node: &mut crate::UseGroup)
where
    V: VisitMut + ?Sized
```

### `visit_use_name_mut`

```rust
fn visit_use_name_mut<V>(v: &mut V, node: &mut crate::UseName)
where
    V: VisitMut + ?Sized
```

### `visit_use_path_mut`

```rust
fn visit_use_path_mut<V>(v: &mut V, node: &mut crate::UsePath)
where
    V: VisitMut + ?Sized
```

### `visit_use_rename_mut`

```rust
fn visit_use_rename_mut<V>(v: &mut V, node: &mut crate::UseRename)
where
    V: VisitMut + ?Sized
```

### `visit_use_tree_mut`

```rust
fn visit_use_tree_mut<V>(v: &mut V, node: &mut crate::UseTree)
where
    V: VisitMut + ?Sized
```

### `visit_variadic_mut`

```rust
fn visit_variadic_mut<V>(v: &mut V, node: &mut crate::Variadic)
where
    V: VisitMut + ?Sized
```

### `visit_variant_mut`

```rust
fn visit_variant_mut<V>(v: &mut V, node: &mut crate::Variant)
where
    V: VisitMut + ?Sized
```

### `visit_vis_restricted_mut`

```rust
fn visit_vis_restricted_mut<V>(v: &mut V, node: &mut crate::VisRestricted)
where
    V: VisitMut + ?Sized
```

### `visit_visibility_mut`

```rust
fn visit_visibility_mut<V>(v: &mut V, node: &mut crate::Visibility)
where
    V: VisitMut + ?Sized
```

### `visit_where_clause_mut`

```rust
fn visit_where_clause_mut<V>(v: &mut V, node: &mut crate::WhereClause)
where
    V: VisitMut + ?Sized
```

### `visit_where_predicate_mut`

```rust
fn visit_where_predicate_mut<V>(v: &mut V, node: &mut crate::WherePredicate)
where
    V: VisitMut + ?Sized
```

## Macros

### `full!`

### `skip!`

