_[memchr](../index.md) / [memchr](index.md)_

---

# Module `memchr`

## Structs

### `Memchr<'h>`

```rust
struct Memchr<'h> {
    needle1: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr_iter`](../index.md) or `[memrchr_iter`]
functions. It can also be created with the `Memchr::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- `fn new(needle1: u8, haystack: &'h [u8]) -> Memchr<'h>` — [`Memchr`](../index.md)

#### Trait Implementations

##### `impl<'h> Clone for Memchr<'h>`

- `fn clone(self: &Self) -> Memchr<'h>` — [`Memchr`](../index.md)

##### `impl<'h> Debug for Memchr<'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'h> DoubleEndedIterator for Memchr<'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl<'h> FusedIterator for Memchr<'h>`

##### `impl<I> IntoIterator for Memchr<'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'h> Iterator for Memchr<'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn count(self: Self) -> usize`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Memchr2<'h>`

```rust
struct Memchr2<'h> {
    needle1: u8,
    needle2: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`](../index.md) or `[memrchr2_iter`]
functions. It can also be created with the `Memchr2::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- `fn new(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>` — [`Memchr2`](../index.md)

#### Trait Implementations

##### `impl<'h> Clone for Memchr2<'h>`

- `fn clone(self: &Self) -> Memchr2<'h>` — [`Memchr2`](../index.md)

##### `impl<'h> Debug for Memchr2<'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'h> DoubleEndedIterator for Memchr2<'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl<'h> FusedIterator for Memchr2<'h>`

##### `impl<I> IntoIterator for Memchr2<'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'h> Iterator for Memchr2<'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `Memchr3<'h>`

```rust
struct Memchr3<'h> {
    needle1: u8,
    needle2: u8,
    needle3: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`](../index.md) or `[memrchr2_iter`]
