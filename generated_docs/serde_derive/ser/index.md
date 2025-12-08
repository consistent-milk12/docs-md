*[serde_derive](../index.md) / [ser](index.md)*

---

# Module `ser`

## Contents

- [Structs](#structs)
  - [`Parameters`](#parameters)
- [Enums](#enums)
  - [`TupleVariant`](#tuplevariant)
  - [`StructVariant`](#structvariant)
  - [`StructTrait`](#structtrait)
  - [`TupleTrait`](#tupletrait)
- [Functions](#functions)
  - [`expand_derive_serialize`](#expand_derive_serialize)
  - [`precondition`](#precondition)
  - [`build_generics`](#build_generics)
  - [`needs_serialize_bound`](#needs_serialize_bound)
  - [`serialize_body`](#serialize_body)
  - [`serialize_transparent`](#serialize_transparent)
  - [`serialize_into`](#serialize_into)
  - [`serialize_unit_struct`](#serialize_unit_struct)
  - [`serialize_newtype_struct`](#serialize_newtype_struct)
  - [`serialize_tuple_struct`](#serialize_tuple_struct)
  - [`serialize_struct`](#serialize_struct)
  - [`serialize_struct_tag_field`](#serialize_struct_tag_field)
  - [`serialize_struct_as_struct`](#serialize_struct_as_struct)
  - [`serialize_struct_as_map`](#serialize_struct_as_map)
  - [`serialize_enum`](#serialize_enum)
  - [`serialize_variant`](#serialize_variant)
  - [`serialize_externally_tagged_variant`](#serialize_externally_tagged_variant)
  - [`serialize_internally_tagged_variant`](#serialize_internally_tagged_variant)
  - [`serialize_adjacently_tagged_variant`](#serialize_adjacently_tagged_variant)
  - [`serialize_untagged_variant`](#serialize_untagged_variant)
  - [`serialize_tuple_variant`](#serialize_tuple_variant)
  - [`serialize_struct_variant`](#serialize_struct_variant)
  - [`serialize_struct_variant_with_flatten`](#serialize_struct_variant_with_flatten)
  - [`serialize_tuple_struct_visitor`](#serialize_tuple_struct_visitor)
  - [`serialize_struct_visitor`](#serialize_struct_visitor)
  - [`wrap_serialize_field_with`](#wrap_serialize_field_with)
  - [`wrap_serialize_variant_with`](#wrap_serialize_variant_with)
  - [`wrap_serialize_with`](#wrap_serialize_with)
  - [`mut_if`](#mut_if)
  - [`get_member`](#get_member)
  - [`effective_style`](#effective_style)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Parameters`](#parameters) | struct |  |
| [`TupleVariant`](#tuplevariant) | enum |  |
| [`StructVariant`](#structvariant) | enum |  |
| [`StructTrait`](#structtrait) | enum |  |
| [`TupleTrait`](#tupletrait) | enum |  |
| [`expand_derive_serialize`](#expand_derive_serialize) | fn |  |
| [`precondition`](#precondition) | fn |  |
| [`build_generics`](#build_generics) | fn |  |
| [`needs_serialize_bound`](#needs_serialize_bound) | fn |  |
| [`serialize_body`](#serialize_body) | fn |  |
| [`serialize_transparent`](#serialize_transparent) | fn |  |
| [`serialize_into`](#serialize_into) | fn |  |
| [`serialize_unit_struct`](#serialize_unit_struct) | fn |  |
| [`serialize_newtype_struct`](#serialize_newtype_struct) | fn |  |
| [`serialize_tuple_struct`](#serialize_tuple_struct) | fn |  |
| [`serialize_struct`](#serialize_struct) | fn |  |
| [`serialize_struct_tag_field`](#serialize_struct_tag_field) | fn |  |
| [`serialize_struct_as_struct`](#serialize_struct_as_struct) | fn |  |
| [`serialize_struct_as_map`](#serialize_struct_as_map) | fn |  |
| [`serialize_enum`](#serialize_enum) | fn |  |
| [`serialize_variant`](#serialize_variant) | fn |  |
| [`serialize_externally_tagged_variant`](#serialize_externally_tagged_variant) | fn |  |
| [`serialize_internally_tagged_variant`](#serialize_internally_tagged_variant) | fn |  |
| [`serialize_adjacently_tagged_variant`](#serialize_adjacently_tagged_variant) | fn |  |
| [`serialize_untagged_variant`](#serialize_untagged_variant) | fn |  |
| [`serialize_tuple_variant`](#serialize_tuple_variant) | fn |  |
| [`serialize_struct_variant`](#serialize_struct_variant) | fn |  |
| [`serialize_struct_variant_with_flatten`](#serialize_struct_variant_with_flatten) | fn |  |
| [`serialize_tuple_struct_visitor`](#serialize_tuple_struct_visitor) | fn |  |
| [`serialize_struct_visitor`](#serialize_struct_visitor) | fn |  |
| [`wrap_serialize_field_with`](#wrap_serialize_field_with) | fn |  |
| [`wrap_serialize_variant_with`](#wrap_serialize_variant_with) | fn |  |
| [`wrap_serialize_with`](#wrap_serialize_with) | fn |  |
| [`mut_if`](#mut_if) | fn |  |
| [`get_member`](#get_member) | fn |  |
| [`effective_style`](#effective_style) | fn |  |

## Structs

### `Parameters`

```rust
struct Parameters {
    self_var: syn::Ident,
    this_type: syn::Path,
    this_value: syn::Path,
    generics: syn::Generics,
    is_remote: bool,
    is_packed: bool,
}
```

#### Fields

- **`self_var`**: `syn::Ident`

  Variable holding the value being serialized. Either `self` for local
  types or `__self` for remote types.

- **`this_type`**: `syn::Path`

  Path to the type the impl is for. Either a single `Ident` for local
  types (does not include generic parameters) or `some::remote::Path` for
  remote types.

- **`this_value`**: `syn::Path`

  Same as `this_type` but using `::<T>` for generic parameters for use in
  expression position.

- **`generics`**: `syn::Generics`

  Generics including any explicit and inferred bounds for the impl.

- **`is_remote`**: `bool`

  Type has a `serde(remote = "...")` attribute.

- **`is_packed`**: `bool`

  Type has a repr(packed) attribute.

#### Implementations

- <span id="parameters-new"></span>`fn new(cont: &Container<'_>) -> Self` — [`Container`](../internals/ast/index.md)

- <span id="parameters-type-name"></span>`fn type_name(&self) -> String`

## Enums

### `TupleVariant<'a>`

```rust
enum TupleVariant<'a> {
    ExternallyTagged {
        type_name: &'a crate::internals::name::Name,
        variant_index: u32,
        variant_name: &'a crate::internals::name::Name,
    },
    Untagged,
}
```

### `StructVariant<'a>`

```rust
enum StructVariant<'a> {
    ExternallyTagged {
        variant_index: u32,
        variant_name: &'a crate::internals::name::Name,
    },
    InternallyTagged {
        tag: &'a str,
        variant_name: &'a crate::internals::name::Name,
    },
    Untagged,
}
```

### `StructTrait`

```rust
enum StructTrait {
    SerializeMap,
    SerializeStruct,
    SerializeStructVariant,
}
```

#### Implementations

- <span id="structtrait-serialize-field"></span>`fn serialize_field(&self, span: Span) -> TokenStream`

- <span id="structtrait-skip-field"></span>`fn skip_field(&self, span: Span) -> Option<TokenStream>`

### `TupleTrait`

```rust
enum TupleTrait {
    SerializeTuple,
    SerializeTupleStruct,
    SerializeTupleVariant,
}
```

#### Implementations

- <span id="tupletrait-serialize-element"></span>`fn serialize_element(&self, span: Span) -> TokenStream`

## Functions

### `expand_derive_serialize`

```rust
fn expand_derive_serialize(input: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

### `precondition`

```rust
fn precondition(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `build_generics`

```rust
fn build_generics(cont: &crate::internals::ast::Container<'_>) -> syn::Generics
```

### `needs_serialize_bound`

```rust
fn needs_serialize_bound(field: &attr::Field, variant: Option<&attr::Variant>) -> bool
```

### `serialize_body`

```rust
fn serialize_body(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

### `serialize_transparent`

```rust
fn serialize_transparent(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

### `serialize_into`

```rust
fn serialize_into(params: &Parameters, type_into: &syn::Type) -> crate::fragment::Fragment
```

### `serialize_unit_struct`

```rust
fn serialize_unit_struct(cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_newtype_struct`

```rust
fn serialize_newtype_struct(params: &Parameters, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_tuple_struct`

```rust
fn serialize_tuple_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_struct`

```rust
fn serialize_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_struct_tag_field`

```rust
fn serialize_struct_tag_field(cattrs: &attr::Container, struct_trait: &StructTrait) -> proc_macro2::TokenStream
```

### `serialize_struct_as_struct`

```rust
fn serialize_struct_as_struct(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_struct_as_map`

```rust
fn serialize_struct_as_map(params: &Parameters, fields: &[crate::internals::ast::Field<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_enum`

```rust
fn serialize_enum(params: &Parameters, variants: &[crate::internals::ast::Variant<'_>], cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_variant`

```rust
fn serialize_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, variant_index: u32, cattrs: &attr::Container) -> proc_macro2::TokenStream
```

### `serialize_externally_tagged_variant`

```rust
fn serialize_externally_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, variant_index: u32, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_internally_tagged_variant`

```rust
fn serialize_internally_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container, tag: &str) -> crate::fragment::Fragment
```

### `serialize_adjacently_tagged_variant`

```rust
fn serialize_adjacently_tagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container, variant_index: u32, tag: &str, content: &str) -> crate::fragment::Fragment
```

### `serialize_untagged_variant`

```rust
fn serialize_untagged_variant(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `serialize_tuple_variant`

```rust
fn serialize_tuple_variant(context: TupleVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>]) -> crate::fragment::Fragment
```

### `serialize_struct_variant`

```rust
fn serialize_struct_variant(context: StructVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], name: &crate::internals::name::Name) -> crate::fragment::Fragment
```

### `serialize_struct_variant_with_flatten`

```rust
fn serialize_struct_variant_with_flatten(context: StructVariant<'_>, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], name: &crate::internals::name::Name) -> crate::fragment::Fragment
```

### `serialize_tuple_struct_visitor`

```rust
fn serialize_tuple_struct_visitor(fields: &[crate::internals::ast::Field<'_>], params: &Parameters, is_enum: bool, tuple_trait: &TupleTrait) -> Vec<proc_macro2::TokenStream>
```

### `serialize_struct_visitor`

```rust
fn serialize_struct_visitor(fields: &[crate::internals::ast::Field<'_>], params: &Parameters, is_enum: bool, struct_trait: &StructTrait) -> Vec<proc_macro2::TokenStream>
```

### `wrap_serialize_field_with`

```rust
fn wrap_serialize_field_with(params: &Parameters, field_ty: &syn::Type, serialize_with: &syn::ExprPath, field_expr: &proc_macro2::TokenStream) -> proc_macro2::TokenStream
```

### `wrap_serialize_variant_with`

```rust
fn wrap_serialize_variant_with(params: &Parameters, serialize_with: &syn::ExprPath, variant: &crate::internals::ast::Variant<'_>) -> proc_macro2::TokenStream
```

### `wrap_serialize_with`

```rust
fn wrap_serialize_with(params: &Parameters, serialize_with: &syn::ExprPath, field_tys: &[&syn::Type], field_exprs: &[proc_macro2::TokenStream]) -> proc_macro2::TokenStream
```

### `mut_if`

```rust
fn mut_if(is_mut: bool) -> Option<proc_macro2::TokenStream>
```

### `get_member`

```rust
fn get_member(params: &Parameters, field: &crate::internals::ast::Field<'_>, member: &syn::Member) -> proc_macro2::TokenStream
```

### `effective_style`

```rust
fn effective_style(variant: &crate::internals::ast::Variant<'_>) -> crate::internals::ast::Style
```

