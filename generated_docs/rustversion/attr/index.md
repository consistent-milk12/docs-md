*[rustversion](../index.md) / [attr](index.md)*

---

# Module `attr`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Args`](#args) | struct |  |
| [`Then`](#then) | enum |  |
| [`parse`](#parse) | fn |  |

## Structs

### `Args`

```rust
struct Args {
    pub condition: crate::expr::Expr,
    pub then: Then,
}
```

## Enums

### `Then`

```rust
enum Then {
    Const(proc_macro::Span),
    Attribute(proc_macro::TokenStream),
}
```

## Functions

### `parse`

```rust
fn parse(input: proc_macro::TokenStream) -> std::result::Result<Args, Error>
```

