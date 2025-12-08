*[memchr](../../../index.md) / [arch](../../index.md) / [generic](../index.md) / [memchr](index.md)*

---

# Module `memchr`

Generic crate-internal routines for the `memchr` family of functions.

## Structs

### `One<V>`

```rust
struct One<V> {
    s1: u8,
    v1: V,
}
```

Finds all occurrences of a single byte in a haystack.

#### Implementations

- `const LOOP_SIZE: usize`

- `unsafe fn new(needle: u8) -> One<V>` — [`One`](#one)

- `fn needle1(self: &Self) -> u8`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn count_raw(self: &Self, start: *const u8, end: *const u8) -> usize`

- `unsafe fn search_chunk(self: &Self, cur: *const u8, mask_to_offset: impl Fn(<V as >::Mask) -> usize) -> Option<*const u8>` — [`Vector`](../../../vector/index.md)

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for One<V>`

- `fn clone(self: &Self) -> One<V>` — [`One`](#one)

##### `impl<V: $crate::marker::Copy> Copy for One<V>`

##### `impl<V: $crate::fmt::Debug> Debug for One<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Two<V>`

```rust
struct Two<V> {
    s1: u8,
    s2: u8,
    v1: V,
    v2: V,
}
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- `const LOOP_SIZE: usize`

- `unsafe fn new(needle1: u8, needle2: u8) -> Two<V>` — [`Two`](#two)

- `fn needle1(self: &Self) -> u8`

- `fn needle2(self: &Self) -> u8`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn search_chunk(self: &Self, cur: *const u8, mask_to_offset: impl Fn(<V as >::Mask) -> usize) -> Option<*const u8>` — [`Vector`](../../../vector/index.md)

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for Two<V>`

- `fn clone(self: &Self) -> Two<V>` — [`Two`](#two)

##### `impl<V: $crate::marker::Copy> Copy for Two<V>`

##### `impl<V: $crate::fmt::Debug> Debug for Two<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Three<V>`

```rust
struct Three<V> {
    s1: u8,
    s2: u8,
    s3: u8,
    v1: V,
    v2: V,
    v3: V,
}
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- `const LOOP_SIZE: usize`

- `unsafe fn new(needle1: u8, needle2: u8, needle3: u8) -> Three<V>` — [`Three`](#three)

- `fn needle1(self: &Self) -> u8`

- `fn needle2(self: &Self) -> u8`

- `fn needle3(self: &Self) -> u8`

- `unsafe fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>`

- `unsafe fn search_chunk(self: &Self, cur: *const u8, mask_to_offset: impl Fn(<V as >::Mask) -> usize) -> Option<*const u8>` — [`Vector`](../../../vector/index.md)

#### Trait Implementations

##### `impl<V: $crate::clone::Clone> Clone for Three<V>`

- `fn clone(self: &Self) -> Three<V>` — [`Three`](#three)

##### `impl<V: $crate::marker::Copy> Copy for Three<V>`

##### `impl<V: $crate::fmt::Debug> Debug for Three<V>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Iter<'h>`

```rust
struct Iter<'h> {
    original_start: *const u8,
    start: *const u8,
    end: *const u8,
    haystack: core::marker::PhantomData<&'h [u8]>,
}
```

An iterator over all occurrences of a set of bytes in a haystack.

This iterator implements the routines necessary to provide a
`DoubleEndedIterator` impl, which means it can also be used to find
occurrences in reverse order.

The lifetime parameters are as follows:

* `'h` refers to the lifetime of the haystack being searched.

This type is intended to be used to implement all iterators for the
`memchr` family of functions. It handles a tiny bit of marginally tricky
raw pointer math, but otherwise expects the caller to provide `find_raw`
and `rfind_raw` routines for each call of `next` and `next_back`,
respectively.

#### Fields

- **`original_start`**: `*const u8`

  The original starting point into the haystack. We use this to convert
  pointers to offsets.

- **`start`**: `*const u8`

  The current starting point into the haystack. That is, where the next
  search will begin.

- **`end`**: `*const u8`

  The current ending point into the haystack. That is, where the next
  reverse search will begin.

- **`haystack`**: `core::marker::PhantomData<&'h [u8]>`

  A marker for tracking the lifetime of the start/cur_start/cur_end
  pointers above, which all point into the haystack.

#### Implementations

- `fn new(haystack: &'h [u8]) -> Iter<'h>` — [`Iter`](#iter)

- `unsafe fn next(self: &mut Self, find_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>) -> Option<usize>`

- `fn count(self: Self, count_raw: impl FnMut(*const u8, *const u8) -> usize) -> usize`

- `unsafe fn next_back(self: &mut Self, rfind_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

#### Trait Implementations

##### `impl<'h> Clone for Iter<'h>`

- `fn clone(self: &Self) -> Iter<'h>` — [`Iter`](#iter)

##### `impl<'h> Debug for Iter<'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'h> Send for Iter<'h>`

##### `impl<'h> Sync for Iter<'h>`

## Functions

### `search_slice_with_raw`

```rust
unsafe fn search_slice_with_raw(haystack: &[u8], find_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>) -> Option<usize>
```

Search a slice using a function that operates on raw pointers.

Given a function to search a contiguous sequence of memory for the location
of a non-empty set of bytes, this will execute that search on a slice of
bytes. The pointer returned by the given function will be converted to an
offset relative to the starting point of the given slice. That is, if a
match is found, the offset returned by this routine is guaranteed to be a
valid index into `haystack`.

Callers may use this for a forward or reverse search.

# Safety

Callers must ensure that if a pointer is returned by `find_raw`, then the
pointer must be greater than or equal to the starting pointer and less than
the end pointer.

### `fwd_byte_by_byte`

```rust
unsafe fn fwd_byte_by_byte<F: Fn(u8) -> bool>(start: *const u8, end: *const u8, confirm: F) -> Option<*const u8>
```

Performs a forward byte-at-a-time loop until either `ptr >= end_ptr` or
until `confirm(*ptr)` returns `true`. If the former occurs, then `None` is
returned. If the latter occurs, then the pointer at which `confirm` returns
`true` is returned.

# Safety

Callers must provide valid pointers and they must satisfy `start_ptr <=
ptr` and `ptr <= end_ptr`.

### `rev_byte_by_byte`

```rust
unsafe fn rev_byte_by_byte<F: Fn(u8) -> bool>(start: *const u8, end: *const u8, confirm: F) -> Option<*const u8>
```

Performs a reverse byte-at-a-time loop until either `ptr < start_ptr` or
until `confirm(*ptr)` returns `true`. If the former occurs, then `None` is
returned. If the latter occurs, then the pointer at which `confirm` returns
`true` is returned.

# Safety

Callers must provide valid pointers and they must satisfy `start_ptr <=
ptr` and `ptr <= end_ptr`.

### `count_byte_by_byte`

```rust
unsafe fn count_byte_by_byte<F: Fn(u8) -> bool>(start: *const u8, end: *const u8, confirm: F) -> usize
```

Performs a forward byte-at-a-time loop until `ptr >= end_ptr` and returns
the number of times `confirm(*ptr)` returns `true`.

# Safety

Callers must provide valid pointers and they must satisfy `start_ptr <=
ptr` and `ptr <= end_ptr`.

