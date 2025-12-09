*[aho_corasick](../../index.md) / [packed](../index.md) / [ext](index.md)*

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

*Defined in [`aho-corasick-1.1.4/src/packed/ext.rs:2-18`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/ext.rs#L2-L18)*

A trait for adding some helper routines to pointers.

#### Required Methods

- `fn distance(self, origin: Self) -> usize`

  Returns the distance, in units of `T`, between `self` and `origin`.

- `fn as_usize(self) -> usize`

  Casts this pointer to `usize`.

#### Implementors

- `*const T`
- `*mut T`

