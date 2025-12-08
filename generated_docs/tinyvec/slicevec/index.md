*[tinyvec](../index.md) / [slicevec](index.md)*

---

# Module `slicevec`

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

- `fn append(self: &mut Self, other: &mut Self)`

- `fn as_mut_ptr(self: &mut Self) -> *mut T`

- `fn as_mut_slice(self: &mut Self) -> &mut [T]`

- `fn as_ptr(self: &Self) -> *const T`

- `fn as_slice(self: &Self) -> &[T]`

- `fn capacity(self: &Self) -> usize`

- `fn clear(self: &mut Self)`

- `fn drain<'p, R: RangeBounds<usize>>(self: &'p mut Self, range: R) -> SliceVecDrain<'p, 's, T>` — [`SliceVecDrain`](../index.md)

- `fn extend_from_slice(self: &mut Self, sli: &[T])`

- `fn fill<I: IntoIterator<Item = T>>(self: &mut Self, iter: I) -> <I as >::IntoIter`

- `fn from_slice_len(data: &'s mut [T], len: usize) -> Self`

- `fn insert(self: &mut Self, index: usize, item: T)`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn pop(self: &mut Self) -> Option<T>`

- `fn push(self: &mut Self, val: T)`

- `fn remove(self: &mut Self, index: usize) -> T`

- `fn resize(self: &mut Self, new_len: usize, new_val: T)`

- `fn resize_with<F: FnMut() -> T>(self: &mut Self, new_len: usize, f: F)`

- `fn retain<F: FnMut(&T) -> bool>(self: &mut Self, acceptable: F)`

- `fn set_len(self: &mut Self, new_len: usize)`

- `fn split_off<'a>(self: &'a mut Self, at: usize) -> SliceVec<'s, T>` — [`SliceVec`](../index.md)

- `fn swap_remove(self: &mut Self, index: usize) -> T`

- `fn truncate(self: &mut Self, new_len: usize)`

- `fn try_from_slice_len(data: &'s mut [T], len: usize) -> Option<Self>`

#### Trait Implementations

##### `impl<'s, T> AsMut for SliceVec<'s, T>`

- `fn as_mut(self: &mut Self) -> &mut [T]`

##### `impl<'s, T> AsRef for SliceVec<'s, T>`

- `fn as_ref(self: &Self) -> &[T]`

##### `impl<'s, T> Binary for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Debug for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Default for SliceVec<'s, T>`

- `fn default() -> Self`

##### `impl<'s, T> Deref for SliceVec<'s, T>`

- `type Target = [T]`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<'s, T> DerefMut for SliceVec<'s, T>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<'s, T> Display for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Eq for SliceVec<'s, T>`

##### `impl<'s, T> Extend for SliceVec<'s, T>`

- `fn extend<I: IntoIterator<Item = T>>(self: &mut Self, iter: I)`

##### `impl<'s, T> Hash for SliceVec<'s, T>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<'s, T, I> Index for SliceVec<'s, T>`

- `type Output = <I as SliceIndex>::Output`

- `fn index(self: &Self, index: I) -> &<Self as >::Output`

##### `impl<'s, T, I> IndexMut for SliceVec<'s, T>`

- `fn index_mut(self: &mut Self, index: I) -> &mut <Self as >::Output`

##### `impl<'s, T> IntoIterator for SliceVec<'s, T>`

- `type Item = &'s mut T`

- `type IntoIter = IterMut<'s, T>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<'s, T> LowerExp for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> LowerHex for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Octal for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> Ord for SliceVec<'s, T>`

- `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`

##### `impl<'s, T> PartialEq for SliceVec<'s, T>`

- `fn eq(self: &Self, other: &&[T]) -> bool`

##### `impl<'s, T> PartialOrd for SliceVec<'s, T>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<'s, T> Pointer for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<P, T> Receiver for SliceVec<'s, T>`

- `type Target = T`

##### `impl<T> ToString for SliceVec<'s, T>`

- `fn to_string(self: &Self) -> String`

##### `impl<'s, T> UpperExp for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<'s, T> UpperHex for SliceVec<'s, T>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

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

- `fn drop(self: &mut Self)`

##### `impl<'p, 's, T: Default> FusedIterator for SliceVecDrain<'p, 's, T>`

##### `impl<I> IntoIterator for SliceVecDrain<'p, 's, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'p, 's, T: Default> Iterator for SliceVecDrain<'p, 's, T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

