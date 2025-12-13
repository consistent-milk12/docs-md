*[aho_corasick](../../index.md) / [packed](../index.md) / [pattern](index.md)*

---

# Module `pattern`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Patterns`](#patterns) | struct | A non-empty collection of non-empty patterns to search for. |
| [`PatternIter`](#patterniter) | struct | An iterator over the patterns in the `Patterns` collection. |
| [`Pattern`](#pattern) | struct | A pattern that is used in packed searching. |
| [`is_prefix`](#is-prefix) | fn | Returns true if and only if `needle` is a prefix of `haystack`. |
| [`is_equal_raw`](#is-equal-raw) | fn | Compare `n` bytes at the given pointers for equality. |

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

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:20-41`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/pattern.rs#L20-L41)*

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

  Create a new collection of patterns for the given match semantics. The

  ID of each pattern is the index of the pattern at which it occurs in

  the `by_id` slice.

  

  If any of the patterns in the slice given are empty, then this panics.

  Similarly, if the number of patterns given is zero, then this also

  panics.

- <span id="patterns-add"></span>`fn add(&mut self, bytes: &[u8])`

  Add a pattern to this collection.

  

  This panics if the pattern given is empty.

- <span id="patterns-set-match-kind"></span>`fn set_match_kind(&mut self, kind: MatchKind)` — [`MatchKind`](../api/index.md#matchkind)

  Set the match kind semantics for this collection of patterns.

  

  If the kind is not set, then the default is leftmost-first.

- <span id="patterns-len"></span>`fn len(&self) -> usize`

  Return the number of patterns in this collection.

  

  This is guaranteed to be greater than zero.

- <span id="patterns-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this collection of patterns is empty.

- <span id="patterns-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by these patterns, in

  units of bytes.

- <span id="patterns-reset"></span>`fn reset(&mut self)`

  Clears all heap memory associated with this collection of patterns and

  resets all state such that it is a valid empty collection.

- <span id="patterns-minimum-len"></span>`fn minimum_len(&self) -> usize`

  Returns the length, in bytes, of the smallest pattern.

  

  This is guaranteed to be at least one.

- <span id="patterns-match-kind"></span>`fn match_kind(&self) -> &MatchKind` — [`MatchKind`](../api/index.md#matchkind)

  Returns the match semantics used by these patterns.

- <span id="patterns-get"></span>`fn get(&self, id: PatternID) -> Pattern<'_>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Pattern`](#pattern)

  Return the pattern with the given identifier. If such a pattern does

  not exist, then this panics.

- <span id="patterns-get-unchecked"></span>`unsafe fn get_unchecked(&self, id: PatternID) -> Pattern<'_>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Pattern`](#pattern)

  Return the pattern with the given identifier without performing bounds

  checks.

  

  # Safety

  

  Callers must ensure that a pattern with the given identifier exists

  before using this method.

- <span id="patterns-iter"></span>`fn iter(&self) -> PatternIter<'_>` — [`PatternIter`](#patterniter)

  Return an iterator over all the patterns in this collection, in the

  order in which they should be matched.

  

  Specifically, in a naive multi-pattern matcher, the following is

  guaranteed to satisfy the match semantics of this collection of

  patterns:

  

  ```ignore

  for i in 0..haystack.len():

    for p in patterns.iter():

      if haystack[i..].starts_with(p.bytes()):

        return Match(p.id(), i, i + p.bytes().len())

  ```

  

  Namely, among the patterns in a collection, if they are matched in

  the order provided by this iterator, then the result is guaranteed

  to satisfy the correct match semantics. (Either leftmost-first or

  leftmost-longest.)

#### Trait Implementations

##### `impl Any for Patterns`

- <span id="patterns-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Patterns`

- <span id="patterns-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Patterns`

- <span id="patterns-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Patterns`

- <span id="patterns-clone"></span>`fn clone(&self) -> Patterns` — [`Patterns`](#patterns)

##### `impl CloneToUninit for Patterns`

- <span id="patterns-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Patterns`

- <span id="patterns-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Patterns`

- <span id="patterns-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Patterns`

- <span id="patterns-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Patterns`

- <span id="patterns-toowned-type-owned"></span>`type Owned = T`

- <span id="patterns-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patterns-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Patterns`

- <span id="patterns-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterns-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Patterns`

- <span id="patterns-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterns-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternIter<'p>`

```rust
struct PatternIter<'p> {
    patterns: &'p Patterns,
    i: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:188-191`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/pattern.rs#L188-L191)*

An iterator over the patterns in the `Patterns` collection.

The order of the patterns provided by this iterator is consistent with the
match semantics of the originating collection of patterns.

The lifetime `'p` corresponds to the lifetime of the collection of patterns
this is iterating over.

#### Trait Implementations

##### `impl Any for PatternIter<'p>`

- <span id="patterniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternIter<'p>`

- <span id="patterniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternIter<'p>`

- <span id="patterniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PatternIter<'p>`

- <span id="patterniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PatternIter<'p>`

- <span id="patterniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternIter<'p>`

- <span id="patterniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PatternIter<'p>`

- <span id="patterniter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIter<'p>`

- <span id="patterniter-iterator-type-item"></span>`type Item = (PatternID, Pattern<'p>)`

- <span id="patterniter-iterator-next"></span>`fn next(&mut self) -> Option<(PatternID, Pattern<'p>)>` — [`PatternID`](../../util/primitives/index.md#patternid), [`Pattern`](#pattern)

##### `impl<U> TryFrom for PatternIter<'p>`

- <span id="patterniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternIter<'p>`

- <span id="patterniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Pattern<'a>`

```rust
struct Pattern<'a>(&'a [u8]);
```

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:209`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/pattern.rs#L209)*

A pattern that is used in packed searching.

#### Implementations

- <span id="pattern-len"></span>`fn len(&self) -> usize`

  Returns the length of this pattern, in bytes.

- <span id="pattern-bytes"></span>`fn bytes(&self) -> &[u8]`

  Returns the bytes of this pattern.

- <span id="pattern-low-nybbles"></span>`fn low_nybbles(&self, len: usize) -> Box<[u8]>`

  Returns the first `len` low nybbles from this pattern. If this pattern

  is shorter than `len`, then this panics.

- <span id="pattern-is-prefix"></span>`fn is_prefix(&self, bytes: &[u8]) -> bool`

  Returns true if this pattern is a prefix of the given bytes.

- <span id="pattern-is-prefix-raw"></span>`unsafe fn is_prefix_raw(&self, start: *const u8, end: *const u8) -> bool`

  Returns true if this pattern is a prefix of the haystack given by the

  raw `start` and `end` pointers.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

#### Trait Implementations

##### `impl Any for Pattern<'a>`

- <span id="pattern-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Pattern<'a>`

- <span id="pattern-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Pattern<'a>`

- <span id="pattern-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Pattern<'a>`

- <span id="pattern-clone"></span>`fn clone(&self) -> Pattern<'a>` — [`Pattern`](#pattern)

##### `impl CloneToUninit for Pattern<'a>`

- <span id="pattern-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Pattern<'a>`

- <span id="pattern-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Pattern<'a>`

- <span id="pattern-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Pattern<'a>`

- <span id="pattern-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Pattern<'a>`

- <span id="pattern-toowned-type-owned"></span>`type Owned = T`

- <span id="pattern-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="pattern-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Pattern<'a>`

- <span id="pattern-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="pattern-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Pattern<'a>`

- <span id="pattern-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="pattern-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `is_prefix`

```rust
fn is_prefix(haystack: &[u8], needle: &[u8]) -> bool
```

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:293-301`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/pattern.rs#L293-L301)*

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

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:368-416`](../../../../.source_1765521767/aho-corasick-1.1.4/src/packed/pattern.rs#L368-L416)*

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

