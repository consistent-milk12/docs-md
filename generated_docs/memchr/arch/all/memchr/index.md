*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [memchr](index.md)*

---

# Module `memchr`

Provides architecture independent implementations of `memchr` and friends.

The main types in this module are [`One`](#one), [`Two`](#two) and [`Three`](#three). They are for
searching for one, two or three distinct bytes, respectively, in a haystack.
Each type also has corresponding double ended iterators. These searchers
are typically slower than hand-coded vector routines accomplishing the same
task, but are also typically faster than naive scalar code. These routines
effectively work by treating a `usize` as a vector of 8-bit lanes, and thus
achieves some level of data parallelism even without explicit vector support.

The `One` searcher also provides a `One::count` routine for efficiently
counting the number of times a single byte occurs in a haystack. This is
useful, for example, for counting the number of lines in a haystack. This
routine exists because it is usually faster, especially with a high match
count, than using `One::find` repeatedly. ([`OneIter`](#oneiter) specializes its
`Iterator::count` implementation to use this routine.)

Only one, two and three bytes are supported because three bytes is about
the point where one sees diminishing returns. Beyond this point and it's
probably (but not necessarily) better to just use a simple `[bool; 256]` array
or similar. However, it depends mightily on the specific work-load and the
expected match frequency.

## Contents

- [Structs](#structs)
  - [`One`](#one)
  - [`OneIter`](#oneiter)
  - [`Two`](#two)
  - [`TwoIter`](#twoiter)
  - [`Three`](#three)
  - [`ThreeIter`](#threeiter)
- [Functions](#functions)
  - [`has_zero_byte`](#has-zero-byte)
  - [`splat`](#splat)
- [Constants](#constants)
  - [`USIZE_BYTES`](#usize-bytes)
  - [`USIZE_ALIGN`](#usize-align)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`One`](#one) | struct | Finds all occurrences of a single byte in a haystack. |
| [`OneIter`](#oneiter) | struct | An iterator over all occurrences of a single byte in a haystack. |
| [`Two`](#two) | struct | Finds all occurrences of two bytes in a haystack. |
| [`TwoIter`](#twoiter) | struct | An iterator over all occurrences of two possible bytes in a haystack. |
| [`Three`](#three) | struct | Finds all occurrences of three bytes in a haystack. |
| [`ThreeIter`](#threeiter) | struct | An iterator over all occurrences of three possible bytes in a haystack. |
| [`has_zero_byte`](#has-zero-byte) | fn | Return `true` if `x` contains any zero byte. |
| [`splat`](#splat) | fn | Repeat the given byte into a word size number. |
| [`USIZE_BYTES`](#usize-bytes) | const | The number of bytes in a single `usize` value. |
| [`USIZE_ALIGN`](#usize-align) | const | The bits that must be zero for a `*const usize` to be properly aligned. |

## Structs

### `One`

```rust
struct One {
    s1: u8,
    v1: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:35-38`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L35-L38)*

Finds all occurrences of a single byte in a haystack.

#### Implementations

- <span id="one-const-loop-bytes"></span>`const LOOP_BYTES: usize`

- <span id="one-new"></span>`fn new(needle: u8) -> One` — [`One`](#one)

  Create a new searcher that finds occurrences of the byte given.

- <span id="one-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of the needle in the given haystack. If no

  such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value for a non-empty haystack is `haystack.len() - 1`.

- <span id="one-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of the needle in the given haystack. If no

  such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value for a non-empty haystack is `haystack.len() - 1`.

- <span id="one-count"></span>`fn count(&self, haystack: &[u8]) -> usize`

  Counts all occurrences of this byte in the given haystack.

- <span id="one-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="one-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="one-count-raw"></span>`unsafe fn count_raw(&self, start: *const u8, end: *const u8) -> usize`

  Counts all occurrences of this byte in the given haystack represented

  by raw pointers.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `0` will always be returned.

- <span id="one-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

  Returns an iterator over all occurrences of the needle byte in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

- <span id="one-has-needle"></span>`fn has_needle(&self, chunk: usize) -> bool`

- <span id="one-confirm"></span>`fn confirm(&self, haystack_byte: u8) -> bool`

#### Trait Implementations

##### `impl Any for One`

- <span id="one-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for One`

- <span id="one-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for One`

- <span id="one-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for One`

- <span id="one-clone"></span>`fn clone(&self) -> One` — [`One`](#one)

##### `impl CloneToUninit for One`

- <span id="one-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for One`

##### `impl Debug for One`

- <span id="one-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for One`

- <span id="one-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for One`

- <span id="one-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for One`

- <span id="one-toowned-type-owned"></span>`type Owned = T`

- <span id="one-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="one-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for One`

- <span id="one-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="one-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for One`

- <span id="one-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="one-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `OneIter<'a, 'h>`

```rust
struct OneIter<'a, 'h> {
    searcher: &'a One,
    it: generic::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:303-308`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L303-L308)*

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `One::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`](#one) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`searcher`**: `&'a One`

  The underlying memchr searcher.

- **`it`**: `generic::Iter<'h>`

  Generic iterator implementation.

#### Trait Implementations

##### `impl Any for OneIter<'a, 'h>`

- <span id="oneiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OneIter<'a, 'h>`

- <span id="oneiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OneIter<'a, 'h>`

- <span id="oneiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OneIter<'a, 'h>`

- <span id="oneiter-clone"></span>`fn clone(&self) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

##### `impl CloneToUninit for OneIter<'a, 'h>`

- <span id="oneiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OneIter<'a, 'h>`

- <span id="oneiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for OneIter<'a, 'h>`

- <span id="oneiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for OneIter<'a, 'h>`

- <span id="oneiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OneIter<'a, 'h>`

- <span id="oneiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for OneIter<'a, 'h>`

- <span id="oneiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="oneiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="oneiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for OneIter<'a, 'h>`

- <span id="oneiter-iterator-type-item"></span>`type Item = usize`

- <span id="oneiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="oneiter-iterator-count"></span>`fn count(self) -> usize`

- <span id="oneiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for OneIter<'a, 'h>`

- <span id="oneiter-toowned-type-owned"></span>`type Owned = T`

- <span id="oneiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="oneiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OneIter<'a, 'h>`

- <span id="oneiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="oneiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OneIter<'a, 'h>`

- <span id="oneiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="oneiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Two`

```rust
struct Two {
    s1: u8,
    s2: u8,
    v1: usize,
    v2: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:352-357`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L352-L357)*

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- <span id="two-new"></span>`fn new(needle1: u8, needle2: u8) -> Two` — [`Two`](#two)

  Create a new searcher that finds occurrences of the two needle bytes

  given.

- <span id="two-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value for a non-empty haystack is `haystack.len() - 1`.

- <span id="two-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value for a non-empty haystack is `haystack.len() - 1`.

- <span id="two-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="two-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="two-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

  Returns an iterator over all occurrences of one of the needle bytes in

  the given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

- <span id="two-has-needle"></span>`fn has_needle(&self, chunk: usize) -> bool`

- <span id="two-confirm"></span>`fn confirm(&self, haystack_byte: u8) -> bool`

#### Trait Implementations

##### `impl Any for Two`

- <span id="two-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Two`

- <span id="two-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Two`

- <span id="two-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Two`

- <span id="two-clone"></span>`fn clone(&self) -> Two` — [`Two`](#two)

##### `impl CloneToUninit for Two`

- <span id="two-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Two`

##### `impl Debug for Two`

- <span id="two-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Two`

- <span id="two-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Two`

- <span id="two-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Two`

- <span id="two-toowned-type-owned"></span>`type Owned = T`

- <span id="two-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="two-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Two`

- <span id="two-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="two-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Two`

- <span id="two-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="two-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `TwoIter<'a, 'h>`

```rust
struct TwoIter<'a, 'h> {
    searcher: &'a Two,
    it: generic::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:568-573`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L568-L573)*

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Two::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`](#two) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`searcher`**: `&'a Two`

  The underlying memchr searcher.

- **`it`**: `generic::Iter<'h>`

  Generic iterator implementation.

#### Trait Implementations

##### `impl Any for TwoIter<'a, 'h>`

- <span id="twoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for TwoIter<'a, 'h>`

- <span id="twoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for TwoIter<'a, 'h>`

- <span id="twoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for TwoIter<'a, 'h>`

- <span id="twoiter-clone"></span>`fn clone(&self) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

##### `impl CloneToUninit for TwoIter<'a, 'h>`

- <span id="twoiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for TwoIter<'a, 'h>`

- <span id="twoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for TwoIter<'a, 'h>`

- <span id="twoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for TwoIter<'a, 'h>`

- <span id="twoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for TwoIter<'a, 'h>`

- <span id="twoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for TwoIter<'a, 'h>`

- <span id="twoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="twoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="twoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TwoIter<'a, 'h>`

- <span id="twoiter-iterator-type-item"></span>`type Item = usize`

- <span id="twoiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="twoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for TwoIter<'a, 'h>`

- <span id="twoiter-toowned-type-owned"></span>`type Owned = T`

- <span id="twoiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="twoiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for TwoIter<'a, 'h>`

- <span id="twoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="twoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for TwoIter<'a, 'h>`

- <span id="twoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="twoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Three`

```rust
struct Three {
    s1: u8,
    s2: u8,
    s3: u8,
    v1: usize,
    v2: usize,
    v3: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:608-615`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L608-L615)*

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

#### Implementations

- <span id="three-new"></span>`fn new(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](#three)

  Create a new searcher that finds occurrences of the three needle bytes

  given.

- <span id="three-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value for a non-empty haystack is `haystack.len() - 1`.

- <span id="three-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value for a non-empty haystack is `haystack.len() - 1`.

- <span id="three-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="three-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="three-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

  Returns an iterator over all occurrences of one of the needle bytes in

  the given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

- <span id="three-has-needle"></span>`fn has_needle(&self, chunk: usize) -> bool`

- <span id="three-confirm"></span>`fn confirm(&self, haystack_byte: u8) -> bool`

#### Trait Implementations

##### `impl Any for Three`

- <span id="three-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Three`

- <span id="three-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Three`

- <span id="three-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Three`

- <span id="three-clone"></span>`fn clone(&self) -> Three` — [`Three`](#three)

##### `impl CloneToUninit for Three`

- <span id="three-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Three`

##### `impl Debug for Three`

- <span id="three-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Three`

- <span id="three-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Three`

- <span id="three-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Three`

- <span id="three-toowned-type-owned"></span>`type Owned = T`

- <span id="three-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="three-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Three`

- <span id="three-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="three-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Three`

- <span id="three-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="three-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ThreeIter<'a, 'h>`

```rust
struct ThreeIter<'a, 'h> {
    searcher: &'a Three,
    it: generic::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:836-841`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L836-L841)*

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Three::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`](#three) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`searcher`**: `&'a Three`

  The underlying memchr searcher.

- **`it`**: `generic::Iter<'h>`

  Generic iterator implementation.

#### Trait Implementations

##### `impl Any for ThreeIter<'a, 'h>`

- <span id="threeiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ThreeIter<'a, 'h>`

- <span id="threeiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ThreeIter<'a, 'h>`

- <span id="threeiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ThreeIter<'a, 'h>`

- <span id="threeiter-clone"></span>`fn clone(&self) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

##### `impl CloneToUninit for ThreeIter<'a, 'h>`

- <span id="threeiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ThreeIter<'a, 'h>`

- <span id="threeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for ThreeIter<'a, 'h>`

- <span id="threeiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ThreeIter<'a, 'h>`

- <span id="threeiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="threeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="threeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ThreeIter<'a, 'h>`

- <span id="threeiter-iterator-type-item"></span>`type Item = usize`

- <span id="threeiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="threeiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for ThreeIter<'a, 'h>`

- <span id="threeiter-toowned-type-owned"></span>`type Owned = T`

- <span id="threeiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="threeiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ThreeIter<'a, 'h>`

- <span id="threeiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="threeiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ThreeIter<'a, 'h>`

- <span id="threeiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="threeiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `has_zero_byte`

```rust
fn has_zero_byte(x: usize) -> bool
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:877-885`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L877-L885)*

Return `true` if `x` contains any zero byte.

That is, this routine treats `x` as a register of 8-bit lanes and returns
true when any of those lanes is `0`.

From "Matters Computational" by J. Arndt.

### `splat`

```rust
const fn splat(b: u8) -> usize
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:892-895`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L892-L895)*

Repeat the given byte into a word size number. That is, every 8 bits
is equivalent to the given byte. For example, if `b` is `\x4E` or
`01001110` in binary, then the returned value on a 32-bit system would be:
`01001110_01001110_01001110_01001110`.

## Constants

### `USIZE_BYTES`
```rust
const USIZE_BYTES: usize = 8usize;
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:29`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L29)*

The number of bytes in a single `usize` value.

### `USIZE_ALIGN`
```rust
const USIZE_ALIGN: usize = 7usize;
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:31`](../../../../../.source_1765633015/memchr-2.7.6/src/arch/all/memchr.rs#L31)*

The bits that must be zero for a `*const usize` to be properly aligned.

