*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [drain](index.md)*

---

# Module `drain`

## Structs

### `Drain<'a, T: 'a, A: Allocator + 'a>`

```rust
struct Drain<'a, T: 'a, A: Allocator + 'a> {
    tail_start: usize,
    tail_len: usize,
    iter: slice::Iter<'a, T>,
    vec: core::ptr::NonNull<super::Vec<T, A>>,
}
```

A draining iterator for `Vec<T>`.

This `struct` is created by `Vec::drain`.
See its documentation for more.

# Example

```rust
let mut v = vec![0, 1, 2];
let iter: std::vec::Drain<_> = v.drain(..);
```

#### Fields

- **`tail_start`**: `usize`

  Index of tail to preserve

- **`tail_len`**: `usize`

  Length of tail

- **`iter`**: `slice::Iter<'a, T>`

  Current remaining range to remove

#### Implementations

- `fn as_slice(self: &Self) -> &[T]`

- `fn allocator(self: &Self) -> &A`

- `fn keep_rest(self: Self)`

#### Trait Implementations

##### `impl<'a, T, A: Allocator> AsRef for Drain<'a, T, A>`

- `fn as_ref(self: &Self) -> &[T]`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A>`

- `fn next_back(self: &mut Self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for Drain<'_, T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<I> IntoIterator for Drain<'a, T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A>`

##### `impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A>`

