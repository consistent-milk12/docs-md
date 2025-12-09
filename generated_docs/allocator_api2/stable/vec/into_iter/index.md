*[allocator_api2](../../../index.md) / [stable](../../index.md) / [vec](../index.md) / [into_iter](index.md)*

---

# Module `into_iter`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`IntoIter`](#intoiter) | struct | An iterator that moves out of a vector. |

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

*Defined in [`allocator-api2-0.2.21/src/stable/vec/into_iter.rs:27-36`](../../../../../.source_1765210505/allocator-api2-0.2.21/src/stable/vec/into_iter.rs#L27-L36)*

An iterator that moves out of a vector.

This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
(provided by the `IntoIterator` trait).

# Example

```rust
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

#### Implementations

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &[T]`

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

- <span id="intoiter-allocator"></span>`fn allocator(&self) -> &A`

- <span id="intoiter-as-raw-mut-slice"></span>`fn as_raw_mut_slice(&mut self) -> *mut [T]`

#### Trait Implementations

##### `impl<T, A: Allocator> AsRef for IntoIter<T, A>`

- <span id="intoiter-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T: Clone, A: Allocator + Clone> Clone for IntoIter<T, A>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T: fmt::Debug, A: Allocator> Debug for IntoIter<T, A>`

- <span id="intoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, A: Allocator> DoubleEndedIterator for IntoIter<T, A>`

- <span id="intoiter-next-back"></span>`fn next_back(&mut self) -> Option<T>`

##### `impl<T, A: Allocator> Drop for IntoIter<T, A>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for IntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for IntoIter<T, A>`

##### `impl<I> IntoIterator for IntoIter<T, A>`

- <span id="intoiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for IntoIter<T, A>`

- <span id="intoiter-type-item"></span>`type Item = T`

- <span id="intoiter-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="intoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="intoiter-count"></span>`fn count(self) -> usize`

##### `impl<T: Send, A: Allocator + Send> Send for IntoIter<T, A>`

##### `impl<T: Sync, A: Allocator + Sync> Sync for IntoIter<T, A>`

