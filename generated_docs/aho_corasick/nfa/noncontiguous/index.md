*[aho_corasick](../../index.md) / [nfa](../index.md) / [noncontiguous](index.md)*

---

# Module `noncontiguous`

Provides a noncontiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a noncontiguous NFA directly. Using an `NFA` directly is typically
only necessary when one needs access to the [`Automaton`](../../automaton/index.md) trait implementation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NFA`](#nfa) | struct | A noncontiguous NFA implementation of Aho-Corasick. |
| [`State`](#state) | struct | A representation of a sparse NFA state for an Aho-Corasick automaton. |
| [`Transition`](#transition) | struct | A single transition in a non-contiguous NFA. |
| [`Match`](#match) | struct | A single match in a non-contiguous NFA. |
| [`Builder`](#builder) | struct | A builder for configuring an Aho-Corasick noncontiguous NFA. |
| [`Compiler`](#compiler) | struct | A compiler uses a builder configuration and builds up the NFA formulation of an Aho-Corasick automaton. |
| [`QueuedSet`](#queuedset) | struct | A set of state identifiers used to avoid revisiting the same state multiple times when filling in failure transitions. |

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

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:82-175`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L82-L175)*

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

- <span id="nfa-new"></span>`fn new<I, P>(patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

- <span id="nfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Automaton for NFA`

