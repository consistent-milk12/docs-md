*[memchr](../index.md) / [memchr](index.md)*

---

# Module `memchr`

## Contents

- [Structs](#structs)
  - [`Memchr`](#memchr)
  - [`Memchr2`](#memchr2)
  - [`Memchr3`](#memchr3)
- [Functions](#functions)
  - [`memchr`](#memchr)
  - [`memrchr`](#memrchr)
  - [`memchr2`](#memchr2)
  - [`memrchr2`](#memrchr2)
  - [`memchr3`](#memchr3)
  - [`memrchr3`](#memrchr3)
  - [`memchr_iter`](#memchr-iter)
  - [`memrchr_iter`](#memrchr-iter)
  - [`memchr2_iter`](#memchr2-iter)
  - [`memrchr2_iter`](#memrchr2-iter)
  - [`memchr3_iter`](#memchr3-iter)
  - [`memrchr3_iter`](#memrchr3-iter)
  - [`memchr_raw`](#memchr-raw)
  - [`memrchr_raw`](#memrchr-raw)
  - [`memchr2_raw`](#memchr2-raw)
  - [`memrchr2_raw`](#memrchr2-raw)
  - [`memchr3_raw`](#memchr3-raw)
  - [`memrchr3_raw`](#memrchr3-raw)
  - [`count_raw`](#count-raw)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Memchr`](#memchr) | struct | An iterator over all occurrences of a single byte in a haystack. |
| [`Memchr2`](#memchr2) | struct | An iterator over all occurrences of two possible bytes in a haystack. |
| [`Memchr3`](#memchr3) | struct | An iterator over all occurrences of three possible bytes in a haystack. |
| [`memchr`](#memchr) | fn | Search for the first occurrence of a byte in a slice. |
| [`memrchr`](#memrchr) | fn | Search for the last occurrence of a byte in a slice. |
| [`memchr2`](#memchr2) | fn | Search for the first occurrence of two possible bytes in a haystack. |
| [`memrchr2`](#memrchr2) | fn | Search for the last occurrence of two possible bytes in a haystack. |
| [`memchr3`](#memchr3) | fn | Search for the first occurrence of three possible bytes in a haystack. |
| [`memrchr3`](#memrchr3) | fn | Search for the last occurrence of three possible bytes in a haystack. |
| [`memchr_iter`](#memchr-iter) | fn | Returns an iterator over all occurrences of the needle in a haystack. |
| [`memrchr_iter`](#memrchr-iter) | fn | Returns an iterator over all occurrences of the needle in a haystack, in reverse. |
| [`memchr2_iter`](#memchr2-iter) | fn | Returns an iterator over all occurrences of the needles in a haystack. |
| [`memrchr2_iter`](#memrchr2-iter) | fn | Returns an iterator over all occurrences of the needles in a haystack, in reverse. |
| [`memchr3_iter`](#memchr3-iter) | fn | Returns an iterator over all occurrences of the needles in a haystack. |
| [`memrchr3_iter`](#memrchr3-iter) | fn | Returns an iterator over all occurrences of the needles in a haystack, in reverse. |
| [`memchr_raw`](#memchr-raw) | fn | memchr, but using raw pointers to represent the haystack. |
| [`memrchr_raw`](#memrchr-raw) | fn | memrchr, but using raw pointers to represent the haystack. |
| [`memchr2_raw`](#memchr2-raw) | fn | memchr2, but using raw pointers to represent the haystack. |
| [`memrchr2_raw`](#memrchr2-raw) | fn | memrchr2, but using raw pointers to represent the haystack. |
| [`memchr3_raw`](#memchr3-raw) | fn | memchr3, but using raw pointers to represent the haystack. |
| [`memrchr3_raw`](#memrchr3-raw) | fn | memrchr3, but using raw pointers to represent the haystack. |
| [`count_raw`](#count-raw) | fn | Count all matching bytes, but using raw pointers to represent the haystack. |

## Structs

### `Memchr<'h>`

```rust
struct Memchr<'h> {
    needle1: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/memchr.rs:288-291`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L288-L291)*

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr_iter`](#memchr-iter) or `[memrchr_iter`]
functions. It can also be created with the `Memchr::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- <span id="memchr-new"></span>`fn new(needle1: u8, haystack: &'h [u8]) -> Memchr<'h>` — [`Memchr`](#memchr)

  Returns an iterator over all occurrences of the needle byte in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

#### Trait Implementations

##### `impl Any for Memchr<'h>`

- <span id="memchr-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memchr<'h>`

- <span id="memchr-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memchr<'h>`

- <span id="memchr-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memchr<'h>`

- <span id="memchr-clone"></span>`fn clone(&self) -> Memchr<'h>` — [`Memchr`](#memchr)

##### `impl CloneToUninit for Memchr<'h>`

- <span id="memchr-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memchr<'h>`

- <span id="memchr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Memchr<'h>`

- <span id="memchr-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for Memchr<'h>`

- <span id="memchr-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Memchr<'h>`

##### `impl<U> Into for Memchr<'h>`

- <span id="memchr-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Memchr<'h>`

- <span id="memchr-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="memchr-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="memchr-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Memchr<'h>`

- <span id="memchr-iterator-type-item"></span>`type Item = usize`

- <span id="memchr-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="memchr-iterator-count"></span>`fn count(self) -> usize`

- <span id="memchr-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Memchr<'h>`

- <span id="memchr-toowned-type-owned"></span>`type Owned = T`

- <span id="memchr-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memchr-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memchr<'h>`

- <span id="memchr-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memchr-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memchr<'h>`

- <span id="memchr-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memchr-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Memchr2<'h>`

```rust
struct Memchr2<'h> {
    needle1: u8,
    needle2: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/memchr.rs:364-368`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L364-L368)*

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`](#memchr2-iter) or `[memrchr2_iter`]
functions. It can also be created with the `Memchr2::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- <span id="memchr2-new"></span>`fn new(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>` — [`Memchr2`](#memchr2)

  Returns an iterator over all occurrences of the needle bytes in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

#### Trait Implementations

##### `impl Any for Memchr2<'h>`

- <span id="memchr2-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memchr2<'h>`

- <span id="memchr2-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memchr2<'h>`

- <span id="memchr2-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memchr2<'h>`

- <span id="memchr2-clone"></span>`fn clone(&self) -> Memchr2<'h>` — [`Memchr2`](#memchr2)

##### `impl CloneToUninit for Memchr2<'h>`

- <span id="memchr2-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memchr2<'h>`

- <span id="memchr2-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Memchr2<'h>`

- <span id="memchr2-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for Memchr2<'h>`

- <span id="memchr2-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Memchr2<'h>`

##### `impl<U> Into for Memchr2<'h>`

- <span id="memchr2-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Memchr2<'h>`

- <span id="memchr2-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="memchr2-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="memchr2-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Memchr2<'h>`

- <span id="memchr2-iterator-type-item"></span>`type Item = usize`

- <span id="memchr2-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="memchr2-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Memchr2<'h>`

- <span id="memchr2-toowned-type-owned"></span>`type Owned = T`

- <span id="memchr2-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memchr2-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memchr2<'h>`

- <span id="memchr2-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memchr2-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memchr2<'h>`

- <span id="memchr2-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memchr2-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Memchr3<'h>`

```rust
struct Memchr3<'h> {
    needle1: u8,
    needle2: u8,
    needle3: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/memchr.rs:432-437`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L432-L437)*

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`](#memchr2-iter) or `[memrchr2_iter`]
functions. It can also be created with the `Memchr3::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- <span id="memchr3-new"></span>`fn new(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>` — [`Memchr3`](#memchr3)

  Returns an iterator over all occurrences of the needle bytes in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

#### Trait Implementations

##### `impl Any for Memchr3<'h>`

- <span id="memchr3-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memchr3<'h>`

- <span id="memchr3-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memchr3<'h>`

- <span id="memchr3-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memchr3<'h>`

- <span id="memchr3-clone"></span>`fn clone(&self) -> Memchr3<'h>` — [`Memchr3`](#memchr3)

##### `impl CloneToUninit for Memchr3<'h>`

- <span id="memchr3-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memchr3<'h>`

- <span id="memchr3-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for Memchr3<'h>`

- <span id="memchr3-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for Memchr3<'h>`

- <span id="memchr3-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Memchr3<'h>`

##### `impl<U> Into for Memchr3<'h>`

- <span id="memchr3-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Memchr3<'h>`

- <span id="memchr3-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="memchr3-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="memchr3-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Memchr3<'h>`

- <span id="memchr3-iterator-type-item"></span>`type Item = usize`

- <span id="memchr3-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="memchr3-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for Memchr3<'h>`

- <span id="memchr3-toowned-type-owned"></span>`type Owned = T`

- <span id="memchr3-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memchr3-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memchr3<'h>`

- <span id="memchr3-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memchr3-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memchr3<'h>`

- <span id="memchr3-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memchr3-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `memchr`

```rust
fn memchr(needle: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:27-35`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L27-L35)*

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

*Defined in [`memchr-2.7.6/src/memchr.rs:59-67`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L59-L67)*

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

*Defined in [`memchr-2.7.6/src/memchr.rs:92-100`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L92-L100)*

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

*Defined in [`memchr-2.7.6/src/memchr.rs:125-133`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L125-L133)*

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

*Defined in [`memchr-2.7.6/src/memchr.rs:158-171`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L158-L171)*

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

*Defined in [`memchr-2.7.6/src/memchr.rs:196-209`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L196-L209)*

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

*Defined in [`memchr-2.7.6/src/memchr.rs:216-218`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L216-L218)*

Returns an iterator over all occurrences of the needle in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr_iter`

```rust
fn memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<Memchr<'_>>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:223-225`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L223-L225)*

Returns an iterator over all occurrences of the needle in a haystack, in
reverse.

### `memchr2_iter`

```rust
fn memchr2_iter<'h>(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:232-238`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L232-L238)*

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr2_iter`

```rust
fn memrchr2_iter(needle1: u8, needle2: u8, haystack: &[u8]) -> core::iter::Rev<Memchr2<'_>>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:243-249`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L243-L249)*

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

### `memchr3_iter`

```rust
fn memchr3_iter<'h>(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:256-263`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L256-L263)*

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr3_iter`

```rust
fn memrchr3_iter(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> core::iter::Rev<Memchr3<'_>>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:268-275`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L268-L275)*

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

### `memchr_raw`

```rust
unsafe fn memchr_raw(needle: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:504-533`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L504-L533)*

memchr, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::find_raw`.

### `memrchr_raw`

```rust
unsafe fn memrchr_raw(needle: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:541-566`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L541-L566)*

memrchr, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::rfind_raw`.

### `memchr2_raw`

```rust
unsafe fn memchr2_raw(needle1: u8, needle2: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:574-601`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L574-L601)*

memchr2, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Two::find_raw`.

### `memrchr2_raw`

```rust
unsafe fn memrchr2_raw(needle1: u8, needle2: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:609-638`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L609-L638)*

memrchr2, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Two::rfind_raw`.

### `memchr3_raw`

```rust
unsafe fn memchr3_raw(needle1: u8, needle2: u8, needle3: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:646-680`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L646-L680)*

memchr3, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Three::find_raw`.

### `memrchr3_raw`

```rust
unsafe fn memrchr3_raw(needle1: u8, needle2: u8, needle3: u8, start: *const u8, end: *const u8) -> Option<*const u8>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:688-722`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L688-L722)*

memrchr3, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `Three::rfind_raw`.

### `count_raw`

```rust
unsafe fn count_raw(needle: u8, start: *const u8, end: *const u8) -> usize
```

*Defined in [`memchr-2.7.6/src/memchr.rs:730-751`](../../../.source_1765633015/memchr-2.7.6/src/memchr.rs#L730-L751)*

Count all matching bytes, but using raw pointers to represent the haystack.

# Safety

Pointers must be valid. See `One::count_raw`.

