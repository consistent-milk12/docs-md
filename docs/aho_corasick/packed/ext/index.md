*[aho_corasick](../../index.md) / [packed](../index.md) / [ext](index.md)*

---

# Module `ext`

## Traits

### `Pointer`

```rust
trait Pointer { ... }
```

A trait for adding some helper routines to pointers.

#### Required Methods

- `fn distance(self: Self, origin: Self) -> usize`

  Returns the distance, in units of `T`, between `self` and `origin`.

- `fn as_usize(self: Self) -> usize`

  Casts this pointer to `usize`.

