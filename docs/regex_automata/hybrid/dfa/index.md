*[regex_automata](../../index.md) / [hybrid](../index.md) / [dfa](index.md)*

---

# Module `dfa`

Types and routines specific to lazy DFAs.

This module is the home of [`hybrid::dfa::DFA`](DFA).

This module also contains a [`hybrid::dfa::Builder`](Builder) and a
[`hybrid::dfa::Config`](Config) for configuring and building a lazy DFA.

## Structs

### `DFA`

```rust
struct DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: crate::util::start::StartByteMap,
    classes: crate::util::alphabet::ByteClasses,
    quitset: crate::util::alphabet::ByteSet,
    cache_capacity: usize,
}
```

A hybrid NFA/DFA (also called a "lazy DFA") for regex searching.

A lazy DFA is a DFA that builds itself at search time. It otherwise has
very similar characteristics as a [`dense::DFA`](crate::dfa::dense::DFA).
Indeed, both support precisely the same regex features with precisely the
same semantics.

Where as a `dense::DFA` must be completely built to handle any input before
it may be used for search, a lazy DFA starts off effectively empty. During
a search, a lazy DFA will build itself depending on whether it has already
computed the next transition or not. If it has, then it looks a lot like
a `dense::DFA` internally: it does a very fast table based access to find
the next transition. Otherwise, if the state hasn't been computed, then it
does determinization _for that specific transition_ to compute the next DFA
state.

The main selling point of a lazy DFA is that, in practice, it has
the performance profile of a `dense::DFA` without the weakness of it
taking worst case exponential time to build. Indeed, for each byte of
input, the lazy DFA will construct as most one new DFA state. Thus, a
lazy DFA achieves worst case `O(mn)` time for regex search (where `m ~
pattern.len()` and `n ~ haystack.len()`).

The main downsides of a lazy DFA are:

1. It requires mutable "cache" space during search. This is where the
transition table, among other things, is stored.
2. In pathological cases (e.g., if the cache is too small), it will run
out of room and either require a bigger cache capacity or will repeatedly
clear the cache and thus repeatedly regenerate DFA states. Overall, this
will tend to be slower than a typical NFA simulation.

# Capabilities

Like a `dense::DFA`, a single lazy DFA fundamentally supports the following
operations:

1. Detection of a match.
2. Location of the end of a match.
3. In the case of a lazy DFA with multiple patterns, which pattern matched
is reported as well.

A notable absence from the above list of capabilities is the location of
the *start* of a match. In order to provide both the start and end of
a match, *two* lazy DFAs are required. This functionality is provided by a
[`Regex`](crate::hybrid::regex::Regex).

# Example

This shows how to build a lazy DFA with the default configuration and
execute a search. Notice how, in contrast to a `dense::DFA`, we must create
a cache and pass it to our search routine.

```rust
use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let mut cache = dfa.create_cache();

let expected = Some(HalfMatch::must(0, 8));
assert_eq!(expected, dfa.try_search_fwd(
    &mut cache, &Input::new("foo12345"))?,
);
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn try_search_fwd(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>` — [`Cache`](../../../hybrid/dfa/index.md), [`Input`](../../../util/search/index.md), [`HalfMatch`](../../../util/search/index.md), [`MatchError`](../../../util/search/index.md)

- `fn try_search_rev(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>` — [`Cache`](../../../hybrid/dfa/index.md), [`Input`](../../../util/search/index.md), [`HalfMatch`](../../../util/search/index.md), [`MatchError`](../../../util/search/index.md)

- `fn try_search_overlapping_fwd(self: &Self, cache: &mut Cache, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>` — [`Cache`](../../../hybrid/dfa/index.md), [`Input`](../../../util/search/index.md), [`OverlappingState`](../../../hybrid/dfa/index.md), [`MatchError`](../../../util/search/index.md)

- `fn try_search_overlapping_rev(self: &Self, cache: &mut Cache, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>` — [`Cache`](../../../hybrid/dfa/index.md), [`Input`](../../../util/search/index.md), [`OverlappingState`](../../../hybrid/dfa/index.md), [`MatchError`](../../../util/search/index.md)

