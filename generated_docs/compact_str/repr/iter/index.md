*[compact_str](../../index.md) / [repr](../index.md) / [iter](index.md)*

---

# Module `iter`

Implementations of the `FromIterator` trait to make building [`Repr`](../index.md)s more ergonomic

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`from_as_ref_str_iterator`](#from_as_ref_str_iterator) | fn |  |

## Functions

### `from_as_ref_str_iterator`

```rust
fn from_as_ref_str_iterator<S, I>(iter: I) -> super::Repr
where
    S: AsRef<str>,
    I: Iterator<Item = S>,
    alloc::string::String: core::iter::Extend<S> + FromIterator<S>
```