- <span id="nfa-start-state"></span>`fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>` — [`Anchored`](../../util/search/index.md#anchored), [`StateID`](../../util/primitives/index.md#stateid), [`MatchError`](../../util/error/index.md#matcherror)

- <span id="nfa-next-state"></span>`fn next_state(&self, anchored: Anchored, sid: StateID, byte: u8) -> StateID` — [`Anchored`](../../util/search/index.md#anchored), [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-is-special"></span>`fn is_special(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-is-dead"></span>`fn is_dead(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-is-match"></span>`fn is_match(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-is-start"></span>`fn is_start(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-match-kind"></span>`fn match_kind(&self) -> MatchKind` — [`MatchKind`](../../util/search/index.md#matchkind)

- <span id="nfa-patterns-len"></span>`fn patterns_len(&self) -> usize`

- <span id="nfa-pattern-len"></span>`fn pattern_len(&self, pid: PatternID) -> usize` — [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="nfa-min-pattern-len"></span>`fn min_pattern_len(&self) -> usize`

- <span id="nfa-max-pattern-len"></span>`fn max_pattern_len(&self) -> usize`

- <span id="nfa-match-len"></span>`fn match_len(&self, sid: StateID) -> usize` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-match-pattern"></span>`fn match_pattern(&self, sid: StateID, index: usize) -> PatternID` — [`StateID`](../../util/primitives/index.md#stateid), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="nfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="nfa-prefilter"></span>`fn prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md#prefilter)

##### `impl Clone for NFA`

- <span id="nfa-clone"></span>`fn clone(&self) -> NFA` — [`NFA`](#nfa)

##### `impl Debug for NFA`

- <span id="nfa-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Remappable for noncontiguous::NFA`

- <span id="noncontiguousnfa-state-len"></span>`fn state_len(&self) -> usize`

- <span id="noncontiguousnfa-swap-states"></span>`fn swap_states(&mut self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="noncontiguousnfa-remap"></span>`fn remap(&mut self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

##### `impl Sealed for crate::nfa::noncontiguous::NFA`

### `State`

```rust
struct State {
    sparse: crate::util::primitives::StateID,
    dense: crate::util::primitives::StateID,
    matches: crate::util::primitives::StateID,
    fail: crate::util::primitives::StateID,
    depth: crate::util::primitives::SmallIndex,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:710-748`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L710-L748)*

A representation of a sparse NFA state for an Aho-Corasick automaton.

It contains the transitions to the next state, a failure transition for
cases where there exists no other transition for the current input byte
and the matches implied by visiting this state (if any).

#### Fields

- **`sparse`**: `crate::util::primitives::StateID`

  A pointer to `NFA::trans` corresponding to the head of a linked list
  containing all of the transitions for this state.
  
  This is `StateID::ZERO` if and only if this state has zero transitions.

- **`dense`**: `crate::util::primitives::StateID`

  A pointer to a row of `N` transitions in `NFA::dense`. These
  transitions correspond precisely to what is obtained by traversing
  `sparse`, but permits constant time lookup.
  
  When this is zero (which is true for most states in the default
  configuration), then this state has no dense representation.
  
  Note that `N` is equal to `NFA::byte_classes::alphabet_len()`. This is
  typically much less than 256 (the maximum value).

- **`matches`**: `crate::util::primitives::StateID`

  A pointer to `NFA::matches` corresponding to the head of a linked list
  containing all of the matches for this state.
  
  This is `StateID::ZERO` if and only if this state is not a match state.

- **`fail`**: `crate::util::primitives::StateID`

  The state that should be transitioned to if the current byte in the
  haystack does not have a corresponding transition defined in this
  state.

- **`depth`**: `crate::util::primitives::SmallIndex`

  The depth of this state. Specifically, this is the distance from this
  state to the starting state. (For the special sentinel states DEAD and
  FAIL, their depth is always 0.) The depth of a starting state is 0.
  
  Note that depth is currently not used in this non-contiguous NFA. It
  may in the future, but it is used in the contiguous NFA. Namely, it
  permits an optimization where states near the starting state have their
  transitions stored in a dense fashion, but all other states have their
  transitions stored in a sparse fashion. (This non-contiguous NFA uses
  a sparse representation for all states unconditionally.) In any case,
  this is really the only convenient place to compute and store this
  information, which we need when building the contiguous NFA.

#### Implementations

- <span id="state-is-match"></span>`fn is_match(&self) -> bool`

- <span id="state-fail"></span>`fn fail(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="state-depth"></span>`fn depth(&self) -> SmallIndex` — [`SmallIndex`](../../util/primitives/index.md#smallindex)

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl Debug for State`

- <span id="state-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Transition`

```rust
struct Transition {
    byte: u8,
    next: crate::util::primitives::StateID,
    link: crate::util::primitives::StateID,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:771-775`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L771-L775)*

A single transition in a non-contiguous NFA.

#### Implementations

- <span id="transition-byte"></span>`fn byte(&self) -> u8`

- <span id="transition-next"></span>`fn next(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="transition-link"></span>`fn link(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

#### Trait Implementations

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](#transition)

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- <span id="transition-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Transition`

- <span id="transition-default"></span>`fn default() -> Transition` — [`Transition`](#transition)

### `Match`

```rust
struct Match {
    pid: crate::util::primitives::PatternID,
    link: crate::util::primitives::StateID,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:808-811`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L808-L811)*

A single match in a non-contiguous NFA.

#### Implementations

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="match-link"></span>`fn link(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

#### Trait Implementations

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](#match)

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Match`

- <span id="match-default"></span>`fn default() -> Match` — [`Match`](#match)

### `Builder`

```rust
struct Builder {
    match_kind: crate::util::search::MatchKind,
    prefilter: bool,
    ascii_case_insensitive: bool,
    dense_depth: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:842-847`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L842-L847)*

A builder for configuring an Aho-Corasick noncontiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

- <span id="builder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

- <span id="builder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../../util/search/index.md#matchkind), [`Builder`](#builder)

- <span id="builder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-dense-depth"></span>`fn dense_depth(&mut self, depth: usize) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-prefilter"></span>`fn prefilter(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

### `Compiler<'a>`

```rust
struct Compiler<'a> {
    builder: &'a Builder,
    prefilter: prefilter::Builder,
    nfa: NFA,
    byteset: crate::util::alphabet::ByteClassSet,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:932-937`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L932-L937)*

A compiler uses a builder configuration and builds up the NFA formulation
of an Aho-Corasick automaton. This roughly corresponds to the standard
formulation described in textbooks, with some tweaks to support leftmost
searching.

#### Implementations

- <span id="compiler-new"></span>`fn new(builder: &'a Builder) -> Result<Compiler<'a>, BuildError>` — [`Builder`](#builder), [`Compiler`](#compiler), [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-compile"></span>`fn compile<I, P>(self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-build-trie"></span>`fn build_trie<I, P>(&mut self, patterns: I) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-fill-failure-transitions"></span>`fn fill_failure_transitions(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-shuffle"></span>`fn shuffle(&mut self)`

- <span id="compiler-densify"></span>`fn densify(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-queued-set"></span>`fn queued_set(&self) -> QueuedSet` — [`QueuedSet`](#queuedset)

- <span id="compiler-init-unanchored-start-state"></span>`fn init_unanchored_start_state(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-set-anchored-start-state"></span>`fn set_anchored_start_state(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-add-unanchored-start-state-loop"></span>`fn add_unanchored_start_state_loop(&mut self)`

- <span id="compiler-close-start-state-loop-for-leftmost"></span>`fn close_start_state_loop_for_leftmost(&mut self)`

- <span id="compiler-add-dead-state-loop"></span>`fn add_dead_state_loop(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

#### Trait Implementations

##### `impl Debug for Compiler<'a>`

- <span id="compiler-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `QueuedSet`

```rust
struct QueuedSet {
    set: Option<alloc::collections::BTreeSet<crate::util::primitives::StateID>>,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:1657-1659`](../../../../.source_1765210505/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L1657-L1659)*

A set of state identifiers used to avoid revisiting the same state multiple
times when filling in failure transitions.

This set has an "inert" and an "active" mode. When inert, the set never
stores anything and always returns `false` for every member test. This is
useful to avoid the performance and memory overhead of maintaining this
set when it is not needed.

#### Implementations

- <span id="queuedset-inert"></span>`fn inert() -> QueuedSet` — [`QueuedSet`](#queuedset)

- <span id="queuedset-active"></span>`fn active() -> QueuedSet` — [`QueuedSet`](#queuedset)

- <span id="queuedset-insert"></span>`fn insert(&mut self, state_id: StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="queuedset-contains"></span>`fn contains(&self, state_id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

#### Trait Implementations

##### `impl Debug for QueuedSet`

- <span id="queuedset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

