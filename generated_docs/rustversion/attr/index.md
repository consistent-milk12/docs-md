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

*Defined in [`rustversion-1.0.22/src/attr.rs:6-9`](../../../.source_1765521767/rustversion-1.0.22/src/attr.rs#L6-L9)*

## Enums

### `Then`

```rust
enum Then {
    Const(proc_macro::Span),
    Attribute(proc_macro::TokenStream),
}
```

*Defined in [`rustversion-1.0.22/src/attr.rs:11-14`](../../../.source_1765521767/rustversion-1.0.22/src/attr.rs#L11-L14)*

## Functions

### `parse`

```rust
fn parse(input: proc_macro::TokenStream) -> std::result::Result<Args, Error>
```

*Defined in [`rustversion-1.0.22/src/attr.rs:16-35`](../../../.source_1765521767/rustversion-1.0.22/src/attr.rs#L16-L35)*

