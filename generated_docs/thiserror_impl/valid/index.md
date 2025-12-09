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

*Defined in [`thiserror-impl-2.0.17/src/valid.rs:109-149`](../../../.source_1765210505/thiserror-impl-2.0.17/src/valid.rs#L109-L149)*

### `check_field_attrs`

```rust
fn check_field_attrs(fields: &[crate::ast::Field<'_>]) -> syn::Result<()>
```

*Defined in [`thiserror-impl-2.0.17/src/valid.rs:151-222`](../../../.source_1765210505/thiserror-impl-2.0.17/src/valid.rs#L151-L222)*

### `contains_non_static_lifetime`

```rust
fn contains_non_static_lifetime(ty: &syn::Type) -> bool
```

*Defined in [`thiserror-impl-2.0.17/src/valid.rs:224-248`](../../../.source_1765210505/thiserror-impl-2.0.17/src/valid.rs#L224-L248)*

