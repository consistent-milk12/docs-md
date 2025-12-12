*[serde_derive](../index.md) / [de](index.md)*

---

# Module `de`

## Contents

- [Modules](#modules)
  - [`enum_`](#enum)
  - [`enum_adjacently`](#enum-adjacently)
  - [`enum_externally`](#enum-externally)
  - [`enum_internally`](#enum-internally)
  - [`enum_untagged`](#enum-untagged)
  - [`identifier`](#identifier)
  - [`struct_`](#struct)
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
  - [`expand_derive_deserialize`](#expand-derive-deserialize)
  - [`precondition`](#precondition)
  - [`precondition_sized`](#precondition-sized)
  - [`precondition_no_de_lifetime`](#precondition-no-de-lifetime)
  - [`build_generics`](#build-generics)
  - [`needs_deserialize_bound`](#needs-deserialize-bound)
  - [`requires_default`](#requires-default)
  - [`borrowed_lifetimes`](#borrowed-lifetimes)
  - [`deserialize_body`](#deserialize-body)
  - [`deserialize_in_place_body`](#deserialize-in-place-body)
  - [`deserialize_transparent`](#deserialize-transparent)
  - [`deserialize_from`](#deserialize-from)
  - [`deserialize_try_from`](#deserialize-try-from)
  - [`deserialize_seq`](#deserialize-seq)
  - [`field_i`](#field-i)
  - [`wrap_deserialize_with`](#wrap-deserialize-with)
  - [`wrap_deserialize_field_with`](#wrap-deserialize-field-with)
  - [`unwrap_to_variant_closure`](#unwrap-to-variant-closure)
  - [`expr_is_missing`](#expr-is-missing)
  - [`expr_is_missing_seq`](#expr-is-missing-seq)
  - [`effective_style`](#effective-style)
  - [`has_flatten`](#has-flatten)
  - [`de_type_generics_to_tokens`](#de-type-generics-to-tokens)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`enum_`](#enum) | mod |  |
| [`enum_adjacently`](#enum-adjacently) | mod | Deserialization for adjacently tagged enums |
| [`enum_externally`](#enum-externally) | mod | Deserialization for externally tagged enums |
| [`enum_internally`](#enum-internally) | mod | Deserialization for internally tagged enums |
| [`enum_untagged`](#enum-untagged) | mod | Deserialization for untagged enums |
| [`identifier`](#identifier) | mod | Deserialization of struct field identifiers and enum variant identifiers by way of a Rust enum. |
| [`struct_`](#struct) | mod |  |
| [`tuple`](#tuple) | mod |  |
| [`unit`](#unit) | mod |  |
| [`Parameters`](#parameters) | struct |  |
| [`FieldWithAliases`](#fieldwithaliases) | struct |  |
| [`DeImplGenerics`](#deimplgenerics) | struct |  |
| [`DeTypeGenerics`](#detypegenerics) | struct |  |
| [`BorrowedLifetimes`](#borrowedlifetimes) | enum |  |
| [`TupleForm`](#tupleform) | enum |  |
| [`StructForm`](#structform) | enum |  |
| [`expand_derive_deserialize`](#expand-derive-deserialize) | fn |  |
| [`precondition`](#precondition) | fn |  |
| [`precondition_sized`](#precondition-sized) | fn |  |
| [`precondition_no_de_lifetime`](#precondition-no-de-lifetime) | fn |  |
| [`build_generics`](#build-generics) | fn |  |
| [`needs_deserialize_bound`](#needs-deserialize-bound) | fn |  |
| [`requires_default`](#requires-default) | fn |  |
| [`borrowed_lifetimes`](#borrowed-lifetimes) | fn |  |
| [`deserialize_body`](#deserialize-body) | fn |  |
| [`deserialize_in_place_body`](#deserialize-in-place-body) | fn |  |
| [`deserialize_transparent`](#deserialize-transparent) | fn | Generates `Deserialize::deserialize` body for a type with `#[serde(transparent)]` attribute |
| [`deserialize_from`](#deserialize-from) | fn | Generates `Deserialize::deserialize` body for a type with `#[serde(from)]` attribute |
| [`deserialize_try_from`](#deserialize-try-from) | fn | Generates `Deserialize::deserialize` body for a type with `#[serde(try_from)]` attribute |
| [`deserialize_seq`](#deserialize-seq) | fn |  |
| [`field_i`](#field-i) | fn |  |
| [`wrap_deserialize_with`](#wrap-deserialize-with) | fn | This function wraps the expression in `#[serde(deserialize_with = "...")]` in a trait to prevent it from accessing the internal `Deserialize` state. |
| [`wrap_deserialize_field_with`](#wrap-deserialize-field-with) | fn |  |
| [`unwrap_to_variant_closure`](#unwrap-to-variant-closure) | fn |  |
| [`expr_is_missing`](#expr-is-missing) | fn |  |
| [`expr_is_missing_seq`](#expr-is-missing-seq) | fn |  |
| [`effective_style`](#effective-style) | fn |  |
| [`has_flatten`](#has-flatten) | fn | True if there is any field with a `#[serde(flatten)]` attribute, other than fields which are skipped. |
| [`de_type_generics_to_tokens`](#de-type-generics-to-tokens) | fn |  |

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

*Defined in [`serde_derive-1.0.228/src/de.rs:116-142`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L116-L142)*

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

- <span id="parameters-new"></span>`fn new(cont: &Container<'_>) -> Self` — [`Container`](../internals/ast/index.md#container)

- <span id="parameters-type-name"></span>`fn type_name(&self) -> String`

- <span id="parameters-generics-with-de-lifetime"></span>`fn generics_with_de_lifetime(&self) -> (DeImplGenerics<'_>, DeTypeGenerics<'_>, syn::TypeGenerics<'_>, Option<&syn::WhereClause>)` — [`DeImplGenerics`](#deimplgenerics), [`DeTypeGenerics`](#detypegenerics)

### `FieldWithAliases<'a>`

```rust
struct FieldWithAliases<'a> {
    ident: syn::Ident,
    aliases: &'a std::collections::BTreeSet<crate::internals::name::Name>,
}
```

*Defined in [`serde_derive-1.0.228/src/de.rs:650-653`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L650-L653)*

### `DeImplGenerics<'a>`

```rust
struct DeImplGenerics<'a>(&'a Parameters);
```

*Defined in [`serde_derive-1.0.228/src/de.rs:853`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L853)*

#### Trait Implementations

##### `impl Spanned for DeImplGenerics<'a>`

- <span id="deimplgenerics-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for DeImplGenerics<'a>`

- <span id="deimplgenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

### `DeTypeGenerics<'a>`

```rust
struct DeTypeGenerics<'a>(&'a Parameters);
```

*Defined in [`serde_derive-1.0.228/src/de.rs:913`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L913)*

#### Trait Implementations

##### `impl Spanned for DeTypeGenerics<'a>`

- <span id="detypegenerics-span"></span>`fn span(&self) -> Span`

##### `impl ToTokens for DeTypeGenerics<'a>`

- <span id="detypegenerics-to-tokens"></span>`fn to_tokens(&self, tokens: &mut TokenStream)`

## Enums

### `BorrowedLifetimes`

```rust
enum BorrowedLifetimes {
    Borrowed(std::collections::BTreeSet<syn::Lifetime>),
    Static,
}
```

*Defined in [`serde_derive-1.0.228/src/de.rs:257-260`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L257-L260)*

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

*Defined in [`serde_derive-1.0.228/src/de.rs:443-449`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L443-L449)*

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

*Defined in [`serde_derive-1.0.228/src/de.rs:640-648`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L640-L648)*

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

*Defined in [`serde_derive-1.0.228/src/de.rs:25-82`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L25-L82)*

### `precondition`

```rust
fn precondition(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

*Defined in [`serde_derive-1.0.228/src/de.rs:84-87`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L84-L87)*

### `precondition_sized`

```rust
fn precondition_sized(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

*Defined in [`serde_derive-1.0.228/src/de.rs:89-100`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L89-L100)*

### `precondition_no_de_lifetime`

```rust
fn precondition_no_de_lifetime(cx: &crate::internals::Ctxt, cont: &crate::internals::ast::Container<'_>)
```

*Defined in [`serde_derive-1.0.228/src/de.rs:102-114`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L102-L114)*

### `build_generics`

```rust
fn build_generics(cont: &crate::internals::ast::Container<'_>, borrowed: &BorrowedLifetimes) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/de.rs:192-228`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L192-L228)*

### `needs_deserialize_bound`

```rust
fn needs_deserialize_bound(field: &attr::Field, variant: Option<&attr::Variant>) -> bool
```

*Defined in [`serde_derive-1.0.228/src/de.rs:236-245`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L236-L245)*

### `requires_default`

```rust
fn requires_default(field: &attr::Field, _variant: Option<&attr::Variant>) -> bool
```

*Defined in [`serde_derive-1.0.228/src/de.rs:249-255`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L249-L255)*

### `borrowed_lifetimes`

```rust
fn borrowed_lifetimes(cont: &crate::internals::ast::Container<'_>) -> BorrowedLifetimes
```

*Defined in [`serde_derive-1.0.228/src/de.rs:292-304`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L292-L304)*

### `deserialize_body`

```rust
fn deserialize_body(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de.rs:306-330`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L306-L330)*

### `deserialize_in_place_body`

```rust
fn deserialize_in_place_body(_cont: &crate::internals::ast::Container<'_>, _params: &Parameters) -> Option<crate::fragment::Stmts>
```

*Defined in [`serde_derive-1.0.228/src/de.rs:378-380`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L378-L380)*

### `deserialize_transparent`

```rust
fn deserialize_transparent(cont: &crate::internals::ast::Container<'_>, params: &Parameters) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de.rs:383-423`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L383-L423)*

Generates `Deserialize::deserialize` body for a type with `#[serde(transparent)]` attribute

### `deserialize_from`

```rust
fn deserialize_from(type_from: &syn::Type) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de.rs:426-432`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L426-L432)*

Generates `Deserialize::deserialize` body for a type with `#[serde(from)]` attribute

### `deserialize_try_from`

```rust
fn deserialize_try_from(type_try_from: &syn::Type) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de.rs:435-441`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L435-L441)*

Generates `Deserialize::deserialize` body for a type with `#[serde(try_from)]` attribute

### `deserialize_seq`

```rust
fn deserialize_seq(type_path: &proc_macro2::TokenStream, params: &Parameters, fields: &[crate::internals::ast::Field<'_>], is_struct: bool, cattrs: &attr::Container, expecting: &str) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de.rs:451-552`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L451-L552)*

### `field_i`

```rust
fn field_i(i: usize) -> syn::Ident
```

*Defined in [`serde_derive-1.0.228/src/de.rs:655-657`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L655-L657)*

### `wrap_deserialize_with`

```rust
fn wrap_deserialize_with(params: &Parameters, value_ty: &proc_macro2::TokenStream, deserialize_with: &syn::ExprPath) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```

*Defined in [`serde_derive-1.0.228/src/de.rs:661-705`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L661-L705)*

This function wraps the expression in `#[serde(deserialize_with = "...")]`
in a trait to prevent it from accessing the internal `Deserialize` state.

### `wrap_deserialize_field_with`

```rust
fn wrap_deserialize_field_with(params: &Parameters, field_ty: &syn::Type, deserialize_with: &syn::ExprPath) -> (proc_macro2::TokenStream, proc_macro2::TokenStream)
```

*Defined in [`serde_derive-1.0.228/src/de.rs:707-713`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L707-L713)*

### `unwrap_to_variant_closure`

```rust
fn unwrap_to_variant_closure(params: &Parameters, variant: &crate::internals::ast::Variant<'_>, with_wrapper: bool) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/de.rs:716-761`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L716-L761)*

### `expr_is_missing`

```rust
fn expr_is_missing(field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container) -> crate::fragment::Fragment
```

*Defined in [`serde_derive-1.0.228/src/de.rs:763-803`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L763-L803)*

### `expr_is_missing_seq`

```rust
fn expr_is_missing_seq(assign_to: Option<proc_macro2::TokenStream>, index: usize, field: &crate::internals::ast::Field<'_>, cattrs: &attr::Container, expecting: &str) -> proc_macro2::TokenStream
```

*Defined in [`serde_derive-1.0.228/src/de.rs:805-836`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L805-L836)*

### `effective_style`

```rust
fn effective_style(variant: &crate::internals::ast::Variant<'_>) -> crate::internals::ast::Style
```

*Defined in [`serde_derive-1.0.228/src/de.rs:838-843`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L838-L843)*

### `has_flatten`

```rust
fn has_flatten(fields: &[crate::internals::ast::Field<'_>]) -> bool
```

*Defined in [`serde_derive-1.0.228/src/de.rs:847-851`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L847-L851)*

True if there is any field with a `#[serde(flatten)]` attribute, other than
fields which are skipped.

### `de_type_generics_to_tokens`

```rust
fn de_type_generics_to_tokens(generics: syn::Generics, borrowed: &BorrowedLifetimes, tokens: &mut proc_macro2::TokenStream)
```

*Defined in [`serde_derive-1.0.228/src/de.rs:917-937`](../../../.source_1765210505/serde_derive-1.0.228/src/de.rs#L917-L937)*

