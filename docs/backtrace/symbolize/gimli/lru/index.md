*[backtrace](../../../index.md) / [symbolize](../../index.md) / [gimli](../index.md) / [lru](index.md)*

---

# Module `lru`

## Structs

### `Lru<T, const N: usize>`

```rust
struct Lru<T, const N: usize> {
    len: usize,
    arr: [core::mem::MaybeUninit<T>; N],
}
```

least-recently-used cache with static size

#### Implementations

- `fn clear(self: &mut Self)`

- `fn iter(self: &Self) -> impl Iterator<Item = &T>`

- `fn push_front(self: &mut Self, value: T) -> Option<&mut T>`

- `fn move_to_front(self: &mut Self, idx: usize) -> Option<&mut T>`

#### Trait Implementations

##### `impl<T, const N: usize> Default for Lru<T, N>`

- `fn default() -> Self`

