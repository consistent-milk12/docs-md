*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [memchr](index.md)*

---

# Module `memchr`

Provides architecture independent implementations of `memchr` and friends.

The main types in this module are [`One`](../../generic/memchr/index.md), [`Two`](#two) and [`Three`](../../generic/memchr/index.md). They are for
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
count, than using `One::find` repeatedly. ([`OneIter`](../../x86_64/sse2/memchr/index.md) specializes its
`Iterator::count` implementation to use this routine.)

Only one, two and three bytes are supported because three bytes is about
the point where one sees diminishing returns. Beyond this point and it's
probably (but not necessarily) better to just use a simple `[bool; 256]` array
or similar. However, it depends mightily on the specific work-load and the
expected match frequency.

## Structs

### `One`

```rust
struct One {
    s1: u8,
    v1: usize,
}
```

Finds all occurrences of a single byte in a haystack.

#### Implementations

- `const LOOP_BYTES: usize`

- `fn new(needle: u8) -> One` — [`One`](../../../../arch/all/memchr/index.md)

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn count(self: &Self, haystack: &[u8]) -> usize`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn count_raw(self: &Self, start: *const u8, end: *const u8) -> usize`

- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](../../../../arch/all/memchr/index.md)

- `fn has_needle(self: &Self, chunk: usize) -> bool`

- `fn confirm(self: &Self, haystack_byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> One` — [`One`](../../../../arch/all/memchr/index.md)

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

* `'a` refers to the lifetime of the underlying [`One`](../../generic/memchr/index.md) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`searcher`**: `&'a One`

  The underlying memchr searcher.

- **`it`**: `generic::Iter<'h>`

  Generic iterator implementation.

#### Trait Implementations

##### `impl Clone<'a, 'h>`

- `fn clone(self: &Self) -> OneIter<'a, 'h>` — [`OneIter`](../../../../arch/all/memchr/index.md)

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a, 'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

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
struct Two {
    s1: u8,
    s2: u8,
    v1: usize,
    v2: usize,
}
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- `fn new(needle1: u8, needle2: u8) -> Two` — [`Two`](../../../../arch/all/memchr/index.md)

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](../../../../arch/all/memchr/index.md)

- `fn has_needle(self: &Self, chunk: usize) -> bool`

- `fn confirm(self: &Self, haystack_byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Two` — [`Two`](../../../../arch/all/memchr/index.md)

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

* `'a` refers to the lifetime of the underlying [`Two`](#two) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`searcher`**: `&'a Two`

  The underlying memchr searcher.

- **`it`**: `generic::Iter<'h>`

  Generic iterator implementation.

#### Trait Implementations

##### `impl Clone<'a, 'h>`

- `fn clone(self: &Self) -> TwoIter<'a, 'h>` — [`TwoIter`](../../../../arch/all/memchr/index.md)

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a, 'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

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
struct Three {
    s1: u8,
    s2: u8,
    s3: u8,
    v1: usize,
    v2: usize,
    v3: usize,
}
```

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

#### Implementations

- `fn new(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](../../../../arch/all/memchr/index.md)

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](../../../../arch/all/memchr/index.md)

- `fn has_needle(self: &Self, chunk: usize) -> bool`

- `fn confirm(self: &Self, haystack_byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Three` — [`Three`](../../../../arch/all/memchr/index.md)

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

* `'a` refers to the lifetime of the underlying [`Three`](../../generic/memchr/index.md) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`searcher`**: `&'a Three`

  The underlying memchr searcher.

- **`it`**: `generic::Iter<'h>`

  Generic iterator implementation.

#### Trait Implementations

##### `impl Clone<'a, 'h>`

- `fn clone(self: &Self) -> ThreeIter<'a, 'h>` — [`ThreeIter`](../../../../arch/all/memchr/index.md)

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator<'a, 'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator<'a, 'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

