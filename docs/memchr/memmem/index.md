*[memchr](../index.md) / [memmem](index.md)*

---

# Module `memmem`

This module provides forward and reverse substring search routines.

Unlike the standard library's substring search routines, these work on
arbitrary bytes. For all non-empty needles, these routines will report exactly
the same values as the corresponding routines in the standard library. For
the empty needle, the standard library reports matches only at valid UTF-8
boundaries, where as these routines will report matches at every position.

Other than being able to work on arbitrary bytes, the primary reason to prefer
these routines over the standard library routines is that these will generally
be faster. In some cases, significantly so.

# Example: iterating over substring matches

This example shows how to use [`find_iter`](memchr/memmem/index.md) to find occurrences of a substring
in a haystack.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# Example: iterating over substring matches in reverse

This example shows how to use [`rfind_iter`](memchr/memmem/index.md) to find occurrences of a substring
in a haystack starting from the end of the haystack.

**NOTE:** This module does not implement double ended iterators, so reverse
searches aren't done by calling `rev` on a forward iterator.

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::rfind_iter(haystack, "foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

# Example: repeating a search for the same needle

It may be possible for the overhead of constructing a substring searcher to be
measurable in some workloads. In cases where the same needle is used to search
many haystacks, it is possible to do construction once and thus to avoid it for
subsequent searches. This can be done with a [`Finder`](memchr/arch/all/packedpair/index.md) (or a [`FinderRev`](memchr/arch/all/rabinkarp/index.md) for
reverse searches).

```
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```

## Structs

### `FindIter<'h, 'n>`

```rust
struct FindIter<'h, 'n> {
}
```

An iterator over non-overlapping substring matches.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

#### Implementations

- `fn into_owned(self: Self) -> FindIter<'h, 'static>`
  Convert this iterator into its owned variant, such that it no longer

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'h, 'n>`

- `fn clone(self: &Self) -> FindIter<'h, 'n>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Iterator<'h, 'n>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, 'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindRevIter<'h, 'n>`

```rust
struct FindRevIter<'h, 'n> {
}
```

An iterator over non-overlapping substring matches in reverse.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

#### Implementations

- `fn into_owned(self: Self) -> FindRevIter<'h, 'static>`
  Convert this iterator into its owned variant, such that it no longer

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'h, 'n>`

- `fn clone(self: &Self) -> FindRevIter<'h, 'n>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Iterator<'h, 'n>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'h, 'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Finder<'n>`

```rust
struct Finder<'n> {
}
```

A single substring searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`find`](memchr/memmem/index.md) is good enough, but `Finder` is useful when you can meaningfully
observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `Finder` that is not connected to
the lifetime of its needle.

#### Implementations

- `fn new<B: ?Sized + AsRef<[u8]>>(needle: &'n B) -> Finder<'n>`
  Create a new finder for the given needle.

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`
  Returns the index of the first occurrence of this needle in the given

- `fn find_iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> FindIter<'h, 'a>`
  Returns an iterator over all occurrences of a substring in a haystack.

- `fn into_owned(self: Self) -> Finder<'static>`
  Convert this finder into its owned variant, such that it no longer

- `fn as_ref(self: &Self) -> Finder<'_>`
  Convert this finder into its borrowed variant.

- `fn needle(self: &Self) -> &[u8]`
  Returns the needle that this finder searches for.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'n>`

- `fn clone(self: &Self) -> Finder<'n>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FinderRev<'n>`

```rust
struct FinderRev<'n> {
}
```

A single substring reverse searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general,
using [`rfind`](memchr/memmem/index.md) is good enough, but `FinderRev` is useful when you can
meaningfully observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `FinderRev` that is not connected to
the lifetime of its needle.

#### Implementations

- `fn new<B: ?Sized + AsRef<[u8]>>(needle: &'n B) -> FinderRev<'n>`
  Create a new reverse finder for the given needle.

- `fn rfind<B: AsRef<[u8]>>(self: &Self, haystack: B) -> Option<usize>`
  Returns the index of the last occurrence of this needle in the given

- `fn rfind_iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> FindRevIter<'h, 'a>`
  Returns a reverse iterator over all occurrences of a substring in a

- `fn into_owned(self: Self) -> FinderRev<'static>`
  Convert this finder into its owned variant, such that it no longer

- `fn as_ref(self: &Self) -> FinderRev<'_>`
  Convert this finder into its borrowed variant.

- `fn needle(self: &Self) -> &[u8]`
  Returns the needle that this finder searches for.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'n>`

