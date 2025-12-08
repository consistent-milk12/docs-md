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

This example shows how to use [`find_iter`](#find-iter) to find occurrences of a substring
in a haystack.

```rust
use memchr::memmem;

let haystack = b"foo bar foo baz foo";

let mut it = memmem::find_iter(haystack, "foo");
assert_eq!(Some(0), it.next());
assert_eq!(Some(8), it.next());
assert_eq!(Some(16), it.next());
assert_eq!(None, it.next());
```

# Example: iterating over substring matches in reverse

This example shows how to use [`rfind_iter`](#rfind-iter) to find occurrences of a substring
in a haystack starting from the end of the haystack.

**NOTE:** This module does not implement double ended iterators, so reverse
searches aren't done by calling `rev` on a forward iterator.

```rust
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
subsequent searches. This can be done with a [`Finder`](../arch/all/twoway/index.md) (or a [`FinderRev`](../arch/all/rabinkarp/index.md) for
reverse searches).

```rust
use memchr::memmem;

let finder = memmem::Finder::new("foo");

assert_eq!(Some(4), finder.find(b"baz foo quux"));
assert_eq!(None, finder.find(b"quux baz bar"));
```

## Structs

### `FindIter<'h, 'n>`

```rust
struct FindIter<'h, 'n> {
    haystack: &'h [u8],
    prestate: crate::memmem::searcher::PrefilterState,
    finder: Finder<'n>,
    pos: usize,
}
```

An iterator over non-overlapping substring matches.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

#### Implementations

- `fn new(haystack: &'h [u8], finder: Finder<'n>) -> FindIter<'h, 'n>` — [`Finder`](#finder), [`FindIter`](#finditer)

- `fn into_owned(self: Self) -> FindIter<'h, 'static>` — [`FindIter`](#finditer)

#### Trait Implementations

##### `impl<'h, 'n> Clone for FindIter<'h, 'n>`

- `fn clone(self: &Self) -> FindIter<'h, 'n>` — [`FindIter`](#finditer)

##### `impl<'h, 'n> Debug for FindIter<'h, 'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindIter<'h, 'n>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'h, 'n> Iterator for FindIter<'h, 'n>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `FindRevIter<'h, 'n>`

```rust
struct FindRevIter<'h, 'n> {
    haystack: &'h [u8],
    finder: FinderRev<'n>,
    pos: Option<usize>,
}
```

An iterator over non-overlapping substring matches in reverse.

Matches are reported by the byte offset at which they begin.

`'h` is the lifetime of the haystack while `'n` is the lifetime of the
needle.

#### Fields

- **`pos`**: `Option<usize>`

  When searching with an empty needle, this gets set to `None` after
  we've yielded the last element at `0`.

#### Implementations

- `fn new(haystack: &'h [u8], finder: FinderRev<'n>) -> FindRevIter<'h, 'n>` — [`FinderRev`](#finderrev), [`FindRevIter`](#findreviter)

- `fn into_owned(self: Self) -> FindRevIter<'h, 'static>` — [`FindRevIter`](#findreviter)

#### Trait Implementations

##### `impl<'h, 'n> Clone for FindRevIter<'h, 'n>`

- `fn clone(self: &Self) -> FindRevIter<'h, 'n>` — [`FindRevIter`](#findreviter)

##### `impl<'h, 'n> Debug for FindRevIter<'h, 'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindRevIter<'h, 'n>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'h, 'n> Iterator for FindRevIter<'h, 'n>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

### `Finder<'n>`

```rust
struct Finder<'n> {
    needle: crate::cow::CowBytes<'n>,
    searcher: crate::memmem::searcher::Searcher,
}
```

A single substring searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general, using
[`find`](#find) is good enough, but `Finder` is useful when you can meaningfully
observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `Finder` that is not connected to
the lifetime of its needle.

#### Implementations

- `fn new<B: ?Sized + AsRef<[u8]>>(needle: &'n B) -> Finder<'n>` — [`Finder`](#finder)

- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>`

- `fn find_iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> FindIter<'h, 'a>` — [`FindIter`](#finditer)

- `fn into_owned(self: Self) -> Finder<'static>` — [`Finder`](#finder)

- `fn as_ref(self: &Self) -> Finder<'_>` — [`Finder`](#finder)

- `fn needle(self: &Self) -> &[u8]`

#### Trait Implementations

##### `impl<'n> Clone for Finder<'n>`

- `fn clone(self: &Self) -> Finder<'n>` — [`Finder`](#finder)

##### `impl<'n> Debug for Finder<'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FinderRev<'n>`

```rust
struct FinderRev<'n> {
    needle: crate::cow::CowBytes<'n>,
    searcher: crate::memmem::searcher::SearcherRev,
}
```

A single substring reverse searcher fixed to a particular needle.

The purpose of this type is to permit callers to construct a substring
searcher that can be used to search haystacks without the overhead of
constructing the searcher in the first place. This is a somewhat niche
concern when it's necessary to re-use the same needle to search multiple
different haystacks with as little overhead as possible. In general,
using [`rfind`](#rfind) is good enough, but `FinderRev` is useful when you can
meaningfully observe searcher construction time in a profile.

When the `std` feature is enabled, then this type has an `into_owned`
version which permits building a `FinderRev` that is not connected to
the lifetime of its needle.

#### Implementations

- `fn new<B: ?Sized + AsRef<[u8]>>(needle: &'n B) -> FinderRev<'n>` — [`FinderRev`](#finderrev)

- `fn rfind<B: AsRef<[u8]>>(self: &Self, haystack: B) -> Option<usize>`

- `fn rfind_iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> FindRevIter<'h, 'a>` — [`FindRevIter`](#findreviter)

- `fn into_owned(self: Self) -> FinderRev<'static>` — [`FinderRev`](#finderrev)

- `fn as_ref(self: &Self) -> FinderRev<'_>` — [`FinderRev`](#finderrev)

- `fn needle(self: &Self) -> &[u8]`

#### Trait Implementations

##### `impl<'n> Clone for FinderRev<'n>`

- `fn clone(self: &Self) -> FinderRev<'n>` — [`FinderRev`](#finderrev)

##### `impl<'n> Debug for FinderRev<'n>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FinderBuilder`

```rust
struct FinderBuilder {
    prefilter: Prefilter,
}
```

A builder for constructing non-default forward or reverse memmem finders.

A builder is primarily useful for configuring a substring searcher.
Currently, the only configuration exposed is the ability to disable
heuristic prefilters used to speed up certain searches.

#### Implementations

- `fn new() -> FinderBuilder` — [`FinderBuilder`](#finderbuilder)

- `fn build_forward<'n, B: ?Sized + AsRef<[u8]>>(self: &Self, needle: &'n B) -> Finder<'n>` — [`Finder`](#finder)

- `fn build_forward_with_ranker<'n, R: HeuristicFrequencyRank, B: ?Sized + AsRef<[u8]>>(self: &Self, ranker: R, needle: &'n B) -> Finder<'n>` — [`Finder`](#finder)

- `fn build_reverse<'n, B: ?Sized + AsRef<[u8]>>(self: &Self, needle: &'n B) -> FinderRev<'n>` — [`FinderRev`](#finderrev)

- `fn prefilter(self: &mut Self, prefilter: Prefilter) -> &mut FinderBuilder` — [`PrefilterConfig`](searcher/index.md), [`FinderBuilder`](#finderbuilder)

#### Trait Implementations

##### `impl Clone for FinderBuilder`

- `fn clone(self: &Self) -> FinderBuilder` — [`FinderBuilder`](#finderbuilder)

##### `impl Debug for FinderBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for FinderBuilder`

- `fn default() -> FinderBuilder` — [`FinderBuilder`](#finderbuilder)

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

#### Implementations

- `fn is_none(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for PrefilterConfig`

- `fn clone(self: &Self) -> PrefilterConfig` — [`PrefilterConfig`](searcher/index.md)

##### `impl Copy for PrefilterConfig`

##### `impl Debug for PrefilterConfig`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for PrefilterConfig`

- `fn default() -> PrefilterConfig` — [`PrefilterConfig`](searcher/index.md)

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

```rust
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

```rust
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
small haystacks, it may be faster to initialize a [`Finder`](../arch/all/twoway/index.md) once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```rust
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
small haystacks, it may be faster to initialize a [`FinderRev`](../arch/all/rabinkarp/index.md) once,
and reuse it for each search.

# Complexity

This routine is guaranteed to have worst case linear time complexity
with respect to both the needle and the haystack. That is, this runs
in `O(needle.len() + haystack.len())` time.

This routine is also guaranteed to have worst case constant space
complexity.

# Examples

Basic usage:

```rust
use memchr::memmem;

let haystack = b"foo bar baz";
assert_eq!(Some(0), memmem::rfind(haystack, b"foo"));
assert_eq!(Some(4), memmem::rfind(haystack, b"bar"));
assert_eq!(Some(8), memmem::rfind(haystack, b"ba"));
assert_eq!(None, memmem::rfind(haystack, b"quux"));
```

