*[syn](../../index.md) / [path](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`const_argument`](#const_argument) | fn |  |
| [`qpath`](#qpath) | fn |  |

## Functions

### `const_argument`

```rust
fn const_argument(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

### `qpath`

```rust
fn qpath(input: crate::parse::ParseStream<'_>, expr_style: bool) -> crate::error::Result<(Option<crate::path::QSelf>, crate::path::Path)>
```

