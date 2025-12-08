*[aho_corasick](../../index.md) / [packed](../index.md) / [pattern](index.md)*

---

# Module `pattern`

## Structs

### `Patterns`

```rust
struct Patterns {
    kind: crate::packed::api::MatchKind,
    by_id: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    order: alloc::vec::Vec<crate::PatternID>,
    minimum_len: usize,
    total_pattern_bytes: usize,
}
```

A non-empty collection of non-empty patterns to search for.

This collection of patterns is what is passed around to both execute
searches and to construct the searchers themselves. Namely, this permits
searches to avoid copying all of the patterns, and allows us to keep only
one copy throughout all packed searchers.

Note that this collection is not a set. The same pattern can appear more
than once.

#### Fields

- **`kind`**: `crate::packed::api::MatchKind`

  The match semantics supported by this collection of patterns.
  
  The match semantics determines the order of the iterator over patterns.
  For leftmost-first, patterns are provided in the same order as were
  provided by the caller. For leftmost-longest, patterns are provided in
  descending order of length, with ties broken by the order in which they
  were provided by the caller.

- **`by_id`**: `alloc::vec::Vec<alloc::vec::Vec<u8>>`

  The collection of patterns, indexed by their identifier.

- **`order`**: `alloc::vec::Vec<crate::PatternID>`

  The order of patterns defined for iteration, given by pattern
  identifiers. The order of `by_id` and `order` is always the same for
  leftmost-first semantics, but may be different for leftmost-longest
  semantics.

- **`minimum_len`**: `usize`

  The length of the smallest pattern, in bytes.

- **`total_pattern_bytes`**: `usize`

  The total number of pattern bytes across the entire collection. This
  is used for reporting total heap usage in constant time.

#### Implementations

- `fn new() -> Patterns` — [`Patterns`](#patterns)

- `fn add(self: &mut Self, bytes: &[u8])`

- `fn set_match_kind(self: &mut Self, kind: MatchKind)` — [`MatchKind`](../api/index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn reset(self: &mut Self)`

- `fn minimum_len(self: &Self) -> usize`

- `fn match_kind(self: &Self) -> &MatchKind` — [`MatchKind`](../api/index.md)

- `fn get(self: &Self, id: PatternID) -> Pattern<'_>` — [`PatternID`](../../util/primitives/index.md), [`Pattern`](#pattern)

- `unsafe fn get_unchecked(self: &Self, id: PatternID) -> Pattern<'_>` — [`PatternID`](../../util/primitives/index.md), [`Pattern`](#pattern)

- `fn iter(self: &Self) -> PatternIter<'_>` — [`PatternIter`](#patterniter)

#### Trait Implementations

##### `impl Clone for Patterns`

- `fn clone(self: &Self) -> Patterns` — [`Patterns`](#patterns)

##### `impl Debug for Patterns`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PatternIter<'p>`

```rust
struct PatternIter<'p> {
    patterns: &'p Patterns,
    i: usize,
}
```

An iterator over the patterns in the `Patterns` collection.

The order of the patterns provided by this iterator is consistent with the
match semantics of the originating collection of patterns.

The lifetime `'p` corresponds to the lifetime of the collection of patterns
this is iterating over.

#### Trait Implementations

##### `impl<'p> Debug for PatternIter<'p>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PatternIter<'p>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'p> Iterator for PatternIter<'p>`

- `type Item = (PatternID, Pattern<'p>)`

- `fn next(self: &mut Self) -> Option<(PatternID, Pattern<'p>)>` — [`PatternID`](../../util/primitives/index.md), [`Pattern`](#pattern)

### `Pattern<'a>`

```rust
struct Pattern<'a>(&'a [u8]);
```

A pattern that is used in packed searching.

#### Implementations

- `fn len(self: &Self) -> usize`

- `fn bytes(self: &Self) -> &[u8]`

- `fn low_nybbles(self: &Self, len: usize) -> Box<[u8]>`

- `fn is_prefix(self: &Self, bytes: &[u8]) -> bool`

- `unsafe fn is_prefix_raw(self: &Self, start: *const u8, end: *const u8) -> bool`

#### Trait Implementations

##### `impl<'a> Clone for Pattern<'a>`

- `fn clone(self: &Self) -> Pattern<'a>` — [`Pattern`](#pattern)

##### `impl<'a> Debug for Pattern<'a>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `is_prefix`

```rust
fn is_prefix(haystack: &[u8], needle: &[u8]) -> bool
```

Returns true if and only if `needle` is a prefix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

### `is_equal_raw`

```rust
unsafe fn is_equal_raw(x: *const u8, y: *const u8, n: usize) -> bool
```

Compare `n` bytes at the given pointers for equality.

This returns true if and only if `*x.add(i) == *y.add(i)` for all
`0 <= i < n`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

# Safety

* Both `x` and `y` must be valid for reads of up to `n` bytes.
* Both `x` and `y` must point to an initialized value.
* Both `x` and `y` must each point to an allocated object and
must either be in bounds or at most one byte past the end of the
allocated object. `x` and `y` do not need to point to the same allocated
object, but they may.
* Both `x` and `y` must be _derived from_ a pointer to their respective
allocated objects.
* The distance between `x` and `x+n` must not overflow `isize`. Similarly
for `y` and `y+n`.
* The distance being in bounds must not rely on "wrapping around" the
address space.

