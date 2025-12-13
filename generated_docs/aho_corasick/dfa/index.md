*[aho_corasick](../index.md) / [dfa](index.md)*

---

# Module `dfa`

Provides direct access to a DFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a DFA directly. Using an `DFA` directly is typically only necessary
when one needs access to the [`Automaton`](../automaton/index.md) trait implementation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DFA`](#dfa) | struct | A DFA implementation of Aho-Corasick. |
| [`Builder`](#builder) | struct | A builder for configuring an Aho-Corasick DFA. |
| [`sparse_iter`](#sparse-iter) | fn | Iterate over all possible equivalence class transitions in this state. |

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

*Defined in [`aho-corasick-1.1.4/src/dfa.rs:91-132`](../../../.source_1765521767/aho-corasick-1.1.4/src/dfa.rs#L91-L132)*

A DFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using a `DFA` directly is typically only necessary when
one needs access to the [`Automaton`](../automaton/index.md) trait implementation.

This DFA can only be built by first constructing a [`noncontiguous::NFA`](../nfa/noncontiguous/index.md).
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

- <span id="dfa-new"></span>`fn new<I, P>(patterns: I) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../util/error/index.md#builderror)

  Create a new Aho-Corasick DFA using the default configuration.

  

  Use a [`Builder`](#builder) if you want to change the configuration.

- <span id="dfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  A convenience method for returning a new Aho-Corasick DFA builder.

  

  This usually permits one to just import the `DFA` type.

#### Trait Implementations

##### `impl Any for DFA`

- <span id="dfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl Automaton for DFA`

- <span id="dfa-automaton-start-state"></span>`fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>` — [`Anchored`](../util/search/index.md#anchored), [`StateID`](../util/primitives/index.md#stateid), [`MatchError`](../util/error/index.md#matcherror)

- <span id="dfa-automaton-next-state"></span>`fn next_state(&self, _anchored: Anchored, sid: StateID, byte: u8) -> StateID` — [`Anchored`](../util/search/index.md#anchored), [`StateID`](../util/primitives/index.md#stateid)

- <span id="dfa-automaton-is-special"></span>`fn is_special(&self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="dfa-automaton-is-dead"></span>`fn is_dead(&self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="dfa-automaton-is-match"></span>`fn is_match(&self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="dfa-automaton-is-start"></span>`fn is_start(&self, sid: StateID) -> bool` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="dfa-automaton-match-kind"></span>`fn match_kind(&self) -> MatchKind` — [`MatchKind`](../util/search/index.md#matchkind)

- <span id="dfa-automaton-patterns-len"></span>`fn patterns_len(&self) -> usize`

- <span id="dfa-automaton-pattern-len"></span>`fn pattern_len(&self, pid: PatternID) -> usize` — [`PatternID`](../util/primitives/index.md#patternid)

- <span id="dfa-automaton-min-pattern-len"></span>`fn min_pattern_len(&self) -> usize`

- <span id="dfa-automaton-max-pattern-len"></span>`fn max_pattern_len(&self) -> usize`

- <span id="dfa-automaton-match-len"></span>`fn match_len(&self, sid: StateID) -> usize` — [`StateID`](../util/primitives/index.md#stateid)

- <span id="dfa-automaton-match-pattern"></span>`fn match_pattern(&self, sid: StateID, index: usize) -> PatternID` — [`StateID`](../util/primitives/index.md#stateid), [`PatternID`](../util/primitives/index.md#patternid)

- <span id="dfa-automaton-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="dfa-automaton-prefilter"></span>`fn prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../util/prefilter/index.md#prefilter)

##### `impl<T> Borrow for DFA`

- <span id="dfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DFA`

- <span id="dfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DFA`

- <span id="dfa-clone"></span>`fn clone(&self) -> DFA` — [`DFA`](#dfa)

##### `impl CloneToUninit for DFA`

- <span id="dfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DFA`

- <span id="dfa-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for DFA`

- <span id="dfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DFA`

- <span id="dfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Sealed for crate::dfa::DFA`

##### `impl ToOwned for DFA`

- <span id="dfa-toowned-type-owned"></span>`type Owned = T`

- <span id="dfa-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="dfa-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DFA`

- <span id="dfa-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="dfa-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DFA`

- <span id="dfa-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="dfa-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Builder`

```rust
struct Builder {
    noncontiguous: noncontiguous::Builder,
    start_kind: crate::util::search::StartKind,
    byte_classes: bool,
}
```

*Defined in [`aho-corasick-1.1.4/src/dfa.rs:389-393`](../../../.source_1765521767/aho-corasick-1.1.4/src/dfa.rs#L389-L393)*

A builder for configuring an Aho-Corasick DFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new builder for configuring an Aho-Corasick DFA.

- <span id="builder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../util/error/index.md#builderror)

  Build an Aho-Corasick DFA from the given iterator of patterns.

  

  A builder may be reused to create more DFAs.

- <span id="builder-build-from-noncontiguous"></span>`fn build_from_noncontiguous(&self, nnfa: &noncontiguous::NFA) -> Result<DFA, BuildError>` — [`NFA`](../nfa/noncontiguous/index.md#nfa), [`DFA`](#dfa), [`BuildError`](../util/error/index.md#builderror)

  Build an Aho-Corasick DFA from the given noncontiguous NFA.

  

  Note that when this method is used, only the `start_kind` and

  `byte_classes` settings on this builder are respected. The other

  settings only apply to the initial construction of the Aho-Corasick

  automaton. Since using this method requires that initial construction

  has already completed, all settings impacting only initial construction

  are no longer relevant.

- <span id="builder-finish-build-one-start"></span>`fn finish_build_one_start(&self, anchored: Anchored, nnfa: &noncontiguous::NFA, dfa: &mut DFA)` — [`Anchored`](../util/search/index.md#anchored), [`NFA`](../nfa/noncontiguous/index.md#nfa), [`DFA`](#dfa)

  Finishes building a DFA for either unanchored or anchored searches,

  but NOT both.

- <span id="builder-finish-build-both-starts"></span>`fn finish_build_both_starts(&self, nnfa: &noncontiguous::NFA, dfa: &mut DFA)` — [`NFA`](../nfa/noncontiguous/index.md#nfa), [`DFA`](#dfa)

  Finishes building a DFA that supports BOTH unanchored and anchored

  searches. It works by inter-leaving unanchored states with anchored

  states in the same transition table. This way, we avoid needing to

  re-shuffle states afterward to ensure that our states still look like

  DEAD, MATCH, ..., START-UNANCHORED, START-ANCHORED, NON-MATCH, ...

  

  Honestly this is pretty inscrutable... Simplifications are most

  welcome.

- <span id="builder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../util/search/index.md#matchkind), [`Builder`](#builder)

  Set the desired match semantics.

  

  This only applies when using `Builder::build` and not

  `Builder::build_from_noncontiguous`.

  

  See

  [`AhoCorasickBuilder::match_kind`](crate::AhoCorasickBuilder::match_kind)

  for more documentation and examples.

- <span id="builder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

  Enable ASCII-aware case insensitive matching.

  

  This only applies when using `Builder::build` and not

  `Builder::build_from_noncontiguous`.

  

  See

  [`AhoCorasickBuilder::ascii_case_insensitive`](crate::AhoCorasickBuilder::ascii_case_insensitive)

  for more documentation and examples.

- <span id="builder-prefilter"></span>`fn prefilter(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

  Enable heuristic prefilter optimizations.

  

  This only applies when using `Builder::build` and not

  `Builder::build_from_noncontiguous`.

  

  See

  [`AhoCorasickBuilder::prefilter`](crate::AhoCorasickBuilder::prefilter)

  for more documentation and examples.

- <span id="builder-start-kind"></span>`fn start_kind(&mut self, kind: StartKind) -> &mut Builder` — [`StartKind`](../util/search/index.md#startkind), [`Builder`](#builder)

  Sets the starting state configuration for the automaton.

  

  See

  [`AhoCorasickBuilder::start_kind`](crate::AhoCorasickBuilder::start_kind)

  for more documentation and examples.

- <span id="builder-byte-classes"></span>`fn byte_classes(&mut self, yes: bool) -> &mut Builder` — [`Builder`](#builder)

  A debug setting for whether to attempt to shrink the size of the

  automaton's alphabet or not.

  

  This should never be enabled unless you're debugging an automaton.

  Namely, disabling byte classes makes transitions easier to reason

  about, since they use the actual bytes instead of equivalence classes.

  Disabling this confers no performance benefit at search time.

  

  See

  [`AhoCorasickBuilder::byte_classes`](crate::AhoCorasickBuilder::byte_classes)

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

## Functions

### `sparse_iter`

```rust
fn sparse_iter<F: FnMut(u8, u8, crate::util::primitives::StateID)>(nnfa: &noncontiguous::NFA, oldsid: crate::util::primitives::StateID, classes: &crate::util::alphabet::ByteClasses, f: F)
```

*Defined in [`aho-corasick-1.1.4/src/dfa.rs:801-835`](../../../.source_1765521767/aho-corasick-1.1.4/src/dfa.rs#L801-L835)*

Iterate over all possible equivalence class transitions in this state.
The closure is called for all transitions with a distinct equivalence
class, even those not explicitly represented in this sparse state. For
any implicitly defined transitions, the given closure is called with
the fail state ID.

The closure is guaranteed to be called precisely
`byte_classes.alphabet_len()` times, once for every possible class in
ascending order.

