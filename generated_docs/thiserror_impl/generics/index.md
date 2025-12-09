*[thiserror_impl](../index.md) / [generics](index.md)*

---

# Module `generics`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParamsInScope`](#paramsinscope) | struct |  |
| [`InferredBounds`](#inferredbounds) | struct |  |
| [`crawl`](#crawl) | fn |  |

## Structs

### `ParamsInScope<'a>`

```rust
struct ParamsInScope<'a> {
    names: std::collections::BTreeSet<&'a syn::Ident>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/generics.rs:8-10`](../../../.source_1765210505/thiserror-impl-2.0.17/src/generics.rs#L8-L10)*

#### Implementations

- <span id="paramsinscope-new"></span>`fn new(generics: &'a Generics) -> Self`

- <span id="paramsinscope-intersects"></span>`fn intersects(&self, ty: &Type) -> bool`

### `InferredBounds`

```rust
struct InferredBounds {
    bounds: std::collections::BTreeMap<String, (std::collections::BTreeSet<String>, syn::punctuated::Punctuated<proc_macro2::TokenStream, token::Plus>)>,
    order: Vec<proc_macro2::TokenStream>,
}
```

*Defined in [`thiserror-impl-2.0.17/src/generics.rs:48-51`](../../../.source_1765210505/thiserror-impl-2.0.17/src/generics.rs#L48-L51)*

#### Implementations

- <span id="inferredbounds-new"></span>`fn new() -> Self`

- <span id="inferredbounds-insert"></span>`fn insert(&mut self, ty: impl ToTokens, bound: impl ToTokens)`

- <span id="inferredbounds-augment-where-clause"></span>`fn augment_where_clause(&self, generics: &Generics) -> WhereClause`

## Functions

### `crawl`

```rust
fn crawl(in_scope: &ParamsInScope<'_>, ty: &syn::Type, found: &mut bool)
```

*Defined in [`thiserror-impl-2.0.17/src/generics.rs:26-46`](../../../.source_1765210505/thiserror-impl-2.0.17/src/generics.rs#L26-L46)*

