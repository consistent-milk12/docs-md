# Crate `tinyvec`

`tinyvec` provides 100% safe vec-like data structures.

## Provided Types
With no features enabled, this crate provides the [`ArrayVec`](#arrayvec) type, which
is an array-backed storage. You can push values into the array and pop them
out of the array and so on. If the array is made to overflow it will panic.

Similarly, there is also a [`SliceVec`](#slicevec) type available, which is a vec-like
that's backed by a slice you provide. You can add and remove elements, but
if you overflow the slice it will panic.

With the `alloc` feature enabled, the crate also has a [`TinyVec`](#tinyvec) type.
This is an enum type which is either an `Inline(ArrayVec)` or a `Heap(Vec)`.
If a `TinyVec` is `Inline` and would overflow it automatically transitions
itself into being `Heap` mode instead of a panic.

All of this is done with no `unsafe` code within the crate. Technically the
`Vec` type from the standard library uses `unsafe` internally, but *this
crate* introduces no new `unsafe` code into your project.

The limitation is that the element type of a vec from this crate must
support the [`Default`](../gimli/index.md) trait. This means that this crate isn't suitable for
all situations, but a very surprising number of types do support `Default`.

## Other Features
* `grab_spare_slice` lets you get access to the "inactive" portions of an
  ArrayVec.
* `serde` provides a `Serialize` and `Deserialize` implementation for
  [`TinyVec`](#tinyvec) and [`ArrayVec`](#arrayvec) types, provided the inner item also has an
  implementation.
* `borsh` provides a `BorshSerialize` and `BorshDeserialize` implementation
  for [`TinyVec`](#tinyvec) and [`ArrayVec`](#arrayvec) types, provided the inner item also has
  an implementation.

## API
The general goal of the crate is that, as much as possible, the vecs here
should be a "drop in" replacement for the standard library `Vec` type. We
strive to provide all of the `Vec` methods with the same names and
signatures. The exception is that the element type of some methods will have
a `Default` bound that's not part of the normal `Vec` type.

The vecs here also have a few additional methods that aren't on the `Vec`
type. In this case, the names tend to be fairly long so that they are
unlikely to clash with any future methods added to `Vec`.

## Contents

- [Modules](#modules)
  - [`array`](#array)
  - [`arrayvec`](#arrayvec)
  - [`arrayvec_drain`](#arrayvec-drain)
  - [`slicevec`](#slicevec)
  - [`tinyvec`](#tinyvec)
  - [`const_generic_impl`](#const-generic-impl)
- [Structs](#structs)
  - [`ArrayVec`](#arrayvec)
  - [`ArrayVecSplice`](#arrayvecsplice)
  - [`TryFromSliceError`](#tryfromsliceerror)
  - [`ArrayVecIterator`](#arrayveciterator)
  - [`ArrayVecDrain`](#arrayvecdrain)
  - [`SliceVec`](#slicevec)
  - [`SliceVecDrain`](#slicevecdrain)
  - [`TinyVecSplice`](#tinyvecsplice)
- [Enums](#enums)
  - [`TinyVec`](#tinyvec)
  - [`TinyVecDrain`](#tinyvecdrain)
  - [`TinyVecIterator`](#tinyveciterator)
- [Traits](#traits)
  - [`Array`](#array)
- [Macros](#macros)
  - [`array_vec!`](#array-vec)
  - [`tiny_vec!`](#tiny-vec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`array`](#array) | mod |  |
| [`arrayvec`](#arrayvec) | mod |  |
| [`arrayvec_drain`](#arrayvec-drain) | mod |  |
| [`slicevec`](#slicevec) | mod |  |
| [`tinyvec`](#tinyvec) | mod |  |
| [`const_generic_impl`](#const-generic-impl) | mod |  |
| [`ArrayVec`](#arrayvec) | struct | An array-backed, vector-like data structure. |
| [`ArrayVecSplice`](#arrayvecsplice) | struct | Splicing iterator for `ArrayVec` See [`ArrayVec::splice`](ArrayVec::<A>::splice) |
| [`TryFromSliceError`](#tryfromsliceerror) | struct | The error type returned when a conversion from a slice to an [`ArrayVec`] fails. |
| [`ArrayVecIterator`](#arrayveciterator) | struct | Iterator for consuming an `ArrayVec` and returning owned elements. |
| [`ArrayVecDrain`](#arrayvecdrain) | struct | Draining iterator for [`ArrayVec`] |
| [`SliceVec`](#slicevec) | struct | A slice-backed vector-like data structure. |
| [`SliceVecDrain`](#slicevecdrain) | struct | Draining iterator for [`SliceVec`] |
| [`TinyVecSplice`](#tinyvecsplice) | struct | Splicing iterator for `TinyVec` See [`TinyVec::splice`](TinyVec::<A>::splice) |
| [`TinyVec`](#tinyvec) | enum | A vector that starts inline, but can automatically move to the heap. |
| [`TinyVecDrain`](#tinyvecdrain) | enum | Draining iterator for `TinyVecDrain` |
| [`TinyVecIterator`](#tinyveciterator) | enum | Iterator for consuming an `TinyVec` and returning owned elements. |
| [`Array`](#array) | trait | A trait for types that are an array. |
| [`array_vec!`](#array-vec) | macro | Helper to make an `ArrayVec`. |
| [`tiny_vec!`](#tiny-vec) | macro | Helper to make a `TinyVec`. |

## Modules

- [`array`](array/index.md)
- [`arrayvec`](arrayvec/index.md)
- [`arrayvec_drain`](arrayvec_drain/index.md)
- [`slicevec`](slicevec/index.md)
- [`tinyvec`](tinyvec/index.md)
- [`const_generic_impl`](const_generic_impl/index.md)

## Structs

### `ArrayVec<A>`

```rust
struct ArrayVec<A> {
    len: u16,
    data: A,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:106-109`](../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L106-L109)*

An array-backed, vector-like data structure.

* `ArrayVec` has a fixed capacity, equal to the minimum of the array size
  and `u16::MAX`. Note that not all capacities are necessarily supported by
  default. See comments in [`Array`](#array).
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

- <span id="arrayvec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut <A as >::Item` — [`Array`](#array)

- <span id="arrayvec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

- <span id="arrayvec-as-ptr"></span>`fn as_ptr(&self) -> *const <A as >::Item` — [`Array`](#array)

- <span id="arrayvec-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

- <span id="arrayvec-capacity"></span>`fn capacity(&self) -> usize`

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

- <span id="arrayvec-drain"></span>`fn drain<R>(&mut self, range: R) -> ArrayVecDrain<'_, <A as >::Item>` — [`ArrayVecDrain`](#arrayvecdrain), [`Array`](#array)

- <span id="arrayvec-into-inner"></span>`fn into_inner(self) -> A`

- <span id="arrayvec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[<A as >::Item])` — [`Array`](#array)

- <span id="arrayvec-fill"></span>`fn fill<I: IntoIterator<Item = <A as >::Item>>(&mut self, iter: I) -> <I as >::IntoIter`

- <span id="arrayvec-from-array-len"></span>`fn from_array_len(data: A, len: usize) -> Self`

- <span id="arrayvec-insert"></span>`fn insert(&mut self, index: usize, item: <A as >::Item)` — [`Array`](#array)

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, item: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="arrayvec-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="arrayvec-len"></span>`fn len(&self) -> usize`

- <span id="arrayvec-new"></span>`fn new() -> Self`

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="arrayvec-push"></span>`fn push(&mut self, val: <A as >::Item)` — [`Array`](#array)

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, val: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="arrayvec-remove"></span>`fn remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](#array)

- <span id="arrayvec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: <A as >::Item)` — [`Array`](#array)

- <span id="arrayvec-resize-with"></span>`fn resize_with<F: FnMut() -> <A as >::Item>(&mut self, new_len: usize, f: F)`

- <span id="arrayvec-retain"></span>`fn retain<F: FnMut(&<A as >::Item) -> bool>(&mut self, acceptable: F)`

- <span id="arrayvec-retain-mut"></span>`fn retain_mut<F>(&mut self, acceptable: F)`

- <span id="arrayvec-set-len"></span>`fn set_len(&mut self, new_len: usize)`

- <span id="arrayvec-split-off"></span>`fn split_off(&mut self, at: usize) -> Self`

- <span id="arrayvec-splice"></span>`fn splice<R, I>(&mut self, range: R, replacement: I) -> ArrayVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`ArrayVecSplice`](#arrayvecsplice)

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](#array)

- <span id="arrayvec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

- <span id="arrayvec-try-from-array-len"></span>`fn try_from_array_len(data: A, len: usize) -> Result<Self, A>`

#### Trait Implementations

##### `impl<A: Array> AsMut for ArrayVec<A>`

- <span id="arrayvec-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for ArrayVec<A>`

- <span id="arrayvec-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](#array)

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

- <span id="arrayvec-deref-type-target"></span>`type Target = [<A as Array>::Item]`

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

- <span id="arrayvec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="arrayvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for ArrayVec<A>`

- <span id="arrayvec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<A: Array> IntoIterator for ArrayVec<A>`

- <span id="arrayvec-intoiterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayvec-intoiterator-type-intoiter"></span>`type IntoIter = ArrayVecIterator<A>`

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

##### `impl Receiver for ArrayVec<A>`

- <span id="arrayvec-receiver-type-target"></span>`type Target = T`

##### `impl ToString for ArrayVec<A>`

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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1264-1269`](../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L1264-L1269)*

Splicing iterator for `ArrayVec`
See [`ArrayVec::splice`](ArrayVec::<A>::splice)

#### Trait Implementations

##### `impl<A, I> DoubleEndedIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Drop for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<A, I> ExactSizeIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-len"></span>`fn len(&self) -> usize`

##### `impl<A, I> FusedIterator for ArrayVecSplice<'p, A, I>`

##### `impl<I> IntoIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecsplice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecsplice-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Iterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayvecsplice-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="arrayvecsplice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `TryFromSliceError`

```rust
struct TryFromSliceError(());
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1425`](../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L1425)*

The error type returned when a conversion from a slice to an [`ArrayVec`](#arrayvec)
fails.

#### Trait Implementations

##### `impl Clone for TryFromSliceError`

- <span id="tryfromsliceerror-clone"></span>`fn clone(&self) -> TryFromSliceError` — [`TryFromSliceError`](#tryfromsliceerror)

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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1478-1482`](../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L1478-L1482)*

Iterator for consuming an `ArrayVec` and returning owned elements.

#### Implementations

- <span id="arrayveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array> Debug for ArrayVecIterator<A>`

- <span id="arrayveciterator-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-len"></span>`fn len(&self) -> usize`

##### `impl<A: Array> FusedIterator for ArrayVecIterator<A>`

##### `impl IntoIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayveciterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayveciterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayveciterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayveciterator-count"></span>`fn count(self) -> usize`

- <span id="arrayveciterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](#array)

### `ArrayVecDrain<'a, T: 'a + Default>`

```rust
struct ArrayVecDrain<'a, T: 'a + Default> {
    iter: slice::IterMut<'a, T>,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec_drain.rs:11-13`](../../.source_1765210505/tinyvec-1.10.0/src/arrayvec_drain.rs#L11-L13)*

Draining iterator for [`ArrayVec`](#arrayvec)

See [`ArrayVec::drain`](ArrayVec::drain)

#### Implementations

- <span id="arrayvecdrain-new"></span>`fn new<A, R>(arr: &'a mut ArrayVec<A>, range: R) -> Self` — [`ArrayVec`](#arrayvec)

#### Trait Implementations

##### `impl<T: 'a + Default> DoubleEndedIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T: 'a + Default> ExactSizeIterator for ArrayVecDrain<'a, T>`

##### `impl<T: 'a + Default> FusedIterator for ArrayVecDrain<'a, T>`

##### `impl IntoIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'a + Default> Iterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-iterator-type-item"></span>`type Item = T`

- <span id="arrayvecdrain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayvecdrain-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-for-each"></span>`fn for_each<F>(self, f: F)`

### `SliceVec<'s, T>`

```rust
struct SliceVec<'s, T> {
    data: &'s mut [T],
    len: usize,
}
```

*Defined in [`tinyvec-1.10.0/src/slicevec.rs:16-19`](../../.source_1765210505/tinyvec-1.10.0/src/slicevec.rs#L16-L19)*

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

- <span id="slicevec-drain"></span>`fn drain<'p, R: RangeBounds<usize>>(self: &'p mut Self, range: R) -> SliceVecDrain<'p, 's, T>` — [`SliceVecDrain`](#slicevecdrain)

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

- <span id="slicevec-split-off"></span>`fn split_off<'a>(self: &'a mut Self, at: usize) -> SliceVec<'s, T>` — [`SliceVec`](#slicevec)

- <span id="slicevec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> T`

- <span id="slicevec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

- <span id="slicevec-try-from-slice-len"></span>`fn try_from_slice_len(data: &'s mut [T], len: usize) -> Option<Self>`

#### Trait Implementations

##### `impl<T> AsMut for SliceVec<'s, T>`

- <span id="slicevec-as-mut"></span>`fn as_mut(&mut self) -> &mut [T]`

##### `impl<T> AsRef for SliceVec<'s, T>`

- <span id="slicevec-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T> Binary for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Debug for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for SliceVec<'s, T>`

- <span id="slicevec-default"></span>`fn default() -> Self`

##### `impl<T> Deref for SliceVec<'s, T>`

- <span id="slicevec-deref-type-target"></span>`type Target = [T]`

- <span id="slicevec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> DerefMut for SliceVec<'s, T>`

- <span id="slicevec-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Display for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Eq for SliceVec<'s, T>`

##### `impl<T> Extend for SliceVec<'s, T>`

- <span id="slicevec-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T> Hash for SliceVec<'s, T>`

- <span id="slicevec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T, I> Index for SliceVec<'s, T>`

- <span id="slicevec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="slicevec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<T, I> IndexMut for SliceVec<'s, T>`

- <span id="slicevec-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<T> IntoIterator for SliceVec<'s, T>`

- <span id="slicevec-intoiterator-type-item"></span>`type Item = &'s mut T`

- <span id="slicevec-intoiterator-type-intoiter"></span>`type IntoIter = IterMut<'s, T>`

- <span id="slicevec-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T> LowerExp for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> LowerHex for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Octal for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Ord for SliceVec<'s, T>`

- <span id="slicevec-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<T> PartialEq for SliceVec<'s, T>`

- <span id="slicevec-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> PartialOrd for SliceVec<'s, T>`

- <span id="slicevec-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<T> Pointer for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Receiver for SliceVec<'s, T>`

- <span id="slicevec-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToString for SliceVec<'s, T>`

- <span id="slicevec-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T> UpperExp for SliceVec<'s, T>`

- <span id="slicevec-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> UpperHex for SliceVec<'s, T>`

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

*Defined in [`tinyvec-1.10.0/src/slicevec.rs:714-719`](../../.source_1765210505/tinyvec-1.10.0/src/slicevec.rs#L714-L719)*

Draining iterator for [`SliceVec`](#slicevec)

See [`SliceVec::drain`](SliceVec::drain)

#### Trait Implementations

##### `impl<T: Default> Drop for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-drop"></span>`fn drop(&mut self)`

##### `impl<T: Default> FusedIterator for SliceVecDrain<'p, 's, T>`

##### `impl IntoIterator for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slicevecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="slicevecdrain-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Default> Iterator for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-iterator-type-item"></span>`type Item = T`

- <span id="slicevecdrain-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut TinyVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1215-1220`](../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L1215-L1220)*

Splicing iterator for `TinyVec`
See [`TinyVec::splice`](TinyVec::<A>::splice)

#### Trait Implementations

##### `impl<A, I> DoubleEndedIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

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

- <span id="tinyvecsplice-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="tinyvecsplice-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `TinyVec<A: Array>`

```rust
enum TinyVec<A: Array> {
    Inline(ArrayVec<A>),
    Heap(alloc::vec::Vec<<A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:96-101`](../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L96-L101)*

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

- <span id="tinyvec-into-boxed-slice"></span>`fn into_boxed_slice(self) -> alloc::boxed::Box<[<A as >::Item]>` — [`Array`](#array)

- <span id="tinyvec-into-vec"></span>`fn into_vec(self) -> Vec<<A as >::Item>` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array> AsMut for TinyVec<A>`

- <span id="tinyvec-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for TinyVec<A>`

- <span id="tinyvec-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](#array)

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

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1166-1171`](../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L1166-L1171)*

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

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1483-1488`](../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L1483-L1488)*

Iterator for consuming an `TinyVec` and returning owned elements.

#### Implementations

- <span id="tinyveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

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

- <span id="tinyveciterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](#array)

## Traits

### `Array`

```rust
trait Array { ... }
```

*Defined in [`tinyvec-1.10.0/src/array.rs:18-41`](../../.source_1765210505/tinyvec-1.10.0/src/array.rs#L18-L41)*

A trait for types that are an array.

An "array", for our purposes, has the following properties:
* Owns some number of elements.
* The element type can be generic, but must implement [`Default`](../gimli/index.md).
* The capacity is fixed at compile time, based on the implementing type.
* You can get a shared or mutable slice to the elements.

You are generally **not** expected to need to implement this yourself. It is
already implemented for all array lengths.

**Additional lengths can easily be added upon request.**

## Safety Reminder

Just a reminder: this trait is 100% safe, which means that `unsafe` code
**must not** rely on an instance of this trait being correct.

#### Associated Types

- `type Item: 1`

#### Associated Constants

- `const CAPACITY: usize`

#### Required Methods

- `fn as_slice(&self) -> &[<Self as >::Item]`

  Gives a shared slice over the whole thing.

- `fn as_slice_mut(&mut self) -> &mut [<Self as >::Item]`

  Gives a unique slice over the whole thing.

- `fn default() -> Self`

  Create a default-initialized instance of ourself, similar to the

#### Implementors

- `[T; N]`

## Macros

### `array_vec!`

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:30-50`](../../.source_1765210505/tinyvec-1.10.0/src/arrayvec.rs#L30-L50)*

Helper to make an `ArrayVec`.

You specify the backing array type, and optionally give all the elements you
want to initially place into the array.

```rust
use tinyvec::*;

// The backing array type can be specified in the macro call
let empty_av = array_vec!([u8; 16]);
let some_ints = array_vec!([i32; 4] => 1, 2, 3);

// Or left to inference
let empty_av: ArrayVec<[u8; 10]> = array_vec!();
let some_ints: ArrayVec<[u8; 10]> = array_vec!(5, 6, 7, 8);
```

### `tiny_vec!`

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:37-66`](../../.source_1765210505/tinyvec-1.10.0/src/tinyvec.rs#L37-L66)*

Helper to make a `TinyVec`.

You specify the backing array type, and optionally give all the elements you
want to initially place into the array.

```rust
use tinyvec::*;

// The backing array type can be specified in the macro call
let empty_tv = tiny_vec!([u8; 16]);
let some_ints = tiny_vec!([i32; 4] => 1, 2, 3);
let many_ints = tiny_vec!([i32; 4] => 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);

// Or left to inference
let empty_tv: TinyVec<[u8; 16]> = tiny_vec!();
let some_ints: TinyVec<[i32; 4]> = tiny_vec!(1, 2, 3);
let many_ints: TinyVec<[i32; 4]> = tiny_vec!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
```

