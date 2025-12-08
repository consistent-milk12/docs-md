*[rustversion](../index.md) / [expr](index.md)*

---

# Module `expr`

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

- `fn eval(self: &Self, rustc: Version) -> bool` — [`Version`](../version/index.md)

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

