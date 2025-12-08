*[aho_corasick](../index.md) / [dfa](index.md)*

---

# Module `dfa`

Provides direct access to a DFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a DFA directly. Using an `DFA` directly is typically only necessary
when one needs access to the [`Automaton`](../automaton/index.md) trait implementation.

## Structs

### `DFA`

```rust
struct DFA {
    trans: alloc::vec::Vec<crate::util::primitives::StateID>,
    matches: alloc::vec::Vec<alloc::vec::Vec<crate::util::primitives::PatternID>>,
    matches_memory_usage: usize,
    pattern_lens: alloc::vec::Vec<crate::util::primitives::SmallIndex>,
    prefilter: Option<crate::util::prefilter::Prefilter>,
    match_kind: crate::util::search::MatchKind,
    state_len: usize,
    alphabet_len: usize,
    stride2: usize,
    byte_classes: crate::util::alphabet::ByteClasses,
    min_pattern_len: usize,
    max_pattern_len: usize,
    special: crate::util::special::Special,
}
```

A DFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using a `DFA` directly is typically only necessary when
one needs access to the [`Automaton`](../automaton/index.md) trait implementation.

This DFA can only be built by first constructing a `noncontiguous::NFA`.
Both `DFA::new` and `Builder::build` do this for you automatically, but
`Builder::build_from_noncontiguous` permits doing it explicitly.

A DFA provides the best possible search performance (in this crate) via two
mechanisms:

* All states use a dense representation for their transitions.
* All failure transitions are pre-computed such that they are never
explicitly handled at search time.

These two facts combined mean that every state transition is performed
using a constant number of instructions. However, this comes at
great cost. The memory usage of a DFA can be quite exorbitant.
It is potentially multiple orders of magnitude greater than a
[`contiguous::NFA`](crate::nfa::contiguous::NFA) for example. In exchange,
a DFA will typically have better search speed than a `contiguous::NFA`, but
not by orders of magnitude.

Unless you have a small number of patterns or memory usage is not a concern
and search performance is critical, a DFA is usually not the best choice.

Moreover, unlike the NFAs in this crate, it is costly for a DFA to
support for anchored and unanchored search configurations. Namely,
since failure transitions are pre-computed, supporting both anchored
and unanchored searches requires a duplication of the transition table,
making the memory usage of such a DFA ever bigger. (The NFAs in this crate
unconditionally support both anchored and unanchored searches because there
is essentially no added cost for doing so.) It is for this reason that
a DFA's support for anchored and unanchored searches can be configured
via `Builder::start_kind`. By default, a DFA only supports unanchored
searches.

# Example

This example shows how to build an `DFA` directly and use it to execute

use aho_corasick::{
    automaton::Automaton,
    dfa::DFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = DFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```rust

It is also possible to implement your own version of `try_find`. See the
[`Automaton`](../automaton/index.md) documentation for an example.

#### Fields

- **`trans`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The DFA transition table. IDs in this table are pre-multiplied. So
  instead of the IDs being 0, 1, 2, 3, ..., they are 0*stride, 1*stride,
  2*stride, 3*stride, ...

- **`matches`**: `alloc::vec::Vec<alloc::vec::Vec<crate::util::primitives::PatternID>>`

  The matches for every match state in this DFA. This is first indexed by
  state index (so that's `sid >> stride2`) and then by order in which the
  matches are meant to occur.

- **`matches_memory_usage`**: `usize`

  The amount of heap memory used, in bytes, by the inner Vecs of
  'matches'.

- **`pattern_lens`**: `alloc::vec::Vec<crate::util::primitives::SmallIndex>`

  The length of each pattern. This is used to compute the start offset
  of a match.

- **`prefilter`**: `Option<crate::util::prefilter::Prefilter>`

  A prefilter for accelerating searches, if one exists.

- **`match_kind`**: `crate::util::search::MatchKind`

  The match semantics built into this DFA.

- **`state_len`**: `usize`

  The total number of states in this DFA.

- **`alphabet_len`**: `usize`

  The alphabet size, or total number of equivalence classes, for this
  DFA. Note that the actual number of transitions in each state is
  stride=2^stride2, where stride is the smallest power of 2 greater than
  or equal to alphabet_len. We do things this way so that we can use
  bitshifting to go from a state ID to an index into 'matches'.

- **`stride2`**: `usize`

  The exponent with a base 2, such that stride=2^stride2. Given a state
  index 'i', its state identifier is 'i << stride2'. Given a state
  identifier 'sid', its state index is 'sid >> stride2'.

- **`byte_classes`**: `crate::util::alphabet::ByteClasses`

  The equivalence classes for this DFA. All transitions are defined on
  equivalence classes and not on the 256 distinct byte values.

- **`min_pattern_len`**: `usize`

  The length of the shortest pattern in this automaton.

- **`max_pattern_len`**: `usize`

  The length of the longest pattern in this automaton.

- **`special`**: `crate::util::special::Special`

  The information required to deduce which states are "special" in this
  DFA.

#### Implementations

- `fn new<I, P>(patterns: I) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../util/error/index.md)

