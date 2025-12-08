*[serde_derive](../index.md) / [bound](index.md)*

---

# Module `bound`

## Functions

### `without_defaults`

```rust
fn without_defaults(generics: &syn::Generics) -> syn::Generics
```

### `with_where_predicates`

```rust
fn with_where_predicates(generics: &syn::Generics, predicates: &[syn::WherePredicate]) -> syn::Generics
```

### `with_where_predicates_from_fields`

```rust
fn with_where_predicates_from_fields(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, from_field: fn(&attr::Field) -> Option<&[syn::WherePredicate]>) -> syn::Generics
```

### `with_where_predicates_from_variants`

```rust
fn with_where_predicates_from_variants(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, from_variant: fn(&attr::Variant) -> Option<&[syn::WherePredicate]>) -> syn::Generics
```

### `with_bound`

```rust
fn with_bound(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, filter: fn(&attr::Field, Option<&attr::Variant>) -> bool, bound: &syn::Path) -> syn::Generics
```

### `with_self_bound`

```rust
fn with_self_bound(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, bound: &syn::Path) -> syn::Generics
```

### `with_lifetime_bound`

```rust
fn with_lifetime_bound(generics: &syn::Generics, lifetime: &str) -> syn::Generics
```

### `type_of_item`

```rust
fn type_of_item(cont: &crate::internals::ast::Container<'_>) -> syn::Type
```

