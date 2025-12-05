*[aho_corasick](../../index.md) / [nfa](../index.md) / [noncontiguous](index.md)*

---

# Module `noncontiguous`

Provides a noncontiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a noncontiguous NFA directly. Using an `NFA` directly is typically
only necessary when one needs access to the [`Automaton`](../../automaton/index.md) trait implementation.

## Structs

### `NFA`

```rust
struct NFA {
    match_kind: crate::util::search::MatchKind,
    states: alloc::vec::Vec<State>,
    sparse: alloc::vec::Vec<Transition>,
    dense: alloc::vec::Vec<crate::util::primitives::StateID>,
    matches: alloc::vec::Vec<Match>,
    pattern_lens: alloc::vec::Vec<crate::util::primitives::SmallIndex>,
    prefilter: Option<crate::util::prefilter::Prefilter>,
    byte_classes: crate::util::alphabet::ByteClasses,
    min_pattern_len: usize,
    max_pattern_len: usize,
    special: crate::util::special::Special,
}
```

A noncontiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`](../../automaton/index.md) trait implementation.

This NFA represents the "core" implementation of Aho-Corasick in this
crate. Namely, constructing this NFA involving building a trie and then
filling in the failure transitions between states, similar to what is
described in any standard textbook description of Aho-Corasick.

In order to minimize heap usage and to avoid additional construction costs,
this implementation represents the transitions of all states as distinct
sparse memory allocations. This is where it gets its name from. That is,
this NFA has no contiguous memory allocation for its transition table. Each
state gets its own allocation.

While the sparse representation keeps memory usage to somewhat reasonable
levels, it is still quite large and also results in somewhat mediocre
search performance. For this reason, it is almost always a good idea to
use a [`contiguous::NFA`](crate::nfa::contiguous::NFA) instead. It is
marginally slower to build, but has higher throughput and can sometimes use
an order of magnitude less memory. The main reason to use a noncontiguous
NFA is when you need the fastest possible construction time, or when a
contiguous NFA does not have the desired capacity. (The total number of NFA
states it can have is fewer than a noncontiguous NFA.)

# Example

This example shows how to build an `NFA` directly and use it to execute

use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
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

- **`match_kind`**: `crate::util::search::MatchKind`

  The match semantics built into this NFA.

- **`states`**: `alloc::vec::Vec<State>`

  A set of states. Each state defines its own transitions, a fail
  transition and a set of indices corresponding to matches.
  
  The first state is always the fail state, which is used only as a
  sentinel. Namely, in the final NFA, no transition into the fail state
  exists. (Well, they do, but they aren't followed. Instead, the state's
  failure transition is followed.)
  
  The second state (index 1) is always the dead state. Dead states are
  in every automaton, but only used when leftmost-{first,longest} match
  semantics are enabled. Specifically, they instruct search to stop
  at specific points in order to report the correct match location. In
  the standard Aho-Corasick construction, there are no transitions to
  the dead state.
  
  The third state (index 2) is generally intended to be the starting or
  "root" state.

- **`sparse`**: `alloc::vec::Vec<Transition>`

  Transitions stored in a sparse representation via a linked list.
  
  Each transition contains three pieces of information: the byte it
  is defined for, the state it transitions to and a link to the next
  transition in the same state (or `StateID::ZERO` if it is the last
  transition).
  
  The first transition for each state is determined by `State::sparse`.
  
  Note that this contains a complete set of all transitions in this NFA,
  including states that have a dense representation for transitions.
  (Adding dense transitions for a state doesn't remove its sparse
  transitions, since deleting transitions from this particular sparse
  representation would be fairly expensive.)

- **`dense`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  Transitions stored in a dense representation.
  
  A state has a row in this table if and only if `State::dense` is
  not equal to `StateID::ZERO`. When not zero, there are precisely
  `NFA::byte_classes::alphabet_len()` entries beginning at `State::dense`
  in this table.
  
  Generally a very small minority of states have a dense representation
  since it uses so much memory.

- **`matches`**: `alloc::vec::Vec<Match>`

  Matches stored in linked list for each state.
  
  Like sparse transitions, each match has a link to the next match in the
  state.
  
  The first match for each state is determined by `State::matches`.

- **`pattern_lens`**: `alloc::vec::Vec<crate::util::primitives::SmallIndex>`

  The length, in bytes, of each pattern in this NFA. This slice is
  indexed by `PatternID`.
  
  The number of entries in this vector corresponds to the total number of
  patterns in this automaton.

- **`prefilter`**: `Option<crate::util::prefilter::Prefilter>`

  A prefilter for quickly skipping to candidate matches, if pertinent.

- **`byte_classes`**: `crate::util::alphabet::ByteClasses`

  A set of equivalence classes in terms of bytes. We compute this while
  building the NFA, but don't use it in the NFA's states. Instead, we
  use this for building the DFA. We store it on the NFA since it's easy
  to compute while visiting the patterns.

- **`min_pattern_len`**: `usize`

  The length, in bytes, of the shortest pattern in this automaton. This
  information is useful for detecting whether an automaton matches the
  empty string or not.

- **`max_pattern_len`**: `usize`

  The length, in bytes, of the longest pattern in this automaton. This
  information is useful for keeping correct buffer sizes when searching
  on streams.

- **`special`**: `crate::util::special::Special`

  The information required to deduce which states are "special" in this
  NFA.
  
  Since the DEAD and FAIL states are always the first two states and
  there are only ever two start states (which follow all of the match
  states), it follows that we can determine whether a state is a fail,
  dead, match or start with just a few comparisons on the ID itself:
  
     is_dead(sid): sid == NFA::DEAD
     is_fail(sid): sid == NFA::FAIL
    is_match(sid): NFA::FAIL < sid && sid <= max_match_id
    is_start(sid): sid == start_unanchored_id || sid == start_anchored_id
  
  Note that this only applies to the NFA after it has been constructed.
  During construction, the start states are the first ones added and the
  match states are inter-leaved with non-match states. Once all of the
  states have been added, the states are shuffled such that the above
  predicates hold.

#### Implementations

- `const DEAD: StateID`

- `const FAIL: StateID`

- `fn byte_classes(self: &Self) -> &ByteClasses` — [`ByteClasses`](../../../util/alphabet/index.md)

- `fn pattern_lens_raw(self: &Self) -> &[SmallIndex]` — [`SmallIndex`](../../../util/primitives/index.md)

- `fn states(self: &Self) -> &[State]` — [`State`](../../../nfa/noncontiguous/index.md)

- `fn special(self: &Self) -> &Special` — [`Special`](../../../util/special/index.md)

- `fn swap_states(self: &mut Self, id1: StateID, id2: StateID)` — [`StateID`](../../../util/primitives/index.md)

- `fn remap(self: &mut Self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../../util/primitives/index.md)

