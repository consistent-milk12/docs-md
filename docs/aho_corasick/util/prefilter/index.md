*[aho_corasick](../../index.md) / [util](../index.md) / [prefilter](index.md)*

---

# Module `prefilter`

## Structs

### `Prefilter`

```rust
struct Prefilter {
    finder: alloc::sync::Arc<dyn PrefilterI>,
    memory_usage: usize,
}
```

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

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for Prefilter`

- `fn clone(self: &Self) -> Prefilter` — [`Prefilter`](#prefilter)

##### `impl Debug for Prefilter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

A builder for constructing the best possible prefilter. When constructed,
this builder will heuristically select the best prefilter it can build,
if any, and discard the rest.

#### Implementations

- `fn new(kind: MatchKind) -> Builder` — [`MatchKind`](../search/index.md), [`Builder`](#builder)

- `fn ascii_case_insensitive(self: Self, yes: bool) -> Builder` — [`Builder`](#builder)

- `fn build(self: &Self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- `fn add(self: &mut Self, bytes: &[u8])`

#### Trait Implementations

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Packed`

```rust
struct Packed(packed::Searcher);
```

A type that wraps a packed searcher and implements the `Prefilter`
interface.

#### Trait Implementations

##### `impl Clone for Packed`

- `fn clone(self: &Self) -> Packed` — [`Packed`](#packed)

##### `impl Debug for Packed`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Packed`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

### `MemmemBuilder`

```rust
struct MemmemBuilder {
    count: usize,
    one: Option<alloc::vec::Vec<u8>>,
}
```

A builder for constructing a prefilter that uses memmem.

#### Fields

- **`count`**: `usize`

  The number of patterns that have been added.

- **`one`**: `Option<alloc::vec::Vec<u8>>`

  The singular pattern to search for. This is only set when count==1.

#### Implementations

- `fn build(self: &Self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- `fn add(self: &mut Self, bytes: &[u8])`

#### Trait Implementations

##### `impl Debug for MemmemBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MemmemBuilder`

- `fn default() -> MemmemBuilder` — [`MemmemBuilder`](#memmembuilder)

### `Memmem`

```rust
struct Memmem(memchr::memmem::Finder<'static>);
```

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

- `fn clone(self: &Self) -> Memmem` — [`Memmem`](#memmem)

##### `impl Debug for Memmem`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for Memmem`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

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

- `fn new() -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

- `fn ascii_case_insensitive(self: Self, yes: bool) -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

- `fn build(self: &Self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- `fn add(self: &mut Self, bytes: &[u8])`

- `fn set_offset(self: &mut Self, pos: usize, byte: u8)`

- `fn add_rare_byte(self: &mut Self, byte: u8)`

- `fn add_one_rare_byte(self: &mut Self, byte: u8)`

#### Trait Implementations

##### `impl Clone for RareBytesBuilder`

- `fn clone(self: &Self) -> RareBytesBuilder` — [`RareBytesBuilder`](#rarebytesbuilder)

##### `impl Debug for RareBytesBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `RareByteOffsets`

```rust
struct RareByteOffsets {
    set: [RareByteOffset; 256],
}
```

A set of byte offsets, keyed by byte.

#### Fields

- **`set`**: `[RareByteOffset; 256]`

  Each entry corresponds to the maximum offset of the corresponding
  byte across all patterns seen.

#### Implementations

- `fn empty() -> RareByteOffsets` — [`RareByteOffsets`](#rarebyteoffsets)

- `fn set(self: &mut Self, byte: u8, off: RareByteOffset)` — [`RareByteOffset`](#rarebyteoffset)

#### Trait Implementations

##### `impl Clone for RareByteOffsets`

- `fn clone(self: &Self) -> RareByteOffsets` — [`RareByteOffsets`](#rarebyteoffsets)

##### `impl Copy for RareByteOffsets`

##### `impl Debug for RareByteOffsets`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `RareByteOffset`

```rust
struct RareByteOffset {
    max: u8,
}
```

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

- `fn new(max: usize) -> Option<RareByteOffset>` — [`RareByteOffset`](#rarebyteoffset)

#### Trait Implementations

##### `impl Clone for RareByteOffset`

- `fn clone(self: &Self) -> RareByteOffset` — [`RareByteOffset`](#rarebyteoffset)

##### `impl Copy for RareByteOffset`

##### `impl Debug for RareByteOffset`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for RareByteOffset`

- `fn default() -> RareByteOffset` — [`RareByteOffset`](#rarebyteoffset)

### `RareBytesOne`

```rust
struct RareBytesOne {
    byte1: u8,
    offset: RareByteOffset,
}
```

A prefilter for scanning for a single "rare" byte.

#### Trait Implementations

##### `impl Clone for RareBytesOne`

- `fn clone(self: &Self) -> RareBytesOne` — [`RareBytesOne`](#rarebytesone)

##### `impl Debug for RareBytesOne`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for RareBytesOne`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

### `RareBytesTwo`

```rust
struct RareBytesTwo {
    offsets: RareByteOffsets,
    byte1: u8,
    byte2: u8,
}
```

A prefilter for scanning for two "rare" bytes.

#### Trait Implementations

##### `impl Clone for RareBytesTwo`

- `fn clone(self: &Self) -> RareBytesTwo` — [`RareBytesTwo`](#rarebytestwo)

##### `impl Debug for RareBytesTwo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for RareBytesTwo`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

### `RareBytesThree`

```rust
struct RareBytesThree {
    offsets: RareByteOffsets,
    byte1: u8,
    byte2: u8,
    byte3: u8,
}
```

A prefilter for scanning for three "rare" bytes.

#### Trait Implementations

##### `impl Clone for RareBytesThree`

- `fn clone(self: &Self) -> RareBytesThree` — [`RareBytesThree`](#rarebytesthree)

##### `impl Debug for RareBytesThree`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for RareBytesThree`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

### `StartBytesBuilder`

```rust
struct StartBytesBuilder {
    ascii_case_insensitive: bool,
    byteset: alloc::vec::Vec<bool>,
    count: usize,
    rank_sum: u16,
}
```

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

- `fn new() -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

- `fn ascii_case_insensitive(self: Self, yes: bool) -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

- `fn build(self: &Self) -> Option<Prefilter>` — [`Prefilter`](#prefilter)

- `fn add(self: &mut Self, bytes: &[u8])`

- `fn add_one_byte(self: &mut Self, byte: u8)`

#### Trait Implementations

##### `impl Clone for StartBytesBuilder`

- `fn clone(self: &Self) -> StartBytesBuilder` — [`StartBytesBuilder`](#startbytesbuilder)

##### `impl Debug for StartBytesBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `StartBytesOne`

```rust
struct StartBytesOne {
    byte1: u8,
}
```

A prefilter for scanning for a single starting byte.

#### Trait Implementations

##### `impl Clone for StartBytesOne`

- `fn clone(self: &Self) -> StartBytesOne` — [`StartBytesOne`](#startbytesone)

##### `impl Debug for StartBytesOne`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for StartBytesOne`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

### `StartBytesTwo`

```rust
struct StartBytesTwo {
    byte1: u8,
    byte2: u8,
}
```

A prefilter for scanning for two starting bytes.

#### Trait Implementations

##### `impl Clone for StartBytesTwo`

- `fn clone(self: &Self) -> StartBytesTwo` — [`StartBytesTwo`](#startbytestwo)

##### `impl Debug for StartBytesTwo`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for StartBytesTwo`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

### `StartBytesThree`

```rust
struct StartBytesThree {
    byte1: u8,
    byte2: u8,
    byte3: u8,
}
```

A prefilter for scanning for three starting bytes.

#### Trait Implementations

##### `impl Clone for StartBytesThree`

- `fn clone(self: &Self) -> StartBytesThree` — [`StartBytesThree`](#startbytesthree)

##### `impl Debug for StartBytesThree`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl PrefilterI for StartBytesThree`

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../search/index.md), [`Candidate`](#candidate)

## Enums

### `Candidate`

```rust
enum Candidate {
    None,
    Match(crate::util::search::Match),
    PossibleStartOfMatch(usize),
}
```

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

- `fn into_option(self: Self) -> Option<usize>`

#### Trait Implementations

##### `impl Clone for Candidate`

- `fn clone(self: &Self) -> Candidate` — [`Candidate`](#candidate)

##### `impl Debug for Candidate`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Traits

### `PrefilterI`

```rust
trait PrefilterI: Send + Sync + RefUnwindSafe + UnwindSafe + Debug + 'static { ... }
```

A prefilter describes the behavior of fast literal scanners for quickly
skipping past bytes in the haystack that we know cannot possibly
participate in a match.

#### Required Methods

- `fn find_in(self: &Self, haystack: &[u8], span: Span) -> Candidate`

  Returns the next possible match candidate. This may yield false

## Functions

### `opposite_ascii_case`

```rust
fn opposite_ascii_case(b: u8) -> u8
```

If the given byte is an ASCII letter, then return it in the opposite case.
e.g., Given `b'A'`, this returns `b'a'`, and given `b'a'`, this returns
`b'A'`. If a non-ASCII letter is given, then the given byte is returned.

### `freq_rank`

```rust
fn freq_rank(b: u8) -> u8
```

Return the frequency rank of the given byte. The higher the rank, the more
common the byte (heuristically speaking).

