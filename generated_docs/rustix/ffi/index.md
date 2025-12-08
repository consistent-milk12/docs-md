*[rustix](../index.md) / [ffi](index.md)*

---

# Module `ffi`

Utilities related to FFI bindings.

## Structs

### `c_void`

```rust
struct c_void {
    kind: crate::packed::api::MatchKind,
    by_id: alloc::vec::Vec<alloc::vec::Vec<u8>>,
    order: alloc::vec::Vec<crate::PatternID>,
    minimum_len: usize,
    total_pattern_bytes: usize,
}
```

*Re-exported from `aho_corasick`*

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

- `fn new() -> Patterns` — [`c_void`](#c-void)

- `fn add(self: &mut Self, bytes: &[u8])`

- `fn set_match_kind(self: &mut Self, kind: MatchKind)` — [`tcdrain`](../backend/termios/syscalls/index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn reset(self: &mut Self)`

- `fn minimum_len(self: &Self) -> usize`

- `fn match_kind(self: &Self) -> &MatchKind` — [`tcdrain`](../backend/termios/syscalls/index.md)

- `fn get(self: &Self, id: PatternID) -> Pattern<'_>`

- `unsafe fn get_unchecked(self: &Self, id: PatternID) -> Pattern<'_>`

- `fn iter(self: &Self) -> PatternIter<'_>`

#### Trait Implementations

##### `impl Clone for Patterns`

- `fn clone(self: &Self) -> Patterns` — [`c_void`](#c-void)

##### `impl Debug for Patterns`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result` — [`ArgReg`](../backend/reg/index.md), [`A0`](../backend/reg/index.md)

## Functions