- `fn try_which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), MatchError>` — [`Cache`](../../../hybrid/dfa/index.md), [`Input`](../../../util/search/index.md), [`PatternSet`](../../../util/search/index.md), [`MatchError`](../../../util/search/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> DFA` — [`DFA`](../../../hybrid/dfa/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Cache`

```rust
struct Cache {
    trans: alloc::vec::Vec<crate::hybrid::id::LazyStateID>,
    starts: alloc::vec::Vec<crate::hybrid::id::LazyStateID>,
    states: alloc::vec::Vec<self::state::State>,
    states_to_id: std::collections::HashMap<self::state::State, crate::hybrid::id::LazyStateID>,
    sparses: crate::util::sparse_set::SparseSets,
    stack: alloc::vec::Vec<crate::util::primitives::StateID>,
    scratch_state_builder: self::state::StateBuilderEmpty,
    state_saver: StateSaver,
    memory_usage_state: usize,
    clear_count: usize,
    bytes_searched: usize,
    progress: Option<SearchProgress>,
}
```

A cache represents a partially computed DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`DFA`](../../meta/wrappers/index.md).

Caches can be created from their corresponding DFA via
`DFA::create_cache`. A cache can only be used with either the DFA that
created it, or the DFA that was most recently used to reset it with
`Cache::reset`. Using a cache with any other DFA may result in panics
or incorrect results.

#### Fields

- **`trans`**: `alloc::vec::Vec<crate::hybrid::id::LazyStateID>`

  The transition table.
  
  Given a `current` LazyStateID and an `input` byte, the next state can
  be computed via `trans[untagged(current) + equiv_class(input)]`. Notice
  that no multiplication is used. That's because state identifiers are
  "premultiplied."
  
  Note that the next state may be the "unknown" state. In this case, the
  next state is not known and determinization for `current` on `input`
  must be performed.

