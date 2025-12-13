*[aho_corasick](../../index.md) / [util](../index.md) / [prefilter](index.md)*

---

# Module `prefilter`

## Contents

- [Structs](#structs)
  - [`Prefilter`](#prefilter)
  - [`Builder`](#builder)
  - [`Packed`](#packed)
  - [`MemmemBuilder`](#memmembuilder)
  - [`Memmem`](#memmem)
  - [`RareBytesBuilder`](#rarebytesbuilder)
  - [`RareByteOffsets`](#rarebyteoffsets)
  - [`RareByteOffset`](#rarebyteoffset)
  - [`RareBytesOne`](#rarebytesone)
  - [`RareBytesTwo`](#rarebytestwo)
  - [`RareBytesThree`](#rarebytesthree)
  - [`StartBytesBuilder`](#startbytesbuilder)
  - [`StartBytesOne`](#startbytesone)
  - [`StartBytesTwo`](#startbytestwo)
  - [`StartBytesThree`](#startbytesthree)
- [Enums](#enums)
  - [`Candidate`](#candidate)
- [Traits](#traits)
  - [`PrefilterI`](#prefilteri)
- [Functions](#functions)
  - [`opposite_ascii_case`](#opposite-ascii-case)
  - [`freq_rank`](#freq-rank)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Prefilter`](#prefilter) | struct | A prefilter for accelerating a search. |
| [`Builder`](#builder) | struct | A builder for constructing the best possible prefilter. |
| [`Packed`](#packed) | struct | A type that wraps a packed searcher and implements the `Prefilter` interface. |
| [`MemmemBuilder`](#memmembuilder) | struct | A builder for constructing a prefilter that uses memmem. |
| [`Memmem`](#memmem) | struct | A type that wraps a SIMD accelerated single substring search from the `memchr` crate for use as a prefilter. |
| [`RareBytesBuilder`](#rarebytesbuilder) | struct | A builder for constructing a rare byte prefilter. |
| [`RareByteOffsets`](#rarebyteoffsets) | struct | A set of byte offsets, keyed by byte. |
| [`RareByteOffset`](#rarebyteoffset) | struct | Offsets associated with an occurrence of a "rare" byte in any of the patterns used to construct a single Aho-Corasick automaton. |
| [`RareBytesOne`](#rarebytesone) | struct | A prefilter for scanning for a single "rare" byte. |
| [`RareBytesTwo`](#rarebytestwo) | struct | A prefilter for scanning for two "rare" bytes. |
| [`RareBytesThree`](#rarebytesthree) | struct | A prefilter for scanning for three "rare" bytes. |
| [`StartBytesBuilder`](#startbytesbuilder) | struct | A builder for constructing a starting byte prefilter. |
| [`StartBytesOne`](#startbytesone) | struct | A prefilter for scanning for a single starting byte. |
| [`StartBytesTwo`](#startbytestwo) | struct | A prefilter for scanning for two starting bytes. |
| [`StartBytesThree`](#startbytesthree) | struct | A prefilter for scanning for three starting bytes. |
| [`Candidate`](#candidate) | enum | A candidate is the result of running a prefilter on a haystack at a particular position. |
| [`PrefilterI`](#prefilteri) | trait | A prefilter describes the behavior of fast literal scanners for quickly skipping past bytes in the haystack that we know cannot possibly participate in a match. |
| [`opposite_ascii_case`](#opposite-ascii-case) | fn | If the given byte is an ASCII letter, then return it in the opposite case. |
| [`freq_rank`](#freq-rank) | fn | Return the frequency rank of the given byte. |

## Structs

### `Prefilter`

```rust
struct Prefilter {
    finder: alloc::sync::Arc<dyn PrefilterI>,
    memory_usage: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:33-36`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L33-L36)*

A prefilter for accelerating a search.

This crate uses prefilters in the core search implementations to accelerate
common cases. They typically only apply to cases where there are a small
number of patterns (less than 100 or so), but when they do, thoughput can
be boosted considerably, perhaps by an order of magnitude. When a prefilter
is active, it is used whenever a search enters an automaton's start state.

Currently, prefilters cannot be constructed by
callers. A `Prefilter` can only be accessed via the
[`Automaton::prefilter`](crate::automaton::Automaton::prefilter)
method and used to execute a search. In other words, a prefilter can be
used to optimize your own search implementation if necessary, but cannot do
much else. If you have a use case for more APIs, please submit an issue.

#### Implementations

- <span id="prefilter-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

  Execute a search in the haystack within the span given. If a match or

  a possible match is returned, then it is guaranteed to occur within

  the bounds of the span.

  

  If the span provided is invalid for the given haystack, then behavior

  is unspecified.

- <span id="prefilter-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for Prefilter`

- <span id="prefilter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Prefilter`

- <span id="prefilter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Prefilter`

- <span id="prefilter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Prefilter`

- <span id="prefilter-clone"></span>`fn clone(&self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl CloneToUninit for Prefilter`

- <span id="prefilter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Prefilter`

- <span id="prefilter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Prefilter`

- <span id="prefilter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Prefilter`

- <span id="prefilter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Prefilter`

- <span id="prefilter-toowned-type-owned"></span>`type Owned = T`

- <span id="prefilter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="prefilter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Prefilter`

- <span id="prefilter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="prefilter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Prefilter`

- <span id="prefilter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="prefilter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Builder`

```rust
struct Builder {
    count: usize,
    ascii_case_insensitive: bool,
    start_bytes: StartBytesBuilder,
    rare_bytes: RareBytesBuilder,
    memmem: MemmemBuilder,
    packed: Option<packed::Builder>,
    enabled: bool,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:121-131`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L121-L131)*

A builder for constructing the best possible prefilter. When constructed,
this builder will heuristically select the best prefilter it can build,
if any, and discard the rest.

#### Implementations

- <span id="builder-new"></span>`fn new(kind: MatchKind) -> Builder` — [`MatchKind`](../search/index.md#matchkind), [`Builder`](#builder)

  Create a new builder for constructing the best possible prefilter.

- <span id="builder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(self, yes: bool) -> Builder` — [`Builder`](#builder)

  Enable ASCII case insensitivity. When set, byte strings added to this

  builder will be interpreted without respect to ASCII case.

- <span id="builder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

  Return a prefilter suitable for quickly finding potential matches.

  

  All patterns added to an Aho-Corasick automaton should be added to this

  builder before attempting to construct the prefilter.

- <span id="builder-add"></span>`fn add(&mut self, bytes: &[u8])`

  Add a literal string to this prefilter builder.

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Builder`

- <span id="builder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Builder`

- <span id="builder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Builder`

- <span id="builder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Builder`

- <span id="builder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Packed`

```rust
struct Packed(packed::Searcher);
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:328`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L328)*

A type that wraps a packed searcher and implements the `Prefilter`
interface.

#### Trait Implementations

##### `impl Any for Packed`

- <span id="packed-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Packed`

- <span id="packed-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Packed`

- <span id="packed-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Packed`

- <span id="packed-clone"></span>`fn clone(&self) -> Packed` — [`Packed`](#packed)

##### `impl CloneToUninit for Packed`

- <span id="packed-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Packed`

- <span id="packed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Packed`

- <span id="packed-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Packed`

- <span id="packed-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Packed`

- <span id="packed-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for Packed`

- <span id="packed-toowned-type-owned"></span>`type Owned = T`

- <span id="packed-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="packed-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Packed`

- <span id="packed-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="packed-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Packed`

- <span id="packed-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="packed-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MemmemBuilder`

```rust
struct MemmemBuilder {
    count: usize,
    one: Option<alloc::vec::Vec<u8>>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:340-345`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L340-L345)*

A builder for constructing a prefilter that uses memmem.

#### Fields

- **`count`**: `usize`

  The number of patterns that have been added.

- **`one`**: `Option<alloc::vec::Vec<u8>>`

  The singular pattern to search for. This is only set when count==1.

#### Implementations

- <span id="memmembuilder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- <span id="memmembuilder-add"></span>`fn add(&mut self, bytes: &[u8])`

#### Trait Implementations

##### `impl Any for MemmemBuilder`

- <span id="memmembuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MemmemBuilder`

- <span id="memmembuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MemmemBuilder`

- <span id="memmembuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for MemmemBuilder`

- <span id="memmembuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MemmemBuilder`

- <span id="memmembuilder-default"></span>`fn default() -> MemmemBuilder` — [`MemmemBuilder`](#memmembuilder)

##### `impl<T> From for MemmemBuilder`

- <span id="memmembuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MemmemBuilder`

- <span id="memmembuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for MemmemBuilder`

- <span id="memmembuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memmembuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MemmemBuilder`

- <span id="memmembuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memmembuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Memmem`

```rust
struct Memmem(memchr::memmem::Finder<'static>);
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:394`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L394)*

A type that wraps a SIMD accelerated single substring search from the
`memchr` crate for use as a prefilter.

Currently, this prefilter is only active for Aho-Corasick searchers with
a single pattern. In theory, this could be extended to support searchers
that have a common prefix of more than one byte (for one byte, we would use
memchr), but it's not clear if it's worth it or not.

Also, unfortunately, this currently also requires the 'std' feature to
be enabled. That's because memchr doesn't have a no-std-but-with-alloc
mode, and so APIs like Finder::into_owned aren't available when 'std' is
disabled. But there should be an 'alloc' feature that brings in APIs like
Finder::into_owned but doesn't use std-only features like runtime CPU
feature detection.

#### Trait Implementations

##### `impl Any for Memmem`

- <span id="memmem-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Memmem`

- <span id="memmem-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Memmem`

- <span id="memmem-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Memmem`

- <span id="memmem-clone"></span>`fn clone(&self) -> Memmem` — [`Memmem`](#memmem)

##### `impl CloneToUninit for Memmem`

- <span id="memmem-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Memmem`

- <span id="memmem-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Memmem`

- <span id="memmem-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Memmem`

- <span id="memmem-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for Memmem`

- <span id="memmem-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for Memmem`

- <span id="memmem-toowned-type-owned"></span>`type Owned = T`

- <span id="memmem-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="memmem-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Memmem`

- <span id="memmem-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="memmem-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Memmem`

- <span id="memmem-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="memmem-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RareBytesBuilder`

```rust
struct RareBytesBuilder {
    ascii_case_insensitive: bool,
    rare_set: crate::util::alphabet::ByteSet,
    byte_offsets: RareByteOffsets,
    available: bool,
    count: usize,
    rank_sum: u16,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:419-441`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L419-L441)*

A builder for constructing a rare byte prefilter.

A rare byte prefilter attempts to pick out a small set of rare bytes that
occurr in the patterns, and then quickly scan to matches of those rare
bytes.

#### Fields

- **`ascii_case_insensitive`**: `bool`

  Whether this prefilter should account for ASCII case insensitivity or
  not.

- **`rare_set`**: `crate::util::alphabet::ByteSet`

  A set of rare bytes, indexed by byte value.

- **`byte_offsets`**: `RareByteOffsets`

  A set of byte offsets associated with bytes in a pattern. An entry
  corresponds to a particular bytes (its index) and is only non-zero if
  the byte occurred at an offset greater than 0 in at least one pattern.
  
  If a byte's offset is not representable in 8 bits, then the rare bytes
  prefilter becomes inert.

- **`available`**: `bool`

  Whether this is available as a prefilter or not. This can be set to
  false during construction if a condition is seen that invalidates the
  use of the rare-byte prefilter.

- **`count`**: `usize`

  The number of bytes set to an active value in `byte_offsets`.

- **`rank_sum`**: `u16`

  The sum of frequency ranks for the rare bytes detected. This is
  intended to give a heuristic notion of how rare the bytes are.

#### Implementations

- <span id="rarebytesbuilder-new"></span>`fn new() -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

  Create a new builder for constructing a rare byte prefilter.

- <span id="rarebytesbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(self, yes: bool) -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

  Enable ASCII case insensitivity. When set, byte strings added to this

  builder will be interpreted without respect to ASCII case.

- <span id="rarebytesbuilder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

  Build the rare bytes prefilter.

  

  If there are more than 3 distinct rare bytes found, or if heuristics

  otherwise determine that this prefilter should not be used, then `None`

  is returned.

- <span id="rarebytesbuilder-add"></span>`fn add(&mut self, bytes: &[u8])`

  Add a byte string to this builder.

  

  All patterns added to an Aho-Corasick automaton should be added to this

  builder before attempting to construct the prefilter.

- <span id="rarebytesbuilder-set-offset"></span>`fn set_offset(&mut self, pos: usize, byte: u8)`

- <span id="rarebytesbuilder-add-rare-byte"></span>`fn add_rare_byte(&mut self, byte: u8)`

- <span id="rarebytesbuilder-add-one-rare-byte"></span>`fn add_one_rare_byte(&mut self, byte: u8)`

#### Trait Implementations

##### `impl Any for RareBytesBuilder`

- <span id="rarebytesbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RareBytesBuilder`

- <span id="rarebytesbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RareBytesBuilder`

- <span id="rarebytesbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RareBytesBuilder`

- <span id="rarebytesbuilder-clone"></span>`fn clone(&self) -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

##### `impl CloneToUninit for RareBytesBuilder`

- <span id="rarebytesbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RareBytesBuilder`

- <span id="rarebytesbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RareBytesBuilder`

- <span id="rarebytesbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RareBytesBuilder`

- <span id="rarebytesbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RareBytesBuilder`

- <span id="rarebytesbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="rarebytesbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarebytesbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RareBytesBuilder`

- <span id="rarebytesbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarebytesbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RareBytesBuilder`

- <span id="rarebytesbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarebytesbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RareByteOffsets`

```rust
struct RareByteOffsets {
    set: [RareByteOffset; 256],
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:445-449`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L445-L449)*

A set of byte offsets, keyed by byte.

#### Fields

- **`set`**: `[RareByteOffset; 256]`

  Each entry corresponds to the maximum offset of the corresponding
  byte across all patterns seen.

#### Implementations

- <span id="rarebyteoffsets-empty"></span>`fn empty() -> RareByteOffsets` — [`RareByteOffsets`](#rarebyteoffsets)

  Create a new empty set of rare byte offsets.

- <span id="rarebyteoffsets-set"></span>`fn set(&mut self, byte: u8, off: RareByteOffset)` — [`RareByteOffset`](#rarebyteoffset)

  Add the given offset for the given byte to this set. If the offset is

  greater than the existing offset, then it overwrites the previous

  value and returns false. If there is no previous value set, then this

  sets it and returns true.

#### Trait Implementations

##### `impl Any for RareByteOffsets`

- <span id="rarebyteoffsets-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RareByteOffsets`

- <span id="rarebyteoffsets-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RareByteOffsets`

- <span id="rarebyteoffsets-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RareByteOffsets`

- <span id="rarebyteoffsets-clone"></span>`fn clone(&self) -> RareByteOffsets` — [`RareByteOffsets`](#rarebyteoffsets)

##### `impl CloneToUninit for RareByteOffsets`

- <span id="rarebyteoffsets-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RareByteOffsets`

##### `impl Debug for RareByteOffsets`

- <span id="rarebyteoffsets-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for RareByteOffsets`

- <span id="rarebyteoffsets-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RareByteOffsets`

- <span id="rarebyteoffsets-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RareByteOffsets`

- <span id="rarebyteoffsets-toowned-type-owned"></span>`type Owned = T`

- <span id="rarebyteoffsets-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarebyteoffsets-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RareByteOffsets`

- <span id="rarebyteoffsets-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarebyteoffsets-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RareByteOffsets`

- <span id="rarebyteoffsets-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarebyteoffsets-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RareByteOffset`

```rust
struct RareByteOffset {
    max: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:482-497`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L482-L497)*

Offsets associated with an occurrence of a "rare" byte in any of the
patterns used to construct a single Aho-Corasick automaton.

#### Fields

- **`max`**: `u8`

  The maximum offset at which a particular byte occurs from the start
  of any pattern. This is used as a shift amount. That is, when an
  occurrence of this byte is found, the candidate position reported by
  the prefilter is `position_of_byte - max`, such that the automaton
  will begin its search at a position that is guaranteed to observe a
  match.
  
  To avoid accidentally quadratic behavior, a prefilter is considered
  ineffective when it is asked to start scanning from a position that it
  has already scanned past.
  
  Using a `u8` here means that if we ever see a pattern that's longer
  than 255 bytes, then the entire rare byte prefilter is disabled.

#### Implementations

- <span id="rarebyteoffset-new"></span>`fn new(max: usize) -> Option<RareByteOffset>` — [`RareByteOffset`](#rarebyteoffset)

  Create a new rare byte offset. If the given offset is too big, then

  None is returned. In that case, callers should render the rare bytes

  prefilter inert.

#### Trait Implementations

##### `impl Any for RareByteOffset`

- <span id="rarebyteoffset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RareByteOffset`

- <span id="rarebyteoffset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RareByteOffset`

- <span id="rarebyteoffset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RareByteOffset`

- <span id="rarebyteoffset-clone"></span>`fn clone(&self) -> RareByteOffset` — [`RareByteOffset`](#rarebyteoffset)

##### `impl CloneToUninit for RareByteOffset`

- <span id="rarebyteoffset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for RareByteOffset`

##### `impl Debug for RareByteOffset`

- <span id="rarebyteoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RareByteOffset`

- <span id="rarebyteoffset-default"></span>`fn default() -> RareByteOffset` — [`RareByteOffset`](#rarebyteoffset)

##### `impl<T> From for RareByteOffset`

- <span id="rarebyteoffset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RareByteOffset`

- <span id="rarebyteoffset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RareByteOffset`

- <span id="rarebyteoffset-toowned-type-owned"></span>`type Owned = T`

- <span id="rarebyteoffset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarebyteoffset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RareByteOffset`

- <span id="rarebyteoffset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarebyteoffset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RareByteOffset`

- <span id="rarebyteoffset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarebyteoffset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RareBytesOne`

```rust
struct RareBytesOne {
    byte1: u8,
    offset: RareByteOffset,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:668-671`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L668-L671)*

A prefilter for scanning for a single "rare" byte.

#### Trait Implementations

##### `impl Any for RareBytesOne`

- <span id="rarebytesone-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RareBytesOne`

- <span id="rarebytesone-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RareBytesOne`

- <span id="rarebytesone-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RareBytesOne`

- <span id="rarebytesone-clone"></span>`fn clone(&self) -> RareBytesOne` — [`RareBytesOne`](#rarebytesone)

##### `impl CloneToUninit for RareBytesOne`

- <span id="rarebytesone-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RareBytesOne`

- <span id="rarebytesone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RareBytesOne`

- <span id="rarebytesone-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RareBytesOne`

- <span id="rarebytesone-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for RareBytesOne`

- <span id="rarebytesone-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for RareBytesOne`

- <span id="rarebytesone-toowned-type-owned"></span>`type Owned = T`

- <span id="rarebytesone-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarebytesone-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RareBytesOne`

- <span id="rarebytesone-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarebytesone-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RareBytesOne`

- <span id="rarebytesone-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarebytesone-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RareBytesTwo`

```rust
struct RareBytesTwo {
    offsets: RareByteOffsets,
    byte1: u8,
    byte2: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:691-695`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L691-L695)*

A prefilter for scanning for two "rare" bytes.

#### Trait Implementations

##### `impl Any for RareBytesTwo`

- <span id="rarebytestwo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RareBytesTwo`

- <span id="rarebytestwo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RareBytesTwo`

- <span id="rarebytestwo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RareBytesTwo`

- <span id="rarebytestwo-clone"></span>`fn clone(&self) -> RareBytesTwo` — [`RareBytesTwo`](#rarebytestwo)

##### `impl CloneToUninit for RareBytesTwo`

- <span id="rarebytestwo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RareBytesTwo`

- <span id="rarebytestwo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RareBytesTwo`

- <span id="rarebytestwo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RareBytesTwo`

- <span id="rarebytestwo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for RareBytesTwo`

- <span id="rarebytestwo-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for RareBytesTwo`

- <span id="rarebytestwo-toowned-type-owned"></span>`type Owned = T`

- <span id="rarebytestwo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarebytestwo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RareBytesTwo`

- <span id="rarebytestwo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarebytestwo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RareBytesTwo`

- <span id="rarebytestwo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarebytestwo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RareBytesThree`

```rust
struct RareBytesThree {
    offsets: RareByteOffsets,
    byte1: u8,
    byte2: u8,
    byte3: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:713-718`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L713-L718)*

A prefilter for scanning for three "rare" bytes.

#### Trait Implementations

##### `impl Any for RareBytesThree`

- <span id="rarebytesthree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RareBytesThree`

- <span id="rarebytesthree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RareBytesThree`

- <span id="rarebytesthree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RareBytesThree`

- <span id="rarebytesthree-clone"></span>`fn clone(&self) -> RareBytesThree` — [`RareBytesThree`](#rarebytesthree)

##### `impl CloneToUninit for RareBytesThree`

- <span id="rarebytesthree-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RareBytesThree`

- <span id="rarebytesthree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RareBytesThree`

- <span id="rarebytesthree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RareBytesThree`

- <span id="rarebytesthree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for RareBytesThree`

- <span id="rarebytesthree-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for RareBytesThree`

- <span id="rarebytesthree-toowned-type-owned"></span>`type Owned = T`

- <span id="rarebytesthree-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="rarebytesthree-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RareBytesThree`

- <span id="rarebytesthree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="rarebytesthree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RareBytesThree`

- <span id="rarebytesthree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="rarebytesthree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StartBytesBuilder`

```rust
struct StartBytesBuilder {
    ascii_case_insensitive: bool,
    byteset: alloc::vec::Vec<bool>,
    count: usize,
    rank_sum: u16,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:746-757`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L746-L757)*

A builder for constructing a starting byte prefilter.

A starting byte prefilter is a simplistic prefilter that looks for possible
matches by reporting all positions corresponding to a particular byte. This
generally only takes affect when there are at most 3 distinct possible
starting bytes. e.g., the patterns `foo`, `bar`, and `baz` have two
distinct starting bytes (`f` and `b`), and this prefilter returns all
occurrences of either `f` or `b`.

In some cases, a heuristic frequency analysis may determine that it would
be better not to use this prefilter even when there are 3 or fewer distinct
starting bytes.

#### Fields

- **`ascii_case_insensitive`**: `bool`

  Whether this prefilter should account for ASCII case insensitivity or
  not.

- **`byteset`**: `alloc::vec::Vec<bool>`

  The set of starting bytes observed.

- **`count`**: `usize`

  The number of bytes set to true in `byteset`.

- **`rank_sum`**: `u16`

  The sum of frequency ranks for the rare bytes detected. This is
  intended to give a heuristic notion of how rare the bytes are.

#### Implementations

- <span id="startbytesbuilder-new"></span>`fn new() -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

  Create a new builder for constructing a start byte prefilter.

- <span id="startbytesbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(self, yes: bool) -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

  Enable ASCII case insensitivity. When set, byte strings added to this

  builder will be interpreted without respect to ASCII case.

- <span id="startbytesbuilder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

  Build the starting bytes prefilter.

  

  If there are more than 3 distinct starting bytes, or if heuristics

  otherwise determine that this prefilter should not be used, then `None`

  is returned.

- <span id="startbytesbuilder-add"></span>`fn add(&mut self, bytes: &[u8])`

  Add a byte string to this builder.

  

  All patterns added to an Aho-Corasick automaton should be added to this

  builder before attempting to construct the prefilter.

- <span id="startbytesbuilder-add-one-byte"></span>`fn add_one_byte(&mut self, byte: u8)`

#### Trait Implementations

##### `impl Any for StartBytesBuilder`

- <span id="startbytesbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartBytesBuilder`

- <span id="startbytesbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartBytesBuilder`

- <span id="startbytesbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartBytesBuilder`

- <span id="startbytesbuilder-clone"></span>`fn clone(&self) -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

##### `impl CloneToUninit for StartBytesBuilder`

- <span id="startbytesbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartBytesBuilder`

- <span id="startbytesbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StartBytesBuilder`

- <span id="startbytesbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartBytesBuilder`

- <span id="startbytesbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StartBytesBuilder`

- <span id="startbytesbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="startbytesbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="startbytesbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StartBytesBuilder`

- <span id="startbytesbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="startbytesbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartBytesBuilder`

- <span id="startbytesbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="startbytesbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StartBytesOne`

```rust
struct StartBytesOne {
    byte1: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:858-860`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L858-L860)*

A prefilter for scanning for a single starting byte.

#### Trait Implementations

##### `impl Any for StartBytesOne`

- <span id="startbytesone-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartBytesOne`

- <span id="startbytesone-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartBytesOne`

- <span id="startbytesone-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartBytesOne`

- <span id="startbytesone-clone"></span>`fn clone(&self) -> StartBytesOne` — [`StartBytesOne`](#startbytesone)

##### `impl CloneToUninit for StartBytesOne`

- <span id="startbytesone-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartBytesOne`

- <span id="startbytesone-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StartBytesOne`

- <span id="startbytesone-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartBytesOne`

- <span id="startbytesone-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for StartBytesOne`

- <span id="startbytesone-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for StartBytesOne`

- <span id="startbytesone-toowned-type-owned"></span>`type Owned = T`

- <span id="startbytesone-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="startbytesone-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StartBytesOne`

- <span id="startbytesone-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="startbytesone-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartBytesOne`

- <span id="startbytesone-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="startbytesone-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StartBytesTwo`

```rust
struct StartBytesTwo {
    byte1: u8,
    byte2: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:874-877`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L874-L877)*

A prefilter for scanning for two starting bytes.

#### Trait Implementations

##### `impl Any for StartBytesTwo`

- <span id="startbytestwo-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartBytesTwo`

- <span id="startbytestwo-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartBytesTwo`

- <span id="startbytestwo-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartBytesTwo`

- <span id="startbytestwo-clone"></span>`fn clone(&self) -> StartBytesTwo` — [`StartBytesTwo`](#startbytestwo)

##### `impl CloneToUninit for StartBytesTwo`

- <span id="startbytestwo-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartBytesTwo`

- <span id="startbytestwo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StartBytesTwo`

- <span id="startbytestwo-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartBytesTwo`

- <span id="startbytestwo-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for StartBytesTwo`

- <span id="startbytestwo-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for StartBytesTwo`

- <span id="startbytestwo-toowned-type-owned"></span>`type Owned = T`

- <span id="startbytestwo-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="startbytestwo-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StartBytesTwo`

- <span id="startbytestwo-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="startbytestwo-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartBytesTwo`

- <span id="startbytestwo-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="startbytestwo-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StartBytesThree`

```rust
struct StartBytesThree {
    byte1: u8,
    byte2: u8,
    byte3: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:891-895`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L891-L895)*

A prefilter for scanning for three starting bytes.

#### Trait Implementations

##### `impl Any for StartBytesThree`

- <span id="startbytesthree-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartBytesThree`

- <span id="startbytesthree-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartBytesThree`

- <span id="startbytesthree-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartBytesThree`

- <span id="startbytesthree-clone"></span>`fn clone(&self) -> StartBytesThree` — [`StartBytesThree`](#startbytesthree)

##### `impl CloneToUninit for StartBytesThree`

- <span id="startbytesthree-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StartBytesThree`

- <span id="startbytesthree-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StartBytesThree`

- <span id="startbytesthree-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartBytesThree`

- <span id="startbytesthree-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PrefilterI for StartBytesThree`

- <span id="startbytesthree-prefilteri-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

##### `impl ToOwned for StartBytesThree`

- <span id="startbytesthree-toowned-type-owned"></span>`type Owned = T`

- <span id="startbytesthree-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="startbytesthree-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StartBytesThree`

- <span id="startbytesthree-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="startbytesthree-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartBytesThree`

- <span id="startbytesthree-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="startbytesthree-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `Candidate`

```rust
enum Candidate {
    None,
    Match(crate::util::search::Match),
    PossibleStartOfMatch(usize),
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:72-81`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L72-L81)*

A candidate is the result of running a prefilter on a haystack at a
particular position.

The result is either no match, a confirmed match or a possible match.

When no match is returned, the prefilter is guaranteeing that no possible
match can be found in the haystack, and the caller may trust this. That is,
all correct prefilters must never report false negatives.

In some cases, a prefilter can confirm a match very quickly, in which case,
the caller may use this to stop what it's doing and report the match. In
this case, prefilter implementations must never report a false positive.
In other cases, the prefilter can only report a potential match, in which
case the callers must attempt to confirm the match. In this case, prefilter
implementations are permitted to return false positives.

#### Variants

- **`None`**

  No match was found. Since false negatives are not possible, this means
  the search can quit as it is guaranteed not to find another match.

- **`Match`**

  A confirmed match was found. Callers do not need to confirm it.

- **`PossibleStartOfMatch`**

  The start of a possible match was found. Callers must confirm it before
  reporting it as a match.

#### Implementations

- <span id="candidate-into-option"></span>`fn into_option(self) -> Option<usize>`

  Convert this candidate into an option. This is useful when callers

  do not distinguish between true positives and false positives (i.e.,

  the caller must always confirm the match).

#### Trait Implementations

##### `impl Any for Candidate`

- <span id="candidate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Candidate`

- <span id="candidate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Candidate`

- <span id="candidate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Candidate`

- <span id="candidate-clone"></span>`fn clone(&self) -> Candidate` — [`Candidate`](#candidate)

##### `impl CloneToUninit for Candidate`

- <span id="candidate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Candidate`

- <span id="candidate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Candidate`

- <span id="candidate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Candidate`

- <span id="candidate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Candidate`

- <span id="candidate-toowned-type-owned"></span>`type Owned = T`

- <span id="candidate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="candidate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Candidate`

- <span id="candidate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="candidate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Candidate`

- <span id="candidate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="candidate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `PrefilterI`

```rust
trait PrefilterI: Send + Sync + RefUnwindSafe + UnwindSafe + Debug + 'static { ... }
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:99-108`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L99-L108)*

A prefilter describes the behavior of fast literal scanners for quickly
skipping past bytes in the haystack that we know cannot possibly
participate in a match.

#### Required Methods

- `fn find_in(&self, haystack: &[u8], span: Span) -> Candidate`

  Returns the next possible match candidate. This may yield false

#### Implementors

- [`Memmem`](#memmem)
- [`Packed`](#packed)
- [`RareBytesOne`](#rarebytesone)
- [`RareBytesThree`](#rarebytesthree)
- [`RareBytesTwo`](#rarebytestwo)
- [`StartBytesOne`](#startbytesone)
- [`StartBytesThree`](#startbytesthree)
- [`StartBytesTwo`](#startbytestwo)
- `alloc::sync::Arc<P>`

## Functions

### `opposite_ascii_case`

```rust
fn opposite_ascii_case(b: u8) -> u8
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:909-917`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L909-L917)*

If the given byte is an ASCII letter, then return it in the opposite case.
e.g., Given `b'A'`, this returns `b'a'`, and given `b'a'`, this returns
`b'A'`. If a non-ASCII letter is given, then the given byte is returned.

### `freq_rank`

```rust
fn freq_rank(b: u8) -> u8
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:921-924`](../../../../.source_1765521767/aho-corasick-1.1.4/src/util/prefilter.rs#L921-L924)*

Return the frequency rank of the given byte. The higher the rank, the more
common the byte (heuristically speaking).

