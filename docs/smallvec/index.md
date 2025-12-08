# Crate `smallvec`

Small vectors in various sizes. These store a certain number of elements inline, and fall back
to the heap for larger allocations.  This can be a useful optimization for improving cache
locality and reducing allocator traffic for workloads that fit within the inline buffer.

## `no_std` support

By default, `smallvec` does not depend on `std`.  However, the optional
`write` feature implements the `std::io::Write` trait for vectors of `u8`.
When this feature is enabled, `smallvec` depends on `std`.

## Optional features

### `serde`

When this optional dependency is enabled, `SmallVec` implements the `serde::Serialize` and
`serde::Deserialize` traits.

### `write`

When this feature is enabled, `SmallVec<[u8; _]>` implements the `std::io::Write` trait.
This feature is not compatible with `#![no_std]` programs.

### `union`

**This feature requires Rust 1.49.**

When the `union` feature is enabled `smallvec` will track its state (inline or spilled)
without the use of an enum tag, reducing the size of the `smallvec` by one machine word.
This means that there is potentially no space overhead compared to `Vec`.
Note that `smallvec` can still be larger than `Vec` if the inline buffer is larger than two
machine words.

To use this feature add `features = ["union"]` in the `smallvec` section of Cargo.toml.
Note that this feature requires Rust 1.49.

