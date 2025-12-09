*[regex_automata](../../index.md) / [dfa](../index.md) / [onepass](index.md)*

---

# Module `onepass`

A DFA that can return spans for matching capturing groups.

This module is the home of a [one-pass DFA](DFA).

This module also contains a [`Builder`](#builder) and a [`Config`](#config) for building and
configuring a one-pass DFA.

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`InternalBuilder`](#internalbuilder)
  - [`DFA`](#dfa)
  - [`SparseTransitionIter`](#sparsetransitioniter)
  - [`Cache`](#cache)
  - [`Transition`](#transition)
  - [`PatternEpsilons`](#patternepsilons)
  - [`Epsilons`](#epsilons)
  - [`Slots`](#slots)
  - [`SlotsIter`](#slotsiter)
  - [`BuildError`](#builderror)
- [Enums](#enums)
  - [`BuildErrorKind`](#builderrorkind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used for building a [one-pass DFA](DFA). |
| [`Builder`](#builder) | struct | A builder for a [one-pass DFA](DFA). |
| [`InternalBuilder`](#internalbuilder) | struct | An internal builder for encapsulating the state necessary to build a |
| [`DFA`](#dfa) | struct | A one-pass DFA for executing a subset of anchored regex searches while |
| [`SparseTransitionIter`](#sparsetransitioniter) | struct | An iterator over groups of consecutive equivalent transitions in a single |
| [`Cache`](#cache) | struct | A cache represents mutable state that a one-pass [`DFA`] requires during a |
| [`Transition`](#transition) | struct | Represents a single transition in a one-pass DFA. |
| [`PatternEpsilons`](#patternepsilons) | struct | A representation of a match state's pattern ID along with the epsilons for |
| [`Epsilons`](#epsilons) | struct | Epsilons represents all of the NFA epsilons transitions that went into a |
| [`Slots`](#slots) | struct | The set of epsilon transitions indicating that the current position in a |
| [`SlotsIter`](#slotsiter) | struct | An iterator over all of the bits set in a slot set. |
| [`BuildError`](#builderror) | struct | An error that occurred during the construction of a one-pass DFA. |
| [`BuildErrorKind`](#builderrorkind) | enum | The kind of error that occurred during the construction of a one-pass DFA. |

## Structs

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
```

The configuration used for building a [one-pass DFA](DFA).

A one-pass DFA configuration is a simple data object that is typically used
with `Builder::configure`. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with `DFA::config`.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md), [`Config`](#config)

- <span id="config-starts-for-each-pattern"></span>`fn starts_for_each_pattern(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-size-limit"></span>`fn size_limit(self, limit: Option<usize>) -> Config` — [`Config`](#config)

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../index.md)

- <span id="config-get-starts-for-each-pattern"></span>`fn get_starts_for_each_pattern(&self) -> bool`

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

- <span id="config-get-size-limit"></span>`fn get_size_limit(&self) -> Option<usize>`

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

A builder for a [one-pass DFA](DFA).

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the DFA construction. This builder is different from a
general purpose regex builder in that it permits fine grain configuration
of the construction process. The trade off for this is complexity, and
the possibility of setting a configuration that might not make sense. For
example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* `thompson::Config::utf8` controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the NFA.
This is generally what you want for matching on arbitrary bytes.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    nfa::thompson,
    util::syntax,
    Match,
};

let re = DFA::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

let haystack = b"foo\xFFarzz\xE2\x98\xFF\n";
re.captures(&mut cache, haystack, &mut caps);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a one-pass DFA Config,
//  since that only impacts regexes that can
// produce matches of length 0.
assert_eq!(Some(Match::must(0, 0..8)), caps.get_match());

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: NFA) -> Result<DFA, BuildError>` — [`NFA`](../../nfa/thompson/index.md), [`DFA`](#dfa), [`BuildError`](#builderror)

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md), [`Builder`](#builder)

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `InternalBuilder<'a>`

```rust
struct InternalBuilder<'a> {
    dfa: DFA,
    uncompiled_nfa_ids: alloc::vec::Vec<crate::util::primitives::StateID>,
    nfa_to_dfa_id: alloc::vec::Vec<crate::util::primitives::StateID>,
    stack: alloc::vec::Vec<(crate::util::primitives::StateID, Epsilons)>,
    seen: crate::util::sparse_set::SparseSet,
    matched: bool,
    config: Config,
    nfa: &'a crate::nfa::thompson::NFA,
    classes: crate::util::alphabet::ByteClasses,
}
```

An internal builder for encapsulating the state necessary to build a
one-pass DFA. Typical use is just `InternalBuilder::new(..).build()`.

There is no separate pass for determining whether the NFA is one-pass or
not. We just try to build the DFA. If during construction we discover that
it is not one-pass, we bail out. This is likely to lead to some undesirable
expense in some cases, so it might make sense to try an identify common
patterns in the NFA that make it definitively not one-pass. That way, we
can avoid ever trying to build a one-pass DFA in the first place. For
example, '\w*\s' is not one-pass, and since '\w' is Unicode-aware by
default, it's probably not a trivial cost to try and build a one-pass DFA
for it and then fail.

Note that some (immutable) fields are duplicated here. For example, the
'nfa' and 'classes' fields are both in the 'DFA'. They are the same thing,
but we duplicate them because it makes composition easier below. Otherwise,
since the borrow checker can't see through method calls, the mutable borrow
we use to mutate the DFA winds up preventing borrowing from any other part
of the DFA, even though we aren't mutating those parts. We only do this
because the duplication is cheap.

#### Fields

- **`dfa`**: `DFA`

  The DFA we're building.

- **`uncompiled_nfa_ids`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  An unordered collection of NFA state IDs that we haven't yet tried to
  build into a DFA state yet.
  
  This collection does not ultimately wind up including every NFA state
  ID. Instead, each ID represents a "start" state for a sub-graph of the
  NFA. The set of NFA states we then use to build a DFA state consists
  of that "start" state and all states reachable from it via epsilon
  transitions.

- **`nfa_to_dfa_id`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  A map from NFA state ID to DFA state ID. This is useful for easily
  determining whether an NFA state has been used as a "starting" point
  to build a DFA state yet. If it hasn't, then it is mapped to DEAD,
  and since DEAD is specially added and never corresponds to any NFA
  state, it follows that a mapping to DEAD implies the NFA state has
  no corresponding DFA state yet.

- **`stack`**: `alloc::vec::Vec<(crate::util::primitives::StateID, Epsilons)>`

  A stack used to traverse the NFA states that make up a single DFA
  state. Traversal occurs until the stack is empty, and we only push to
  the stack when the state ID isn't in 'seen'. Actually, even more than
  that, if we try to push something on to this stack that is already in
  'seen', then we bail out on construction completely, since it implies
  that the NFA is not one-pass.

- **`seen`**: `crate::util::sparse_set::SparseSet`

  The set of NFA states that we've visited via 'stack'.

- **`matched`**: `bool`

  Whether a match NFA state has been observed while constructing a
  one-pass DFA state. Once a match state is seen, assuming we are using
  leftmost-first match semantics, then we don't add any more transitions
  to the DFA state we're building.

- **`config`**: `Config`

  The config passed to the builder.
  
  This is duplicated in dfa.config.

- **`nfa`**: `&'a crate::nfa::thompson::NFA`

  The NFA we're building a one-pass DFA from.
  
  This is duplicated in dfa.nfa.

- **`classes`**: `crate::util::alphabet::ByteClasses`

  The equivalence classes that make up the alphabet for this DFA>
  
  This is duplicated in dfa.classes.

#### Implementations

- <span id="internalbuilder-new"></span>`fn new(config: Config, nfa: &'a NFA) -> InternalBuilder<'a>` — [`Config`](#config), [`NFA`](../../nfa/thompson/index.md), [`InternalBuilder`](#internalbuilder)

- <span id="internalbuilder-build"></span>`fn build(self) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- <span id="internalbuilder-shuffle-states"></span>`fn shuffle_states(&mut self)`

- <span id="internalbuilder-compile-transition"></span>`fn compile_transition(&mut self, dfa_id: StateID, trans: &thompson::Transition, epsilons: Epsilons) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`Transition`](../../nfa/thompson/index.md), [`Epsilons`](#epsilons), [`BuildError`](#builderror)

- <span id="internalbuilder-add-start-state"></span>`fn add_start_state(&mut self, pid: Option<PatternID>, nfa_id: StateID) -> Result<StateID, BuildError>` — [`PatternID`](../../index.md), [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- <span id="internalbuilder-add-dfa-state-for-nfa-state"></span>`fn add_dfa_state_for_nfa_state(&mut self, nfa_id: StateID) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- <span id="internalbuilder-add-empty-state"></span>`fn add_empty_state(&mut self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- <span id="internalbuilder-stack-push"></span>`fn stack_push(&mut self, nfa_id: StateID, epsilons: Epsilons) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`Epsilons`](#epsilons), [`BuildError`](#builderror)

#### Trait Implementations

##### `impl<'a> Debug for InternalBuilder<'a>`

- <span id="internalbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `DFA`

```rust
struct DFA {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
    table: alloc::vec::Vec<Transition>,
    starts: alloc::vec::Vec<crate::util::primitives::StateID>,
    min_match_id: crate::util::primitives::StateID,
    classes: crate::util::alphabet::ByteClasses,
    alphabet_len: usize,
    stride2: usize,
    pateps_offset: usize,
    explicit_slot_start: usize,
}
```

A one-pass DFA for executing a subset of anchored regex searches while
resolving capturing groups.

A one-pass DFA can be built from an NFA that is one-pass. An NFA is
one-pass when there is never any ambiguity about how to continue a search.
For example, `a*a` is not one-pass because during a search, it's not
possible to know whether to continue matching the `a*` or to move on to
the single `a`. However, `a*b` is one-pass, because for every byte in the
input, it's always clear when to move on from `a*` to `b`.

# Only anchored searches are supported

In this crate, especially for DFAs, unanchored searches are implemented by
treating the pattern as if it had a `(?s-u:.)*?` prefix. While the prefix
is one-pass on its own, adding anything after it, e.g., `(?s-u:.)*?a` will
make the overall pattern not one-pass. Why? Because the `(?s-u:.)` matches
any byte, and there is therefore ambiguity as to when the prefix should
stop matching and something else should start matching.

Therefore, one-pass DFAs do not support unanchored searches. In addition
to many regexes simply not being one-pass, it implies that one-pass DFAs
have limited utility. With that said, when a one-pass DFA can be used, it
can potentially provide a dramatic speed up over alternatives like the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
and the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM). In particular,
a one-pass DFA is the only DFA capable of reporting the spans of matching
capturing groups.

To clarify, when we say that unanchored searches are not supported, what
that actually means is:

* The high level routines, `DFA::is_match` and `DFA::captures`, always
do anchored searches.
* Since iterators are most useful in the context of unanchored searches,
there is no `DFA::captures_iter` method.
* For lower level routines like `DFA::try_search`, an error will be
returned if the given [`Input`](../../index.md) is configured to do an unanchored search or
search for an invalid pattern ID. (Note that an [`Input`](../../index.md) is configured to
do an unanchored search by default, so just giving a `Input::new` is
guaranteed to return an error.)

# Other limitations

In addition to the [configurable heap limit](Config::size_limit) and
the requirement that a regex pattern be one-pass, there are some other
limitations:

* There is an internal limit on the total number of explicit capturing
groups that appear across all patterns. It is somewhat small and there is
no way to configure it. If your pattern(s) exceed this limit, then building
a one-pass DFA will fail.
* If the number of patterns exceeds an internal unconfigurable limit, then
building a one-pass DFA will fail. This limit is quite large and you're
unlikely to hit it.
* If the total number of states exceeds an internal unconfigurable limit,
then building a one-pass DFA will fail. This limit is quite large and
you're unlikely to hit it.

# Other examples of regexes that aren't one-pass

One particularly unfortunate example is that enabling Unicode can cause
regexes that were one-pass to no longer be one-pass. Consider the regex
`(?-u)\w*\s` for example. It is one-pass because there is exactly no
overlap between the ASCII definitions of `\w` and `\s`. But `\w*\s`
(i.e., with Unicode enabled) is *not* one-pass because `\w` and `\s` get
translated to UTF-8 automatons. And while the *codepoints* in `\w` and `\s`
do not overlap, the underlying UTF-8 encodings do. Indeed, because of the
overlap between UTF-8 automata, the use of Unicode character classes will
tend to vastly increase the likelihood of a regex not being one-pass.

# How does one know if a regex is one-pass or not?

At the time of writing, the only way to know is to try and build a one-pass
DFA. The one-pass property is checked while constructing the DFA.

This does mean that you might potentially waste some CPU cycles and memory
by optimistically trying to build a one-pass DFA. But this is currently the
only way. In the future, building a one-pass DFA might be able to use some
heuristics to detect common violations of the one-pass property and bail
more quickly.

# Resource usage

Unlike a general DFA, a one-pass DFA has stricter bounds on its resource
usage. Namely, construction of a one-pass DFA has a time and space
complexity of `O(n)`, where `n ~ nfa.states().len()`. (A general DFA's time
and space complexity is `O(2^n)`.) This smaller time bound is achieved
because there is at most one DFA state created for each NFA state. If
additional DFA states would be required, then the pattern is not one-pass
and construction will fail.

Note though that currently, this DFA uses a fully dense representation.
This means that while its space complexity is no worse than an NFA, it may
in practice use more memory because of higher constant factors. The reason
for this trade off is two-fold. Firstly, a dense representation makes the
search faster. Secondly, the bigger an NFA, the more unlikely it is to be
one-pass. Therefore, most one-pass DFAs are usually pretty small.

# Example

This example shows that the one-pass DFA implements Unicode word boundaries
correctly while simultaneously reporting spans for capturing groups that
participate in a match. (This is the only DFA that implements full support
for Unicode word boundaries.)

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::onepass::DFA, Match, Span};

let re = DFA::new(r"\b(?P<first>\w+)[[:space:]]+(?P<last>\w+)\b")?;
let (mut cache, mut caps) = (re.create_cache(), re.create_captures());

re.captures(&mut cache, "Шерлок Холмс", &mut caps);
assert_eq!(Some(Match::must(0, 0..23)), caps.get_match());
assert_eq!(Some(Span::from(0..12)), caps.get_group_by_name("first"));
assert_eq!(Some(Span::from(13..23)), caps.get_group_by_name("last"));
Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: iteration

Unlike other regex engines in this crate, this one does not provide
iterator search functions. This is because a one-pass DFA only supports
anchored searches, and so iterator functions are generally not applicable.

However, if you know that all of your matches are
directly adjacent, then an iterator can be used. The
[`util::iter::Searcher`](crate::util::iter::Searcher) type can be used for
this purpose:

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::onepass::DFA,
    util::iter::Searcher,
    Anchored, Input, Span,
};

let re = DFA::new(r"\w(\d)\w")?;
let (mut cache, caps) = (re.create_cache(), re.create_captures());
let input = Input::new("a1zb2yc3x").anchored(Anchored::Yes);

let mut it = Searcher::new(input).into_captures_iter(caps, |input, caps| {
    Ok(re.try_search(&mut cache, input, caps)?)
}).infallible();
let caps0 = it.next().unwrap();
assert_eq!(Some(Span::from(1..2)), caps0.get_group(1));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`config`**: `Config`

  The configuration provided by the caller.

- **`nfa`**: `crate::nfa::thompson::NFA`

  The NFA used to build this DFA.
  
  NOTE: We probably don't need to store the NFA here, but we use enough
  bits from it that it's convenient to do so. And there really isn't much
  cost to doing so either, since an NFA is reference counted internally.

- **`table`**: `alloc::vec::Vec<Transition>`

  The transition table. Given a state ID 's' and a byte of haystack 'b',
  the next state is `table[sid + classes[byte]]`.
  
  The stride of this table (i.e., the number of columns) is always
  a power of 2, even if the alphabet length is smaller. This makes
  converting between state IDs and state indices very cheap.
  
  Note that the stride always includes room for one extra "transition"
  that isn't actually a transition. It is a 'PatternEpsilons' that is
  used for match states only. Because of this, the maximum number of
  active columns in the transition table is 257, which means the maximum
  stride is 512 (the next power of 2 greater than or equal to 257).

- **`starts`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The DFA state IDs of the starting states.
  
  `starts[0]` is always present and corresponds to the starting state
  when searching for matches of any pattern in the DFA.
  
  `starts[i]` where i>0 corresponds to the starting state for the pattern
  ID 'i-1'. These starting states are optional.

- **`min_match_id`**: `crate::util::primitives::StateID`

  Every state ID >= this value corresponds to a match state.
  
  This is what a search uses to detect whether a state is a match state
  or not. It requires only a simple comparison instead of bit-unpacking
  the PatternEpsilons from every state.

- **`classes`**: `crate::util::alphabet::ByteClasses`

  The alphabet of this DFA, split into equivalence classes. Bytes in the
  same equivalence class can never discriminate between a match and a
  non-match.

- **`alphabet_len`**: `usize`

  The number of elements in each state in the transition table. This may
  be less than the stride, since the stride is always a power of 2 and
  the alphabet length can be anything up to and including 256.

- **`stride2`**: `usize`

  The number of columns in the transition table, expressed as a power of
  2.

- **`pateps_offset`**: `usize`

  The offset at which the PatternEpsilons for a match state is stored in
  the transition table.
  
  PERF: One wonders whether it would be better to put this in a separate
  allocation, since only match states have a non-empty PatternEpsilons
  and the number of match states tends be dwarfed by the number of
  non-match states. So this would save '8*len(non_match_states)' for each
  DFA. The question is whether moving this to a different allocation will
  lead to a perf hit during searches. You might think dealing with match
  states is rare, but some regexes spend a lot of time in match states
  gobbling up input. But... match state handling is already somewhat
  expensive, so maybe this wouldn't do much? Either way, it's worth
  experimenting.

- **`explicit_slot_start`**: `usize`

  The first explicit slot index. This refers to the first slot appearing
  immediately after the last implicit slot. It is always 'patterns.len()
  * 2'.
  
  We record this because we only store the explicit slots in our DFA
  transition table that need to be saved. Implicit slots are handled
  automatically as part of the search.

#### Implementations

- <span id="dfa-start"></span>`fn start(&self) -> StateID` — [`StateID`](../../util/primitives/index.md)

- <span id="dfa-start-pattern"></span>`fn start_pattern(&self, pid: PatternID) -> Result<StateID, MatchError>` — [`PatternID`](../../index.md), [`StateID`](../../util/primitives/index.md), [`MatchError`](../../index.md)

- <span id="dfa-transition"></span>`fn transition(&self, sid: StateID, byte: u8) -> Transition` — [`StateID`](../../util/primitives/index.md), [`Transition`](#transition)

- <span id="dfa-set-transition"></span>`fn set_transition(&mut self, sid: StateID, byte: u8, to: Transition)` — [`StateID`](../../util/primitives/index.md), [`Transition`](#transition)

- <span id="dfa-sparse-transitions"></span>`fn sparse_transitions(&self, sid: StateID) -> SparseTransitionIter<'_>` — [`StateID`](../../util/primitives/index.md), [`SparseTransitionIter`](#sparsetransitioniter)

- <span id="dfa-pattern-epsilons"></span>`fn pattern_epsilons(&self, sid: StateID) -> PatternEpsilons` — [`StateID`](../../util/primitives/index.md), [`PatternEpsilons`](#patternepsilons)

- <span id="dfa-set-pattern-epsilons"></span>`fn set_pattern_epsilons(&mut self, sid: StateID, pateps: PatternEpsilons)` — [`StateID`](../../util/primitives/index.md), [`PatternEpsilons`](#patternepsilons)

- <span id="dfa-prev-state-id"></span>`fn prev_state_id(&self, id: StateID) -> Option<StateID>` — [`StateID`](../../util/primitives/index.md)

- <span id="dfa-last-state-id"></span>`fn last_state_id(&self) -> StateID` — [`StateID`](../../util/primitives/index.md)

- <span id="dfa-swap-states"></span>`fn swap_states(&mut self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md)

- <span id="dfa-remap"></span>`fn remap(&mut self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for DFA`

- <span id="dfa-clone"></span>`fn clone(&self) -> DFA` — [`DFA`](#dfa)

##### `impl Debug for DFA`

- <span id="dfa-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Remappable for crate::dfa::onepass::DFA`

- <span id="cratedfaonepassdfa-state-len"></span>`fn state_len(&self) -> usize`

- <span id="cratedfaonepassdfa-stride2"></span>`fn stride2(&self) -> usize`

- <span id="cratedfaonepassdfa-swap-states"></span>`fn swap_states(&mut self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md)

- <span id="cratedfaonepassdfa-remap"></span>`fn remap(&mut self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md)

### `SparseTransitionIter<'a>`

```rust
struct SparseTransitionIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, Transition>>,
    cur: Option<(u8, u8, Transition)>,
}
```

An iterator over groups of consecutive equivalent transitions in a single
state.

#### Trait Implementations

##### `impl<'a> Debug for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="sparsetransitioniter-intoiter"></span>`type IntoIter = I`

- <span id="sparsetransitioniter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'a> Iterator for SparseTransitionIter<'a>`

- <span id="sparsetransitioniter-item"></span>`type Item = (u8, u8, Transition)`

- <span id="sparsetransitioniter-next"></span>`fn next(&mut self) -> Option<(u8, u8, Transition)>` — [`Transition`](#transition)

### `Cache`

```rust
struct Cache {
    explicit_slots: alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>,
    explicit_slot_len: usize,
}
```

A cache represents mutable state that a one-pass [`DFA`](#dfa) requires during a
search.

For a given one-pass DFA, its corresponding cache may be created either via
`DFA::create_cache`, or via `Cache::new`. They are equivalent in every
way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the one-pass DFA from which it was
created. It may only be used with that one-pass DFA. A cache and its
allocations may be re-purposed via `Cache::reset`, in which case, it can
only be used with the new one-pass DFA (and not the old one).

#### Fields

- **`explicit_slots`**: `alloc::vec::Vec<Option<crate::util::primitives::NonMaxUsize>>`

  Scratch space used to store slots during a search. Basically, we use
  the caller provided slots to store slots known when a match occurs.
  But after a match occurs, we might continue a search but ultimately
  fail to extend the match. When continuing the search, we need some
  place to store candidate capture offsets without overwriting the slot
  offsets recorded for the most recently seen match.

- **`explicit_slot_len`**: `usize`

  The number of slots in the caller-provided 'Captures' value for the
  current search. This is always at most 'explicit_slots.len()', but
  might be less than it, if the caller provided fewer slots to fill.

#### Implementations

- <span id="cache-new"></span>`fn new(re: &DFA) -> Cache` — [`DFA`](#dfa), [`Cache`](#cache)

- <span id="cache-reset"></span>`fn reset(&mut self, re: &DFA)` — [`DFA`](#dfa)

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="cache-explicit-slots"></span>`fn explicit_slots(&mut self) -> &mut [Option<NonMaxUsize>]` — [`NonMaxUsize`](../../util/primitives/index.md)

- <span id="cache-setup-search"></span>`fn setup_search(&mut self, explicit_slot_len: usize)`

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Transition`

```rust
struct Transition(u64);
```

Represents a single transition in a one-pass DFA.

The high 21 bits corresponds to the state ID. The bit following corresponds
to the special "match wins" flag. The remaining low 42 bits corresponds to
the transition epsilons, which contains the slots that should be saved when
this transition is followed and the conditional epsilon transitions that
must be satisfied in order to follow this transition.

#### Implementations

- <span id="transition-state-id-bits"></span>`const STATE_ID_BITS: u64`

- <span id="transition-state-id-shift"></span>`const STATE_ID_SHIFT: u64`

- <span id="transition-state-id-limit"></span>`const STATE_ID_LIMIT: u64`

- <span id="transition-match-wins-shift"></span>`const MATCH_WINS_SHIFT: u64`

- <span id="transition-info-mask"></span>`const INFO_MASK: u64`

- <span id="transition-new"></span>`fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition` — [`StateID`](../../util/primitives/index.md), [`Epsilons`](#epsilons), [`Transition`](#transition)

- <span id="transition-is-dead"></span>`fn is_dead(self) -> bool`

- <span id="transition-match-wins"></span>`fn match_wins(&self) -> bool`

- <span id="transition-state-id"></span>`fn state_id(&self) -> StateID` — [`StateID`](../../util/primitives/index.md)

- <span id="transition-set-state-id"></span>`fn set_state_id(&mut self, sid: StateID)` — [`StateID`](../../util/primitives/index.md)

- <span id="transition-epsilons"></span>`fn epsilons(&self) -> Epsilons` — [`Epsilons`](#epsilons)

#### Trait Implementations

##### `impl Clone for Transition`

- <span id="transition-clone"></span>`fn clone(&self) -> Transition` — [`Transition`](#transition)

##### `impl Copy for Transition`

##### `impl Debug for Transition`

- <span id="transition-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Transition`

##### `impl PartialEq for Transition`

- <span id="transition-eq"></span>`fn eq(&self, other: &Transition) -> bool` — [`Transition`](#transition)

##### `impl StructuralPartialEq for Transition`

### `PatternEpsilons`

```rust
struct PatternEpsilons(u64);
```

A representation of a match state's pattern ID along with the epsilons for
when a match occurs.

A match state in a one-pass DFA, unlike in a more general DFA, has exactly
one pattern ID. If it had more, then the original NFA would not have been
one-pass.

The "epsilons" part of this corresponds to what was found in the epsilon
transitions between the transition taken in the last byte of input and the
ultimate match state. This might include saving slots and/or conditional
epsilon transitions that must be satisfied before one can report the match.

Technically, every state has room for a 'PatternEpsilons', but it is only
ever non-empty for match states.

#### Implementations

- <span id="patternepsilons-pattern-id-bits"></span>`const PATTERN_ID_BITS: u64`

- <span id="patternepsilons-pattern-id-shift"></span>`const PATTERN_ID_SHIFT: u64`

- <span id="patternepsilons-pattern-id-none"></span>`const PATTERN_ID_NONE: u64`

- <span id="patternepsilons-pattern-id-limit"></span>`const PATTERN_ID_LIMIT: u64`

- <span id="patternepsilons-pattern-id-mask"></span>`const PATTERN_ID_MASK: u64`

- <span id="patternepsilons-epsilons-mask"></span>`const EPSILONS_MASK: u64`

- <span id="patternepsilons-empty"></span>`fn empty() -> PatternEpsilons` — [`PatternEpsilons`](#patternepsilons)

- <span id="patternepsilons-is-empty"></span>`fn is_empty(self) -> bool`

- <span id="patternepsilons-pattern-id"></span>`fn pattern_id(self) -> Option<PatternID>` — [`PatternID`](../../index.md)

- <span id="patternepsilons-pattern-id-unchecked"></span>`fn pattern_id_unchecked(self) -> PatternID` — [`PatternID`](../../index.md)

- <span id="patternepsilons-set-pattern-id"></span>`fn set_pattern_id(self, pid: PatternID) -> PatternEpsilons` — [`PatternID`](../../index.md), [`PatternEpsilons`](#patternepsilons)

- <span id="patternepsilons-epsilons"></span>`fn epsilons(self) -> Epsilons` — [`Epsilons`](#epsilons)

- <span id="patternepsilons-set-epsilons"></span>`fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons` — [`Epsilons`](#epsilons), [`PatternEpsilons`](#patternepsilons)

#### Trait Implementations

##### `impl Clone for PatternEpsilons`

- <span id="patternepsilons-clone"></span>`fn clone(&self) -> PatternEpsilons` — [`PatternEpsilons`](#patternepsilons)

##### `impl Copy for PatternEpsilons`

##### `impl Debug for PatternEpsilons`

- <span id="patternepsilons-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Epsilons`

```rust
struct Epsilons(u64);
```

Epsilons represents all of the NFA epsilons transitions that went into a
single transition in a single DFA state. In this case, it only represents
the epsilon transitions that have some kind of non-consuming side effect:
either the transition requires storing the current position of the search
into a slot, or the transition is conditional and requires the current
position in the input to satisfy an assertion before the transition may be
taken.

This folds the cumulative effect of a group of NFA states (all connected
by epsilon transitions) down into a single set of bits. While these bits
can represent all possible conditional epsilon transitions, it only permits
storing up to a somewhat small number of slots.

Epsilons is represented as a 42-bit integer. For example, it is packed into
the lower 42 bits of a `Transition`. (Where the high 22 bits contains a
`StateID` and a special "match wins" property.)

#### Implementations

- <span id="epsilons-slot-mask"></span>`const SLOT_MASK: u64`

- <span id="epsilons-slot-shift"></span>`const SLOT_SHIFT: u64`

- <span id="epsilons-look-mask"></span>`const LOOK_MASK: u64`

- <span id="epsilons-empty"></span>`fn empty() -> Epsilons` — [`Epsilons`](#epsilons)

- <span id="epsilons-is-empty"></span>`fn is_empty(self) -> bool`

- <span id="epsilons-slots"></span>`fn slots(self) -> Slots` — [`Slots`](#slots)

- <span id="epsilons-set-slots"></span>`fn set_slots(self, slots: Slots) -> Epsilons` — [`Slots`](#slots), [`Epsilons`](#epsilons)

- <span id="epsilons-looks"></span>`fn looks(self) -> LookSet` — [`LookSet`](../../util/look/index.md)

- <span id="epsilons-set-looks"></span>`fn set_looks(self, look_set: LookSet) -> Epsilons` — [`LookSet`](../../util/look/index.md), [`Epsilons`](#epsilons)

#### Trait Implementations

##### `impl Clone for Epsilons`

- <span id="epsilons-clone"></span>`fn clone(&self) -> Epsilons` — [`Epsilons`](#epsilons)

##### `impl Copy for Epsilons`

##### `impl Debug for Epsilons`

- <span id="epsilons-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Slots`

```rust
struct Slots(u32);
```

The set of epsilon transitions indicating that the current position in a
search should be saved to a slot.

This *only* represents explicit slots. So for example, the pattern
`[a-z]+([0-9]+)([a-z]+)` has:

* 3 capturing groups, thus 6 slots.
* 1 implicit capturing group, thus 2 implicit slots.
* 2 explicit capturing groups, thus 4 explicit slots.

While implicit slots are represented by epsilon transitions in an NFA, we
do not explicitly represent them here. Instead, implicit slots are assumed
to be present and handled automatically in the search code. Therefore,
that means we only need to represent explicit slots in our epsilon
transitions.

Its representation is a bit set. The bit 'i' is set if and only if there
exists an explicit slot at index 'c', where 'c = (#patterns * 2) + i'. That
is, the bit 'i' corresponds to the first explicit slot and the first
explicit slot appears immediately following the last implicit slot. (If
this is confusing, see `GroupInfo` for more details on how slots works.)

A single `Slots` represents all the active slots in a sub-graph of an NFA,
where all the states are connected by epsilon transitions. In effect, when
traversing the one-pass DFA during a search, all slots set in a particular
transition must be captured by recording the current search position.

The API of `Slots` requires the caller to handle the explicit slot offset.
That is, a `Slots` doesn't know where the explicit slots start for a
particular NFA. Thus, if the callers see's the bit 'i' is set, then they
need to do the arithmetic above to find 'c', which is the real actual slot
index in the corresponding NFA.

#### Implementations

- <span id="slots-limit"></span>`const LIMIT: usize`

- <span id="slots-insert"></span>`fn insert(self, slot: usize) -> Slots` — [`Slots`](#slots)

- <span id="slots-remove"></span>`fn remove(self, slot: usize) -> Slots` — [`Slots`](#slots)

- <span id="slots-is-empty"></span>`fn is_empty(self) -> bool`

- <span id="slots-iter"></span>`fn iter(self) -> SlotsIter` — [`SlotsIter`](#slotsiter)

- <span id="slots-apply"></span>`fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>])` — [`NonMaxUsize`](../../util/primitives/index.md)

#### Trait Implementations

##### `impl Clone for Slots`

- <span id="slots-clone"></span>`fn clone(&self) -> Slots` — [`Slots`](#slots)

##### `impl Copy for Slots`

##### `impl Debug for Slots`

- <span id="slots-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `SlotsIter`

```rust
struct SlotsIter {
    slots: Slots,
}
```

An iterator over all of the bits set in a slot set.

This returns the bit index that is set, so callers may need to offset it
to get the actual NFA slot index.

#### Trait Implementations

##### `impl Debug for SlotsIter`

- <span id="slotsiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for SlotsIter`

- <span id="slotsiter-item"></span>`type Item = <I as Iterator>::Item`

- <span id="slotsiter-intoiter"></span>`type IntoIter = I`

- <span id="slotsiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SlotsIter`

- <span id="slotsiter-item"></span>`type Item = usize`

- <span id="slotsiter-next"></span>`fn next(&mut self) -> Option<usize>`

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

An error that occurred during the construction of a one-pass DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`thompson::BuildError`](../../nfa/thompson/index.md) type from its `source`
method via the `std::error::Error` trait. This error only occurs when using
convenience routines for building a one-pass DFA directly from a pattern
string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="builderror-nfa"></span>`fn nfa(err: crate::nfa::thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/index.md)

- <span id="builderror-word"></span>`fn word(err: UnicodeWordBoundaryError) -> BuildError` — [`UnicodeWordBoundaryError`](../../util/look/index.md), [`BuildError`](#builderror)

- <span id="builderror-too-many-states"></span>`fn too_many_states(limit: u64) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-too-many-patterns"></span>`fn too_many_patterns(limit: u64) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-unsupported-look"></span>`fn unsupported_look(look: Look) -> BuildError` — [`Look`](../../util/look/index.md), [`BuildError`](#builderror)

- <span id="builderror-exceeded-size-limit"></span>`fn exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

- <span id="builderror-not-one-pass"></span>`fn not_one_pass(msg: &'static str) -> BuildError` — [`BuildError`](#builderror)

#### Trait Implementations

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](#builderror)

##### `impl Debug for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

- <span id="builderror-source"></span>`fn source(&self) -> Option<&dyn std::error::Error>`

##### `impl<T> ToString for BuildError`

- <span id="builderror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `BuildErrorKind`

```rust
enum BuildErrorKind {
    NFA(crate::nfa::thompson::BuildError),
    Word(crate::util::look::UnicodeWordBoundaryError),
    TooManyStates {
        limit: u64,
    },
    TooManyPatterns {
        limit: u64,
    },
    UnsupportedLook {
        look: crate::util::look::Look,
    },
    ExceededSizeLimit {
        limit: usize,
    },
    NotOnePass {
        msg: &'static str,
    },
}
```

The kind of error that occurred during the construction of a one-pass DFA.

#### Trait Implementations

##### `impl Clone for BuildErrorKind`

- <span id="builderrorkind-clone"></span>`fn clone(&self) -> BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

##### `impl Debug for BuildErrorKind`

- <span id="builderrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

