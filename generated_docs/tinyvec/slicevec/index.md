*[tinyvec](../index.md) / [slicevec](index.md)*

---

# Module `slicevec`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SliceVec`](#slicevec) | struct | A slice-backed vector-like data structure. |
| [`SliceVecDrain`](#slicevecdrain) | struct | Draining iterator for [`SliceVec`] |

## Structs

### `SliceVec<'s, T>`

```rust
struct SliceVec<'s, T> {
    data: &'s mut [T],
    len: usize,
}
```

*Defined in [`tinyvec-1.10.0/src/slicevec.rs:16-19`](../../../.source_1765521767/tinyvec-1.10.0/src/slicevec.rs#L16-L19)*

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

- <span id="slicevec-drain"></span>`fn drain<'p, R: RangeBounds<usize>>(self: &'p mut Self, range: R) -> SliceVecDrain<'p, 's, T>` — [`SliceVecDrain`](../index.md#slicevecdrain)

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

- <span id="slicevec-split-off"></span>`fn split_off<'a>(self: &'a mut Self, at: usize) -> SliceVec<'s, T>` — [`SliceVec`](../index.md#slicevec)

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

*Defined in [`tinyvec-1.10.0/src/slicevec.rs:714-719`](../../../.source_1765521767/tinyvec-1.10.0/src/slicevec.rs#L714-L719)*

Draining iterator for [`SliceVec`](../index.md)

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

