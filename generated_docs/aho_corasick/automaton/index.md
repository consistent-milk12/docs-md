*[aho_corasick](../index.md) / [automaton](index.md)*

---

# Module `automaton`

Provides [`Automaton`](#automaton) trait for abstracting over Aho-Corasick automata.

The `Automaton` trait provides a way to write generic code over any
Aho-Corasick automaton. It also provides access to lower level APIs that
permit walking the state transitions of an Aho-Corasick automaton manually.

## Contents

- [Modules](#modules)
  - [`private`](#private)
- [Structs](#structs)
  - [`Prefilter`](#prefilter)
  - [`StateID`](#stateid)
  - [`StateIDError`](#stateiderror)
  - [`OverlappingState`](#overlappingstate)
  - [`FindIter`](#finditer)
  - [`FindOverlappingIter`](#findoverlappingiter)
  - [`StreamFindIter`](#streamfinditer)
  - [`StreamChunkIter`](#streamchunkiter)
- [Enums](#enums)
  - [`Candidate`](#candidate)
  - [`StreamChunk`](#streamchunk)
- [Traits](#traits)
  - [`Automaton`](#automaton)
- [Functions](#functions)
  - [`try_find_fwd`](#try-find-fwd)
  - [`try_find_fwd_imp`](#try-find-fwd-imp)
  - [`try_find_overlapping_fwd`](#try-find-overlapping-fwd)
  - [`try_find_overlapping_fwd_imp`](#try-find-overlapping-fwd-imp)
  - [`get_match`](#get-match)
  - [`fmt_state_indicator`](#fmt-state-indicator)
  - [`sparse_transitions`](#sparse-transitions)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`private`](#private) | mod | We seal the `Automaton` trait for now. |
| [`Prefilter`](#prefilter) | struct |  |
| [`StateID`](#stateid) | struct |  |
| [`StateIDError`](#stateiderror) | struct |  |
| [`OverlappingState`](#overlappingstate) | struct | Represents the current state of an overlapping search. |
| [`FindIter`](#finditer) | struct | An iterator of non-overlapping matches in a particular haystack. |
| [`FindOverlappingIter`](#findoverlappingiter) | struct | An iterator of overlapping matches in a particular haystack. |
| [`StreamFindIter`](#streamfinditer) | struct | An iterator that reports matches in a stream. |
| [`StreamChunkIter`](#streamchunkiter) | struct | An iterator that reports matches in a stream. |
| [`Candidate`](#candidate) | enum |  |
| [`StreamChunk`](#streamchunk) | enum | A single chunk yielded by the stream chunk iterator. |
| [`Automaton`](#automaton) | trait | A trait that abstracts over Aho-Corasick automata. |
| [`try_find_fwd`](#try-find-fwd) | fn |  |
| [`try_find_fwd_imp`](#try-find-fwd-imp) | fn |  |
| [`try_find_overlapping_fwd`](#try-find-overlapping-fwd) | fn |  |
| [`try_find_overlapping_fwd_imp`](#try-find-overlapping-fwd-imp) | fn |  |
| [`get_match`](#get-match) | fn |  |
| [`fmt_state_indicator`](#fmt-state-indicator) | fn | Write a prefix "state" indicator for fmt::Debug impls. |
| [`sparse_transitions`](#sparse-transitions) | fn | Return an iterator of transitions in a sparse format given an iterator of all explicitly defined transitions. |

## Modules

- [`private`](private/index.md) — We seal the `Automaton` trait for now. It's a big trait, and it's

## Structs

### `Prefilter`

```rust
struct Prefilter {
    finder: alloc::sync::Arc<dyn PrefilterI>,
    memory_usage: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:33-36`](../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L33-L36)*

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

- <span id="prefilter-find-in"></span>`fn find_in(&self, haystack: &[u8], span: Span) -> Candidate` — [`Span`](../util/search/index.md#span), [`Candidate`](../util/prefilter/index.md#candidate)

- <span id="prefilter-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Prefilter`

- <span id="prefilter-clone"></span>`fn clone(&self) -> Prefilter` — [`Prefilter`](../util/prefilter/index.md#prefilter)

##### `impl Debug for Prefilter`

- <span id="prefilter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StateID`

```rust
struct StateID(SmallIndex);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:734`](../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L734)*

The identifier of a finite automaton state.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `StateID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

#### Implementations

- <span id="stateid-const-max"></span>`const MAX: StateID`

- <span id="stateid-const-limit"></span>`const LIMIT: usize`

- <span id="stateid-const-zero"></span>`const ZERO: StateID`

- <span id="stateid-const-size"></span>`const SIZE: usize`

- <span id="stateid-new"></span>`fn new(value: usize) -> Result<StateID, StateIDError>` — [`StateID`](../util/primitives/index.md#stateid), [`StateIDError`](../util/primitives/index.md#stateiderror)

- <span id="stateid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> StateID` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="stateid-from-u32-unchecked"></span>`const fn from_u32_unchecked(index: u32) -> StateID` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="stateid-must"></span>`fn must(value: usize) -> StateID` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="stateid-as-usize"></span>`const fn as_usize(&self) -> usize`

- <span id="stateid-as-u64"></span>`const fn as_u64(&self) -> u64`

- <span id="stateid-as-u32"></span>`const fn as_u32(&self) -> u32`

- <span id="stateid-as-i32"></span>`const fn as_i32(&self) -> i32`

- <span id="stateid-one-more"></span>`fn one_more(&self) -> usize`

- <span id="stateid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<StateID, StateIDError>` — [`StateID`](../util/primitives/index.md#stateid), [`StateIDError`](../util/primitives/index.md#stateiderror)

- <span id="stateid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> StateID` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="stateid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

- <span id="stateid-iter"></span>`fn iter(len: usize) -> StateIDIter` — [`StateIDIter`](../util/primitives/index.md#stateiditer)

#### Trait Implementations

##### `impl Clone for StateID`

- <span id="stateid-clone"></span>`fn clone(&self) -> StateID` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl Copy for StateID`

##### `impl Debug for StateID`

- <span id="stateid-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for StateID`

- <span id="stateid-default"></span>`fn default() -> StateID` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl Eq for StateID`

##### `impl Hash for StateID`

- <span id="stateid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: StateID) -> &T` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl<T> IndexMut for [T]`

- <span id="t-index-mut"></span>`fn index_mut(&mut self, index: StateID) -> &mut T` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl Ord for StateID`

- <span id="stateid-cmp"></span>`fn cmp(&self, other: &StateID) -> cmp::Ordering` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl PartialEq for StateID`

- <span id="stateid-eq"></span>`fn eq(&self, other: &StateID) -> bool` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl PartialOrd for StateID`

- <span id="stateid-partial-cmp"></span>`fn partial_cmp(&self, other: &StateID) -> option::Option<cmp::Ordering>` — [`StateID`](../util/primitives/index.md#stateid)

##### `impl StructuralPartialEq for StateID`

### `StateIDError`

```rust
struct StateIDError(SmallIndexError);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:737`](../../../.source_1765210505/aho-corasick-1.1.4/src/util/primitives.rs#L737)*

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- <span id="stateiderror-attempted"></span>`fn attempted(&self) -> u64`

#### Trait Implementations

##### `impl Clone for StateIDError`

- <span id="stateiderror-clone"></span>`fn clone(&self) -> StateIDError` — [`StateIDError`](../util/primitives/index.md#stateiderror)

##### `impl Debug for StateIDError`

- <span id="stateiderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for StateIDError`

- <span id="stateiderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for StateIDError`

##### `impl Error for StateIDError`

##### `impl PartialEq for StateIDError`

- <span id="stateiderror-eq"></span>`fn eq(&self, other: &StateIDError) -> bool` — [`StateIDError`](../util/primitives/index.md#stateiderror)

##### `impl StructuralPartialEq for StateIDError`

##### `impl ToString for StateIDError`

- <span id="stateiderror-to-string"></span>`fn to_string(&self) -> String`

### `OverlappingState`

```rust
struct OverlappingState {
    mat: Option<crate::util::search::Match>,
    id: Option<StateID>,
    at: usize,
    next_match_index: Option<usize>,
}
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:782-811`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L782-L811)*

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in and the current offset of the search
in the haystack.

This type provides limited introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and to ask whether a match has been found.

Callers should always provide a fresh state constructed via
`OverlappingState::start` when starting a new search. That same state
should be reused for subsequent searches on the same `Input`. The state
given will advance through the haystack itself. Callers can detect the end
of a search when neither an error nor a match is returned.

# Example

This example shows how to manually iterate over all overlapping matches. If
you need this, you might consider using
[`AhoCorasick::find_overlapping_iter`](crate::AhoCorasick::find_overlapping_iter)
instead, but this shows how to correctly use an `OverlappingState`.

```rust
use aho_corasick::{
    automaton::OverlappingState,
    AhoCorasick, Input, Match,
};

let patterns = &["append", "appendage", "app"];
let haystack = "append the app to the appendage";

let ac = AhoCorasick::new(patterns).unwrap();
let mut state = OverlappingState::start();
let mut matches = vec![];

loop {
    ac.find_overlapping(haystack, &mut state);
    let mat = match state.get_match() {
        None => break,
        Some(mat) => mat,
    };
    matches.push(mat);
}
let expected = vec![
    Match::must(2, 0..3),
    Match::must(0, 0..6),
    Match::must(2, 11..14),
    Match::must(2, 22..25),
    Match::must(0, 22..28),
    Match::must(1, 22..31),
];
assert_eq!(expected, matches);
```

#### Fields

- **`mat`**: `Option<crate::util::search::Match>`

  The match reported by the most recent overlapping search to use this
  state.
  
  If a search does not find any matches, then it is expected to clear
  this value.

- **`id`**: `Option<StateID>`

  The state ID of the state at which the search was in when the call
  terminated. When this is a match state, `last_match` must be set to a
  non-None value.
  
  A `None` value indicates the start state of the corresponding
  automaton. We cannot use the actual ID, since any one automaton may
  have many start states, and which one is in use depends on search-time
  factors (such as whether the search is anchored or not).

- **`at`**: `usize`

  The position of the search.
  
  When `id` is None (i.e., we are starting a search), this is set to
  the beginning of the search as given by the caller regardless of its
  current value. Subsequent calls to an overlapping search pick up at
  this offset.

- **`next_match_index`**: `Option<usize>`

  The index into the matching patterns of the next match to report if the
  current state is a match state. Note that this may be 1 greater than
  the total number of matches to report for the current match state. (In
  which case, no more matches should be reported at the current position
  and the search should advance to the next position.)

#### Implementations

- <span id="overlappingstate-start"></span>`fn start() -> OverlappingState` — [`OverlappingState`](#overlappingstate)

- <span id="overlappingstate-get-match"></span>`fn get_match(&self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

#### Trait Implementations

##### `impl Clone for OverlappingState`

- <span id="overlappingstate-clone"></span>`fn clone(&self) -> OverlappingState` — [`OverlappingState`](#overlappingstate)

##### `impl Debug for OverlappingState`

- <span id="overlappingstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FindIter<'a, 'h, A>`

```rust
struct FindIter<'a, 'h, A> {
    aut: &'a A,
    input: crate::util::search::Input<'h>,
    last_match_end: Option<usize>,
}
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:844-855`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L844-L855)*

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`](../util/search/index.md) used by this
automaton.

This iterator is constructed via the `Automaton::try_find_iter` method.

The type variable `A` refers to the implementation of the [`Automaton`](#automaton)
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`](#automaton)
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Fields

- **`aut`**: `&'a A`

  The automaton used to drive the search.

- **`input`**: `crate::util::search::Input<'h>`

  The input parameters to give to each search call.
  
  The start position of the search is mutated during iteration.

- **`last_match_end`**: `Option<usize>`

  Records the end offset of the most recent match. This is necessary to
  handle a corner case for preventing empty matches from overlapping with
  the ending bounds of a prior match.

#### Implementations

- <span id="finditer-new"></span>`fn new(aut: &'a A, input: Input<'h>) -> Result<FindIter<'a, 'h, A>, MatchError>` — [`Input`](../util/search/index.md#input), [`FindIter`](#finditer), [`MatchError`](../util/error/index.md#matcherror)

- <span id="finditer-search"></span>`fn search(&self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

- <span id="finditer-handle-overlapping-empty-match"></span>`fn handle_overlapping_empty_match(&mut self, m: Match) -> Option<Match>` — [`Match`](../util/search/index.md#match)

#### Trait Implementations

##### `impl<A: fmt::Debug> Debug for FindIter<'a, 'h, A>`

- <span id="finditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindIter<'a, 'h, A>`

- <span id="finditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="finditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="finditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Automaton> Iterator for FindIter<'a, 'h, A>`

- <span id="finditer-iterator-type-item"></span>`type Item = Match`

- <span id="finditer-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

### `FindOverlappingIter<'a, 'h, A>`

```rust
struct FindOverlappingIter<'a, 'h, A> {
    aut: &'a A,
    input: crate::util::search::Input<'h>,
    state: OverlappingState,
}
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:954-958`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L954-L958)*

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the
`Automaton::try_find_overlapping_iter` method.

The type variable `A` refers to the implementation of the [`Automaton`](#automaton)
trait used to execute the search.

The lifetime `'a` refers to the lifetime of the [`Automaton`](#automaton)
implementation.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl<A: fmt::Debug> Debug for FindOverlappingIter<'a, 'h, A>`

- <span id="findoverlappingiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindOverlappingIter<'a, 'h, A>`

- <span id="findoverlappingiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findoverlappingiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findoverlappingiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Automaton> Iterator for FindOverlappingIter<'a, 'h, A>`

- <span id="findoverlappingiter-iterator-type-item"></span>`type Item = Match`

- <span id="findoverlappingiter-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

### `StreamFindIter<'a, A, R>`

```rust
struct StreamFindIter<'a, A, R> {
    it: StreamChunkIter<'a, A, R>,
}
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:991-993`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L991-L993)*

An iterator that reports matches in a stream.

This iterator yields elements of type `io::Result<Match>`, where an error
is reported if there was a problem reading from the underlying stream.
The iterator terminates only when the underlying stream reaches `EOF`.

This iterator is constructed via the `Automaton::try_stream_find_iter`
method.

The type variable `A` refers to the implementation of the [`Automaton`](#automaton)
trait used to execute the search.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the [`Automaton`](#automaton)
implementation.

#### Trait Implementations

##### `impl<A: fmt::Debug, R: fmt::Debug> Debug for StreamFindIter<'a, A, R>`

- <span id="streamfinditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for StreamFindIter<'a, A, R>`

- <span id="streamfinditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamfinditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamfinditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Automaton, R: std::io::Read> Iterator for StreamFindIter<'a, A, R>`

- <span id="streamfinditer-iterator-type-item"></span>`type Item = Result<Match, Error>`

- <span id="streamfinditer-next"></span>`fn next(&mut self) -> Option<std::io::Result<Match>>` — [`Match`](../util/search/index.md#match)

### `StreamChunkIter<'a, A, R>`

```rust
struct StreamChunkIter<'a, A, R> {
    aut: &'a A,
    rdr: R,
    buf: crate::util::buffer::Buffer,
    start: StateID,
    sid: StateID,
    absolute_pos: usize,
    buffer_pos: usize,
    buffer_reported_pos: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1036-1063`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1036-L1063)*

An iterator that reports matches in a stream.

(This doesn't actually implement the `Iterator` trait because it returns
something with a lifetime attached to a buffer it owns, but that's OK. It
still has a `next` method and is iterator-like enough to be fine.)

This iterator yields elements of type `io::Result<StreamChunk>`, where
an error is reported if there was a problem reading from the underlying
stream. The iterator terminates only when the underlying stream reaches
`EOF`.

The idea here is that each chunk represents either a match or a non-match,
and if you concatenated all of the chunks together, you'd reproduce the
entire contents of the stream, byte-for-byte.

This chunk machinery is a bit complicated and it isn't strictly required
for a stream searcher that just reports matches. But we do need something
like this to deal with the "replacement" API, which needs to know which
chunks it can copy and which it needs to replace.

#### Fields

- **`aut`**: `&'a A`

  The underlying automaton to do the search.

- **`rdr`**: `R`

  The source of bytes we read from.

- **`buf`**: `crate::util::buffer::Buffer`

  A roll buffer for managing bytes from `rdr`. Basically, this is used
  to handle the case of a match that is split by two different
  calls to `rdr.read()`. This isn't strictly needed if all we needed to
  do was report matches, but here we are reporting chunks of non-matches
  and matches and in order to do that, we really just cannot treat our
  stream as non-overlapping blocks of bytes. We need to permit some
  overlap while we retain bytes from a previous `read` call in memory.

- **`start`**: `StateID`

  The unanchored starting state of this automaton.

- **`sid`**: `StateID`

  The state of the automaton.

- **`absolute_pos`**: `usize`

  The absolute position over the entire stream.

- **`buffer_pos`**: `usize`

  The position we're currently at within `buf`.

- **`buffer_reported_pos`**: `usize`

  The buffer position of the end of the bytes that we last returned
  to the caller. Basically, whenever we find a match, we look to see if
  there is a difference between where the match started and the position
  of the last byte we returned to the caller. If there's a difference,
  then we need to return a 'NonMatch' chunk.

#### Implementations

- <span id="streamchunkiter-new"></span>`fn new(aut: &'a A, rdr: R) -> Result<StreamChunkIter<'a, A, R>, MatchError>` — [`StreamChunkIter`](#streamchunkiter), [`MatchError`](../util/error/index.md#matcherror)

- <span id="streamchunkiter-next"></span>`fn next(&mut self) -> Option<std::io::Result<StreamChunk<'_>>>` — [`StreamChunk`](#streamchunk)

- <span id="streamchunkiter-get-match-chunk"></span>`fn get_match_chunk(&self, mat: Match) -> core::ops::Range<usize>` — [`Match`](../util/search/index.md#match)

- <span id="streamchunkiter-get-non-match-chunk"></span>`fn get_non_match_chunk(&self, mat: Match) -> Option<core::ops::Range<usize>>` — [`Match`](../util/search/index.md#match)

- <span id="streamchunkiter-get-pre-roll-non-match-chunk"></span>`fn get_pre_roll_non_match_chunk(&self) -> Option<core::ops::Range<usize>>`

- <span id="streamchunkiter-get-eof-non-match-chunk"></span>`fn get_eof_non_match_chunk(&self) -> Option<core::ops::Range<usize>>`

- <span id="streamchunkiter-get-match"></span>`fn get_match(&self) -> Match` — [`Match`](../util/search/index.md#match)

#### Trait Implementations

##### `impl<A: fmt::Debug, R: fmt::Debug> Debug for StreamChunkIter<'a, A, R>`

- <span id="streamchunkiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Candidate`

```rust
enum Candidate {
    None,
    Match(crate::util::search::Match),
    PossibleStartOfMatch(usize),
}
```

*Defined in [`aho-corasick-1.1.4/src/util/prefilter.rs:72-81`](../../../.source_1765210505/aho-corasick-1.1.4/src/util/prefilter.rs#L72-L81)*

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

- <span id="candidate-clone"></span>`fn clone(&self) -> Candidate` — [`Candidate`](../util/prefilter/index.md#candidate)

##### `impl Debug for Candidate`

- <span id="candidate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StreamChunk<'r>`

```rust
enum StreamChunk<'r> {
    NonMatch {
        bytes: &'r [u8],
    },
    Match {
        bytes: &'r [u8],
        mat: crate::util::search::Match,
    },
}
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1251-1256`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1251-L1256)*

A single chunk yielded by the stream chunk iterator.

The `'r` lifetime refers to the lifetime of the stream chunk iterator.

#### Variants

- **`NonMatch`**

  A chunk that does not contain any matches.

- **`Match`**

  A chunk that precisely contains a match.

#### Trait Implementations

##### `impl Debug for StreamChunk<'r>`

- <span id="streamchunk-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Traits

### `Automaton`

```rust
trait Automaton: private::Sealed { ... }
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:198-637`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L198-L637)*

A trait that abstracts over Aho-Corasick automata.

This trait primarily exists for niche use cases such as:

* Using an NFA or DFA directly, bypassing the top-level
[`AhoCorasick`](crate::AhoCorasick) searcher. Currently, these include
[`noncontiguous::NFA`](crate::nfa::noncontiguous::NFA),
[`contiguous::NFA`](crate::nfa::contiguous::NFA) and
[`dfa::DFA`](crate::dfa::DFA).
* Implementing your own custom search routine by walking the automaton
yourself. This might be useful for implementing search on non-contiguous
strings or streams.

For most use cases, it is not expected that users will need
to use or even know about this trait. Indeed, the top level
[`AhoCorasick`](crate::AhoCorasick) searcher does not expose any details
about this trait, nor does it implement it itself.

Note that this trait defines a number of default methods, such as
`Automaton::try_find` and `Automaton::try_find_iter`, which implement
higher level search routines in terms of the lower level automata API.

# Sealed

Currently, this trait is sealed. That means users of this crate can write
generic routines over this trait but cannot implement it themselves. This
restriction may be lifted in the future, but sealing the trait permits
adding new required methods in a backwards compatible fashion.

# Special states

This trait encodes a notion of "special" states in an automaton. Namely,
a state is treated as special if it is a dead, match or start state:

* A dead state is a state that cannot be left once entered. All transitions
on a dead state lead back to itself. The dead state is meant to be treated
as a sentinel indicating that the search should stop and return a match if
one has been found, and nothing otherwise.
* A match state is a state that indicates one or more patterns have
matched. Depending on the [`MatchKind`](../util/search/index.md) of the automaton, a search may
stop once a match is seen, or it may continue looking for matches until
it enters a dead state or sees the end of the haystack.
* A start state is a state that a search begins in. It is useful to know
when a search enters a start state because it may mean that a prefilter can
be used to skip ahead and quickly look for candidate matches. Unlike dead
and match states, it is never necessary to explicitly handle start states
for correctness. Indeed, in this crate, implementations of `Automaton`
will only treat start states as "special" when a prefilter is enabled and
active. Otherwise, treating it as special has no purpose and winds up
slowing down the overall search because it results in ping-ponging between
the main state transition and the "special" state logic.

Since checking whether a state is special by doing three different
checks would be too expensive inside a fast search loop, the
`Automaton::is_special` method is provided for quickly checking whether
the state is special. The `Automaton::is_dead`, `Automaton::is_match` and
`Automaton::is_start` predicates can then be used to determine which kind
of special state it is.

# Panics

Most of the APIs on this trait should panic or give incorrect results
if invalid inputs are given to it. For example, `Automaton::next_state`
has unspecified behavior if the state ID given to it is not a valid
state ID for the underlying automaton. Valid state IDs can only be
retrieved in one of two ways: calling `Automaton::start_state` or calling
`Automaton::next_state` with a valid state ID.

# Safety

This trait is not safe to implement so that code may rely on the
correctness of implementations of this trait to avoid undefined behavior.
The primary correctness guarantees are:

* `Automaton::start_state` always returns a valid state ID or an error or
panics.
* `Automaton::next_state`, when given a valid state ID, always returns
a valid state ID for all values of `anchored` and `byte`, or otherwise
panics.

In general, the rest of the methods on `Automaton` need to uphold their
contracts as well. For example, `Automaton::is_dead` should only returns
true if the given state ID is actually a dead state.

Note that currently this crate does not rely on the safety property defined
here to avoid undefined behavior. Instead, this was done to make it
_possible_ to do in the future.

# Example

This example shows how one might implement a basic but correct search
routine. We keep things simple by not using prefilters or worrying about
anchored searches, but do make sure our search is correct for all possible
[`MatchKind`](../util/search/index.md) semantics. (The comments in the code below note the parts
that are needed to support certain `MatchKind` semantics.)

```rust
use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
    Anchored, Match, MatchError, MatchKind,
};

// Run an unanchored search for 'aut' in 'haystack'. Return the first match
// seen according to the automaton's match semantics. This returns an error
// if the given automaton does not support unanchored searches.
fn find<A: Automaton>(
    aut: A,
    haystack: &[u8],
) -> Result<Option<Match>, MatchError> {
    let mut sid = aut.start_state(Anchored::No)?;
    let mut at = 0;
    let mut mat = None;
    let get_match = |sid, at| {
        let pid = aut.match_pattern(sid, 0);
        let len = aut.pattern_len(pid);
        Match::new(pid, (at - len)..at)
    };
    // Start states can be match states!
    if aut.is_match(sid) {
        mat = Some(get_match(sid, at));
        // Standard semantics require matches to be reported as soon as
        // they're seen. Otherwise, we continue until we see a dead state
        // or the end of the haystack.
        if matches!(aut.match_kind(), MatchKind::Standard) {
            return Ok(mat);
        }
    }
    while at < haystack.len() {
        sid = aut.next_state(Anchored::No, sid, haystack[at]);
        if aut.is_special(sid) {
            if aut.is_dead(sid) {
                return Ok(mat);
            } else if aut.is_match(sid) {
                mat = Some(get_match(sid, at + 1));
                // As above, standard semantics require that we return
                // immediately once a match is found.
                if matches!(aut.match_kind(), MatchKind::Standard) {
                    return Ok(mat);
                }
            }
        }
        at += 1;
    }
    Ok(mat)
}

// Show that it works for standard searches.
let nfa = NFA::new(&["samwise", "sam"]).unwrap();
assert_eq!(Some(Match::must(1, 0..3)), find(&nfa, b"samwise")?);

// But also works when using leftmost-first. Notice how the match result
// has changed!
let nfa = NFA::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(&["samwise", "sam"])
    .unwrap();
assert_eq!(Some(Match::must(0, 0..7)), find(&nfa, b"samwise")?);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Required Methods

- `fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>`

  Returns the starting state for the given anchor mode.

- `fn next_state(&self, anchored: Anchored, sid: StateID, byte: u8) -> StateID`

  Performs a state transition from `sid` for `byte` and returns the next

- `fn is_special(&self, sid: StateID) -> bool`

  Returns true if the given ID represents a "special" state. A special

- `fn is_dead(&self, sid: StateID) -> bool`

  Returns true if the given ID represents a dead state.

- `fn is_match(&self, sid: StateID) -> bool`

  Returns true if the given ID represents a match state.

- `fn is_start(&self, sid: StateID) -> bool`

  Returns true if the given ID represents a start state.

- `fn match_kind(&self) -> MatchKind`

  Returns the match semantics that this automaton was built with.

- `fn match_len(&self, sid: StateID) -> usize`

  Returns the total number of matches for the given state ID.

- `fn match_pattern(&self, sid: StateID, index: usize) -> PatternID`

  Returns the pattern ID for the match state given by `sid` at the

- `fn patterns_len(&self) -> usize`

  Returns the total number of patterns compiled into this automaton.

- `fn pattern_len(&self, pid: PatternID) -> usize`

  Returns the length of the pattern for the given ID.

- `fn min_pattern_len(&self) -> usize`

  Returns the length, in bytes, of the shortest pattern in this

- `fn max_pattern_len(&self) -> usize`

  Returns the length, in bytes, of the longest pattern in this automaton.

- `fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, used by this automaton.

- `fn prefilter(&self) -> Option<&Prefilter>`

  Returns a prefilter, if available, that can be used to accelerate

#### Provided Methods

- `fn try_find(&self, input: &Input<'_>) -> Result<Option<Match>, MatchError>`

  Executes a non-overlapping search with this automaton using the given

- `fn try_find_overlapping(&self, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>`

  Executes a overlapping search with this automaton using the given

- `fn try_find_iter<'a, 'h>(self: &'a Self, input: Input<'h>) -> Result<FindIter<'a, 'h, Self>, MatchError>`

  Returns an iterator of non-overlapping matches with this automaton

- `fn try_find_overlapping_iter<'a, 'h>(self: &'a Self, input: Input<'h>) -> Result<FindOverlappingIter<'a, 'h, Self>, MatchError>`

  Returns an iterator of overlapping matches with this automaton

- `fn try_replace_all<B>(&self, haystack: &str, replace_with: &[B]) -> Result<String, MatchError>`

  Replaces all non-overlapping matches in `haystack` with

- `fn try_replace_all_bytes<B>(&self, haystack: &[u8], replace_with: &[B]) -> Result<Vec<u8>, MatchError>`

  Replaces all non-overlapping matches in `haystack` with

- `fn try_replace_all_with<F>(&self, haystack: &str, dst: &mut String, replace_with: F) -> Result<(), MatchError>`

  Replaces all non-overlapping matches in `haystack` by calling the

- `fn try_replace_all_with_bytes<F>(&self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) -> Result<(), MatchError>`

  Replaces all non-overlapping matches in `haystack` by calling the

- `fn try_stream_find_iter<'a, R: std::io::Read>(self: &'a Self, rdr: R) -> Result<StreamFindIter<'a, Self, R>, MatchError>`

  Returns an iterator of non-overlapping matches with this automaton

- `fn try_stream_replace_all<R, W, B>(&self, rdr: R, wtr: W, replace_with: &[B]) -> std::io::Result<()>`

  Replaces all non-overlapping matches in `rdr` with strings from

- `fn try_stream_replace_all_with<R, W, F>(&self, rdr: R, wtr: W, replace_with: F) -> std::io::Result<()>`

  Replaces all non-overlapping matches in `rdr` by calling the

#### Implementors

- [`DFA`](../dfa/index.md#dfa)
- [`NFA`](../nfa/contiguous/index.md#nfa)
- [`NFA`](../nfa/noncontiguous/index.md#nfa)
- `&'a A`

## Functions

### `try_find_fwd`

```rust
fn try_find_fwd<A: Automaton + ?Sized>(aut: &A, input: &crate::util::search::Input<'_>) -> Result<Option<crate::util::search::Match>, crate::util::error::MatchError>
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1259-1282`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1259-L1282)*

### `try_find_fwd_imp`

```rust
fn try_find_fwd_imp<A: Automaton + ?Sized>(aut: &A, input: &crate::util::search::Input<'_>, pre: Option<&Prefilter>, anchored: crate::util::search::Anchored, earliest: bool) -> Result<Option<crate::util::search::Match>, crate::util::error::MatchError>
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1285-1420`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1285-L1420)*

### `try_find_overlapping_fwd`

```rust
fn try_find_overlapping_fwd<A: Automaton + ?Sized>(aut: &A, input: &crate::util::search::Input<'_>, state: &mut OverlappingState) -> Result<(), crate::util::error::MatchError>
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1423-1440`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1423-L1440)*

### `try_find_overlapping_fwd_imp`

```rust
fn try_find_overlapping_fwd_imp<A: Automaton + ?Sized>(aut: &A, input: &crate::util::search::Input<'_>, pre: Option<&Prefilter>, state: &mut OverlappingState) -> Result<(), crate::util::error::MatchError>
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1443-1537`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1443-L1537)*

### `get_match`

```rust
fn get_match<A: Automaton + ?Sized>(aut: &A, sid: StateID, index: usize, at: usize) -> crate::util::search::Match
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1540-1549`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1540-L1549)*

### `fmt_state_indicator`

```rust
fn fmt_state_indicator<A: Automaton>(f: &mut core::fmt::Formatter<'_>, aut: A, id: StateID) -> core::fmt::Result
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1558-1577`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1558-L1577)*

Write a prefix "state" indicator for fmt::Debug impls. It always writes
exactly two printable bytes to the given formatter.

Specifically, this tries to succinctly distinguish the different types of
states: dead states, start states and match states. It even accounts for
the possible overlappings of different state types. (The only possible
overlapping is that of match and start states.)

### `sparse_transitions`

```rust
fn sparse_transitions<'a>(it: impl Iterator<Item = (u8, StateID)> + 'a) -> impl Iterator<Item = (u8, u8, StateID)> + 'a
```

*Defined in [`aho-corasick-1.1.4/src/automaton.rs:1583-1608`](../../../.source_1765210505/aho-corasick-1.1.4/src/automaton.rs#L1583-L1608)*

Return an iterator of transitions in a sparse format given an iterator
of all explicitly defined transitions. The iterator yields ranges of
transitions, such that any adjacent transitions mapped to the same
state are combined into a single range.

