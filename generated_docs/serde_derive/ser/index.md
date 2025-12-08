*[serde_derive](../index.md) / [ser](index.md)*

---

# Module `ser`

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

- `fn new(cont: &Container<'_>) -> Self` — [`Container`](../internals/ast/index.md)

- `fn type_name(self: &Self) -> String`

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

- `fn serialize_field(self: &Self, span: Span) -> TokenStream`

- `fn skip_field(self: &Self, span: Span) -> Option<TokenStream>`

### `TupleTrait`

```rust
enum TupleTrait {
    SerializeTuple,
    SerializeTupleStruct,
    SerializeTupleVariant,
}
```

#### Implementations

- `fn serialize_element(self: &Self, span: Span) -> TokenStream`

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

