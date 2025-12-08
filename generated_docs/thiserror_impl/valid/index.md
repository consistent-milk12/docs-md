*[thiserror_impl](../index.md) / [valid](index.md)*

---

# Module `valid`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`check_non_field_attrs`](#check_non_field_attrs) | fn |  |
| [`check_field_attrs`](#check_field_attrs) | fn |  |
| [`contains_non_static_lifetime`](#contains_non_static_lifetime) | fn |  |

## Functions

### `check_non_field_attrs`

```rust
fn check_non_field_attrs(attrs: &crate::attr::Attrs<'_>) -> syn::Result<()>
```

### `check_field_attrs`

```rust
fn check_field_attrs(fields: &[crate::ast::Field<'_>]) -> syn::Result<()>
```

### `contains_non_static_lifetime`

```rust
fn contains_non_static_lifetime(ty: &syn::Type) -> bool
```