- `fn builder() -> Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Automaton for DFA`

- `fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError>` — [`Anchored`](../util/search/index.md), [`StateID`](../util/primitives/index.md), [`MatchError`](../util/error/index.md)

- `fn next_state(self: &Self, _anchored: Anchored, sid: StateID, byte: u8) -> StateID` — [`Anchored`](../util/search/index.md), [`StateID`](../util/primitives/index.md)

- `fn is_special(self: &Self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md)

- `fn is_dead(self: &Self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md)

- `fn is_match(self: &Self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md)

- `fn is_start(self: &Self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md)

- `fn match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../util/search/index.md)

- `fn patterns_len(self: &Self) -> usize`

- `fn pattern_len(self: &Self, pid: PatternID) -> usize` — [`PatternID`](../util/primitives/index.md)

- `fn min_pattern_len(self: &Self) -> usize`

- `fn max_pattern_len(self: &Self) -> usize`

- `fn match_len(self: &Self, sid: StateID) -> usize` — [`StateID`](../util/primitives/index.md)

- `fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID` — [`StateID`](../util/primitives/index.md), [`PatternID`](../util/primitives/index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../util/prefilter/index.md)

##### `impl Clone for DFA`

- `fn clone(self: &Self) -> DFA` — [`DFA`](#dfa)

##### `impl Debug for DFA`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Sealed for crate::dfa::DFA`

### `Builder`

```rust
struct Builder {
    noncontiguous: noncontiguous::Builder,
    start_kind: crate::util::search::StartKind,
    byte_classes: bool,
}
```

A builder for configuring an Aho-Corasick DFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn build<I, P>(self: &Self, patterns: I) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../util/error/index.md)

- `fn build_from_noncontiguous(self: &Self, nnfa: &noncontiguous::NFA) -> Result<DFA, BuildError>` — [`NFA`](../nfa/noncontiguous/index.md), [`DFA`](#dfa), [`BuildError`](../util/error/index.md)

- `fn finish_build_one_start(self: &Self, anchored: Anchored, nnfa: &noncontiguous::NFA, dfa: &mut DFA)` — [`Anchored`](../util/search/index.md), [`NFA`](../nfa/noncontiguous/index.md), [`DFA`](#dfa)

- `fn finish_build_both_starts(self: &Self, nnfa: &noncontiguous::NFA, dfa: &mut DFA)` — [`NFA`](../nfa/noncontiguous/index.md), [`DFA`](#dfa)

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../util/search/index.md), [`Builder`](#builder)

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn prefilter(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn start_kind(self: &mut Self, kind: StartKind) -> &mut Builder` — [`StartKind`](../util/search/index.md), [`Builder`](#builder)

- `fn byte_classes(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](#builder)

