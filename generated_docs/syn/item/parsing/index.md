*[syn](../../index.md) / [item](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Contents

- [Structs](#structs)
  - [`FlexibleItemType`](#flexibleitemtype)
- [Enums](#enums)
  - [`TypeDefaultness`](#typedefaultness)
  - [`WhereClauseLocation`](#whereclauselocation)
  - [`FnArgOrVariadic`](#fnargorvariadic)
- [Functions](#functions)
  - [`parse_rest_of_item`](#parse-rest-of-item)
  - [`parse_macro2`](#parse-macro2)
  - [`parse_item_use`](#parse-item-use)
  - [`parse_use_tree`](#parse-use-tree)
  - [`peek_signature`](#peek-signature)
  - [`parse_signature`](#parse-signature)
  - [`parse_rest_of_fn`](#parse-rest-of-fn)
  - [`parse_fn_arg_or_variadic`](#parse-fn-arg-or-variadic)
  - [`parse_fn_args`](#parse-fn-args)
  - [`parse_foreign_item_type`](#parse-foreign-item-type)
  - [`parse_item_type`](#parse-item-type)
  - [`parse_trait_or_trait_alias`](#parse-trait-or-trait-alias)
  - [`parse_rest_of_trait`](#parse-rest-of-trait)
  - [`parse_start_of_trait_alias`](#parse-start-of-trait-alias)
  - [`parse_rest_of_trait_alias`](#parse-rest-of-trait-alias)
  - [`parse_trait_item_type`](#parse-trait-item-type)
  - [`parse_impl`](#parse-impl)
  - [`parse_impl_item_fn`](#parse-impl-item-fn)
  - [`parse_impl_item_type`](#parse-impl-item-type)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FlexibleItemType`](#flexibleitemtype) | struct |  |
| [`TypeDefaultness`](#typedefaultness) | enum |  |
| [`WhereClauseLocation`](#whereclauselocation) | enum |  |
| [`FnArgOrVariadic`](#fnargorvariadic) | enum |  |
| [`parse_rest_of_item`](#parse-rest-of-item) | fn |  |
| [`parse_macro2`](#parse-macro2) | fn |  |
| [`parse_item_use`](#parse-item-use) | fn |  |
| [`parse_use_tree`](#parse-use-tree) | fn |  |
| [`peek_signature`](#peek-signature) | fn |  |
| [`parse_signature`](#parse-signature) | fn |  |
| [`parse_rest_of_fn`](#parse-rest-of-fn) | fn |  |
| [`parse_fn_arg_or_variadic`](#parse-fn-arg-or-variadic) | fn |  |
| [`parse_fn_args`](#parse-fn-args) | fn |  |
| [`parse_foreign_item_type`](#parse-foreign-item-type) | fn |  |
| [`parse_item_type`](#parse-item-type) | fn |  |
| [`parse_trait_or_trait_alias`](#parse-trait-or-trait-alias) | fn |  |
| [`parse_rest_of_trait`](#parse-rest-of-trait) | fn |  |
| [`parse_start_of_trait_alias`](#parse-start-of-trait-alias) | fn |  |
| [`parse_rest_of_trait_alias`](#parse-rest-of-trait-alias) | fn |  |
| [`parse_trait_item_type`](#parse-trait-item-type) | fn |  |
| [`parse_impl`](#parse-impl) | fn |  |
| [`parse_impl_item_fn`](#parse-impl-item-fn) | fn |  |
| [`parse_impl_item_type`](#parse-impl-item-type) | fn |  |

## Structs

### `FlexibleItemType`

```rust
struct FlexibleItemType {
    vis: crate::restriction::Visibility,
    defaultness: Option<token::Default>,
    type_token: token::Type,
    ident: crate::ident::Ident,
    generics: crate::generics::Generics,
    colon_token: Option<token::Colon>,
    bounds: crate::punctuated::Punctuated<crate::generics::TypeParamBound, token::Plus>,
    ty: Option<(token::Eq, crate::ty::Type)>,
    semi_token: token::Semi,
}
```

*Defined in [`syn-2.0.111/src/item.rs:1123-1133`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1123-L1133)*

#### Implementations

- <span id="flexibleitemtype-parse"></span>`fn parse(input: ParseStream<'_>, allow_defaultness: TypeDefaultness, where_clause_location: WhereClauseLocation) -> Result<Self>` — [`ParseStream`](../../parse/index.md#parsestream), [`TypeDefaultness`](#typedefaultness), [`WhereClauseLocation`](#whereclauselocation), [`Result`](../../error/index.md#result)

- <span id="flexibleitemtype-parse-optional-bounds"></span>`fn parse_optional_bounds(input: ParseStream<'_>) -> Result<(Option<token::Colon>, Punctuated<TypeParamBound, token::Plus>)>` — [`ParseStream`](../../parse/index.md#parsestream), [`Result`](../../error/index.md#result), [`Colon`](../../token/index.md#colon), [`Punctuated`](../../punctuated/index.md#punctuated), [`TypeParamBound`](../../generics/index.md#typeparambound), [`Plus`](../../token/index.md#plus)

- <span id="flexibleitemtype-parse-optional-definition"></span>`fn parse_optional_definition(input: ParseStream<'_>) -> Result<Option<(token::Eq, Type)>>` — [`ParseStream`](../../parse/index.md#parsestream), [`Result`](../../error/index.md#result), [`Eq`](../../token/index.md#eq), [`Type`](../../ty/index.md#type)

## Enums

### `TypeDefaultness`

```rust
enum TypeDefaultness {
    Optional,
    Disallowed,
}
```

*Defined in [`syn-2.0.111/src/item.rs:1135-1138`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1135-L1138)*

### `WhereClauseLocation`

```rust
enum WhereClauseLocation {
    BeforeEq,
    AfterEq,
    Both,
}
```

*Defined in [`syn-2.0.111/src/item.rs:1140-1147`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1140-L1147)*

### `FnArgOrVariadic`

```rust
enum FnArgOrVariadic {
    FnArg(crate::item::FnArg),
    Variadic(crate::item::Variadic),
}
```

*Defined in [`syn-2.0.111/src/item.rs:1595-1598`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1595-L1598)*

## Functions

### `parse_rest_of_item`

```rust
fn parse_rest_of_item(begin: crate::parse::ParseBuffer<'_>, attrs: Vec<crate::attr::Attribute>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

*Defined in [`syn-2.0.111/src/item.rs:946-1121`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L946-L1121)*

### `parse_macro2`

```rust
fn parse_macro2(begin: crate::parse::ParseBuffer<'_>, _vis: crate::restriction::Visibility, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

*Defined in [`syn-2.0.111/src/item.rs:1266-1287`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1266-L1287)*

### `parse_item_use`

```rust
fn parse_item_use(input: crate::parse::ParseStream<'_>, allow_crate_root_in_path: bool) -> crate::error::Result<Option<crate::item::ItemUse>>
```

*Defined in [`syn-2.0.111/src/item.rs:1330-1354`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1330-L1354)*

### `parse_use_tree`

```rust
fn parse_use_tree(input: crate::parse::ParseStream<'_>, allow_crate_root_in_path: bool) -> crate::error::Result<Option<crate::item::UseTree>>
```

*Defined in [`syn-2.0.111/src/item.rs:1364-1438`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1364-L1438)*

### `peek_signature`

```rust
fn peek_signature(input: crate::parse::ParseStream<'_>, allow_safe: bool) -> bool
```

*Defined in [`syn-2.0.111/src/item.rs:1493-1503`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1493-L1503)*

### `parse_signature`

```rust
fn parse_signature(input: crate::parse::ParseStream<'_>, allow_safe: bool) -> crate::error::Result<Option<crate::item::Signature>>
```

*Defined in [`syn-2.0.111/src/item.rs:1513-1552`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1513-L1552)*

### `parse_rest_of_fn`

```rust
fn parse_rest_of_fn(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, sig: crate::item::Signature) -> crate::error::Result<crate::item::ItemFn>
```

*Defined in [`syn-2.0.111/src/item.rs:1564-1581`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1564-L1581)*

### `parse_fn_arg_or_variadic`

```rust
fn parse_fn_arg_or_variadic(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, allow_variadic: bool) -> crate::error::Result<FnArgOrVariadic>
```

*Defined in [`syn-2.0.111/src/item.rs:1600-1648`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1600-L1648)*

### `parse_fn_args`

```rust
fn parse_fn_args(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(crate::punctuated::Punctuated<crate::item::FnArg, token::Comma>, Option<crate::item::Variadic>)>
```

*Defined in [`syn-2.0.111/src/item.rs:1695-1762`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1695-L1762)*

### `parse_foreign_item_type`

```rust
fn parse_foreign_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::ForeignItem>
```

*Defined in [`syn-2.0.111/src/item.rs:1987-2016`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L1987-L2016)*

### `parse_item_type`

```rust
fn parse_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

*Defined in [`syn-2.0.111/src/item.rs:2056-2088`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2056-L2088)*

### `parse_trait_or_trait_alias`

```rust
fn parse_trait_or_trait_alias(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::Item>
```

*Defined in [`syn-2.0.111/src/item.rs:2161-2187`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2161-L2187)*

### `parse_rest_of_trait`

```rust
fn parse_rest_of_trait(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, unsafety: Option<token::Unsafe>, auto_token: Option<token::Auto>, trait_token: token::Trait, ident: crate::ident::Ident, generics: crate::generics::Generics) -> crate::error::Result<crate::item::ItemTrait>
```

*Defined in [`syn-2.0.111/src/item.rs:2212-2266`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2212-L2266)*

### `parse_start_of_trait_alias`

```rust
fn parse_start_of_trait_alias(input: crate::parse::ParseStream<'_>) -> crate::error::Result<(Vec<crate::attr::Attribute>, crate::restriction::Visibility, token::Trait, crate::ident::Ident, crate::generics::Generics)>
```

*Defined in [`syn-2.0.111/src/item.rs:2276-2285`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2276-L2285)*

### `parse_rest_of_trait_alias`

```rust
fn parse_rest_of_trait_alias(input: crate::parse::ParseStream<'_>, attrs: Vec<crate::attr::Attribute>, vis: crate::restriction::Visibility, trait_token: token::Trait, ident: crate::ident::Ident, generics: crate::generics::Generics) -> crate::error::Result<crate::item::ItemTraitAlias>
```

*Defined in [`syn-2.0.111/src/item.rs:2287-2326`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2287-L2326)*

### `parse_trait_item_type`

```rust
fn parse_trait_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::TraitItem>
```

*Defined in [`syn-2.0.111/src/item.rs:2504-2535`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2504-L2535)*

### `parse_impl`

```rust
fn parse_impl(input: crate::parse::ParseStream<'_>, allow_verbatim_impl: bool) -> crate::error::Result<Option<crate::item::ItemImpl>>
```

*Defined in [`syn-2.0.111/src/item.rs:2563-2657`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2563-L2657)*

### `parse_impl_item_fn`

```rust
fn parse_impl_item_fn(input: crate::parse::ParseStream<'_>, allow_omitted_body: bool) -> crate::error::Result<Option<crate::item::ImplItemFn>>
```

*Defined in [`syn-2.0.111/src/item.rs:2800-2831`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2800-L2831)*

### `parse_impl_item_type`

```rust
fn parse_impl_item_type(begin: crate::parse::ParseBuffer<'_>, input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::item::ImplItem>
```

*Defined in [`syn-2.0.111/src/item.rs:2860-2893`](../../../../.source_1765521767/syn-2.0.111/src/item.rs#L2860-L2893)*

