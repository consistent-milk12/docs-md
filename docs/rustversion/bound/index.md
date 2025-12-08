*[rustversion](../index.md) / [bound](index.md)*

---

# Module `bound`

## Enums

### `Bound`

```rust
enum Bound {
    Nightly(crate::date::Date),
    Stable(crate::release::Release),
}
```

## Functions

### `parse`

```rust
fn parse(paren: proc_macro::Group, iter: &'_ mut IterImpl) -> std::result::Result<Bound, Error>
```

