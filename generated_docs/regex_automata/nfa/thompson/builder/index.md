*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [builder](index.md)*

---

# Module `builder`

## Structs

### `Builder`

```rust
struct Builder {
    pattern_id: Option<crate::util::primitives::PatternID>,
    states: alloc::vec::Vec<State>,
    start_pattern: alloc::vec::Vec<crate::util::primitives::StateID>,
    captures: alloc::vec::Vec<alloc::vec::Vec<Option<alloc::sync::Arc<str>>>>,
    memory_states: usize,
    utf8: bool,
    reverse: bool,
    look_matcher: crate::util::look::LookMatcher,
    size_limit: Option<usize>,
}
```

An abstraction for building Thompson NFAs by hand.

A builder is what a [`thompson::Compiler`](crate::nfa::thompson::Compiler)
uses internally to translate a regex's high-level intermediate
representation into an [`NFA`](../index.md).

The primary function of this builder is to abstract away the internal
representation of an NFA and make it difficult to produce NFAs are that
internally invalid or inconsistent. This builder also provides a way to
add "empty" states (which can be thought of as unconditional epsilon
transitions), despite the fact that [`thompson::State`](nfa::State) does
not have any "empty" representation. The advantage of "empty" states is
that they make the code for constructing a Thompson NFA logically simpler.

Many of the routines on this builder may panic or return errors. Generally
speaking, panics occur when an invalid sequence of method calls were made,
where as an error occurs if things get too big. (Where "too big" might mean
exhausting identifier space or using up too much heap memory in accordance
with the configured [`size_limit`](Builder::set_size_limit).)

# Overview

## Adding multiple patterns

Each pattern you add to an NFA should correspond to a pair of
`Builder::start_pattern` and `Builder::finish_pattern` calls, with
calls inbetween that add NFA states for that pattern. NFA states may be
added without first calling `start_pattern`, with the exception of adding
capturing states.

## Adding NFA states

Here is a very brief overview of each of the methods that add NFA states.
Every method adds a single state.

* [`add_empty`](Builder::add_empty): Add a state with a single
unconditional epsilon transition to another state.
* [`add_union`](Builder::add_union): Adds a state with unconditional
epsilon transitions to two or more states, with earlier transitions
preferred over later ones.
* [`add_union_reverse`](Builder::add_union_reverse): Adds a state with
unconditional epsilon transitions to two or more states, with later
transitions preferred over earlier ones.
* [`add_range`](Builder::add_range): Adds a state with a single transition
to another state that can only be followed if the current input byte is
within the range given.
* [`add_sparse`](Builder::add_sparse): Adds a state with two or more
range transitions to other states, where a transition is only followed
if the current input byte is within one of the ranges. All transitions
in this state have equal priority, and the corresponding ranges must be
non-overlapping.
* [`add_look`](Builder::add_look): Adds a state with a single *conditional*
epsilon transition to another state, where the condition depends on a
limited look-around property.
* [`add_capture_start`](Builder::add_capture_start): Adds a state with
a single unconditional epsilon transition that also instructs an NFA
simulation to record the current input position to a specific location in
memory. This is intended to represent the starting location of a capturing
group.
* [`add_capture_end`](Builder::add_capture_end): Adds a state with
a single unconditional epsilon transition that also instructs an NFA
simulation to record the current input position to a specific location in
memory. This is intended to represent the ending location of a capturing
group.
* [`add_fail`](Builder::add_fail): Adds a state that never transitions to
another state.
* [`add_match`](Builder::add_match): Add a state that indicates a match has
been found for a particular pattern. A match state is a final state with
no outgoing transitions.

## Setting transitions between NFA states

The `Builder::patch` method creates a transition from one state to the
next. If the `from` state corresponds to a state that supports multiple
outgoing transitions (such as "union"), then this adds the corresponding
transition. Otherwise, it sets the single transition. (This routine panics
if `from` corresponds to a state added by `add_sparse`, since sparse states
need more specialized handling.)

# Example

This annotated example shows how to hand construct the regex `[a-z]+`
(without an unanchored prefix).

