*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [builder](index.md)*

---

# Module `builder`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Builder`](#builder) | struct | An abstraction for building Thompson NFAs by hand. |
| [`State`](#state) | enum | An intermediate NFA state used during construction. |

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/builder.rs:313-357`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/builder.rs#L313-L357)*

An abstraction for building Thompson NFAs by hand.

A builder is what a [`thompson::Compiler`](crate::nfa::thompson::Compiler)
uses internally to translate a regex's high-level intermediate
representation into an [`NFA`](../nfa/index.md).

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

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

  Create a new builder for hand-assembling NFAs.

- <span id="builder-clear"></span>`fn clear(&mut self)`

  Clear this builder.

  

  Clearing removes all state associated with building an NFA, but does

  not reset configuration (such as size limits and whether the NFA

  should only match UTF-8). After clearing, the builder can be reused to

  assemble an entirely new NFA.

- <span id="builder-build"></span>`fn build(&self, start_anchored: StateID, start_unanchored: StateID) -> Result<NFA, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`NFA`](../nfa/index.md#nfa), [`BuildError`](../error/index.md#builderror)

  Assemble a [`NFA`](../nfa/index.md) from the states added so far.

  

  After building an NFA, more states may be added and `build` may be

  called again. To reuse a builder to produce an entirely new NFA from

  scratch, call the [`clear`](Builder::clear) method first.

  

  `start_anchored` refers to the ID of the starting state that anchored

  searches should use. That is, searches who matches are limited to the

  starting position of the search.

  

  `start_unanchored` refers to the ID of the starting state that

  unanchored searches should use. This permits searches to report matches

  that start after the beginning of the search. In cases where unanchored

  searches are not supported, the unanchored starting state ID must be

  the same as the anchored starting state ID.

  

  # Errors

  

  This returns an error if there was a problem producing the final NFA.

  In particular, this might include an error if the capturing groups

  added to this builder violate any of the invariants documented on

  [`GroupInfo`](crate::util::captures::GroupInfo).

  

  # Panics

  

  If `start_pattern` was called, then `finish_pattern` must be called

  before `build`, otherwise this panics.

  

  This may panic for other invalid uses of a builder. For example, if

  a "start capture" state was added without a corresponding "end capture"

  state.

- <span id="builder-start-pattern"></span>`fn start_pattern(&mut self) -> Result<PatternID, BuildError>` — [`PatternID`](../../../util/primitives/index.md#patternid), [`BuildError`](../error/index.md#builderror)

  Start the assembly of a pattern in this NFA.

  

  Upon success, this returns the identifier for the new pattern.

  Identifiers start at `0` and are incremented by 1 for each new pattern.

  

  It is necessary to call this routine before adding capturing states.

  Otherwise, any other NFA state may be added before starting a pattern.

  

  # Errors

  

  If the pattern identifier space is exhausted, then this returns an

  error.

  

  # Panics

  

  If this is called while assembling another pattern (i.e., before

  `finish_pattern` is called), then this panics.

- <span id="builder-finish-pattern"></span>`fn finish_pattern(&mut self, start_id: StateID) -> Result<PatternID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`PatternID`](../../../util/primitives/index.md#patternid), [`BuildError`](../error/index.md#builderror)

  Finish the assembly of a pattern in this NFA.

  

  Upon success, this returns the identifier for the new pattern.

  Identifiers start at `0` and are incremented by 1 for each new

  pattern. This is the same identifier returned by the corresponding

  `start_pattern` call.

  

  Note that `start_pattern` and `finish_pattern` pairs cannot be

  interleaved or nested. A correct `finish_pattern` call _always_

  corresponds to the most recently called `start_pattern` routine.

  

  # Errors

  

  This currently never returns an error, but this is subject to change.

  

  # Panics

  

  If this is called without a corresponding `start_pattern` call, then

  this panics.

- <span id="builder-current-pattern-id"></span>`fn current_pattern_id(&self) -> PatternID` — [`PatternID`](../../../util/primitives/index.md#patternid)

  Returns the pattern identifier of the current pattern.

  

  # Panics

  

  If this doesn't occur after a `start_pattern` call and before the

  corresponding `finish_pattern` call, then this panics.

- <span id="builder-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the number of patterns added to this builder so far.

  

  This only includes patterns that have had `finish_pattern` called

  for them.

- <span id="builder-add-empty"></span>`fn add_empty(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add an "empty" NFA state.

  

  An "empty" NFA state is a state with a single unconditional epsilon

  transition to another NFA state. Such empty states are removed before

  building the final [`NFA`](../nfa/index.md) (which has no such "empty" states), but they

  can be quite useful in the construction process of an NFA.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-union"></span>`fn add_union(&mut self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a "union" NFA state.

  

  A "union" NFA state that contains zero or more unconditional epsilon

  transitions to other NFA states. The order of these transitions

  reflects a priority order where earlier transitions are preferred over

  later transitions.

  

  Callers may provide an empty set of alternates to this method call, and

  then later add transitions via `patch`. At final build time, a "union"

  state with no alternates is converted to a "fail" state, and a "union"

  state with exactly one alternate is treated as if it were an "empty"

  state.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-union-reverse"></span>`fn add_union_reverse(&mut self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a "reverse union" NFA state.

  

  A "reverse union" NFA state contains zero or more unconditional epsilon

  transitions to other NFA states. The order of these transitions

  reflects a priority order where later transitions are preferred

  over earlier transitions. This is an inverted priority order when

  compared to `add_union`. This is useful, for example, for implementing

  non-greedy repetition operators.

  

  Callers may provide an empty set of alternates to this method call, and

  then later add transitions via `patch`. At final build time, a "reverse

  union" state with no alternates is converted to a "fail" state, and a

  "reverse union" state with exactly one alternate is treated as if it

  were an "empty" state.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-range"></span>`fn add_range(&mut self, trans: Transition) -> Result<StateID, BuildError>` — [`Transition`](../nfa/index.md#transition), [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a "range" NFA state.

  

  A "range" NFA state is a state with one outgoing transition to another

  state, where that transition may only be followed if the current input

  byte falls between a range of bytes given.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-sparse"></span>`fn add_sparse(&mut self, transitions: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](../nfa/index.md#transition), [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a "sparse" NFA state.

  

  A "sparse" NFA state contains zero or more outgoing transitions, where

  the transition to be followed (if any) is chosen based on whether the

  current input byte falls in the range of one such transition. The

  transitions given *must* be non-overlapping and in ascending order. (A

  "sparse" state with no transitions is equivalent to a "fail" state.)

  

  A "sparse" state is like adding a "union" state and pointing it at a

  bunch of "range" states, except that the different alternates have

  equal priority.

  

  Note that a "sparse" state is the only state that cannot be patched.

  This is because a "sparse" state has many transitions, each of which

  may point to a different NFA state. Moreover, adding more such

  transitions requires more than just an NFA state ID to point to. It

  also requires a byte range. The `patch` routine does not support the

  additional information required. Therefore, callers must ensure that

  all outgoing transitions for this state are included when `add_sparse`

  is called. There is no way to add more later.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

  

  # Panics

  

  This routine _may_ panic if the transitions given overlap or are not

  in ascending order.

- <span id="builder-add-look"></span>`fn add_look(&mut self, next: StateID, look: Look) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`Look`](../../../util/look/index.md#look), [`BuildError`](../error/index.md#builderror)

  Add a "look" NFA state.

  

  A "look" NFA state corresponds to a state with exactly one

  *conditional* epsilon transition to another NFA state. Namely, it

  represents one of a small set of simplistic look-around operators.

  

  Callers may provide a "dummy" state ID (typically `StateID::ZERO`),

  and then change it later with [`patch`](Builder::patch).

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-capture-start"></span>`fn add_capture_start(&mut self, next: StateID, group_index: u32, name: Option<Arc<str>>) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a "start capture" NFA state.

  

  A "start capture" NFA state corresponds to a state with exactly one

  outgoing unconditional epsilon transition to another state. Unlike

  "empty" states, a "start capture" state also carries with it an

  instruction for saving the current position of input to a particular

  location in memory. NFA simulations, like the Pike VM, may use this

  information to report the match locations of capturing groups in a

  regex pattern.

  

  If the corresponding capturing group has a name, then callers should

  include it here.

  

  Callers may provide a "dummy" state ID (typically `StateID::ZERO`),

  and then change it later with [`patch`](Builder::patch).

  

  Note that unlike `start_pattern`/`finish_pattern`, capturing start and

  end states may be interleaved. Indeed, it is typical for many "start

  capture" NFA states to appear before the first "end capture" state.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded or if the given

  capture index overflows `usize`.

  

  While the above are the only conditions in which this routine can

  currently return an error, it is possible to call this method with an

  inputs that results in the final `build()` step failing to produce an

  NFA. For example, if one adds two distinct capturing groups with the

  same name, then that will result in `build()` failing with an error.

  

  See the [`GroupInfo`](crate::util::captures::GroupInfo) type for

  more information on what qualifies as valid capturing groups.

  

  # Example

  

  This example shows that an error occurs when one tries to add multiple

  capturing groups with the same name to the same pattern.

  

  ```rust

  use regex_automata::{

      nfa::thompson::Builder,

      util::primitives::StateID,

  };

  

  let name = Some(std::sync::Arc::from("foo"));

  let mut builder = Builder::new();

  builder.start_pattern()?;

  // 0th capture group should always be unnamed.

  let start = builder.add_capture_start(StateID::ZERO, 0, None)?;

  // OK

  builder.add_capture_start(StateID::ZERO, 1, name.clone())?;

  // This is not OK, but 'add_capture_start' still succeeds. We don't

  // get an error until we call 'build' below. Without this call, the

  // call to 'build' below would succeed.

  builder.add_capture_start(StateID::ZERO, 2, name.clone())?;

  // Finish our pattern so we can try to build the NFA.

  builder.finish_pattern(start)?;

  let result = builder.build(start, start);

  assert!(result.is_err());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  However, adding multiple capturing groups with the same name to

  distinct patterns is okay:

  

  ```rust

  use std::sync::Arc;

  

  use regex_automata::{

      nfa::thompson::{pikevm::PikeVM, Builder, Transition},

      util::{

          captures::Captures,

          primitives::{PatternID, StateID},

      },

      Span,

  };

  

  // Hand-compile the patterns '(?P<foo>[a-z])' and '(?P<foo>[A-Z])'.

  let mut builder = Builder::new();

  // We compile them to support an unanchored search, which requires

  // adding an implicit '(?s-u:.)*?' prefix before adding either pattern.

  let unanchored_prefix = builder.add_union_reverse(vec![])?;

  let any = builder.add_range(Transition {

      start: b'\x00', end: b'\xFF', next: StateID::ZERO,

  })?;

  builder.patch(unanchored_prefix, any)?;

  builder.patch(any, unanchored_prefix)?;

  

  // Compile an alternation that permits matching multiple patterns.

  let alt = builder.add_union(vec![])?;

  builder.patch(unanchored_prefix, alt)?;

  

  // Compile '(?P<foo>[a-z]+)'.

  builder.start_pattern()?;

  let start0 = builder.add_capture_start(StateID::ZERO, 0, None)?;

  // N.B. 0th capture group must always be unnamed.

  let foo_start0 = builder.add_capture_start(

      StateID::ZERO, 1, Some(Arc::from("foo")),

  )?;

  let lowercase = builder.add_range(Transition {

      start: b'a', end: b'z', next: StateID::ZERO,

  })?;

  let foo_end0 = builder.add_capture_end(StateID::ZERO, 1)?;

  let end0 = builder.add_capture_end(StateID::ZERO, 0)?;

  let match0 = builder.add_match()?;

  builder.patch(start0, foo_start0)?;

  builder.patch(foo_start0, lowercase)?;

  builder.patch(lowercase, foo_end0)?;

  builder.patch(foo_end0, end0)?;

  builder.patch(end0, match0)?;

  builder.finish_pattern(start0)?;

  

  // Compile '(?P<foo>[A-Z]+)'.

  builder.start_pattern()?;

  let start1 = builder.add_capture_start(StateID::ZERO, 0, None)?;

  // N.B. 0th capture group must always be unnamed.

  let foo_start1 = builder.add_capture_start(

      StateID::ZERO, 1, Some(Arc::from("foo")),

  )?;

  let uppercase = builder.add_range(Transition {

      start: b'A', end: b'Z', next: StateID::ZERO,

  })?;

  let foo_end1 = builder.add_capture_end(StateID::ZERO, 1)?;

  let end1 = builder.add_capture_end(StateID::ZERO, 0)?;

  let match1 = builder.add_match()?;

  builder.patch(start1, foo_start1)?;

  builder.patch(foo_start1, uppercase)?;

  builder.patch(uppercase, foo_end1)?;

  builder.patch(foo_end1, end1)?;

  builder.patch(end1, match1)?;

  builder.finish_pattern(start1)?;

  

  // Now add the patterns to our alternation that we started above.

  builder.patch(alt, start0)?;

  builder.patch(alt, start1)?;

  

  // Finally build the NFA. The first argument is the anchored starting

  // state (the pattern alternation) where as the second is the

  // unanchored starting state (the unanchored prefix).

  let nfa = builder.build(alt, unanchored_prefix)?;

  

  // Now build a Pike VM from our NFA and access the 'foo' capture

  // group regardless of which pattern matched, since it is defined

  // for both patterns.

  let vm = PikeVM::new_from_nfa(nfa)?;

  let mut cache = vm.create_cache();

  let caps: Vec<Captures> =

      vm.captures_iter(&mut cache, "0123aAaAA").collect();

  assert_eq!(5, caps.len());

  

  assert_eq!(Some(PatternID::must(0)), caps[0].pattern());

  assert_eq!(Some(Span::from(4..5)), caps[0].get_group_by_name("foo"));

  

  assert_eq!(Some(PatternID::must(1)), caps[1].pattern());

  assert_eq!(Some(Span::from(5..6)), caps[1].get_group_by_name("foo"));

  

  assert_eq!(Some(PatternID::must(0)), caps[2].pattern());

  assert_eq!(Some(Span::from(6..7)), caps[2].get_group_by_name("foo"));

  

  assert_eq!(Some(PatternID::must(1)), caps[3].pattern());

  assert_eq!(Some(Span::from(7..8)), caps[3].get_group_by_name("foo"));

  

  assert_eq!(Some(PatternID::must(1)), caps[4].pattern());

  assert_eq!(Some(Span::from(8..9)), caps[4].get_group_by_name("foo"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-add-capture-end"></span>`fn add_capture_end(&mut self, next: StateID, group_index: u32) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a "end capture" NFA state.

  

  A "end capture" NFA state corresponds to a state with exactly one

  outgoing unconditional epsilon transition to another state. Unlike

  "empty" states, a "end capture" state also carries with it an

  instruction for saving the current position of input to a particular

  location in memory. NFA simulations, like the Pike VM, may use this

  information to report the match locations of capturing groups in a

  

  Callers may provide a "dummy" state ID (typically `StateID::ZERO`),

  and then change it later with [`patch`](Builder::patch).

  

  Note that unlike `start_pattern`/`finish_pattern`, capturing start and

  end states may be interleaved. Indeed, it is typical for many "start

  capture" NFA states to appear before the first "end capture" state.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded or if the given

  capture index overflows `usize`.

  

  While the above are the only conditions in which this routine can

  currently return an error, it is possible to call this method with an

  inputs that results in the final `build()` step failing to produce an

  NFA. For example, if one adds two distinct capturing groups with the

  same name, then that will result in `build()` failing with an error.

  

  See the [`GroupInfo`](crate::util::captures::GroupInfo) type for

  more information on what qualifies as valid capturing groups.

- <span id="builder-add-fail"></span>`fn add_fail(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Adds a "fail" NFA state.

  

  A "fail" state is simply a state that has no outgoing transitions. It

  acts as a way to cause a search to stop without reporting a match.

  For example, one way to represent an NFA with zero patterns is with a

  single "fail" state.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-match"></span>`fn add_match(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Adds a "match" NFA state.

  

  A "match" state has no outgoing transitions (just like a "fail"

  state), but it has special significance in that if a search enters

  this state, then a match has been found. The match state that is added

  automatically has the current pattern ID associated with it. This is

  used to report the matching pattern ID at search time.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

  

  # Panics

  

  This must be called after a `start_pattern` call but before the

  corresponding `finish_pattern` call. Otherwise, it panics.

- <span id="builder-add"></span>`fn add(&mut self, state: State) -> Result<StateID, BuildError>` — [`State`](#state), [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  The common implementation of "add a state." It handles the common

  error cases of state ID exhausting (by owning state ID allocation) and

  whether the size limit has been exceeded.

- <span id="builder-patch"></span>`fn patch(&mut self, from: StateID, to: StateID) -> Result<(), BuildError>` — [`StateID`](../../../util/primitives/index.md#stateid), [`BuildError`](../error/index.md#builderror)

  Add a transition from one state to another.

  

  This routine is called "patch" since it is very common to add the

  states you want, typically with "dummy" state ID transitions, and then

  "patch" in the real state IDs later. This is because you don't always

  know all of the necessary state IDs to add because they might not

  exist yet.

  

  # Errors

  

  This may error if patching leads to an increase in heap usage beyond

  the configured size limit. Heap usage only grows when patching adds a

  new transition (as in the case of a "union" state).

  

  # Panics

  

  This panics if `from` corresponds to a "sparse" state. When "sparse"

  states are added, there is no way to patch them after-the-fact. (If you

  have a use case where this would be helpful, please file an issue. It

  will likely require a new API.)

- <span id="builder-set-utf8"></span>`fn set_utf8(&mut self, yes: bool)`

  Set whether the NFA produced by this builder should only match UTF-8.

  

  This should be set when both of the following are true:

  

  1. The caller guarantees that the NFA created by this build will only

  report non-empty matches with spans that are valid UTF-8.

  2. The caller desires regex engines using this NFA to avoid reporting

  empty matches with a span that splits a valid UTF-8 encoded codepoint.

  

  Property (1) is not checked. Instead, this requires the caller to

  promise that it is true. Property (2) corresponds to the behavior of

  regex engines using the NFA created by this builder. Namely, there

  is no way in the NFA's graph itself to say that empty matches found

  by, for example, the regex `a*` will fall on valid UTF-8 boundaries.

  Instead, this option is used to communicate the UTF-8 semantic to regex

  engines that will typically implement it as a post-processing step by

  filtering out empty matches that don't fall on UTF-8 boundaries.

  

  If you're building an NFA from an HIR (and not using a

  [`thompson::Compiler`](crate::nfa::thompson::Compiler)), then you can

  use the [`syntax::Config::utf8`](crate::util::syntax::Config::utf8)

  option to guarantee that if the HIR detects a non-empty match, then it

  is guaranteed to be valid UTF-8.

  

  Note that property (2) does *not* specify the behavior of executing

  a search on a haystack that is not valid UTF-8. Therefore, if you're

  *not* running this NFA on strings that are guaranteed to be valid

  UTF-8, you almost certainly do not want to enable this option.

  Similarly, if you are running the NFA on strings that *are* guaranteed

  to be valid UTF-8, then you almost certainly want to enable this option

  unless you can guarantee that your NFA will never produce a zero-width

  match.

  

  It is disabled by default.

- <span id="builder-get-utf8"></span>`fn get_utf8(&self) -> bool`

  Returns whether UTF-8 mode is enabled for this builder.

  

  See `Builder::set_utf8` for more details about what "UTF-8 mode" is.

- <span id="builder-set-reverse"></span>`fn set_reverse(&mut self, yes: bool)`

  Sets whether the NFA produced by this builder should be matched in

  reverse or not. Generally speaking, when enabled, the NFA produced

  should be matched by moving backwards through a haystack, from a higher

  memory address to a lower memory address.

  

  See also `NFA::is_reverse` for more details.

  

  This is disabled by default, which means NFAs are by default matched

  in the forward direction.

- <span id="builder-get-reverse"></span>`fn get_reverse(&self) -> bool`

  Returns whether reverse mode is enabled for this builder.

  

  See `Builder::set_reverse` for more details about what "reverse mode"

  is.

- <span id="builder-set-look-matcher"></span>`fn set_look_matcher(&mut self, m: LookMatcher)` — [`LookMatcher`](../../../util/look/index.md#lookmatcher)

  Sets the look-around matcher that should be used for the resulting NFA.

  

  A look-around matcher can be used to configure how look-around

  assertions are matched. For example, a matcher might carry

  configuration that changes the line terminator used for `(?m:^)` and

  `(?m:$)` assertions.

- <span id="builder-get-look-matcher"></span>`fn get_look_matcher(&self) -> &LookMatcher` — [`LookMatcher`](../../../util/look/index.md#lookmatcher)

  Returns the look-around matcher used for this builder.

  

  If a matcher was not explicitly set, then `LookMatcher::default()` is

  returned.

- <span id="builder-set-size-limit"></span>`fn set_size_limit(&mut self, limit: Option<usize>) -> Result<(), BuildError>` — [`BuildError`](../error/index.md#builderror)

  Set the size limit on this builder.

  

  Setting the size limit will also check whether the NFA built so far

  fits within the given size limit. If it doesn't, then an error is

  returned.

  

  By default, there is no configured size limit.

- <span id="builder-get-size-limit"></span>`fn get_size_limit(&self) -> Option<usize>`

  Return the currently configured size limit.

  

  By default, this returns `None`, which corresponds to no configured

  size limit.

- <span id="builder-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, used by the NFA states added

  so far.

  

  Note that this is an approximation of how big the final NFA will be.

  In practice, the final NFA will likely be a bit smaller because of

  its simpler state representation. (For example, using things like

  `Box<[StateID]>` instead of `Vec<StateID>`.)

- <span id="builder-check-size-limit"></span>`fn check_size_limit(&self) -> Result<(), BuildError>` — [`BuildError`](../error/index.md#builderror)

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/builder.rs:28-128`](../../../../../.source_1765633015/regex-automata-0.4.13/src/nfa/thompson/builder.rs#L28-L128)*

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

- <span id="state-goto"></span>`fn goto(&self) -> Option<StateID>` — [`StateID`](../../../util/primitives/index.md#stateid)

  If this state is an unconditional epsilon transition, then this returns

  the target of the transition.

- <span id="state-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, of this state.

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

##### `impl Eq for State`

##### `impl<T> From for State`

- <span id="state-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for State`

- <span id="state-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for State`

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

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

