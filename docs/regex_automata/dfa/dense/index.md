*[regex_automata](../../index.md) / [dfa](../index.md) / [dense](index.md)*

---

# Module `dense`

Types and routines specific to dense DFAs.

This module is the home of [`dense::DFA`](DFA).

This module also contains a [`dense::Builder`](Builder) and a
[`dense::Config`](Config) for building and configuring a dense DFA.

## Structs

### `Config`

```rust
struct Config {
    accelerate: Option<bool>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
    minimize: Option<bool>,
    match_kind: Option<crate::util::search::MatchKind>,
    start_kind: Option<crate::dfa::start::StartKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<crate::util::alphabet::ByteSet>,
    specialize_start_states: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}
```

The configuration used for compiling a dense DFA.

As a convenience, `DFA::config` is an alias for `Config::new`. The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A dense DFA configuration is a simple data object that is typically used
with [`dense::Builder::configure`](self::Builder::configure).

The default configuration guarantees that a search will never return
a "quit" error, although it is possible for a search to fail if
`Config::starts_for_each_pattern` wasn't enabled (which it is
not by default) and an [`Anchored::Pattern`](../../index.md) mode is requested via
[`Input`](crate::Input).

#### Implementations

- `fn new() -> Config` — [`Config`](#config)

- `fn accelerate(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../util/prefilter/index.md), [`Config`](#config)

- `fn minimize(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn match_kind(self: Self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md), [`Config`](#config)

- `fn start_kind(self: Self, kind: StartKind) -> Config` — [`StartKind`](../start/index.md), [`Config`](#config)

- `fn starts_for_each_pattern(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn byte_classes(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn unicode_word_boundary(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn quit(self: Self, byte: u8, yes: bool) -> Config` — [`Config`](#config)

- `fn specialize_start_states(self: Self, yes: bool) -> Config` — [`Config`](#config)

- `fn dfa_size_limit(self: Self, bytes: Option<usize>) -> Config` — [`Config`](#config)

- `fn determinize_size_limit(self: Self, bytes: Option<usize>) -> Config` — [`Config`](#config)

- `fn get_accelerate(self: &Self) -> bool`

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md)

- `fn get_minimize(self: &Self) -> bool`

- `fn get_match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../../index.md)

- `fn get_starts(self: &Self) -> StartKind` — [`StartKind`](../start/index.md)

- `fn get_starts_for_each_pattern(self: &Self) -> bool`

- `fn get_byte_classes(self: &Self) -> bool`

- `fn get_unicode_word_boundary(self: &Self) -> bool`

- `fn get_quit(self: &Self, byte: u8) -> bool`

- `fn get_specialize_start_states(self: &Self) -> bool`

- `fn get_dfa_size_limit(self: &Self) -> Option<usize>`

- `fn get_determinize_size_limit(self: &Self) -> Option<usize>`

- `fn overwrite(self: &Self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](#config)

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

A builder for constructing a deterministic finite automaton from regular
expressions.

This builder provides two main things:

1. It provides a few different `build` routines for actually constructing
a DFA from different kinds of inputs. The most convenient is
`Builder::build`, which builds a DFA directly from a pattern string. The
most flexible is `Builder::build_from_nfa`, which builds a DFA straight
from an NFA.
2. The builder permits configuring a number of things.
`Builder::configure` is used with [`Config`](#config) to configure aspects of
the DFA and the construction process itself. `Builder::syntax` and
`Builder::thompson` permit configuring the regex parser and Thompson NFA
construction, respectively. The syntax and thompson configurations only
apply when building from a pattern string.

This builder always constructs a *single* DFA. As such, this builder
can only be used to construct regexes that either detect the presence
of a match or find the end location of a match. A single DFA cannot
produce both the start and end of a match. For that information, use a
[`Regex`](crate::dfa::regex::Regex), which can be similarly configured
using [`regex::Builder`](crate::dfa::regex::Builder). The main reason to
use a DFA directly is if the end location of a match is enough for your use
case. Namely, a `Regex` will construct two DFAs instead of one, since a
second reverse DFA is needed to find the start of a match.

Note that if one wants to build a sparse DFA, you must first build a dense
DFA and convert that to a sparse DFA. There is no way to build a sparse
DFA without first building a dense DFA.

# Example

This example shows how to build a minimized DFA that completely disables
Unicode. That is:

* Things such as `\w`, `.` and `\b` are no longer Unicode-aware. `\w`
  and `\b` are ASCII-only while `.` matches any byte except for `\n`
  (instead of any UTF-8 encoding of a Unicode scalar value except for
  `\n`). Things that are Unicode only, such as `\pL`, are not allowed.
* The pattern itself is permitted to match invalid UTF-8. For example,
  things like `[^a]` that match any byte except for `a` are permitted.

```rust
use regex_automata::{
    dfa::{Automaton, dense},
    util::syntax,
    HalfMatch, Input,
};

let dfa = dense::Builder::new()
    .configure(dense::Config::new().minimize(false))
    .syntax(syntax::Config::new().unicode(false).utf8(false))
    .build(r"foo[^b]ar.*")?;

let haystack = b"\xFEfoo\xFFar\xE2\x98\xFF\n";
let expected = Some(HalfMatch::must(0, 10));
let got = dfa.try_search_fwd(&Input::new(haystack))?;
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn build(self: &Self, pattern: &str) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>` — [`DFA`](#dfa), [`BuildError`](#builderror)

- `fn build_from_nfa(self: &Self, nfa: &thompson::NFA) -> Result<DFA<alloc::vec::Vec<u32>>, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md), [`DFA`](#dfa), [`BuildError`](#builderror)

- `fn configure(self: &mut Self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md), [`Builder`](#builder)

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](#builder)

### `DFA<T>`

```rust
struct DFA<T> {
    tt: TransitionTable<T>,
    st: StartTable<T>,
    ms: MatchStates<T>,
    special: crate::dfa::special::Special,
    accels: crate::dfa::accel::Accels<T>,
    pre: Option<crate::util::prefilter::Prefilter>,
    quitset: crate::util::alphabet::ByteSet,
    flags: Flags,
}
```

A dense table-based deterministic finite automaton (DFA).

All dense DFAs have one or more start states, zero or more match states
and a transition table that maps the current state and the current byte
of input to the next state. A DFA can use this information to implement
fast searching. In particular, the use of a dense DFA generally makes the
trade off that match speed is the most valuable characteristic, even if
building the DFA may take significant time *and* space. (More concretely,
building a DFA takes time and space that is exponential in the size of the
pattern in the worst case.) As such, the processing of every byte of input
is done with a small constant number of operations that does not vary with
the pattern, its size or the size of the alphabet. If your needs don't line
up with this trade off, then a dense DFA may not be an adequate solution to
your problem.

In contrast, a [`sparse::DFA`](../sparse/index.md) makes the opposite
trade off: it uses less space but will execute a variable number of
instructions per byte at match time, which makes it slower for matching.
(Note that space usage is still exponential in the size of the pattern in
the worst case.)

A DFA can be built using the default configuration via the
`DFA::new` constructor. Otherwise, one can
configure various aspects via [`dense::Builder`](Builder).

A single DFA fundamentally supports the following operations:

1. Detection of a match.
2. Location of the end of a match.
3. In the case of a DFA with multiple patterns, which pattern matched is
   reported as well.

A notable absence from the above list of capabilities is the location of
the *start* of a match. In order to provide both the start and end of
a match, *two* DFAs are required. This functionality is provided by a
[`Regex`](crate::dfa::regex::Regex).

# Type parameters

A `DFA` has one type parameter, `T`, which is used to represent state IDs,
pattern IDs and accelerators. `T` is typically a `Vec<u32>` or a `&[u32]`.

# The `Automaton` trait

This type implements the [`Automaton`](../automaton/index.md) trait, which means it can be used
for searching. For example:

```rust
use regex_automata::{dfa::{Automaton, dense::DFA}, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let expected = HalfMatch::must(0, 8);
assert_eq!(Some(expected), dfa.try_search_fwd(&Input::new("foo12345"))?);
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`tt`**: `TransitionTable<T>`

  The transition table for this DFA. This includes the transitions
  themselves, along with the stride, number of states and the equivalence
  class mapping.

- **`st`**: `StartTable<T>`

  The set of starting state identifiers for this DFA. The starting state
  IDs act as pointers into the transition table. The specific starting
  state chosen for each search is dependent on the context at which the
  search begins.

- **`ms`**: `MatchStates<T>`

  The set of match states and the patterns that match for each
  corresponding match state.
  
  This structure is technically only needed because of support for
  multi-regexes. Namely, multi-regexes require answering not just whether
  a match exists, but _which_ patterns match. So we need to store the
  matching pattern IDs for each match state. We do this even when there
  is only one pattern for the sake of simplicity. In practice, this uses
  up very little space for the case of one pattern.

- **`special`**: `crate::dfa::special::Special`

  Information about which states are "special." Special states are states
  that are dead, quit, matching, starting or accelerated. For more info,
  see the docs for `Special`.

- **`accels`**: `crate::dfa::accel::Accels<T>`

  The accelerators for this DFA.
  
  If a state is accelerated, then there exist only a small number of
  bytes that can cause the DFA to leave the state. This permits searching
  to use optimized routines to find those specific bytes instead of using
  the transition table.
  
  All accelerated states exist in a contiguous range in the DFA's
  transition table. See dfa/special.rs for more details on how states are
  arranged.

- **`pre`**: `Option<crate::util::prefilter::Prefilter>`

  Any prefilter attached to this DFA.
  
  Note that currently prefilters are not serialized. When deserializing
  a DFA from bytes, this is always set to `None`.

- **`quitset`**: `crate::util::alphabet::ByteSet`

  The set of "quit" bytes for this DFA.
  
  This is only used when computing the start state for a particular
  position in a haystack. Namely, in the case where there is a quit
  byte immediately before the start of the search, this set needs to be
  explicitly consulted. In all other cases, quit bytes are detected by
  the DFA itself, by transitioning all quit bytes to a special "quit
  state."

- **`flags`**: `Flags`

  Various flags describing the behavior of this DFA.

#### Implementations

- `fn set_start_state(self: &mut Self, anchored: Anchored, start: Start, id: StateID)` — [`Anchored`](../../index.md), [`Start`](../../util/start/index.md), [`StateID`](../../util/primitives/index.md)

- `fn set_transition(self: &mut Self, from: StateID, byte: alphabet::Unit, to: StateID)` — [`StateID`](../../util/primitives/index.md), [`Unit`](../../util/alphabet/index.md)

- `fn add_empty_state(self: &mut Self) -> Result<StateID, BuildError>` — [`StateID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn swap_states(self: &mut Self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md)

- `fn remap(self: &mut Self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md)

- `fn remap_state(self: &mut Self, id: StateID, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md)

- `fn truncate_states(self: &mut Self, len: usize)`

- `fn minimize(self: &mut Self)`

- `fn set_pattern_map(self: &mut Self, map: &BTreeMap<StateID, Vec<PatternID>>) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn accelerate(self: &mut Self)`

- `fn shuffle(self: &mut Self, matches: BTreeMap<StateID, Vec<PatternID>>) -> Result<(), BuildError>` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md), [`BuildError`](#builderror)

- `fn set_universal_starts(self: &mut Self)`

#### Trait Implementations

##### `impl<T: AsRef<[u32]>> Automaton for DFA<T>`

- `fn is_special_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_dead_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_quit_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_match_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_start_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn is_accel_state(self: &Self, id: StateID) -> bool` — [`StateID`](../../util/primitives/index.md)

- `fn next_state(self: &Self, current: StateID, input: u8) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `unsafe fn next_state_unchecked(self: &Self, current: StateID, byte: u8) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `fn next_eoi_state(self: &Self, current: StateID) -> StateID` — [`StateID`](../../util/primitives/index.md)

- `fn pattern_len(self: &Self) -> usize`

- `fn match_len(self: &Self, id: StateID) -> usize` — [`StateID`](../../util/primitives/index.md)

- `fn match_pattern(self: &Self, id: StateID, match_index: usize) -> PatternID` — [`StateID`](../../util/primitives/index.md), [`PatternID`](../../util/primitives/index.md)

- `fn has_empty(self: &Self) -> bool`

- `fn is_utf8(self: &Self) -> bool`

- `fn is_always_start_anchored(self: &Self) -> bool`

- `fn start_state(self: &Self, config: &start::Config) -> Result<StateID, StartError>` — [`Config`](../../util/start/index.md), [`StateID`](../../util/primitives/index.md), [`StartError`](../automaton/index.md)

- `fn universal_start_state(self: &Self, mode: Anchored) -> Option<StateID>` — [`Anchored`](../../index.md), [`StateID`](../../util/primitives/index.md)

- `fn accelerator(self: &Self, id: StateID) -> &[u8]` — [`StateID`](../../util/primitives/index.md)

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md)

##### `impl<T: $crate::clone::Clone> Clone for DFA<T>`

- `fn clone(self: &Self) -> DFA<T>` — [`DFA`](#dfa)

##### `impl<T: AsRef<[u32]>> Debug for DFA<T>`

- `fn fmt(self: &Self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Remappable for DFA<alloc::vec::Vec<u32>>`

- `fn state_len(self: &Self) -> usize`

- `fn stride2(self: &Self) -> usize`

- `fn swap_states(self: &mut Self, id1: StateID, id2: StateID)` — [`StateID`](../../util/primitives/index.md)

- `fn remap(self: &mut Self, map: impl Fn(StateID) -> StateID)` — [`StateID`](../../util/primitives/index.md)

### `BuildError`

```rust
struct BuildError {
    kind: BuildErrorKind,
}
```

An error that occurred during the construction of a DFA.

This error does not provide many introspection capabilities. There are
generally only two things you can do with it:

* Obtain a human readable message via its `std::fmt::Display` impl.
* Access an underlying [`nfa::thompson::BuildError`](thompson::BuildError)
type from its `source` method via the `std::error::Error` trait. This error
only occurs when using convenience routines for building a DFA directly
from a pattern string.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- `fn kind(self: &Self) -> &BuildErrorKind` — [`BuildErrorKind`](#builderrorkind)

- `fn nfa(err: thompson::BuildError) -> BuildError` — [`BuildError`](../../nfa/thompson/error/index.md)

- `fn unsupported_dfa_word_boundary_unicode() -> BuildError` — [`BuildError`](#builderror)

- `fn too_many_states() -> BuildError` — [`BuildError`](#builderror)

- `fn too_many_start_states() -> BuildError` — [`BuildError`](#builderror)

- `fn too_many_match_pattern_ids() -> BuildError` — [`BuildError`](#builderror)

- `fn dfa_exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

- `fn determinize_exceeded_size_limit(limit: usize) -> BuildError` — [`BuildError`](#builderror)

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

