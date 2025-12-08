*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [drain](index.md)*

---

# Module `drain`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | A draining iterator for `Vec<T>`. |

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

- <span id="drain-as-slice"></span>`fn as_slice(&self) -> &[T]`

- <span id="drain-allocator"></span>`fn allocator(&self) -> &A`

- <span id="drain-keep-rest"></span>`fn keep_rest(self)`

#### Trait Implementations

##### `impl<'a, T, A: Allocator> AsRef for Drain<'a, T, A>`

- <span id="drain-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: fmt::Debug, A: Allocator> Debug for Drain<'_, T, A>`

- <span id="drain-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for Drain<'_, T, A>`

- <span id="drain-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for Drain<'_, T, A>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for Drain<'_, T, A>`

##### `impl<T, A: Allocator> FusedIterator for Drain<'_, T, A>`

##### `impl<I> IntoIterator for Drain<'a, T, A>`

- <span id="drain-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiter"></span>`type IntoIter = I`

- <span id="drain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for Drain<'_, T, A>`

- <span id="drain-item"></span>`type Item = T`

- <span id="drain-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="drain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send, A: Send + Allocator> Send for Drain<'_, T, A>`

##### `impl<T: Sync, A: Sync + Allocator> Sync for Drain<'_, T, A>`

