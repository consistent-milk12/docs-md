*[tinyvec](../index.md) / [slicevec](index.md)*

---

# Module `slicevec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SliceVec`](#slicevec) | struct | A slice-backed vector-like data structure. |
| [`SliceVecDrain`](#slicevecdrain) | struct | Draining iterator for [`SliceVec`] |

## Structs

### `SliceVec<'s, T>`

```rust
struct SliceVec<'s, T> {
    data: &'s mut [T],
    len: usize,
}
```

A slice-backed vector-like data structure.

This is a very similar concept to `ArrayVec`, but instead
of the backing memory being an owned array, the backing
memory is a unique-borrowed slice. You can thus create
one of these structures "around" some slice that you're
working with to make it easier to manipulate.

* Has a fixed capacity (the initial slice size).
* Has a variable length.

#### Implementations

- <span id="slicevec-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="slicevec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut T`

- <span id="slicevec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

- <span id="slicevec-as-ptr"></span>`fn as_ptr(&self) -> *const T`

- <span id="slicevec-as-slice"></span>`fn as_slice(&self) -> &[T]`

- <span id="slicevec-capacity"></span>`fn capacity(&self) -> usize`

- <span id="slicevec-clear"></span>`fn clear(&mut self)`

- <span id="slicevec-drain"></span>`fn drain<'p, R: RangeBounds<usize>>(self: &'p mut Self, range: R) -> SliceVecDrain<'p, 's, T>` — [`SliceVecDrain`](../index.md)

- <span id="slicevec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[T])`

- <span id="slicevec-fill"></span>`fn fill<I: IntoIterator<Item = T>>(&mut self, iter: I) -> <I as >::IntoIter`

- <span id="slicevec-from-slice-len"></span>`fn from_slice_len(data: &'s mut [T], len: usize) -> Self`

- <span id="slicevec-insert"></span>`fn insert(&mut self, index: usize, item: T)`

- <span id="slicevec-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="slicevec-len"></span>`fn len(&self) -> usize`

- <span id="slicevec-pop"></span>`fn pop(&mut self) -> Option<T>`

- <span id="slicevec-push"></span>`fn push(&mut self, val: T)`

- <span id="slicevec-remove"></span>`fn remove(&mut self, index: usize) -> T`

- <span id="slicevec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: T)`

- <span id="slicevec-resize-with"></span>`fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, f: F)`

- <span id="slicevec-retain"></span>`fn retain<F: FnMut(&T) -> bool>(&mut self, acceptable: F)`

- <span id="slicevec-set-len"></span>`fn set_len(&mut self, new_len: usize)`

- <span id="slicevec-split-off"></span>`fn split_off<'a>(self: &'a mut Self, at: usize) -> SliceVec<'s, T>` — [`SliceVec`](../index.md)

- <span id="slicevec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> T`

- <span id="slicevec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

- <span id="slicevec-try-from-slice-len"></span>`fn try_from_slice_len(data: &'s mut [T], len: usize) -> Option<Self>`

#### Trait Implementations

##### `impl<'s, T> AsMut for SliceVec<'s, T>`

- <span id="slicevec-as-mut"></span>`fn as_mut(&mut self) -> &mut [T]`

##### `impl<'s, T> AsRef for SliceVec<'s, T>`

- <span id="slicevec-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<'s, T> Binary for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Debug for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Default for SliceVec<'s, T>`

- <span id="slicevec-default"></span>`fn default() -> Self`

##### `impl<'s, T> Deref for SliceVec<'s, T>`

- <span id="slicevec-target"></span>`type Target = [T]`

- <span id="slicevec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<'s, T> DerefMut for SliceVec<'s, T>`

- <span id="slicevec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<'s, T> Display for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Eq for SliceVec<'s, T>`

##### `impl<'s, T> Extend for SliceVec<'s, T>`

- <span id="slicevec-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<'s, T> Hash for SliceVec<'s, T>`

- <span id="slicevec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<'s, T, I> Index for SliceVec<'s, T>`

- <span id="slicevec-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="slicevec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<'s, T, I> IndexMut for SliceVec<'s, T>`

- <span id="slicevec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<'s, T> IntoIterator for SliceVec<'s, T>`

- <span id="slicevec-item"></span>`type Item = &'s mut T`

- <span id="slicevec-intoiter"></span>`type IntoIter = IterMut<'s, T>`

- <span id="slicevec-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<'s, T> LowerExp for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> LowerHex for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Octal for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Ord for SliceVec<'s, T>`

- <span id="slicevec-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<'s, T> PartialEq for SliceVec<'s, T>`

- <span id="slicevec-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<'s, T> PartialOrd for SliceVec<'s, T>`

- <span id="slicevec-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<'s, T> Pointer for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<P, T> Receiver for SliceVec<'s, T>`

- <span id="slicevec-target"></span>`type Target = T`

##### `impl<T> ToString for SliceVec<'s, T>`

- <span id="slicevec-to-string"></span>`fn to_string(&self) -> String`

##### `impl<'s, T> UpperExp for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> UpperHex for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `SliceVecDrain<'p, 's, T: Default>`

```rust
struct SliceVecDrain<'p, 's, T: Default> {
    parent: &'p mut SliceVec<'s, T>,
    target_start: usize,
    target_index: usize,
    target_end: usize,
}
```

Draining iterator for [`SliceVec`](../index.md)

See [`SliceVec::drain`](SliceVec::drain)

#### Trait Implementations

##### `impl<'p, 's, T: Default> Drop for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-drop"></span>`fn drop(&mut self)`

##### `impl<'p, 's, T: Default> FusedIterator for SliceVecDrain<'p, 's, T>`

##### `impl<I> IntoIterator for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slicevecdrain-intoiter"></span>`type IntoIter = I`

- <span id="slicevecdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'p, 's, T: Default> Iterator for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-item"></span>`type Item = T`

- <span id="slicevecdrain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

