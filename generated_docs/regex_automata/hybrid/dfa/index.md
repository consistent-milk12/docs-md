*[regex_automata](../../index.md) / [hybrid](../index.md) / [dfa](index.md)*

---

# Module `dfa`

Types and routines specific to lazy DFAs.

This module is the home of [`hybrid::dfa::DFA`](DFA).

This module also contains a [`hybrid::dfa::Builder`](Builder) and a
[`hybrid::dfa::Config`](Config) for configuring and building a lazy DFA.

## Contents

- [Structs](#structs)
  - [`DFA`](#dfa)
  - [`Cache`](#cache)
  - [`SearchProgress`](#searchprogress)
  - [`Lazy`](#lazy)
  - [`LazyRef`](#lazyref)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`OverlappingState`](#overlappingstate)
- [Enums](#enums)
  - [`StateSaver`](#statesaver)
- [Functions](#functions)
  - [`skip_empty_utf8_splits_overlapping`](#skip_empty_utf8_splits_overlapping)
  - [`minimum_lazy_state_id`](#minimum_lazy_state_id)
  - [`minimum_cache_capacity`](#minimum_cache_capacity)
- [Type Aliases](#type-aliases)
  - [`StateMap`](#statemap)
- [Constants](#constants)
  - [`MIN_STATES`](#min_states)
  - [`SENTINEL_STATES`](#sentinel_states)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DFA`](#dfa) | struct | A hybrid NFA/DFA (also called a "lazy DFA") for regex searching. |
| [`Cache`](#cache) | struct | A cache represents a partially computed DFA. |
| [`SearchProgress`](#searchprogress) | struct | Keeps track of the progress of the current search. |
| [`Lazy`](#lazy) | struct | A type that groups methods that require the base NFA/DFA and writable access to the cache. |
| [`LazyRef`](#lazyref) | struct | A type that groups methods that require the base NFA/DFA and read-only access to the cache. |
| [`Config`](#config) | struct | The configuration used for building a lazy DFA. |
| [`Builder`](#builder) | struct | A builder for constructing a lazy deterministic finite automaton from regular expressions. |
| [`OverlappingState`](#overlappingstate) | struct | Represents the current state of an overlapping search. |
| [`StateSaver`](#statesaver) | enum | A simple type that encapsulates the saving of a state ID through a cache clearing. |
| [`skip_empty_utf8_splits_overlapping`](#skip_empty_utf8_splits_overlapping) | fn | Runs the given overlapping `search` function (forwards or backwards) until a match is found whose offset does not split a codepoint. |
| [`minimum_lazy_state_id`](#minimum_lazy_state_id) | fn | Based on the minimum number of states required for a useful lazy DFA cache, this returns the minimum lazy state ID that must be representable. |
| [`minimum_cache_capacity`](#minimum_cache_capacity) | fn | Based on the minimum number of states required for a useful lazy DFA cache, this returns a heuristic minimum number of bytes of heap space required. |
| [`StateMap`](#statemap) | type | A map from states to state identifiers. |
| [`MIN_STATES`](#min_states) | const | The minimum number of states that a lazy DFA's cache size must support. |
| [`SENTINEL_STATES`](#sentinel_states) | const | The number of "sentinel" states that get added to every lazy DFA. |

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:118-126`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L118-L126)*

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

- <span id="dfa-new"></span>`fn new(pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="dfa-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="dfa-always-match"></span>`fn always_match() -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="dfa-never-match"></span>`fn never_match() -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="dfa-config"></span>`fn config() -> Config` — [`Config`](#config)

- <span id="dfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

- <span id="dfa-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

- <span id="dfa-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

- <span id="dfa-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="dfa-byte-classes"></span>`fn byte_classes(&self) -> &ByteClasses` — [`ByteClasses`](../../util/alphabet/index.md)

- <span id="dfa-get-config"></span>`fn get_config(&self) -> &Config` — [`Config`](#config)

- <span id="dfa-get-nfa"></span>`fn get_nfa(&self) -> &thompson::NFA` — [`NFA`](../../nfa/thompson/nfa/index.md)

- <span id="dfa-stride2"></span>`fn stride2(&self) -> usize`

- <span id="dfa-stride"></span>`fn stride(&self) -> usize`

- <span id="dfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for DFA`

- <span id="dfa-clone"></span>`fn clone(&self) -> DFA` — [`DFA`](#dfa)

##### `impl Debug for DFA`

- <span id="dfa-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:1777-1867`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L1777-L1867)*

A cache represents a partially computed DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`DFA`](#dfa).

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

- <span id="cache-new"></span>`fn new(dfa: &DFA) -> Cache` — [`DFA`](#dfa), [`Cache`](#cache)

- <span id="cache-reset"></span>`fn reset(&mut self, dfa: &DFA)` — [`DFA`](#dfa)

- <span id="cache-search-start"></span>`fn search_start(&mut self, at: usize)`

- <span id="cache-search-update"></span>`fn search_update(&mut self, at: usize)`

- <span id="cache-search-finish"></span>`fn search_finish(&mut self, at: usize)`

- <span id="cache-search-total-len"></span>`fn search_total_len(&self) -> usize`

- <span id="cache-clear-count"></span>`fn clear_count(&self) -> usize`

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SearchProgress`

```rust
struct SearchProgress {
    start: usize,
    at: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2048-2051`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L2048-L2051)*

Keeps track of the progress of the current search.

This is updated via the `Cache::search_{start,update,finish}` APIs to
record how many bytes have been searched. This permits computing a
heuristic that represents the efficiency of a cache, and thus helps inform
whether the lazy DFA should give up or not.

#### Implementations

- <span id="searchprogress-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for SearchProgress`

- <span id="searchprogress-clone"></span>`fn clone(&self) -> SearchProgress` — [`SearchProgress`](#searchprogress)

##### `impl Debug for SearchProgress`

- <span id="searchprogress-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Lazy<'i, 'c>`

```rust
struct Lazy<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c mut Cache,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2081-2084`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L2081-L2084)*

A type that groups methods that require the base NFA/DFA and writable
access to the cache.

#### Implementations

- <span id="lazy-new"></span>`fn new(dfa: &'i DFA, cache: &'c mut Cache) -> Lazy<'i, 'c>` — [`DFA`](#dfa), [`Cache`](#cache), [`Lazy`](#lazy)

- <span id="lazy-as-ref"></span>`fn as_ref<'a>(self: &'a Self) -> LazyRef<'i, 'a>` — [`LazyRef`](#lazyref)

- <span id="lazy-cache-next-state"></span>`fn cache_next_state(&mut self, current: LazyStateID, unit: alphabet::Unit) -> Result<LazyStateID, CacheError>` — [`LazyStateID`](../id/index.md), [`Unit`](../../util/alphabet/index.md), [`CacheError`](../error/index.md)

- <span id="lazy-cache-start-group"></span>`fn cache_start_group(&mut self, anchored: Anchored, start: Start) -> Result<LazyStateID, StartError>` — [`Anchored`](../../index.md), [`Start`](../../util/start/index.md), [`LazyStateID`](../id/index.md), [`StartError`](../error/index.md)

- <span id="lazy-cache-start-one"></span>`fn cache_start_one(&mut self, nfa_start_id: NFAStateID, start: Start) -> Result<LazyStateID, CacheError>` — [`StateID`](../../util/primitives/index.md), [`Start`](../../util/start/index.md), [`LazyStateID`](../id/index.md), [`CacheError`](../error/index.md)

- <span id="lazy-add-builder-state"></span>`fn add_builder_state(&mut self, builder: StateBuilderNFA, idmap: impl Fn(LazyStateID) -> LazyStateID) -> Result<LazyStateID, CacheError>` — [`StateBuilderNFA`](../../util/determinize/state/index.md), [`LazyStateID`](../id/index.md), [`CacheError`](../error/index.md)

- <span id="lazy-add-state"></span>`fn add_state(&mut self, state: State, idmap: impl Fn(LazyStateID) -> LazyStateID) -> Result<LazyStateID, CacheError>` — [`State`](../../util/determinize/state/index.md), [`LazyStateID`](../id/index.md), [`CacheError`](../error/index.md)

- <span id="lazy-next-state-id"></span>`fn next_state_id(&mut self) -> Result<LazyStateID, CacheError>` — [`LazyStateID`](../id/index.md), [`CacheError`](../error/index.md)

- <span id="lazy-try-clear-cache"></span>`fn try_clear_cache(&mut self) -> Result<(), CacheError>` — [`CacheError`](../error/index.md)

- <span id="lazy-reset-cache"></span>`fn reset_cache(&mut self)`

- <span id="lazy-clear-cache"></span>`fn clear_cache(&mut self)`

- <span id="lazy-init-cache"></span>`fn init_cache(&mut self)`

- <span id="lazy-save-state"></span>`fn save_state(&mut self, id: LazyStateID)` — [`LazyStateID`](../id/index.md)

- <span id="lazy-saved-state-id"></span>`fn saved_state_id(&mut self) -> LazyStateID` — [`LazyStateID`](../id/index.md)

- <span id="lazy-set-all-transitions"></span>`fn set_all_transitions(&mut self, from: LazyStateID, to: LazyStateID)` — [`LazyStateID`](../id/index.md)

- <span id="lazy-set-transition"></span>`fn set_transition(&mut self, from: LazyStateID, unit: alphabet::Unit, to: LazyStateID)` — [`LazyStateID`](../id/index.md), [`Unit`](../../util/alphabet/index.md)

- <span id="lazy-set-start-state"></span>`fn set_start_state(&mut self, anchored: Anchored, start: Start, id: LazyStateID)` — [`Anchored`](../../index.md), [`Start`](../../util/start/index.md), [`LazyStateID`](../id/index.md)

- <span id="lazy-get-state-builder"></span>`fn get_state_builder(&mut self) -> StateBuilderEmpty` — [`StateBuilderEmpty`](../../util/determinize/state/index.md)

- <span id="lazy-put-state-builder"></span>`fn put_state_builder(&mut self, builder: StateBuilderNFA)` — [`StateBuilderNFA`](../../util/determinize/state/index.md)

#### Trait Implementations

##### `impl Debug for Lazy<'i, 'c>`

- <span id="lazy-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `LazyRef<'i, 'c>`

```rust
struct LazyRef<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c Cache,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2677-2680`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L2677-L2680)*

A type that groups methods that require the base NFA/DFA and read-only
access to the cache.

#### Implementations

- <span id="lazyref-new"></span>`fn new(dfa: &'i DFA, cache: &'c Cache) -> LazyRef<'i, 'c>` — [`DFA`](#dfa), [`Cache`](#cache), [`LazyRef`](#lazyref)

- <span id="lazyref-get-cached-start-id"></span>`fn get_cached_start_id(&self, anchored: Anchored, start: Start) -> Result<LazyStateID, StartError>` — [`Anchored`](../../index.md), [`Start`](../../util/start/index.md), [`LazyStateID`](../id/index.md), [`StartError`](../error/index.md)

- <span id="lazyref-get-cached-state"></span>`fn get_cached_state(&self, sid: LazyStateID) -> &State` — [`LazyStateID`](../id/index.md), [`State`](../../util/determinize/state/index.md)

- <span id="lazyref-is-sentinel"></span>`fn is_sentinel(&self, id: LazyStateID) -> bool` — [`LazyStateID`](../id/index.md)

- <span id="lazyref-unknown-id"></span>`fn unknown_id(&self) -> LazyStateID` — [`LazyStateID`](../id/index.md)

- <span id="lazyref-dead-id"></span>`fn dead_id(&self) -> LazyStateID` — [`LazyStateID`](../id/index.md)

- <span id="lazyref-quit-id"></span>`fn quit_id(&self) -> LazyStateID` — [`LazyStateID`](../id/index.md)

- <span id="lazyref-is-valid"></span>`fn is_valid(&self, id: LazyStateID) -> bool` — [`LazyStateID`](../id/index.md)

- <span id="lazyref-state-fits-in-cache"></span>`fn state_fits_in_cache(&self, state: &State) -> bool` — [`State`](../../util/determinize/state/index.md)

- <span id="lazyref-state-builder-fits-in-cache"></span>`fn state_builder_fits_in_cache(&self, state: &StateBuilderNFA) -> bool` — [`StateBuilderNFA`](../../util/determinize/state/index.md)

- <span id="lazyref-memory-usage-for-one-more-state"></span>`fn memory_usage_for_one_more_state(&self, state_heap_size: usize) -> usize`

#### Trait Implementations

##### `impl Debug for LazyRef<'i, 'c>`

- <span id="lazyref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2880-2899`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L2880-L2899)*

The configuration used for building a lazy DFA.

As a convenience, `DFA::config` is an alias for `Config::new`. The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A lazy DFA configuration is a simple data object that is typically used
with `Builder::configure`.

The default configuration guarantees that a search will never return a
"gave up" or "quit" error, although it is possible for a search to fail
if `Config::starts_for_each_pattern` wasn't enabled (which it is not by
default) and an [`Anchored::Pattern`](../../index.md) mode is requested via [`Input`](../../index.md).

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md), [`Config`](#config)

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../util/prefilter/index.md), [`Config`](#config)

- <span id="config-starts-for-each-pattern"></span>`fn starts_for_each_pattern(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-unicode-word-boundary"></span>`fn unicode_word_boundary(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-quit"></span>`fn quit(self, byte: u8, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-specialize-start-states"></span>`fn specialize_start_states(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-cache-capacity"></span>`fn cache_capacity(self, bytes: usize) -> Config` — [`Config`](#config)

- <span id="config-skip-cache-capacity-check"></span>`fn skip_cache_capacity_check(self, yes: bool) -> Config` — [`Config`](#config)

- <span id="config-minimum-cache-clear-count"></span>`fn minimum_cache_clear_count(self, min: Option<usize>) -> Config` — [`Config`](#config)

- <span id="config-minimum-bytes-per-state"></span>`fn minimum_bytes_per_state(self, min: Option<usize>) -> Config` — [`Config`](#config)

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../index.md)

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md)

- <span id="config-get-starts-for-each-pattern"></span>`fn get_starts_for_each_pattern(&self) -> bool`

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

- <span id="config-get-unicode-word-boundary"></span>`fn get_unicode_word_boundary(&self) -> bool`

- <span id="config-get-quit"></span>`fn get_quit(&self, byte: u8) -> bool`

- <span id="config-get-specialize-start-states"></span>`fn get_specialize_start_states(&self) -> bool`

- <span id="config-get-cache-capacity"></span>`fn get_cache_capacity(&self) -> usize`

- <span id="config-get-skip-cache-capacity-check"></span>`fn get_skip_cache_capacity_check(&self) -> bool`

- <span id="config-get-minimum-cache-clear-count"></span>`fn get_minimum_cache_clear_count(&self) -> Option<usize>`

- <span id="config-get-minimum-bytes-per-state"></span>`fn get_minimum_bytes_per_state(&self) -> Option<usize>`

- <span id="config-get-minimum-cache-capacity"></span>`fn get_minimum_cache_capacity(&self, nfa: &thompson::NFA) -> Result<usize, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md), [`BuildError`](../error/index.md)

- <span id="config-byte-classes-from-nfa"></span>`fn byte_classes_from_nfa(&self, nfa: &thompson::NFA, quit: &ByteSet) -> ByteClasses` — [`NFA`](../../nfa/thompson/nfa/index.md), [`ByteSet`](../../util/alphabet/index.md), [`ByteClasses`](../../util/alphabet/index.md)

- <span id="config-quit-set-from-nfa"></span>`fn quit_set_from_nfa(&self, nfa: &thompson::NFA) -> Result<ByteSet, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md), [`ByteSet`](../../util/alphabet/index.md), [`BuildError`](../error/index.md)

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:3991-3995`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L3991-L3995)*

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
`Builder::configure` is used with [`Config`](#config) to configure aspects of
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

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: thompson::NFA) -> Result<DFA, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md), [`DFA`](#dfa), [`BuildError`](../error/index.md)

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md), [`Builder`](#builder)

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4188-4227`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L4188-L4227)*

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

- <span id="overlappingstate-start"></span>`fn start() -> OverlappingState` — [`OverlappingState`](#overlappingstate)

- <span id="overlappingstate-get-match"></span>`fn get_match(&self) -> Option<HalfMatch>` — [`HalfMatch`](../../index.md)

#### Trait Implementations

##### `impl Clone for OverlappingState`

- <span id="overlappingstate-clone"></span>`fn clone(&self) -> OverlappingState` — [`OverlappingState`](#overlappingstate)

##### `impl Debug for OverlappingState`

- <span id="overlappingstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OverlappingState`

##### `impl PartialEq for OverlappingState`

- <span id="overlappingstate-eq"></span>`fn eq(&self, other: &OverlappingState) -> bool` — [`OverlappingState`](#overlappingstate)

##### `impl StructuralPartialEq for OverlappingState`

## Enums

### `StateSaver`

```rust
enum StateSaver {
    None,
    ToSave {
        id: crate::hybrid::id::LazyStateID,
        state: self::state::State,
    },
    Saved(crate::hybrid::id::LazyStateID),
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2819-2832`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L2819-L2832)*

A simple type that encapsulates the saving of a state ID through a cache
clearing.

A state ID can be marked for saving with ToSave, while a state ID can be
saved itself with Saved.

#### Variants

- **`None`**

  An empty state saver. In this case, no states (other than the special
  sentinel states) are preserved after clearing the cache.

- **`ToSave`**

  An ID of a state (and the state itself) that should be preserved after
  the lazy DFA's cache has been cleared. After clearing, the updated ID
  is stored in 'Saved' since it may have changed.

- **`Saved`**

  An ID that of a state that has been persisted through a lazy DFA
  cache clearing. The ID recorded here corresponds to an ID that was
  once marked as ToSave. The IDs are likely not equivalent even though
  the states they point to are.

#### Implementations

- <span id="statesaver-none"></span>`fn none() -> StateSaver` — [`StateSaver`](#statesaver)

- <span id="statesaver-take-to-save"></span>`fn take_to_save(&mut self) -> Option<(LazyStateID, State)>` — [`LazyStateID`](../id/index.md), [`State`](../../util/determinize/state/index.md)

- <span id="statesaver-take-saved"></span>`fn take_saved(&mut self) -> Option<LazyStateID>` — [`LazyStateID`](../id/index.md)

#### Trait Implementations

##### `impl Clone for StateSaver`

- <span id="statesaver-clone"></span>`fn clone(&self) -> StateSaver` — [`StateSaver`](#statesaver)

##### `impl Debug for StateSaver`

- <span id="statesaver-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `skip_empty_utf8_splits_overlapping`

```rust
fn skip_empty_utf8_splits_overlapping<F>(input: &crate::util::search::Input<'_>, state: &mut OverlappingState, search: F) -> Result<(), crate::util::search::MatchError>
where
    F: FnMut(&crate::util::search::Input<'_>, &mut OverlappingState) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4261-4293`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L4261-L4293)*

Runs the given overlapping `search` function (forwards or backwards) until
a match is found whose offset does not split a codepoint.

This is *not* always correct to call. It should only be called when the
underlying NFA has UTF-8 mode enabled *and* it can produce zero-width
matches. Calling this when both of those things aren't true might result
in legitimate matches getting skipped.

### `minimum_lazy_state_id`

```rust
fn minimum_lazy_state_id(classes: &crate::util::alphabet::ByteClasses) -> Result<crate::hybrid::id::LazyStateID, crate::hybrid::id::LazyStateIDError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4301-4307`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L4301-L4307)*

Based on the minimum number of states required for a useful lazy DFA cache,
this returns the minimum lazy state ID that must be representable.

It's not likely for this to have any impact 32-bit systems (or higher), but
on 16-bit systems, the lazy state ID space is quite constrained and thus
may be insufficient if our MIN_STATES value is (for some reason) too high.

### `minimum_cache_capacity`

```rust
fn minimum_cache_capacity(nfa: &thompson::NFA, classes: &crate::util::alphabet::ByteClasses, starts_for_each_pattern: bool) -> usize
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4335-4392`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L4335-L4392)*

Based on the minimum number of states required for a useful lazy DFA cache,
this returns a heuristic minimum number of bytes of heap space required.

This is a "heuristic" because the minimum it returns is likely bigger than
the true minimum. Namely, it assumes that each powerset NFA/DFA state uses
the maximum number of NFA states (all of them). This is likely bigger
than what is required in practice. Computing the true minimum effectively
requires determinization, which is probably too much work to do for a
simple check like this.

One of the issues with this approach IMO is that it requires that this
be in sync with the calculation above for computing how much heap memory
the DFA cache uses. If we get it wrong, it's possible for example for the
minimum to be smaller than the computed heap memory, and thus, it may be
the case that we can't add the required minimum number of states. That in
turn will make lazy DFA panic because we assume that we can add at least a
minimum number of states.

Another approach would be to always allow the minimum number of states to
be added to the lazy DFA cache, even if it exceeds the configured cache
limit. This does mean that the limit isn't really a limit in all cases,
which is unfortunate. But it does at least guarantee that the lazy DFA can
always make progress, even if it is slow. (This approach is very similar to
enabling the 'skip_cache_capacity_check' config knob, except it wouldn't
rely on cache size calculation. Instead, it would just always permit a
minimum number of states to be added.)

## Type Aliases

### `StateMap`

```rust
type StateMap = std::collections::HashMap<self::state::State, crate::hybrid::id::LazyStateID>;
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2074`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L2074)*

A map from states to state identifiers. When using std, we use a standard
hashmap, since it's a bit faster for this use case. (Other maps, like
one's based on FNV, have not yet been benchmarked.)

The main purpose of this map is to reuse states where possible. This won't
fully minimize the DFA, but it works well in a lot of cases.

## Constants

### `MIN_STATES`
```rust
const MIN_STATES: usize = 5usize;
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:42`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L42)*

The minimum number of states that a lazy DFA's cache size must support.

This is checked at time of construction to ensure that at least some small
number of states can fit in the given capacity allotment. If we can't fit
at least this number of states, then the thinking is that it's pretty
senseless to use the lazy DFA. More to the point, parts of the code do
assume that the cache can fit at least some small number of states.

### `SENTINEL_STATES`
```rust
const SENTINEL_STATES: usize = 3usize;
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:50`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/dfa.rs#L50)*

The number of "sentinel" states that get added to every lazy DFA.

These are special states indicating status conditions of a search: unknown,
dead and quit. These states in particular also use zero NFA states, so
their memory usage is quite small. This is relevant for computing the
minimum memory needed for a lazy DFA cache.

