*[rustix](../../index.md) / [buffer](../index.md) / [private](index.md)*

---

# Module `private`

## Traits

### `Sealed<T>`

```rust
trait Sealed<T> { ... }
```

#### Required Methods

- `type Output`

- `fn parts_mut(self: &mut Self) -> (*mut T, usize)`

  Return a pointer and length for this buffer.

- `fn assume_init(self: Self, len: usize) -> <Self as >::Output`

  Convert a finished buffer pointer into its result.

