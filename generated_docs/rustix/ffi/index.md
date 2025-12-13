*[rustix](../index.md) / [ffi](index.md)*

---

# Module `ffi`

Utilities related to FFI bindings.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`c_void`](#c-void) | struct |  |
| [`c_uint`](#c-uint) | fn |  |

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

*Defined in [`aho-corasick-1.1.4/src/packed/pattern.rs:20-41`](../../../.source_1765633015/aho-corasick-1.1.4/src/packed/pattern.rs#L20-L41)*

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

- <span id="patterns-new"></span>`fn new() -> Patterns` — [`c_void`](#c-void)

  Create a new collection of patterns for the given match semantics. The

  ID of each pattern is the index of the pattern at which it occurs in

  the `by_id` slice.

  

  If any of the patterns in the slice given are empty, then this panics.

  Similarly, if the number of patterns given is zero, then this also

  panics.

- <span id="patterns-add"></span>`fn add(&mut self, bytes: &[u8])`

  Add a pattern to this collection.

  

  This panics if the pattern given is empty.

- <span id="patterns-set-match-kind"></span>`fn set_match_kind(&mut self, kind: MatchKind)` — [`tcdrain`](../backend/termios/syscalls/index.md#tcdrain)

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

- <span id="patterns-match-kind"></span>`fn match_kind(&self) -> &MatchKind` — [`tcdrain`](../backend/termios/syscalls/index.md#tcdrain)

  Returns the match semantics used by these patterns.

- <span id="patterns-get"></span>`fn get(&self, id: PatternID) -> Pattern<'_>`

  Return the pattern with the given identifier. If such a pattern does

  not exist, then this panics.

- <span id="patterns-get-unchecked"></span>`unsafe fn get_unchecked(&self, id: PatternID) -> Pattern<'_>`

  Return the pattern with the given identifier without performing bounds

  checks.

  

  # Safety

  

  Callers must ensure that a pattern with the given identifier exists

  before using this method.

- <span id="patterns-iter"></span>`fn iter(&self) -> PatternIter<'_>`

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

- <span id="patterns-any-type-id"></span>`fn type_id(&self) -> TypeId` — [`os`](../maybe_polyfill/os/index.md#os)

##### `impl<T> Borrow for Patterns`

- <span id="patterns-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Patterns`

- <span id="patterns-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Patterns`

- <span id="patterns-clone"></span>`fn clone(&self) -> Patterns` — [`c_void`](#c-void)

##### `impl CloneToUninit for Patterns`

- <span id="patterns-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Patterns`

- <span id="patterns-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result` — [`ArgReg`](../backend/reg/index.md#argreg), [`A0`](../backend/reg/index.md#a0)

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

- <span id="patterns-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>` — [`unnamed`](../fd/index.md#unnamed)

##### `impl<U> TryInto for Patterns`

- <span id="patterns-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterns-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>` — [`unnamed`](../fd/index.md#unnamed)

## Functions

### `c_uint`

```rust
fn c_uint(sections: &gimli::Dwarf<R>) -> Result<Self, gimli::Error>
```

*Defined in [`addr2line-0.25.1/src/unit.rs:504-521`](../../../.source_1765633015/addr2line-0.25.1/src/unit.rs#L504-L521)*

