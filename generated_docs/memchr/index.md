# Crate `memchr`

This library provides heavily optimized routines for string search primitives.

# Overview

This section gives a brief high level overview of what this crate offers.

* The top-level module provides routines for searching for 1, 2 or 3 bytes
  in the forward or reverse direction. When searching for more than one byte,
  positions are considered a match if the byte at that position matches any
  of the bytes.
* The [`memmem`](memmem/index.md) sub-module provides forward and reverse substring search
  routines.

In all such cases, routines operate on `&[u8]` without regard to encoding. This
is exactly what you want when searching either UTF-8 or arbitrary bytes.

# Example: using `memchr`

This example shows how to use `memchr` to find the first occurrence of `z` in
a haystack:

```rust
use memchr::memchr;

let haystack = b"foo bar baz quuz";
assert_eq!(Some(10), memchr(b'z', haystack));
```

# Example: matching one of three possible bytes

This examples shows how to use `memrchr3` to find occurrences of `a`, `b` or
`c`, starting at the end of the haystack.

```rust
use memchr::memchr3_iter;

let haystack = b"xyzaxyzbxyzc";

let mut it = memchr3_iter(b'a', b'b', b'c', haystack).rev();
assert_eq!(Some(11), it.next());
assert_eq!(Some(7), it.next());
assert_eq!(Some(3), it.next());
assert_eq!(None, it.next());
```

# Example: iterating over substring matches

This example shows how to use the [`memmem`](memmem/index.md) sub-module to find occurrences of
a substring in a haystack.

