*[aho_corasick](../../index.md) / [nfa](../index.md) / [contiguous](index.md)*

---

# Module `contiguous`

Provides a contiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a contiguous NFA directly. Using an `NFA` directly is typically only
necessary when one needs access to the [`Automaton`](../../automaton/index.md) trait implementation.

## Structs

### `NFA`

```rust
struct NFA {
    repr: alloc::vec::Vec<u32>,
    pattern_lens: alloc::vec::Vec<crate::util::primitives::SmallIndex>,
    state_len: usize,
    prefilter: Option<crate::util::prefilter::Prefilter>,
    match_kind: crate::util::search::MatchKind,
    alphabet_len: usize,
    byte_classes: crate::util::alphabet::ByteClasses,
    min_pattern_len: usize,
    max_pattern_len: usize,
    special: crate::util::special::Special,
}
```

A contiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`](../../automaton/index.md) trait implementation.

This NFA can only be built by first constructing a [`noncontiguous::NFA`](../noncontiguous/index.md).
Both `NFA::new` and `Builder::build` do this for you automatically, but
`Builder::build_from_noncontiguous` permits doing it explicitly.

The main difference between a noncontiguous NFA and a contiguous NFA is
that the latter represents all of its states and transitions in a single
allocation, where as the former uses a separate allocation for each state.
Doing this at construction time while keeping a low memory footprint isn't
feasible, which is primarily why there are two different NFA types: one
that does the least amount of work possible to build itself, and another
that does a little extra work to compact itself and make state transitions
faster by making some states use a dense representation.

Because a contiguous NFA uses a single allocation, there is a lot more
opportunity for compression tricks to reduce the heap memory used. Indeed,
it is not uncommon for a contiguous NFA to use an order of magnitude less
heap memory than a noncontiguous NFA. Since building a contiguous NFA
usually only takes a fraction of the time it takes to build a noncontiguous
NFA, the overall build time is not much slower. Thus, in most cases, a
contiguous NFA is the best choice.

Since a contiguous NFA uses various tricks for compression and to achieve
faster state transitions, currently, its limit on the number of states
is somewhat smaller than what a noncontiguous NFA can achieve. Generally
speaking, you shouldn't expect to run into this limit if the number of
patterns is under 1 million. It is plausible that this limit will be
increased in the future. If the limit is reached, building a contiguous NFA
will return an error. Often, since building a contiguous NFA is relatively
cheap, it can make sense to always try it even if you aren't sure if it
will fail or not. If it does, you can always fall back to a noncontiguous
NFA. (Indeed, the main [`AhoCorasick`](crate::AhoCorasick) type employs a
strategy similar to this at construction time.)

# Example

This example shows how to build an `NFA` directly and use it to execute

use aho_corasick::{
    automaton::Automaton,
    nfa::contiguous::NFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = NFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```rust

It is also possible to implement your own version of `try_find`. See the
[`Automaton`](../../automaton/index.md) documentation for an example.

#### Fields

- **`repr`**: `alloc::vec::Vec<u32>`

  The raw NFA representation. Each state is packed with a header
  (containing the format of the state, the failure transition and, for
  a sparse state, the number of transitions), its transitions and any
  matching pattern IDs for match states.

- **`pattern_lens`**: `alloc::vec::Vec<crate::util::primitives::SmallIndex>`

  The length of each pattern. This is used to compute the start offset
  of a match.

- **`state_len`**: `usize`

  The total number of states in this NFA.

- **`prefilter`**: `Option<crate::util::prefilter::Prefilter>`

  A prefilter for accelerating searches, if one exists.

- **`match_kind`**: `crate::util::search::MatchKind`

  The match semantics built into this NFA.

- **`alphabet_len`**: `usize`

  The alphabet size, or total number of equivalence classes, for this
  NFA. Dense states always have this many transitions.

- **`byte_classes`**: `crate::util::alphabet::ByteClasses`

  The equivalence classes for this NFA. All transitions, dense and
  sparse, are defined on equivalence classes and not on the 256 distinct
  byte values.

- **`min_pattern_len`**: `usize`

  The length of the shortest pattern in this automaton.

- **`max_pattern_len`**: `usize`

  The length of the longest pattern in this automaton.

- **`special`**: `crate::util::special::Special`

  The information required to deduce which states are "special" in this
  NFA.

#### Implementations

