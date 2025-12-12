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

*Defined in [`memchr-2.7.6/src/ext.rs:2-18`](../../../.source_1765521767/memchr-2.7.6/src/ext.rs#L2-L18)*

A trait for adding some helper routines to pointers.

#### Required Methods

- `fn distance(self, origin: Self) -> usize`

  Returns the distance, in units of `T`, between `self` and `origin`.

- `fn as_usize(self) -> usize`

  Casts this pointer to `usize`.

#### Implementors

- `*const T`
- `*mut T`

