*[aho_corasick](../../index.md) / [nfa](../index.md) / [contiguous](index.md)*

---

# Module `contiguous`

Provides a contiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a contiguous NFA directly. Using an `NFA` directly is typically only
necessary when one needs access to the [`Automaton`](../../automaton/index.md) trait implementation.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NFA`](#nfa) | struct | A contiguous NFA implementation of Aho-Corasick. |
| [`State`](#state) | struct | The "in memory" representation a single dense or sparse state. |
| [`Builder`](#builder) | struct | A builder for configuring an Aho-Corasick contiguous NFA. |
| [`StateTrans`](#statetrans) | enum | The underlying representation of sparse or dense transitions for a state. |
| [`u32_len`](#u32-len) | fn | Computes the number of u32 values needed to represent one byte per the number of transitions given. |

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

*Defined in [`aho-corasick-1.1.4/src/nfa/contiguous.rs:91-120`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/contiguous.rs#L91-L120)*

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

- <span id="nfa-new"></span>`fn new<I, P>(patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

  Create a new Aho-Corasick contiguous NFA using the default

  configuration.

  

  Use a [`Builder`](#builder) if you want to change the configuration.

- <span id="nfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  A convenience method for returning a new Aho-Corasick contiguous NFA

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

##### `impl Sealed for crate::nfa::contiguous::NFA`

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

### `State<'a>`

```rust
struct State<'a> {
    fail: crate::util::primitives::StateID,
    match_len: usize,
    trans: StateTrans<'a>,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/contiguous.rs:390-399`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/contiguous.rs#L390-L399)*

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

- <span id="state-const-kind"></span>`const KIND: usize`

- <span id="state-const-kind-dense"></span>`const KIND_DENSE: u32`

- <span id="state-const-kind-one"></span>`const KIND_ONE: u32`

- <span id="state-const-max-sparse-transitions"></span>`const MAX_SPARSE_TRANSITIONS: usize`

- <span id="state-remap"></span>`fn remap(alphabet_len: usize, old_to_new: &[StateID], state: &mut [u32]) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](../../util/error/index.md#builderror)

  Remap state IDs in-place.

  

  `state` should be the the raw binary encoding of a state. (The start

  of the slice must correspond to the start of the state, but the slice

  may extend past the end of the encoding of the state.)

- <span id="state-len"></span>`fn len(alphabet_len: usize, is_match: bool, state: &[u32]) -> usize`

  Returns the length, in number of u32s, of this state.

  

  This is useful for reading states consecutively, e.g., in the Debug

  impl without needing to store a separate map from state index to state

  identifier.

  

  `state` should be the the raw binary encoding of a state. (The start

  of the slice must correspond to the start of the state, but the slice

  may extend past the end of the encoding of the state.)

- <span id="state-kind"></span>`fn kind(state: &[u32]) -> u32`

  Returns the kind of this state.

  

  This only includes the low byte.

- <span id="state-sparse-trans-len"></span>`fn sparse_trans_len(state: &[u32]) -> usize`

  Get the number of sparse transitions in this state. This can never

  be more than State::MAX_SPARSE_TRANSITIONS, as all states with more

  transitions are encoded as dense states.

  

  `state` should be the the raw binary encoding of a sparse state. (The

  start of the slice must correspond to the start of the state, but the

  slice may extend past the end of the encoding of the state.) If this

  isn't a sparse state, then the return value is unspecified.

  

  Do note that this is only legal to call on a sparse state. So for

  example, "one transition" state is not a sparse state, so it would not

  be legal to call this method on such a state.

- <span id="state-match-len"></span>`fn match_len(alphabet_len: usize, state: &[u32]) -> usize`

  Returns the total number of matching pattern IDs in this state. Calling

  this on a state that isn't a match results in unspecified behavior.

  Thus, the returned number is never 0 for all correct calls.

  

  `state` should be the the raw binary encoding of a state. (The start

  of the slice must correspond to the start of the state, but the slice

  may extend past the end of the encoding of the state.)

- <span id="state-match-pattern"></span>`fn match_pattern(alphabet_len: usize, state: &[u32], index: usize) -> PatternID` — [`PatternID`](../../util/primitives/index.md#patternid)

  Returns the pattern ID corresponding to the given index for the state

  given. The `index` provided must be less than the number of pattern IDs

  in this state.

  

  `state` should be the the raw binary encoding of a state. (The start of

  the slice must correspond to the start of the state, but the slice may

  extend past the end of the encoding of the state.)

  

  If the given state is not a match state or if the index is out of

  bounds, then this has unspecified behavior.

- <span id="state-read"></span>`fn read(alphabet_len: usize, is_match: bool, state: &'a [u32]) -> State<'a>` — [`State`](#state)

  Read a state's binary encoding to its in-memory representation.

  

  `alphabet_len` should be the total number of transitions defined for

  dense states.

  

  `is_match` should be true if this state is a match state and false

  otherwise.

  

  `state` should be the the raw binary encoding of a state. (The start

  of the slice must correspond to the start of the state, but the slice

  may extend past the end of the encoding of the state.)

- <span id="state-write"></span>`fn write(nnfa: &noncontiguous::NFA, oldsid: StateID, old: &noncontiguous::State, classes: &ByteClasses, dst: &mut Vec<u32>, force_dense: bool) -> Result<StateID, BuildError>` — [`NFA`](../noncontiguous/index.md#nfa), [`StateID`](../../util/primitives/index.md#stateid), [`State`](../noncontiguous/index.md#state), [`ByteClasses`](../../util/alphabet/index.md#byteclasses), [`BuildError`](../../util/error/index.md#builderror)

  Encode the "old" state from a noncontiguous NFA to its binary

  representation to the given `dst` slice. `classes` should be the byte

  classes computed for the noncontiguous NFA that the given state came

  from.

  

  This returns an error if `dst` became so big that `StateID`s can no

  longer be created for new states. Otherwise, it returns the state ID of

  the new state created.

  

  When `force_dense` is true, then the encoded state will always use a

  dense format. Otherwise, the choice between dense and sparse will be

  automatically chosen based on the old state.

- <span id="state-write-sparse-trans"></span>`fn write_sparse_trans(nnfa: &noncontiguous::NFA, oldsid: StateID, classes: &ByteClasses, dst: &mut Vec<u32>) -> Result<(), BuildError>` — [`NFA`](../noncontiguous/index.md#nfa), [`StateID`](../../util/primitives/index.md#stateid), [`ByteClasses`](../../util/alphabet/index.md#byteclasses), [`BuildError`](../../util/error/index.md#builderror)

  Encode the "old" state transitions from a noncontiguous NFA to its

  binary sparse representation to the given `dst` slice. `classes` should

  be the byte classes computed for the noncontiguous NFA that the given

  state came from.

  

  This returns an error if `dst` became so big that `StateID`s can no

  longer be created for new states.

- <span id="state-write-dense-trans"></span>`fn write_dense_trans(nnfa: &noncontiguous::NFA, oldsid: StateID, classes: &ByteClasses, dst: &mut Vec<u32>) -> Result<(), BuildError>` — [`NFA`](../noncontiguous/index.md#nfa), [`StateID`](../../util/primitives/index.md#stateid), [`ByteClasses`](../../util/alphabet/index.md#byteclasses), [`BuildError`](../../util/error/index.md#builderror)

  Encode the "old" state transitions from a noncontiguous NFA to its

  binary dense representation to the given `dst` slice. `classes` should

  be the byte classes computed for the noncontiguous NFA that the given

  state came from.

  

  This returns an error if `dst` became so big that `StateID`s can no

  longer be created for new states.

- <span id="state-transitions"></span>`fn transitions(&self) -> impl Iterator<Item = (u8, StateID)> + '_` — [`StateID`](../../util/primitives/index.md#stateid)

  Return an iterator over every explicitly defined transition in this

  state.

#### Trait Implementations

##### `impl Any for State<'a>`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State<'a>`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State<'a>`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State<'a>`

- <span id="state-clone"></span>`fn clone(&self) -> State<'a>` — [`State`](#state)

##### `impl CloneToUninit for State<'a>`

- <span id="state-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for State<'a>`

- <span id="state-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for State<'a>`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State<'a>`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for State<'a>`

- <span id="state-toowned-type-owned"></span>`type Owned = T`

- <span id="state-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="state-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for State<'a>`

- <span id="state-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="state-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for State<'a>`

- <span id="state-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="state-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Builder`

```rust
struct Builder {
    noncontiguous: noncontiguous::Builder,
    dense_depth: usize,
    byte_classes: bool,
}
```

*Defined in [`aho-corasick-1.1.4/src/nfa/contiguous.rs:894-898`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/contiguous.rs#L894-L898)*

A builder for configuring an Aho-Corasick contiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new builder for configuring an Aho-Corasick contiguous NFA.

- <span id="builder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](../../util/error/index.md#builderror)

  Build an Aho-Corasick contiguous NFA from the given iterator of

  patterns.

  

  A builder may be reused to create more NFAs.

- <span id="builder-build-from-noncontiguous"></span>`fn build_from_noncontiguous(&self, nnfa: &noncontiguous::NFA) -> Result<NFA, BuildError>` — [`NFA`](../noncontiguous/index.md#nfa), [`BuildError`](../../util/error/index.md#builderror)

  Build an Aho-Corasick contiguous NFA from the given noncontiguous NFA.

  

  Note that when this method is used, only the `dense_depth` and

  `byte_classes` settings on this builder are respected. The other

  settings only apply to the initial construction of the Aho-Corasick

  automaton. Since using this method requires that initial construction

  has already completed, all settings impacting only initial construction

  are no longer relevant.

- <span id="builder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Builder` — [`MatchKind`](../../util/search/index.md#matchkind), [`Builder`](#builder)

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

- <span id="builder-dense-depth"></span>`fn dense_depth(&mut self, depth: usize) -> &mut Builder` — [`Builder`](#builder)

  Set the limit on how many states use a dense representation for their

  transitions. Other states will generally use a sparse representation.

  

  See

  [`AhoCorasickBuilder::dense_depth`](crate::AhoCorasickBuilder::dense_depth)

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

*Defined in [`aho-corasick-1.1.4/src/nfa/contiguous.rs:407-446`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/contiguous.rs#L407-L446)*

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

##### `impl Any for StateTrans<'a>`

- <span id="statetrans-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateTrans<'a>`

- <span id="statetrans-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateTrans<'a>`

- <span id="statetrans-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateTrans<'a>`

- <span id="statetrans-clone"></span>`fn clone(&self) -> StateTrans<'a>` — [`StateTrans`](#statetrans)

##### `impl CloneToUninit for StateTrans<'a>`

- <span id="statetrans-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<T> From for StateTrans<'a>`

- <span id="statetrans-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateTrans<'a>`

- <span id="statetrans-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StateTrans<'a>`

- <span id="statetrans-toowned-type-owned"></span>`type Owned = T`

- <span id="statetrans-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="statetrans-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateTrans<'a>`

- <span id="statetrans-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statetrans-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateTrans<'a>`

- <span id="statetrans-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statetrans-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `u32_len`

```rust
fn u32_len(ntrans: usize) -> usize
```

*Defined in [`aho-corasick-1.1.4/src/nfa/contiguous.rs:1080-1086`](../../../../.source_1765633015/aho-corasick-1.1.4/src/nfa/contiguous.rs#L1080-L1086)*

Computes the number of u32 values needed to represent one byte per the
number of transitions given.

