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

- <span id="one-new-unchecked"></span>`unsafe fn new_unchecked(needle: u8) -> One` — [`One`](#one)

- <span id="one-is-available"></span>`fn is_available() -> bool`

- <span id="one-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

- <span id="one-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

- <span id="one-count"></span>`fn count(&self, haystack: &[u8]) -> usize`

- <span id="one-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="one-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="one-count-raw"></span>`unsafe fn count_raw(&self, start: *const u8, end: *const u8) -> usize`

- <span id="one-find-raw-impl"></span>`unsafe fn find_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="one-rfind-raw-impl"></span>`unsafe fn rfind_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="one-count-raw-impl"></span>`unsafe fn count_raw_impl(&self, start: *const u8, end: *const u8) -> usize`

- <span id="one-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

#### Trait Implementations

##### `impl Clone for One`

- <span id="one-clone"></span>`fn clone(&self) -> One` — [`One`](#one)

##### `impl Copy for One`

##### `impl Debug for One`

- <span id="one-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl Clone for OneIter<'a, 'h>`

- <span id="oneiter-clone"></span>`fn clone(&self) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

##### `impl Debug for OneIter<'a, 'h>`

- <span id="oneiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for OneIter<'a, 'h>`

- <span id="oneiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for OneIter<'a, 'h>`

##### `impl IntoIterator for OneIter<'a, 'h>`

- <span id="oneiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="oneiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="oneiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for OneIter<'a, 'h>`

- <span id="oneiter-iterator-type-item"></span>`type Item = usize`

- <span id="oneiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="oneiter-count"></span>`fn count(self) -> usize`

- <span id="oneiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="two-new-unchecked"></span>`unsafe fn new_unchecked(needle1: u8, needle2: u8) -> Two` — [`Two`](#two)

- <span id="two-is-available"></span>`fn is_available() -> bool`

- <span id="two-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

- <span id="two-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

- <span id="two-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="two-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="two-find-raw-impl"></span>`unsafe fn find_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="two-rfind-raw-impl"></span>`unsafe fn rfind_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="two-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

#### Trait Implementations

##### `impl Clone for Two`

- <span id="two-clone"></span>`fn clone(&self) -> Two` — [`Two`](#two)

##### `impl Copy for Two`

##### `impl Debug for Two`

- <span id="two-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl Clone for TwoIter<'a, 'h>`

- <span id="twoiter-clone"></span>`fn clone(&self) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

##### `impl Debug for TwoIter<'a, 'h>`

- <span id="twoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for TwoIter<'a, 'h>`

- <span id="twoiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for TwoIter<'a, 'h>`

##### `impl IntoIterator for TwoIter<'a, 'h>`

- <span id="twoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="twoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="twoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TwoIter<'a, 'h>`

- <span id="twoiter-iterator-type-item"></span>`type Item = usize`

- <span id="twoiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="twoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

- <span id="three-new-unchecked"></span>`unsafe fn new_unchecked(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](#three)

- <span id="three-is-available"></span>`fn is_available() -> bool`

- <span id="three-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

- <span id="three-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

- <span id="three-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="three-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="three-find-raw-impl"></span>`unsafe fn find_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="three-rfind-raw-impl"></span>`unsafe fn rfind_raw_impl(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="three-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

#### Trait Implementations

##### `impl Clone for Three`

- <span id="three-clone"></span>`fn clone(&self) -> Three` — [`Three`](#three)

##### `impl Copy for Three`

##### `impl Debug for Three`

- <span id="three-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

##### `impl Clone for ThreeIter<'a, 'h>`

- <span id="threeiter-clone"></span>`fn clone(&self) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

##### `impl Debug for ThreeIter<'a, 'h>`

- <span id="threeiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for ThreeIter<'a, 'h>`

##### `impl IntoIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="threeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="threeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ThreeIter<'a, 'h>`

- <span id="threeiter-iterator-type-item"></span>`type Item = usize`

- <span id="threeiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="threeiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

