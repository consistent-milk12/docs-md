*[tinyvec](../index.md) / [arrayvec](index.md)*

---

# Module `arrayvec`

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

- `const fn from_array_empty(data: A) -> Self`

#### Trait Implementations

##### `impl<A: Array> AsMut for ArrayVec<A>`

- `fn as_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](../index.md)

##### `impl<A: Array> AsRef for ArrayVec<A>`

- `fn as_ref(self: &Self) -> &[<A as >::Item]` — [`Array`](../index.md)

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

- `fn eq(self: &Self, other: &&[<A as >::Item]) -> bool` — [`Array`](../index.md)

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

- `fn next_back(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](../index.md)

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

- `fn next(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](../index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `TryFromSliceError`

```rust
struct TryFromSliceError(());
```

The error type returned when a conversion from a slice to an [`ArrayVec`](../index.md)
fails.

#### Trait Implementations

##### `impl Clone for TryFromSliceError`

- `fn clone(self: &Self) -> TryFromSliceError` — [`TryFromSliceError`](../index.md)

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

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](../index.md)

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

- `fn nth(self: &mut Self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md)

