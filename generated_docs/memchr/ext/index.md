*[memchr](../index.md) / [ext](index.md)*

---

# Module `ext`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Pointer`](#pointer) | trait | A trait for adding some helper routines to pointers. |

## Traits

### `Pointer`

```rust
trait Pointer { ... }
```

A trait for adding some helper routines to pointers.

#### Required Methods

- `fn distance(self, origin: Self) -> usize`

  Returns the distance, in units of `T`, between `self` and `origin`.

- `fn as_usize(self) -> usize`

  Casts this pointer to `usize`.