Tracking issue: [rust-lang/rust#55149](https://github.com/rust-lang/rust/issues/55149)

### `const_generics`

**This feature requires Rust 1.51.**

When this feature is enabled, `SmallVec` works with any arrays of any size, not just a fixed
list of sizes.

### `const_new`

**This feature requires Rust 1.51.**

This feature exposes the functions `SmallVec::new_const`, `SmallVec::from_const`, and `smallvec_inline` which enables the `SmallVec` to be initialized from a const context.
For details, see the
[Rust Reference](https://doc.rust-lang.org/reference/const_eval.html#const-functions).

### `drain_filter`

**This feature is unstable.** It may change to match the unstable `drain_filter` method in libstd.

Enables the `drain_filter` method, which produces an iterator that calls a user-provided
closure to determine which elements of the vector to remove and yield from the iterator.

### `drain_keep_rest`

**This feature is unstable.** It may change to match the unstable `drain_keep_rest` method in libstd.

Enables the `DrainFilter::keep_rest` method.

### `specialization`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

When this feature is enabled, `SmallVec::from(slice)` has improved performance for slices
of `Copy` types.  (Without this feature, you can use `SmallVec::from_slice` to get optimal
performance for `Copy` types.)

Tracking issue: [rust-lang/rust#31844](https://github.com/rust-lang/rust/issues/31844)

### `may_dangle`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

This feature makes the Rust compiler less strict about use of vectors that contain borrowed
references. For details, see the
[Rustonomicon](https://doc.rust-lang.org/1.42.0/nomicon/dropck.html#an-escape-hatch).

Tracking issue: [rust-lang/rust#34761](https://github.com/rust-lang/rust/issues/34761)

## Structs

### `Drain<'a, T: 'a + Array>`

```rust
struct Drain<'a, T: 'a + Array> {
    tail_start: usize,
    tail_len: usize,
    iter: slice::Iter<'a, <T as >::Item>,
    vec: core::ptr::NonNull<SmallVec<T>>,
}
```

An iterator that removes the items from a `SmallVec` and yields them by value.

Returned from [`SmallVec::drain`][1].


#### Trait Implementations

##### `impl<'a, T: 'a + Array> Debug for Drain<'a, T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, T: 'a + Array> DoubleEndedIterator for Drain<'a, T>`

- `fn next_back(self: &mut Self) -> Option<<T as >::Item>` — [`Array`](#array)

##### `impl<'a, T: 'a + Array> Drop for Drain<'a, T>`

- `fn drop(self: &mut Self)`

##### `impl<'a, T: Array> ExactSizeIterator for Drain<'a, T>`

- `fn len(self: &Self) -> usize`

##### `impl<'a, T: Array> FusedIterator for Drain<'a, T>`

##### `impl<I> IntoIterator for Drain<'a, T>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, T: 'a + Array> Iterator for Drain<'a, T>`

- `type Item = <T as Array>::Item`

- `fn next(self: &mut Self) -> Option<<T as >::Item>` — [`Array`](#array)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl<'a, T: Send + Array> Send for Drain<'a, T>`

##### `impl<'a, T: Sync + Array> Sync for Drain<'a, T>`

### `SmallVec<A: Array>`

```rust
struct SmallVec<A: Array> {
    capacity: usize,
    data: SmallVecData<A>,
}
```

A `Vec`-like container that can store a small number of elements inline.

`SmallVec` acts like a vector, but can store a limited amount of data inline within the
`SmallVec` struct rather than in a separate allocation.  If the data exceeds this limit, the
`SmallVec` will "spill" its data onto the heap, allocating a new buffer to hold it.

The amount of data that a `SmallVec` can store inline depends on its backing store. The backing
store can be any type that implements the `Array` trait; usually it is a small fixed-sized
array.  For example a `SmallVec<[u64; 8]>` can hold up to eight 64-bit integers inline.

## Example

```rust
use smallvec::SmallVec;
let mut v = SmallVec::<[u8; 4]>::new(); // initialize an empty vector

// The vector can hold up to 4 items without spilling onto the heap.
v.extend(0..4);
assert_eq!(v.len(), 4);
assert!(!v.spilled());

// Pushing another element will force the buffer to spill:
v.push(4);
assert_eq!(v.len(), 5);
assert!(v.spilled());
```

#### Implementations

- `fn from_slice(slice: &[<A as >::Item]) -> Self` — [`Array`](#array)

- `fn insert_from_slice(self: &mut Self, index: usize, slice: &[<A as >::Item])` — [`Array`](#array)

- `fn extend_from_slice(self: &mut Self, slice: &[<A as >::Item])` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array> AsMut for SmallVec<A>`

- `fn as_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for SmallVec<A>`

- `fn as_ref(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Clone for SmallVec<A>`

- `fn clone(self: &Self) -> SmallVec<A>` — [`SmallVec`](#smallvec)

- `fn clone_from(self: &mut Self, source: &Self)`

##### `impl<A: Array> Debug for SmallVec<A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: Array> Default for SmallVec<A>`

- `fn default() -> SmallVec<A>` — [`SmallVec`](#smallvec)

##### `impl<A: Array> Deref for SmallVec<A>`

- `type Target = [<A as Array>::Item]`

- `fn deref(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> DerefMut for SmallVec<A>`

- `fn deref_mut(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Drop for SmallVec<A>`

- `fn drop(self: &mut Self)`

##### `impl<A: Array> Eq for SmallVec<A>`

##### `impl<A: Array> Extend for SmallVec<A>`

- `fn extend<I: IntoIterator<Item = <A as >::Item>>(self: &mut Self, iterable: I)`

##### `impl<A: Array> FromIterator for SmallVec<A>`

- `fn from_iter<I: IntoIterator<Item = <A as >::Item>>(iterable: I) -> SmallVec<A>` — [`SmallVec`](#smallvec)

##### `impl<A: Array> Hash for SmallVec<A>`

- `fn hash<H: Hasher>(self: &Self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for SmallVec<A>`

- `type Output = <I as SliceIndex>::Output`

- `fn index(self: &Self, index: I) -> &<I as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for SmallVec<A>`

- `fn index_mut(self: &mut Self, index: I) -> &mut <I as >::Output`

##### `impl<A: Array> IntoIterator for SmallVec<A>`

- `type IntoIter = IntoIter<A>`

- `type Item = <A as Array>::Item`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

##### `impl<A: Array> Ord for SmallVec<A>`

- `fn cmp(self: &Self, other: &SmallVec<A>) -> cmp::Ordering` — [`SmallVec`](#smallvec)

##### `impl<A: Array, B: Array> PartialEq for SmallVec<A>`

- `fn eq(self: &Self, other: &SmallVec<B>) -> bool` — [`SmallVec`](#smallvec)

##### `impl<A: Array> PartialOrd for SmallVec<A>`

- `fn partial_cmp(self: &Self, other: &SmallVec<A>) -> Option<cmp::Ordering>` — [`SmallVec`](#smallvec)

##### `impl<P, T> Receiver for SmallVec<A>`

- `type Target = T`

##### `impl<A: Array> Send for SmallVec<A>`

### `IntoIter<A: Array>`

```rust
struct IntoIter<A: Array> {
    data: SmallVec<A>,
    current: usize,
    end: usize,
}
```

An iterator that consumes a `SmallVec` and yields its items by value.

Returned from [`SmallVec::into_iter`][1].


#### Implementations

- `fn as_slice(self: &Self) -> &[<A as >::Item]` — [`Array`](#array)

- `fn as_mut_slice(self: &mut Self) -> &mut [<A as >::Item]` — [`Array`](#array)

#### Trait Implementations

##### `impl<A: Array + Clone> Clone for IntoIter<A>`

- `fn clone(self: &Self) -> IntoIter<A>` — [`IntoIter`](#intoiter)

##### `impl<A: Array> Debug for IntoIter<A>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for IntoIter<A>`

- `fn next_back(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<A: Array> Drop for IntoIter<A>`

- `fn drop(self: &mut Self)`

##### `impl<A: Array> ExactSizeIterator for IntoIter<A>`

##### `impl<A: Array> FusedIterator for IntoIter<A>`

##### `impl<I> IntoIterator for IntoIter<A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<A: Array> Iterator for IntoIter<A>`

- `type Item = <A as Array>::Item`

- `fn next(self: &mut Self) -> Option<<A as >::Item>` — [`Array`](#array)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Enums

### `CollectionAllocErr`

```rust
enum CollectionAllocErr {
    CapacityOverflow,
    AllocErr {
        layout: alloc::alloc::Layout,
    },
}
```

Error type for APIs with fallible heap allocation

#### Variants

- **`CapacityOverflow`**

  Overflow `usize::MAX` or other error during size computation

- **`AllocErr`**

  The allocator return an error

#### Trait Implementations

##### `impl Debug for CollectionAllocErr`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for CollectionAllocErr`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for CollectionAllocErr`

- `fn to_string(self: &Self) -> String`

## Traits

### `Array`

```rust
trait Array { ... }
```

Types that can be used as the backing store for a [`SmallVec`](#smallvec).

#### Required Methods

- `type Item`

- `fn size() -> usize`

  Returns the number of items the array can hold.

### `ToSmallVec<A: Array>`

```rust
trait ToSmallVec<A: Array> { ... }
```

Convenience trait for constructing a `SmallVec`

#### Required Methods

- `fn to_smallvec(self: &Self) -> SmallVec<A>`

  Construct a new `SmallVec` from a slice.

## Macros

### `smallvec!`

Creates a [`SmallVec`](#smallvec) containing the arguments.

`smallvec!` allows `SmallVec`s to be defined with the same syntax as array expressions.
There are two forms of this macro:

- Create a [`SmallVec`](#smallvec) containing a given list of elements:

```rust
use smallvec::{smallvec, SmallVec};
fn main() {
let v: SmallVec<[_; 128]> = smallvec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
}
```

- Create a [`SmallVec`](#smallvec) from a given element and size:

```rust
use smallvec::{smallvec, SmallVec};
fn main() {
let v: SmallVec<[_; 0x8000]> = smallvec![1; 3];
assert_eq!(v, SmallVec::from_buf([1, 1, 1]));
}
```

Note that unlike array expressions this syntax supports all elements
which implement `Clone` and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `smallvec![Rc::new(1); 5]` will create a vector of five references
to the same boxed integer value, not five references pointing to independently
boxed integers.

