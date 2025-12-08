*[serde_derive](../index.md) / [pretend](index.md)*

---

# Module `pretend`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pretend_used`](#pretend_used) | fn |  |
| [`pretend_fields_used`](#pretend_fields_used) | fn |  |
| [`pretend_fields_used_struct`](#pretend_fields_used_struct) | fn |  |
| [`pretend_fields_used_struct_packed`](#pretend_fields_used_struct_packed) | fn |  |
| [`pretend_fields_used_enum`](#pretend_fields_used_enum) | fn |  |
| [`pretend_variants_used`](#pretend_variants_used) | fn |  |

## Functions

### `pretend_used`

```rust
fn pretend_used(cont: &crate::internals::ast::Container<'_>, is_packed: bool) -> proc_macro2::TokenStream
```

### `pretend_fields_used`

```rust
fn pretend_fields_used(cont: &crate::internals::ast::Container<'_>, is_packed: bool) -> proc_macro2::TokenStream
```

### `pretend_fields_used_struct`

```rust
fn pretend_fields_used_struct(cont: &crate::internals::ast::Container<'_>, fields: &[crate::internals::ast::Field<'_>]) -> proc_macro2::TokenStream
```

### `pretend_fields_used_struct_packed`

```rust
fn pretend_fields_used_struct_packed(cont: &crate::internals::ast::Container<'_>, fields: &[crate::internals::ast::Field<'_>]) -> proc_macro2::TokenStream
```

### `pretend_fields_used_enum`

```rust
fn pretend_fields_used_enum(cont: &crate::internals::ast::Container<'_>, variants: &[crate::internals::ast::Variant<'_>]) -> proc_macro2::TokenStream
```

### `pretend_variants_used`

```rust
fn pretend_variants_used(cont: &crate::internals::ast::Container<'_>) -> proc_macro2::TokenStream
```

