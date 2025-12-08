*[tinyvec](../index.md) / [tinyvec](index.md)*

---

# Module `tinyvec`

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

- `fn next_back(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](../index.md)

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

- `fn next(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](../index.md)

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

- `fn swap_remove(self: &mut Self, index: usize) -> <A as >::Item` — [`Array`](../index.md)

- `fn pop(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](../index.md)

- `fn remove(self: &mut Self, index: usize) -> <A as >::Item` — [`Array`](../index.md)

- `fn len(self: &Self) -> usize`

- `fn capacity(self: &Self) -> usize`

- `fn truncate(self: &mut Self, new_len: usize)`

- `fn as_mut_ptr(self: &mut Self) -> *mut <A as >::Item` — [`Array`](../index.md)

- `fn as_ptr(self: &Self) -> *const <A as >::Item` — [`Array`](../index.md)

- `fn retain<F: FnMut(&<A as >::Item) -> bool>(self: &mut Self, acceptable: F)`

- `fn as_mut_slice(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](../index.md)

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](../index.md)

- `fn clear(self: &mut Self)`

- `fn drain<R: RangeBounds<usize>>(self: &mut Self, range: R) -> TinyVecDrain<'_, A>` — [`TinyVecDrain`](../index.md)

- `fn extend_from_slice(self: &mut Self, sli: &[<A as >::Item])` — [`Array`](../index.md)

- `fn from_array_len(data: A, len: usize) -> Self`

- `fn insert(self: &mut Self, index: usize, item: <A as >::Item)` — [`Array`](../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn new() -> Self`

- `fn push(self: &mut Self, val: <A as >::Item)` — [`Array`](../index.md)

- `fn resize(self: &mut Self, new_len: usize, new_val: <A as >::Item)` — [`Array`](../index.md)

- `fn resize_with<F: FnMut() -> <A as >::Item>(self: &mut Self, new_len: usize, f: F)`

- `fn split_off(self: &mut Self, at: usize) -> Self`

- `fn splice<R, I>(self: &mut Self, range: R, replacement: I) -> TinyVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`TinyVecSplice`](../index.md)

- `fn try_from_array_len(data: A, len: usize) -> Result<Self, A>`

#### Trait Implementations

##### `impl<A: Array> AsMut for TinyVec<A>`

- `fn as_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](../index.md)

##### `impl<A: Array> AsRef for TinyVec<A>`

- `fn as_ref(self: &Self) -> &[<A as >::Item]` — [`Array`](../index.md)

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

- `fn eq(self: &Self, other: &&[<A as >::Item]) -> bool` — [`Array`](../index.md)

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

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](../index.md)

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

- `fn nth(self: &mut Self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md)