```rust
use regex_automata::{
    nfa::thompson::{pikevm::PikeVM, Builder, Transition},
    util::primitives::StateID,
    Match,
};

let mut builder = Builder::new();
// Before adding NFA states for our pattern, we need to tell the builder
// that we are starting the pattern.
builder.start_pattern()?;
// Since we use the Pike VM below for searching, we need to add capturing
// states. If you're just going to build a DFA from the NFA, then capturing
// states do not need to be added.
let start = builder.add_capture_start(StateID::ZERO, 0, None)?;
let range = builder.add_range(Transition {
    // We don't know the state ID of the 'next' state yet, so we just fill
    // in a dummy 'ZERO' value.
    start: b'a', end: b'z', next: StateID::ZERO,
})?;
// This state will point back to 'range', but also enable us to move ahead.
// That is, this implements the '+' repetition operator. We add 'range' and
// then 'end' below to this alternation.
let alt = builder.add_union(vec![])?;
// The final state before the match state, which serves to capture the
// end location of the match.
let end = builder.add_capture_end(StateID::ZERO, 0)?;
// The match state for our pattern.
let mat = builder.add_match()?;
// Now we fill in the transitions between states.
builder.patch(start, range)?;
builder.patch(range, alt)?;
// If we added 'end' before 'range', then we'd implement non-greedy
// matching, i.e., '+?'.
builder.patch(alt, range)?;
builder.patch(alt, end)?;
builder.patch(end, mat)?;
// We must explicitly finish pattern and provide the starting state ID for
// this particular pattern.
builder.finish_pattern(start)?;
// Finally, when we build the NFA, we provide the anchored and unanchored
// starting state IDs. Since we didn't bother with an unanchored prefix
// here, we only support anchored searching. Thus, both starting states are
// the same.
let nfa = builder.build(start, start)?;

// Now build a Pike VM from our NFA, and use it for searching. This shows
// how we can use a regex engine without ever worrying about syntax!
let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 0..3));
re.captures(&mut cache, "foo0", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`pattern_id`**: `Option<crate::util::primitives::PatternID>`

  The ID of the pattern that we're currently building.
  
  Callers are required to set (and unset) this by calling
  {start,finish}_pattern. Otherwise, most methods will panic.

- **`states`**: `alloc::vec::Vec<State>`

  A sequence of intermediate NFA states. Once a state is added to this
  sequence, it is assigned a state ID equivalent to its index. Once a
  state is added, it is still expected to be mutated, e.g., to set its
  transition to a state that didn't exist at the time it was added.

- **`start_pattern`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The starting states for each individual pattern. Starting at any
  of these states will result in only an anchored search for the
  corresponding pattern. The vec is indexed by pattern ID. When the NFA
  contains a single regex, then `start_pattern[0]` and `start_anchored`
  are always equivalent.

- **`captures`**: `alloc::vec::Vec<alloc::vec::Vec<Option<alloc::sync::Arc<str>>>>`

  A map from pattern ID to capture group index to name. (If no name
  exists, then a None entry is present. Thus, all capturing groups are
  present in this mapping.)
  
  The outer vec is indexed by pattern ID, while the inner vec is indexed
  by capture index offset for the corresponding pattern.
  
  The first capture group for each pattern is always unnamed and is thus
  always None.

- **`memory_states`**: `usize`

  The combined memory used by each of the 'State's in 'states'. This
  only includes heap usage by each state, and not the size of the state
  itself. In other words, this tracks heap memory used that isn't
  captured via `size_of::<State>() * states.len()`.

- **`utf8`**: `bool`

  Whether this NFA only matches UTF-8 and whether regex engines using
  this NFA for searching should report empty matches that split a
  codepoint.

- **`reverse`**: `bool`

  Whether this NFA should be matched in reverse or not.

- **`look_matcher`**: `crate::util::look::LookMatcher`

  The matcher to use for look-around assertions.

- **`size_limit`**: `Option<usize>`

  A size limit to respect when building an NFA. If the total heap memory
  of the intermediate NFA states exceeds (or would exceed) this amount,
  then an error is returned.

#### Implementations

- `fn new() -> Builder` — [`Builder`](../index.md)

- `fn clear(self: &mut Self)`

- `fn build(self: &Self, start_anchored: StateID, start_unanchored: StateID) -> Result<NFA, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`NFA`](../index.md), [`BuildError`](../index.md)

- `fn start_pattern(self: &mut Self) -> Result<PatternID, BuildError>` — [`PatternID`](../../../index.md), [`BuildError`](../index.md)

