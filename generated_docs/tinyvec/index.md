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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:106-109`](../../.source_1765521767/tinyvec-1.10.0/src/arrayvec.rs#L106-L109)*

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

  Move all values from `other` into this vec.

  

  ## Panics

  * If the vec overflows its capacity

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 10] => 1, 2, 3);

  let mut av2 = array_vec!([i32; 10] => 4, 5, 6);

  av.append(&mut av2);

  assert_eq!(av, &[1, 2, 3, 4, 5, 6][..]);

  assert_eq!(av2, &[][..]);

  ```

- <span id="arrayvec-try-append"></span>`fn try_append<'other>(&mut self, other: &'other mut Self) -> Option<&'other mut Self>`

  Move all values from `other` into this vec.

  If appending would overflow the capacity, Some(other) is returned.

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 7] => 1, 2, 3);

  let mut av2 = array_vec!([i32; 7] => 4, 5, 6);

  av.append(&mut av2);

  assert_eq!(av, &[1, 2, 3, 4, 5, 6][..]);

  assert_eq!(av2, &[][..]);

  

  let mut av3 = array_vec!([i32; 7] => 7, 8, 9);

  assert!(av.try_append(&mut av3).is_some());

  assert_eq!(av, &[1, 2, 3, 4, 5, 6][..]);

  assert_eq!(av3, &[7, 8, 9][..]);

  ```

- <span id="arrayvec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut <A as >::Item` — [`Array`](#array)

  A `*mut` pointer to the backing array.

  

  ## Safety

  

  This pointer has provenance over the _entire_ backing array.

- <span id="arrayvec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

  Performs a `deref_mut`, into unique slice form.

- <span id="arrayvec-as-ptr"></span>`fn as_ptr(&self) -> *const <A as >::Item` — [`Array`](#array)

  A `*const` pointer to the backing array.

  

  ## Safety

  

  This pointer has provenance over the _entire_ backing array.

- <span id="arrayvec-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

  Performs a `deref`, into shared slice form.

- <span id="arrayvec-capacity"></span>`fn capacity(&self) -> usize`

  The capacity of the `ArrayVec`.

  

  This is fixed based on the array type, but can't yet be made a `const fn`

  on Stable Rust.

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

  Truncates the `ArrayVec` down to length 0.

- <span id="arrayvec-drain"></span>`fn drain<R>(&mut self, range: R) -> ArrayVecDrain<'_, <A as >::Item>` — [`ArrayVecDrain`](#arrayvecdrain), [`Array`](#array)

  Creates a draining iterator that removes the specified range in the vector

  and yields the removed items.

  

  ## Panics

  * If the start is greater than the end

  * If the end is past the edge of the vec.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 4] => 1, 2, 3);

  let av2: ArrayVec<[i32; 4]> = av.drain(1..).collect();

  assert_eq!(av.as_slice(), &[1][..]);

  assert_eq!(av2.as_slice(), &[2, 3][..]);

  

  av.drain(..);

  assert_eq!(av.as_slice(), &[]);

  ```

- <span id="arrayvec-into-inner"></span>`fn into_inner(self) -> A`

  Returns the inner array of the `ArrayVec`.

  

  This returns the full array, even if the `ArrayVec` length is currently

  less than that.

  

  ## Example

  

  ```rust

  use tinyvec::{array_vec, ArrayVec};

  let mut favorite_numbers = array_vec!([i32; 5] => 87, 48, 33, 9, 26);

  assert_eq!(favorite_numbers.clone().into_inner(), [87, 48, 33, 9, 26]);

  

  favorite_numbers.pop();

  assert_eq!(favorite_numbers.into_inner(), [87, 48, 33, 9, 0]);

  ```

  

  A use for this function is to build an array from an iterator by first

  collecting it into an `ArrayVec`.

  

  ```rust

  use tinyvec::ArrayVec;

  let arr_vec: ArrayVec<[i32; 10]> = (1..=3).cycle().take(10).collect();

  let inner = arr_vec.into_inner();

  assert_eq!(inner, [1, 2, 3, 1, 2, 3, 1, 2, 3, 1]);

  ```

- <span id="arrayvec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[<A as >::Item])` — [`Array`](#array)

  Clone each element of the slice into this `ArrayVec`.

  

  ## Panics

  * If the `ArrayVec` would overflow, this will panic.

- <span id="arrayvec-fill"></span>`fn fill<I: IntoIterator<Item = <A as >::Item>>(&mut self, iter: I) -> <I as >::IntoIter`

  Fill the vector until its capacity has been reached.

  

  Successively fills unused space in the spare slice of the vector with

  elements from the iterator. It then returns the remaining iterator

  without exhausting it. This also allows appending the head of an

  infinite iterator.

  

  This is an alternative to `Extend::extend` method for cases where the

  length of the iterator can not be checked. Since this vector can not

  reallocate to increase its capacity, it is unclear what to do with

  remaining elements in the iterator and the iterator itself. The

  interface also provides no way to communicate this to the caller.

  

  ## Panics

  * If the `next` method of the provided iterator panics.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 4]);

  let mut to_inf = av.fill(0..);

  assert_eq!(&av[..], [0, 1, 2, 3]);

  assert_eq!(to_inf.next(), Some(4));

  ```

- <span id="arrayvec-from-array-len"></span>`fn from_array_len(data: A, len: usize) -> Self`

  Wraps up an array and uses the given length as the initial length.

  

  If you want to simply use the full array, use `from` instead.

  

  ## Panics

  

  * The length specified must be less than or equal to the capacity of the

    array.

- <span id="arrayvec-insert"></span>`fn insert(&mut self, index: usize, item: <A as >::Item)` — [`Array`](#array)

  Inserts an item at the position given, moving all following elements +1

  index.

  

  ## Panics

  * If `index` > `len`

  * If the capacity is exhausted

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 10] => 1, 2, 3);

  av.insert(1, 4);

  assert_eq!(av.as_slice(), &[1, 4, 2, 3]);

  av.insert(4, 5);

  assert_eq!(av.as_slice(), &[1, 4, 2, 3, 5]);

  ```

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, item: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](#array)

  Tries to insert an item at the position given, moving all following

  elements +1 index.

  Returns back the element if the capacity is exhausted,

  otherwise returns None.

  

  ## Panics

  * If `index` > `len`

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([&'static str; 4] => "one", "two", "three");

  av.insert(1, "four");

  assert_eq!(av.as_slice(), &["one", "four", "two", "three"]);

  assert_eq!(av.try_insert(4, "five"), Some("five"));

  ```

- <span id="arrayvec-is-empty"></span>`fn is_empty(&self) -> bool`

  Checks if the length is 0.

- <span id="arrayvec-len"></span>`fn len(&self) -> usize`

  The length of the `ArrayVec` (in elements).

- <span id="arrayvec-new"></span>`fn new() -> Self`

  Makes a new, empty `ArrayVec`.

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

  Remove and return the last element of the vec, if there is one.

  

  ## Failure

  * If the vec is empty you get `None`.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 10] => 1, 2);

  assert_eq!(av.pop(), Some(2));

  assert_eq!(av.pop(), Some(1));

  assert_eq!(av.pop(), None);

  ```

- <span id="arrayvec-push"></span>`fn push(&mut self, val: <A as >::Item)` — [`Array`](#array)

  Place an element onto the end of the vec.

  

  ## Panics

  * If the length of the vec would overflow the capacity.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 2]);

  assert_eq!(&av[..], []);

  av.push(1);

  assert_eq!(&av[..], [1]);

  av.push(2);

  assert_eq!(&av[..], [1, 2]);

  // av.push(3); this would overflow the ArrayVec and panic!

  ```

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, val: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](#array)

  Tries to place an element onto the end of the vec.\

  Returns back the element if the capacity is exhausted,

  otherwise returns None.

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 2]);

  assert_eq!(av.as_slice(), []);

  assert_eq!(av.try_push(1), None);

  assert_eq!(&av[..], [1]);

  assert_eq!(av.try_push(2), None);

  assert_eq!(&av[..], [1, 2]);

  assert_eq!(av.try_push(3), Some(3));

  ```

- <span id="arrayvec-remove"></span>`fn remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](#array)

  Removes the item at `index`, shifting all others down by one index.

  

  Returns the removed element.

  

  ## Panics

  

  * If the index is out of bounds.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 4] => 1, 2, 3);

  assert_eq!(av.remove(1), 2);

  assert_eq!(&av[..], [1, 3]);

  ```

- <span id="arrayvec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: <A as >::Item)` — [`Array`](#array)

  As [`resize_with`](ArrayVec::resize_with)

  and it clones the value as the closure.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  

  let mut av = array_vec!([&str; 10] => "hello");

  av.resize(3, "world");

  assert_eq!(&av[..], ["hello", "world", "world"]);

  

  let mut av = array_vec!([i32; 10] => 1, 2, 3, 4);

  av.resize(2, 0);

  assert_eq!(&av[..], [1, 2]);

  ```

- <span id="arrayvec-resize-with"></span>`fn resize_with<F: FnMut() -> <A as >::Item>(&mut self, new_len: usize, f: F)`

  Resize the vec to the new length.

  

  If it needs to be longer, it's filled with repeated calls to the provided

  function. If it needs to be shorter, it's truncated.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  

  let mut av = array_vec!([i32; 10] => 1, 2, 3);

  av.resize_with(5, Default::default);

  assert_eq!(&av[..], [1, 2, 3, 0, 0]);

  

  let mut av = array_vec!([i32; 10]);

  let mut p = 1;

  av.resize_with(4, || {

    p *= 2;

    p

  });

  assert_eq!(&av[..], [2, 4, 8, 16]);

  ```

- <span id="arrayvec-retain"></span>`fn retain<F: FnMut(&<A as >::Item) -> bool>(&mut self, acceptable: F)`

  Walk the vec and keep only the elements that pass the predicate given.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  

  let mut av = array_vec!([i32; 10] => 1, 1, 2, 3, 3, 4);

  av.retain(|&x| x % 2 == 0);

  assert_eq!(&av[..], [2, 4]);

  ```

- <span id="arrayvec-retain-mut"></span>`fn retain_mut<F>(&mut self, acceptable: F)`

  Retains only the elements specified by the predicate, passing a mutable

  reference to it.

  

  In other words, remove all elements e such that f(&mut e) returns false.

  This method operates in place, visiting each element exactly once in the

  original order, and preserves the order of the retained elements.

  

  

  ## Example

  

  ```rust

  use tinyvec::*;

  

  let mut av = array_vec!([i32; 10] => 1, 1, 2, 3, 3, 4);

  av.retain_mut(|x| if *x % 2 == 0 { *x *= 2; true } else { false });

  assert_eq!(&av[..], [4, 8]);

  ```

- <span id="arrayvec-set-len"></span>`fn set_len(&mut self, new_len: usize)`

  Forces the length of the vector to `new_len`.

  

  ## Panics

  * If `new_len` is greater than the vec's capacity.

  

  ## Safety

  * This is a fully safe operation! The inactive memory already counts as

    "initialized" by Rust's rules.

  * Other than "the memory is initialized" there are no other guarantees

    regarding what you find in the inactive portion of the vec.

- <span id="arrayvec-split-off"></span>`fn split_off(&mut self, at: usize) -> Self`

  Splits the collection at the point given.

  

  * `[0, at)` stays in this vec

  * `[at, len)` ends up in the new vec.

  

  ## Panics

  * if at > len

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 4] => 1, 2, 3);

  let av2 = av.split_off(1);

  assert_eq!(&av[..], [1]);

  assert_eq!(&av2[..], [2, 3]);

  ```

- <span id="arrayvec-splice"></span>`fn splice<R, I>(&mut self, range: R, replacement: I) -> ArrayVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`ArrayVecSplice`](#arrayvecsplice)

  Creates a splicing iterator that removes the specified range in the

  vector, yields the removed items, and replaces them with elements from

  the provided iterator.

  

  `splice` fuses the provided iterator, so elements after the first `None`

  are ignored.

  

  ## Panics

  * If the start is greater than the end.

  * If the end is past the edge of the vec.

  * If the provided iterator panics.

  * If the new length would overflow the capacity of the array. Because

    `ArrayVecSplice` adds elements to this vec in its destructor when

    necessary, this panic would occur when it is dropped.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([i32; 4] => 1, 2, 3);

  let av2: ArrayVec<[i32; 4]> = av.splice(1.., 4..=6).collect();

  assert_eq!(av.as_slice(), &[1, 4, 5, 6][..]);

  assert_eq!(av2.as_slice(), &[2, 3][..]);

  

  av.splice(.., None);

  assert_eq!(av.as_slice(), &[]);

  ```

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](#array)

  Remove an element, swapping the end of the vec into its place.

  

  ## Panics

  * If the index is out of bounds.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut av = array_vec!([&str; 4] => "foo", "bar", "quack", "zap");

  

  assert_eq!(av.swap_remove(1), "bar");

  assert_eq!(&av[..], ["foo", "zap", "quack"]);

  

  assert_eq!(av.swap_remove(0), "foo");

  assert_eq!(&av[..], ["quack", "zap"]);

  ```

- <span id="arrayvec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

  Reduces the vec's length to the given value.

  

  If the vec is already shorter than the input, nothing happens.

- <span id="arrayvec-try-from-array-len"></span>`fn try_from_array_len(data: A, len: usize) -> Result<Self, A>`

  Wraps an array, using the given length as the starting length.

  

  If you want to use the whole length of the array, you can just use the

  `From` impl.

  

  ## Failure

  

  If the given length is greater than the capacity of the array this will

  error, and you'll get the array back in the `Err`.

#### Trait Implementations

##### `impl Any for ArrayVec<A>`

- <span id="arrayvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<A: Array> AsMut for ArrayVec<A>`

- <span id="arrayvec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for ArrayVec<A>`

- <span id="arrayvec-asref-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Binary for ArrayVec<A>`

- <span id="arrayvec-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Borrow for ArrayVec<A>`

- <span id="arrayvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVec<A>`

- <span id="arrayvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A> Clone for ArrayVec<A>`

- <span id="arrayvec-clone"></span>`fn clone(&self) -> Self`

- <span id="arrayvec-clone-clone-from"></span>`fn clone_from(&mut self, o: &Self)`

##### `impl CloneToUninit for ArrayVec<A>`

- <span id="arrayvec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A> Copy for ArrayVec<A>`

##### `impl<A: Array> Debug for ArrayVec<A>`

- <span id="arrayvec-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for ArrayVec<A>`

- <span id="arrayvec-default"></span>`fn default() -> Self`

##### `impl<A: Array> Deref for ArrayVec<A>`

- <span id="arrayvec-deref-type-target"></span>`type Target = [<A as Array>::Item]`

- <span id="arrayvec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for ArrayVec<A>`

- <span id="arrayvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for ArrayVec<A>`

- <span id="arrayvec-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for ArrayVec<A>`

##### `impl<A: Array> Extend for ArrayVec<A>`

- <span id="arrayvec-extend"></span>`fn extend<T: IntoIterator<Item = <A as >::Item>>(&mut self, iter: T)`

##### `impl<T> From for ArrayVec<A>`

- <span id="arrayvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: Array> FromIterator for ArrayVec<A>`

- <span id="arrayvec-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for ArrayVec<A>`

- <span id="arrayvec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for ArrayVec<A>`

- <span id="arrayvec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="arrayvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for ArrayVec<A>`

- <span id="arrayvec-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<U> Into for ArrayVec<A>`

- <span id="arrayvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<A: Array> IntoIterator for ArrayVec<A>`

- <span id="arrayvec-intoiterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayvec-intoiterator-type-intoiter"></span>`type IntoIter = ArrayVecIterator<A>`

- <span id="arrayvec-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for ArrayVec<A>`

- <span id="arrayvec-lowerexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for ArrayVec<A>`

- <span id="arrayvec-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for ArrayVec<A>`

- <span id="arrayvec-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for ArrayVec<A>`

- <span id="arrayvec-ord-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for ArrayVec<A>`

- <span id="arrayvec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<A: Array> PartialOrd for ArrayVec<A>`

- <span id="arrayvec-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for ArrayVec<A>`

- <span id="arrayvec-pointer-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl Receiver for ArrayVec<A>`

- <span id="arrayvec-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for ArrayVec<A>`

- <span id="arrayvec-toowned-type-owned"></span>`type Owned = T`

- <span id="arrayvec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="arrayvec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for ArrayVec<A>`

- <span id="arrayvec-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ArrayVec<A>`

- <span id="arrayvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVec<A>`

- <span id="arrayvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<A: Array> UpperExp for ArrayVec<A>`

- <span id="arrayvec-upperexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for ArrayVec<A>`

- <span id="arrayvec-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `ArrayVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct ArrayVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut ArrayVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1264-1269`](../../.source_1765521767/tinyvec-1.10.0/src/arrayvec.rs#L1264-L1269)*

Splicing iterator for `ArrayVec`
See [`ArrayVec::splice`](ArrayVec::<A>::splice)

#### Trait Implementations

##### `impl Any for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A, I> DoubleEndedIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Drop for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<A, I> ExactSizeIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, I> FusedIterator for ArrayVecSplice<'p, A, I>`

##### `impl<U> Into for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecsplice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecsplice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Iterator for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayvecsplice-iterator-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="arrayvecsplice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvecsplice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVecSplice<'p, A, I>`

- <span id="arrayvecsplice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvecsplice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TryFromSliceError`

```rust
struct TryFromSliceError(());
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1425`](../../.source_1765521767/tinyvec-1.10.0/src/arrayvec.rs#L1425)*

The error type returned when a conversion from a slice to an [`ArrayVec`](#arrayvec)
fails.

#### Trait Implementations

##### `impl Any for TryFromSliceError`

- <span id="tryfromsliceerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFromSliceError`

- <span id="tryfromsliceerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFromSliceError`

- <span id="tryfromsliceerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TryFromSliceError`

- <span id="tryfromsliceerror-clone"></span>`fn clone(&self) -> TryFromSliceError` — [`TryFromSliceError`](#tryfromsliceerror)

##### `impl CloneToUninit for TryFromSliceError`

- <span id="tryfromsliceerror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for TryFromSliceError`

##### `impl Debug for TryFromSliceError`

- <span id="tryfromsliceerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for TryFromSliceError`

- <span id="tryfromsliceerror-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for TryFromSliceError`

- <span id="tryfromsliceerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TryFromSliceError`

- <span id="tryfromsliceerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for TryFromSliceError`

- <span id="tryfromsliceerror-toowned-type-owned"></span>`type Owned = T`

- <span id="tryfromsliceerror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tryfromsliceerror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for TryFromSliceError`

- <span id="tryfromsliceerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for TryFromSliceError`

- <span id="tryfromsliceerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tryfromsliceerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TryFromSliceError`

- <span id="tryfromsliceerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tryfromsliceerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArrayVecIterator<A: Array>`

```rust
struct ArrayVecIterator<A: Array> {
    base: u16,
    tail: u16,
    data: A,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1478-1482`](../../.source_1765521767/tinyvec-1.10.0/src/arrayvec.rs#L1478-L1482)*

Iterator for consuming an `ArrayVec` and returning owned elements.

#### Implementations

- <span id="arrayveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

  Returns the remaining items of this iterator as a slice.

#### Trait Implementations

##### `impl Any for ArrayVecIterator<A>`

- <span id="arrayveciterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVecIterator<A>`

- <span id="arrayveciterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVecIterator<A>`

- <span id="arrayveciterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: Array> Debug for ArrayVecIterator<A>`

- <span id="arrayveciterator-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for ArrayVecIterator<A>`

- <span id="arrayveciterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: Array> FusedIterator for ArrayVecIterator<A>`

##### `impl<U> Into for ArrayVecIterator<A>`

- <span id="arrayveciterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayveciterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayveciterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for ArrayVecIterator<A>`

- <span id="arrayveciterator-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="arrayveciterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayveciterator-iterator-count"></span>`fn count(self) -> usize`

- <span id="arrayveciterator-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayveciterator-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<U> TryFrom for ArrayVecIterator<A>`

- <span id="arrayveciterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayveciterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVecIterator<A>`

- <span id="arrayveciterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayveciterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ArrayVecDrain<'a, T: 'a + Default>`

```rust
struct ArrayVecDrain<'a, T: 'a + Default> {
    iter: slice::IterMut<'a, T>,
}
```

*Defined in [`tinyvec-1.10.0/src/arrayvec_drain.rs:11-13`](../../.source_1765521767/tinyvec-1.10.0/src/arrayvec_drain.rs#L11-L13)*

Draining iterator for [`ArrayVec`](#arrayvec)

See [`ArrayVec::drain`](ArrayVec::drain)

#### Implementations

- <span id="arrayvecdrain-new"></span>`fn new<A, R>(arr: &'a mut ArrayVec<A>, range: R) -> Self` — [`ArrayVec`](#arrayvec)

#### Trait Implementations

##### `impl<T> Any for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: 'a + Default> DoubleEndedIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T: 'a + Default> ExactSizeIterator for ArrayVecDrain<'a, T>`

##### `impl<T> From for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: 'a + Default> FusedIterator for ArrayVecDrain<'a, T>`

##### `impl<T, U> Into for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="arrayvecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="arrayvecdrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'a + Default> Iterator for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-iterator-type-item"></span>`type Item = T`

- <span id="arrayvecdrain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="arrayvecdrain-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="arrayvecdrain-iterator-for-each"></span>`fn for_each<F>(self, f: F)`

##### `impl<T, U> TryFrom for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayvecdrain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for ArrayVecDrain<'a, T>`

- <span id="arrayvecdrain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayvecdrain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SliceVec<'s, T>`

```rust
struct SliceVec<'s, T> {
    data: &'s mut [T],
    len: usize,
}
```

*Defined in [`tinyvec-1.10.0/src/slicevec.rs:16-19`](../../.source_1765521767/tinyvec-1.10.0/src/slicevec.rs#L16-L19)*

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

  A `*mut` pointer to the backing slice.

  

  ## Safety

  

  This pointer has provenance over the _entire_ backing slice.

- <span id="slicevec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [T]`

  Performs a `deref_mut`, into unique slice form.

- <span id="slicevec-as-ptr"></span>`fn as_ptr(&self) -> *const T`

  A `*const` pointer to the backing slice.

  

  ## Safety

  

  This pointer has provenance over the _entire_ backing slice.

- <span id="slicevec-as-slice"></span>`fn as_slice(&self) -> &[T]`

  Performs a `deref`, into shared slice form.

- <span id="slicevec-capacity"></span>`fn capacity(&self) -> usize`

  The capacity of the `SliceVec`.

  

  This the length of the initial backing slice.

- <span id="slicevec-clear"></span>`fn clear(&mut self)`

  Truncates the `SliceVec` down to length 0.

- <span id="slicevec-drain"></span>`fn drain<'p, R: RangeBounds<usize>>(self: &'p mut Self, range: R) -> SliceVecDrain<'p, 's, T>` — [`SliceVecDrain`](#slicevecdrain)

  Creates a draining iterator that removes the specified range in the vector

  and yields the removed items.

  

  ## Panics

  * If the start is greater than the end

  * If the end is past the edge of the vec.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut arr = [6, 7, 8];

  let mut sv = SliceVec::from(&mut arr);

  let drained_values: ArrayVec<[i32; 4]> = sv.drain(1..).collect();

  assert_eq!(sv.as_slice(), &[6][..]);

  assert_eq!(drained_values.as_slice(), &[7, 8][..]);

  

  sv.drain(..);

  assert_eq!(sv.as_slice(), &[]);

  ```

- <span id="slicevec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[T])`

- <span id="slicevec-fill"></span>`fn fill<I: IntoIterator<Item = T>>(&mut self, iter: I) -> <I as >::IntoIter`

  Fill the vector until its capacity has been reached.

  

  Successively fills unused space in the spare slice of the vector with

  elements from the iterator. It then returns the remaining iterator

  without exhausting it. This also allows appending the head of an

  infinite iterator.

  

  This is an alternative to `Extend::extend` method for cases where the

  length of the iterator can not be checked. Since this vector can not

  reallocate to increase its capacity, it is unclear what to do with

  remaining elements in the iterator and the iterator itself. The

  interface also provides no way to communicate this to the caller.

  

  ## Panics

  * If the `next` method of the provided iterator panics.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut arr = [7, 7, 7, 7];

  let mut sv = SliceVec::from_slice_len(&mut arr, 0);

  let mut to_inf = sv.fill(0..);

  assert_eq!(&sv[..], [0, 1, 2, 3]);

  assert_eq!(to_inf.next(), Some(4));

  ```

- <span id="slicevec-from-slice-len"></span>`fn from_slice_len(data: &'s mut [T], len: usize) -> Self`

  Wraps up a slice and uses the given length as the initial length.

  

  If you want to simply use the full slice, use `from` instead.

  

  ## Panics

  

  * The length specified must be less than or equal to the capacity of the

    slice.

- <span id="slicevec-insert"></span>`fn insert(&mut self, index: usize, item: T)`

  Inserts an item at the position given, moving all following elements +1

  index.

  

  ## Panics

  * If `index` > `len`

  * If the capacity is exhausted

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut arr = [1, 2, 3, 0, 0];

  let mut sv = SliceVec::from_slice_len(&mut arr, 3);

  sv.insert(1, 4);

  assert_eq!(sv.as_slice(), &[1, 4, 2, 3]);

  sv.insert(4, 5);

  assert_eq!(sv.as_slice(), &[1, 4, 2, 3, 5]);

  ```

- <span id="slicevec-is-empty"></span>`fn is_empty(&self) -> bool`

  Checks if the length is 0.

- <span id="slicevec-len"></span>`fn len(&self) -> usize`

  The length of the `SliceVec` (in elements).

- <span id="slicevec-pop"></span>`fn pop(&mut self) -> Option<T>`

  Remove and return the last element of the vec, if there is one.

  

  ## Failure

  * If the vec is empty you get `None`.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut arr = [1, 2];

  let mut sv = SliceVec::from(&mut arr);

  assert_eq!(sv.pop(), Some(2));

  assert_eq!(sv.pop(), Some(1));

  assert_eq!(sv.pop(), None);

  ```

- <span id="slicevec-push"></span>`fn push(&mut self, val: T)`

  Place an element onto the end of the vec.

  

  ## Panics

  * If the length of the vec would overflow the capacity.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut arr = [0, 0];

  let mut sv = SliceVec::from_slice_len(&mut arr, 0);

  assert_eq!(&sv[..], []);

  sv.push(1);

  assert_eq!(&sv[..], [1]);

  sv.push(2);

  assert_eq!(&sv[..], [1, 2]);

  // sv.push(3); this would overflow the ArrayVec and panic!

  ```

- <span id="slicevec-remove"></span>`fn remove(&mut self, index: usize) -> T`

  Removes the item at `index`, shifting all others down by one index.

  

  Returns the removed element.

  

  ## Panics

  

  * If the index is out of bounds.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut arr = [1, 2, 3];

  let mut sv = SliceVec::from(&mut arr);

  assert_eq!(sv.remove(1), 2);

  assert_eq!(&sv[..], [1, 3]);

  ```

- <span id="slicevec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: T)`

  As [`resize_with`](SliceVec::resize_with)

  and it clones the value as the closure.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  // bigger

  let mut arr = ["hello", "", "", "", ""];

  let mut sv = SliceVec::from_slice_len(&mut arr, 1);

  sv.resize(3, "world");

  assert_eq!(&sv[..], ["hello", "world", "world"]);

  

  // smaller

  let mut arr = ['a', 'b', 'c', 'd'];

  let mut sv = SliceVec::from(&mut arr);

  sv.resize(2, 'z');

  assert_eq!(&sv[..], ['a', 'b']);

  ```

- <span id="slicevec-resize-with"></span>`fn resize_with<F: FnMut() -> T>(&mut self, new_len: usize, f: F)`

  Resize the vec to the new length.

  

  * If it needs to be longer, it's filled with repeated calls to the

    provided function.

  * If it needs to be shorter, it's truncated.

    * If the type needs to drop the truncated slots are filled with calls to

      the provided function.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut arr = [1, 2, 3, 7, 7, 7, 7];

  let mut sv = SliceVec::from_slice_len(&mut arr, 3);

  sv.resize_with(5, Default::default);

  assert_eq!(&sv[..], [1, 2, 3, 0, 0]);

  

  let mut arr = [0, 0, 0, 0];

  let mut sv = SliceVec::from_slice_len(&mut arr, 0);

  let mut p = 1;

  sv.resize_with(4, || {

    p *= 2;

    p

  });

  assert_eq!(&sv[..], [2, 4, 8, 16]);

  ```

- <span id="slicevec-retain"></span>`fn retain<F: FnMut(&T) -> bool>(&mut self, acceptable: F)`

  Walk the vec and keep only the elements that pass the predicate given.

  

  ## Example

  

  ```rust

  use tinyvec::*;

  

  let mut arr = [1, 1, 2, 3, 3, 4];

  let mut sv = SliceVec::from(&mut arr);

  sv.retain(|&x| x % 2 == 0);

  assert_eq!(&sv[..], [2, 4]);

  ```

- <span id="slicevec-set-len"></span>`fn set_len(&mut self, new_len: usize)`

  Forces the length of the vector to `new_len`.

  

  ## Panics

  * If `new_len` is greater than the vec's capacity.

  

  ## Safety

  * This is a fully safe operation! The inactive memory already counts as

    "initialized" by Rust's rules.

  * Other than "the memory is initialized" there are no other guarantees

    regarding what you find in the inactive portion of the vec.

- <span id="slicevec-split-off"></span>`fn split_off<'a>(self: &'a mut Self, at: usize) -> SliceVec<'s, T>` — [`SliceVec`](#slicevec)

  Splits the collection at the point given.

  

  * `[0, at)` stays in this vec (and this vec is now full).

  * `[at, len)` ends up in the new vec (with any spare capacity).

  

  ## Panics

  * if `at` > `self.len()`

  

  ## Example

  

  ```rust

  use tinyvec::*;

  let mut arr = [1, 2, 3];

  let mut sv = SliceVec::from(&mut arr);

  let sv2 = sv.split_off(1);

  assert_eq!(&sv[..], [1]);

  assert_eq!(&sv2[..], [2, 3]);

  ```

- <span id="slicevec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> T`

  Remove an element, swapping the end of the vec into its place.

  

  ## Panics

  * If the index is out of bounds.

  

  ## Example

  ```rust

  use tinyvec::*;

  let mut arr = ["foo", "bar", "quack", "zap"];

  let mut sv = SliceVec::from(&mut arr);

  

  assert_eq!(sv.swap_remove(1), "bar");

  assert_eq!(&sv[..], ["foo", "zap", "quack"]);

  

  assert_eq!(sv.swap_remove(0), "foo");

  assert_eq!(&sv[..], ["quack", "zap"]);

  ```

- <span id="slicevec-truncate"></span>`fn truncate(&mut self, new_len: usize)`

  Reduces the vec's length to the given value.

  

  If the vec is already shorter than the input, nothing happens.

- <span id="slicevec-try-from-slice-len"></span>`fn try_from_slice_len(data: &'s mut [T], len: usize) -> Option<Self>`

  Wraps a slice, using the given length as the starting length.

  

  If you want to use the whole length of the slice, you can just use the

  `From` impl.

  

  ## Failure

  

  If the given length is greater than the length of the slice you get

  `None`.

#### Trait Implementations

##### `impl<T> Any for SliceVec<'s, T>`

- <span id="slicevec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> AsMut for SliceVec<'s, T>`

- <span id="slicevec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [T]`

##### `impl<T> AsRef for SliceVec<'s, T>`

- <span id="slicevec-asref-as-ref"></span>`fn as_ref(&self) -> &[T]`

##### `impl<T> Binary for SliceVec<'s, T>`

- <span id="slicevec-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Borrow for SliceVec<'s, T>`

- <span id="slicevec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SliceVec<'s, T>`

- <span id="slicevec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T> Debug for SliceVec<'s, T>`

- <span id="slicevec-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Default for SliceVec<'s, T>`

- <span id="slicevec-default"></span>`fn default() -> Self`

##### `impl<T> Deref for SliceVec<'s, T>`

- <span id="slicevec-deref-type-target"></span>`type Target = [T]`

- <span id="slicevec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T> DerefMut for SliceVec<'s, T>`

- <span id="slicevec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Display for SliceVec<'s, T>`

- <span id="slicevec-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Eq for SliceVec<'s, T>`

##### `impl<T> Extend for SliceVec<'s, T>`

- <span id="slicevec-extend"></span>`fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I)`

##### `impl<T> From for SliceVec<'s, T>`

- <span id="slicevec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T> Hash for SliceVec<'s, T>`

- <span id="slicevec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<T, I> Index for SliceVec<'s, T>`

- <span id="slicevec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="slicevec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<T, I> IndexMut for SliceVec<'s, T>`

- <span id="slicevec-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<T, U> Into for SliceVec<'s, T>`

- <span id="slicevec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<T> IntoIterator for SliceVec<'s, T>`

- <span id="slicevec-intoiterator-type-item"></span>`type Item = &'s mut T`

- <span id="slicevec-intoiterator-type-intoiter"></span>`type IntoIter = IterMut<'s, T>`

- <span id="slicevec-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T> LowerExp for SliceVec<'s, T>`

- <span id="slicevec-lowerexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> LowerHex for SliceVec<'s, T>`

- <span id="slicevec-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Octal for SliceVec<'s, T>`

- <span id="slicevec-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Ord for SliceVec<'s, T>`

- <span id="slicevec-ord-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<T> PartialEq for SliceVec<'s, T>`

- <span id="slicevec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<T> PartialOrd for SliceVec<'s, T>`

- <span id="slicevec-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<T> Pointer for SliceVec<'s, T>`

- <span id="slicevec-pointer-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Receiver for SliceVec<'s, T>`

- <span id="slicevec-receiver-type-target"></span>`type Target = T`

##### `impl<T> ToString for SliceVec<'s, T>`

- <span id="slicevec-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<T, U> TryFrom for SliceVec<'s, T>`

- <span id="slicevec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slicevec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SliceVec<'s, T>`

- <span id="slicevec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slicevec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<T> UpperExp for SliceVec<'s, T>`

- <span id="slicevec-upperexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> UpperHex for SliceVec<'s, T>`

- <span id="slicevec-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `SliceVecDrain<'p, 's, T: Default>`

```rust
struct SliceVecDrain<'p, 's, T: Default> {
    parent: &'p mut SliceVec<'s, T>,
    target_start: usize,
    target_index: usize,
    target_end: usize,
}
```

*Defined in [`tinyvec-1.10.0/src/slicevec.rs:714-719`](../../.source_1765521767/tinyvec-1.10.0/src/slicevec.rs#L714-L719)*

Draining iterator for [`SliceVec`](#slicevec)

See [`SliceVec::drain`](SliceVec::drain)

#### Trait Implementations

##### `impl<T> Any for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<T: Default> Drop for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-drop"></span>`fn drop(&mut self)`

##### `impl<T> From for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<T: Default> FusedIterator for SliceVecDrain<'p, 's, T>`

##### `impl<T, U> Into for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slicevecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="slicevecdrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Default> Iterator for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-iterator-type-item"></span>`type Item = T`

- <span id="slicevecdrain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

##### `impl<T, U> TryFrom for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="slicevecdrain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<T, U> TryInto for SliceVecDrain<'p, 's, T>`

- <span id="slicevecdrain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="slicevecdrain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>>`

```rust
struct TinyVecSplice<'p, A: Array, I: Iterator<Item = <A as >::Item>> {
    parent: &'p mut TinyVec<A>,
    removal_start: usize,
    removal_end: usize,
    replacement: I,
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1215-1220`](../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L1215-L1220)*

Splicing iterator for `TinyVec`
See [`TinyVec::splice`](TinyVec::<A>::splice)

#### Trait Implementations

##### `impl Any for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A, I> DoubleEndedIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<A: Array, I: Iterator<Item = <A as >::Item>> Drop for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-drop"></span>`fn drop(&mut self)`

##### `impl<A, I> ExactSizeIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A, I> FusedIterator for TinyVecSplice<'p, A, I>`

##### `impl<U> Into for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<I> IntoIterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecsplice-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecsplice-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A, I> Iterator for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecsplice-iterator-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="tinyvecsplice-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyvecsplice-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVecSplice<'p, A, I>`

- <span id="tinyvecsplice-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyvecsplice-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `TinyVec<A: Array>`

```rust
enum TinyVec<A: Array> {
    Inline(ArrayVec<A>),
    Heap(alloc::vec::Vec<<A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:96-101`](../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L96-L101)*

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

  Returns whether elements are on heap

- <span id="tinyvec-is-inline"></span>`fn is_inline(&self) -> bool`

  Returns whether elements are on stack

- <span id="tinyvec-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

  Shrinks the capacity of the vector as much as possible.\

  It is inlined if length is less than `A::CAPACITY`.

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 2] => 1, 2, 3);

  assert!(tv.is_heap());

  let _ = tv.pop();

  assert!(tv.is_heap());

  tv.shrink_to_fit();

  assert!(tv.is_inline());

  ```

- <span id="tinyvec-move-to-the-heap"></span>`fn move_to_the_heap(&mut self)`

  Moves the content of the TinyVec to the heap, if it's inline.

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3);

  assert!(tv.is_inline());

  tv.move_to_the_heap();

  assert!(tv.is_heap());

  ```

- <span id="tinyvec-move-to-the-heap-and-reserve"></span>`fn move_to_the_heap_and_reserve(&mut self, n: usize)`

  If TinyVec is inline, moves the content of it to the heap.

  Also reserves additional space.

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3);

  assert!(tv.is_inline());

  tv.move_to_the_heap_and_reserve(32);

  assert!(tv.is_heap());

  assert!(tv.capacity() >= 35);

  ```

- <span id="tinyvec-reserve"></span>`fn reserve(&mut self, n: usize)`

  Reserves additional space.

  Moves to the heap if array can't hold `n` more items

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3, 4);

  assert!(tv.is_inline());

  tv.reserve(1);

  assert!(tv.is_heap());

  assert!(tv.capacity() >= 5);

  ```

- <span id="tinyvec-reserve-exact"></span>`fn reserve_exact(&mut self, n: usize)`

  Reserves additional space.

  Moves to the heap if array can't hold `n` more items

  

  From [Vec::reserve_exact](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact)

  ```text

  Note that the allocator may give the collection more space than it requests.

  Therefore, capacity can not be relied upon to be precisely minimal.

  Prefer `reserve` if future insertions are expected.

  ```

  ```rust

  use tinyvec::*;

  let mut tv = tiny_vec!([i32; 4] => 1, 2, 3, 4);

  assert!(tv.is_inline());

  tv.reserve_exact(1);

  assert!(tv.is_heap());

  assert!(tv.capacity() >= 5);

  ```

- <span id="tinyvec-with-capacity"></span>`fn with_capacity(cap: usize) -> Self`

  Makes a new TinyVec with _at least_ the given capacity.

  

  If the requested capacity is less than or equal to the array capacity you

  get an inline vec. If it's greater than you get a heap vec.

  ```rust

  use tinyvec::*;

  let t = TinyVec::<[u8; 10]>::with_capacity(5);

  assert!(t.is_inline());

  assert!(t.capacity() >= 5);

  

  let t = TinyVec::<[u8; 10]>::with_capacity(20);

  assert!(t.is_heap());

  assert!(t.capacity() >= 20);

  ```

- <span id="tinyvec-into-boxed-slice"></span>`fn into_boxed_slice(self) -> alloc::boxed::Box<[<A as >::Item]>` — [`Array`](#array)

  Converts a `TinyVec<[T; N]>` into a `Box<[T]>`.

  

  - For `TinyVec::Heap(Vec<T>)`, it takes the `Vec<T>` and converts it into

    a `Box<[T]>` without heap reallocation.

  - For `TinyVec::Inline(inner_data)`, it first converts the `inner_data` to

    `Vec<T>`, then into a `Box<[T]>`. Requiring only a single heap

    allocation.

  

  ## Example

  

  ```rust

  use core::mem::size_of_val as mem_size_of;

  use tinyvec::TinyVec;

  

  // Initialize TinyVec with 256 elements (exceeding inline capacity)

  let v: TinyVec<[_; 128]> = (0u8..=255).collect();

  

  assert!(v.is_heap());

  assert_eq!(mem_size_of(&v), 136); // mem size of TinyVec<[u8; N]>: N+8

  assert_eq!(v.len(), 256);

  

  let boxed = v.into_boxed_slice();

  assert_eq!(mem_size_of(&boxed), 16); // mem size of Box<[u8]>: 16 bytes (fat pointer)

  assert_eq!(boxed.len(), 256);

  ```

- <span id="tinyvec-into-vec"></span>`fn into_vec(self) -> Vec<<A as >::Item>` — [`Array`](#array)

  Converts a `TinyVec<[T; N]>` into a `Vec<T>`.

  

  `v.into_vec()` is equivalent to `Into::<Vec<_>>::into(v)`.

  

  - For `TinyVec::Inline(_)`, `.into_vec()` **does not** offer a performance

    advantage over `.to_vec()`.

  - For `TinyVec::Heap(vec_data)`, `.into_vec()` will take `vec_data`

    without heap reallocation.

  

  ## Example

  

  ```rust

  use tinyvec::TinyVec;

  

  let v = TinyVec::from([0u8; 8]);

  let v2 = v.clone();

  

  let vec = v.into_vec();

  let vec2: Vec<_> = v2.into();

  

  assert_eq!(vec, vec2);

  ```

#### Trait Implementations

##### `impl Any for TinyVec<A>`

- <span id="tinyvec-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<A: Array> AsMut for TinyVec<A>`

- <span id="tinyvec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for TinyVec<A>`

- <span id="tinyvec-asref-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Binary for TinyVec<A>`

- <span id="tinyvec-binary-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<T> Borrow for TinyVec<A>`

- <span id="tinyvec-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVec<A>`

- <span id="tinyvec-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A> Clone for TinyVec<A>`

- <span id="tinyvec-clone"></span>`fn clone(&self) -> Self`

- <span id="tinyvec-clone-clone-from"></span>`fn clone_from(&mut self, o: &Self)`

##### `impl CloneToUninit for TinyVec<A>`

- <span id="tinyvec-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<A: Array> Debug for TinyVec<A>`

- <span id="tinyvec-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Default for TinyVec<A>`

- <span id="tinyvec-default"></span>`fn default() -> Self`

##### `impl<A: Array> Deref for TinyVec<A>`

- <span id="tinyvec-deref-type-target"></span>`type Target = [<A as Array>::Item]`

- <span id="tinyvec-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<A: Array> DerefMut for TinyVec<A>`

- <span id="tinyvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<A: Array> Display for TinyVec<A>`

- <span id="tinyvec-display-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Eq for TinyVec<A>`

##### `impl<A: Array> Extend for TinyVec<A>`

- <span id="tinyvec-extend"></span>`fn extend<T: IntoIterator<Item = <A as >::Item>>(&mut self, iter: T)`

##### `impl<T> From for TinyVec<A>`

- <span id="tinyvec-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: Array> FromIterator for TinyVec<A>`

- <span id="tinyvec-fromiterator-from-iter"></span>`fn from_iter<T: IntoIterator<Item = <A as >::Item>>(iter: T) -> Self`

##### `impl<A: Array> Hash for TinyVec<A>`

- <span id="tinyvec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for TinyVec<A>`

- <span id="tinyvec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="tinyvec-index"></span>`fn index(&self, index: I) -> &<Self as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for TinyVec<A>`

- <span id="tinyvec-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <Self as >::Output`

##### `impl<U> Into for TinyVec<A>`

- <span id="tinyvec-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<A: Array> IntoIterator for TinyVec<A>`

- <span id="tinyvec-intoiterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvec-intoiterator-type-intoiter"></span>`type IntoIter = TinyVecIterator<A>`

- <span id="tinyvec-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<A: Array> LowerExp for TinyVec<A>`

- <span id="tinyvec-lowerexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> LowerHex for TinyVec<A>`

- <span id="tinyvec-lowerhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Octal for TinyVec<A>`

- <span id="tinyvec-octal-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> Ord for TinyVec<A>`

- <span id="tinyvec-ord-cmp"></span>`fn cmp(&self, other: &Self) -> core::cmp::Ordering`

##### `impl<A: Array> PartialEq for TinyVec<A>`

- <span id="tinyvec-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

##### `impl<A: Array> PartialOrd for TinyVec<A>`

- <span id="tinyvec-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering>`

##### `impl<A: Array> Pointer for TinyVec<A>`

- <span id="tinyvec-pointer-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl Receiver for TinyVec<A>`

- <span id="tinyvec-receiver-type-target"></span>`type Target = T`

##### `impl ToOwned for TinyVec<A>`

- <span id="tinyvec-toowned-type-owned"></span>`type Owned = T`

- <span id="tinyvec-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="tinyvec-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for TinyVec<A>`

- <span id="tinyvec-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for TinyVec<A>`

- <span id="tinyvec-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyvec-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVec<A>`

- <span id="tinyvec-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyvec-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<A: Array> UpperExp for TinyVec<A>`

- <span id="tinyvec-upperexp-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> UpperHex for TinyVec<A>`

- <span id="tinyvec-upperhex-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

### `TinyVecDrain<'p, A: Array>`

```rust
enum TinyVecDrain<'p, A: Array> {
    Inline(ArrayVecDrain<'p, <A as >::Item>),
    Heap(vec::Drain<'p, <A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1166-1171`](../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L1166-L1171)*

Draining iterator for `TinyVecDrain`

See [`TinyVecDrain::drain`](TinyVecDrain::<A>::drain)

#### Trait Implementations

##### `impl Any for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: Array> DoubleEndedIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<T> From for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyvecdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyvecdrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyvecdrain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyvecdrain-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyvecdrain-iterator-count"></span>`fn count(self) -> usize`

- <span id="tinyvecdrain-iterator-for-each"></span>`fn for_each<F: FnMut(<Self as >::Item)>(self, f: F)`

##### `impl<U> TryFrom for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyvecdrain-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVecDrain<'p, A>`

- <span id="tinyvecdrain-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyvecdrain-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TinyVecIterator<A: Array>`

```rust
enum TinyVecIterator<A: Array> {
    Inline(ArrayVecIterator<A>),
    Heap(alloc::vec::IntoIter<<A as >::Item>),
}
```

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:1483-1488`](../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L1483-L1488)*

Iterator for consuming an `TinyVec` and returning owned elements.

#### Implementations

- <span id="tinyveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

  Returns the remaining items of this iterator as a slice.

#### Trait Implementations

##### `impl Any for TinyVecIterator<A>`

- <span id="tinyveciterator-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TinyVecIterator<A>`

- <span id="tinyveciterator-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TinyVecIterator<A>`

- <span id="tinyveciterator-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<A: Array> Debug for TinyVecIterator<A>`

- <span id="tinyveciterator-debug-fmt"></span>`fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-doubleendediterator-nth-back"></span>`fn nth_back(&mut self, n: usize) -> Option<<Self as >::Item>`

##### `impl<A: Array> ExactSizeIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> From for TinyVecIterator<A>`

- <span id="tinyveciterator-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<A: Array> FusedIterator for TinyVecIterator<A>`

##### `impl<U> Into for TinyVecIterator<A>`

- <span id="tinyveciterator-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TinyVecIterator<A>`

- <span id="tinyveciterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tinyveciterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tinyveciterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for TinyVecIterator<A>`

- <span id="tinyveciterator-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="tinyveciterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="tinyveciterator-iterator-count"></span>`fn count(self) -> usize`

- <span id="tinyveciterator-iterator-last"></span>`fn last(self) -> Option<<Self as >::Item>`

- <span id="tinyveciterator-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<U> TryFrom for TinyVecIterator<A>`

- <span id="tinyveciterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="tinyveciterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TinyVecIterator<A>`

- <span id="tinyveciterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="tinyveciterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Array`

```rust
trait Array { ... }
```

*Defined in [`tinyvec-1.10.0/src/array.rs:18-41`](../../.source_1765521767/tinyvec-1.10.0/src/array.rs#L18-L41)*

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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:30-50`](../../.source_1765521767/tinyvec-1.10.0/src/arrayvec.rs#L30-L50)*

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

*Defined in [`tinyvec-1.10.0/src/tinyvec.rs:37-66`](../../.source_1765521767/tinyvec-1.10.0/src/tinyvec.rs#L37-L66)*

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