- `fn new<I, P>(patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md)

- `fn builder() -> Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Automaton for NFA`

- `fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError>` — [`Anchored`](../../util/search/index.md), [`StateID`](../../util/primitives/index.md), [`MatchError`](../../util/error/index.md)

- `fn next_state(self: &Self, anchored: Anchored, sid: StateID, byte: u8) -> StateID` — [`Anchored`](../../util/search/index.md), [`StateID`](../../util/primitives/index.md)

- `fn is_special(self: &Self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_dead(self: &Self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_match(self: &Self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_start(self: &Self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../../util/search/index.md)

- `fn patterns_len(self: &Self) -> usize`

- `fn pattern_len(self: &Self, pid: PatternID) -> usize` — [`PatternID`](../../util/primitives/index.md)

- `fn min_pattern_len(self: &Self) -> usize`

- `fn max_pattern_len(self: &Self) -> usize`

- `fn match_len(self: &Self, sid: StateID) -> usize` — [`StateID`](../../util/primitives/index.md)

- `fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md)

##### `impl Clone for NFA`

- `fn clone(self: &Self) -> NFA` — [`NFA`](#nfa)

##### `impl Debug for NFA`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Sealed for crate::nfa::contiguous::NFA`

### `State<'a>`

```rust
struct State<'a> {
    fail: crate::util::primitives::StateID,
    match_len: usize,
    trans: StateTrans<'a>,
}
```

The "in memory" representation a single dense or sparse state.

A `State`'s in memory representation is not ever actually materialized
during a search with a contiguous NFA. Doing so would be too slow. (Indeed,
the only time a `State` is actually constructed is in `Debug` impls.)
Instead, a `State` exposes a number of static methods for reading certain
things from the raw binary encoding of the state.

#### Fields

- **`fail`**: `crate::util::primitives::StateID`

  The state to transition to when 'class_to_next' yields a transition
  to the FAIL state.

- **`match_len`**: `usize`

  The number of pattern IDs in this state. For a non-match state, this is
  always zero. Otherwise it is always bigger than zero.

- **`trans`**: `StateTrans<'a>`

  The sparse or dense representation of the transitions for this state.

#### Implementations

- `const KIND: usize`

- `const KIND_DENSE: u32`

- `const KIND_ONE: u32`

- `const MAX_SPARSE_TRANSITIONS: usize`

- `fn remap(alphabet_len: usize, old_to_new: &[StateID], state: &mut [u32]) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](../../util/error/index.md)

- `fn len(alphabet_len: usize, is_match: bool, state: &[u32]) -> usize`

- `fn kind(state: &[u32]) -> u32`

- `fn sparse_trans_len(state: &[u32]) -> usize`

- `fn match_len(alphabet_len: usize, state: &[u32]) -> usize`

- `fn match_pattern(alphabet_len: usize, state: &[u32], index: usize) -> PatternID` — [`PatternID`](../../util/primitives/index.md)

- `fn read(alphabet_len: usize, is_match: bool, state: &'a [u32]) -> State<'a>` — [`State`](#state)

- `fn write(nnfa: &noncontiguous::NFA, oldsid: StateID, old: &noncontiguous::State, classes: &ByteClasses, dst: &mut Vec<u32>, force_dense: bool) -> Result<StateID, BuildError>` — [`NFA`](../noncontiguous/index.md), [`StateID`](../../util/primitives/index.md), [`State`](../noncontiguous/index.md), [`ByteClasses`](../../util/alphabet/index.md), [`BuildError`](../../util/error/index.md)

- `fn write_sparse_trans(nnfa: &noncontiguous::NFA, oldsid: StateID, classes: &ByteClasses, dst: &mut Vec<u32>) -> Result<(), BuildError>` — [`NFA`](../noncontiguous/index.md), [`StateID`](../../util/primitives/index.md), [`ByteClasses`](../../util/alphabet/index.md), [`BuildError`](../../util/error/index.md)

- `fn write_dense_trans(nnfa: &noncontiguous::NFA, oldsid: StateID, classes: &ByteClasses, dst: &mut Vec<u32>) -> Result<(), BuildError>` — [`NFA`](../noncontiguous/index.md), [`StateID`](../../util/primitives/index.md), [`ByteClasses`](../../util/alphabet/index.md), [`BuildError`](../../util/error/index.md)

- `fn transitions(self: &Self) -> impl Iterator<Item = (u8, StateID)> + '_` — [`StateID`](../../util/primitives/index.md)

#### Trait Implementations

##### `impl<'a> Clone for State<'a>`

- `fn clone(self: &Self) -> State<'a>` — [`State`](#state)

##### `impl<'a> Debug for State<'a>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Builder`

```rust
struct Builder {
    noncontiguous: noncontiguous::Builder,
    dense_depth: usize,
    byte_classes: bool,
}
```

A builder for configuring an Aho-Corasick contiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn build<I, P>(self: &Self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md)

- `fn build_from_noncontiguous(self: &Self, nnfa: &noncontiguous::NFA) -> Result<NFA, BuildError>` — [`NFA`](../noncontiguous/index.md), [`BuildError`](../../util/error/index.md)

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../../util/search/index.md), [`Builder`](#builder)

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn prefilter(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- `fn dense_depth(self: &mut Self, depth: usize) -> &mut Builder` — [`Builder`](#builder)

- `fn byte_classes(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](#builder)

## Enums

### `StateTrans<'a>`

```rust
enum StateTrans<'a> {
    Sparse {
        classes: &'a [u32],
        nexts: &'a [u32],
    },
    One {
        class: u8,
        next: u32,
    },
    Dense {
        class_to_next: &'a [u32],
    },
}
```

The underlying representation of sparse or dense transitions for a state.

Note that like `State`, we don't typically construct values of this type
during a search since we don't always need all values and thus would
represent a lot of wasteful work.

#### Variants

- **`Sparse`**

  A sparse representation of transitions for a state, where only non-FAIL
  transitions are explicitly represented.

- **`One`**

  A "one transition" state that is never a match state.
  
  These are by far the most common state, so we use a specialized and
  very compact representation for them.

- **`Dense`**

  A dense representation of transitions for a state, where all
  transitions are explicitly represented, including transitions to the
  FAIL state.

#### Trait Implementations

##### `impl<'a> Clone for StateTrans<'a>`

- `fn clone(self: &Self) -> StateTrans<'a>` — [`StateTrans`](#statetrans)

## Functions

### `u32_len`

```rust
fn u32_len(ntrans: usize) -> usize
```

Computes the number of u32 values needed to represent one byte per the
number of transitions given.