```rust
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# Example: repeating a search for the same needle

It may be possible for the overhead of constructing a substring searcher to be
measurable in some workloads. In cases where the same needle is used to search
many haystacks, it is possible to do construction once and thus to avoid it for
subsequent searches. This can be done with a [`memmem::Finder`](memmem/index.md):

```rust
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```

# Why use this crate?

At first glance, the APIs provided by this crate might seem weird. Why provide
a dedicated routine like `memchr` for something that could be implemented
clearly and trivially in one line:

```rust
fn memchr(needle: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == needle)
}
```

Or similarly, why does this crate provide substring search routines when Rust's
core library already provides them?

```rust
fn search(haystack: &str, needle: &str) -> Option<usize> {
    haystack.find(needle)
}
```

The primary reason for both of them to exist is performance. When it comes to
performance, at a high level at least, there are two primary ways to look at
it:

* **Throughput**: For this, think about it as, "given some very large haystack
  and a byte that never occurs in that haystack, how long does it take to
  search through it and determine that it, in fact, does not occur?"
* **Latency**: For this, think about it as, "given a tiny haystack---just a
  few bytes---how long does it take to determine if a byte is in it?"

The `memchr` routine in this crate has _slightly_ worse latency than the
solution presented above, however, its throughput can easily be over an
order of magnitude faster. This is a good general purpose trade off to make.
You rarely lose, but often gain big.

**NOTE:** The name `memchr` comes from the corresponding routine in `libc`. A
key advantage of using this library is that its performance is not tied to its
quality of implementation in the `libc` you happen to be using, which can vary
greatly from platform to platform.

But what about substring search? This one is a bit more complicated. The
primary reason for its existence is still indeed performance, but it's also
useful because Rust's core library doesn't actually expose any substring
search routine on arbitrary bytes. The only substring search routine that
exists works exclusively on valid UTF-8.

So if you have valid UTF-8, is there a reason to use this over the standard
library substring search routine? Yes. This routine is faster on almost every
metric, including latency. The natural question then, is why isn't this
implementation in the standard library, even if only for searching on UTF-8?
The reason is that the implementation details for using SIMD in the standard
library haven't quite been worked out yet.

**NOTE:** Currently, only `x86_64`, `wasm32` and `aarch64` targets have vector
accelerated implementations of `memchr` (and friends) and `memmem`.

# Crate features

* **std** - When enabled (the default), this will permit features specific to
the standard library. Currently, the only thing used from the standard library
is runtime SIMD CPU feature detection. This means that this feature must be
enabled to get AVX2 accelerated routines on `x86_64` targets without enabling
the `avx2` feature at compile time, for example. When `std` is not enabled,
this crate will still attempt to use SSE2 accelerated routines on `x86_64`. It
will also use AVX2 accelerated routines when the `avx2` feature is enabled at
compile time. In general, enable this feature if you can.
* **alloc** - When enabled (the default), APIs in this crate requiring some
kind of allocation will become available. For example, the
[`memmem::Finder::into_owned`](crate::memmem::Finder::into_owned) API and the
[`arch::all::shiftor`](crate::arch::all::shiftor) substring search
implementation. Otherwise, this crate is designed from the ground up to be
usable in core-only contexts, so the `alloc` feature doesn't add much
currently. Notably, disabling `std` but enabling `alloc` will **not** result
in the use of AVX2 on `x86_64` targets unless the `avx2` feature is enabled
at compile time. (With `std` enabled, AVX2 can be used even without the `avx2`
feature enabled at compile time by way of runtime CPU feature detection.)
* **logging** - When enabled (disabled by default), the `log` crate is used
to emit log messages about what kinds of `memchr` and `memmem` algorithms
are used. Namely, both `memchr` and `memmem` have a number of different
implementation choices depending on the target and CPU, and the log messages
can help show what specific implementations are being used. Generally, this is
useful for debugging performance issues.
* **libc** - **DEPRECATED**. Previously, this enabled the use of the target's
`memchr` function from whatever `libc` was linked into the program. This
feature is now a no-op because this crate's implementation of `memchr` should
now be sufficiently fast on a number of platforms that `libc` should no longer
be needed. (This feature is somewhat of a holdover from this crate's origins.
Originally, this crate was literally just a safe wrapper function around the
`memchr` function from `libc`.)

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`arch`](#arch)
  - [`cow`](#cow)
  - [`ext`](#ext)
  - [`memchr`](#memchr)
  - [`memmem`](#memmem)
  - [`vector`](#vector)
- [Structs](#structs)
  - [`Memchr`](#memchr)
  - [`Memchr2`](#memchr2)
  - [`Memchr3`](#memchr3)
- [Functions](#functions)
  - [`memchr`](#memchr)
  - [`memchr2`](#memchr2)
  - [`memchr2_iter`](#memchr2-iter)
  - [`memchr3`](#memchr3)
  - [`memchr3_iter`](#memchr3-iter)
  - [`memchr_iter`](#memchr-iter)
  - [`memrchr`](#memrchr)
  - [`memrchr2`](#memrchr2)
  - [`memrchr2_iter`](#memrchr2-iter)
  - [`memrchr3`](#memrchr3)
  - [`memrchr3_iter`](#memrchr3-iter)
  - [`memrchr_iter`](#memrchr-iter)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`arch`](#arch) | mod | A module with low-level architecture dependent routines. |
| [`cow`](#cow) | mod |  |
| [`ext`](#ext) | mod |  |
| [`memchr`](#memchr) | mod |  |
| [`memmem`](#memmem) | mod | This module provides forward and reverse substring search routines. |
| [`vector`](#vector) | mod |  |
| [`Memchr`](#memchr) | struct |  |
| [`Memchr2`](#memchr2) | struct |  |
| [`Memchr3`](#memchr3) | struct |  |
| [`memchr`](#memchr) | fn |  |
| [`memchr2`](#memchr2) | fn |  |
| [`memchr2_iter`](#memchr2-iter) | fn |  |
| [`memchr3`](#memchr3) | fn |  |
| [`memchr3_iter`](#memchr3-iter) | fn |  |
| [`memchr_iter`](#memchr-iter) | fn |  |
| [`memrchr`](#memrchr) | fn |  |
| [`memrchr2`](#memrchr2) | fn |  |
| [`memrchr2_iter`](#memrchr2-iter) | fn |  |
| [`memrchr3`](#memrchr3) | fn |  |
| [`memrchr3_iter`](#memrchr3-iter) | fn |  |
| [`memrchr_iter`](#memrchr-iter) | fn |  |

## Modules

- [`macros`](macros/index.md)
- [`arch`](arch/index.md) — A module with low-level architecture dependent routines.
- [`cow`](cow/index.md)
- [`ext`](ext/index.md)
- [`memchr`](memchr/index.md)
- [`memmem`](memmem/index.md) — This module provides forward and reverse substring search routines.
- [`vector`](vector/index.md)

## Structs

### `Memchr<'h>`