- **`starts`**: `alloc::vec::Vec<crate::hybrid::id::LazyStateID>`

  The starting states for this DFA.
  
  These are computed lazily. Initially, these are all set to "unknown"
  lazy state IDs.
  
  When 'starts_for_each_pattern' is disabled (the default), then the size
  of this is constrained to the possible starting configurations based
  on the search parameters. (At time of writing, that's 4.) However,
  when starting states for each pattern is enabled, then there are N
  additional groups of starting states, where each group reflects the
  different possible configurations and N is the number of patterns.

- **`states`**: `alloc::vec::Vec<self::state::State>`

  A sequence of NFA/DFA powerset states that have been computed for this
  lazy DFA. This sequence is indexable by untagged LazyStateIDs. (Every
  tagged LazyStateID can be used to index this sequence by converting it
  to its untagged form.)

- **`states_to_id`**: `std::collections::HashMap<self::state::State, crate::hybrid::id::LazyStateID>`

  A map from states to their corresponding IDs. This map may be accessed
  via the raw byte representation of a state, which means that a `State`
  does not need to be allocated to determine whether it already exists
  in this map. Indeed, the existence of such a state is what determines
  whether we allocate a new `State` or not.
  
  The higher level idea here is that we do just enough determinization
  for a state to check whether we've already computed it. If we have,
  then we can save a little (albeit not much) work. The real savings is
  in memory usage. If we never checked for trivially duplicate states,
  then our memory usage would explode to unreasonable levels.

- **`sparses`**: `crate::util::sparse_set::SparseSets`

  Sparse sets used to track which NFA states have been visited during
  various traversals.

- **`stack`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  Scratch space for traversing the NFA graph. (We use space on the heap
  instead of the call stack.)

- **`scratch_state_builder`**: `self::state::StateBuilderEmpty`

  Scratch space for building a NFA/DFA powerset state. This is used to
  help amortize allocation since not every powerset state generated is
  added to the cache. In particular, if it already exists in the cache,
  then there is no need to allocate a new `State` for it.

- **`state_saver`**: `StateSaver`

  A simple abstraction for handling the saving of at most a single state
  across a cache clearing. This is required for correctness. Namely, if
  adding a new state after clearing the cache fails, then the caller
  must retain the ability to continue using the state ID given. The
  state corresponding to the state ID is what we preserve across cache
  clearings.

- **`memory_usage_state`**: `usize`

  The memory usage, in bytes, used by 'states' and 'states_to_id'. We
  track this as new states are added since states use a variable amount
  of heap. Tracking this as we add states makes it possible to compute
  the total amount of memory used by the determinizer in constant time.

- **`clear_count`**: `usize`

  The number of times the cache has been cleared. When a minimum cache
  clear count is set, then the cache will return an error instead of
  clearing the cache if the count has been exceeded.

- **`bytes_searched`**: `usize`

  The total number of bytes searched since the last time this cache was
  cleared, not including the current search.
  
  This can be added to the length of the current search to get the true
  total number of bytes searched.
  
  This is generally only non-zero when the
  `Cache::search_{start,update,finish}` APIs are used to track search
  progress.

- **`progress`**: `Option<SearchProgress>`

  The progress of the current search.
  
  This is only non-`None` when callers utilize the `Cache::search_start`,
  `Cache::search_update` and `Cache::search_finish` APIs.
  
  The purpose of recording search progress is to be able to make a
  determination about the efficiency of the cache. Namely, by keeping
  track of the

#### Implementations

- `fn new(dfa: &DFA) -> Cache` — [`DFA`](../../../hybrid/dfa/index.md), [`Cache`](../../../hybrid/dfa/index.md)

- `fn reset(self: &mut Self, dfa: &DFA)` — [`DFA`](../../../hybrid/dfa/index.md)

- `fn search_start(self: &mut Self, at: usize)`

- `fn search_update(self: &mut Self, at: usize)`

- `fn search_finish(self: &mut Self, at: usize)`

- `fn search_total_len(self: &Self) -> usize`

- `fn clear_count(self: &Self) -> usize`

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Cache` — [`Cache`](../../../hybrid/dfa/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<crate::util::alphabet::ByteSet>,
    specialize_start_states: Option<bool>,
    cache_capacity: Option<usize>,
    skip_cache_capacity_check: Option<bool>,
    minimum_cache_clear_count: Option<Option<usize>>,
    minimum_bytes_per_state: Option<Option<usize>>,
}
```

The configuration used for building a lazy DFA.

As a convenience, `DFA::config` is an alias for `Config::new`. The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A lazy DFA configuration is a simple data object that is typically used
with `Builder::configure`.

The default configuration guarantees that a search will never return a
"gave up" or "quit" error, although it is possible for a search to fail
if `Config::starts_for_each_pattern` wasn't enabled (which it is not by
default) and an `Anchored::Pattern` mode is requested via [`Input`](../../util/search/index.md).

#### Implementations

- `fn new() -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn match_kind(self: Self, kind: MatchKind) -> Config` — [`MatchKind`](../../../util/search/index.md), [`Config`](../../../hybrid/dfa/index.md)

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../../util/prefilter/index.md), [`Config`](../../../hybrid/dfa/index.md)

- `fn starts_for_each_pattern(self: Self, yes: bool) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn byte_classes(self: Self, yes: bool) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn unicode_word_boundary(self: Self, yes: bool) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn quit(self: Self, byte: u8, yes: bool) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn specialize_start_states(self: Self, yes: bool) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn cache_capacity(self: Self, bytes: usize) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn skip_cache_capacity_check(self: Self, yes: bool) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn minimum_cache_clear_count(self: Self, min: Option<usize>) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn minimum_bytes_per_state(self: Self, min: Option<usize>) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

- `fn get_match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../../../util/search/index.md)

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md)

- `fn get_starts_for_each_pattern(self: &Self) -> bool`

- `fn get_byte_classes(self: &Self) -> bool`

- `fn get_unicode_word_boundary(self: &Self) -> bool`

- `fn get_quit(self: &Self, byte: u8) -> bool`