functions. It can also be created with the `Memchr3::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- `fn new(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>` — [`Memchr3`](../index.md)

#### Trait Implementations

##### `impl<'h> Clone for Memchr3<'h>`

- `fn clone(self: &Self) -> Memchr3<'h>` — [`Memchr3`](../index.md)

##### `impl<'h> Debug for Memchr3<'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'h> DoubleEndedIterator for Memchr3<'h>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl<'h> FusedIterator for Memchr3<'h>`

##### `impl<I> IntoIterator for Memchr3<'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'h> Iterator for Memchr3<'h>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

## Functions

### `memchr`

```rust
fn memchr(needle: u8, haystack: &[u8]) -> Option<usize>
```

Search for the first occurrence of a byte in a slice.

This returns the index corresponding to the first occurrence of `needle` in
`haystack`, or `None` if one is not found. If an index is returned, it is
guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().position(|&b| b == needle)`, this routine will attempt to
use highly optimized vector operations that can be an order of magnitude
faster (or more).

# Example

This shows how to find the first position of a byte in a byte string.

```rust
use memchr::memchr;

let haystack = b"the quick brown fox";
assert_eq!(memchr(b'k', haystack), Some(8));
```

### `memrchr`

```rust
fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize>
```

Search for the last occurrence of a byte in a slice.

This returns the index corresponding to the last occurrence of `needle` in
`haystack`, or `None` if one is not found. If an index is returned, it is
guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().rposition(|&b| b == needle)`, this routine will attempt to
use highly optimized vector operations that can be an order of magnitude
faster (or more).

# Example

This shows how to find the last position of a byte in a byte string.

```rust
use memchr::memrchr;

let haystack = b"the quick brown fox";
assert_eq!(memrchr(b'o', haystack), Some(17));
```

### `memchr2`

```rust
fn memchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize>
```

Search for the first occurrence of two possible bytes in a haystack.

This returns the index corresponding to the first occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().position(|&b| b == needle1 || b == needle2)`, this routine
will attempt to use highly optimized vector operations that can be an order
of magnitude faster (or more).

# Example

This shows how to find the first position of one of two possible bytes in a
haystack.

```rust
use memchr::memchr2;

let haystack = b"the quick brown fox";
assert_eq!(memchr2(b'k', b'q', haystack), Some(4));
```

### `memrchr2`

```rust
fn memrchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize>
```

Search for the last occurrence of two possible bytes in a haystack.

This returns the index corresponding to the last occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().rposition(|&b| b == needle1 || b == needle2)`, this
routine will attempt to use highly optimized vector operations that can be
an order of magnitude faster (or more).

# Example

This shows how to find the last position of one of two possible bytes in a
haystack.

```rust
use memchr::memrchr2;

let haystack = b"the quick brown fox";
assert_eq!(memrchr2(b'k', b'o', haystack), Some(17));
```

### `memchr3`

```rust
fn memchr3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize>
```

Search for the first occurrence of three possible bytes in a haystack.

This returns the index corresponding to the first occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().position(|&b| b == needle1 || b == needle2 || b == needle3)`,
this routine will attempt to use highly optimized vector operations that
can be an order of magnitude faster (or more).

# Example

This shows how to find the first position of one of three possible bytes in
a haystack.

```rust
use memchr::memchr3;

let haystack = b"the quick brown fox";
assert_eq!(memchr3(b'k', b'q', b'u', haystack), Some(4));
```

### `memrchr3`

```rust
fn memrchr3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize>
```

Search for the last occurrence of three possible bytes in a haystack.

This returns the index corresponding to the last occurrence of one of the
needle bytes in `haystack`, or `None` if one is not found. If an index is
returned, it is guaranteed to be less than `haystack.len()`.

While this is semantically the same as something like
`haystack.iter().rposition(|&b| b == needle1 || b == needle2 || b == needle3)`,
this routine will attempt to use highly optimized vector operations that
can be an order of magnitude faster (or more).

# Example

This shows how to find the last position of one of three possible bytes in
a haystack.

```rust
use memchr::memrchr3;

let haystack = b"the quick brown fox";
assert_eq!(memrchr3(b'k', b'o', b'n', haystack), Some(17));
```

### `memchr_iter`

```rust
fn memchr_iter<'h>(needle: u8, haystack: &'h [u8]) -> Memchr<'h>
```

Returns an iterator over all occurrences of the needle in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr_iter`

```rust
fn memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<Memchr<'_>>
```

Returns an iterator over all occurrences of the needle in a haystack, in
reverse.

### `memchr2_iter`

```rust
fn memchr2_iter<'h>(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>
```

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr2_iter`

```rust
fn memrchr2_iter(needle1: u8, needle2: u8, haystack: &[u8]) -> core::iter::Rev<Memchr2<'_>>
```

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

### `memchr3_iter`

```rust
fn memchr3_iter<'h>(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>
```

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr3_iter`

```rust
fn memrchr3_iter(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> core::iter::Rev<Memchr3<'_>>
```

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

### `memchr_raw`

```rust
unsafe fn memchr_raw(needle: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

memchr, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::find_raw`.

### `memrchr_raw`

```rust
unsafe fn memrchr_raw(needle: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

memrchr, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::rfind_raw`.

### `memchr2_raw`

```rust
unsafe fn memchr2_raw(needle1: u8, needle2: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

memchr2, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Two::find_raw`.

### `memrchr2_raw`

```rust
unsafe fn memrchr2_raw(needle1: u8, needle2: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

memrchr2, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Two::rfind_raw`.

### `memchr3_raw`

```rust
unsafe fn memchr3_raw(needle1: u8, needle2: u8, needle3: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

memchr3, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Three::find_raw`.

### `memrchr3_raw`

```rust
unsafe fn memrchr3_raw(needle1: u8, needle2: u8, needle3: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

memrchr3, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Three::rfind_raw`.

### `count_raw`

```rust
unsafe fn count_raw(needle: u8, start: *const u8, end: *const u8) -> usize
```

Count all matching bytes, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::count_raw`.
