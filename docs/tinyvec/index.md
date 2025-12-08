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
support the [`Default`](#default) trait. This means that this crate isn't suitable for
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

## Structs

### `ArrayVec<A>`

```rust
struct ArrayVec<A> {
    len: u16,
    data: A,
}
```

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

- `fn drain_to_vec_and_reserve(self: &mut Self, n: usize) -> Vec<<A as >::Item>` — [`Array`](#array)

- `fn drain_to_vec(self: &mut Self) -> Vec<<A as >::Item>` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array> AsMut for ArrayVec<A>`

- `fn as_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for ArrayVec<A>`

- `fn as_ref(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Binary for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A> Clone for ArrayVec<A>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, o: &Self)`

##### `impl<A> Copy for ArrayVec<A>`

##### `impl<A: Array> Debug for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for ArrayVec<A>`

- `fn default() -> Self`

##### `impl<A: Array> Deref for ArrayVec<A>`

- `type Target = [<A as Array>::Item]`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for ArrayVec<A>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for ArrayVec<A>`

##### `impl<A: Array> Extend for ArrayVec<A>`

- `fn extend<T: IntoIterator<Item = <A as >::Item>>(self: &mut Self, iter: T)`

##### `impl<A: Array> FromIterator for ArrayVec<A>`

- `fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for ArrayVec<A>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for ArrayVec<A>`

- `type Output = <I as SliceIndex>::Output`

- `fn index(self: &Self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for ArrayVec<A>`

- `fn index_mut(self: &mut Self, index: I) -> &mut <Self as >::Output`

##### `impl<A: Array> IntoIterator for ArrayVec<A>`

- `type Item = <A as Array>::Item`

- `type IntoIter = ArrayVecIterator<A>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for ArrayVec<A>`

- `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for ArrayVec<A>`

- `fn eq(self: &Self, other: &Self) -> bool`

##### `impl<A: Array> PartialOrd for ArrayVec<A>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<P, T> Receiver for ArrayVec<A>`

- `type Target = T`

##### `impl<T> ToString for ArrayVec<A>`

- `fn to_string(self: &Self) -> String`

##### `impl<A: Array> UpperExp for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for ArrayVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `ArrayVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct ArrayVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut ArrayVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

Splicing iterator for `ArrayVec`
See [`ArrayVec::splice`](ArrayVec::<A>::splice)

#### Trait Implementations

##### `impl<'p, A, I> DoubleEndedIterator for ArrayVecSplice<'p, A, I>`

- `fn next_back(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<'p, A: Array, I: Iterator<Item = <A as >::Item>> Drop for ArrayVecSplice<'p, A, I>`

- `fn drop(self: &mut Self)`

##### `impl<'p, A, I> ExactSizeIterator for ArrayVecSplice<'p, A, I>`

- `fn len(self: &Self) -> usize`

##### `impl<'p, A, I> FusedIterator for ArrayVecSplice<'p, A, I>`

##### `impl<I> IntoIterator for ArrayVecSplice<'p, A, I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'p, A: Array, I: Iterator<Item = <A as >::Item>> Iterator for ArrayVecSplice<'p, A, I>`

- `type Item = <A as Array>::Item`

- `fn next(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `TryFromSliceError`

```rust
struct TryFromSliceError(());
```

The error type returned when a conversion from a slice to an [`ArrayVec`](#arrayvec)
fails.

#### Trait Implementations

##### `impl Clone for TryFromSliceError`

- `fn clone(self: &Self) -> TryFromSliceError` — [`TryFromSliceError`](#tryfromsliceerror)

##### `impl Copy for TryFromSliceError`

##### `impl Debug for TryFromSliceError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for TryFromSliceError`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> ToString for TryFromSliceError`

- `fn to_string(self: &Self) -> String`

### `ArrayVecIterator<A: Array>`

```rust
struct ArrayVecIterator<A: Array> {
    base: u16,
    tail: u16,
    data: A,
}
```

Iterator for consuming an `ArrayVec` and returning owned elements.

#### Implementations

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array> Debug for ArrayVecIterator<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for ArrayVecIterator<A>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for ArrayVecIterator<A>`

- `fn len(self: &Self) -> usize`

##### `impl<A: Array> FusedIterator for ArrayVecIterator<A>`

##### `impl<I> IntoIterator for ArrayVecIterator<A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<A: Array> Iterator for ArrayVecIterator<A>`

- `type Item = <A as Array>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn count(self: Self) -> usize`

- `fn last(self: Self) -> Option<<Self as >::Item>`

- `fn nth(self: &mut Self, n: usize) -> Option<<A as >::Item>` — [`Array`](#array)

### `ArrayVecDrain<'a, T: 'a + Default>`

```rust
struct ArrayVecDrain<'a, T: 'a + Default> {
    iter: slice::IterMut<'a, T>,
}
```

Draining iterator for [`ArrayVec`](#arrayvec)

See [`ArrayVec::drain`](ArrayVec::drain)

#### Implementations

- `fn new<A, R>(arr: &'a mut ArrayVec<A>, range: R) -> Self` — [`ArrayVec`](#arrayvec)

#### Trait Implementations

##### `impl<'a, T: 'a + Default> DoubleEndedIterator for ArrayVecDrain<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

##### `impl<'a, T: 'a + Default> ExactSizeIterator for ArrayVecDrain<'a, T>`

##### `impl<'a, T: 'a + Default> FusedIterator for ArrayVecDrain<'a, T>`

##### `impl<I> IntoIterator for ArrayVecDrain<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: 'a + Default> Iterator for ArrayVecDrain<'a, T>`

- `type Item = T`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn last(self: Self) -> Option<<Self as >::Item>`

- `fn for_each<F>(self: Self, f: F)`

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

- `fn drain<'p, R: RangeBounds<usize>>(self: &'p mut Self, range: R) -> SliceVecDrain<'p, 's, T>` — [`SliceVecDrain`](#slicevecdrain)

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

- `fn split_off<'a>(self: &'a mut Self, at: usize) -> SliceVec<'s, T>` — [`SliceVec`](#slicevec)

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

Draining iterator for [`SliceVec`](#slicevec)

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

- `fn next_back(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<'p, A: Array, I: Iterator<Item = <A as >::Item>> Drop for TinyVecSplice<'p, A, I>`

- `fn drop(self: &mut Self)`

##### `impl<'p, A, I> ExactSizeIterator for TinyVecSplice<'p, A, I>`

- `fn len(self: &Self) -> usize`

##### `impl<'p, A, I> FusedIterator for TinyVecSplice<'p, A, I>`

##### `impl<I> IntoIterator for TinyVecSplice<'p, A, I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'p, A, I> Iterator for TinyVecSplice<'p, A, I>`

- `type Item = <A as Array>::Item`

- `fn next(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

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

- `fn append(self: &mut Self, other: &mut Self)`

- `fn swap_remove(self: &mut Self, index: usize) -> <A as >::Item` — [`Array`](#array)

- `fn pop(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

- `fn remove(self: &mut Self, index: usize) -> <A as >::Item` — [`Array`](#array)

- `fn len(self: &Self) -> usize`

- `fn capacity(self: &Self) -> usize`

- `fn truncate(self: &mut Self, new_len: usize)`

- `fn as_mut_ptr(self: &mut Self) -> *mut <A as >::Item` — [`Array`](#array)

- `fn as_ptr(self: &Self) -> *const <A as >::Item` — [`Array`](#array)

- `fn retain<F: FnMut(&<A as >::Item) -> bool>(self: &mut Self, acceptable: F)`

- `fn as_mut_slice(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](#array)

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

- `fn clear(self: &mut Self)`

- `fn drain<R: RangeBounds<usize>>(self: &mut Self, range: R) -> TinyVecDrain<'_, A>` — [`TinyVecDrain`](#tinyvecdrain)

- `fn extend_from_slice(self: &mut Self, sli: &[<A as >::Item])` — [`Array`](#array)

- `fn from_array_len(data: A, len: usize) -> Self`

- `fn insert(self: &mut Self, index: usize, item: <A as >::Item)` — [`Array`](#array)

- `fn is_empty(self: &Self) -> bool`

- `fn new() -> Self`

- `fn push(self: &mut Self, val: <A as >::Item)` — [`Array`](#array)

- `fn resize(self: &mut Self, new_len: usize, new_val: <A as >::Item)` — [`Array`](#array)

- `fn resize_with<F: FnMut() -> <A as >::Item>(self: &mut Self, new_len: usize, f: F)`

- `fn split_off(self: &mut Self, at: usize) -> Self`

- `fn splice<R, I>(self: &mut Self, range: R, replacement: I) -> TinyVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`TinyVecSplice`](#tinyvecsplice)

- `fn try_from_array_len(data: A, len: usize) -> Result<Self, A>`

#### Trait Implementations

##### `impl<A: Array> AsMut for TinyVec<A>`

- `fn as_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for TinyVec<A>`

- `fn as_ref(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Binary for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A> Clone for TinyVec<A>`

- `fn clone(self: &Self) -> Self`

- `fn clone_from(self: &mut Self, o: &Self)`

##### `impl<A: Array> Debug for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for TinyVec<A>`

- `fn default() -> Self`

##### `impl<A: Array> Deref for TinyVec<A>`

- `type Target = [<A as Array>::Item]`

- `fn deref(self: &Self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for TinyVec<A>`

- `fn deref_mut(self: &mut Self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for TinyVec<A>`

##### `impl<A: Array> Extend for TinyVec<A>`

- `fn extend<T: IntoIterator<Item = <A as >::Item>>(self: &mut Self, iter: T)`

##### `impl<A: Array> FromIterator for TinyVec<A>`

- `fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for TinyVec<A>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for TinyVec<A>`

- `type Output = <I as SliceIndex>::Output`

- `fn index(self: &Self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for TinyVec<A>`

- `fn index_mut(self: &mut Self, index: I) -> &mut <Self as >::Output`

##### `impl<A: Array> IntoIterator for TinyVec<A>`

- `type Item = <A as Array>::Item`

- `type IntoIter = TinyVecIterator<A>`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for TinyVec<A>`

- `fn cmp(self: &Self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for TinyVec<A>`

- `fn eq(self: &Self, other: &&A) -> bool`

##### `impl<A: Array> PartialOrd for TinyVec<A>`

- `fn partial_cmp(self: &Self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<P, T> Receiver for TinyVec<A>`

- `type Target = T`

##### `impl<T> ToString for TinyVec<A>`

- `fn to_string(self: &Self) -> String`

##### `impl<A: Array> UpperExp for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for TinyVec<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

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

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

##### `impl<I> IntoIterator for TinyVecDrain<'p, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'p, A: Array> Iterator for TinyVecDrain<'p, A>`

- `type Item = <A as Array>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn last(self: Self) -> Option<<Self as >::Item>`

- `fn count(self: Self) -> usize`

- `fn for_each<F: FnMut(<Self as >::Item)>(self: Self, f: F)`

### `TinyVecIterator<A: Array>`

```rust
enum TinyVecIterator<A: Array> {
    Inline(ArrayVecIterator<A>),
    Heap(alloc::vec::IntoIter<<A as >::Item>),
}
```

Iterator for consuming an `TinyVec` and returning owned elements.

#### Implementations

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array> Debug for TinyVecIterator<A>`

- `fn fmt(self: &Self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for TinyVecIterator<A>`

- `fn next_back(self: &mut Self) -> Option<<Self as >::Item>`

- `fn nth_back(self: &mut Self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for TinyVecIterator<A>`

- `fn len(self: &Self) -> usize`

##### `impl<A: Array> FusedIterator for TinyVecIterator<A>`

##### `impl<I> IntoIterator for TinyVecIterator<A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<A: Array> Iterator for TinyVecIterator<A>`

- `type Item = <A as Array>::Item`

- `fn next(self: &mut Self) -> Option<<Self as >::Item>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

- `fn count(self: Self) -> usize`

- `fn last(self: Self) -> Option<<Self as >::Item>`

- `fn nth(self: &mut Self, n: usize) -> Option<<A as >::Item>` — [`Array`](#array)

## Traits

### `Array`

```rust
trait Array { ... }
```

A trait for types that are an array.

An "array", for our purposes, has the following properties:
* Owns some number of elements.
* The element type can be generic, but must implement [`Default`](#default).
* The capacity is fixed at compile time, based on the implementing type.
* You can get a shared or mutable slice to the elements.

You are generally **not** expected to need to implement this yourself. It is
already implemented for all array lengths.

**Additional lengths can easily be added upon request.**

## Safety Reminder

Just a reminder: this trait is 100% safe, which means that `unsafe` code
**must not** rely on an instance of this trait being correct.

#### Required Methods

- `type Item: 1`

- `const CAPACITY: usize`

- `fn as_slice(self: &Self) -> &[<Self as >::Item]`

  Gives a shared slice over the whole thing.

- `fn as_slice_mut(self: &mut Self) -> &mut [<Self as >::Item]`

  Gives a unique slice over the whole thing.

- `fn default() -> Self`

  Create a default-initialized instance of ourself, similar to the

## Macros

### `array_vec!`

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

