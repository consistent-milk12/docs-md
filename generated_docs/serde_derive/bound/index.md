*[serde_derive](../index.md) / [bound](index.md)*

---

# Module `bound`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`without_defaults`](#without-defaults) | fn |  |
| [`with_where_predicates`](#with-where-predicates) | fn |  |
| [`with_where_predicates_from_fields`](#with-where-predicates-from-fields) | fn |  |
| [`with_where_predicates_from_variants`](#with-where-predicates-from-variants) | fn |  |
| [`with_bound`](#with-bound) | fn |  |
| [`with_self_bound`](#with-self-bound) | fn |  |
| [`with_lifetime_bound`](#with-lifetime-bound) | fn |  |
| [`type_of_item`](#type-of-item) | fn |  |

## Functions

### `without_defaults`

```rust
fn without_defaults(generics: &syn::Generics) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:11-27`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L11-L27)*

### `with_where_predicates`

```rust
fn with_where_predicates(generics: &syn::Generics, predicates: &[syn::WherePredicate]) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:29-39`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L29-L39)*

### `with_where_predicates_from_fields`

```rust
fn with_where_predicates_from_fields(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, from_field: fn(&attr::Field) -> Option<&[syn::WherePredicate]>) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:41-55`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L41-L55)*

### `with_where_predicates_from_variants`

```rust
fn with_where_predicates_from_variants(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, from_variant: fn(&attr::Variant) -> Option<&[syn::WherePredicate]>) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:57-77`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L57-L77)*

### `with_bound`

```rust
fn with_bound(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, filter: fn(&attr::Field, Option<&attr::Variant>) -> bool, bound: &syn::Path) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:91-310`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L91-L310)*

### `with_self_bound`

```rust
fn with_self_bound(cont: &crate::internals::ast::Container<'_>, generics: &syn::Generics, bound: &syn::Path) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:312-337`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L312-L337)*

### `with_lifetime_bound`

```rust
fn with_lifetime_bound(generics: &syn::Generics, lifetime: &str) -> syn::Generics
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:339-370`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L339-L370)*

### `type_of_item`

```rust
fn type_of_item(cont: &crate::internals::ast::Container<'_>) -> syn::Type
```

*Defined in [`serde_derive-1.0.228/src/bound.rs:372-410`](../../../.source_1765521767/serde_derive-1.0.228/src/bound.rs#L372-L410)*

