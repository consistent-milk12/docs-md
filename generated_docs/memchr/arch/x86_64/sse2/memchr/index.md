*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [sse2](../index.md) / [memchr](index.md)*

---

# Module `memchr`

This module defines 128-bit vector implementations of `memchr` and friends.

The main types in this module are [`One`](#one), [`Two`](#two) and [`Three`](#three). They are for
searching for one, two or three distinct bytes, respectively, in a haystack.
Each type also has corresponding double ended iterators. These searchers are
typically much faster than scalar routines accomplishing the same task.

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

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`One`](#one) | struct | Finds all occurrences of a single byte in a haystack. |
| [`OneIter`](#oneiter) | struct | An iterator over all occurrences of a single byte in a haystack. |
| [`Two`](#two) | struct | Finds all occurrences of two bytes in a haystack. |
| [`TwoIter`](#twoiter) | struct | An iterator over all occurrences of two possible bytes in a haystack. |
| [`Three`](#three) | struct | Finds all occurrences of three bytes in a haystack. |
| [`ThreeIter`](#threeiter) | struct | An iterator over all occurrences of three possible bytes in a haystack. |

## Structs

### `One`

```rust
struct One(generic::One<core::arch::x86_64::__m128i>);
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs:29`](../../../../../../.source_1765521767/memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs#L29)*

Finds all occurrences of a single byte in a haystack.

#### Implementations

- <span id="one-new"></span>`fn new(needle: u8) -> Option<One>` — [`One`](#one)

  Create a new searcher that finds occurrences of the needle byte given.

  

  This particular searcher is specialized to use SSE2 vector instructions

  that typically make it quite fast.

  

  If SSE2 is unavailable in the current environment, then `None` is

  returned.

- <span id="one-new-unchecked"></span>`unsafe fn new_unchecked(needle: u8) -> One` — [`One`](#one)

  Create a new finder specific to SSE2 vectors and routines without

  checking that SSE2 is available.

  

  # Safety

  

  Callers must guarantee that it is safe to execute `sse2` instructions

  in the current environment.

  

  Note that it is a common misconception that if one compiles for an

  `x86_64` target, then they therefore automatically have access to SSE2

  instructions. While this is almost always the case, it isn't true in

  100% of cases.

- <span id="one-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `One::new` will return

  a `Some` value. Similarly, when it is false, it is guaranteed that

  `One::new` will return a `None` value.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="one-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="one-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

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

- <span id="one-find-raw-impl"></span>`unsafe fn find_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `One::find_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="one-rfind-raw-impl"></span>`unsafe fn rfind_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `One::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="one-count-raw-impl"></span>`unsafe fn count_raw_impl(&self, start: *const u8, end: *const u8) -> usize`

  Execute a count using SSE2 vectors and routines.

  

  # Safety

  

  Same as `One::count_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="one-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

  Returns an iterator over all occurrences of the needle byte in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

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

*Defined in [`memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs:356-359`](../../../../../../.source_1765521767/memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs#L356-L359)*

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `One::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`](#one) searcher.
* `'h` refers to the lifetime of the haystack being searched.

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

##### `impl FusedIterator for OneIter<'a, 'h>`

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
struct Two(generic::Two<core::arch::x86_64::__m128i>);
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs:405`](../../../../../../.source_1765521767/memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs#L405)*

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- <span id="two-new"></span>`fn new(needle1: u8, needle2: u8) -> Option<Two>` — [`Two`](#two)

  Create a new searcher that finds occurrences of the needle bytes given.

  

  This particular searcher is specialized to use SSE2 vector instructions

  that typically make it quite fast.

  

  If SSE2 is unavailable in the current environment, then `None` is

  returned.

- <span id="two-new-unchecked"></span>`unsafe fn new_unchecked(needle1: u8, needle2: u8) -> Two` — [`Two`](#two)

  Create a new finder specific to SSE2 vectors and routines without

  checking that SSE2 is available.

  

  # Safety

  

  Callers must guarantee that it is safe to execute `sse2` instructions

  in the current environment.

  

  Note that it is a common misconception that if one compiles for an

  `x86_64` target, then they therefore automatically have access to SSE2

  instructions. While this is almost always the case, it isn't true in

  100% of cases.

- <span id="two-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `Two::new` will return

  a `Some` value. Similarly, when it is false, it is guaranteed that

  `Two::new` will return a `None` value.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="two-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="two-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

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

- <span id="two-find-raw-impl"></span>`unsafe fn find_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Two::find_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Two`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="two-rfind-raw-impl"></span>`unsafe fn rfind_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Two::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Two`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="two-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

  Returns an iterator over all occurrences of the needle bytes in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

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

*Defined in [`memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs:662-665`](../../../../../../.source_1765521767/memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs#L662-L665)*

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Two::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`](#two) searcher.
* `'h` refers to the lifetime of the haystack being searched.

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

##### `impl FusedIterator for TwoIter<'a, 'h>`

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
struct Three(generic::Three<core::arch::x86_64::__m128i>);
```

*Defined in [`memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs:702`](../../../../../../.source_1765521767/memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs#L702)*

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

#### Implementations

- <span id="three-new"></span>`fn new(needle1: u8, needle2: u8, needle3: u8) -> Option<Three>` — [`Three`](#three)

  Create a new searcher that finds occurrences of the needle bytes given.

  

  This particular searcher is specialized to use SSE2 vector instructions

  that typically make it quite fast.

  

  If SSE2 is unavailable in the current environment, then `None` is

  returned.

- <span id="three-new-unchecked"></span>`unsafe fn new_unchecked(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](#three)

  Create a new finder specific to SSE2 vectors and routines without

  checking that SSE2 is available.

  

  # Safety

  

  Callers must guarantee that it is safe to execute `sse2` instructions

  in the current environment.

  

  Note that it is a common misconception that if one compiles for an

  `x86_64` target, then they therefore automatically have access to SSE2

  instructions. While this is almost always the case, it isn't true in

  100% of cases.

- <span id="three-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `Three::new` will return

  a `Some` value. Similarly, when it is false, it is guaranteed that

  `Three::new` will return a `None` value.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="three-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="three-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

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

- <span id="three-find-raw-impl"></span>`unsafe fn find_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Three::find_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Three`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="three-rfind-raw-impl"></span>`unsafe fn rfind_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Three::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Three`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="three-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

  Returns an iterator over all occurrences of the needle byte in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

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

*Defined in [`memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs:967-970`](../../../../../../.source_1765521767/memchr-2.7.6/src/arch/x86_64/sse2/memchr.rs#L967-L970)*

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Three::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`](#three) searcher.
* `'h` refers to the lifetime of the haystack being searched.

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

##### `impl FusedIterator for ThreeIter<'a, 'h>`

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

