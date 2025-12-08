*[thiserror_impl](../index.md) / [prop](index.md)*

---

# Module `prop`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`from_field`](#from_field) | fn |  |
| [`source_field`](#source_field) | fn |  |
| [`backtrace_field`](#backtrace_field) | fn |  |
| [`distinct_backtrace_field`](#distinct_backtrace_field) | fn |  |
| [`type_is_backtrace`](#type_is_backtrace) | fn |  |

## Functions

### `from_field`

```rust
fn from_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```

### `source_field`

```rust
fn source_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```

### `backtrace_field`

```rust
fn backtrace_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```

### `distinct_backtrace_field`

```rust
fn distinct_backtrace_field<'a, 'b>(backtrace_field: &'a crate::ast::Field<'b>, from_field: Option<&crate::ast::Field<'_>>) -> Option<&'a crate::ast::Field<'b>>
```

### `type_is_backtrace`

```rust
fn type_is_backtrace(ty: &syn::Type) -> bool
```