- `fn clone(self: &Self) -> FinderRev<'n>`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FinderBuilder`

```rust
struct FinderBuilder {
}
```

A builder for constructing non-default forward or reverse memmem finders.

A builder is primarily useful for configuring a substring searcher.
Currently, the only configuration exposed is the ability to disable
heuristic prefilters used to speed up certain searches.

#### Implementations

- `fn new() -> FinderBuilder`
  Create a new finder builder with default settings.

- `fn build_forward<'n, B: ?Sized + AsRef<[u8]>>(self: &Self, needle: &'n B) -> Finder<'n>`
  Build a forward finder using the given needle from the current

- `fn build_forward_with_ranker<'n, R: HeuristicFrequencyRank, B: ?Sized + AsRef<[u8]>>(self: &Self, ranker: R, needle: &'n B) -> Finder<'n>`
  Build a forward finder using the given needle and a custom heuristic for

- `fn build_reverse<'n, B: ?Sized + AsRef<[u8]>>(self: &Self, needle: &'n B) -> FinderRev<'n>`
  Build a reverse finder using the given needle from the current

- `fn prefilter(self: &mut Self, prefilter: Prefilter) -> &mut FinderBuilder`
  Configure the prefilter setting for the finder.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> FinderBuilder`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> FinderBuilder`

## Enums

### `Prefilter`

```rust
enum Prefilter {
    None,
    Auto,
}
```

Prefilter controls whether heuristics are used to accelerate searching.

A prefilter refers to the idea of detecting candidate matches very quickly,
and then confirming whether those candidates are full matches. This
idea can be quite effective since it's often the case that looking for
candidates can be a lot faster than running a complete substring search
over the entire input. Namely, looking for candidates can be done with
extremely fast vectorized code.

The downside of a prefilter is that it assumes false positives (which are
candidates generated by a prefilter that aren't matches) are somewhat rare
relative to the frequency of full matches. That is, if a lot of false
positives are generated, then it's possible for search time to be worse
than if the prefilter wasn't enabled in the first place.

Another downside of a prefilter is that it can result in highly variable
performance, where some cases are extraordinarily fast and others aren't.
Typically, variable performance isn't a problem, but it may be for your use
case.

The use of prefilters in this implementation does use a heuristic to detect
when a prefilter might not be carrying its weight, and will dynamically
disable its use. Nevertheless, this configuration option gives callers
the ability to disable prefilters if you have knowledge that they won't be
useful.

#### Variants

- **`None`**

  Never used a prefilter in substring search.

- **`Auto`**

  Automatically detect whether a heuristic prefilter should be used. If
  it is used, then heuristics will be used to dynamically disable the
  prefilter if it is believed to not be carrying its weight.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> PrefilterConfig`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> PrefilterConfig`

## Functions

### `find_iter`

```rust
fn find_iter<'h, 'n, N: 'n + ?Sized + AsRef<[u8]>>(haystack: &'h [u8], needle: &'n N) -> FindIter<'h, 'n>
```

Returns an iterator over all non-overlapping occurrences of a substring in
a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::find_iter(haystack, b"foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

### `rfind_iter`

```rust
fn rfind_iter<'h, 'n, N: 'n + ?Sized + AsRef<[u8]>>(haystack: &'h [u8], needle: &'n N) -> FindRevIter<'h, 'n>
```

Returns a reverse iterator over all non-overlapping occurrences of a
substring in a haystack.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar foo baz foo";
let mut it = memmem::rfind_iter(haystack, b"foo");
assert_eq!(Some(16), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(0), it.next());
assert_eq!(None, it.next());
```

### `find`

```rust
fn find(haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Returns the index of the first occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`Finder`](memchr/arch/all/packedpair/index.md) once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::find(haystack, b"foo"));
assert_eq!(Some(4), memmem::find(haystack, b"bar"));
assert_eq!(None, memmem::find(haystack, b"quux"));
```

### `rfind`

```rust
fn rfind(haystack: &[u8], needle: &[u8]) -> Option<usize>
```

Returns the index of the last occurrence of the given needle.

Note that if you're are searching for the same needle in many different
small haystacks, it may be faster to initialize a [`FinderRev`](memchr/arch/all/rabinkarp/index.md) once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::rfind(haystack, b"foo"));
assert_eq!(Some(4), memmem::rfind(haystack, b"bar"));
assert_eq!(Some(8), memmem::rfind(haystack, b"ba"));
assert_eq!(None, memmem::rfind(haystack, b"quux"));
```

