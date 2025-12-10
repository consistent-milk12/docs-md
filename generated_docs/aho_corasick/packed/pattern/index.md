*[aho_corasick](../../index.md) / [packed](../index.md) / [pattern](index.md)*

---

# Module `pattern`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Patterns`](#patterns) | struct | A non-empty collection of non-empty patterns to search for. |
| [`PatternIter`](#patterniter) | struct | An iterator over the patterns in the `Patterns` collection. |
| [`Pattern`](#pattern) | struct | A pattern that is used in packed searching. |
| [`is_prefix`](#is_prefix) | fn | Returns true if and only if `needle` is a prefix of `haystack`. |
| [`is_equal_raw`](#is_equal_raw) | fn | Compare `n` bytes at the given pointers for equality. |

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

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:20-41`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/pattern.rs#L20-L41)*

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

- <span id="patterns-new"></span>`fn new() -> Patterns` — [`Patterns`](#patterns)

- <span id="patterns-add"></span>`fn add(&mut self, bytes: &[u8])`

- <span id="patterns-set-match-kind"></span>`fn set_match_kind(&mut self, kind: MatchKind)` — [`MatchKind`](../api/index.md#matchkind)

- <span id="patterns-len"></span>`fn len(&self) -> usize`

- <span id="patterns-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="patterns-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="patterns-reset"></span>`fn reset(&mut self)`

- <span id="patterns-minimum-len"></span>`fn minimum_len(&self) -> usize`

- <span id="patterns-match-kind"></span>`fn match_kind(&self) -> &MatchKind` — [`MatchKind`](../api/index.md#matchkind)

- <span id="patterns-get"></span>`fn get(&self, id: PatternID) -> Pattern<'_>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Pattern`](#pattern)

- <span id="patterns-get-unchecked"></span>`unsafe fn get_unchecked(&self, id: PatternID) -> Pattern<'_>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Pattern`](#pattern)

- <span id="patterns-iter"></span>`fn iter(&self) -> PatternIter<'_>` — [`PatternIter`](#patterniter)

#### Trait Implementations

##### `impl Clone for Patterns`

- <span id="patterns-clone"></span>`fn clone(&self) -> Patterns` — [`Patterns`](#patterns)

##### `impl Debug for Patterns`

- <span id="patterns-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `PatternIter<'p>`

```rust
struct PatternIter<'p> {
    patterns: &'p Patterns,
    i: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:188-191`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/pattern.rs#L188-L191)*

An iterator over the patterns in the `Patterns` collection.

The order of the patterns provided by this iterator is consistent with the
match semantics of the originating collection of patterns.

The lifetime `'p` corresponds to the lifetime of the collection of patterns
this is iterating over.

#### Trait Implementations

##### `impl Debug for PatternIter<'p>`

- <span id="patterniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for PatternIter<'p>`

- <span id="patterniter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniter-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIter<'p>`

- <span id="patterniter-type-item"></span>`type Item = (PatternID, Pattern<'p>)`

- <span id="patterniter-next"></span>`fn next(&mut self) -> Option<(PatternID, Pattern<'p>)>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Pattern`](#pattern)

### `Pattern<'a>`

```rust
struct Pattern<'a>(&'a [u8]);
```

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:209`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/pattern.rs#L209)*

A pattern that is used in packed searching.

#### Implementations

- <span id="pattern-len"></span>`fn len(&self) -> usize`

- <span id="pattern-bytes"></span>`fn bytes(&self) -> &[u8]`

- <span id="pattern-low-nybbles"></span>`fn low_nybbles(&self, len: usize) -> Box<[u8]>`

- <span id="pattern-is-prefix"></span>`fn is_prefix(&self, bytes: &[u8]) -> bool`

- <span id="pattern-is-prefix-raw"></span>`unsafe fn is_prefix_raw(&self, start: *const u8, end: *const u8) -> bool`

#### Trait Implementations

##### `impl Clone for Pattern<'a>`

- <span id="pattern-clone"></span>`fn clone(&self) -> Pattern<'a>` — [`Pattern`](#pattern)

##### `impl Debug for Pattern<'a>`

- <span id="pattern-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `is_prefix`

```rust
fn is_prefix(haystack: &[u8], needle: &[u8]) -> bool
```

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:293-301`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/pattern.rs#L293-L301)*

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

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:368-416`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/pattern.rs#L368-L416)*

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