- `fn iter_trans(self: &Self, sid: StateID) -> impl Iterator<Item = Transition> + '_` — [`StateID`](../../../util/primitives/index.md), [`Transition`](../../../nfa/noncontiguous/index.md)

- `fn iter_matches(self: &Self, sid: StateID) -> impl Iterator<Item = PatternID> + '_` — [`StateID`](../../../util/primitives/index.md), [`PatternID`](../../../util/primitives/index.md)

- `fn next_link(self: &Self, sid: StateID, prev: Option<StateID>) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md)

- `fn follow_transition(self: &Self, sid: StateID, byte: u8) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn follow_transition_sparse(self: &Self, sid: StateID, byte: u8) -> StateID` — [`StateID`](../../../util/primitives/index.md)

- `fn add_transition(self: &mut Self, prev: StateID, byte: u8, next: StateID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn init_full_state(self: &mut Self, prev: StateID, next: StateID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn add_match(self: &mut Self, sid: StateID, pid: PatternID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md), [`PatternID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn copy_matches(self: &mut Self, src: StateID, dst: StateID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn alloc_transition(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn alloc_match(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn alloc_dense_state(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

- `fn alloc_state(self: &mut Self, depth: usize) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../../../util/error/index.md)

#### Trait Implementations

##### `impl Automaton`

- `fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError>` — [`Anchored`](../../../util/search/index.md), [`StateID`](../../../util/primitives/index.md), [`MatchError`](../../../util/error/index.md)

- `fn next_state(self: &Self, anchored: Anchored, sid: StateID, byte: u8) -> StateID` — [`Anchored`](../../../util/search/index.md), [`StateID`](../../../util/primitives/index.md)

- `fn is_special(self: &Self, sid: StateID) -> bool` — [`StateID`](../../../util/primitives/index.md)

- `fn is_dead(self: &Self, sid: StateID) -> bool` — [`StateID`](../../../util/primitives/index.md)

- `fn is_match(self: &Self, sid: StateID) -> bool` — [`StateID`](../../../util/primitives/index.md)

- `fn is_start(self: &Self, sid: StateID) -> bool` — [`StateID`](../../../util/primitives/index.md)

- `fn match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../../../util/search/index.md)

- `fn patterns_len(self: &Self) -> usize`

- `fn pattern_len(self: &Self, pid: PatternID) -> usize` — [`PatternID`](../../../util/primitives/index.md)

- `fn min_pattern_len(self: &Self) -> usize`

- `fn max_pattern_len(self: &Self) -> usize`

- `fn match_len(self: &Self, sid: StateID) -> usize` — [`StateID`](../../../util/primitives/index.md)

- `fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID` — [`StateID`](../../../util/primitives/index.md), [`PatternID`](../../../util/primitives/index.md)

- `fn memory_usage(self: &Self) -> usize`

- `fn prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md)

##### `impl Clone`

- `fn clone(self: &Self) -> NFA` — [`NFA`](../../../nfa/noncontiguous/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Remappable`

- `fn state_len(self: &Self) -> usize`

- `fn swap_states(self: &mut Self, id1: StateID, id2: StateID)` — [`StateID`](../../../util/primitives/index.md)

- `fn remap(self: &mut Self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../../util/primitives/index.md)

##### `impl Sealed`

### `Builder`

```rust
struct Builder {
    match_kind: crate::util::search::MatchKind,
    prefilter: bool,
    ascii_case_insensitive: bool,
    dense_depth: usize,
}
```

A builder for configuring an Aho-Corasick noncontiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- `fn new() -> Builder` — [`Builder`](../../../nfa/noncontiguous/index.md)

- `fn build<I, P>(self: &Self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](../../../nfa/noncontiguous/index.md), [`BuildError`](../../../util/error/index.md)

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../../../util/search/index.md), [`Builder`](../../../nfa/noncontiguous/index.md)

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](../../../nfa/noncontiguous/index.md)

- `fn dense_depth(self: &mut Self, depth: usize) -> &mut Builder` — [`Builder`](../../../nfa/noncontiguous/index.md)

- `fn prefilter(self: &mut Self, yes: bool) -> &mut Builder` — [`Builder`](../../../nfa/noncontiguous/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Builder` — [`Builder`](../../../nfa/noncontiguous/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Builder` — [`Builder`](../../../nfa/noncontiguous/index.md)

