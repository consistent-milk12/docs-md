*[regex_automata](../../index.md) / [nfa](../index.md) / [thompson](index.md)*

---

# Module `thompson`

Defines a Thompson NFA and provides the [`PikeVM`](pikevm::PikeVM) and
[`BoundedBacktracker`](backtrack::BoundedBacktracker) regex engines.

A Thompson NFA (non-deterministic finite automaton) is arguably _the_ central
data type in this library. It is the result of what is commonly referred to as
"regex compilation." That is, turning a regex pattern from its concrete syntax
string into something that can run a search looks roughly like this:

* A `&str` is parsed into a [`regex-syntax::ast::Ast`](regex_syntax::ast::Ast).
* An `Ast` is translated into a [`regex-syntax::hir::Hir`](regex_syntax::hir::Hir).
* An `Hir` is compiled into a [`NFA`](nfa/index.md).
* The `NFA` is then used to build one of a few different regex engines:
  * An `NFA` is used directly in the `PikeVM` and `BoundedBacktracker` engines.
  * An `NFA` is used by a [hybrid NFA/DFA](crate::hybrid) to build out a DFA's
  transition table at search time.
  * An `NFA`, assuming it is one-pass, is used to build a full
  [one-pass DFA](crate::dfa::onepass) ahead of time.
  * An `NFA` is used to build a [full DFA](crate::dfa) ahead of time.

The [`meta`](crate::meta) regex engine makes all of these choices for you based
on various criteria. However, if you have a lower level use case, _you_ can
build any of the above regex engines and use them directly. But you must start
here by building an `NFA`.

# Details

It is perhaps worth expanding a bit more on what it means to go through the
`&str`->`Ast`->`Hir`->`NFA` process.

* Parsing a string into an `Ast` gives it a structured representation.
Crucially, the size and amount of work done in this step is proportional to the
size of the original string. No optimization or Unicode handling is done at
this point. This means that parsing into an `Ast` has very predictable costs.
Moreover, an `Ast` can be round-tripped back to its original pattern string as
written.
* Translating an `Ast` into an `Hir` is a process by which the structured
representation is simplified down to its most fundamental components.
Translation deals with flags such as case insensitivity by converting things
like `(?i:a)` to `[Aa]`. Translation is also where Unicode tables are consulted
to resolve things like `\p{Emoji}` and `\p{Greek}`. It also flattens each
character class, regardless of how deeply nested it is, into a single sequence
of non-overlapping ranges. All the various literal forms are thrown out in
favor of one common representation. Overall, the `Hir` is small enough to fit
into your head and makes analysis and other tasks much simpler.
* Compiling an `Hir` into an `NFA` formulates the regex into a finite state
machine whose transitions are defined over bytes. For example, an `Hir` might
have a Unicode character class corresponding to a sequence of ranges defined
in terms of `char`. Compilation is then responsible for turning those ranges
into a UTF-8 automaton. That is, an automaton that matches the UTF-8 encoding
of just the codepoints specified by those ranges. Otherwise, the main job of
an `NFA` is to serve as a byte-code of sorts for a virtual machine. It can be
seen as a sequence of instructions for how to match a regex.

## Contents