```rust
struct Memchr<'h> {
    needle1: u8,
    it: crate::arch::generic::memchr::Iter<'h>,
}
```

*Defined in [`memchr-2.7.6/src/memchr.rs:288-291`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L288-L291)*

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr_iter`](memchr/index.md) or `[memrchr_iter`]
functions. It can also be created with the `Memchr::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- <span id="memchr-new"></span>`fn new(needle1: u8, haystack: &'h [u8]) -> Memchr<'h>` — [`Memchr`](memchr/index.md#memchr)

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

- <span id="memchr-clone"></span>`fn clone(&self) -> Memchr<'h>` — [`Memchr`](memchr/index.md#memchr)

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

*Defined in [`memchr-2.7.6/src/memchr.rs:364-368`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L364-L368)*

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`](memchr/index.md) or `[memrchr2_iter`]
functions. It can also be created with the `Memchr2::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- <span id="memchr2-new"></span>`fn new(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>` — [`Memchr2`](memchr/index.md#memchr2)

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

- <span id="memchr2-clone"></span>`fn clone(&self) -> Memchr2<'h>` — [`Memchr2`](memchr/index.md#memchr2)

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

*Defined in [`memchr-2.7.6/src/memchr.rs:432-437`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L432-L437)*

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`memchr2_iter`](memchr/index.md) or `[memrchr2_iter`]
functions. It can also be created with the `Memchr3::new` method.

The lifetime parameter `'h` refers to the lifetime of the haystack being
searched.

#### Implementations

- <span id="memchr3-new"></span>`fn new(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>` — [`Memchr3`](memchr/index.md#memchr3)

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

- <span id="memchr3-clone"></span>`fn clone(&self) -> Memchr3<'h>` — [`Memchr3`](memchr/index.md#memchr3)

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

*Defined in [`memchr-2.7.6/src/memchr.rs:27-35`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L27-L35)*

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

### `memchr2`

```rust
fn memchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:92-100`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L92-L100)*

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

### `memchr2_iter`

```rust
fn memchr2_iter<'h>(needle1: u8, needle2: u8, haystack: &'h [u8]) -> Memchr2<'h>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:232-238`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L232-L238)*

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memchr3`

```rust
fn memchr3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:158-171`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L158-L171)*

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

### `memchr3_iter`

```rust
fn memchr3_iter<'h>(needle1: u8, needle2: u8, needle3: u8, haystack: &'h [u8]) -> Memchr3<'h>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:256-263`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L256-L263)*

Returns an iterator over all occurrences of the needles in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memchr_iter`

```rust
fn memchr_iter<'h>(needle: u8, haystack: &'h [u8]) -> Memchr<'h>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:216-218`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L216-L218)*

Returns an iterator over all occurrences of the needle in a haystack.

The iterator returned implements `DoubleEndedIterator`. This means it
can also be used to find occurrences in reverse order.

### `memrchr`

```rust
fn memrchr(needle: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:59-67`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L59-L67)*

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

### `memrchr2`

```rust
fn memrchr2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:125-133`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L125-L133)*

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

### `memrchr2_iter`

```rust
fn memrchr2_iter(needle1: u8, needle2: u8, haystack: &[u8]) -> core::iter::Rev<Memchr2<'_>>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:243-249`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L243-L249)*

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

### `memrchr3`

```rust
fn memrchr3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:196-209`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L196-L209)*

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

### `memrchr3_iter`

```rust
fn memrchr3_iter(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> core::iter::Rev<Memchr3<'_>>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:268-275`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L268-L275)*

Returns an iterator over all occurrences of the needles in a haystack, in
reverse.

### `memrchr_iter`

```rust
fn memrchr_iter(needle: u8, haystack: &[u8]) -> core::iter::Rev<Memchr<'_>>
```

*Defined in [`memchr-2.7.6/src/memchr.rs:223-225`](../../.source_1765521767/memchr-2.7.6/src/memchr.rs#L223-L225)*

Returns an iterator over all occurrences of the needle in a haystack, in
reverse.

