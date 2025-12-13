*[thiserror_impl](../index.md) / [prop](index.md)*

---

# Module `prop`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`from_field`](#from-field) | fn |  |
| [`source_field`](#source-field) | fn |  |
| [`backtrace_field`](#backtrace-field) | fn |  |
| [`distinct_backtrace_field`](#distinct-backtrace-field) | fn |  |
| [`type_is_backtrace`](#type-is-backtrace) | fn |  |

## Functions

### `from_field`

```rust
fn from_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```

*Defined in [`thiserror-impl-2.0.17/src/prop.rs:88-95`](../../../.source_1765633015/thiserror-impl-2.0.17/src/prop.rs#L88-L95)*

### `source_field`

```rust
fn source_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```

*Defined in [`thiserror-impl-2.0.17/src/prop.rs:97-110`](../../../.source_1765633015/thiserror-impl-2.0.17/src/prop.rs#L97-L110)*

### `backtrace_field`

```rust
fn backtrace_field<'a, 'b>(fields: &'a [crate::ast::Field<'b>]) -> Option<&'a crate::ast::Field<'b>>
```

*Defined in [`thiserror-impl-2.0.17/src/prop.rs:112-124`](../../../.source_1765633015/thiserror-impl-2.0.17/src/prop.rs#L112-L124)*

### `distinct_backtrace_field`

```rust
fn distinct_backtrace_field<'a, 'b>(backtrace_field: &'a crate::ast::Field<'b>, from_field: Option<&crate::ast::Field<'_>>) -> Option<&'a crate::ast::Field<'b>>
```

*Defined in [`thiserror-impl-2.0.17/src/prop.rs:127-138`](../../../.source_1765633015/thiserror-impl-2.0.17/src/prop.rs#L127-L138)*

### `type_is_backtrace`

```rust
fn type_is_backtrace(ty: &syn::Type) -> bool
```

*Defined in [`thiserror-impl-2.0.17/src/prop.rs:140-148`](../../../.source_1765633015/thiserror-impl-2.0.17/src/prop.rs#L140-L148)*