- [Modules](#modules)
  - [`backtrack`](#backtrack)
  - [`builder`](#builder)
  - [`compiler`](#compiler)
  - [`error`](#error)
  - [`literal_trie`](#literal-trie)
  - [`map`](#map)
  - [`nfa`](#nfa)
  - [`pikevm`](#pikevm)
  - [`range_trie`](#range-trie)
- [Structs](#structs)
  - [`Builder`](#builder)
  - [`BuildError`](#builderror)
  - [`DenseTransitions`](#densetransitions)
  - [`PatternIter`](#patterniter)
  - [`SparseTransitions`](#sparsetransitions)
  - [`Transition`](#transition)
  - [`NFA`](#nfa)
  - [`Compiler`](#compiler)
  - [`Config`](#config)
- [Enums](#enums)
  - [`State`](#state)
  - [`WhichCaptures`](#whichcaptures)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`backtrack`](#backtrack) | mod | An NFA backed bounded backtracker for executing regex searches with capturing groups. |
| [`builder`](#builder) | mod |  |
| [`compiler`](#compiler) | mod |  |
| [`error`](#error) | mod |  |
| [`literal_trie`](#literal-trie) | mod |  |
| [`map`](#map) | mod |  |
| [`nfa`](#nfa) | mod |  |
| [`pikevm`](#pikevm) | mod | An NFA backed Pike VM for executing regex searches with capturing groups. |
| [`range_trie`](#range-trie) | mod |  |
| [`Builder`](#builder) | struct |  |
| [`BuildError`](#builderror) | struct |  |
| [`DenseTransitions`](#densetransitions) | struct |  |
| [`PatternIter`](#patterniter) | struct |  |
| [`SparseTransitions`](#sparsetransitions) | struct |  |
| [`Transition`](#transition) | struct |  |
| [`NFA`](#nfa) | struct |  |
| [`Compiler`](#compiler) | struct |  |
| [`Config`](#config) | struct |  |
| [`State`](#state) | enum |  |
| [`WhichCaptures`](#whichcaptures) | enum |  |

## Modules

- [`backtrack`](backtrack/index.md) — An NFA backed bounded backtracker for executing regex searches with capturing
- [`builder`](builder/index.md)
- [`compiler`](compiler/index.md)
- [`error`](error/index.md)
- [`literal_trie`](literal_trie/index.md)
- [`map`](map/index.md)
- [`nfa`](nfa/index.md)
- [`pikevm`](pikevm/index.md) — An NFA backed Pike VM for executing regex searches with capturing groups.
- [`range_trie`](range_trie/index.md)

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/builder.rs:313-357`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/builder.rs#L313-L357)*

An abstraction for building Thompson NFAs by hand.

A builder is what a [`thompson::Compiler`](crate::nfa::thompson::Compiler)
uses internally to translate a regex's high-level intermediate
representation into an [`NFA`](nfa/index.md).

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

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](builder/index.md#builder)

  Create a new builder for hand-assembling NFAs.

- <span id="builder-clear"></span>`fn clear(&mut self)`

  Clear this builder.

  

  Clearing removes all state associated with building an NFA, but does

  not reset configuration (such as size limits and whether the NFA

  should only match UTF-8). After clearing, the builder can be reused to

  assemble an entirely new NFA.

- <span id="builder-build"></span>`fn build(&self, start_anchored: StateID, start_unanchored: StateID) -> Result<NFA, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Assemble a [`NFA`](nfa/index.md) from the states added so far.

  

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

- <span id="builder-start-pattern"></span>`fn start_pattern(&mut self) -> Result<PatternID, BuildError>` — [`PatternID`](../../util/primitives/index.md#patternid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-finish-pattern"></span>`fn finish_pattern(&mut self, start_id: StateID) -> Result<PatternID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`PatternID`](../../util/primitives/index.md#patternid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-current-pattern-id"></span>`fn current_pattern_id(&self) -> PatternID` — [`PatternID`](../../util/primitives/index.md#patternid)

  Returns the pattern identifier of the current pattern.

  

  # Panics

  

  If this doesn't occur after a `start_pattern` call and before the

  corresponding `finish_pattern` call, then this panics.

- <span id="builder-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the number of patterns added to this builder so far.

  

  This only includes patterns that have had `finish_pattern` called

  for them.

- <span id="builder-add-empty"></span>`fn add_empty(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

  Add an "empty" NFA state.

  

  An "empty" NFA state is a state with a single unconditional epsilon

  transition to another NFA state. Such empty states are removed before

  building the final [`NFA`](nfa/index.md) (which has no such "empty" states), but they

  can be quite useful in the construction process of an NFA.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-union"></span>`fn add_union(&mut self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-add-union-reverse"></span>`fn add_union_reverse(&mut self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-add-range"></span>`fn add_range(&mut self, trans: Transition) -> Result<StateID, BuildError>` — [`Transition`](nfa/index.md#transition), [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

  Add a "range" NFA state.

  

  A "range" NFA state is a state with one outgoing transition to another

  state, where that transition may only be followed if the current input

  byte falls between a range of bytes given.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-sparse"></span>`fn add_sparse(&mut self, transitions: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](nfa/index.md#transition), [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-add-look"></span>`fn add_look(&mut self, next: StateID, look: Look) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`Look`](../../util/look/index.md#look), [`BuildError`](error/index.md#builderror)

  Add a "look" NFA state.

  

  A "look" NFA state corresponds to a state with exactly one

  *conditional* epsilon transition to another NFA state. Namely, it

  represents one of a small set of simplistic look-around operators.

  

  Callers may provide a "dummy" state ID (typically `StateID::ZERO`),

  and then change it later with [`patch`](Builder::patch).

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-capture-start"></span>`fn add_capture_start(&mut self, next: StateID, group_index: u32, name: Option<Arc<str>>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-add-capture-end"></span>`fn add_capture_end(&mut self, next: StateID, group_index: u32) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-add-fail"></span>`fn add_fail(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

  Adds a "fail" NFA state.

  

  A "fail" state is simply a state that has no outgoing transitions. It

  acts as a way to cause a search to stop without reporting a match.

  For example, one way to represent an NFA with zero patterns is with a

  single "fail" state.

  

  # Errors

  

  This returns an error if the state identifier space is exhausted, or if

  the configured heap size limit has been exceeded.

- <span id="builder-add-match"></span>`fn add_match(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-add"></span>`fn add(&mut self, state: State) -> Result<StateID, BuildError>` — [`State`](builder/index.md#state), [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

  The common implementation of "add a state." It handles the common

  error cases of state ID exhausting (by owning state ID allocation) and

  whether the size limit has been exceeded.

- <span id="builder-patch"></span>`fn patch(&mut self, from: StateID, to: StateID) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md#stateid), [`BuildError`](error/index.md#builderror)

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

- <span id="builder-set-look-matcher"></span>`fn set_look_matcher(&mut self, m: LookMatcher)` — [`LookMatcher`](../../util/look/index.md#lookmatcher)

  Sets the look-around matcher that should be used for the resulting NFA.

  

  A look-around matcher can be used to configure how look-around

  assertions are matched. For example, a matcher might carry

  configuration that changes the line terminator used for `(?m:^)` and

  `(?m:$)` assertions.

- <span id="builder-get-look-matcher"></span>`fn get_look_matcher(&self) -> &LookMatcher` — [`LookMatcher`](../../util/look/index.md#lookmatcher)

  Returns the look-around matcher used for this builder.

  

  If a matcher was not explicitly set, then `LookMatcher::default()` is

  returned.

- <span id="builder-set-size-limit"></span>`fn set_size_limit(&mut self, limit: Option<usize>) -> Result<(), BuildError>` — [`BuildError`](error/index.md#builderror)

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

- <span id="builder-check-size-limit"></span>`fn check_size_limit(&self) -> Result<(), BuildError>` — [`BuildError`](error/index.md#builderror)

#### Trait Implementations

##### `impl Any for Builder`

- <span id="builder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Builder`

- <span id="builder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Builder`

- <span id="builder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](builder/index.md#builder)

##### `impl CloneToUninit for Builder`

- <span id="builder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Builder`

- <span id="builder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](builder/index.md#builder)

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

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/error.rs:21-23`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/error.rs#L21-L23)*

An error that can occurred during the construction of a thompson NFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`regex_syntax::Error`](../../../regex_syntax/hir/index.md) type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building an NFA directly from a pattern string.

Otherwise, errors typically occur when a limit has been breached. For
example, if the total heap usage of the compiled NFA exceeds the limit
set by [`Config::nfa_size_limit`](crate::nfa::thompson::Config), then
building the NFA will fail.

#### Implementations

- <span id="builderror-size-limit"></span>`fn size_limit(&self) -> Option<usize>`

  If this error occurred because the NFA exceeded the configured size

  limit before being built, then this returns the configured size limit.

  

  The limit returned is what was configured, and corresponds to the

  maximum amount of heap usage in bytes.

- <span id="builderror-kind"></span>`fn kind(&self) -> &BuildErrorKind` — [`BuildErrorKind`](error/index.md#builderrorkind)

- <span id="builderror-syntax"></span>`fn syntax(err: regex_syntax::Error) -> BuildError` — [`BuildError`](error/index.md#builderror)

- <span id="builderror-captures"></span>`fn captures(err: captures::GroupInfoError) -> BuildError` — [`GroupInfoError`](../../util/captures/index.md#groupinfoerror), [`BuildError`](error/index.md#builderror)

- <span id="builderror-word"></span>`fn word(err: look::UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../util/look/index.md#unicodewordboundaryerror), [`BuildError`](error/index.md#builderror)

- <span id="builderror-too-many-patterns"></span>`fn too_many_patterns(given: usize) -> BuildError` — [`BuildError`](error/index.md#builderror)

- <span id="builderror-too-many-states"></span>`fn too_many_states(given: usize) -> BuildError` — [`BuildError`](error/index.md#builderror)

- <span id="builderror-exceeded-size-limit"></span>`fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](error/index.md#builderror)

- <span id="builderror-invalid-capture-index"></span>`fn invalid_capture_index(index: u32) -> BuildError` — [`BuildError`](error/index.md#builderror)

- <span id="builderror-unsupported-captures"></span>`fn unsupported_captures() -> BuildError` — [`BuildError`](error/index.md#builderror)

#### Trait Implementations

##### `impl Any for BuildError`

- <span id="builderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildError`

- <span id="builderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildError`

- <span id="builderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](error/index.md#builderror)

##### `impl CloneToUninit for BuildError`

- <span id="builderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildError`

- <span id="builderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-error-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> From for BuildError`

- <span id="builderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildError`

- <span id="builderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildError`

- <span id="builderror-toowned-type-owned"></span>`type Owned = T`

- <span id="builderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for BuildError`

- <span id="builderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BuildError`

- <span id="builderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildError`

- <span id="builderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `DenseTransitions`

```rust
struct DenseTransitions {
    pub transitions: alloc::boxed::Box<[crate::util::primitives::StateID]>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1882-1886`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1882-L1886)*

A sequence of transitions used to represent a dense state.

This is the primary representation of a [`Dense`](State::Dense) state. It
provides constant time matching. That is, given a byte in a haystack and
a `DenseTransitions`, one can determine if the state matches in constant
time.

This is in contrast to `SparseTransitions`, whose time complexity is
necessarily bigger than constant time. Also in contrast, `DenseTransitions`
usually requires (much) more heap memory.

#### Fields

- **`transitions`**: `alloc::boxed::Box<[crate::util::primitives::StateID]>`

  A dense representation of this state's transitions on the heap. This
  always has length 256.

#### Implementations

- <span id="densetransitions-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a transition that

  doesn't correspond to `StateID::ZERO` for the byte `at` the given

  position in `haystack`.

  

  If `at >= haystack.len()`, then this returns `None`.

- <span id="densetransitions-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../util/alphabet/index.md#unit), [`StateID`](../../util/primitives/index.md#stateid)

  This follows the matching transition for any member of the alphabet.

  

  The matching transition is found by looking for a transition that

  doesn't correspond to `StateID::ZERO` for the given alphabet `unit`.

  

  If the given alphabet unit is [`EOI`](alphabet::Unit::eoi), then

  this returns `None`.

- <span id="densetransitions-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a transition that

  doesn't correspond to `StateID::ZERO` for the given `byte`.

- <span id="densetransitions-iter"></span>`fn iter(&self) -> impl Iterator<Item = Transition> + '_` — [`Transition`](nfa/index.md#transition)

  Returns an iterator over all transitions that don't point to

  `StateID::ZERO`.

#### Trait Implementations

##### `impl Any for DenseTransitions`

- <span id="densetransitions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DenseTransitions`

- <span id="densetransitions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DenseTransitions`

- <span id="densetransitions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DenseTransitions`

- <span id="densetransitions-clone"></span>`fn clone(&self) -> DenseTransitions` — [`DenseTransitions`](nfa/index.md#densetransitions)

##### `impl CloneToUninit for DenseTransitions`

- <span id="densetransitions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DenseTransitions`

- <span id="densetransitions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for DenseTransitions`

##### `impl<T> From for DenseTransitions`

- <span id="densetransitions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DenseTransitions`

- <span id="densetransitions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for DenseTransitions`

- <span id="densetransitions-partialeq-eq"></span>`fn eq(&self, other: &DenseTransitions) -> bool` — [`DenseTransitions`](nfa/index.md#densetransitions)

##### `impl StructuralPartialEq for DenseTransitions`

##### `impl ToOwned for DenseTransitions`

- <span id="densetransitions-toowned-type-owned"></span>`type Owned = T`

- <span id="densetransitions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="densetransitions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for DenseTransitions`

- <span id="densetransitions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="densetransitions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for DenseTransitions`

- <span id="densetransitions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="densetransitions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternIter<'a>`

```rust
struct PatternIter<'a> {
    it: crate::util::primitives::PatternIDIter,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:2023-2031`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L2023-L2031)*

An iterator over all pattern IDs in an NFA.

This iterator is created by `NFA::patterns`.

The lifetime parameter `'a` refers to the lifetime of the NFA from which
this pattern iterator was created.

#### Fields

- **`_marker`**: `core::marker::PhantomData<&'a ()>`

  We explicitly associate a lifetime with this iterator even though we
  don't actually borrow anything from the NFA. We do this for backward
  compatibility purposes. If we ever do need to borrow something from
  the NFA, then we can and just get rid of this marker without breaking
  the public API.

#### Trait Implementations

##### `impl Any for PatternIter<'a>`

- <span id="patterniter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternIter<'a>`

- <span id="patterniter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternIter<'a>`

- <span id="patterniter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for PatternIter<'a>`

- <span id="patterniter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for PatternIter<'a>`

- <span id="patterniter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternIter<'a>`

- <span id="patterniter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for PatternIter<'a>`

- <span id="patterniter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patterniter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="patterniter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternIter<'a>`

- <span id="patterniter-iterator-type-item"></span>`type Item = PatternID`

- <span id="patterniter-iterator-next"></span>`fn next(&mut self) -> Option<PatternID>` — [`PatternID`](../../util/primitives/index.md#patternid)

##### `impl<U> TryFrom for PatternIter<'a>`

- <span id="patterniter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterniter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternIter<'a>`

- <span id="patterniter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterniter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SparseTransitions`

```rust
struct SparseTransitions {
    pub transitions: alloc::boxed::Box<[Transition]>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1795-1798`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1795-L1798)*

A sequence of transitions used to represent a sparse state.

This is the primary representation of a [`Sparse`](State::Sparse) state.
It corresponds to a sorted sequence of transitions with non-overlapping
byte ranges. If the byte at the current position in the haystack matches
one of the byte ranges, then the finite state machine should take the
corresponding transition.

#### Fields

- **`transitions`**: `alloc::boxed::Box<[Transition]>`

  The sorted sequence of non-overlapping transitions.

#### Implementations

- <span id="sparsetransitions-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a matching byte

  range (there is at most one) corresponding to the position `at` in

  `haystack`.

  

  If `at >= haystack.len()`, then this returns `None`.

- <span id="sparsetransitions-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../util/alphabet/index.md#unit), [`StateID`](../../util/primitives/index.md#stateid)

  This follows the matching transition for any member of the alphabet.

  

  The matching transition is found by looking for a matching byte

  range (there is at most one) corresponding to the position `at` in

  `haystack`. If the given alphabet unit is [`EOI`](alphabet::Unit::eoi),

  then this always returns `None`.

- <span id="sparsetransitions-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md#stateid)

  This follows the matching transition for a particular byte.

  

  The matching transition is found by looking for a matching byte range

  (there is at most one) corresponding to the byte given.

#### Trait Implementations

##### `impl Any for SparseTransitions`

- <span id="sparsetransitions-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SparseTransitions`

- <span id="sparsetransitions-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SparseTransitions`

- <span id="sparsetransitions-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SparseTransitions`

- <span id="sparsetransitions-clone"></span>`fn clone(&self) -> SparseTransitions` — [`SparseTransitions`](nfa/index.md#sparsetransitions)

##### `impl CloneToUninit for SparseTransitions`

- <span id="sparsetransitions-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SparseTransitions`

- <span id="sparsetransitions-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SparseTransitions`

##### `impl<T> From for SparseTransitions`

- <span id="sparsetransitions-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SparseTransitions`

- <span id="sparsetransitions-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for SparseTransitions`

- <span id="sparsetransitions-partialeq-eq"></span>`fn eq(&self, other: &SparseTransitions) -> bool` — [`SparseTransitions`](nfa/index.md#sparsetransitions)

##### `impl StructuralPartialEq for SparseTransitions`

##### `impl ToOwned for SparseTransitions`

- <span id="sparsetransitions-toowned-type-owned"></span>`type Owned = T`

- <span id="sparsetransitions-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="sparsetransitions-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SparseTransitions`

- <span id="sparsetransitions-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="sparsetransitions-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SparseTransitions`

- <span id="sparsetransitions-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="sparsetransitions-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Transition`

```rust
struct Transition {
    pub start: u8,
    pub end: u8,
    pub next: crate::util::primitives::StateID,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1965-1972`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1965-L1972)*

A single transition to another state.

This transition may only be followed if the current byte in the haystack
falls in the inclusive range of bytes specified.

#### Fields

- **`start`**: `u8`

  The inclusive start of the byte range.

- **`end`**: `u8`

  The inclusive end of the byte range.

- **`next`**: `crate::util::primitives::StateID`

  The identifier of the state to transition to.

#### Implementations

- <span id="transition-matches"></span>`fn matches(&self, haystack: &[u8], at: usize) -> bool`

  Returns true if the position `at` in `haystack` falls in this

  transition's range of bytes.

  

  If `at >= haystack.len()`, then this returns `false`.

- <span id="transition-matches-unit"></span>`fn matches_unit(&self, unit: alphabet::Unit) -> bool` — [`Unit`](../../util/alphabet/index.md#unit)

  Returns true if the given alphabet unit falls in this transition's

  range of bytes. If the given unit is [`EOI`](alphabet::Unit::eoi), then

  this returns `false`.

- <span id="transition-matches-byte"></span>`fn matches_byte(&self, byte: u8) -> bool`

  Returns true if the given byte falls in this transition's range of

  bytes.

#### Trait Implementations

##### `impl Any for Transition`

- <span id="transition-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Transition`

- <span id="transition-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Transition`

- <span id="transition-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](nfa/index.md#transition)

##### `impl CloneToUninit for Transition`

- <span id="transition-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- <span id="transition-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Transition`

##### `impl<T> From for Transition`

- <span id="transition-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Transition`

- <span id="transition-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Transition`

- <span id="transition-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Transition`

- <span id="transition-partialeq-eq"></span>`fn eq(&self, other: &Transition) -> bool` — [`Transition`](nfa/index.md#transition)

##### `impl StructuralPartialEq for Transition`

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

### `NFA`

```rust
struct NFA(alloc::sync::Arc<Inner>);
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:190-202`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L190-L202)*

A byte oriented Thompson non-deterministic finite automaton (NFA).

A Thompson NFA is a finite state machine that permits unconditional epsilon
transitions, but guarantees that there exists at most one non-epsilon
transition for each element in the alphabet for each state.

An NFA may be used directly for searching, for analysis or to build
a deterministic finite automaton (DFA).

# Cheap clones

Since an NFA is a core data type in this crate that many other regex
engines are based on top of, it is convenient to give ownership of an NFA
to said regex engines. Because of this, an NFA uses reference counting
internally. Therefore, it is cheap to clone and it is encouraged to do so.

# Capabilities

Using an NFA for searching via the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) provides the most amount
of "power" of any regex engine in this crate. Namely, it supports the
following in all cases:

1. Detection of a match.
2. Location of a match, including both the start and end offset, in a
single pass of the haystack.
3. Location of matching capturing groups.
4. Handles multiple patterns, including (1)-(3) when multiple patterns are
present.

# Capturing Groups

Groups refer to parenthesized expressions inside a regex pattern. They look
like this, where `exp` is an arbitrary regex:

* `(exp)` - An unnamed capturing group.
* `(?P<name>exp)` or `(?<name>exp)` - A named capturing group.
* `(?:exp)` - A non-capturing group.
* `(?i:exp)` - A non-capturing group that sets flags.

Only the first two forms are said to be _capturing_. Capturing
means that the last position at which they match is reportable. The
[`Captures`](crate::util::captures::Captures) type provides convenient
access to the match positions of capturing groups, which includes looking
up capturing groups by their name.

# Byte oriented

This NFA is byte oriented, which means that all of its transitions are
defined on bytes. In other words, the alphabet of an NFA consists of the
256 different byte values.

While DFAs nearly demand that they be byte oriented for performance
reasons, an NFA could conceivably be *Unicode codepoint* oriented. Indeed,
a previous version of this NFA supported both byte and codepoint oriented
modes. A codepoint oriented mode can work because an NFA fundamentally uses
a sparse representation of transitions, which works well with the large
sparse space of Unicode codepoints.

Nevertheless, this NFA is only byte oriented. This choice is primarily
driven by implementation simplicity, and also in part memory usage. In
practice, performance between the two is roughly comparable. However,
building a DFA (including a hybrid DFA) really wants a byte oriented NFA.
So if we do have a codepoint oriented NFA, then we also need to generate
byte oriented NFA in order to build an hybrid NFA/DFA. Thus, by only
generating byte oriented NFAs, we can produce one less NFA. In other words,
if we made our NFA codepoint oriented, we'd need to *also* make it support
a byte oriented mode, which is more complicated. But a byte oriented mode
can support everything.

# Differences with DFAs

At the theoretical level, the precise difference between an NFA and a DFA
is that, in a DFA, for every state, an input symbol unambiguously refers
to a single transition _and_ that an input symbol is required for each
transition. At a practical level, this permits DFA implementations to be
implemented at their core with a small constant number of CPU instructions
for each byte of input searched. In practice, this makes them quite a bit
faster than NFAs _in general_. Namely, in order to execute a search for any
Thompson NFA, one needs to keep track of a _set_ of states, and execute
the possible transitions on all of those states for each input symbol.
Overall, this results in much more overhead. To a first approximation, one
can expect DFA searches to be about an order of magnitude faster.

So why use an NFA at all? The main advantage of an NFA is that it takes
linear time (in the size of the pattern string after repetitions have been
expanded) to build and linear memory usage. A DFA, on the other hand, may
take exponential time and/or space to build. Even in non-pathological
cases, DFAs often take quite a bit more memory than their NFA counterparts,
_especially_ if large Unicode character classes are involved. Of course,
an NFA also provides additional capabilities. For example, it can match
Unicode word boundaries on non-ASCII text and resolve the positions of
capturing groups.

Note that a [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) strikes a
good balance between an NFA and a DFA. It avoids the exponential build time
of a DFA while maintaining its fast search time. The downside of a hybrid
NFA/DFA is that in some cases it can be slower at search time than the NFA.
(It also has less functionality than a pure NFA. It cannot handle Unicode
word boundaries on non-ASCII text and cannot resolve capturing groups.)

# Example

This shows how to build an NFA with the default configuration and execute a
search using the Pike VM.

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"foo[0-9]+")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let expected = Some(Match::must(0, 0..8));
re.captures(&mut cache, b"foo12345", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: resolving capturing groups

This example shows how to parse some simple dates and extract the
components of each date via capturing groups.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::captures::Captures,
};

let vm = PikeVM::new(r"(?P<y>\d{4})-(?P<m>\d{2})-(?P<d>\d{2})")?;
let mut cache = vm.create_cache();

let haystack = "2012-03-14, 2013-01-01 and 2014-07-05";
let all: Vec<Captures> = vm.captures_iter(
    &mut cache, haystack.as_bytes()
).collect();
// There should be a total of 3 matches.
assert_eq!(3, all.len());
// The year from the second match is '2013'.
let span = all[1].get_group_by_name("y").unwrap();
assert_eq!("2013", &haystack[span]);

Ok::<(), Box<dyn std::error::Error>>(())
```

This example shows that only the last match of a capturing group is
reported, even if it had to match multiple times for an overall match
to occur.

```rust
use regex_automata::{nfa::thompson::pikevm::PikeVM, Span};

let re = PikeVM::new(r"([a-z]){4}")?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();

let haystack = b"quux";
re.captures(&mut cache, haystack, &mut caps);
assert!(caps.is_match());
assert_eq!(Some(Span::from(3..4)), caps.get_group(1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="nfa-new"></span>`fn new(pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Parse the given regular expression using a default configuration and

  build an NFA from it.

  

  If you want a non-default configuration, then use the NFA

  [`Compiler`](compiler/index.md) with a [`Config`](compiler/index.md).

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::new(r"foo[0-9]+")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(0, 0..8));

  re.captures(&mut cache, b"foo12345", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Parse the given regular expressions using a default configuration and

  build a multi-NFA from them.

  

  If you want a non-default configuration, then use the NFA

  [`Compiler`](compiler/index.md) with a [`Config`](compiler/index.md).

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

  

  let re = PikeVM::new_many(&["[0-9]+", "[a-z]+"])?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(1, 0..3));

  re.captures(&mut cache, b"foo12345bar", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-always-match"></span>`fn always_match() -> NFA` — [`NFA`](nfa/index.md#nfa)

  Returns an NFA with a single regex pattern that always matches at every

  position.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  

  let re = PikeVM::new_from_nfa(NFA::always_match())?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(0, 0..0));

  re.captures(&mut cache, b"", &mut caps);

  assert_eq!(expected, caps.get_match());

  re.captures(&mut cache, b"foo", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-never-match"></span>`fn never_match() -> NFA` — [`NFA`](nfa/index.md#nfa)

  Returns an NFA that never matches at any position.

  

  This is a convenience routine for creating an NFA with zero patterns.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, pikevm::PikeVM};

  

  let re = PikeVM::new_from_nfa(NFA::never_match())?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  re.captures(&mut cache, b"", &mut caps);

  assert!(!caps.is_match());

  re.captures(&mut cache, b"foo", &mut caps);

  assert!(!caps.is_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-config"></span>`fn config() -> Config` — [`Config`](compiler/index.md#config)

  Return a default configuration for an `NFA`.

  

  This is a convenience routine to avoid needing to import the `Config`

  type when customizing the construction of an NFA.

  

  # Example

  

  This example shows how to build an NFA with a small size limit that

  results in a compilation error for any regex that tries to use more

  heap memory than the configured limit.

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, pikevm::PikeVM};

  

  let result = PikeVM::builder()

      .thompson(NFA::config().nfa_size_limit(Some(1_000)))

      // Remember, \w is Unicode-aware by default and thus huge.

      .build(r"\w+");

  assert!(result.is_err());

  ```

- <span id="nfa-compiler"></span>`fn compiler() -> Compiler` — [`Compiler`](compiler/index.md#compiler)

  Return a compiler for configuring the construction of an `NFA`.

  

  This is a convenience routine to avoid needing to import the

  [`Compiler`](compiler/index.md) type in common cases.

  

  # Example

  

  This example shows how to build an NFA that is permitted match invalid

  UTF-8. Without the additional syntax configuration here, compilation of

  `(?-u:.)` would fail because it is permitted to match invalid UTF-8.

  

  ```rust

  use regex_automata::{

      nfa::thompson::pikevm::PikeVM,

      util::syntax,

      Match,

  };

  

  let re = PikeVM::builder()

      .syntax(syntax::Config::new().utf8(false))

      .build(r"[a-z]+(?-u:.)")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  let expected = Some(Match::must(0, 1..5));

  re.captures(&mut cache, b"\xFFabc\xFF", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-patterns"></span>`fn patterns(&self) -> PatternIter<'_>` — [`PatternIter`](nfa/index.md#patterniter)

  Returns an iterator over all pattern identifiers in this NFA.

  

  Pattern IDs are allocated in sequential order starting from zero,

  where the order corresponds to the order of patterns provided to the

  `NFA::new_many` constructor.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, PatternID};

  

  let nfa = NFA::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  let pids: Vec<PatternID> = nfa.patterns().collect();

  assert_eq!(pids, vec![

      PatternID::must(0),

      PatternID::must(1),

      PatternID::must(2),

  ]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the total number of regex patterns in this NFA.

  

  This may return zero if the NFA was constructed with no patterns. In

  this case, the NFA can never produce a match for any input.

  

  This is guaranteed to be no bigger than `PatternID::LIMIT` because

  NFA construction will fail if too many patterns are added.

  

  It is always true that `nfa.patterns().count() == nfa.pattern_len()`.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let nfa = NFA::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  assert_eq!(3, nfa.pattern_len());

  

  let nfa = NFA::never_match();

  assert_eq!(0, nfa.pattern_len());

  

  let nfa = NFA::always_match();

  assert_eq!(1, nfa.pattern_len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-start-anchored"></span>`fn start_anchored(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Return the state identifier of the initial anchored state of this NFA.

  

  The returned identifier is guaranteed to be a valid index into the

  slice returned by `NFA::states`, and is also a valid argument to

  `NFA::state`.

  

  # Example

  

  This example shows a somewhat contrived example where we can easily

  predict the anchored starting state.

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, State, WhichCaptures};

  

  let nfa = NFA::compiler()

      .configure(NFA::config().which_captures(WhichCaptures::None))

      .build("a")?;

  let state = nfa.state(nfa.start_anchored());

  match *state {

      State::ByteRange { trans } => {

          assert_eq!(b'a', trans.start);

          assert_eq!(b'a', trans.end);

      }

      _ => unreachable!("unexpected state"),

  }

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-start-unanchored"></span>`fn start_unanchored(&self) -> StateID` — [`StateID`](../../util/primitives/index.md#stateid)

  Return the state identifier of the initial unanchored state of this

  NFA.

  

  This is equivalent to the identifier returned by

  `NFA::start_anchored` when the NFA has no unanchored starting state.

  

  The returned identifier is guaranteed to be a valid index into the

  slice returned by `NFA::states`, and is also a valid argument to

  `NFA::state`.

  

  # Example

  

  This example shows that the anchored and unanchored starting states

  are equivalent when an anchored NFA is built.

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let nfa = NFA::new("^a")?;

  assert_eq!(nfa.start_anchored(), nfa.start_unanchored());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-start-pattern"></span>`fn start_pattern(&self, pid: PatternID) -> Option<StateID>` — [`PatternID`](../../util/primitives/index.md#patternid), [`StateID`](../../util/primitives/index.md#stateid)

  Return the state identifier of the initial anchored state for the given

  pattern, or `None` if there is no pattern corresponding to the given

  identifier.

  

  If one uses the starting state for a particular pattern, then the only

  match that can be returned is for the corresponding pattern.

  

  The returned identifier is guaranteed to be a valid index into the

  slice returned by `NFA::states`, and is also a valid argument to

  `NFA::state`.

  

  # Errors

  

  If the pattern doesn't exist in this NFA, then this returns an error.

  This occurs when `pid.as_usize() >= nfa.pattern_len()`.

  

  # Example

  

  This example shows that the anchored and unanchored starting states

  are equivalent when an anchored NFA is built.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, PatternID};

  

  let nfa = NFA::new_many(&["^a", "^b"])?;

  // The anchored and unanchored states for the entire NFA are the same,

  // since all of the patterns are anchored.

  assert_eq!(nfa.start_anchored(), nfa.start_unanchored());

  // But the anchored starting states for each pattern are distinct,

  // because these starting states can only lead to matches for the

  // corresponding pattern.

  let anchored = Some(nfa.start_anchored());

  assert_ne!(anchored, nfa.start_pattern(PatternID::must(0)));

  assert_ne!(anchored, nfa.start_pattern(PatternID::must(1)));

  // Requesting a pattern not in the NFA will result in None:

  assert_eq!(None, nfa.start_pattern(PatternID::must(2)));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-byte-class-set"></span>`fn byte_class_set(&self) -> &ByteClassSet` — [`ByteClassSet`](../../util/alphabet/index.md#byteclassset)

  Get the byte class set for this NFA.

  

  A byte class set is a partitioning of this NFA's alphabet into

  equivalence classes. Any two bytes in the same equivalence class are

  guaranteed to never discriminate between a match or a non-match. (The

  partitioning may not be minimal.)

  

  Byte classes are used internally by this crate when building DFAs.

  Namely, among other optimizations, they enable a space optimization

  where the DFA's internal alphabet is defined over the equivalence

  classes of bytes instead of all possible byte values. The former is

  often quite a bit smaller than the latter, which permits the DFA to use

  less space for its transition table.

- <span id="nfa-byte-classes"></span>`fn byte_classes(&self) -> &ByteClasses` — [`ByteClasses`](../../util/alphabet/index.md#byteclasses)

  Get the byte classes for this NFA.

  

  Byte classes represent a partitioning of this NFA's alphabet into

  equivalence classes. Any two bytes in the same equivalence class are

  guaranteed to never discriminate between a match or a non-match. (The

  partitioning may not be minimal.)

  

  Byte classes are used internally by this crate when building DFAs.

  Namely, among other optimizations, they enable a space optimization

  where the DFA's internal alphabet is defined over the equivalence

  classes of bytes instead of all possible byte values. The former is

  often quite a bit smaller than the latter, which permits the DFA to use

  less space for its transition table.

  

  # Example

  

  This example shows how to query the class of various bytes.

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let nfa = NFA::new("[a-z]+")?;

  let classes = nfa.byte_classes();

  // 'a' and 'z' are in the same class for this regex.

  assert_eq!(classes.get(b'a'), classes.get(b'z'));

  // But 'a' and 'A' are not.

  assert_ne!(classes.get(b'a'), classes.get(b'A'));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-state"></span>`fn state(&self, id: StateID) -> &State` — [`StateID`](../../util/primitives/index.md#stateid), [`State`](nfa/index.md#state)

  Return a reference to the NFA state corresponding to the given ID.

  

  This is a convenience routine for `nfa.states()[id]`.

  

  # Panics

  

  This panics when the given identifier does not reference a valid state.

  That is, when `id.as_usize() >= nfa.states().len()`.

  

  # Example

  

  The anchored state for a pattern will typically correspond to a

  capturing state for that pattern. (Although, this is not an API

  guarantee!)

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, State}, PatternID};

  

  let nfa = NFA::new("a")?;

  let state = nfa.state(nfa.start_pattern(PatternID::ZERO).unwrap());

  match *state {

      State::Capture { slot, .. } => {

          assert_eq!(0, slot.as_usize());

      }

      _ => unreachable!("unexpected state"),

  }

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-states"></span>`fn states(&self) -> &[State]` — [`State`](nfa/index.md#state)

  Returns a slice of all states in this NFA.

  

  The slice returned is indexed by `StateID`. This provides a convenient

  way to access states while following transitions among those states.

  

  # Example

  

  This demonstrates that disabling UTF-8 mode can shrink the size of the

  NFA considerably in some cases, especially when using Unicode character

  classes.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::NFA;

  

  let nfa_unicode = NFA::new(r"\w")?;

  let nfa_ascii = NFA::new(r"(?-u)\w")?;

  // Yes, a factor of 45 difference. No lie.

  assert!(40 * nfa_ascii.states().len() < nfa_unicode.states().len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-group-info"></span>`fn group_info(&self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md#groupinfo)

  Returns the capturing group info for this NFA.

  

  The [`GroupInfo`](../../util/captures/index.md) provides a way to map to and from capture index

  and capture name for each pattern. It also provides a mapping from

  each of the capturing groups in every pattern to their corresponding

  slot offsets encoded in [`State::Capture`](../../index.md) states.

  

  Note that `GroupInfo` uses reference counting internally, such that

  cloning a `GroupInfo` is very cheap.

  

  # Example

  

  This example shows how to get a list of all capture group names for

  a particular pattern.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, PatternID};

  

  let nfa = NFA::new(r"(a)(?P<foo>b)(c)(d)(?P<bar>e)")?;

  // The first is the implicit group that is always unnamed. The next

  // 5 groups are the explicit groups found in the concrete syntax above.

  let expected = vec![None, None, Some("foo"), None, None, Some("bar")];

  let got: Vec<Option<&str>> =

      nfa.group_info().pattern_names(PatternID::ZERO).collect();

  assert_eq!(expected, got);

  

  // Using an invalid pattern ID will result in nothing yielded.

  let got = nfa.group_info().pattern_names(PatternID::must(999)).count();

  assert_eq!(0, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-has-capture"></span>`fn has_capture(&self) -> bool`

  Returns true if and only if this NFA has at least one

  [`Capture`](State::Capture) in its sequence of states.

  

  This is useful as a way to perform a quick test before attempting

  something that does or does not require capture states. For example,

  some regex engines (like the PikeVM) require capture states in order to

  work at all.

  

  # Example

  

  This example shows a few different NFAs and whether they have captures

  or not.

  

  ```rust

  use regex_automata::nfa::thompson::{NFA, WhichCaptures};

  

  // Obviously has capture states.

  let nfa = NFA::new("(a)")?;

  assert!(nfa.has_capture());

  

  // Less obviously has capture states, because every pattern has at

  // least one anonymous capture group corresponding to the match for the

  // entire pattern.

  let nfa = NFA::new("a")?;

  assert!(nfa.has_capture());

  

  // Other than hand building your own NFA, this is the only way to build

  // an NFA without capturing groups. In general, you should only do this

  // if you don't intend to use any of the NFA-oriented regex engines.

  // Overall, capturing groups don't have many downsides. Although they

  // can add a bit of noise to simple NFAs, so it can be nice to disable

  // them for debugging purposes.

  //

  // Notice that 'has_capture' is false here even when we have an

  // explicit capture group in the pattern.

  let nfa = NFA::compiler()

      .configure(NFA::config().which_captures(WhichCaptures::None))

      .build("(a)")?;

  assert!(!nfa.has_capture());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-has-empty"></span>`fn has_empty(&self) -> bool`

  Returns true if and only if this NFA can match the empty string.

  When it returns false, all possible matches are guaranteed to have a

  non-zero length.

  

  This is useful as cheap way to know whether code needs to handle the

  case of a zero length match. This is particularly important when UTF-8

  modes are enabled, as when UTF-8 mode is enabled, empty matches that

  split a codepoint must never be reported. This extra handling can

  sometimes be costly, and since regexes matching an empty string are

  somewhat rare, it can be beneficial to treat such regexes specially.

  

  # Example

  

  This example shows a few different NFAs and whether they match the

  empty string or not. Notice the empty string isn't merely a matter

  of a string of length literally `0`, but rather, whether a match can

  occur between specific pairs of bytes.

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::syntax};

  

  // The empty regex matches the empty string.

  let nfa = NFA::new("")?;

  assert!(nfa.has_empty(), "empty matches empty");

  // The '+' repetition operator requires at least one match, and so

  // does not match the empty string.

  let nfa = NFA::new("a+")?;

  assert!(!nfa.has_empty(), "+ does not match empty");

  // But the '*' repetition operator does.

  let nfa = NFA::new("a*")?;

  assert!(nfa.has_empty(), "* does match empty");

  // And wrapping '+' in an operator that can match an empty string also

  // causes it to match the empty string too.

  let nfa = NFA::new("(a+)*")?;

  assert!(nfa.has_empty(), "+ inside of * matches empty");

  

  // If a regex is just made of a look-around assertion, even if the

  // assertion requires some kind of non-empty string around it (such as

  // \b), then it is still treated as if it matches the empty string.

  // Namely, if a match occurs of just a look-around assertion, then the

  // match returned is empty.

  let nfa = NFA::compiler()

      .syntax(syntax::Config::new().utf8(false))

      .build(r"^$\A\z\b\B(?-u:\b\B)")?;

  assert!(nfa.has_empty(), "assertions match empty");

  // Even when an assertion is wrapped in a '+', it still matches the

  // empty string.

  let nfa = NFA::new(r"\b+")?;

  assert!(nfa.has_empty(), "+ of an assertion matches empty");

  

  // An alternation with even one branch that can match the empty string

  // is also said to match the empty string overall.

  let nfa = NFA::new("foo|(bar)?|quux")?;

  assert!(nfa.has_empty(), "alternations can match empty");

  

  // An NFA that matches nothing does not match the empty string.

  let nfa = NFA::new("[a&&b]")?;

  assert!(!nfa.has_empty(), "never matching means not matching empty");

  // But if it's wrapped in something that doesn't require a match at

  // all, then it can match the empty string!

  let nfa = NFA::new("[a&&b]*")?;

  assert!(nfa.has_empty(), "* on never-match still matches empty");

  // Since a '+' requires a match, using it on something that can never

  // match will itself produce a regex that can never match anything,

  // and thus does not match the empty string.

  let nfa = NFA::new("[a&&b]+")?;

  assert!(!nfa.has_empty(), "+ on never-match still matches nothing");

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-is-utf8"></span>`fn is_utf8(&self) -> bool`

  Whether UTF-8 mode is enabled for this NFA or not.

  

  When UTF-8 mode is enabled, all matches reported by a regex engine

  derived from this NFA are guaranteed to correspond to spans of valid

  UTF-8. This includes zero-width matches. For example, the regex engine

  must guarantee that the empty regex will not match at the positions

  between code units in the UTF-8 encoding of a single codepoint.

  

  See `Config::utf8` for more information.

  

  This is enabled by default.

  

  # Example

  

  This example shows how UTF-8 mode can impact the match spans that may

  be reported in certain cases.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      Match, Input,

  };

  

  let re = PikeVM::new("")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  // UTF-8 mode is enabled by default.

  let mut input = Input::new("☃");

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 0..0)), caps.get_match());

  

  // Even though an empty regex matches at 1..1, our next match is

  // 3..3 because 1..1 and 2..2 split the snowman codepoint (which is

  // three bytes long).

  input.set_start(1);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 3..3)), caps.get_match());

  

  // But if we disable UTF-8, then we'll get matches at 1..1 and 2..2:

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().utf8(false))

      .build("")?;

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 1..1)), caps.get_match());

  

  input.set_start(2);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 2..2)), caps.get_match());

  

  input.set_start(3);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 3..3)), caps.get_match());

  

  input.set_start(4);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(None, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-is-reverse"></span>`fn is_reverse(&self) -> bool`

  Returns true when this NFA is meant to be matched in reverse.

  

  Generally speaking, when this is true, it means the NFA is supposed to

  be used in conjunction with moving backwards through the haystack. That

  is, from a higher memory address to a lower memory address.

  

  It is often the case that lower level routines dealing with an NFA

  don't need to care about whether it is "meant" to be matched in reverse

  or not. However, there are some specific cases where it matters. For

  example, the implementation of CRLF-aware `^` and `$` line anchors

  needs to know whether the search is in the forward or reverse

  direction. In the forward direction, neither `^` nor `$` should match

  when a `\r` has been seen previously and a `\n` is next. However, in

  the reverse direction, neither `^` nor `$` should match when a `\n`

  has been seen previously and a `\r` is next. This fundamentally changes

  how the state machine is constructed, and thus needs to be altered

  based on the direction of the search.

  

  This is automatically set when using a [`Compiler`](compiler/index.md) with a configuration

  where `Config::reverse` is enabled. If you're building your own NFA

  by hand via a [`Builder`](builder/index.md)

- <span id="nfa-is-always-start-anchored"></span>`fn is_always_start_anchored(&self) -> bool`

  Returns true if and only if all starting states for this NFA correspond

  to the beginning of an anchored search.

  

  Typically, an NFA will have both an anchored and an unanchored starting

  state. Namely, because it tends to be useful to have both and the cost

  of having an unanchored starting state is almost zero (for an NFA).

  However, if all patterns in the NFA are themselves anchored, then even

  the unanchored starting state will correspond to an anchored search

  since the pattern doesn't permit anything else.

  

  # Example

  

  This example shows a few different scenarios where this method's

  return value varies.

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  // The unanchored starting state permits matching this pattern anywhere

  // in a haystack, instead of just at the beginning.

  let nfa = NFA::new("a")?;

  assert!(!nfa.is_always_start_anchored());

  

  // In this case, the pattern is itself anchored, so there is no way

  // to run an unanchored search.

  let nfa = NFA::new("^a")?;

  assert!(nfa.is_always_start_anchored());

  

  // When multiline mode is enabled, '^' can match at the start of a line

  // in addition to the start of a haystack, so an unanchored search is

  // actually possible.

  let nfa = NFA::new("(?m)^a")?;

  assert!(!nfa.is_always_start_anchored());

  

  // Weird cases also work. A pattern is only considered anchored if all

  // matches may only occur at the start of a haystack.

  let nfa = NFA::new("(^a)|a")?;

  assert!(!nfa.is_always_start_anchored());

  

  // When multiple patterns are present, if they are all anchored, then

  // the NFA is always anchored too.

  let nfa = NFA::new_many(&["^a", "^b", "^c"])?;

  assert!(nfa.is_always_start_anchored());

  

  // But if one pattern is unanchored, then the NFA must permit an

  // unanchored search.

  let nfa = NFA::new_many(&["^a", "b", "^c"])?;

  assert!(!nfa.is_always_start_anchored());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-look-matcher"></span>`fn look_matcher(&self) -> &LookMatcher` — [`LookMatcher`](../../util/look/index.md#lookmatcher)

  Returns the look-around matcher associated with this NFA.

  

  A look-around matcher determines how to match look-around assertions.

  In particular, some assertions are configurable. For example, the

  `(?m:^)` and `(?m:$)` assertions can have their line terminator changed

  from the default of `\n` to any other byte.

  

  If the NFA was built using a [`Compiler`](compiler/index.md), then this matcher

  can be set via the `Config::look_matcher` configuration

  knob. Otherwise, if you've built an NFA by hand, it is set via

  `Builder::set_look_matcher`.

  

  # Example

  

  This shows how to change the line terminator for multi-line assertions.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      util::look::LookMatcher,

      Match, Input,

  };

  

  let mut lookm = LookMatcher::new();

  lookm.set_line_terminator(b'\x00');

  

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().look_matcher(lookm))

      .build(r"(?m)^[a-z]+$")?;

  let mut cache = re.create_cache();

  

  // Multi-line assertions now use NUL as a terminator.

  assert_eq!(

      Some(Match::must(0, 1..4)),

      re.find(&mut cache, b"\x00abc\x00"),

  );

  // ... and \n is no longer recognized as a terminator.

  assert_eq!(

      None,

      re.find(&mut cache, b"\nabc\n"),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-look-set-any"></span>`fn look_set_any(&self) -> LookSet` — [`LookSet`](../../util/look/index.md#lookset)

  Returns the union of all look-around assertions used throughout this

  NFA. When the returned set is empty, it implies that the NFA has no

  look-around assertions and thus zero conditional epsilon transitions.

  

  This is useful in some cases enabling optimizations. It is not

  unusual, for example, for optimizations to be of the form, "for any

  regex with zero conditional epsilon transitions, do ..." where "..."

  is some kind of optimization.

  

  This isn't only helpful for optimizations either. Sometimes look-around

  assertions are difficult to support. For example, many of the DFAs in

  this crate don't support Unicode word boundaries or handle them using

  heuristics. Handling that correctly typically requires some kind of

  cheap check of whether the NFA has a Unicode word boundary in the first

  place.

  

  # Example

  

  This example shows how this routine varies based on the regex pattern:

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::look::Look};

  

  // No look-around at all.

  let nfa = NFA::new("a")?;

  assert!(nfa.look_set_any().is_empty());

  

  // When multiple patterns are present, since this returns the union,

  // it will include look-around assertions that only appear in one

  // pattern.

  let nfa = NFA::new_many(&["a", "b", "a^b", "c"])?;

  assert!(nfa.look_set_any().contains(Look::Start));

  

  // Some groups of assertions have various shortcuts. For example:

  let nfa = NFA::new(r"(?-u:\b)")?;

  assert!(nfa.look_set_any().contains_word());

  assert!(!nfa.look_set_any().contains_word_unicode());

  assert!(nfa.look_set_any().contains_word_ascii());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-look-set-prefix-any"></span>`fn look_set_prefix_any(&self) -> LookSet` — [`LookSet`](../../util/look/index.md#lookset)

  Returns the union of all prefix look-around assertions for every

  pattern in this NFA. When the returned set is empty, it implies none of

  the patterns require moving through a conditional epsilon transition

  before inspecting the first byte in the haystack.

  

  This can be useful for determining what kinds of assertions need to be

  satisfied at the beginning of a search. For example, typically DFAs

  in this crate will build a distinct starting state for each possible

  starting configuration that might result in look-around assertions

  being satisfied differently. However, if the set returned here is

  empty, then you know that the start state is invariant because there

  are no conditional epsilon transitions to consider.

  

  # Example

  

  This example shows how this routine varies based on the regex pattern:

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::look::Look};

  

  // No look-around at all.

  let nfa = NFA::new("a")?;

  assert!(nfa.look_set_prefix_any().is_empty());

  

  // When multiple patterns are present, since this returns the union,

  // it will include look-around assertions that only appear in one

  // pattern. But it will only include assertions that are in the prefix

  // of a pattern. For example, this includes '^' but not '$' even though

  // '$' does appear.

  let nfa = NFA::new_many(&["a", "b", "^ab$", "c"])?;

  assert!(nfa.look_set_prefix_any().contains(Look::Start));

  assert!(!nfa.look_set_prefix_any().contains(Look::End));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="nfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the memory usage, in bytes, of this NFA.

  

  This does **not** include the stack size used up by this NFA. To

  compute that, use `std::mem::size_of::<NFA>()`.

  

  # Example

  

  This example shows that large Unicode character classes can use quite

  a bit of memory.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::NFA;

  

  let nfa_unicode = NFA::new(r"\w")?;

  let nfa_ascii = NFA::new(r"(?-u:\w)")?;

  

  assert!(10 * nfa_ascii.memory_usage() < nfa_unicode.memory_usage());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for NFA`

- <span id="nfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NFA`

- <span id="nfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NFA`

- <span id="nfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NFA`

- <span id="nfa-clone"></span>`fn clone(&self) -> NFA` — [`NFA`](nfa/index.md#nfa)

##### `impl CloneToUninit for NFA`

- <span id="nfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NFA`

- <span id="nfa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NFA`

- <span id="nfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NFA`

- <span id="nfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

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

### `Compiler`

```rust
struct Compiler {
    parser: regex_syntax::ParserBuilder,
    config: Config,
    builder: core::cell::RefCell<crate::nfa::thompson::builder::Builder>,
    utf8_state: core::cell::RefCell<Utf8State>,
    trie_state: core::cell::RefCell<crate::nfa::thompson::range_trie::RangeTrie>,
    utf8_suffix: core::cell::RefCell<crate::nfa::thompson::map::Utf8SuffixMap>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:718-736`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L718-L736)*

A builder for compiling an NFA from a regex's high-level intermediate
representation (HIR).

This compiler provides a way to translate a parsed regex pattern into an
NFA state graph. The NFA state graph can either be used directly to execute
a search (e.g., with a Pike VM), or it can be further used to build a DFA.

This compiler provides APIs both for compiling regex patterns directly from
their concrete syntax, or via a [`regex_syntax::hir::Hir`](../../../regex_syntax/hir/index.md).

This compiler has various options that may be configured via
[`thompson::Config`](Config).

Note that a compiler is not the same as a [`thompson::Builder`](Builder).
A `Builder` provides a lower level API that is uncoupled from a regex
pattern's concrete syntax or even its HIR. Instead, it permits stitching
together an NFA by hand. See its docs for examples.

# Example: compilation from concrete syntax

This shows how to compile an NFA from a pattern string while setting a size
limit on how big the NFA is allowed to be (in terms of bytes of heap used).

```rust
use regex_automata::{
    nfa::thompson::{NFA, pikevm::PikeVM},
    Match,
};

let config = NFA::config().nfa_size_limit(Some(1_000));
let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 3..4));
re.captures(&mut cache, "!@#A#@!", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: compilation from HIR

This shows how to hand assemble a regular expression via its HIR, and then
compile an NFA directly from it.

```rust
use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};
use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![
    ClassBytesRange::new(b'0', b'9'),
    ClassBytesRange::new(b'A', b'Z'),
    ClassBytesRange::new(b'_', b'_'),
    ClassBytesRange::new(b'a', b'z'),
])));

let config = NFA::config().nfa_size_limit(Some(1_000));
let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

let re = PikeVM::new_from_nfa(nfa)?;
let mut cache = re.create_cache();
let mut caps = re.create_captures();
let expected = Some(Match::must(0, 3..4));
re.captures(&mut cache, "!@#A#@!", &mut caps);
assert_eq!(expected, caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`parser`**: `regex_syntax::ParserBuilder`

  A regex parser, used when compiling an NFA directly from a pattern
  string.

- **`config`**: `Config`

  The compiler configuration.

- **`builder`**: `core::cell::RefCell<crate::nfa::thompson::builder::Builder>`

  The builder for actually constructing an NFA. This provides a
  convenient abstraction for writing a compiler.

- **`utf8_state`**: `core::cell::RefCell<Utf8State>`

  State used for compiling character classes to UTF-8 byte automata.
  State is not retained between character class compilations. This just
  serves to amortize allocation to the extent possible.

- **`trie_state`**: `core::cell::RefCell<crate::nfa::thompson::range_trie::RangeTrie>`

  State used for arranging character classes in reverse into a trie.

- **`utf8_suffix`**: `core::cell::RefCell<crate::nfa::thompson::map::Utf8SuffixMap>`

  State used for caching common suffixes when compiling reverse UTF-8
  automata (for Unicode character classes).

#### Implementations

- <span id="compiler-new"></span>`fn new() -> Compiler` — [`Compiler`](compiler/index.md#compiler)

  Create a new NFA builder with its default configuration.

- <span id="compiler-build"></span>`fn build(&self, pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Compile the given regular expression pattern into an NFA.

  

  If there was a problem parsing the regex, then that error is returned.

  

  Otherwise, if there was a problem building the NFA, then an error is

  returned. The only error that can occur is if the compiled regex would

  exceed the size limits configured on this builder, or if any part of

  the NFA would exceed the integer representations used. (For example,

  too many states might plausibly occur on a 16-bit target.)

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(0, 3..4));

  re.captures(&mut cache, "!@#A#@!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Compile the given regular expression patterns into a single NFA.

  

  When matches are returned, the pattern ID corresponds to the index of

  the pattern in the slice given.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_many(&[

      r"(?-u)\s",

      r"(?-u)\w",

  ])?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(1, 1..2));

  re.captures(&mut cache, "!A! !A!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-build-from-hir"></span>`fn build_from_hir(&self, expr: &Hir) -> Result<NFA, BuildError>` — [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Compile the given high level intermediate representation of a regular

  expression into an NFA.

  

  If there was a problem building the NFA, then an error is returned. The

  only error that can occur is if the compiled regex would exceed the

  size limits configured on this builder, or if any part of the NFA would

  exceed the integer representations used. (For example, too many states

  might plausibly occur on a 16-bit target.)

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hir = Hir::class(Class::Bytes(ClassBytes::new(vec![

      ClassBytesRange::new(b'0', b'9'),

      ClassBytesRange::new(b'A', b'Z'),

      ClassBytesRange::new(b'_', b'_'),

      ClassBytesRange::new(b'a', b'z'),

  ])));

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_from_hir(&hir)?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(0, 3..4));

  re.captures(&mut cache, "!@#A#@!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-build-many-from-hir"></span>`fn build_many_from_hir<H: Borrow<Hir>>(&self, exprs: &[H]) -> Result<NFA, BuildError>` — [`NFA`](nfa/index.md#nfa), [`BuildError`](error/index.md#builderror)

  Compile the given high level intermediate representations of regular

  expressions into a single NFA.

  

  When matches are returned, the pattern ID corresponds to the index of

  the pattern in the slice given.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::{NFA, pikevm::PikeVM}, Match};

  use regex_syntax::hir::{Hir, Class, ClassBytes, ClassBytesRange};

  

  let hirs = &[

      Hir::class(Class::Bytes(ClassBytes::new(vec![

          ClassBytesRange::new(b'\t', b'\r'),

          ClassBytesRange::new(b' ', b' '),

      ]))),

      Hir::class(Class::Bytes(ClassBytes::new(vec![

          ClassBytesRange::new(b'0', b'9'),

          ClassBytesRange::new(b'A', b'Z'),

          ClassBytesRange::new(b'_', b'_'),

          ClassBytesRange::new(b'a', b'z'),

      ]))),

  ];

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build_many_from_hir(hirs)?;

  

  let re = PikeVM::new_from_nfa(nfa)?;

  let mut cache = re.create_cache();

  let mut caps = re.create_captures();

  let expected = Some(Match::must(1, 1..2));

  re.captures(&mut cache, "!A! !A!", &mut caps);

  assert_eq!(expected, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-configure"></span>`fn configure(&mut self, config: Config) -> &mut Compiler` — [`Config`](compiler/index.md#config), [`Compiler`](compiler/index.md#compiler)

  Apply the given NFA configuration options to this builder.

  

  # Example

  

  ```rust

  use regex_automata::nfa::thompson::NFA;

  

  let config = NFA::config().nfa_size_limit(Some(1_000));

  let nfa = NFA::compiler().configure(config).build(r"(?-u)\w")?;

  assert_eq!(nfa.pattern_len(), 1);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="compiler-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Compiler` — [`Config`](../../util/syntax/index.md#config), [`Compiler`](compiler/index.md#compiler)

  Set the syntax configuration for this builder using

  [`syntax::Config`](crate::util::syntax::Config).

  

  This permits setting things like case insensitivity, Unicode and multi

  line mode.

  

  This syntax configuration only applies when an NFA is built directly

  from a pattern string. If an NFA is built from an HIR, then all syntax

  settings are ignored.

  

  # Example

  

  ```rust

  use regex_automata::{nfa::thompson::NFA, util::syntax};

  

  let syntax_config = syntax::Config::new().unicode(false);

  let nfa = NFA::compiler().syntax(syntax_config).build(r"\w")?;

  // If Unicode were enabled, the number of states would be much bigger.

  assert!(nfa.states().len() < 15);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

#### Trait Implementations

##### `impl Any for Compiler`

- <span id="compiler-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Compiler`

- <span id="compiler-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Compiler`

- <span id="compiler-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Compiler`

- <span id="compiler-clone"></span>`fn clone(&self) -> Compiler` — [`Compiler`](compiler/index.md#compiler)

##### `impl CloneToUninit for Compiler`

- <span id="compiler-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Compiler`

- <span id="compiler-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Compiler`

- <span id="compiler-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Compiler`

- <span id="compiler-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Compiler`

- <span id="compiler-toowned-type-owned"></span>`type Owned = T`

- <span id="compiler-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="compiler-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Compiler`

- <span id="compiler-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="compiler-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Compiler`

- <span id="compiler-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="compiler-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Config`

```rust
struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<crate::util::look::LookMatcher>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:28-37`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L28-L37)*

The configuration used for a Thompson NFA compiler.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](compiler/index.md#config)

  Return a new default Thompson NFA compiler configuration.

- <span id="config-utf8"></span>`fn utf8(self, yes: bool) -> Config` — [`Config`](compiler/index.md#config)

  Whether to enable UTF-8 mode during search or not.

  

  A regex engine is said to be in UTF-8 mode when it guarantees that

  all matches returned by it have spans consisting of only valid UTF-8.

  That is, it is impossible for a match span to be returned that

  contains any invalid UTF-8.

  

  UTF-8 mode generally consists of two things:

  

  1. Whether the NFA's states are constructed such that all paths to a

  match state that consume at least one byte always correspond to valid

  UTF-8.

  2. Whether all paths to a match state that do _not_ consume any bytes

  should always correspond to valid UTF-8 boundaries.

  

  (1) is a guarantee made by whoever constructs the NFA.

  If you're parsing a regex from its concrete syntax, then

  [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) can make

  this guarantee for you. It does it by returning an error if the regex

  pattern could every report a non-empty match span that contains invalid

  UTF-8. So long as `syntax::Config::utf8` mode is enabled and your regex

  successfully parses, then you're guaranteed that the corresponding NFA

  will only ever report non-empty match spans containing valid UTF-8.

  

  (2) is a trickier guarantee because it cannot be enforced by the NFA

  state graph itself. Consider, for example, the regex `a*`. It matches

  the empty strings in `☃` at positions `0`, `1`, `2` and `3`, where

  positions `1` and `2` occur within the UTF-8 encoding of a codepoint,

  and thus correspond to invalid UTF-8 boundaries. Therefore, this

  guarantee must be made at a higher level than the NFA state graph

  itself. This crate deals with this case in each regex engine. Namely,

  when a zero-width match that splits a codepoint is found and UTF-8

  mode enabled, then it is ignored and the engine moves on looking for

  the next match.

  

  Thus, UTF-8 mode is both a promise that the NFA built only reports

  non-empty matches that are valid UTF-8, and an *instruction* to regex

  engines that empty matches that split codepoints should be banned.

  

  Because UTF-8 mode is fundamentally about avoiding invalid UTF-8 spans,

  it only makes sense to enable this option when you *know* your haystack

  is valid UTF-8. (For example, a `&str`.) Enabling UTF-8 mode and

  searching a haystack that contains invalid UTF-8 leads to **unspecified

  behavior**.

  

  Therefore, it may make sense to enable `syntax::Config::utf8` while

  simultaneously *disabling* this option. That would ensure all non-empty

  match spans are valid UTF-8, but that empty match spans may still split

  a codepoint or match at other places that aren't valid UTF-8.

  

  In general, this mode is only relevant if your regex can match the

  empty string. Most regexes don't.

  

  This is enabled by default.

  

  # Example

  

  This example shows how UTF-8 mode can impact the match spans that may

  be reported in certain cases.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      Match, Input,

  };

  

  let re = PikeVM::new("")?;

  let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

  

  // UTF-8 mode is enabled by default.

  let mut input = Input::new("☃");

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 0..0)), caps.get_match());

  

  // Even though an empty regex matches at 1..1, our next match is

  // 3..3 because 1..1 and 2..2 split the snowman codepoint (which is

  // three bytes long).

  input.set_start(1);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 3..3)), caps.get_match());

  

  // But if we disable UTF-8, then we'll get matches at 1..1 and 2..2:

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().utf8(false))

      .build("")?;

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 1..1)), caps.get_match());

  

  input.set_start(2);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 2..2)), caps.get_match());

  

  input.set_start(3);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(Some(Match::must(0, 3..3)), caps.get_match());

  

  input.set_start(4);

  re.search(&mut cache, &input, &mut caps);

  assert_eq!(None, caps.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-reverse"></span>`fn reverse(self, yes: bool) -> Config` — [`Config`](compiler/index.md#config)

  Reverse the NFA.

  

  A NFA reversal is performed by reversing all of the concatenated

  sub-expressions in the original pattern, recursively. (Look around

  operators are also inverted.) The resulting NFA can be used to match

  the pattern starting from the end of a string instead of the beginning

  of a string.

  

  Reversing the NFA is useful for building a reverse DFA, which is most

  useful for finding the start of a match after its ending position has

  been found. NFA execution engines typically do not work on reverse

  NFAs. For example, currently, the Pike VM reports the starting location

  of matches without a reverse NFA.

  

  Currently, enabling this setting requires disabling the

  [`captures`](Config::captures) setting. If both are enabled, then the

  compiler will return an error. It is expected that this limitation will

  be lifted in the future.

  

  This is disabled by default.

  

  # Example

  

  This example shows how to build a DFA from a reverse NFA, and then use

  the DFA to search backwards.

  

  ```rust

  use regex_automata::{

      dfa::{self, Automaton},

      nfa::thompson::{NFA, WhichCaptures},

      HalfMatch, Input,

  };

  

  let dfa = dfa::dense::Builder::new()

      .thompson(NFA::config()

          .which_captures(WhichCaptures::None)

          .reverse(true)

      )

      .build("baz[0-9]+")?;

  let expected = Some(HalfMatch::must(0, 3));

  assert_eq!(

      expected,

      dfa.try_search_rev(&Input::new("foobaz12345bar"))?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-nfa-size-limit"></span>`fn nfa_size_limit(self, bytes: Option<usize>) -> Config` — [`Config`](compiler/index.md#config)

  Sets an approximate size limit on the total heap used by the NFA being

  compiled.

  

  This permits imposing constraints on the size of a compiled NFA. This

  may be useful in contexts where the regex pattern is untrusted and one

  wants to avoid using too much memory.

  

  This size limit does not apply to auxiliary heap used during

  compilation that is not part of the built NFA.

  

  Note that this size limit is applied during compilation in order for

  the limit to prevent too much heap from being used. However, the

  implementation may use an intermediate NFA representation that is

  otherwise slightly bigger than the final public form. Since the size

  limit may be applied to an intermediate representation, there is not

  necessarily a precise correspondence between the configured size limit

  and the heap usage of the final NFA.

  

  There is no size limit by default.

  

  # Example

  

  This example demonstrates how Unicode mode can greatly increase the

  size of the NFA.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::NFA;

  

  // 300KB isn't enough!

  NFA::compiler()

      .configure(NFA::config().nfa_size_limit(Some(300_000)))

      .build(r"\w{20}")

      .unwrap_err();

  

  // ... but 500KB probably is.

  let nfa = NFA::compiler()

      .configure(NFA::config().nfa_size_limit(Some(500_000)))

      .build(r"\w{20}")?;

  

  assert_eq!(nfa.pattern_len(), 1);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-shrink"></span>`fn shrink(self, yes: bool) -> Config` — [`Config`](compiler/index.md#config)

  Apply best effort heuristics to shrink the NFA at the expense of more

  time/memory.

  

  Generally speaking, if one is using an NFA to compile a DFA, then the

  extra time used to shrink the NFA will be more than made up for during

  DFA construction (potentially by a lot). In other words, enabling this

  can substantially decrease the overall amount of time it takes to build

  a DFA.

  

  A reason to keep this disabled is if you want to compile an NFA and

  start using it as quickly as possible without needing to build a DFA,

  and you don't mind using a bit of extra memory for the NFA. e.g., for

  an NFA simulation or for a lazy DFA.

  

  NFA shrinking is currently most useful when compiling a reverse

  NFA with large Unicode character classes. In particular, it trades

  additional CPU time during NFA compilation in favor of generating fewer

  NFA states.

  

  This is disabled by default because it can increase compile times

  quite a bit if you aren't building a full DFA.

  

  # Example

  

  This example shows that NFA shrinking can lead to substantial space

  savings in some cases. Notice that, as noted above, we build a reverse

  DFA and use a pattern with a large Unicode character class.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::nfa::thompson::{NFA, WhichCaptures};

  

  // Currently we have to disable captures when enabling reverse NFA.

  let config = NFA::config()

      .which_captures(WhichCaptures::None)

      .reverse(true);

  let not_shrunk = NFA::compiler()

      .configure(config.clone().shrink(false))

      .build(r"\w")?;

  let shrunk = NFA::compiler()

      .configure(config.clone().shrink(true))

      .build(r"\w")?;

  

  // While a specific shrink factor is not guaranteed, the savings can be

  // considerable in some cases.

  assert!(shrunk.states().len() * 2 < not_shrunk.states().len());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-captures"></span>`fn captures(self, yes: bool) -> Config` — [`Config`](compiler/index.md#config)

  Whether to include 'Capture' states in the NFA.

  

  Currently, enabling this setting requires disabling the

  [`reverse`](Config::reverse) setting. If both are enabled, then the

  compiler will return an error. It is expected that this limitation will

  be lifted in the future.

  

  This is enabled by default.

  

  # Example

  

  This example demonstrates that some regex engines, like the Pike VM,

  require capturing states to be present in the NFA to report match

  offsets.

  

  (Note that since this method is deprecated, the example below uses

  `Config::which_captures` to disable capture states.)

  

  ```rust

  use regex_automata::nfa::thompson::{

      pikevm::PikeVM,

      NFA,

      WhichCaptures,

  };

  

  let re = PikeVM::builder()

      .thompson(NFA::config().which_captures(WhichCaptures::None))

      .build(r"[a-z]+")?;

  let mut cache = re.create_cache();

  

  assert!(re.is_match(&mut cache, "abc"));

  assert_eq!(None, re.find(&mut cache, "abc"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-which-captures"></span>`fn which_captures(self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](compiler/index.md#whichcaptures), [`Config`](compiler/index.md#config)

  Configures what kinds of capture groups are compiled into

  [`State::Capture`](crate::nfa::thompson::State::Capture) states in a

  Thompson NFA.

  

  Currently, using any option except for [`WhichCaptures::None`](../../index.md) requires

  disabling the [`reverse`](Config::reverse) setting. If both are

  enabled, then the compiler will return an error. It is expected that

  this limitation will be lifted in the future.

  

  This is set to [`WhichCaptures::All`](../../index.md) by default. Callers may wish to

  use [`WhichCaptures::Implicit`](../../index.md) in cases where one wants avoid the

  overhead of capture states for explicit groups. Usually this occurs

  when one wants to use the `PikeVM` only for determining the overall

  match. Otherwise, the `PikeVM` could use much more memory than is

  necessary.

  

  # Example

  

  This example demonstrates that some regex engines, like the Pike VM,

  require capturing states to be present in the NFA to report match

  offsets.

  

  ```rust

  use regex_automata::nfa::thompson::{

      pikevm::PikeVM,

      NFA,

      WhichCaptures,

  };

  

  let re = PikeVM::builder()

      .thompson(NFA::config().which_captures(WhichCaptures::None))

      .build(r"[a-z]+")?;

  let mut cache = re.create_cache();

  

  assert!(re.is_match(&mut cache, "abc"));

  assert_eq!(None, re.find(&mut cache, "abc"));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  The same applies to the bounded backtracker:

  

  ```rust

  use regex_automata::nfa::thompson::{

      backtrack::BoundedBacktracker,

      NFA,

      WhichCaptures,

  };

  

  let re = BoundedBacktracker::builder()

      .thompson(NFA::config().which_captures(WhichCaptures::None))

      .build(r"[a-z]+")?;

  let mut cache = re.create_cache();

  

  assert!(re.try_is_match(&mut cache, "abc")?);

  assert_eq!(None, re.try_find(&mut cache, "abc")?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-look-matcher"></span>`fn look_matcher(self, m: LookMatcher) -> Config` — [`LookMatcher`](../../util/look/index.md#lookmatcher), [`Config`](compiler/index.md#config)

  Sets the look-around matcher that should be used with this NFA.

  

  A look-around matcher determines how to match look-around assertions.

  In particular, some assertions are configurable. For example, the

  `(?m:^)` and `(?m:$)` assertions can have their line terminator changed

  from the default of `\n` to any other byte.

  

  # Example

  

  This shows how to change the line terminator for multi-line assertions.

  

  ```rust

  use regex_automata::{

      nfa::thompson::{self, pikevm::PikeVM},

      util::look::LookMatcher,

      Match, Input,

  };

  

  let mut lookm = LookMatcher::new();

  lookm.set_line_terminator(b'\x00');

  

  let re = PikeVM::builder()

      .thompson(thompson::Config::new().look_matcher(lookm))

      .build(r"(?m)^[a-z]+$")?;

  let mut cache = re.create_cache();

  

  // Multi-line assertions now use NUL as a terminator.

  assert_eq!(

      Some(Match::must(0, 1..4)),

      re.find(&mut cache, b"\x00abc\x00"),

  );

  // ... and \n is no longer recognized as a terminator.

  assert_eq!(

      None,

      re.find(&mut cache, b"\nabc\n"),

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-get-utf8"></span>`fn get_utf8(&self) -> bool`

  Returns whether this configuration has enabled UTF-8 mode.

- <span id="config-get-reverse"></span>`fn get_reverse(&self) -> bool`

  Returns whether this configuration has enabled reverse NFA compilation.

- <span id="config-get-nfa-size-limit"></span>`fn get_nfa_size_limit(&self) -> Option<usize>`

  Return the configured NFA size limit, if it exists, in the number of

  bytes of heap used.

- <span id="config-get-shrink"></span>`fn get_shrink(&self) -> bool`

  Return whether NFA shrinking is enabled.

- <span id="config-get-captures"></span>`fn get_captures(&self) -> bool`

  Return whether NFA compilation is configured to produce capture states.

- <span id="config-get-which-captures"></span>`fn get_which_captures(&self) -> WhichCaptures` — [`WhichCaptures`](compiler/index.md#whichcaptures)

  Return what kinds of capture states will be compiled into an NFA.

- <span id="config-get-look-matcher"></span>`fn get_look_matcher(&self) -> LookMatcher` — [`LookMatcher`](../../util/look/index.md#lookmatcher)

  Return the look-around matcher for this NFA.

- <span id="config-get-unanchored-prefix"></span>`fn get_unanchored_prefix(&self) -> bool`

  Return whether NFA compilation is configured to include an unanchored

  prefix.

  

  This is always false when not in test mode.

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](compiler/index.md#config)

  Overwrite the default configuration such that the options in `o` are

  always used. If an option in `o` is not set, then the corresponding

  option in `self` is used. If it's not set in `self` either, then it

  remains not set.

#### Trait Implementations

##### `impl Any for Config`

- <span id="config-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Config`

- <span id="config-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Config`

- <span id="config-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](compiler/index.md#config)

##### `impl CloneToUninit for Config`

- <span id="config-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Config`

- <span id="config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](compiler/index.md#config)

##### `impl<T> From for Config`

- <span id="config-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Config`

- <span id="config-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Config`

- <span id="config-toowned-type-owned"></span>`type Owned = T`

- <span id="config-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="config-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Config`

- <span id="config-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="config-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Config`

- <span id="config-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="config-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `State`

```rust
enum State {
    ByteRange {
        trans: Transition,
    },
    Sparse(SparseTransitions),
    Dense(DenseTransitions),
    Look {
        look: crate::util::look::Look,
        next: crate::util::primitives::StateID,
    },
    Union {
        alternates: alloc::boxed::Box<[crate::util::primitives::StateID]>,
    },
    BinaryUnion {
        alt1: crate::util::primitives::StateID,
        alt2: crate::util::primitives::StateID,
    },
    Capture {
        next: crate::util::primitives::StateID,
        pattern_id: crate::util::primitives::PatternID,
        group_index: crate::util::primitives::SmallIndex,
        slot: crate::util::primitives::SmallIndex,
    },
    Fail,
    Match {
        pattern_id: crate::util::primitives::PatternID,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/nfa.rs:1514-1621`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/nfa.rs#L1514-L1621)*

A state in an NFA.

In theory, it can help to conceptualize an `NFA` as a graph consisting of
`State`s. Each `State` contains its complete set of outgoing transitions.

In practice, it can help to conceptualize an `NFA` as a sequence of
instructions for a virtual machine. Each `State` says what to do and where
to go next.

Strictly speaking, the practical interpretation is the most correct one,
because of the [`Capture`](State::Capture) state. Namely, a `Capture`
state always forwards execution to another state unconditionally. Its only
purpose is to cause a side effect: the recording of the current input
position at a particular location in memory. In this sense, an `NFA`
has more power than a theoretical non-deterministic finite automaton.

For most uses of this crate, it is likely that one may never even need to
be aware of this type at all. The main use cases for looking at `State`s
directly are if you need to write your own search implementation or if you
need to do some kind of analysis on the NFA.

#### Variants

- **`ByteRange`**

  A state with a single transition that can only be taken if the current
  input symbol is in a particular range of bytes.

- **`Sparse`**

  A state with possibly many transitions represented in a sparse fashion.
  Transitions are non-overlapping and ordered lexicographically by input
  range.
  
  In practice, this is used for encoding UTF-8 automata. Its presence is
  primarily an optimization that avoids many additional unconditional
  epsilon transitions (via [`Union`](State::Union) states), and thus
  decreases the overhead of traversing the NFA. This can improve both
  matching time and DFA construction time.

- **`Dense`**

  A dense representation of a state with multiple transitions.

- **`Look`**

  A conditional epsilon transition satisfied via some sort of
  look-around. Look-around is limited to anchor and word boundary
  assertions.
  
  Look-around states are meant to be evaluated while performing epsilon
  closure (computing the set of states reachable from a particular state
  via only epsilon transitions). If the current position in the haystack
  satisfies the look-around assertion, then you're permitted to follow
  that epsilon transition.

- **`Union`**

  An alternation such that there exists an epsilon transition to all
  states in `alternates`, where matches found via earlier transitions
  are preferred over later transitions.

- **`BinaryUnion`**

  An alternation such that there exists precisely two unconditional
  epsilon transitions, where matches found via `alt1` are preferred over
  matches found via `alt2`.
  
  This state exists as a common special case of Union where there are
  only two alternates. In this case, we don't need any allocations to
  represent the state. This saves a bit of memory and also saves an
  additional memory access when traversing the NFA.

- **`Capture`**

  An empty state that records a capture location.
  
  From the perspective of finite automata, this is precisely equivalent
  to an unconditional epsilon transition, but serves the purpose of
  instructing NFA simulations to record additional state when the finite
  state machine passes through this epsilon transition.
  
  `slot` in this context refers to the specific capture group slot
  offset that is being recorded. Each capturing group has two slots
  corresponding to the start and end of the matching portion of that
  group.
  
  The pattern ID and capture group index are also included in this state
  in case they are useful. But mostly, all you'll need is `next` and
  `slot`.

- **`Fail`**

  A state that cannot be transitioned out of. This is useful for cases
  where you want to prevent matching from occurring. For example, if your
  regex parser permits empty character classes, then one could choose
  a `Fail` state to represent them. (An empty character class can be
  thought of as an empty set. Since nothing is in an empty set, they can
  never match anything.)

- **`Match`**

  A match state. There is at least one such occurrence of this state for
  each regex that can match that is in this NFA.

#### Implementations

- <span id="state-is-epsilon"></span>`fn is_epsilon(&self) -> bool`

  Returns true if and only if this state contains one or more epsilon

  transitions.

  

  In practice, a state has no outgoing transitions (like `Match`), has

  only non-epsilon transitions (like `ByteRange`) or has only epsilon

  transitions (like `Union`).

  

  # Example

  

  ```rust

  use regex_automata::{

      nfa::thompson::{State, Transition},

      util::primitives::{PatternID, StateID, SmallIndex},

  };

  

  // Capture states are epsilon transitions.

  let state = State::Capture {

      next: StateID::ZERO,

      pattern_id: PatternID::ZERO,

      group_index: SmallIndex::ZERO,

      slot: SmallIndex::ZERO,

  };

  assert!(state.is_epsilon());

  

  // ByteRange states are not.

  let state = State::ByteRange {

      trans: Transition { start: b'a', end: b'z', next: StateID::ZERO },

  };

  assert!(!state.is_epsilon());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="state-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage of this NFA state in bytes.

- <span id="state-remap"></span>`fn remap(&mut self, remap: &[StateID])` — [`StateID`](../../util/primitives/index.md#stateid)

  Remap the transitions in this state using the given map. Namely, the

  given map should be indexed according to the transitions currently

  in this state.

  

  This is used during the final phase of the NFA compiler, which turns

  its intermediate NFA into the final NFA.

#### Trait Implementations

##### `impl Any for State`

- <span id="state-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for State`

- <span id="state-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for State`

- <span id="state-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](nfa/index.md#state)

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

- <span id="state-partialeq-eq"></span>`fn eq(&self, other: &State) -> bool` — [`State`](nfa/index.md#state)

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

### `WhichCaptures`

```rust
enum WhichCaptures {
    All,
    Implicit,
    None,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/compiler.rs:547-589`](../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/compiler.rs#L547-L589)*

A configuration indicating which kinds of
[`State::Capture`](crate::nfa::thompson::State::Capture) states to include.

This configuration can be used with `Config::which_captures` to control
which capture states are compiled into a Thompson NFA.

The default configuration is [`WhichCaptures::All`](../../index.md).

#### Variants

- **`All`**

  All capture states, including those corresponding to both implicit and
  explicit capture groups, are included in the Thompson NFA.

- **`Implicit`**

  Only capture states corresponding to implicit capture groups are
  included. Implicit capture groups appear in every pattern implicitly
  and correspond to the overall match of a pattern.
  
  This is useful when one only cares about the overall match of a
  pattern. By excluding capture states from explicit capture groups,
  one might be able to reduce the memory usage of a multi-pattern regex
  substantially if it was otherwise written to have many explicit capture
  groups.

- **`None`**

  No capture states are compiled into the Thompson NFA.
  
  This is useful when capture states are either not needed (for example,
  if one is only trying to build a DFA) or if they aren't supported (for
  example, a reverse NFA).
  
  # Warning
  
  Callers must be exceedingly careful when using this
  option. In particular, not all regex engines support
  reporting match spans when using this option (for example,
  [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) or
  [`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)).
  
  Perhaps more confusingly, using this option with such an
  engine means that an `is_match` routine could report `true`
  when `find` reports `None`. This is generally not something
  that _should_ happen, but the low level control provided by
  this crate makes it possible.
  
  Similarly, any regex engines (like [`meta::Regex`](crate::meta::Regex))
  should always return `None` from `find` routines when this option is
  used, even if _some_ of its internal engines could find the match
  boundaries. This is because inputs from user data could influence
  engine selection, and thus influence whether a match is found or not.
  Indeed, `meta::Regex::find` will always return `None` when configured
  with this option.

#### Implementations

- <span id="whichcaptures-is-none"></span>`fn is_none(&self) -> bool`

  Returns true if this configuration indicates that no capture states

  should be produced in an NFA.

- <span id="whichcaptures-is-any"></span>`fn is_any(&self) -> bool`

  Returns true if this configuration indicates that some capture states

  should be added to an NFA. Note that this might only include capture

  states for implicit capture groups.

#### Trait Implementations

##### `impl Any for WhichCaptures`

- <span id="whichcaptures-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for WhichCaptures`

- <span id="whichcaptures-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for WhichCaptures`

- <span id="whichcaptures-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for WhichCaptures`

- <span id="whichcaptures-clone"></span>`fn clone(&self) -> WhichCaptures` — [`WhichCaptures`](compiler/index.md#whichcaptures)

##### `impl CloneToUninit for WhichCaptures`

- <span id="whichcaptures-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for WhichCaptures`

##### `impl Debug for WhichCaptures`

- <span id="whichcaptures-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for WhichCaptures`

- <span id="whichcaptures-default"></span>`fn default() -> WhichCaptures` — [`WhichCaptures`](compiler/index.md#whichcaptures)

##### `impl<T> From for WhichCaptures`

- <span id="whichcaptures-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for WhichCaptures`

- <span id="whichcaptures-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for WhichCaptures`

- <span id="whichcaptures-toowned-type-owned"></span>`type Owned = T`

- <span id="whichcaptures-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="whichcaptures-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for WhichCaptures`

- <span id="whichcaptures-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="whichcaptures-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for WhichCaptures`

- <span id="whichcaptures-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="whichcaptures-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

