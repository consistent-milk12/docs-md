*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [into_iter](index.md)*

---

# Module `into_iter`

## Structs

### `IntoIter<T, A: Allocator>`

```rust
struct IntoIter<T, A: Allocator> {
    buf: core::ptr::NonNull<T>,
    phantom: core::marker::PhantomData<T>,
    cap: usize,
    alloc: core::mem::ManuallyDrop<A>,
    ptr: *const T,
    end: *const T,
}
```

An iterator that moves out of a vector.

This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
(provided by the `IntoIterator` trait).

# Example

```rust
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

#### Implementations

- `fn as_slice(self: &Self) -> &[T]`

- `fn as_mut_slice(self: &mut Self) -> &mut [T]`

- `fn allocator(self: &Self) -> &A`

- `fn as_raw_mut_slice(self: &mut Self) -> *mut [T]`

#### Trait Implementations

##### `impl<T, A: Allocator> AsRef for IntoIter<T, A>`

- `fn as_ref(self: &Self) -> &[T]`

##### `impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A>`

- `fn clone(self: &Self) -> Self`

##### `impl<T: fmt::Debug, A: Allocator> Debug for IntoIter<T, A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A>`

- `fn next_back(self: &mut Self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for IntoIter<T, A>`

- `fn drop(self: &mut Self)`

##### `impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for IntoIter<T, A>`

##### `impl<I> IntoIterator for IntoIter<T, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<T, A: Allocator> Iterator for IntoIter<T, A>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<T>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn count(self: Self) -> usize`

##### `impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A>`

##### `impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A>`

