*[serde_derive](../index.md) / [bound](index.md)*

---

# Module `bound`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`without_defaults`](#without_defaults) | fn |  |
| [`with_where_predicates`](#with_where_predicates) | fn |  |
| [`with_where_predicates_from_fields`](#with_where_predicates_from_fields) | fn |  |
| [`with_where_predicates_from_variants`](#with_where_predicates_from_variants) | fn |  |
| [`with_bound`](#with_bound) | fn |  |
| [`with_self_bound`](#with_self_bound) | fn |  |
| [`with_lifetime_bound`](#with_lifetime_bound) | fn |  |
| [`type_of_item`](#type_of_item) | fn |  |

## Functions

### `without_defaults`

```rust
fn without_defaults(generics: &syn::Generics) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:11-27`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L11-L27)*

### `with_where_predicates`

```rust
fn with_where_predicates(generics: &syn::Generics, predicates: &[syn::WherePredicate]) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:29-39`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L29-L39)*

### `with_where_predicates_from_fields`

```rust
fn with_where_predicates_from_fields(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, from_field: fn(&attr::Field) -> Option<&[syn::WherePredicate]>) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:41-55`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L41-L55)*

### `with_where_predicates_from_variants`

```rust
fn with_where_predicates_from_variants(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, from_variant: fn(&attr::Variant) -> Option<&[syn::WherePredicate]>) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:57-77`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L57-L77)*

### `with_bound`

```rust
fn with_bound(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, filter: fn(&attr::Field, Option<&attr::Variant>) -> bool, bound: &syn::Path) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:91-310`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L91-L310)*

### `with_self_bound`

```rust
fn with_self_bound(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, bound: &syn::Path) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:312-337`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L312-L337)*

### `with_lifetime_bound`

```rust
fn with_lifetime_bound(generics: &syn::Generics, lifetime: &str) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:339-370`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L339-L370)*

### `type_of_item`

```rust
fn type_of_item(cont: &crate::internals::ast::Container<'_>) -> syn::Type
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:372-410`](../../../.source_1765210505/serde_derive-1.0.228/src/bound.rs#L372-L410)*

