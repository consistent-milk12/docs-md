*[tinyvec](../index.md) / [arrayvec](index.md)*

---

# Module `arrayvec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ArrayVec`](#arrayvec) | struct | An array-backed, vector-like data structure. |
| [`ArrayVecSplice`](#arrayvecsplice) | struct | Splicing iterator for `ArrayVec` See [`ArrayVec::splice`](ArrayVec::<A>::splice) |
| [`TryFromSliceError`](#tryfromsliceerror) | struct | The error type returned when a conversion from a slice to an [`ArrayVec`] fails. |
| [`ArrayVecIterator`](#arrayveciterator) | struct | Iterator for consuming an `ArrayVec` and returning owned elements. |

## Structs

### `ArrayVec<A>`

```rust
struct ArrayVec<A> {
    len: u16,
    data: A,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:106-109`](../../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L106-L109)*

An array-backed, vector-like data structure.

* `ArrayVec` has a fixed capacity, equal to the minimum of the array size
  and `u16::MAX`. Note that not all capacities are necessarily supported by
  default. See comments in [`Array`](../index.md).
* `ArrayVec` has a variable length, as you add and remove elements. Attempts
  to fill the vec beyond its capacity will cause a panic.
* All of the vec's array slots are always initialized in terms of Rust's
  memory model. When you remove a element from a location, the old value at
  that location is replaced with the type's default value.

The overall API of this type is intended to, as much as possible, emulate
the API of the [`Vec`](https://doc.rust-lang.org/alloc/vec/struct.Vec.html)
type.

## Construction

You can use the `array_vec!` macro similarly to how you might use the `vec!`
macro. Specify the array type, then optionally give all the initial values
you want to have.
```rust
use tinyvec::*;
let some_ints = array_vec!([i32; 4] => 1, 2, 3);
assert_eq!(some_ints.len(), 3);
```

The [`default`](ArrayVec::new) for an `ArrayVec` is to have a default
array with length 0. The [`new`](ArrayVec::new) method is the same as
calling `default`
```rust
use tinyvec::*;
let some_ints = ArrayVec::<[i32; 7]>::default();
assert_eq!(some_ints.len(), 0);

let more_ints = ArrayVec::<[i32; 7]>::new();
assert_eq!(some_ints, more_ints);
```

If you have an array and want the _whole thing_ so count as being "in" the
new `ArrayVec` you can use one of the `from` implementations. If you want
_part of_ the array then you can use

# use tinyvec::*;
let some_ints = ArrayVec::from([5, 6, 7, 8]);
assert_eq!(some_ints.len(), 4);

let more_ints = ArrayVec::from_array_len([5, 6, 7, 8], 2);
assert_eq!(more_ints.len(), 2);

let no_ints: ArrayVec<[u8; 5]> = ArrayVec::from_array_empty([1, 2, 3, 4, 5]);
assert_eq!(no_ints.len(), 0);
```rust

#### Implementations

- <span id="arrayvec-append"></span>`fn append(&mut self, other: &mut Self)`

- <span id="arrayvec-try-append"></span>`fn try_append<'other>(&mut self, other: &'other mut Self) -> Option<&'other mut Self>`

- <span id="arrayvec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut <A as >::Item` — [`Array`](../index.md#array)

- <span id="arrayvec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md#array)

- <span id="arrayvec-as-ptr"></span>`fn as_ptr(&self) -> *const <A as >::Item` — [`Array`](../index.md#array)

- <span id="arrayvec-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

- <span id="arrayvec-capacity"></span>`fn capacity(&self) -> usize`

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

- <span id="arrayvec-drain"></span>`fn drain<R>(&mut self, range: R) -> ArrayVecDrain<'_, <A as >::Item>` — [`ArrayVecDrain`](../index.md#arrayvecdrain), [`Array`](../index.md#array)

- <span id="arrayvec-into-inner"></span>`fn into_inner(self) -> A`

- <span id="arrayvec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[<A as >::Item])` — [`Array`](../index.md#array)

- <span id="arrayvec-fill"></span>`fn fill<I: IntoIterator<Item = <A as >::Item>>(&mut self, iter: I) -> <I as >::IntoIter`

- <span id="arrayvec-from-array-len"></span>`fn from_array_len(data: A, len: usize) -> Self`

- <span id="arrayvec-insert"></span>`fn insert(&mut self, index: usize, item: <A as >::Item)` — [`Array`](../index.md#array)

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, item: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

- <span id="arrayvec-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="arrayvec-len"></span>`fn len(&self) -> usize`

- <span id="arrayvec-new"></span>`fn new() -> Self`

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

- <span id="arrayvec-push"></span>`fn push(&mut self, val: <A as >::Item)` — [`Array`](../index.md#array)

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, val: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

- <span id="arrayvec-remove"></span>`fn remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](../index.md#array)

- <span id="arrayvec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: <A as >::Item)` — [`Array`](../index.md#array)

- <span id="arrayvec-resize-with"></span>`fn resize_with<F: FnMut() -> <A as >::Item>(&mut self, new_len: usize, f: F)`

- <span id="arrayvec-retain"></span>`fn retain<F: FnMut(&<A as >::Item) -> bool>(&mut self, acceptable: F)`

- <span id="arrayvec-retain-mut"></span>`fn retain_mut<F>(&mut self, acceptable: F)`

- <span id="arrayvec-set-len"></span>`fn set_len(&mut self, new_len: usize)`

- <span id="arrayvec-split-off"></span>`fn split_off(&mut self, at: usize) -> Self`

- <span id="arrayvec-splice"></span>`fn splice<R, I>(&mut self, range: R, replacement: I) -> ArrayVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`ArrayVecSplice`](../index.md#arrayvecsplice)

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](../index.md#array)

- <span id="arrayvec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

- <span id="arrayvec-try-from-array-len"></span>`fn try_from_array_len(data: A, len: usize) -> Result<Self, A>`

#### Trait Implementations

##### `impl<A: Array> AsMut for ArrayVec<A>`

- <span id="arrayvec-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md#array)

##### `impl<A: Array> AsRef for ArrayVec<A>`

- <span id="arrayvec-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

##### `impl<A: Array> Binary for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

- <span id="arrayvec-clone-from"></span>`fn clone_from(&mut self, o: &Self)`

##### `impl<A> Copy for ArrayVec<A>`

##### `impl<A: Array> Debug for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: Array> Deref for ArrayVec<A>`

- <span id="arrayvec-type-target"></span>`type Target = [<A as Array>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for ArrayVec<A>`

- <span id="arrayvec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for ArrayVec<A>`

##### `impl<A: Array> Extend for ArrayVec<A>`

- <span id="arrayvec-extend"></span>`fn extend<T: IntoIterator<Item = <A as >::Item>>(&mut self, iter: T)`

##### `impl<A: Array> FromIterator for ArrayVec<A>`

- <span id="arrayvec-from-iter"></span>`fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for ArrayVec<A>`

- <span id="arrayvec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for ArrayVec<A>`

- <span id="arrayvec-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="arrayvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for ArrayVec<A>`

- <span id="arrayvec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<A: Array> IntoIterator for ArrayVec<A>`

- <span id="arrayvec-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayvec-type-intoiter"></span>`type IntoIter = ArrayVecIterator<A>`

- <span id="arrayvec-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for ArrayVec<A>`

- <span id="arrayvec-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for ArrayVec<A>`

- <span id="arrayvec-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<A: Array> PartialOrd for ArrayVec<A>`

- <span id="arrayvec-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<P, T> Receiver for ArrayVec<A>`

- <span id="arrayvec-type-target"></span>`type Target = T`

##### `impl<T> ToString for ArrayVec<A>`

- <span id="arrayvec-to-string"></span>`fn to_string(&self) -> String`

##### `impl<A: Array> UpperExp for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for ArrayVec<A>`

- <span id="arrayvec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `ArrayVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct ArrayVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut ArrayVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1264-1269`](../../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L1264-L1269)*

Splicing iterator for `ArrayVec`
See [`ArrayVec::splice`](ArrayVec::<A>::splice)

#### Trait Implementations

##### `impl<'p, A, I> DoubleEndedIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

##### `impl<'p, A: Array, I: Iterator<Item = <A as >::Item>> Drop for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<'p, A, I> ExactSizeIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-len"></span>`fn len(&self) -> usize`

##### `impl<'p, A, I> FusedIterator for ArrayVecSplice<'p, A, I>`

##### `impl<I> IntoIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecsplice-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecsplice-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'p, A: Array, I: Iterator<Item = <A as >::Item>> Iterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayvecsplice-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

- <span id="arrayvecsplice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `TryFromSliceError`

```rust
struct TryFromSliceError(());
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1425`](../../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L1425)*

The error type returned when a conversion from a slice to an [`ArrayVec`](../index.md)
fails.

#### Trait Implementations

##### `impl Clone for TryFromSliceError`

- <span id="tryfromsliceerror-clone"></span>`fn clone(&self) -> TryFromSliceError` — [`TryFromSliceError`](../index.md#tryfromsliceerror)

##### `impl Copy for TryFromSliceError`

##### `impl Debug for TryFromSliceError`

- <span id="tryfromsliceerror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryFromSliceError`

- <span id="tryfromsliceerror-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl ToString for TryFromSliceError`

- <span id="tryfromsliceerror-to-string"></span>`fn to_string(&self) -> String`

### `ArrayVecIterator<A: Array>`

```rust
struct ArrayVecIterator<A: Array> {
    base: u16,
    tail: u16,
    data: A,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1478-1482`](../../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L1478-L1482)*

Iterator for consuming an `ArrayVec` and returning owned elements.

#### Implementations

- <span id="arrayveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

#### Trait Implementations

##### `impl<A: Array> Debug for ArrayVecIterator<A>`

- <span id="arrayveciterator-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-len"></span>`fn len(&self) -> usize`

##### `impl<A: Array> FusedIterator for ArrayVecIterator<A>`

##### `impl<I> IntoIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayveciterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayveciterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayveciterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayveciterator-count"></span>`fn count(self) -> usize`

- <span id="arrayveciterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

