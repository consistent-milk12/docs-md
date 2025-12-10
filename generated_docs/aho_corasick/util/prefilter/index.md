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
  - [`opposite_ascii_case`](#opposite_ascii_case)
  - [`freq_rank`](#freq_rank)

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
| [`opposite_ascii_case`](#opposite_ascii_case) | fn | If the given byte is an ASCII letter, then return it in the opposite case. |
| [`freq_rank`](#freq_rank) | fn | Return the frequency rank of the given byte. |

## Structs

### `Prefilter`

```rust
struct Prefilter {
    finder: alloc::sync::Arc<dyn PrefilterI>,
    memory_usage: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:33-36`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L33-L36)*

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

- <span id="prefilter-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Prefilter`

- <span id="prefilter-clone"></span>`fn clone(&self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl Debug for Prefilter`

- <span id="prefilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:121-131`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L121-L131)*

A builder for constructing the best possible prefilter. When constructed,
this builder will heuristically select the best prefilter it can build,
if any, and discard the rest.

#### Implementations

- <span id="builder-new"></span>`fn new(kind: MatchKind) -> Builder` — [`MatchKind`](../search/index.md#matchkind), [`Builder`](#builder)

- <span id="builder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(self, yes: bool) -> Builder` — [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- <span id="builder-add"></span>`fn add(&mut self, bytes: &[u8])`

#### Trait Implementations

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Packed`

```rust
struct Packed(packed::Searcher);
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:328`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L328)*

A type that wraps a packed searcher and implements the `Prefilter`
interface.

#### Trait Implementations

##### `impl Clone for Packed`

- <span id="packed-clone"></span>`fn clone(&self) -> Packed` — [`Packed`](#packed)

##### `impl Debug for Packed`

- <span id="packed-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Packed`

- <span id="packed-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

### `MemmemBuilder`

```rust
struct MemmemBuilder {
    count: usize,
    one: Option<alloc::vec::Vec<u8>>,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:340-345`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L340-L345)*

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

##### `impl Debug for MemmemBuilder`

- <span id="memmembuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MemmemBuilder`

- <span id="memmembuilder-default"></span>`fn default() -> MemmemBuilder` — [`MemmemBuilder`](#memmembuilder)

### `Memmem`

```rust
struct Memmem(memchr::memmem::Finder<'static>);
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:394`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L394)*

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

##### `impl Clone for Memmem`

- <span id="memmem-clone"></span>`fn clone(&self) -> Memmem` — [`Memmem`](#memmem)

##### `impl Debug for Memmem`

- <span id="memmem-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for Memmem`

- <span id="memmem-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

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

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:419-441`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L419-L441)*

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

- <span id="rarebytesbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(self, yes: bool) -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

- <span id="rarebytesbuilder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- <span id="rarebytesbuilder-add"></span>`fn add(&mut self, bytes: &[u8])`

- <span id="rarebytesbuilder-set-offset"></span>`fn set_offset(&mut self, pos: usize, byte: u8)`

- <span id="rarebytesbuilder-add-rare-byte"></span>`fn add_rare_byte(&mut self, byte: u8)`

- <span id="rarebytesbuilder-add-one-rare-byte"></span>`fn add_one_rare_byte(&mut self, byte: u8)`

#### Trait Implementations

##### `impl Clone for RareBytesBuilder`

- <span id="rarebytesbuilder-clone"></span>`fn clone(&self) -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

##### `impl Debug for RareBytesBuilder`

- <span id="rarebytesbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RareByteOffsets`

```rust
struct RareByteOffsets {
    set: [RareByteOffset; 256],
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:445-449`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L445-L449)*

A set of byte offsets, keyed by byte.

#### Fields

- **`set`**: `[RareByteOffset; 256]`

  Each entry corresponds to the maximum offset of the corresponding
  byte across all patterns seen.

#### Implementations

- <span id="rarebyteoffsets-empty"></span>`fn empty() -> RareByteOffsets` — [`RareByteOffsets`](#rarebyteoffsets)

- <span id="rarebyteoffsets-set"></span>`fn set(&mut self, byte: u8, off: RareByteOffset)` — [`RareByteOffset`](#rarebyteoffset)

#### Trait Implementations

##### `impl Clone for RareByteOffsets`

- <span id="rarebyteoffsets-clone"></span>`fn clone(&self) -> RareByteOffsets` — [`RareByteOffsets`](#rarebyteoffsets)

##### `impl Copy for RareByteOffsets`

##### `impl Debug for RareByteOffsets`

- <span id="rarebyteoffsets-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `RareByteOffset`

```rust
struct RareByteOffset {
    max: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:482-497`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L482-L497)*

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

#### Trait Implementations

##### `impl Clone for RareByteOffset`

- <span id="rarebyteoffset-clone"></span>`fn clone(&self) -> RareByteOffset` — [`RareByteOffset`](#rarebyteoffset)

##### `impl Copy for RareByteOffset`

##### `impl Debug for RareByteOffset`

- <span id="rarebyteoffset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RareByteOffset`

- <span id="rarebyteoffset-default"></span>`fn default() -> RareByteOffset` — [`RareByteOffset`](#rarebyteoffset)

### `RareBytesOne`

```rust
struct RareBytesOne {
    byte1: u8,
    offset: RareByteOffset,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:668-671`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L668-L671)*

A prefilter for scanning for a single "rare" byte.

#### Trait Implementations

##### `impl Clone for RareBytesOne`

- <span id="rarebytesone-clone"></span>`fn clone(&self) -> RareBytesOne` — [`RareBytesOne`](#rarebytesone)

##### `impl Debug for RareBytesOne`

- <span id="rarebytesone-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for RareBytesOne`

- <span id="rarebytesone-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

### `RareBytesTwo`

```rust
struct RareBytesTwo {
    offsets: RareByteOffsets,
    byte1: u8,
    byte2: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:691-695`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L691-L695)*

A prefilter for scanning for two "rare" bytes.

#### Trait Implementations

##### `impl Clone for RareBytesTwo`

- <span id="rarebytestwo-clone"></span>`fn clone(&self) -> RareBytesTwo` — [`RareBytesTwo`](#rarebytestwo)

##### `impl Debug for RareBytesTwo`

- <span id="rarebytestwo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for RareBytesTwo`

- <span id="rarebytestwo-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

### `RareBytesThree`

```rust
struct RareBytesThree {
    offsets: RareByteOffsets,
    byte1: u8,
    byte2: u8,
    byte3: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:713-718`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L713-L718)*

A prefilter for scanning for three "rare" bytes.

#### Trait Implementations

##### `impl Clone for RareBytesThree`

- <span id="rarebytesthree-clone"></span>`fn clone(&self) -> RareBytesThree` — [`RareBytesThree`](#rarebytesthree)

##### `impl Debug for RareBytesThree`

- <span id="rarebytesthree-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for RareBytesThree`

- <span id="rarebytesthree-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

### `StartBytesBuilder`

```rust
struct StartBytesBuilder {
    ascii_case_insensitive: bool,
    byteset: alloc::vec::Vec<bool>,
    count: usize,
    rank_sum: u16,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:746-757`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L746-L757)*

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

- <span id="startbytesbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(self, yes: bool) -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

- <span id="startbytesbuilder-build"></span>`fn build(&self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- <span id="startbytesbuilder-add"></span>`fn add(&mut self, bytes: &[u8])`

- <span id="startbytesbuilder-add-one-byte"></span>`fn add_one_byte(&mut self, byte: u8)`

#### Trait Implementations

##### `impl Clone for StartBytesBuilder`

- <span id="startbytesbuilder-clone"></span>`fn clone(&self) -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

##### `impl Debug for StartBytesBuilder`

- <span id="startbytesbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StartBytesOne`

```rust
struct StartBytesOne {
    byte1: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:858-860`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L858-L860)*

A prefilter for scanning for a single starting byte.

#### Trait Implementations

##### `impl Clone for StartBytesOne`

- <span id="startbytesone-clone"></span>`fn clone(&self) -> StartBytesOne` — [`StartBytesOne`](#startbytesone)

##### `impl Debug for StartBytesOne`

- <span id="startbytesone-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for StartBytesOne`

- <span id="startbytesone-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

### `StartBytesTwo`

```rust
struct StartBytesTwo {
    byte1: u8,
    byte2: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:874-877`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L874-L877)*

A prefilter for scanning for two starting bytes.

#### Trait Implementations

##### `impl Clone for StartBytesTwo`

- <span id="startbytestwo-clone"></span>`fn clone(&self) -> StartBytesTwo` — [`StartBytesTwo`](#startbytestwo)

##### `impl Debug for StartBytesTwo`

- <span id="startbytestwo-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for StartBytesTwo`

- <span id="startbytestwo-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

### `StartBytesThree`

```rust
struct StartBytesThree {
    byte1: u8,
    byte2: u8,
    byte3: u8,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:891-895`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L891-L895)*

A prefilter for scanning for three starting bytes.

#### Trait Implementations

##### `impl Clone for StartBytesThree`

- <span id="startbytesthree-clone"></span>`fn clone(&self) -> StartBytesThree` — [`StartBytesThree`](#startbytesthree)

##### `impl Debug for StartBytesThree`

- <span id="startbytesthree-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl PrefilterI for StartBytesThree`

- <span id="startbytesthree-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md#span), [`Candidate`](#candidate)

## Enums

### `Candidate`

```rust
enum Candidate {
    None,
    Match(crate::util::search::Match),
    PossibleStartOfMatch(usize),
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:72-81`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L72-L81)*

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

#### Trait Implementations

##### `impl Clone for Candidate`

- <span id="candidate-clone"></span>`fn clone(&self) -> Candidate` — [`Candidate`](#candidate)

##### `impl Debug for Candidate`

- <span id="candidate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `PrefilterI`

```rust
trait PrefilterI: Send + Sync + RefUnwindSafe + UnwindSafe + Debug + 'static { ... }
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:99-108`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L99-L108)*

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

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:909-917`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L909-L917)*

If the given byte is an ASCII letter, then return it in the opposite case.
e.g., Given `b'A'`, this returns `b'a'`, and given `b'a'`, this returns
`b'A'`. If a non-ASCII letter is given, then the given byte is returned.

### `freq_rank`

```rust
fn freq_rank(b: u8) -> u8
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:921-924`](../../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L921-L924)*

Return the frequency rank of the given byte. The higher the rank, the more
common the byte (heuristically speaking).

