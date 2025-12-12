*[syn](../../index.md) / [lit](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse_negative_lit`](#parse-negative-lit) | fn |  |
| [`peek_impl`](#peek-impl) | fn |  |
| [`impl_token!`](#impl-token) | macro |  |

## Functions

### `parse_negative_lit`

```rust
fn parse_negative_lit(neg: proc_macro2::Punct, cursor: crate::buffer::Cursor<'_>) -> Option<(crate::lit::Lit, crate::buffer::Cursor<'_>)>
```

*Defined in [`syn-2.0.111/src/lit.rs:904-941`](../../../../.source_1765521767/syn-2.0.111/src/lit.rs#L904-L941)*

### `peek_impl`

```rust
fn peek_impl(cursor: crate::buffer::Cursor<'_>, peek: fn(crate::parse::ParseStream<'_>) -> bool) -> bool
```

*Defined in [`syn-2.0.111/src/lit.rs:1031-1036`](../../../../.source_1765521767/syn-2.0.111/src/lit.rs#L1031-L1036)*

## Macros

### `impl_token!`

*Defined in [`syn-2.0.111/src/lit.rs:1038-1055`](../../../../.source_1765521767/syn-2.0.111/src/lit.rs#L1038-L1055)*

