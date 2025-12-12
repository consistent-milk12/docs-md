*[tinyvec](../index.md) / [tinyvec](index.md)*

---

# Module `tinyvec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`TinyVecSplice`](#tinyvecsplice) | struct | Splicing iterator for `TinyVec` See [`TinyVec::splice`](TinyVec::<A>::splice) |
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

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1215-1220`](../../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L1215-L1220)*

Splicing iterator for `TinyVec`
See [`TinyVec::splice`](TinyVec::<A>::splice)

#### Trait Implementations

##### `impl<A, I> DoubleEndedIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Drop for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<A, I> ExactSizeIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-len"></span>`fn len(&self) -> usize`

##### `impl<A, I> FusedIterator for TinyVecSplice<'p, A, I>`

##### `impl<I> IntoIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecsplice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecsplice-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A, I> Iterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecsplice-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

- <span id="tinyvecsplice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `TinyVec<A: Array>`

```rust
enum TinyVec<A: Array> {
    Inline(ArrayVec<A>),
    Heap(alloc::vec::Vec<<A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:96-101`](../../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L96-L101)*

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

- <span id="tinyvec-is-heap"></span>`fn is_heap(&self) -> bool`

- <span id="tinyvec-is-inline"></span>`fn is_inline(&self) -> bool`

- <span id="tinyvec-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

- <span id="tinyvec-move-to-the-heap"></span>`fn move_to_the_heap(&mut self)`

- <span id="tinyvec-move-to-the-heap-and-reserve"></span>`fn move_to_the_heap_and_reserve(&mut self, n: usize)`

- <span id="tinyvec-reserve"></span>`fn reserve(&mut self, n: usize)`

- <span id="tinyvec-reserve-exact"></span>`fn reserve_exact(&mut self, n: usize)`

- <span id="tinyvec-with-capacity"></span>`fn with_capacity(cap: usize) -> Self`

- <span id="tinyvec-into-boxed-slice"></span>`fn into_boxed_slice(self) -> alloc::boxed::Box<[<A as >::Item]>` — [`Array`](../index.md#array)

- <span id="tinyvec-into-vec"></span>`fn into_vec(self) -> Vec<<A as >::Item>` — [`Array`](../index.md#array)

#### Trait Implementations

##### `impl<A: Array> AsMut for TinyVec<A>`

- <span id="tinyvec-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md#array)

##### `impl<A: Array> AsRef for TinyVec<A>`

- <span id="tinyvec-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

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

- <span id="tinyvec-deref-type-target"></span>`type Target = [<A as Array>::Item]`

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

- <span id="tinyvec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="tinyvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for TinyVec<A>`

- <span id="tinyvec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<A: Array> IntoIterator for TinyVec<A>`

- <span id="tinyvec-intoiterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvec-intoiterator-type-intoiter"></span>`type IntoIter = TinyVecIterator<A>`

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

##### `impl Receiver for TinyVec<A>`

- <span id="tinyvec-receiver-type-target"></span>`type Target = T`

##### `impl ToString for TinyVec<A>`

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

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1166-1171`](../../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L1166-L1171)*

Draining iterator for `TinyVecDrain`

See [`TinyVecDrain::drain`](TinyVecDrain::<A>::drain)

#### Trait Implementations

##### `impl<A: Array> DoubleEndedIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl IntoIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-iterator-type-item"></span>`type Item = <A as Array>::Item`

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

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1483-1488`](../../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L1483-L1488)*

Iterator for consuming an `TinyVec` and returning owned elements.

#### Implementations

- <span id="tinyveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

#### Trait Implementations

##### `impl<A: Array> Debug for TinyVecIterator<A>`

- <span id="tinyveciterator-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-len"></span>`fn len(&self) -> usize`

##### `impl<A: Array> FusedIterator for TinyVecIterator<A>`

##### `impl IntoIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyveciterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyveciterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecIterator<A>`

- <span id="tinyveciterator-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyveciterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyveciterator-count"></span>`fn count(self) -> usize`

- <span id="tinyveciterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

