*[allocator_api2](../../index.md) / [stable](../index.md) / [slice](index.md)*

---

# Module `slice`

## Traits

### `SliceExt<T>`

```rust
trait SliceExt<T> { ... }
```

Slice methods that use `Box` and `Vec` from this crate.

#### Required Methods

- `fn to_vec(self: &Self) -> Vec<T, Global>`

  Copies `self` into a new `Vec`.

- `fn to_vec_in<A: Allocator>(self: &Self, alloc: A) -> Vec<T, A>`

  Copies `self` into a new `Vec` with an allocator.

- `fn repeat(self: &Self, n: usize) -> Vec<T, Global>`

  Creates a vector by copying a slice `n` times.

