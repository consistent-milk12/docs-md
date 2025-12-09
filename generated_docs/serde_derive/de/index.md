*[serde_derive](../index.md) / [de](index.md)*

---

# Module `de`

## Contents

- [Modules](#modules)
  - [`enum_`](#enum_)
  - [`enum_adjacently`](#enum_adjacently)
  - [`enum_externally`](#enum_externally)
  - [`enum_internally`](#enum_internally)
  - [`enum_untagged`](#enum_untagged)
  - [`identifier`](#identifier)
  - [`struct_`](#struct_)
  - [`tuple`](#tuple)
  - [`unit`](#unit)
- [Structs](#structs)
  - [`Parameters`](#parameters)
  - [`FieldWithAliases`](#fieldwithaliases)
  - [`DeImplGenerics`](#deimplgenerics)
  - [`DeTypeGenerics`](#detypegenerics)
- [Enums](#enums)
  - [`BorrowedLifetimes`](#borrowedlifetimes)
  - [`TupleForm`](#tupleform)
  - [`StructForm`](#structform)
- [Functions](#functions)
  - [`expand_derive_deserialize`](#expand_derive_deserialize)
  - [`precondition`](#precondition)
  - [`precondition_sized`](#precondition_sized)
  - [`precondition_no_de_lifetime`](#precondition_no_de_lifetime)
  - [`build_generics`](#build_generics)
  - [`needs_deserialize_bound`](#needs_deserialize_bound)
  - [`requires_default`](#requires_default)
  - [`borrowed_lifetimes`](#borrowed_lifetimes)
  - [`deserialize_body`](#deserialize_body)
  - [`deserialize_in_place_body`](#deserialize_in_place_body)
  - [`deserialize_transparent`](#deserialize_transparent)
  - [`deserialize_from`](#deserialize_from)
  - [`deserialize_try_from`](#deserialize_try_from)
  - [`deserialize_seq`](#deserialize_seq)
  - [`field_i`](#field_i)
  - [`wrap_deserialize_with`](#wrap_deserialize_with)
  - [`wrap_deserialize_field_with`](#wrap_deserialize_field_with)
  - [`unwrap_to_variant_closure`](#unwrap_to_variant_closure)
  - [`expr_is_missing`](#expr_is_missing)
  - [`expr_is_missing_seq`](#expr_is_missing_seq)
  - [`effective_style`](#effective_style)
  - [`has_flatten`](#has_flatten)
  - [`de_type_generics_to_tokens`](#de_type_generics_to_tokens)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`enum_`](#enum_) | mod |  |
| [`enum_adjacently`](#enum_adjacently) | mod | Deserialization for adjacently tagged enums |
| [`enum_externally`](#enum_externally) | mod | Deserialization for externally tagged enums |
| [`enum_internally`](#enum_internally) | mod | Deserialization for internally tagged enums |
| [`enum_untagged`](#enum_untagged) | mod | Deserialization for untagged enums |
| [`identifier`](#identifier) | mod | Deserialization of struct field identifiers and enum variant identifiers by |
| [`struct_`](#struct_) | mod |  |
| [`tuple`](#tuple) | mod |  |
| [`unit`](#unit) | mod |  |
| [`Parameters`](#parameters) | struct |  |
| [`FieldWithAliases`](#fieldwithaliases) | struct |  |
| [`DeImplGenerics`](#deimplgenerics) | struct |  |
| [`DeTypeGenerics`](#detypegenerics) | struct |  |
| [`BorrowedLifetimes`](#borrowedlifetimes) | enum |  |
| [`TupleForm`](#tupleform) | enum |  |
| [`StructForm`](#structform) | enum |  |
| [`expand_derive_deserialize`](#expand_derive_deserialize) | fn |  |
| [`precondition`](#precondition) | fn |  |
| [`precondition_sized`](#precondition_sized) | fn |  |
| [`precondition_no_de_lifetime`](#precondition_no_de_lifetime) | fn |  |
| [`build_generics`](#build_generics) | fn |  |
| [`needs_deserialize_bound`](#needs_deserialize_bound) | fn |  |
| [`requires_default`](#requires_default) | fn |  |
| [`borrowed_lifetimes`](#borrowed_lifetimes) | fn |  |
| [`deserialize_body`](#deserialize_body) | fn |  |
| [`deserialize_in_place_body`](#deserialize_in_place_body) | fn |  |
| [`deserialize_transparent`](#deserialize_transparent) | fn | Generates `Deserialize::deserialize` body for a type with `#[serde(transparent)]` attribute |
| [`deserialize_from`](#deserialize_from) | fn | Generates `Deserialize::deserialize` body for a type with `#[serde(from)]` attribute |
| [`deserialize_try_from`](#deserialize_try_from) | fn | Generates `Deserialize::deserialize` body for a type with `#[serde(try_from)]` attribute |
| [`deserialize_seq`](#deserialize_seq) | fn |  |
| [`field_i`](#field_i) | fn |  |
| [`wrap_deserialize_with`](#wrap_deserialize_with) | fn | This function wraps the expression in `#[serde(deserialize_with = "...")]` |
| [`wrap_deserialize_field_with`](#wrap_deserialize_field_with) | fn |  |
| [`unwrap_to_variant_closure`](#unwrap_to_variant_closure) | fn |  |
| [`expr_is_missing`](#expr_is_missing) | fn |  |
| [`expr_is_missing_seq`](#expr_is_missing_seq) | fn |  |
| [`effective_style`](#effective_style) | fn |  |
| [`has_flatten`](#has_flatten) | fn | True if there is any field with a `#[serde(flatten)]` attribute, other than |
| [`de_type_generics_to_tokens`](#de_type_generics_to_tokens) | fn |  |

## Modules

- [`enum_`](enum_/index.md)
- [`enum_adjacently`](enum_adjacently/index.md) — Deserialization for adjacently tagged enums:
- [`enum_externally`](enum_externally/index.md) — Deserialization for externally tagged enums:
- [`enum_internally`](enum_internally/index.md) — Deserialization for internally tagged enums:
- [`enum_untagged`](enum_untagged/index.md) — Deserialization for untagged enums:
- [`identifier`](identifier/index.md) — Deserialization of struct field identifiers and enum variant identifiers by
- [`struct_`](struct_/index.md)
- [`tuple`](tuple/index.md)
- [`unit`](unit/index.md)

## Structs

### `Parameters`

```rust
struct Parameters {
    local: syn::Ident,
    this_type: syn::Path,
    this_value: syn::Path,
    generics: syn::Generics,
    borrowed: BorrowedLifetimes,
    has_getter: bool,
    is_packed: bool,
}
```

#### Fields

- **`local`**: `syn::Ident`

  Name of the type the `derive` is on.

- **`this_type`**: `syn::Path`

  Path to the type the impl is for. Either a single `Ident` for local
  types (does not include generic parameters) or `some::remote::Path` for
  remote types.

- **`this_value`**: `syn::Path`

  Same as `this_type` but using `::<T>` for generic parameters for use in
  expression position.

- **`generics`**: `syn::Generics`

  Generics including any explicit and inferred bounds for the impl.

- **`borrowed`**: `BorrowedLifetimes`

  Lifetimes borrowed from the deserializer. These will become bounds on
  the `'de` lifetime of the deserializer.

- **`has_getter`**: `bool`

  At least one field has a serde(getter) attribute, implying that the
  remote type has a private field.

- **`is_packed`**: `bool`

  Type has a repr(packed) attribute.

#### Implementations

- <span id="parameters-new"></span>`fn new(cont: &Container<'_>) -> Self` — [`Container`](../internals/ast/index.md)

- <span id="parameters-type-name"></span>`fn type_name(&self) -> String`

- <span id="parameters-generics-with-de-lifetime"></span>`fn generics_with_de_lifetime(&self) -> (DeImplGenerics<'_>, DeTypeGenerics<'_>, syn::TypeGenerics<'_>, Option<&syn::WhereClause>)` — [`DeImplGenerics`](#deimplgenerics), [`DeTypeGenerics`](#detypegenerics)

### `FieldWithAliases<'a>`

```rust
struct FieldWithAliases<'a> {
    ident: syn::Ident,
    aliases: &'a std::collections::BTreeSet<crate::internals::name::Name>,
}
```

### `DeImplGenerics<'a>`

```rust
struct DeImplGenerics<'a>(&'a Parameters);
```

#### Trait Implementations

##### `impl<T> Spanned for DeImplGenerics<'a>`

- <span id="deimplgenerics-span"></span>`fn span(&self) -> Span`

##### `impl<'a> ToTokens for DeImplGenerics<'a>`

- <span id="deimplgenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `DeTypeGenerics<'a>`

```rust
struct DeTypeGenerics<'a>(&'a Parameters);
```

#### Trait Implementations

##### `impl<T> Spanned for DeTypeGenerics<'a>`

- <span id="detypegenerics-span"></span>`fn span(&self) -> Span`

##### `impl<'a> ToTokens for DeTypeGenerics<'a>`

- <span id="detypegenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `BorrowedLifetimes`

```rust
enum BorrowedLifetimes {
    Borrowed(std::collections::BTreeSet<syn::Lifetime>),
    Static,
}
```

#### Implementations

- <span id="borrowedlifetimes-de-lifetime"></span>`fn de_lifetime(&self) -> syn::Lifetime`

- <span id="borrowedlifetimes-de-lifetime-param"></span>`fn de_lifetime_param(&self) -> Option<syn::LifetimeParam>`

### `TupleForm<'a>`

```rust
enum TupleForm<'a> {
    Tuple,
    ExternallyTagged(&'a syn::Ident),
    Untagged(&'a syn::Ident),
}
```

#### Variants

- **`ExternallyTagged`**

  Contains a variant name

- **`Untagged`**

  Contains a variant name

### `StructForm<'a>`

```rust
enum StructForm<'a> {
    Struct,
    ExternallyTagged(&'a syn::Ident),
    InternallyTagged(&'a syn::Ident),
    Untagged(&'a syn::Ident),
}
```

#### Variants

- **`ExternallyTagged`**

  Contains a variant name

- **`InternallyTagged`**

  Contains a variant name

- **`Untagged`**

  Contains a variant name

## Functions

### `expand_derive_deserialize`

```rust
fn expand_derive_deserialize(input: &mut syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream>
```

### `precondition`

```rust
fn precondition(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `precondition_sized`

```rust
fn precondition_sized(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `precondition_no_de_lifetime`

```rust
fn precondition_no_de_lifetime(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

### `build_generics`

```rust
fn build_generics(cont: &crate::internals::ast::Container<'_>, borrowed: &BorrowedLifetimes) -> syn::Generics
```

### `needs_deserialize_bound`

```rust
fn needs_deserialize_bound(field: &attr::Field, variant: Option<&attr::Variant>) -> bool
```

### `requires_default`

```rust
fn requires_default(field: &attr::Field, _variant: Option<&attr::Variant>) -> bool
```

### `borrowed_lifetimes`

```rust
fn borrowed_lifetimes(cont: &crate::internals::ast::Container<'_>) -> BorrowedLifetimes
```

### `deserialize_body`

```rust
fn deserialize_body(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

### `deserialize_in_place_body`

```rust
fn deserialize_in_place_body(_cont: &crate::internals::ast::Container<'_>, _params: &Parameters) -> Option<crate::fragment::Stmts>
```

### `deserialize_transparent`

```rust
fn deserialize_transparent(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for a type with `#[serde(transparent)]` attribute

### `deserialize_from`

```rust
fn deserialize_from(type_from: &syn::Type) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for a type with `#[serde(from)]` attribute

### `deserialize_try_from`

```rust
fn deserialize_try_from(type_try_from: &syn::Type) -> crate::fragment::Fragment
```

Generates `Deserialize::deserialize` body for a type with `#[serde(try_from)]` attribute

### `deserialize_seq`

```rust
fn deserialize_seq(type_path: &proc_macro2::TokenStream, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], is_struct: bool, cattrs: &attr::Container, expecting: &str) -> crate::fragment::Fragment
```

### `field_i`

```rust
fn field_i(i: usize) -> syn::Ident
```

### `wrap_deserialize_with`

```rust
fn wrap_deserialize_with(params: &Parameters, value_ty: &proc_macro2::TokenStream, deserialize_with: &syn::ExprPath) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```

This function wraps the expression in `#[serde(deserialize_with = "...")]`
in a trait to prevent it from accessing the internal `Deserialize` state.

### `wrap_deserialize_field_with`

```rust
fn wrap_deserialize_field_with(params: &Parameters, field_ty: &syn::Type, deserialize_with: &syn::ExprPath) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```

### `unwrap_to_variant_closure`

```rust
fn unwrap_to_variant_closure(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, with_wrapper: bool) -> proc_macro2::TokenStream
```

### `expr_is_missing`

```rust
fn expr_is_missing(field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

### `expr_is_missing_seq`

```rust
fn expr_is_missing_seq(assign_to: Option<proc_macro2::TokenStream>, index: usize, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container, expecting: &str) -> proc_macro2::TokenStream
```

### `effective_style`

```rust
fn effective_style(variant: &crate::internals::ast::Variant<'_>) -> crate::internals::ast::Style
```

### `has_flatten`

```rust
fn has_flatten(fields: &[crate::internals::ast::Field<'_>]) -> bool
```

True if there is any field with a `#[serde(flatten)]` attribute, other than
fields which are skipped.

### `de_type_generics_to_tokens`

```rust
fn de_type_generics_to_tokens(generics: syn::Generics, borrowed: &BorrowedLifetimes, tokens: &mut proc_macro2::TokenStream)
```

