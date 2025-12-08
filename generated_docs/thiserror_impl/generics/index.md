*[thiserror_impl](../index.md) / [generics](index.md)*

---

# Module `generics`

## Structs

### `ParamsInScope<'a>`

```rust
struct ParamsInScope<'a> {
    names: std::collections::BTreeSet<&'a syn::Ident>,
}
```

#### Implementations

- `fn new(generics: &'a Generics) -> Self`

- `fn intersects(self: &Self, ty: &Type) -> bool`

### `InferredBounds`

```rust
struct InferredBounds {
    bounds: std::collections::BTreeMap<String, (std::collections::BTreeSet<String>, syn::punctuated::Punctuated<proc_macro2::TokenStream, $crate::token::Plus>)>,
    order: Vec<proc_macro2::TokenStream>,
}
```

#### Implementations

- `fn new() -> Self`

- `fn insert(self: &mut Self, ty: impl ToTokens, bound: impl ToTokens)`

- `fn augment_where_clause(self: &Self, generics: &Generics) -> WhereClause`

## Functions

### `crawl`

```rust
fn crawl(in_scope: &ParamsInScope<'_>, ty: &syn::Type, found: &mut bool)
```

