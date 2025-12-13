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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:106-109`](../../../.source_1765633015/tinyvec-1.10.0/src/arrayvec.rs#L106-L109)*

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

- <span id="arrayvec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut <A as >::Item` — [`Array`](../index.md#array)

  A `*mut` pointer to the backing array.

  

  ## Safety

  

  This pointer has provenance over the _entire_ backing array.

- <span id="arrayvec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md#array)

  Performs a `deref_mut`, into unique slice form.

- <span id="arrayvec-as-ptr"></span>`fn as_ptr(&self) -> *const <A as >::Item` — [`Array`](../index.md#array)

  A `*const` pointer to the backing array.

  

  ## Safety

  

  This pointer has provenance over the _entire_ backing array.

- <span id="arrayvec-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

  Performs a `deref`, into shared slice form.

- <span id="arrayvec-capacity"></span>`fn capacity(&self) -> usize`

  The capacity of the `ArrayVec`.

  

  This is fixed based on the array type, but can't yet be made a `const fn`

  on Stable Rust.

- <span id="arrayvec-clear"></span>`fn clear(&mut self)`

  Truncates the `ArrayVec` down to length 0.

- <span id="arrayvec-drain"></span>`fn drain<R>(&mut self, range: R) -> ArrayVecDrain<'_, <A as >::Item>` — [`ArrayVecDrain`](../index.md#arrayvecdrain), [`Array`](../index.md#array)

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

- <span id="arrayvec-extend-from-slice"></span>`fn extend_from_slice(&mut self, sli: &[<A as >::Item])` — [`Array`](../index.md#array)

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

- <span id="arrayvec-insert"></span>`fn insert(&mut self, index: usize, item: <A as >::Item)` — [`Array`](../index.md#array)

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

- <span id="arrayvec-try-insert"></span>`fn try_insert(&mut self, index: usize, item: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

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

- <span id="arrayvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

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

- <span id="arrayvec-push"></span>`fn push(&mut self, val: <A as >::Item)` — [`Array`](../index.md#array)

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

- <span id="arrayvec-try-push"></span>`fn try_push(&mut self, val: <A as >::Item) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

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

- <span id="arrayvec-remove"></span>`fn remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](../index.md#array)

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

- <span id="arrayvec-resize"></span>`fn resize(&mut self, new_len: usize, new_val: <A as >::Item)` — [`Array`](../index.md#array)

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

- <span id="arrayvec-splice"></span>`fn splice<R, I>(&mut self, range: R, replacement: I) -> ArrayVecSplice<'_, A, core::iter::Fuse<<I as >::IntoIter>>` — [`ArrayVecSplice`](../index.md#arrayvecsplice)

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

- <span id="arrayvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](../index.md#array)

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

- <span id="arrayvec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](../index.md#array)

##### `impl<A: Array> AsRef for ArrayVec<A>`

- <span id="arrayvec-asref-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1264-1269`](../../../.source_1765633015/tinyvec-1.10.0/src/arrayvec.rs#L1264-L1269)*

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

- <span id="arrayvecsplice-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

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

- <span id="arrayvecsplice-iterator-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1425`](../../../.source_1765633015/tinyvec-1.10.0/src/arrayvec.rs#L1425)*

The error type returned when a conversion from a slice to an [`ArrayVec`](../index.md)
fails.

#### Trait Implementations

##### `impl Any for TryFromSliceError`

- <span id="tryfromsliceerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TryFromSliceError`

- <span id="tryfromsliceerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TryFromSliceError`

- <span id="tryfromsliceerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TryFromSliceError`

- <span id="tryfromsliceerror-clone"></span>`fn clone(&self) -> TryFromSliceError` — [`TryFromSliceError`](../index.md#tryfromsliceerror)

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

*Defined in [`tinyvec-1.10.0/src/arrayvec.rs:1478-1482`](../../../.source_1765633015/tinyvec-1.10.0/src/arrayvec.rs#L1478-L1482)*

Iterator for consuming an `ArrayVec` and returning owned elements.

#### Implementations

- <span id="arrayveciterator-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](../index.md#array)

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

- <span id="arrayveciterator-iterator-nth"></span>`fn nth(&mut self, n: usize) -> Option<<A as >::Item>` — [`Array`](../index.md#array)

##### `impl<U> TryFrom for ArrayVecIterator<A>`

- <span id="arrayveciterator-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="arrayveciterator-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ArrayVecIterator<A>`

- <span id="arrayveciterator-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="arrayveciterator-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