- `fn get_specialize_start_states(self: &Self) -> bool`

- `fn get_cache_capacity(self: &Self) -> usize`

- `fn get_skip_cache_capacity_check(self: &Self) -> bool`

- `fn get_minimum_cache_clear_count(self: &Self) -> Option<usize>`

- `fn get_minimum_bytes_per_state(self: &Self) -> Option<usize>`

- `fn get_minimum_cache_capacity(self: &Self, nfa: &thompson::NFA) -> Result<usize, BuildError>` — [`NFA`](../../../nfa/thompson/nfa/index.md), [`BuildError`](../../../hybrid/error/index.md)

- `fn byte_classes_from_nfa(self: &Self, nfa: &thompson::NFA, quit: &ByteSet) -> ByteClasses` — [`NFA`](../../../nfa/thompson/nfa/index.md), [`ByteSet`](../../../util/alphabet/index.md), [`ByteClasses`](../../../util/alphabet/index.md)

- `fn quit_set_from_nfa(self: &Self, nfa: &thompson::NFA) -> Result<ByteSet, BuildError>` — [`NFA`](../../../nfa/thompson/nfa/index.md), [`ByteSet`](../../../util/alphabet/index.md), [`BuildError`](../../../hybrid/error/index.md)

- `fn overwrite(self: &Self, o: Config) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Config` — [`Config`](../../../hybrid/dfa/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Config` — [`Config`](../../../hybrid/dfa/index.md)

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

A builder for constructing a lazy deterministic finite automaton from
regular expressions.

As a convenience, `DFA::builder` is an alias for `Builder::new`. The
advantage of the former is that it often lets you avoid importing the
`Builder` type directly.

This builder provides two main things:

1. It provides a few different `build` routines for actually constructing
a DFA from different kinds of inputs. The most convenient is
`Builder::build`, which builds a DFA directly from a pattern string. The
most flexible is `Builder::build_from_nfa`, which builds a DFA straight
from an NFA.
2. The builder permits configuring a number of things.
`Builder::configure` is used with [`Config`](../../dfa/onepass/index.md) to configure aspects of
the DFA and the construction process itself. `Builder::syntax` and
`Builder::thompson` permit configuring the regex parser and Thompson NFA
construction, respectively. The syntax and thompson configurations only
apply when building from a pattern string.

This builder always constructs a *single* lazy DFA. As such, this builder
can only be used to construct regexes that either detect the presence
of a match or find the end location of a match. A single DFA cannot
produce both the start and end of a match. For that information, use a
[`Regex`](crate::hybrid::regex::Regex), which can be similarly configured
using [`regex::Builder`](crate::hybrid::regex::Builder). The main reason
to use a DFA directly is if the end location of a match is enough for your
use case. Namely, a `Regex` will construct two lazy DFAs instead of one,
since a second reverse DFA is needed to find the start of a match.

# Example

This example shows how to build a lazy DFA that uses a tiny cache capacity
and completely disables Unicode. That is:

* Things such as `\w`, `.` and `\b` are no longer Unicode-aware. `\w`
  and `\b` are ASCII-only while `.` matches any byte except for `\n`
  (instead of any UTF-8 encoding of a Unicode scalar value except for
  `\n`). Things that are Unicode only, such as `\pL`, are not allowed.
* The pattern itself is permitted to match invalid UTF-8. For example,
  things like `[^a]` that match any byte except for `a` are permitted.

```rust
use regex_automata::{
    hybrid::dfa::DFA,
    nfa::thompson,
    util::syntax,
    HalfMatch, Input,
};

let dfa = DFA::builder()
    .configure(DFA::config().cache_capacity(5_000))
    .thompson(thompson::Config::new().utf8(false))
    .syntax(syntax::Config::new().unicode(false).utf8(false))
    .build(r"foo[^b]ar.*")?;
let mut cache = dfa.create_cache();

let haystack = b"\xFEfoo\xFFar\xE2\x98\xFF\n";
let expected = Some(HalfMatch::must(0, 10));
let got = dfa.try_search_fwd(&mut cache, &Input::new(haystack))?;
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder` — [`Builder`](../../../hybrid/dfa/index.md)

- `fn build(self: &Self, pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](../../../hybrid/dfa/index.md), [`BuildError`](../../../hybrid/error/index.md)

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](../../../hybrid/dfa/index.md), [`BuildError`](../../../hybrid/error/index.md)

