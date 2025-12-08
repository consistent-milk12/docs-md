*[rustversion](../index.md) / [expr](index.md)*

---

# Module `expr`

## Contents

- [Enums](#enums)
  - [`Expr`](#expr)
- [Functions](#functions)
  - [`parse`](#parse)
  - [`parse_nightly`](#parse_nightly)
  - [`parse_stable`](#parse_stable)
  - [`parse_since`](#parse_since)
  - [`parse_before`](#parse_before)
  - [`parse_not`](#parse_not)
  - [`parse_any`](#parse_any)
  - [`parse_all`](#parse_all)
  - [`parse_comma_separated`](#parse_comma_separated)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Expr`](#expr) | enum |  |
| [`parse`](#parse) | fn |  |
| [`parse_nightly`](#parse_nightly) | fn |  |
| [`parse_stable`](#parse_stable) | fn |  |
| [`parse_since`](#parse_since) | fn |  |
| [`parse_before`](#parse_before) | fn |  |
| [`parse_not`](#parse_not) | fn |  |
| [`parse_any`](#parse_any) | fn |  |
| [`parse_all`](#parse_all) | fn |  |
| [`parse_comma_separated`](#parse_comma_separated) | fn |  |

## Enums

### `Expr`

```rust
enum Expr {
    Stable,
    Beta,
    Nightly,
    Date(crate::date::Date),
    Since(crate::bound::Bound),
    Before(crate::bound::Bound),
    Release(crate::release::Release),
    Not(Box<Expr>),
    Any(Vec<Expr>),
    All(Vec<Expr>),
}
```

#### Implementations

- <span id="expr-eval"></span>`fn eval(&self, rustc: Version) -> bool` — [`Version`](../version/index.md)

## Functions

### `parse`

```rust
fn parse(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_nightly`

```rust
fn parse_nightly(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_stable`

```rust
fn parse_stable(iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_since`

```rust
fn parse_since(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_before`

```rust
fn parse_before(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_not`

```rust
fn parse_not(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_any`

```rust
fn parse_any(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_all`

```rust
fn parse_all(introducer: &proc_macro::Ident, iter: &'_ mut IterImpl) -> std::result::Result<Expr, Error>
```

### `parse_comma_separated`

```rust
fn parse_comma_separated(iter: &'_ mut IterImpl) -> std::result::Result<Vec<Expr>, Error>
```

