*[allocator_api2](../../index.md) / [stable](../index.md) / [slice](index.md)*

---

# Module `slice`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SliceExt`](#sliceext) | trait | Slice methods that use `Box` and `Vec` from this crate. |

## Traits

### `SliceExt<T>`

```rust
trait SliceExt<T> { ... }
```

Slice methods that use `Box` and `Vec` from this crate.

#### Required Methods

- `fn to_vec_in<A: Allocator>(&self, alloc: A) -> Vec<T, A>`

  Copies `self` into a new `Vec` with an allocator.

- `fn repeat(&self, n: usize) -> Vec<T, Global>`

  Creates a vector by copying a slice `n` times.

#### Provided Methods

- `fn to_vec(&self) -> Vec<T, Global>`

  Copies `self` into a new `Vec`.

#### Implementors

- `[T]`