- `fn build_from_nfa(self: &Self, nfa: thompson::NFA) -> Result<DFA, BuildError>` — [`NFA`](../../../nfa/thompson/nfa/index.md), [`DFA`](../../../hybrid/dfa/index.md), [`BuildError`](../../../hybrid/error/index.md)

- `fn configure(self: &mut Self, config: Config) -> &mut Builder` — [`Config`](../../../hybrid/dfa/index.md), [`Builder`](../../../hybrid/dfa/index.md)

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../../util/syntax/index.md), [`Builder`](../../../hybrid/dfa/index.md)

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder` — [`Config`](../../../nfa/thompson/compiler/index.md), [`Builder`](../../../hybrid/dfa/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Builder` — [`Builder`](../../../hybrid/dfa/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OverlappingState`

```rust
struct OverlappingState {
    mat: Option<crate::util::search::HalfMatch>,
    id: Option<crate::hybrid::id::LazyStateID>,
    at: usize,
    next_match_index: Option<usize>,
    rev_eoi: bool,
}
```

Represents the current state of an overlapping search.

This is used for overlapping searches since they need to know something
about the previous search. For example, when multiple patterns match at the
same position, this state tracks the last reported pattern so that the next
search knows whether to report another matching pattern or continue with
the search at the next position. Additionally, it also tracks which state
the last search call terminated in.

This type provides little introspection capabilities. The only thing a
caller can do is construct it and pass it around to permit search routines
to use it to track state, and also ask whether a match has been found.

Callers should always provide a fresh state constructed via
`OverlappingState::start` when starting a new search. Reusing state from
a previous search may result in incorrect results.

#### Fields

- **`mat`**: `Option<crate::util::search::HalfMatch>`

  The match reported by the most recent overlapping search to use this
  state.
  
  If a search does not find any matches, then it is expected to clear
  this value.

- **`id`**: `Option<crate::hybrid::id::LazyStateID>`

  The state ID of the state at which the search was in when the call
  terminated. When this is a match state, `last_match` must be set to a
  non-None value.
  
  A `None` value indicates the start state of the corresponding
  automaton. We cannot use the actual ID, since any one automaton may
  have many start states, and which one is in use depends on several
  search-time factors.

- **`at`**: `usize`

  The position of the search.
  
  When `id` is None (i.e., we are starting a search), this is set to
  the beginning of the search as given by the caller regardless of its
  current value. Subsequent calls to an overlapping search pick up at
  this offset.

- **`next_match_index`**: `Option<usize>`

  The index into the matching patterns of the next match to report if the
  current state is a match state. Note that this may be 1 greater than
  the total number of matches to report for the current match state. (In
  which case, no more matches should be reported at the current position
  and the search should advance to the next position.)

- **`rev_eoi`**: `bool`

  This is set to true when a reverse overlapping search has entered its
  EOI transitions.
  
  This isn't used in a forward search because it knows to stop once the
  position exceeds the end of the search range. In a reverse search,
  since we use unsigned offsets, we don't "know" once we've gone past
  `0`. So the only way to detect it is with this extra flag. The reverse
  overlapping search knows to terminate specifically after it has
  reported all matches after following the EOI transition.

#### Implementations

- `fn start() -> OverlappingState` — [`OverlappingState`](../../../hybrid/dfa/index.md)

- `fn get_match(self: &Self) -> Option<HalfMatch>` — [`HalfMatch`](../../../util/search/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> OverlappingState` — [`OverlappingState`](../../../hybrid/dfa/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &OverlappingState) -> bool` — [`OverlappingState`](../../../hybrid/dfa/index.md)

##### `impl StructuralPartialEq`

