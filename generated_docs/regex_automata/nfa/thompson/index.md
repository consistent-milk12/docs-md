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
* An `Hir` is compiled into a [`NFA`](#nfa).
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

## Modules

- [`backtrack`](backtrack/index.md) - An NFA backed bounded backtracker for executing regex searches with capturing
- [`builder`](builder/index.md) - 
- [`compiler`](compiler/index.md) - 
- [`error`](error/index.md) - 
- [`literal_trie`](literal_trie/index.md) - 
- [`map`](map/index.md) - 
- [`nfa`](nfa/index.md) - 
- [`pikevm`](pikevm/index.md) - An NFA backed Pike VM for executing regex searches with capturing groups.
- [`range_trie`](range_trie/index.md) - 

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
representation into an [`NFA`](#nfa).

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

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn clear(self: &mut Self)`

- `fn build(self: &Self, start_anchored: StateID, start_unanchored: StateID) -> Result<NFA, BuildError>` — [`StateID`](../../util/primitives/index.md), [`NFA`](#nfa), [`BuildError`](#builderror)

- `fn start_pattern(self: &mut Self) -> Result<PatternID, BuildError>` — [`PatternID`](../../index.md), [`BuildError`](#builderror)

- `fn finish_pattern(self: &mut Self, start_id: StateID) -> Result<PatternID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../index.md), [`BuildError`](#builderror)

- `fn current_pattern_id(self: &Self) -> PatternID` — [`PatternID`](../../index.md)

- `fn pattern_len(self: &Self) -> usize`

- `fn add_empty(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_union(self: &mut Self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_union_reverse(self: &mut Self, alternates: Vec<StateID>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_range(self: &mut Self, trans: Transition) -> Result<StateID, BuildError>` — [`Transition`](#transition), [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_sparse(self: &mut Self, transitions: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](#transition), [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_look(self: &mut Self, next: StateID, look: Look) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`Look`](../../util/look/index.md), [`BuildError`](#builderror)

- `fn add_capture_start(self: &mut Self, next: StateID, group_index: u32, name: Option<Arc<str>>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_capture_end(self: &mut Self, next: StateID, group_index: u32) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_fail(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_match(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add(self: &mut Self, state: State) -> Result<StateID, BuildError>` — [`State`](builder/index.md), [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn patch(self: &mut Self, from: StateID, to: StateID) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn set_utf8(self: &mut Self, yes: bool)`

- `fn get_utf8(self: &Self) -> bool`

- `fn set_reverse(self: &mut Self, yes: bool)`

- `fn get_reverse(self: &Self) -> bool`

- `fn set_look_matcher(self: &mut Self, m: LookMatcher)` — [`LookMatcher`](../../util/look/index.md)

- `fn get_look_matcher(self: &Self) -> &LookMatcher` — [`LookMatcher`](../../util/look/index.md)

- `fn set_size_limit(self: &mut Self, limit: Option<usize>) -> Result<(), BuildError>` — [`BuildError`](#builderror)

- `fn get_size_limit(self: &Self) -> Option<usize>`

- `fn memory_usage(self: &Self) -> usize`

- `fn check_size_limit(self: &Self) -> Result<(), BuildError>` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](#builder)

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

An error that can occurred during the construction of a thompson NFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`regex_syntax::Error`](../../../regex_syntax/unicode/index.md) type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building an NFA directly from a pattern string.

Otherwise, errors typically occur when a limit has been breached. For
example, if the total heap usage of the compiled NFA exceeds the limit
set by [`Config::nfa_size_limit`](crate::nfa::thompson::Config), then
building the NFA will fail.

#### Implementations

- `fn size_limit(self: &Self) -> Option<usize>`

- `fn kind(self: &Self) -> &BuildErrorKind` — [`BuildErrorKind`](error/index.md)

- `fn syntax(err: regex_syntax::Error) -> BuildError` — [`BuildError`](#builderror)

- `fn captures(err: captures::GroupInfoError) -> BuildError` — [`GroupInfoError`](../../util/captures/index.md), [`BuildError`](#builderror)

- `fn word(err: look::UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../util/look/index.md), [`BuildError`](#builderror)

- `fn too_many_patterns(given: usize) -> BuildError` — [`BuildError`](#builderror)

- `fn too_many_states(given: usize) -> BuildError` — [`BuildError`](#builderror)

- `fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

- `fn invalid_capture_index(index: u32) -> BuildError` — [`BuildError`](#builderror)

- `fn unsupported_captures() -> BuildError` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- `fn clone(self: &Self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for BuildError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for BuildError`

- `fn to_string(self: &Self) -> String`

### `DenseTransitions`

```rust
struct DenseTransitions {
    pub transitions: alloc::boxed::Box<[crate::util::primitives::StateID]>,
}
```

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

- `fn matches(self: &Self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md)

- `fn matches_unit(self: &Self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../util/alphabet/index.md), [`StateID`](../../util/primitives/index.md)

- `fn matches_byte(self: &Self, byte: u8) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md)

- `fn iter(self: &Self) -> impl Iterator<Item = Transition> + '_` — [`Transition`](#transition)

#### Trait Implementations

##### `impl Clone for DenseTransitions`

- `fn clone(self: &Self) -> DenseTransitions` — [`DenseTransitions`](#densetransitions)

##### `impl Debug for DenseTransitions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for DenseTransitions`

##### `impl PartialEq for DenseTransitions`

- `fn eq(self: &Self, other: &DenseTransitions) -> bool` — [`DenseTransitions`](#densetransitions)

##### `impl StructuralPartialEq for DenseTransitions`

### `PatternIter<'a>`

```rust
struct PatternIter<'a> {
    it: crate::util::primitives::PatternIDIter,
    _marker: core::marker::PhantomData<&'a ()>,
}
```

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

##### `impl<'a> Debug for PatternIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for PatternIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for PatternIter<'a>`

- `type Item = PatternID`

- `fn next(self: &mut Self) -> Option<PatternID>` — [`PatternID`](../../index.md)

### `SparseTransitions`

```rust
struct SparseTransitions {
    pub transitions: alloc::boxed::Box<[Transition]>,
}
```

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

- `fn matches(self: &Self, haystack: &[u8], at: usize) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md)

- `fn matches_unit(self: &Self, unit: alphabet::Unit) -> Option<StateID>` — [`Unit`](../../util/alphabet/index.md), [`StateID`](../../util/primitives/index.md)

- `fn matches_byte(self: &Self, byte: u8) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for SparseTransitions`

- `fn clone(self: &Self) -> SparseTransitions` — [`SparseTransitions`](#sparsetransitions)

##### `impl Debug for SparseTransitions`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for SparseTransitions`

##### `impl PartialEq for SparseTransitions`

- `fn eq(self: &Self, other: &SparseTransitions) -> bool` — [`SparseTransitions`](#sparsetransitions)

##### `impl StructuralPartialEq for SparseTransitions`

### `Transition`

```rust
struct Transition {
    pub start: u8,
    pub end: u8,
    pub next: crate::util::primitives::StateID,
}
```

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

- `fn matches(self: &Self, haystack: &[u8], at: usize) -> bool`

- `fn matches_unit(self: &Self, unit: alphabet::Unit) -> bool` — [`Unit`](../../util/alphabet/index.md)

- `fn matches_byte(self: &Self, byte: u8) -> bool`

#### Trait Implementations

##### `impl Clone for Transition`

- `fn clone(self: &Self) -> Transition` — [`Transition`](#transition)

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Transition`

##### `impl Hash for Transition`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Transition`

- `fn eq(self: &Self, other: &Transition) -> bool` — [`Transition`](#transition)

##### `impl StructuralPartialEq for Transition`

### `NFA`

```rust
struct NFA(alloc::sync::Arc<Inner>);
```

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

- `fn new(pattern: &str) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](#builderror)

- `fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](#builderror)

- `fn always_match() -> NFA` — [`NFA`](#nfa)

- `fn never_match() -> NFA` — [`NFA`](#nfa)

- `fn config() -> Config` — [`Config`](#config)

- `fn compiler() -> Compiler` — [`Compiler`](#compiler)

- `fn patterns(self: &Self) -> PatternIter<'_>` — [`PatternIter`](#patterniter)

- `fn pattern_len(self: &Self) -> usize`

- `fn start_anchored(self: &Self) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `fn start_unanchored(self: &Self) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `fn start_pattern(self: &Self, pid: PatternID) -> Option<StateID>` — [`PatternID`](../../index.md), [`StateID`](../../util/primitives/index.md)

- `fn byte_class_set(self: &Self) -> &ByteClassSet` — [`ByteClassSet`](../../util/alphabet/index.md)

- `fn byte_classes(self: &Self) -> &ByteClasses` — [`ByteClasses`](../../util/alphabet/index.md)

- `fn state(self: &Self, id: StateID) -> &State` — [`StateID`](../../util/primitives/index.md), [`State`](#state)

- `fn states(self: &Self) -> &[State]` — [`State`](#state)

- `fn group_info(self: &Self) -> &GroupInfo` — [`GroupInfo`](../../util/captures/index.md)

- `fn has_capture(self: &Self) -> bool`

- `fn has_empty(self: &Self) -> bool`

- `fn is_utf8(self: &Self) -> bool`

- `fn is_reverse(self: &Self) -> bool`

- `fn is_always_start_anchored(self: &Self) -> bool`

- `fn look_matcher(self: &Self) -> &LookMatcher` — [`LookMatcher`](../../util/look/index.md)

- `fn look_set_any(self: &Self) -> LookSet` — [`LookSet`](../../util/look/index.md)

- `fn look_set_prefix_any(self: &Self) -> LookSet` — [`LookSet`](../../util/look/index.md)

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for NFA`

- `fn clone(self: &Self) -> NFA` — [`NFA`](#nfa)

##### `impl Debug for NFA`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

- `fn compile<H: Borrow<Hir>>(self: &Self, exprs: &[H]) -> Result<NFA, BuildError>` — [`NFA`](#nfa), [`BuildError`](#builderror)

- `fn c(self: &Self, expr: &Hir) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_concat<I>(self: &Self, it: I) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_alt_slice(self: &Self, exprs: &[Hir]) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_alt_iter<I>(self: &Self, it: I) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_cap(self: &Self, index: u32, name: Option<&str>, expr: &Hir) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_repetition(self: &Self, rep: &hir::Repetition) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_bounded(self: &Self, expr: &Hir, greedy: bool, min: u32, max: u32) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_at_least(self: &Self, expr: &Hir, greedy: bool, n: u32) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_zero_or_one(self: &Self, expr: &Hir, greedy: bool) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_exactly(self: &Self, expr: &Hir, n: u32) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_byte_class(self: &Self, cls: &hir::ClassBytes) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_unicode_class(self: &Self, cls: &hir::ClassUnicode) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_unicode_class_reverse_with_suffix(self: &Self, cls: &hir::ClassUnicode) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_look(self: &Self, anchor: &hir::Look) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_literal(self: &Self, bytes: &[u8]) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_range(self: &Self, start: u8, end: u8) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_empty(self: &Self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn c_fail(self: &Self) -> Result<ThompsonRef, BuildError>` — [`ThompsonRef`](compiler/index.md), [`BuildError`](#builderror)

- `fn patch(self: &Self, from: StateID, to: StateID) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn start_pattern(self: &Self) -> Result<PatternID, BuildError>` — [`PatternID`](../../index.md), [`BuildError`](#builderror)

- `fn finish_pattern(self: &Self, start_id: StateID) -> Result<PatternID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../index.md), [`BuildError`](#builderror)

- `fn add_empty(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_range(self: &Self, start: u8, end: u8) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_sparse(self: &Self, ranges: Vec<Transition>) -> Result<StateID, BuildError>` — [`Transition`](#transition), [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_look(self: &Self, look: Look) -> Result<StateID, BuildError>` — [`Look`](../../util/look/index.md), [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_union(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_union_reverse(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_capture_start(self: &Self, capture_index: u32, name: Option<&str>) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_capture_end(self: &Self, capture_index: u32) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_fail(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn add_match(self: &Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn is_reverse(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for Compiler`

- `fn clone(self: &Self) -> Compiler` — [`Compiler`](#compiler)

##### `impl Debug for Compiler`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

The configuration used for a Thompson NFA compiler.

#### Implementations

- `fn new() -> Config` — [`Config`](#config)

- `fn utf8(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn reverse(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn nfa_size_limit(self: Self, bytes: Option<usize>) -> Config` — [`Config`](#config)

- `fn shrink(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn captures(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn which_captures(self: Self, which_captures: WhichCaptures) -> Config` — [`WhichCaptures`](#whichcaptures), [`Config`](#config)

- `fn look_matcher(self: Self, m: LookMatcher) -> Config` — [`LookMatcher`](../../util/look/index.md), [`Config`](#config)

- `fn get_utf8(self: &Self) -> bool`

- `fn get_reverse(self: &Self) -> bool`

- `fn get_nfa_size_limit(self: &Self) -> Option<usize>`

- `fn get_shrink(self: &Self) -> bool`

- `fn get_captures(self: &Self) -> bool`

- `fn get_which_captures(self: &Self) -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

- `fn get_look_matcher(self: &Self) -> LookMatcher` — [`LookMatcher`](../../util/look/index.md)

- `fn get_unanchored_prefix(self: &Self) -> bool`

- `fn overwrite(self: &Self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](#config)

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

- `fn is_epsilon(self: &Self) -> bool`

- `fn memory_usage(self: &Self) -> usize`

- `fn remap(self: &mut Self, remap: &[StateID])` — [`StateID`](../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for State`

- `fn clone(self: &Self) -> State` — [`State`](#state)

##### `impl Debug for State`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for State`

##### `impl PartialEq for State`

- `fn eq(self: &Self, other: &State) -> bool` — [`State`](#state)

##### `impl StructuralPartialEq for State`

### `WhichCaptures`

```rust
enum WhichCaptures {
    All,
    Implicit,
    None,
}
```

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

- `fn is_none(self: &Self) -> bool`

- `fn is_any(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for WhichCaptures`

- `fn clone(self: &Self) -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

##### `impl Copy for WhichCaptures`

##### `impl Debug for WhichCaptures`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for WhichCaptures`

- `fn default() -> WhichCaptures` — [`WhichCaptures`](#whichcaptures)

