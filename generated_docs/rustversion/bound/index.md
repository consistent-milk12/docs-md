*[rustversion](../index.md) / [bound](index.md)*

---

# Module `bound`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Bound`](#bound) | enum |  |
| [`parse`](#parse) | fn |  |

## Enums

### `Bound`

```rust
enum Bound {
    Nightly(crate::date::Date),
    Stable(crate::release::Release),
}
```

*Defined in [`rustversion-1.0.22/src/bound.rs:10-13`](../../../.source_1765210505/rustversion-1.0.22/src/bound.rs#L10-L13)*

#### Trait Implementations

##### `impl PartialEq for crate::version::Version`

- <span id="crateversionversion-eq"></span>`fn eq(&self, rhs: &Bound) -> bool` — [`Bound`](#bound)

##### `impl PartialOrd for crate::version::Version`

- <span id="crateversionversion-partial-cmp"></span>`fn partial_cmp(&self, rhs: &Bound) -> Option<Ordering>` — [`Bound`](#bound)

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Bound, Error>
```

*Defined in [`rustversion-1.0.22/src/bound.rs:15-31`](../../../.source_1765210505/rustversion-1.0.22/src/bound.rs#L15-L31)*

