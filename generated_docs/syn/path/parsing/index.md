*[syn](../../index.md) / [path](../index.md) / [parsing](index.md)*

---

# Module `parsing`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`const_argument`](#const-argument) | fn |  |
| [`qpath`](#qpath) | fn |  |

## Functions

### `const_argument`

```rust
fn const_argument(input: crate::parse::ParseStream<'_>) -> crate::error::Result<crate::expr::Expr>
```

*Defined in [`syn-2.0.111/src/path.rs:409-445`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L409-L445)*

### `qpath`

```rust
fn qpath(input: crate::parse::ParseStream<'_>, expr_style: bool) -> crate::error::Result<(Option<crate::path::QSelf>, crate::path::Path)>
```

*Defined in [`syn-2.0.111/src/path.rs:646-696`](../../../../.source_1765210505/syn-2.0.111/src/path.rs#L646-L696)*

