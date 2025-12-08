*[syn](../../index.md) / [lit](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parse_negative_lit`](#parse_negative_lit) | fn |  |
| [`peek_impl`](#peek_impl) | fn |  |
| [`impl_token!`](#impl_token) | macro |  |

## Functions

### `parse_negative_lit`

```rust
fn parse_negative_lit(neg: proc_macro2::Punct, cursor: crate::buffer::Cursor<'_>) -> Option<(crate::lit::Lit, crate::buffer::Cursor<'_>)>
```

### `peek_impl`

```rust
fn peek_impl(cursor: crate::buffer::Cursor<'_>, peek: fn(crate::parse::ParseStream<'_>) -> bool) -> bool
```

## Macros

### `impl_token!`

