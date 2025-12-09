*[tinyvec](../index.md) / [tinyvec](index.md)*

---

# Module `tinyvec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TinyVecSplice`](#tinyvecsplice) | struct | Splicing iterator for `TinyVec` |
| [`TinyVec`](#tinyvec) | enum | A vector that starts inline, but can automatically move to the heap. |
| [`TinyVecDrain`](#tinyvecdrain) | enum | Draining iterator for `TinyVecDrain` |
| [`TinyVecIterator`](#tinyveciterator) | enum | Iterator for consuming an `TinyVec` and returning owned elements. |

## Structs

### `TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut TinyVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

Splicing iterator for `TinyVec`
See [`TinyVec::splice`](TinyVec::<A>::splice)

#### Trait Implementations

##### `impl<'p, A, I> DoubleEndedIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md)

##### `impl<'p, A: Array, I: Iterator<Item = <A as >::Item>> Drop for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<'p, A, I> ExactSizeIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-len"></span>`fn len(&self) -> usize`

##### `impl<'p, A, I> FusedIterator for TinyVecSplice<'p, A, I>`

##### `impl<I> IntoIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecsplice-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecsplice-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'p, A, I> Iterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecsplice-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md)

- <span id="tinyvecsplice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `TinyVec<A: Array>`

```rust
enum TinyVec<A: Array> {
    Inline(ArrayVec<A>),
    Heap(alloc::vec::Vec<<A as >::Item>),
}
```

A vector that starts inline, but can automatically move to the heap.

* Requires the `alloc` feature

A `TinyVec` is either an Inline([`ArrayVec`](crate::ArrayVec::<A>)) or
Heap([`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)). The
interface for the type as a whole is a bunch of methods that just match on
the enum variant and then call the same method on the inner vec.

## Construction

Because it's an enum, you can construct a `TinyVec` simply by making an
`ArrayVec` or `Vec` and then putting it into the enum.

There is also a macro

```rust
use tinyvec::*;
let empty_tv = tiny_vec!([u8; 16]);
let some_ints = tiny_vec!([i32; 4] => 1, 2, 3);
```

#### Implementations

- <span id="tinyvec-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="tinyvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](../index.md)

- <span id="tinyvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md)

- <span id="tinyvec-remove"></span>`fn remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](../index.md)

- <span id="tinyvec-len"></span>`fn len(&self) -> usize`

- <span id="tinyvec-capacity"></span>`fn capacity(&self) -> usize`

- <span id="tinyvec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

- <span id="tinyvec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut <A as >::Item` — [`Array`](../index.md)

- <span id="tinyvec-as-ptr"></span>`fn as_ptr(&self) -> *const <A as >::Item` — [`Array`](../index.md)

- <span id="tinyvec-retain"></span>`fn retain<F: FnMut(&<A as >::Item) -> bool>(&mut self, acceptable: F)`

- <span id="tinyvec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md)

- <span id="tinyvec-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md)

- <span id="tinyvec-clear"></span>`fn clear(&mut self)`

- <span id="tinyvec-drain"></span>`fn drain<R: RangeBounds<usize>>(&mut self, range: R) -> TinyVecDrain<'_, A>` — [`TinyVecDrain`](../index.md)

- <span id="tinyvec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[<A as >::Item])` — [`Array`](../index.md)

- <span id="tinyvec-from-array-len"></span>`fn from_array_len(data: A, len: usize) -> Self`

- <span id="tinyvec-insert"></span>`fn insert(&mut self, index: usize, item: <A as >::Item)` — [`Array`](../index.md)

- <span id="tinyvec-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="tinyvec-new"></span>`fn new() -> Self`

- <span id="tinyvec-push"></span>`fn push(&mut self, val: <A as >::Item)` — [`Array`](../index.md)

- <span id="tinyvec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: <A as >::Item)` — [`Array`](../index.md)

- <span id="tinyvec-resize-with"></span>`fn resize_with<F: FnMut() -> <A as >::Item>(&mut self, new_len: usize, f: F)`

- <span id="tinyvec-split-off"></span>`fn split_off(&mut self, at: usize) -> Self`

- <span id="tinyvec-splice"></span>`fn splice<R, I>(&mut self, range: R, replacement: I) -> TinyVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`TinyVecSplice`](../index.md)

- <span id="tinyvec-try-from-array-len"></span>`fn try_from_array_len(data: A, len: usize) -> Result<Self, A>`

#### Trait Implementations

##### `impl<A: Array> AsMut for TinyVec<A>`

- <span id="tinyvec-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md)

##### `impl<A: Array> AsRef for TinyVec<A>`

- <span id="tinyvec-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](../index.md)

##### `impl<A: Array> Binary for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A> Clone for TinyVec<A>`

- <span id="tinyvec-clone"></span>`fn clone(&self) -> Self`

- <span id="tinyvec-clone-from"></span>`fn clone_from(&mut self, o: &Self)`

##### `impl<A: Array> Debug for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for TinyVec<A>`

- <span id="tinyvec-default"></span>`fn default() -> Self`

##### `impl<A: Array> Deref for TinyVec<A>`

- <span id="tinyvec-target"></span>`type Target = [<A as Array>::Item]`

- <span id="tinyvec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for TinyVec<A>`

- <span id="tinyvec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for TinyVec<A>`

##### `impl<A: Array> Extend for TinyVec<A>`

- <span id="tinyvec-extend"></span>`fn extend<T: IntoIterator<Item = <A as >::Item>>(&mut self, iter: T)`

##### `impl<A: Array> FromIterator for TinyVec<A>`

- <span id="tinyvec-from-iter"></span>`fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for TinyVec<A>`

- <span id="tinyvec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for TinyVec<A>`

- <span id="tinyvec-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="tinyvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for TinyVec<A>`

- <span id="tinyvec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<A: Array> IntoIterator for TinyVec<A>`

- <span id="tinyvec-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvec-intoiter"></span>`type IntoIter = TinyVecIterator<A>`

- <span id="tinyvec-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for TinyVec<A>`

- <span id="tinyvec-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for TinyVec<A>`

- <span id="tinyvec-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<A: Array> PartialOrd for TinyVec<A>`

- <span id="tinyvec-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<P, T> Receiver for TinyVec<A>`

- <span id="tinyvec-target"></span>`type Target = T`

##### `impl<T> ToString for TinyVec<A>`

- <span id="tinyvec-to-string"></span>`fn to_string(&self) -> String`

##### `impl<A: Array> UpperExp for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for TinyVec<A>`

- <span id="tinyvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `TinyVecDrain<'p, A: Array>`

```rust
enum TinyVecDrain<'p, A: Array> {
    Inline(ArrayVecDrain<'p, <A as >::Item>),
    Heap(vec::Drain<'p, <A as >::Item>),
}
```

Draining iterator for `TinyVecDrain`

See [`TinyVecDrain::drain`](TinyVecDrain::<A>::drain)

#### Trait Implementations

##### `impl<'p, A: Array> DoubleEndedIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<I> IntoIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecdrain-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'p, A: Array> Iterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecdrain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyvecdrain-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-count"></span>`fn count(self) -> usize`

- <span id="tinyvecdrain-for-each"></span>`fn for_each<F: FnMut(<Self as >::Item)>(self, f: F)`

### `TinyVecIterator<A: Array>`

```rust
enum TinyVecIterator<A: Array> {
    Inline(ArrayVecIterator<A>),
    Heap(alloc::vec::IntoIter<<A as >::Item>),
}
```

Iterator for consuming an `TinyVec` and returning owned elements.

#### Implementations

- <span id="tinyveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md)

#### Trait Implementations

##### `impl<A: Array> Debug for TinyVecIterator<A>`

- <span id="tinyveciterator-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-len"></span>`fn len(&self) -> usize`

##### `impl<A: Array> FusedIterator for TinyVecIterator<A>`

##### `impl<I> IntoIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyveciterator-intoiter"></span>`type IntoIter = I`

- <span id="tinyveciterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecIterator<A>`

- <span id="tinyveciterator-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyveciterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyveciterator-count"></span>`fn count(self) -> usize`

- <span id="tinyveciterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md)