- `fn finish_pattern(self: &mut Self, start_id: StateID) -> Result<PatternID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`PatternID`](../../../index.md), [`BuildError`](../index.md)

- `fn current_pattern_id(self: &Self) -> PatternID` — [`PatternID`](../../../index.md)

- `fn pattern_len(self: &Self) -> usize`

- `fn add_empty(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_union(self: &mut Self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_union_reverse(self: &mut Self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_range(self: &mut Self, trans: Transition) -> Result<StateID, BuildError>` — [`Transition`](../index.md), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_sparse(self: &mut Self, transitions: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](../index.md), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_look(self: &mut Self, next: StateID, look: Look) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`Look`](../../../util/look/index.md), [`BuildError`](../index.md)

- `fn add_capture_start(self: &mut Self, next: StateID, group_index: u32, name: Option<Arc<str>>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_capture_end(self: &mut Self, next: StateID, group_index: u32) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_fail(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add_match(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn add(self: &mut Self, state: State) -> Result<StateID, BuildError>` — [`State`](#state), [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn patch(self: &mut Self, from: StateID, to: StateID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md), [`BuildError`](../index.md)

- `fn set_utf8(self: &mut Self, yes: bool)`

- `fn get_utf8(self: &Self) -> bool`

- `fn set_reverse(self: &mut Self, yes: bool)`

- `fn get_reverse(self: &Self) -> bool`

- `fn set_look_matcher(self: &mut Self, m: LookMatcher)` — [`LookMatcher`](../../../util/look/index.md)

- `fn get_look_matcher(self: &Self) -> &LookMatcher` — [`LookMatcher`](../../../util/look/index.md)

- `fn set_size_limit(self: &mut Self, limit: Option<usize>) -> Result<(), BuildError>` — [`BuildError`](../index.md)

- `fn get_size_limit(self: &Self) -> Option<usize>`

- `fn memory_usage(self: &Self) -> usize`

- `fn check_size_limit(self: &Self) -> Result<(), BuildError>` — [`BuildError`](../index.md)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](../index.md)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](../index.md)

## Enums

### `State`

```rust
enum State {
    Empty {
        next: crate::util::primitives::StateID,
    },
    ByteRange {
        trans: crate::nfa::thompson::nfa::Transition,
    },
    Sparse {
        transitions: alloc::vec::Vec<crate::nfa::thompson::nfa::Transition>,
    },
    Look {
        look: crate::util::look::Look,
        next: crate::util::primitives::StateID,
    },
    CaptureStart {
        pattern_id: crate::util::primitives::PatternID,
        group_index: crate::util::primitives::SmallIndex,
        next: crate::util::primitives::StateID,
    },
    CaptureEnd {
        pattern_id: crate::util::primitives::PatternID,
        group_index: crate::util::primitives::SmallIndex,
        next: crate::util::primitives::StateID,
    },
    Union {
        alternates: alloc::vec::Vec<crate::util::primitives::StateID>,
    },
    UnionReverse {
        alternates: alloc::vec::Vec<crate::util::primitives::StateID>,
    },
    Fail,
    Match {
        pattern_id: crate::util::primitives::PatternID,
    },
}
```

An intermediate NFA state used during construction.

During construction of an NFA, it is often convenient to work with states
that are amenable to mutation and other carry more information than we
otherwise need once an NFA has been built. This type represents those
needs.

Once construction is finished, the builder will convert these states to a
[`nfa::thompson::State`](crate::nfa::thompson::State). This conversion not
only results in a simpler representation, but in some cases, entire classes
of states are completely removed (such as [`State::Empty`](../../../index.md)).

#### Variants

- **`Empty`**

  An empty state whose only purpose is to forward the automaton to
  another state via an unconditional epsilon transition.
  
  Unconditional epsilon transitions are quite useful during the
  construction of an NFA, as they permit the insertion of no-op
  placeholders that make it easier to compose NFA sub-graphs. When
  the Thompson NFA builder produces a final NFA, all unconditional
  epsilon transitions are removed, and state identifiers are remapped
  accordingly.

- **`ByteRange`**

  A state that only transitions to another state if the current input
  byte is in a particular range of bytes.

- **`Sparse`**

  A state with possibly many transitions, represented in a sparse
  fashion. Transitions must be ordered lexicographically by input range
  and be non-overlapping. As such, this may only be used when every
  transition has equal priority. (In practice, this is only used for
  encoding large UTF-8 automata.) In contrast, a `Union` state has each
  alternate in order of priority. Priority is used to implement greedy
  matching and also alternations themselves, e.g., `abc|a` where `abc`
  has priority over `a`.
  
  To clarify, it is possible to remove `Sparse` and represent all things
  that `Sparse` is used for via `Union`. But this creates a more bloated
  NFA with more epsilon transitions than is necessary in the special case
  of character classes.

- **`Look`**

  A conditional epsilon transition satisfied via some sort of
  look-around.

- **`CaptureStart`**

  An empty state that records the start of a capture location. This is an
  unconditional epsilon transition like `Empty`, except it can be used to
  record position information for a capture group when using the NFA for
  search.

- **`CaptureEnd`**

  An empty state that records the end of a capture location. This is an
  unconditional epsilon transition like `Empty`, except it can be used to
  record position information for a capture group when using the NFA for
  search.

- **`Union`**

  An alternation such that there exists an epsilon transition to all
  states in `alternates`, where matches found via earlier transitions
  are preferred over later transitions.

- **`UnionReverse`**

  An alternation such that there exists an epsilon transition to all
  states in `alternates`, where matches found via later transitions are
  preferred over earlier transitions.
  
  This "reverse" state exists for convenience during compilation that
  permits easy construction of non-greedy combinations of NFA states. At
  the end of compilation, Union and UnionReverse states are merged into
  one Union type of state, where the latter has its epsilon transitions
  reversed to reflect the priority inversion.
  
  The "convenience" here arises from the fact that as new states are
  added to the list of `alternates`, we would like that add operation
  to be amortized constant time. But if we used a `Union`, we'd need to
  prepend the state, which takes O(n) time. There are other approaches we
  could use to solve this, but this seems simple enough.

- **`Fail`**

  A state that cannot be transitioned out of. This is useful for cases
  where you want to prevent matching from occurring. For example, if your
  regex parser permits empty character classes, then one could choose a
  `Fail` state to represent it.

- **`Match`**

  A match state. There is at most one such occurrence of this state in
  an NFA for each pattern compiled into the NFA. At time of writing, a
  match state is always produced for every pattern given, but in theory,
  if a pattern can never lead to a match, then the match state could be
  omitted.
  
  `pattern_id` refers to the ID of the pattern itself, which corresponds
  to the pattern's index (starting at 0).

#### Implementations

- `fn goto(self: &Self) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for State`

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

