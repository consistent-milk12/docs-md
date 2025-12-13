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

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:82-175`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L82-L175)*

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

  Create a new Aho-Corasick noncontiguous NFA using the default

  configuration.

  

  Use a [`Builder`](#builder) if you want to change the configuration.

- <span id="nfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  A convenience method for returning a new Aho-Corasick noncontiguous NFA

  builder.

  

  This usually permits one to just import the `NFA` type.

#### Trait Implementations

##### `impl Any for NFA`

- <span id="nfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Automaton for NFA`

- <span id="nfa-automaton-start-state"></span>`fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>` — [`Anchored`](../../util/search/index.md#anchored), [`StateID`](../../util/primitives/index.md#stateid), [`MatchError`](../../util/error/index.md#matcherror)

- <span id="nfa-automaton-next-state"></span>`fn next_state(&self, anchored: Anchored, sid: StateID, byte: u8) -> StateID` — [`Anchored`](../../util/search/index.md#anchored), [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-automaton-is-special"></span>`fn is_special(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-automaton-is-dead"></span>`fn is_dead(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-automaton-is-match"></span>`fn is_match(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-automaton-is-start"></span>`fn is_start(&self, sid: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-automaton-match-kind"></span>`fn match_kind(&self) -> MatchKind` — [`MatchKind`](../../util/search/index.md#matchkind)

- <span id="nfa-automaton-patterns-len"></span>`fn patterns_len(&self) -> usize`

- <span id="nfa-automaton-pattern-len"></span>`fn pattern_len(&self, pid: PatternID) -> usize` — [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="nfa-automaton-min-pattern-len"></span>`fn min_pattern_len(&self) -> usize`

- <span id="nfa-automaton-max-pattern-len"></span>`fn max_pattern_len(&self) -> usize`

- <span id="nfa-automaton-match-len"></span>`fn match_len(&self, sid: StateID) -> usize` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="nfa-automaton-match-pattern"></span>`fn match_pattern(&self, sid: StateID, index: usize) -> PatternID` — [`StateID`](../../util/primitives/index.md#stateid), [`PatternID`](../../util/primitives/index.md#patternid)

- <span id="nfa-automaton-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="nfa-automaton-prefilter"></span>`fn prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md#prefilter)

##### `impl<T> Borrow for NFA`

- <span id="nfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NFA`

- <span id="nfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NFA`

- <span id="nfa-clone"></span>`fn clone(&self) -> NFA` — [`NFA`](#nfa)

##### `impl CloneToUninit for NFA`

- <span id="nfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NFA`

- <span id="nfa-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for NFA`

- <span id="nfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NFA`

- <span id="nfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Remappable for noncontiguous::NFA`

- <span id="noncontiguousnfa-remappable-state-len"></span>`fn state_len(&self) -> usize`

- <span id="noncontiguousnfa-remappable-swap-states"></span>`fn swap_states(&mut self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

- <span id="noncontiguousnfa-remappable-remap"></span>`fn remap(&mut self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

##### `impl Sealed for crate::nfa::noncontiguous::NFA`

##### `impl ToOwned for NFA`

- <span id="nfa-toowned-type-owned"></span>`type Owned = T`

- <span id="nfa-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="nfa-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NFA`

- <span id="nfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NFA`

- <span id="nfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:710-748`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L710-L748)*

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

  Return true if and only if this state is a match state.

- <span id="state-fail"></span>`fn fail(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Returns the failure transition for this state.

- <span id="state-depth"></span>`fn depth(&self) -> SmallIndex` — [`SmallIndex`](../../util/primitives/index.md#smallindex)

  Returns the depth of this state. That is, the number of transitions

  this state is from the start state of the NFA.

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

##### `impl CloneToUninit for State`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for State`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for State`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Transition`

```rust
struct Transition {
    byte: u8,
    next: crate::util::primitives::StateID,
    link: crate::util::primitives::StateID,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:771-775`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L771-L775)*

A single transition in a non-contiguous NFA.

#### Implementations

- <span id="transition-byte"></span>`fn byte(&self) -> u8`

  Return the byte for which this transition is defined.

- <span id="transition-next"></span>`fn next(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Return the ID of the state that this transition points to.

- <span id="transition-link"></span>`fn link(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Return the ID of the next transition.

#### Trait Implementations

##### `impl Any for Transition`

- <span id="transition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Transition`

- <span id="transition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Transition`

- <span id="transition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](#transition)

##### `impl CloneToUninit for Transition`

- <span id="transition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- <span id="transition-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Transition`

- <span id="transition-default"></span>`fn default() -> Transition` — [`Transition`](#transition)

##### `impl<T> From for Transition`

- <span id="transition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Transition`

- <span id="transition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Transition`

- <span id="transition-toowned-type-owned"></span>`type Owned = T`

- <span id="transition-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="transition-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Transition`

- <span id="transition-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="transition-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Transition`

- <span id="transition-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="transition-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Match`

```rust
struct Match {
    pid: crate::util::primitives::PatternID,
    link: crate::util::primitives::StateID,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:808-811`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L808-L811)*

A single match in a non-contiguous NFA.

#### Implementations

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](../../util/primitives/index.md#patternid)

  Return the pattern ID for this match.

- <span id="match-link"></span>`fn link(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Return the ID of the next match.

#### Trait Implementations

##### `impl Any for Match`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](#match)

##### `impl CloneToUninit for Match`

- <span id="match-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for Match`

- <span id="match-default"></span>`fn default() -> Match` — [`Match`](#match)

##### `impl<T> From for Match`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Match`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Match`

- <span id="match-toowned-type-owned"></span>`type Owned = T`

- <span id="match-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="match-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Match`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Builder`

```rust
struct Builder {
    match_kind: crate::util::search::MatchKind,
    prefilter: bool,
    ascii_case_insensitive: bool,
    dense_depth: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:842-847`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L842-L847)*

A builder for configuring an Aho-Corasick noncontiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new builder for configuring an Aho-Corasick noncontiguous NFA.

- <span id="builder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

  Build an Aho-Corasick noncontiguous NFA from the given iterator of

  patterns.

  

  A builder may be reused to create more NFAs.

- <span id="builder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../../util/search/index.md#matchkind), [`Builder`](#builder)

  Set the desired match semantics.

  

  See

  [`AhoCorasickBuilder::match_kind`](crate::AhoCorasickBuilder::match_kind)

  for more documentation and examples.

- <span id="builder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

  Enable ASCII-aware case insensitive matching.

  

  See

  [`AhoCorasickBuilder::ascii_case_insensitive`](crate::AhoCorasickBuilder::ascii_case_insensitive)

  for more documentation and examples.

- <span id="builder-dense-depth"></span>`fn dense_depth(&mut self, depth: usize) -> &mut Builder` — [`Builder`](#builder)

  Set the limit on how many states use a dense representation for their

  transitions. Other states will generally use a sparse representation.

  

  See

  [`AhoCorasickBuilder::dense_depth`](crate::AhoCorasickBuilder::dense_depth)

  for more documentation and examples.

- <span id="builder-prefilter"></span>`fn prefilter(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

  Enable heuristic prefilter optimizations.

  

  See

  [`AhoCorasickBuilder::prefilter`](crate::AhoCorasickBuilder::prefilter)

  for more documentation and examples.

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl CloneToUninit for Builder`

- <span id="builder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

##### `impl<T> From for Builder`

- <span id="builder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Builder`

- <span id="builder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Builder`

- <span id="builder-toowned-type-owned"></span>`type Owned = T`

- <span id="builder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Builder`

- <span id="builder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Builder`

- <span id="builder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Compiler<'a>`

```rust
struct Compiler<'a> {
    builder: &'a Builder,
    prefilter: prefilter::Builder,
    nfa: NFA,
    byteset: crate::util::alphabet::ByteClassSet,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:932-937`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L932-L937)*

A compiler uses a builder configuration and builds up the NFA formulation
of an Aho-Corasick automaton. This roughly corresponds to the standard
formulation described in textbooks, with some tweaks to support leftmost
searching.

#### Implementations

- <span id="compiler-new"></span>`fn new(builder: &'a Builder) -> Result<Compiler<'a>, BuildError>` — [`Builder`](#builder), [`Compiler`](#compiler), [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-compile"></span>`fn compile<I, P>(self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

- <span id="compiler-build-trie"></span>`fn build_trie<I, P>(&mut self, patterns: I) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

  This sets up the initial prefix trie that makes up the Aho-Corasick

  automaton. Effectively, it creates the basic structure of the

  automaton, where every pattern given has a path from the start state to

  the end of the pattern.

- <span id="compiler-fill-failure-transitions"></span>`fn fill_failure_transitions(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

  This routine creates failure transitions according to the standard

  textbook formulation of the Aho-Corasick algorithm, with a couple small

  tweaks to support "leftmost" semantics.

  

  Building failure transitions is the most interesting part of building

  the Aho-Corasick automaton, because they are what allow searches to

  be performed in linear time. Specifically, a failure transition is

  a single transition associated with each state that points back to

  the longest proper suffix of the pattern being searched. The failure

  transition is followed whenever there exists no transition on the

  current state for the current input byte. If there is no other proper

  suffix, then the failure transition points back to the starting state.

  

  For example, let's say we built an Aho-Corasick automaton with the

  following patterns: 'abcd' and 'cef'. The trie looks like this:

  

  ```ignore

           a - S1 - b - S2 - c - S3 - d - S4*

          /

      S0 - c - S5 - e - S6 - f - S7*

  ```

  

  At this point, it should be fairly straight-forward to see how this

  trie can be used in a simplistic way. At any given position in the

  text we're searching (called the "subject" string), all we need to do

  is follow the transitions in the trie by consuming one transition for

  each byte in the subject string. If we reach a match state, then we can

  report that location as a match.

  

  The trick comes when searching a subject string like 'abcef'. We'll

  initially follow the transition from S0 to S1 and wind up in S3 after

  observng the 'c' byte. At this point, the next byte is 'e' but state

  S3 has no transition for 'e', so the search fails. We then would need

  to restart the search at the next position in 'abcef', which

  corresponds to 'b'. The match would fail, but the next search starting

  at 'c' would finally succeed. The problem with this approach is that

  we wind up searching the subject string potentially many times. In

  effect, this makes the algorithm have worst case `O(n * m)` complexity,

  where `n ~ len(subject)` and `m ~ len(all patterns)`. We would instead

  like to achieve a `O(n + m)` worst case complexity.

  

  This is where failure transitions come in. Instead of dying at S3 in

  the first search, the automaton can instruct the search to move to

  another part of the automaton that corresponds to a suffix of what

  we've seen so far. Recall that we've seen 'abc' in the subject string,

  and the automaton does indeed have a non-empty suffix, 'c', that could

  potentially lead to another match. Thus, the actual Aho-Corasick

  automaton for our patterns in this case looks like this:

  

  ```ignore

           a - S1 - b - S2 - c - S3 - d - S4*

          /                      /

         /       ----------------

        /       /

      S0 - c - S5 - e - S6 - f - S7*

  ```

  

  That is, we have a failure transition from S3 to S5, which is followed

  exactly in cases when we are in state S3 but see any byte other than

  'd' (that is, we've "failed" to find a match in this portion of our

  trie). We know we can transition back to S5 because we've already seen

  a 'c' byte, so we don't need to re-scan it. We can then pick back up

  with the search starting at S5 and complete our match.

  

  Adding failure transitions to a trie is fairly simple, but subtle. The

  key issue is that you might have multiple failure transition that you

  need to follow. For example, look at the trie for the patterns

  'abcd', 'b', 'bcd' and 'cd':

  

  ```ignore

           - a - S1 - b - S2* - c - S3 - d - S4*

          /               /         /

         /         -------   -------

        /         /         /

      S0 --- b - S5* - c - S6 - d - S7*

        \                  /

         \         --------

          \       /

           - c - S8 - d - S9*

  ```

  

  The failure transitions for this trie are defined from S2 to S5,

  S3 to S6 and S6 to S8. Moreover, state S2 needs to track that it

  corresponds to a match, since its failure transition to S5 is itself

  a match state.

  

  Perhaps simplest way to think about adding these failure transitions

  is recursively. That is, if you know the failure transitions for every

  possible previous state that could be visited (e.g., when computing the

  failure transition for S3, you already know the failure transitions

  for S0, S1 and S2), then you can simply follow the failure transition

  of the previous state and check whether the incoming transition is

  defined after following the failure transition.

  

  For example, when determining the failure state for S3, by our

  assumptions, we already know that there is a failure transition from

  S2 (the previous state) to S5. So we follow that transition and check

  whether the transition connecting S2 to S3 is defined. Indeed, it is,

  as there is a transition from S5 to S6 for the byte 'c'. If no such

  transition existed, we could keep following the failure transitions

  until we reach the start state, which is the failure transition for

  every state that has no corresponding proper suffix.

  

  We don't actually use recursion to implement this, but instead, use a

  breadth first search of the automaton. Our base case is the start

  state, whose failure transition is just a transition to itself.

  

  When building a leftmost automaton, we proceed as above, but only

  include a subset of failure transitions. Namely, we omit any failure

  transitions that appear after a match state in the trie. This is

  because failure transitions always point back to a proper suffix of

  what has been seen so far. Thus, following a failure transition after

  a match implies looking for a match that starts after the one that has

  already been seen, which is of course therefore not the leftmost match.

  

  N.B. I came up with this algorithm on my own, and after scouring all of

  the other AC implementations I know of (Perl, Snort, many on GitHub).

  I couldn't find any that implement leftmost semantics like this.

  Perl of course needs leftmost-first semantics, but they implement it

  with a seeming hack at *search* time instead of encoding it into the

  automaton. There are also a couple Java libraries that support leftmost

  longest semantics, but they do it by building a queue of matches at

  search time, which is even worse than what Perl is doing. ---AG

- <span id="compiler-shuffle"></span>`fn shuffle(&mut self)`

  Shuffle the states so that they appear in this sequence:

  

    DEAD, FAIL, MATCH..., START, START, NON-MATCH...

  

  The idea here is that if we know how special states are laid out in our

  transition table, then we can determine what "kind" of state we're in

  just by comparing our current state ID with a particular value. In this

  way, we avoid doing extra memory lookups.

  

  Before shuffling begins, our states look something like this:

  

    DEAD, FAIL, START, START, (MATCH | NON-MATCH)...

  

  So all we need to do is move all of the MATCH states so that they

  all appear before any NON-MATCH state, like so:

  

    DEAD, FAIL, START, START, MATCH... NON-MATCH...

  

  Then it's just a simple matter of swapping the two START states with

  the last two MATCH states.

  

  (This is the same technique used for fully compiled DFAs in

  regex-automata.)

- <span id="compiler-densify"></span>`fn densify(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

  Attempts to convert the transition representation of a subset of states

  in this NFA from sparse to dense. This can greatly improve search

  performance since states with a higher number of transitions tend to

  correlate with very active states.

  

  We generally only densify states that are close to the start state.

  These tend to be the most active states and thus benefit from a dense

  representation more than other states.

  

  This tends to best balance between memory usage and performance. In

  particular, the *vast majority* of all states in a typical Aho-Corasick

  automaton have only 1 transition and are usually farther from the start

  state and thus don't get densified.

  

  Note that this doesn't remove the sparse representation of transitions

  for states that are densified. It could be done, but actually removing

  entries from `NFA::sparse` is likely more expensive than it's worth.

- <span id="compiler-queued-set"></span>`fn queued_set(&self) -> QueuedSet` — [`QueuedSet`](#queuedset)

  Returns a set that tracked queued states.

  

  This is only necessary when ASCII case insensitivity is enabled, since

  it is the only way to visit the same state twice. Otherwise, this

  returns an inert set that nevers adds anything and always reports

  `false` for every member test.

- <span id="compiler-init-unanchored-start-state"></span>`fn init_unanchored_start_state(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

  Initializes the unanchored start state by making it dense. This is

  achieved by explicitly setting every transition to the FAIL state.

  This isn't necessary for correctness, since any missing transition is

  automatically assumed to be mapped to the FAIL state. We do this to

  make the unanchored starting state dense, and thus in turn make

  transition lookups on it faster. (Which is worth doing because it's

  the most active state.)

- <span id="compiler-set-anchored-start-state"></span>`fn set_anchored_start_state(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

  Setup the anchored start state by copying all of the transitions and

  matches from the unanchored starting state with one change: the failure

  transition is changed to the DEAD state, so that for any undefined

  transitions, the search will stop.

- <span id="compiler-add-unanchored-start-state-loop"></span>`fn add_unanchored_start_state_loop(&mut self)`

  Set the failure transitions on the start state to loop back to the

  start state. This effectively permits the Aho-Corasick automaton to

  match at any position. This is also required for finding the next

  state to terminate, namely, finding the next state should never return

  a fail_id.

  

  This must be done after building the initial trie, since trie

  construction depends on transitions to `fail_id` to determine whether a

  state already exists or not.

- <span id="compiler-close-start-state-loop-for-leftmost"></span>`fn close_start_state_loop_for_leftmost(&mut self)`

  Remove the start state loop by rewriting any transitions on the start

  state back to the start state with transitions to the dead state.

  

  The loop is only closed when two conditions are met: the start state

  is a match state and the match kind is leftmost-first or

  leftmost-longest.

  

  The reason for this is that under leftmost semantics, a start state

  that is also a match implies that we should never restart the search

  process. We allow normal transitions out of the start state, but if

  none exist, we transition to the dead state, which signals that

  searching should stop.

- <span id="compiler-add-dead-state-loop"></span>`fn add_dead_state_loop(&mut self) -> Result<(), BuildError>` — [`BuildError`](../../util/error/index.md#builderror)

  Sets all transitions on the dead state to point back to the dead state.

  Normally, missing transitions map back to the failure state, but the

  point of the dead state is to act as a sink that can never be escaped.

#### Trait Implementations

##### `impl Any for Compiler<'a>`

- <span id="compiler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Compiler<'a>`

- <span id="compiler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Compiler<'a>`

- <span id="compiler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Compiler<'a>`

- <span id="compiler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Compiler<'a>`

- <span id="compiler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Compiler<'a>`

- <span id="compiler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Compiler<'a>`

- <span id="compiler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compiler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Compiler<'a>`

- <span id="compiler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compiler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `QueuedSet`

```rust
struct QueuedSet {
    set: Option<alloc::collections::BTreeSet<crate::util::primitives::StateID>>,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/noncontiguous.rs:1657-1659`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/noncontiguous.rs#L1657-L1659)*

A set of state identifiers used to avoid revisiting the same state multiple
times when filling in failure transitions.

This set has an "inert" and an "active" mode. When inert, the set never
stores anything and always returns `false` for every member test. This is
useful to avoid the performance and memory overhead of maintaining this
set when it is not needed.

#### Implementations

- <span id="queuedset-inert"></span>`fn inert() -> QueuedSet` — [`QueuedSet`](#queuedset)

  Return an inert set that returns `false` for every state ID membership

  test.

- <span id="queuedset-active"></span>`fn active() -> QueuedSet` — [`QueuedSet`](#queuedset)

  Return an active set that tracks state ID membership.

- <span id="queuedset-insert"></span>`fn insert(&mut self, state_id: StateID)` — [`StateID`](../../util/primitives/index.md#stateid)

  Inserts the given state ID into this set. (If the set is inert, then

  this is a no-op.)

- <span id="queuedset-contains"></span>`fn contains(&self, state_id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md#stateid)

  Returns true if and only if the given state ID is in this set. If the

  set is inert, this always returns false.

#### Trait Implementations

##### `impl Any for QueuedSet`

- <span id="queuedset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for QueuedSet`

- <span id="queuedset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for QueuedSet`

- <span id="queuedset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for QueuedSet`

- <span id="queuedset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for QueuedSet`

- <span id="queuedset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for QueuedSet`

- <span id="queuedset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for QueuedSet`

- <span id="queuedset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="queuedset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for QueuedSet`

- <span id="queuedset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="queuedset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

