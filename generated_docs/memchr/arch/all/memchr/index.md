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
  - [`has_zero_byte`](#has_zero_byte)
  - [`splat`](#splat)
- [Constants](#constants)
  - [`USIZE_BYTES`](#usize_bytes)
  - [`USIZE_ALIGN`](#usize_align)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`One`](#one) | struct | Finds all occurrences of a single byte in a haystack. |
| [`OneIter`](#oneiter) | struct | An iterator over all occurrences of a single byte in a haystack. |
| [`Two`](#two) | struct | Finds all occurrences of two bytes in a haystack. |
| [`TwoIter`](#twoiter) | struct | An iterator over all occurrences of two possible bytes in a haystack. |
| [`Three`](#three) | struct | Finds all occurrences of three bytes in a haystack. |
| [`ThreeIter`](#threeiter) | struct | An iterator over all occurrences of three possible bytes in a haystack. |
| [`has_zero_byte`](#has_zero_byte) | fn | Return `true` if `x` contains any zero byte. |
| [`splat`](#splat) | fn | Repeat the given byte into a word size number. |
| [`USIZE_BYTES`](#usize_bytes) | const | The number of bytes in a single `usize` value. |
| [`USIZE_ALIGN`](#usize_align) | const | The bits that must be zero for a `*const usize` to be properly aligned. |

## Structs

### `One`

```rust
struct One {
    s1: u8,
    v1: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:35-38`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L35-L38)*

Finds all occurrences of a single byte in a haystack.

#### Implementations

- <span id="one-const-loop-bytes"></span>`const LOOP_BYTES: usize`

- <span id="one-new"></span>`fn new(needle: u8) -> One` — [`One`](#one)

- <span id="one-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

- <span id="one-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

- <span id="one-count"></span>`fn count(&self, haystack: &[u8]) -> usize`

- <span id="one-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="one-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="one-count-raw"></span>`unsafe fn count_raw(&self, start: *const u8, end: *const u8) -> usize`

- <span id="one-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

- <span id="one-has-needle"></span>`fn has_needle(&self, chunk: usize) -> bool`

- <span id="one-confirm"></span>`fn confirm(&self, haystack_byte: u8) -> bool`

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

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:303-308`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L303-L308)*

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

##### `impl Clone for OneIter<'a, 'h>`

- <span id="oneiter-clone"></span>`fn clone(&self) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

##### `impl Debug for OneIter<'a, 'h>`

- <span id="oneiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for OneIter<'a, 'h>`

- <span id="oneiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl IntoIterator for OneIter<'a, 'h>`

- <span id="oneiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="oneiter-type-intoiter"></span>`type IntoIter = I`

- <span id="oneiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for OneIter<'a, 'h>`

- <span id="oneiter-type-item"></span>`type Item = usize`

- <span id="oneiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="oneiter-count"></span>`fn count(self) -> usize`

- <span id="oneiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Two`

```rust
struct Two {
    s1: u8,
    s2: u8,
    v1: usize,
    v2: usize,
}
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:352-357`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L352-L357)*

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- <span id="two-new"></span>`fn new(needle1: u8, needle2: u8) -> Two` — [`Two`](#two)

- <span id="two-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

- <span id="two-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

- <span id="two-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="two-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="two-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

- <span id="two-has-needle"></span>`fn has_needle(&self, chunk: usize) -> bool`

- <span id="two-confirm"></span>`fn confirm(&self, haystack_byte: u8) -> bool`

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

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:568-573`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L568-L573)*

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

##### `impl Clone for TwoIter<'a, 'h>`

- <span id="twoiter-clone"></span>`fn clone(&self) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

##### `impl Debug for TwoIter<'a, 'h>`

- <span id="twoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for TwoIter<'a, 'h>`

- <span id="twoiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl IntoIterator for TwoIter<'a, 'h>`

- <span id="twoiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="twoiter-type-intoiter"></span>`type IntoIter = I`

- <span id="twoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TwoIter<'a, 'h>`

- <span id="twoiter-type-item"></span>`type Item = usize`

- <span id="twoiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="twoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

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

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:608-615`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L608-L615)*

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

#### Implementations

- <span id="three-new"></span>`fn new(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](#three)

- <span id="three-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

- <span id="three-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

- <span id="three-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="three-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

- <span id="three-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

- <span id="three-has-needle"></span>`fn has_needle(&self, chunk: usize) -> bool`

- <span id="three-confirm"></span>`fn confirm(&self, haystack_byte: u8) -> bool`

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

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:836-841`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L836-L841)*

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

##### `impl Clone for ThreeIter<'a, 'h>`

- <span id="threeiter-clone"></span>`fn clone(&self) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

##### `impl Debug for ThreeIter<'a, 'h>`

- <span id="threeiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl IntoIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="threeiter-type-intoiter"></span>`type IntoIter = I`

- <span id="threeiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ThreeIter<'a, 'h>`

- <span id="threeiter-type-item"></span>`type Item = usize`

- <span id="threeiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="threeiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Functions

### `has_zero_byte`

```rust
fn has_zero_byte(x: usize) -> bool
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:877-885`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L877-L885)*

Return `true` if `x` contains any zero byte.

That is, this routine treats `x` as a register of 8-bit lanes and returns
true when any of those lanes is `0`.

From "Matters Computational" by J. Arndt.

### `splat`

```rust
const fn splat(b: u8) -> usize
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:892-895`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L892-L895)*

Repeat the given byte into a word size number. That is, every 8 bits
is equivalent to the given byte. For example, if `b` is `\x4E` or
`01001110` in binary, then the returned value on a 32-bit system would be:
`01001110_01001110_01001110_01001110`.

## Constants

### `USIZE_BYTES`
```rust
const USIZE_BYTES: usize = 8usize;
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:29`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L29)*

The number of bytes in a single `usize` value.

### `USIZE_ALIGN`
```rust
const USIZE_ALIGN: usize = 7usize;
```

*Defined in [`memchr-2.7.6/src/arch/all/memchr.rs:31`](../../../../../.source_1765210505/memchr-2.7.6/src/arch/all/memchr.rs#L31)*

The bits that must be zero for a `*const usize` to be properly aligned.

