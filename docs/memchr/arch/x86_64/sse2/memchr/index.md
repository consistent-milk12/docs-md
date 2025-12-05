*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [sse2](../index.md) / [memchr](index.md)*

---

# Module `memchr`

This module defines 128-bit vector implementations of `memchr` and friends.

The main types in this module are [`One`](../../../generic/memchr/index.md), [`Two`](../../../all/memchr/index.md) and [`Three`](../../../generic/memchr/index.md). They are for
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

## Structs

### `One`

```rust
struct One(generic::One<core::arch::x86_64::__m128i>);
```

Finds all occurrences of a single byte in a haystack.

#### Implementations

- `fn new(needle: u8) -> Option<One>` — [`One`](../../../../../arch/x86_64/sse2/memchr/index.md)

- `unsafe fn new_unchecked(needle: u8) -> One` — [`One`](../../../../../arch/x86_64/sse2/memchr/index.md)

- `fn is_available() -> bool`

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn count(self: &Self, haystack: &[u8]) -> usize`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn count_raw(self: &Self, start: *const u8, end: *const u8) -> usize`

- `unsafe fn find_raw_impl(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw_impl(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn count_raw_impl(self: &Self, start: *const u8, end: *const u8) -> usize`

- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](../../../../../arch/x86_64/sse2/memchr/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> One` — [`One`](../../../../../arch/x86_64/sse2/memchr/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OneIter<'a, 'h>`

```rust
struct OneIter<'a, 'h> {
    searcher: &'a One,
    it: generic::Iter<'h>,
}
```

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `One::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`](../../../generic/memchr/index.md) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Clone<'a, 'h>`

- `fn clone(self: &Self) -> OneIter<'a, 'h>` — [`OneIter`](../../../../../arch/x86_64/sse2/memchr/index.md)

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a, 'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl FusedIterator<'a, 'h>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, 'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn count(self: Self) -> usize`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Two`

```rust
struct Two(generic::Two<core::arch::x86_64::__m128i>);
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- `fn new(needle1: u8, needle2: u8) -> Option<Two>` — [`Two`](../../../../../arch/x86_64/sse2/memchr/index.md)

- `unsafe fn new_unchecked(needle1: u8, needle2: u8) -> Two` — [`Two`](../../../../../arch/x86_64/sse2/memchr/index.md)

- `fn is_available() -> bool`

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn find_raw_impl(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw_impl(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](../../../../../arch/x86_64/sse2/memchr/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Two` — [`Two`](../../../../../arch/x86_64/sse2/memchr/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `TwoIter<'a, 'h>`

```rust
struct TwoIter<'a, 'h> {
    searcher: &'a Two,
    it: generic::Iter<'h>,
}
```

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Two::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`](../../../all/memchr/index.md) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Clone<'a, 'h>`

- `fn clone(self: &Self) -> TwoIter<'a, 'h>` — [`TwoIter`](../../../../../arch/x86_64/sse2/memchr/index.md)

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a, 'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl FusedIterator<'a, 'h>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, 'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Three`

```rust
struct Three(generic::Three<core::arch::x86_64::__m128i>);
```

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

#### Implementations

- `fn new(needle1: u8, needle2: u8, needle3: u8) -> Option<Three>` — [`Three`](../../../../../arch/x86_64/sse2/memchr/index.md)

- `unsafe fn new_unchecked(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](../../../../../arch/x86_64/sse2/memchr/index.md)

- `fn is_available() -> bool`

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn find_raw_impl(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw_impl(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](../../../../../arch/x86_64/sse2/memchr/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Three` — [`Three`](../../../../../arch/x86_64/sse2/memchr/index.md)

##### `impl Copy`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `ThreeIter<'a, 'h>`

```rust
struct ThreeIter<'a, 'h> {
    searcher: &'a Three,
    it: generic::Iter<'h>,
}
```

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Three::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`](../../../generic/memchr/index.md) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Clone<'a, 'h>`

- `fn clone(self: &Self) -> ThreeIter<'a, 'h>` — [`ThreeIter`](../../../../../arch/x86_64/sse2/memchr/index.md)

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a, 'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl FusedIterator<'a, 'h>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, 'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

