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
  - [`skip_empty_utf8_splits_overlapping`](#skip-empty-utf8-splits-overlapping)
  - [`minimum_lazy_state_id`](#minimum-lazy-state-id)
  - [`minimum_cache_capacity`](#minimum-cache-capacity)
- [Type Aliases](#type-aliases)
  - [`StateMap`](#statemap)
- [Constants](#constants)
  - [`MIN_STATES`](#min-states)
  - [`SENTINEL_STATES`](#sentinel-states)

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
| [`skip_empty_utf8_splits_overlapping`](#skip-empty-utf8-splits-overlapping) | fn | Runs the given overlapping `search` function (forwards or backwards) until a match is found whose offset does not split a codepoint. |
| [`minimum_lazy_state_id`](#minimum-lazy-state-id) | fn | Based on the minimum number of states required for a useful lazy DFA cache, this returns the minimum lazy state ID that must be representable. |
| [`minimum_cache_capacity`](#minimum-cache-capacity) | fn | Based on the minimum number of states required for a useful lazy DFA cache, this returns a heuristic minimum number of bytes of heap space required. |
| [`StateMap`](#statemap) | type | A map from states to state identifiers. |
| [`MIN_STATES`](#min-states) | const | The minimum number of states that a lazy DFA's cache size must support. |
| [`SENTINEL_STATES`](#sentinel-states) | const | The number of "sentinel" states that get added to every lazy DFA. |

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:118-126`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L118-L126)*

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

- <span id="dfa-new"></span>`fn new(pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Parse the given regular expression using a default configuration and

  return the corresponding lazy DFA.

  

  If you want a non-default configuration, then use the [`Builder`](#builder) to

  set your own configuration.

  

  # Example

  

  ```rust

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let dfa = DFA::new("foo[0-9]+bar")?;

  let mut cache = dfa.create_cache();

  

  let expected = HalfMatch::must(0, 11);

  assert_eq!(

      Some(expected),

      dfa.try_search_fwd(&mut cache, &Input::new("foo12345bar"))?,

  );

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Parse the given regular expressions using a default configuration and

  return the corresponding lazy multi-DFA.

  

  If you want a non-default configuration, then use the [`Builder`](#builder) to

  set your own configuration.

  

  # Example

  

  ```rust

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let dfa = DFA::new_many(&["[0-9]+", "[a-z]+"])?;

  let mut cache = dfa.create_cache();

  

  let expected = HalfMatch::must(1, 3);

  assert_eq!(

      Some(expected),

      dfa.try_search_fwd(&mut cache, &Input::new("foo12345bar"))?,

  );

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-always-match"></span>`fn always_match() -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Create a new lazy DFA that matches every input.

  

  # Example

  

  ```rust

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let dfa = DFA::always_match()?;

  let mut cache = dfa.create_cache();

  

  let expected = HalfMatch::must(0, 0);

  assert_eq!(Some(expected), dfa.try_search_fwd(

      &mut cache, &Input::new(""))?,

  );

  assert_eq!(Some(expected), dfa.try_search_fwd(

      &mut cache, &Input::new("foo"))?,

  );

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-never-match"></span>`fn never_match() -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Create a new lazy DFA that never matches any input.

  

  # Example

  

  ```rust

  use regex_automata::{hybrid::dfa::DFA, Input};

  

  let dfa = DFA::never_match()?;

  let mut cache = dfa.create_cache();

  

  assert_eq!(None, dfa.try_search_fwd(&mut cache, &Input::new(""))?);

  assert_eq!(None, dfa.try_search_fwd(&mut cache, &Input::new("foo"))?);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-config"></span>`fn config() -> Config` — [`Config`](#config)

  Return a default configuration for a `DFA`.

  

  This is a convenience routine to avoid needing to import the [`Config`](#config)

  type when customizing the construction of a lazy DFA.

  

  # Example

  

  This example shows how to build a lazy DFA that heuristically supports

  Unicode word boundaries.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, MatchError, Input};

  

  let re = DFA::builder()

      .configure(DFA::config().unicode_word_boundary(true))

      .build(r"\b\w+\b")?;

  let mut cache = re.create_cache();

  

  // Since our haystack is all ASCII, the DFA search sees then and knows

  // it is legal to interpret Unicode word boundaries as ASCII word

  // boundaries.

  let input = Input::new("!!foo!!");

  let expected = HalfMatch::must(0, 5);

  assert_eq!(Some(expected), re.try_search_fwd(&mut cache, &input)?);

  

  // But if our haystack contains non-ASCII, then the search will fail

  // with an error.

  let input = Input::new("!!βββ!!");

  let expected = MatchError::quit(b'\xCE', 2);

  assert_eq!(Err(expected), re.try_search_fwd(&mut cache, &input));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  Return a builder for configuring the construction of a `Regex`.

  

  This is a convenience routine to avoid needing to import the

  [`Builder`](#builder) type in common cases.

  

  # Example

  

  This example shows how to use the builder to disable UTF-8 mode

  everywhere for lazy DFAs.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, util::syntax, HalfMatch, Input};

  

  let re = DFA::builder()

      .syntax(syntax::Config::new().utf8(false))

      .build(r"foo(?-u:[^b])ar.*")?;

  let mut cache = re.create_cache();

  

  let input = Input::new(b"\xFEfoo\xFFarzz\xE2\x98\xFF\n");

  let expected = Some(HalfMatch::must(0, 9));

  let got = re.try_search_fwd(&mut cache, &input)?;

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

  Create a new cache for this lazy DFA.

  

  The cache returned should only be used for searches for this

  lazy DFA. If you want to reuse the cache for another DFA, then

  you must call `Cache::reset` with that DFA (or, equivalently,

  `DFA::reset_cache`).

- <span id="dfa-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

  Reset the given cache such that it can be used for searching with the

  this lazy DFA (and only this DFA).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different lazy DFA.

  

  Resetting a cache sets its "clear count" to 0. This is relevant if the

  lazy DFA has been configured to "give up" after it has cleared the

  cache a certain number of times.

  

  Any lazy state ID generated by the cache prior to resetting it is

  invalid after the reset.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different DFA.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let dfa1 = DFA::new(r"\w")?;

  let dfa2 = DFA::new(r"\W")?;

  

  let mut cache = dfa1.create_cache();

  assert_eq!(

      Some(HalfMatch::must(0, 2)),

      dfa1.try_search_fwd(&mut cache, &Input::new("Δ"))?,

  );

  

  // Using 'cache' with dfa2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the DFA we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 'dfa1' is also not

  // allowed.

  dfa2.reset_cache(&mut cache);

  assert_eq!(

      Some(HalfMatch::must(0, 3)),

      dfa2.try_search_fwd(&mut cache, &Input::new("☃"))?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-pattern-len"></span>`fn pattern_len(&self) -> usize`

  Returns the total number of patterns compiled into this lazy DFA.

  

  In the case of a DFA that contains no patterns, this returns `0`.

  

  # Example

  

  This example shows the pattern length for a DFA that never matches:

  

  ```rust

  use regex_automata::hybrid::dfa::DFA;

  

  let dfa = DFA::never_match()?;

  assert_eq!(dfa.pattern_len(), 0);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And another example for a DFA that matches at every position:

  

  ```rust

  use regex_automata::hybrid::dfa::DFA;

  

  let dfa = DFA::always_match()?;

  assert_eq!(dfa.pattern_len(), 1);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  And finally, a DFA that was constructed from multiple patterns:

  

  ```rust

  use regex_automata::hybrid::dfa::DFA;

  

  let dfa = DFA::new_many(&["[0-9]+", "[a-z]+", "[A-Z]+"])?;

  assert_eq!(dfa.pattern_len(), 3);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="dfa-byte-classes"></span>`fn byte_classes(&self) -> &ByteClasses` — [`ByteClasses`](../../util/alphabet/index.md#byteclasses)

  Returns the equivalence classes that make up the alphabet for this DFA.

  

  Unless `Config::byte_classes` was disabled, it is possible that

  multiple distinct bytes are grouped into the same equivalence class

  if it is impossible for them to discriminate between a match and a

  non-match. This has the effect of reducing the overall alphabet size

  and in turn potentially substantially reducing the size of the DFA's

  transition table.

  

  The downside of using equivalence classes like this is that every state

  transition will automatically use this map to convert an arbitrary

  byte to its corresponding equivalence class. In practice this has a

  negligible impact on performance.

- <span id="dfa-get-config"></span>`fn get_config(&self) -> &Config` — [`Config`](#config)

  Returns this lazy DFA's configuration.

- <span id="dfa-get-nfa"></span>`fn get_nfa(&self) -> &thompson::NFA` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa)

  Returns a reference to the underlying NFA.

- <span id="dfa-stride2"></span>`fn stride2(&self) -> usize`

  Returns the stride, as a base-2 exponent, required for these

  equivalence classes.

  

  The stride is always the smallest power of 2 that is greater than or

  equal to the alphabet length. This is done so that converting between

  state IDs and indices can be done with shifts alone, which is much

  faster than integer division.

- <span id="dfa-stride"></span>`fn stride(&self) -> usize`

  Returns the total stride for every state in this lazy DFA. This

  corresponds to the total number of transitions used by each state in

  this DFA's transition table.

- <span id="dfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the memory usage, in bytes, of this lazy DFA.

  

  This does **not** include the stack size used up by this lazy DFA. To

  compute that, use `std::mem::size_of::<DFA>()`. This also does not

  include the size of the `Cache` used.

  

  This also does not include any heap memory used by the NFA inside of

  this hybrid NFA/DFA. This is because the NFA's ownership is shared, and

  thus not owned by this hybrid NFA/DFA. More practically, several regex

  engines in this crate embed an NFA, and reporting the NFA's memory

  usage in all of them would likely result in reporting higher heap

  memory than is actually used.

#### Trait Implementations

##### `impl Any for DFA`

- <span id="dfa-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for DFA`

- <span id="dfa-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for DFA`

- <span id="dfa-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for DFA`

- <span id="dfa-clone"></span>`fn clone(&self) -> DFA` — [`DFA`](#dfa)

##### `impl CloneToUninit for DFA`

- <span id="dfa-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for DFA`

- <span id="dfa-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for DFA`

- <span id="dfa-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for DFA`

- <span id="dfa-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:1777-1867`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L1777-L1867)*

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

  Create a new cache for the given lazy DFA.

  

  The cache returned should only be used for searches for the given DFA.

  If you want to reuse the cache for another DFA, then you must call

  `Cache::reset` with that DFA.

- <span id="cache-reset"></span>`fn reset(&mut self, dfa: &DFA)` — [`DFA`](#dfa)

  Reset this cache such that it can be used for searching with the given

  lazy DFA (and only that DFA).

  

  A cache reset permits reusing memory already allocated in this cache

  with a different lazy DFA.

  

  Resetting a cache sets its "clear count" to 0. This is relevant if the

  lazy DFA has been configured to "give up" after it has cleared the

  cache a certain number of times.

  

  Any lazy state ID generated by the cache prior to resetting it is

  invalid after the reset.

  

  # Example

  

  This shows how to re-purpose a cache for use with a different DFA.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let dfa1 = DFA::new(r"\w")?;

  let dfa2 = DFA::new(r"\W")?;

  

  let mut cache = dfa1.create_cache();

  assert_eq!(

      Some(HalfMatch::must(0, 2)),

      dfa1.try_search_fwd(&mut cache, &Input::new("Δ"))?,

  );

  

  // Using 'cache' with dfa2 is not allowed. It may result in panics or

  // incorrect results. In order to re-purpose the cache, we must reset

  // it with the DFA we'd like to use it with.

  //

  // Similarly, after this reset, using the cache with 'dfa1' is also not

  // allowed.

  cache.reset(&dfa2);

  assert_eq!(

      Some(HalfMatch::must(0, 3)),

      dfa2.try_search_fwd(&mut cache, &Input::new("☃"))?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="cache-search-start"></span>`fn search_start(&mut self, at: usize)`

  Initializes a new search starting at the given position.

  

  If a previous search was unfinished, then it is finished automatically

  and a new search is begun.

  

  Note that keeping track of search progress is _not necessary_

  for correct implementations of search using a lazy DFA. Keeping

  track of search progress is only necessary if you want the

  `Config::minimum_bytes_per_state` configuration knob to work.

- <span id="cache-search-update"></span>`fn search_update(&mut self, at: usize)`

  Updates the current search to indicate that it has search to the

  current position.

  

  No special care needs to be taken for reverse searches. Namely, the

  position given may be _less than_ the starting position of the search.

  

  # Panics

  

  This panics if no search has been started by `Cache::search_start`.

- <span id="cache-search-finish"></span>`fn search_finish(&mut self, at: usize)`

  Indicates that a search has finished at the given position.

  

  # Panics

  

  This panics if no search has been started by `Cache::search_start`.

- <span id="cache-search-total-len"></span>`fn search_total_len(&self) -> usize`

  Returns the total number of bytes that have been searched since this

  cache was last cleared.

  

  This is useful for determining the efficiency of the cache. For

  example, the lazy DFA uses this value in conjunction with the

  `Config::minimum_bytes_per_state` knob to help determine whether it

  should quit searching.

  

  This always returns `0` if search progress isn't being tracked. Note

  that the lazy DFA search routines in this crate always track search

  progress.

- <span id="cache-clear-count"></span>`fn clear_count(&self) -> usize`

  Returns the total number of times this cache has been cleared since it

  was either created or last reset.

  

  This is useful for informational purposes or if you want to change

  search strategies based on the number of times the cache has been

  cleared.

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the heap memory usage, in bytes, of this cache.

  

  This does **not** include the stack size used up by this cache. To

  compute that, use `std::mem::size_of::<Cache>()`.

#### Trait Implementations

##### `impl Any for Cache`

- <span id="cache-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Cache`

- <span id="cache-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Cache`

- <span id="cache-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl CloneToUninit for Cache`

- <span id="cache-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Cache`

- <span id="cache-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Cache`

- <span id="cache-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Cache`

- <span id="cache-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Cache`

- <span id="cache-toowned-type-owned"></span>`type Owned = T`

- <span id="cache-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="cache-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Cache`

- <span id="cache-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="cache-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Cache`

- <span id="cache-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="cache-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SearchProgress`

```rust
struct SearchProgress {
    start: usize,
    at: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2048-2051`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L2048-L2051)*

Keeps track of the progress of the current search.

This is updated via the `Cache::search_{start,update,finish}` APIs to
record how many bytes have been searched. This permits computing a
heuristic that represents the efficiency of a cache, and thus helps inform
whether the lazy DFA should give up or not.

#### Implementations

- <span id="searchprogress-len"></span>`fn len(&self) -> usize`

  Returns the length, in bytes, of this search so far.

  

  This automatically handles the case of a reverse search, where `at`

  is likely to be less than `start`.

#### Trait Implementations

##### `impl Any for SearchProgress`

- <span id="searchprogress-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SearchProgress`

- <span id="searchprogress-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SearchProgress`

- <span id="searchprogress-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SearchProgress`

- <span id="searchprogress-clone"></span>`fn clone(&self) -> SearchProgress` — [`SearchProgress`](#searchprogress)

##### `impl CloneToUninit for SearchProgress`

- <span id="searchprogress-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SearchProgress`

- <span id="searchprogress-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SearchProgress`

- <span id="searchprogress-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SearchProgress`

- <span id="searchprogress-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SearchProgress`

- <span id="searchprogress-toowned-type-owned"></span>`type Owned = T`

- <span id="searchprogress-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searchprogress-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SearchProgress`

- <span id="searchprogress-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searchprogress-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SearchProgress`

- <span id="searchprogress-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searchprogress-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Lazy<'i, 'c>`

```rust
struct Lazy<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c mut Cache,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2081-2084`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L2081-L2084)*

A type that groups methods that require the base NFA/DFA and writable
access to the cache.

#### Implementations

- <span id="lazy-new"></span>`fn new(dfa: &'i DFA, cache: &'c mut Cache) -> Lazy<'i, 'c>` — [`DFA`](#dfa), [`Cache`](#cache), [`Lazy`](#lazy)

  Creates a new 'Lazy' wrapper for a DFA and its corresponding cache.

- <span id="lazy-as-ref"></span>`fn as_ref<'a>(self: &'a Self) -> LazyRef<'i, 'a>` — [`LazyRef`](#lazyref)

  Return an immutable view by downgrading a writable cache to a read-only

  cache.

- <span id="lazy-cache-next-state"></span>`fn cache_next_state(&mut self, current: LazyStateID, unit: alphabet::Unit) -> Result<LazyStateID, CacheError>` — [`LazyStateID`](../id/index.md#lazystateid), [`Unit`](../../util/alphabet/index.md#unit), [`CacheError`](../error/index.md#cacheerror)

  This is marked as 'inline(never)' to avoid bloating methods on 'DFA'

  like 'next_state' and 'next_eoi_state' that are called in critical

  areas. The idea is to let the optimizer focus on the other areas of

  those methods as the hot path.

  

  Here's an example that justifies 'inline(never)'

  

  ```ignore

  regex-cli find match hybrid \

    --cache-capacity 100000000 \

    -p '\pL{100}'

    all-codepoints-utf8-100x

  ```

  

  Where 'all-codepoints-utf8-100x' is the UTF-8 encoding of every

  codepoint, in sequence, repeated 100 times.

  

  With 'inline(never)' hyperfine reports 1.1s per run. With

  'inline(always)', hyperfine reports 1.23s. So that's a 10% improvement.

- <span id="lazy-cache-start-group"></span>`fn cache_start_group(&mut self, anchored: Anchored, start: Start) -> Result<LazyStateID, StartError>` — [`Anchored`](../../index.md#anchored), [`Start`](../../util/start/index.md#start), [`LazyStateID`](../id/index.md#lazystateid), [`StartError`](../error/index.md#starterror)

  Compute and cache the starting state for the given pattern ID (if

  present) and the starting configuration.

  

  This panics if a pattern ID is given and the DFA isn't configured to

  build anchored start states for each pattern.

  

  This will never return an unknown lazy state ID.

  

  If caching this state would otherwise result in a cache that has been

  cleared too many times, then an error is returned.

- <span id="lazy-cache-start-one"></span>`fn cache_start_one(&mut self, nfa_start_id: NFAStateID, start: Start) -> Result<LazyStateID, CacheError>` — [`StateID`](../../util/primitives/index.md#stateid), [`Start`](../../util/start/index.md#start), [`LazyStateID`](../id/index.md#lazystateid), [`CacheError`](../error/index.md#cacheerror)

  Compute and cache the starting state for the given NFA state ID and the

  starting configuration. The NFA state ID might be one of the following:

  

  1) An unanchored start state to match any pattern.

  2) An anchored start state to match any pattern.

  3) An anchored start state for a particular pattern.

  

  This will never return an unknown lazy state ID.

  

  If caching this state would otherwise result in a cache that has been

  cleared too many times, then an error is returned.

- <span id="lazy-add-builder-state"></span>`fn add_builder_state(&mut self, builder: StateBuilderNFA, idmap: impl Fn(LazyStateID) -> LazyStateID) -> Result<LazyStateID, CacheError>` — [`StateBuilderNFA`](../../util/determinize/state/index.md#statebuildernfa), [`LazyStateID`](../id/index.md#lazystateid), [`CacheError`](../error/index.md#cacheerror)

  Either add the given builder state to this cache, or return an ID to an

  equivalent state already in this cache.

  

  In the case where no equivalent state exists, the idmap function given

  may be used to transform the identifier allocated. This is useful if

  the caller needs to tag the ID with additional information.

  

  This will never return an unknown lazy state ID.

  

  If caching this state would otherwise result in a cache that has been

  cleared too many times, then an error is returned.

- <span id="lazy-add-state"></span>`fn add_state(&mut self, state: State, idmap: impl Fn(LazyStateID) -> LazyStateID) -> Result<LazyStateID, CacheError>` — [`State`](../../util/determinize/state/index.md#state), [`LazyStateID`](../id/index.md#lazystateid), [`CacheError`](../error/index.md#cacheerror)

  Allocate a new state ID and add the given state to this cache.

  

  The idmap function given may be used to transform the identifier

  allocated. This is useful if the caller needs to tag the ID with

  additional information.

  

  This will never return an unknown lazy state ID.

  

  If caching this state would otherwise result in a cache that has been

  cleared too many times, then an error is returned.

- <span id="lazy-next-state-id"></span>`fn next_state_id(&mut self) -> Result<LazyStateID, CacheError>` — [`LazyStateID`](../id/index.md#lazystateid), [`CacheError`](../error/index.md#cacheerror)

  Allocate a new state ID.

  

  This will never return an unknown lazy state ID.

  

  If caching this state would otherwise result in a cache that has been

  cleared too many times, then an error is returned.

- <span id="lazy-try-clear-cache"></span>`fn try_clear_cache(&mut self) -> Result<(), CacheError>` — [`CacheError`](../error/index.md#cacheerror)

  Attempt to clear the cache used by this lazy DFA.

  

  If clearing the cache exceeds the minimum number of required cache

  clearings, then this will return a cache error. In this case,

  callers should bubble this up as the cache can't be used until it is

  reset. Implementations of search should convert this error into a

  `MatchError::gave_up`.

  

  If 'self.state_saver' is set to save a state, then this state is

  persisted through cache clearing. Otherwise, the cache is returned to

  its state after initialization with two exceptions: its clear count

  is incremented and some of its memory likely has additional capacity.

  That is, clearing a cache does _not_ release memory.

  

  Otherwise, any lazy state ID generated by the cache prior to resetting

  it is invalid after the reset.

- <span id="lazy-reset-cache"></span>`fn reset_cache(&mut self)`

  Clears _and_ resets the cache. Resetting the cache means that no

  states are persisted and the clear count is reset to 0. No heap memory

  is released.

  

  Note that the caller may reset a cache with a different DFA than what

  it was created from. In which case, the cache can now be used with the

  new DFA (and not the old DFA).

- <span id="lazy-clear-cache"></span>`fn clear_cache(&mut self)`

  Clear the cache used by this lazy DFA.

  

  If 'self.state_saver' is set to save a state, then this state is

  persisted through cache clearing. Otherwise, the cache is returned to

  its state after initialization with two exceptions: its clear count

  is incremented and some of its memory likely has additional capacity.

  That is, clearing a cache does _not_ release memory.

  

  Otherwise, any lazy state ID generated by the cache prior to resetting

  it is invalid after the reset.

- <span id="lazy-init-cache"></span>`fn init_cache(&mut self)`

  Initialize this cache from emptiness to a place where it can be used

  for search.

  

  This is called both at cache creation time and after the cache has been

  cleared.

  

  Primarily, this adds the three sentinel states and allocates some

  initial memory.

- <span id="lazy-save-state"></span>`fn save_state(&mut self, id: LazyStateID)` — [`LazyStateID`](../id/index.md#lazystateid)

  Save the state corresponding to the ID given such that the state

  persists through a cache clearing.

  

  While the state may persist, the ID may not. In order to discover the

  new state ID, one must call 'saved_state_id' after a cache clearing.

- <span id="lazy-saved-state-id"></span>`fn saved_state_id(&mut self) -> LazyStateID` — [`LazyStateID`](../id/index.md#lazystateid)

  Returns the updated lazy state ID for a state that was persisted

  through a cache clearing.

  

  It is only correct to call this routine when both a state has been

  saved and the cache has just been cleared. Otherwise, this panics.

- <span id="lazy-set-all-transitions"></span>`fn set_all_transitions(&mut self, from: LazyStateID, to: LazyStateID)` — [`LazyStateID`](../id/index.md#lazystateid)

  Set all transitions on the state 'from' to 'to'.

- <span id="lazy-set-transition"></span>`fn set_transition(&mut self, from: LazyStateID, unit: alphabet::Unit, to: LazyStateID)` — [`LazyStateID`](../id/index.md#lazystateid), [`Unit`](../../util/alphabet/index.md#unit)

  Set the transition on 'from' for 'unit' to 'to'.

  

  This panics if either 'from' or 'to' is invalid.

  

  All unit values are OK.

- <span id="lazy-set-start-state"></span>`fn set_start_state(&mut self, anchored: Anchored, start: Start, id: LazyStateID)` — [`Anchored`](../../index.md#anchored), [`Start`](../../util/start/index.md#start), [`LazyStateID`](../id/index.md#lazystateid)

  Set the start ID for the given pattern ID (if given) and starting

  configuration to the ID given.

  

  This panics if 'id' is not valid or if a pattern ID is given and

  'starts_for_each_pattern' is not enabled.

- <span id="lazy-get-state-builder"></span>`fn get_state_builder(&mut self) -> StateBuilderEmpty` — [`StateBuilderEmpty`](../../util/determinize/state/index.md#statebuilderempty)

  Returns a state builder from this DFA that might have existing

  capacity. This helps avoid allocs in cases where a state is built that

  turns out to already be cached.

  

  Callers must put the state builder back with 'put_state_builder',

  otherwise the allocation reuse won't work.

- <span id="lazy-put-state-builder"></span>`fn put_state_builder(&mut self, builder: StateBuilderNFA)` — [`StateBuilderNFA`](../../util/determinize/state/index.md#statebuildernfa)

  Puts the given state builder back into this DFA for reuse.

  

  Note that building a 'State' from a builder always creates a new alloc,

  so callers should always put the builder back.

#### Trait Implementations

##### `impl Any for Lazy<'i, 'c>`

- <span id="lazy-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Lazy<'i, 'c>`

- <span id="lazy-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Lazy<'i, 'c>`

- <span id="lazy-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Lazy<'i, 'c>`

- <span id="lazy-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Lazy<'i, 'c>`

- <span id="lazy-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Lazy<'i, 'c>`

- <span id="lazy-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Lazy<'i, 'c>`

- <span id="lazy-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazy-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Lazy<'i, 'c>`

- <span id="lazy-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazy-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `LazyRef<'i, 'c>`

```rust
struct LazyRef<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c Cache,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2677-2680`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L2677-L2680)*

A type that groups methods that require the base NFA/DFA and read-only
access to the cache.

#### Implementations

- <span id="lazyref-new"></span>`fn new(dfa: &'i DFA, cache: &'c Cache) -> LazyRef<'i, 'c>` — [`DFA`](#dfa), [`Cache`](#cache), [`LazyRef`](#lazyref)

  Creates a new 'Lazy' wrapper for a DFA and its corresponding cache.

- <span id="lazyref-get-cached-start-id"></span>`fn get_cached_start_id(&self, anchored: Anchored, start: Start) -> Result<LazyStateID, StartError>` — [`Anchored`](../../index.md#anchored), [`Start`](../../util/start/index.md#start), [`LazyStateID`](../id/index.md#lazystateid), [`StartError`](../error/index.md#starterror)

  Return the ID of the start state for the given configuration.

  

  If the start state has not yet been computed, then this returns an

  unknown lazy state ID.

- <span id="lazyref-get-cached-state"></span>`fn get_cached_state(&self, sid: LazyStateID) -> &State` — [`LazyStateID`](../id/index.md#lazystateid), [`State`](../../util/determinize/state/index.md#state)

  Return the cached NFA/DFA powerset state for the given ID.

  

  This panics if the given ID does not address a valid state.

- <span id="lazyref-is-sentinel"></span>`fn is_sentinel(&self, id: LazyStateID) -> bool` — [`LazyStateID`](../id/index.md#lazystateid)

  Returns true if and only if the given ID corresponds to a "sentinel"

  state.

  

  A sentinel state is a state that signifies a special condition of

  search, and where every transition maps back to itself. See LazyStateID

  for more details. Note that start and match states are _not_ sentinels

  since they may otherwise be real states with non-trivial transitions.

  The purposes of sentinel states is purely to indicate something. Their

  transitions are not meant to be followed.

- <span id="lazyref-unknown-id"></span>`fn unknown_id(&self) -> LazyStateID` — [`LazyStateID`](../id/index.md#lazystateid)

  Returns the ID of the unknown state for this lazy DFA.

- <span id="lazyref-dead-id"></span>`fn dead_id(&self) -> LazyStateID` — [`LazyStateID`](../id/index.md#lazystateid)

  Returns the ID of the dead state for this lazy DFA.

- <span id="lazyref-quit-id"></span>`fn quit_id(&self) -> LazyStateID` — [`LazyStateID`](../id/index.md#lazystateid)

  Returns the ID of the quit state for this lazy DFA.

- <span id="lazyref-is-valid"></span>`fn is_valid(&self, id: LazyStateID) -> bool` — [`LazyStateID`](../id/index.md#lazystateid)

  Returns true if and only if the given ID is valid.

  

  An ID is valid if it is both a valid index into the transition table

  and is a multiple of the DFA's stride.

- <span id="lazyref-state-fits-in-cache"></span>`fn state_fits_in_cache(&self, state: &State) -> bool` — [`State`](../../util/determinize/state/index.md#state)

  Returns true if adding the state given would fit in this cache.

- <span id="lazyref-state-builder-fits-in-cache"></span>`fn state_builder_fits_in_cache(&self, state: &StateBuilderNFA) -> bool` — [`StateBuilderNFA`](../../util/determinize/state/index.md#statebuildernfa)

  Returns true if adding the state to be built by the given builder would

  fit in this cache.

- <span id="lazyref-memory-usage-for-one-more-state"></span>`fn memory_usage_for_one_more_state(&self, state_heap_size: usize) -> usize`

  Returns the additional memory usage, in bytes, required to add one more

  state to this cache. The given size should be the heap size, in bytes,

  that would be used by the new state being added.

#### Trait Implementations

##### `impl Any for LazyRef<'i, 'c>`

- <span id="lazyref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for LazyRef<'i, 'c>`

- <span id="lazyref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for LazyRef<'i, 'c>`

- <span id="lazyref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for LazyRef<'i, 'c>`

- <span id="lazyref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for LazyRef<'i, 'c>`

- <span id="lazyref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for LazyRef<'i, 'c>`

- <span id="lazyref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for LazyRef<'i, 'c>`

- <span id="lazyref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="lazyref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for LazyRef<'i, 'c>`

- <span id="lazyref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="lazyref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2880-2899`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L2880-L2899)*

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

  Return a new default lazy DFA builder configuration.

- <span id="config-match-kind"></span>`fn match_kind(self, kind: MatchKind) -> Config` — [`MatchKind`](../../index.md#matchkind), [`Config`](#config)

  Set the desired match semantics.

  

  The default is [`MatchKind::LeftmostFirst`](../../index.md), which corresponds to the

  match semantics of Perl-like regex engines. That is, when multiple

  patterns would match at the same leftmost position, the pattern that

  appears first in the concrete syntax is chosen.

  

  Currently, the only other kind of match semantics supported is

  [`MatchKind::All`](../../index.md). This corresponds to classical DFA construction

  where all possible matches are added to the lazy DFA.

  

  Typically, `All` is used when one wants to execute an overlapping

  search and `LeftmostFirst` otherwise. In particular, it rarely makes

  sense to use `All` with the various "leftmost" find routines, since the

  leftmost routines depend on the `LeftmostFirst` automata construction

  strategy. Specifically, `LeftmostFirst` adds dead states to the

  lazy DFA as a way to terminate the search and report a match.

  `LeftmostFirst` also supports non-greedy matches using this strategy

  where as `All` does not.

  

  # Example: overlapping search

  

  This example shows the typical use of `MatchKind::All`, which is to

  report overlapping matches.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{

      hybrid::dfa::{DFA, OverlappingState},

      HalfMatch, Input, MatchKind,

  };

  

  let dfa = DFA::builder()

      .configure(DFA::config().match_kind(MatchKind::All))

      .build_many(&[r"\w+$", r"\S+$"])?;

  let mut cache = dfa.create_cache();

  let haystack = "@foo";

  let mut state = OverlappingState::start();

  

  let expected = Some(HalfMatch::must(1, 4));

  dfa.try_search_overlapping_fwd(

      &mut cache, &Input::new(haystack), &mut state,

  )?;

  assert_eq!(expected, state.get_match());

  

  // The first pattern also matches at the same position, so re-running

  // the search will yield another match. Notice also that the first

  // pattern is returned after the second. This is because the second

  // pattern begins its match before the first, is therefore an earlier

  // match and is thus reported first.

  let expected = Some(HalfMatch::must(0, 4));

  dfa.try_search_overlapping_fwd(

      &mut cache, &Input::new(haystack), &mut state,

  )?;

  assert_eq!(expected, state.get_match());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  # Example: reverse automaton to find start of match

  

  Another example for using `MatchKind::All` is for constructing a

  reverse automaton to find the start of a match. `All` semantics are

  used for this in order to find the longest possible match, which

  corresponds to the leftmost starting position.

  

  Note that if you need the starting position then

  [`hybrid::regex::Regex`](crate::hybrid::regex::Regex) will handle this

  for you, so it's usually not necessary to do this yourself.

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      nfa::thompson::NFA,

      Anchored, HalfMatch, Input, MatchKind,

  };

  

  let input = Input::new("123foobar456");

  let pattern = r"[a-z]+r";

  

  let dfa_fwd = DFA::new(pattern)?;

  let dfa_rev = DFA::builder()

      .thompson(NFA::config().reverse(true))

      .configure(DFA::config().match_kind(MatchKind::All))

      .build(pattern)?;

  let mut cache_fwd = dfa_fwd.create_cache();

  let mut cache_rev = dfa_rev.create_cache();

  

  let expected_fwd = HalfMatch::must(0, 9);

  let expected_rev = HalfMatch::must(0, 3);

  let got_fwd = dfa_fwd.try_search_fwd(&mut cache_fwd, &input)?.unwrap();

  // Here we don't specify the pattern to search for since there's only

  // one pattern and we're doing a leftmost search. But if this were an

  // overlapping search, you'd need to specify the pattern that matched

  // in the forward direction. (Otherwise, you might wind up finding the

  // starting position of a match of some other pattern.) That in turn

  // requires building the reverse automaton with starts_for_each_pattern

  // enabled.

  let input = input

      .clone()

      .range(..got_fwd.offset())

      .anchored(Anchored::Yes);

  let got_rev = dfa_rev.try_search_rev(&mut cache_rev, &input)?.unwrap();

  assert_eq!(expected_fwd, got_fwd);

  assert_eq!(expected_rev, got_rev);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../util/prefilter/index.md#prefilter), [`Config`](#config)

  Set a prefilter to be used whenever a start state is entered.

  

  A [`Prefilter`](../../util/prefilter/index.md) in this context is meant to accelerate searches by

  looking for literal prefixes that every match for the corresponding

  pattern (or patterns) must start with. Once a prefilter produces a

  match, the underlying search routine continues on to try and confirm

  the match.

  

  Be warned that setting a prefilter does not guarantee that the search

  will be faster. While it's usually a good bet, if the prefilter

  produces a lot of false positive candidates (i.e., positions matched

  by the prefilter but not by the regex), then the overall result can

  be slower than if you had just executed the regex engine without any

  prefilters.

  

  Note that unless `Config::specialize_start_states` has been

  explicitly set, then setting this will also enable (when `pre` is

  `Some`) or disable (when `pre` is `None`) start state specialization.

  This occurs because without start state specialization, a prefilter

  is likely to be less effective. And without a prefilter, start state

  specialization is usually pointless.

  

  By default no prefilter is set.

  

  # Example

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      util::prefilter::Prefilter,

      Input, HalfMatch, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["foo", "bar"]);

  let re = DFA::builder()

      .configure(DFA::config().prefilter(pre))

      .build(r"(foo|bar)[a-z]+")?;

  let mut cache = re.create_cache();

  let input = Input::new("foo1 barfox bar");

  assert_eq!(

      Some(HalfMatch::must(0, 11)),

      re.try_search_fwd(&mut cache, &input)?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Be warned though that an incorrect prefilter can lead to incorrect

  results!

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      util::prefilter::Prefilter,

      Input, HalfMatch, MatchKind,

  };

  

  let pre = Prefilter::new(MatchKind::LeftmostFirst, &["foo", "car"]);

  let re = DFA::builder()

      .configure(DFA::config().prefilter(pre))

      .build(r"(foo|bar)[a-z]+")?;

  let mut cache = re.create_cache();

  let input = Input::new("foo1 barfox bar");

  assert_eq!(

      // No match reported even though there clearly is one!

      None,

      re.try_search_fwd(&mut cache, &input)?,

  );

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-starts-for-each-pattern"></span>`fn starts_for_each_pattern(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to compile a separate start state for each pattern in the

  lazy DFA.

  

  When enabled, a separate **anchored** start state is added for each

  pattern in the lazy DFA. When this start state is used, then the DFA

  will only search for matches for the pattern specified, even if there

  are other patterns in the DFA.

  

  The main downside of this option is that it can potentially increase

  the size of the DFA and/or increase the time it takes to build the

  DFA at search time. However, since this is configuration for a lazy

  DFA, these states aren't actually built unless they're used. Enabling

  this isn't necessarily free, however, as it may result in higher cache

  usage.

  

  There are a few reasons one might want to enable this (it's disabled

  by default):

  

  1. When looking for the start of an overlapping match (using a reverse

  DFA), doing it correctly requires starting the reverse search using the

  starting state of the pattern that matched in the forward direction.

  Indeed, when building a [`Regex`](crate::hybrid::regex::Regex), it

  will automatically enable this option when building the reverse DFA

  internally.

  2. When you want to use a DFA with multiple patterns to both search

  for matches of any pattern or to search for anchored matches of one

  particular pattern while using the same DFA. (Otherwise, you would need

  to compile a new DFA for each pattern.)

  

  By default this is disabled.

  

  # Example

  

  This example shows how to use this option to permit the same lazy DFA

  to run both general searches for any pattern and anchored searches for

  a specific pattern.

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      Anchored, HalfMatch, Input, PatternID,

  };

  

  let dfa = DFA::builder()

      .configure(DFA::config().starts_for_each_pattern(true))

      .build_many(&[r"[a-z0-9]{6}", r"[a-z][a-z0-9]{5}"])?;

  let mut cache = dfa.create_cache();

  let haystack = "bar foo123";

  

  // Here's a normal unanchored search that looks for any pattern.

  let expected = HalfMatch::must(0, 10);

  let input = Input::new(haystack);

  assert_eq!(Some(expected), dfa.try_search_fwd(&mut cache, &input)?);

  // We can also do a normal anchored search for any pattern. Since it's

  // an anchored search, we position the start of the search where we

  // know the match will begin.

  let expected = HalfMatch::must(0, 10);

  let input = Input::new(haystack).range(4..);

  assert_eq!(Some(expected), dfa.try_search_fwd(&mut cache, &input)?);

  // Since we compiled anchored start states for each pattern, we can

  // also look for matches of other patterns explicitly, even if a

  // different pattern would have normally matched.

  let expected = HalfMatch::must(1, 10);

  let input = Input::new(haystack)

      .range(4..)

      .anchored(Anchored::Pattern(PatternID::must(1)));

  assert_eq!(Some(expected), dfa.try_search_fwd(&mut cache, &input)?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-byte-classes"></span>`fn byte_classes(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to attempt to shrink the size of the lazy DFA's alphabet or

  not.

  

  This option is enabled by default and should never be disabled unless

  one is debugging the lazy DFA.

  

  When enabled, the lazy DFA will use a map from all possible bytes

  to their corresponding equivalence class. Each equivalence class

  represents a set of bytes that does not discriminate between a match

  and a non-match in the DFA. For example, the pattern `[ab]+` has at

  least two equivalence classes: a set containing `a` and `b` and a set

  containing every byte except for `a` and `b`. `a` and `b` are in the

  same equivalence classes because they never discriminate between a

  match and a non-match.

  

  The advantage of this map is that the size of the transition table

  can be reduced drastically from `#states * 256 * sizeof(LazyStateID)`

  to `#states * k * sizeof(LazyStateID)` where `k` is the number of

  equivalence classes (rounded up to the nearest power of 2). As a

  result, total space usage can decrease substantially. Moreover, since a

  smaller alphabet is used, DFA compilation during search becomes faster

  as well since it will potentially be able to reuse a single transition

  for multiple bytes.

  

  **WARNING:** This is only useful for debugging lazy DFAs. Disabling

  this does not yield any speed advantages. Namely, even when this is

  disabled, a byte class map is still used while searching. The only

  difference is that every byte will be forced into its own distinct

  equivalence class. This is useful for debugging the actual generated

  transitions because it lets one see the transitions defined on actual

  bytes instead of the equivalence classes.

- <span id="config-unicode-word-boundary"></span>`fn unicode_word_boundary(self, yes: bool) -> Config` — [`Config`](#config)

  Heuristically enable Unicode word boundaries.

  

  When set, this will attempt to implement Unicode word boundaries as if

  they were ASCII word boundaries. This only works when the search input

  is ASCII only. If a non-ASCII byte is observed while searching, then a

  `MatchError::quit` error is returned.

  

  A possible alternative to enabling this option is to simply use an

  ASCII word boundary, e.g., via `(?-u:\b)`. The main reason to use this

  option is if you absolutely need Unicode support. This option lets one

  use a fast search implementation (a DFA) for some potentially very

  common cases, while providing the option to fall back to some other

  regex engine to handle the general case when an error is returned.

  

  If the pattern provided has no Unicode word boundary in it, then this

  option has no effect. (That is, quitting on a non-ASCII byte only

  occurs when this option is enabled _and_ a Unicode word boundary is

  present in the pattern.)

  

  This is almost equivalent to setting all non-ASCII bytes to be quit

  bytes. The only difference is that this will cause non-ASCII bytes to

  be quit bytes _only_ when a Unicode word boundary is present in the

  pattern.

  

  When enabling this option, callers _must_ be prepared to

  handle a [`MatchError`](../../index.md) error during search. When using a

  [`Regex`](crate::hybrid::regex::Regex), this corresponds to using the

  `try_` suite of methods. Alternatively, if callers can guarantee that

  their input is ASCII only, then a `MatchError::quit` error will never

  be returned while searching.

  

  This is disabled by default.

  

  # Example

  

  This example shows how to heuristically enable Unicode word boundaries

  in a pattern. It also shows what happens when a search comes across a

  non-ASCII byte.

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      HalfMatch, Input, MatchError,

  };

  

  let dfa = DFA::builder()

      .configure(DFA::config().unicode_word_boundary(true))

      .build(r"\b[0-9]+\b")?;

  let mut cache = dfa.create_cache();

  

  // The match occurs before the search ever observes the snowman

  // character, so no error occurs.

  let haystack = "foo 123  ☃";

  let expected = Some(HalfMatch::must(0, 7));

  let got = dfa.try_search_fwd(&mut cache, &Input::new(haystack))?;

  assert_eq!(expected, got);

  

  // Notice that this search fails, even though the snowman character

  // occurs after the ending match offset. This is because search

  // routines read one byte past the end of the search to account for

  // look-around, and indeed, this is required here to determine whether

  // the trailing \b matches.

  let haystack = "foo 123 ☃";

  let expected = MatchError::quit(0xE2, 8);

  let got = dfa.try_search_fwd(&mut cache, &Input::new(haystack));

  assert_eq!(Err(expected), got);

  

  // Another example is executing a search where the span of the haystack

  // we specify is all ASCII, but there is non-ASCII just before it. This

  // correctly also reports an error.

  let input = Input::new("β123").range(2..);

  let expected = MatchError::quit(0xB2, 1);

  let got = dfa.try_search_fwd(&mut cache, &input);

  assert_eq!(Err(expected), got);

  

  // And similarly for the trailing word boundary.

  let input = Input::new("123β").range(..3);

  let expected = MatchError::quit(0xCE, 3);

  let got = dfa.try_search_fwd(&mut cache, &input);

  assert_eq!(Err(expected), got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-quit"></span>`fn quit(self, byte: u8, yes: bool) -> Config` — [`Config`](#config)

  Add a "quit" byte to the lazy DFA.

  

  When a quit byte is seen during search time, then search will return a

  `MatchError::quit` error indicating the offset at which the search

  stopped.

  

  A quit byte will always overrule any other aspects of a regex. For

  example, if the `x` byte is added as a quit byte and the regex `\w` is

  used, then observing `x` will cause the search to quit immediately

  despite the fact that `x` is in the `\w` class.

  

  This mechanism is primarily useful for heuristically enabling certain

  features like Unicode word boundaries in a DFA. Namely, if the input

  to search is ASCII, then a Unicode word boundary can be implemented

  via an ASCII word boundary with no change in semantics. Thus, a DFA

  can attempt to match a Unicode word boundary but give up as soon as it

  observes a non-ASCII byte. Indeed, if callers set all non-ASCII bytes

  to be quit bytes, then Unicode word boundaries will be permitted when

  building lazy DFAs. Of course, callers should enable

  `Config::unicode_word_boundary` if they want this behavior instead.

  (The advantage being that non-ASCII quit bytes will only be added if a

  Unicode word boundary is in the pattern.)

  

  When enabling this option, callers _must_ be prepared to

  handle a [`MatchError`](../../index.md) error during search. When using a

  [`Regex`](crate::hybrid::regex::Regex), this corresponds to using the

  `try_` suite of methods.

  

  By default, there are no quit bytes set.

  

  # Panics

  

  This panics if heuristic Unicode word boundaries are enabled and any

  non-ASCII byte is removed from the set of quit bytes. Namely, enabling

  Unicode word boundaries requires setting every non-ASCII byte to a quit

  byte. So if the caller attempts to undo any of that, then this will

  panic.

  

  # Example

  

  This example shows how to cause a search to terminate if it sees a

  `\n` byte. This could be useful if, for example, you wanted to prevent

  a user supplied pattern from matching across a line boundary.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, MatchError, Input};

  

  let dfa = DFA::builder()

      .configure(DFA::config().quit(b'\n', true))

      .build(r"foo\p{any}+bar")?;

  let mut cache = dfa.create_cache();

  

  let haystack = "foo\nbar";

  // Normally this would produce a match, since \p{any} contains '\n'.

  // But since we instructed the automaton to enter a quit state if a

  // '\n' is observed, this produces a match error instead.

  let expected = MatchError::quit(b'\n', 3);

  let got = dfa.try_search_fwd(

      &mut cache,

      &Input::new(haystack),

  ).unwrap_err();

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-specialize-start-states"></span>`fn specialize_start_states(self, yes: bool) -> Config` — [`Config`](#config)

  Enable specializing start states in the lazy DFA.

  

  When start states are specialized, an implementor of a search routine

  using a lazy DFA can tell when the search has entered a starting state.

  When start states aren't specialized, then it is impossible to know

  whether the search has entered a start state.

  

  Ideally, this option wouldn't need to exist and we could always

  specialize start states. The problem is that start states can be quite

  active. This in turn means that an efficient search routine is likely

  to ping-pong between a heavily optimized hot loop that handles most

  states and to a less optimized specialized handling of start states.

  This causes branches to get heavily mispredicted and overall can

  materially decrease throughput. Therefore, specializing start states

  should only be enabled when it is needed.

  

  Knowing whether a search is in a start state is typically useful when a

  prefilter is active for the search. A prefilter is typically only run

  when in a start state and a prefilter can greatly accelerate a search.

  Therefore, the possible cost of specializing start states is worth it

  in this case. Otherwise, if you have no prefilter, there is likely no

  reason to specialize start states.

  

  This is disabled by default, but note that it is automatically

  enabled (or disabled) if `Config::prefilter` is set. Namely, unless

  `specialize_start_states` has already been set, `Config::prefilter`

  will automatically enable or disable it based on whether a prefilter

  is present or not, respectively. This is done because a prefilter's

  effectiveness is rooted in being executed whenever the DFA is in a

  start state, and that's only possible to do when they are specialized.

  

  Note that it is plausibly reasonable to _disable_ this option

  explicitly while _enabling_ a prefilter. In that case, a prefilter

  will still be run at the beginning of a search, but never again. This

  in theory could strike a good balance if you're in a situation where a

  prefilter is likely to produce many false positive candidates.

  

  # Example

  

  This example shows how to enable start state specialization and then

  shows how to check whether a state is a start state or not.

  

  ```rust

  use regex_automata::{hybrid::dfa::DFA, MatchError, Input};

  

  let dfa = DFA::builder()

      .configure(DFA::config().specialize_start_states(true))

      .build(r"[a-z]+")?;

  let mut cache = dfa.create_cache();

  

  let haystack = "123 foobar 4567".as_bytes();

  let sid = dfa.start_state_forward(&mut cache, &Input::new(haystack))?;

  // The ID returned by 'start_state_forward' will always be tagged as

  // a start state when start state specialization is enabled.

  assert!(sid.is_tagged());

  assert!(sid.is_start());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Compare the above with the default lazy DFA configuration where

  start states are _not_ specialized. In this case, the start state

  is not tagged and `sid.is_start()` returns false.

  

  ```rust

  use regex_automata::{hybrid::dfa::DFA, MatchError, Input};

  

  let dfa = DFA::new(r"[a-z]+")?;

  let mut cache = dfa.create_cache();

  

  let haystack = "123 foobar 4567".as_bytes();

  let sid = dfa.start_state_forward(&mut cache, &Input::new(haystack))?;

  // Start states are not tagged in the default configuration!

  assert!(!sid.is_tagged());

  assert!(!sid.is_start());

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-cache-capacity"></span>`fn cache_capacity(self, bytes: usize) -> Config` — [`Config`](#config)

  Sets the maximum amount of heap memory, in bytes, to allocate to the

  cache for use during a lazy DFA search. If the lazy DFA would otherwise

  use more heap memory, then, depending on other configuration knobs,

  either stop the search and return an error or clear the cache and

  continue the search.

  

  The default cache capacity is some "reasonable" number that will

  accommodate most regular expressions. You may find that if you need

  to build a large DFA then it may be necessary to increase the cache

  capacity.

  

  Note that while building a lazy DFA will do a "minimum" check to ensure

  the capacity is big enough, this is more or less about correctness.

  If the cache is bigger than the minimum but still "too small," then the

  lazy DFA could wind up spending a lot of time clearing the cache and

  recomputing transitions, thus negating the performance benefits of a

  lazy DFA. Thus, setting the cache capacity is mostly an experimental

  endeavor. For most common patterns, however, the default should be

  sufficient.

  

  For more details on how the lazy DFA's cache is used, see the

  documentation for [`Cache`](#cache).

  

  # Example

  

  This example shows what happens if the configured cache capacity is

  too small. In such cases, one can override the cache capacity to make

  it bigger. Alternatively, one might want to use less memory by setting

  a smaller cache capacity.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let pattern = r"\p{L}{1000}";

  

  // The default cache capacity is likely too small to deal with regexes

  // that are very large. Large repetitions of large Unicode character

  // classes are a common way to make very large regexes.

  let _ = DFA::new(pattern).unwrap_err();

  // Bump up the capacity to something bigger.

  let dfa = DFA::builder()

      .configure(DFA::config().cache_capacity(100 * (1<<20))) // 100 MB

      .build(pattern)?;

  let mut cache = dfa.create_cache();

  

  let haystack = "ͰͲͶͿΆΈΉΊΌΎΏΑΒΓΔΕΖΗΘΙ".repeat(50);

  let expected = Some(HalfMatch::must(0, 2000));

  let got = dfa.try_search_fwd(&mut cache, &Input::new(&haystack))?;

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-skip-cache-capacity-check"></span>`fn skip_cache_capacity_check(self, yes: bool) -> Config` — [`Config`](#config)

  Configures construction of a lazy DFA to use the minimum cache capacity

  if the configured capacity is otherwise too small for the provided NFA.

  

  This is useful if you never want lazy DFA construction to fail because

  of a capacity that is too small.

  

  In general, this option is typically not a good idea. In particular,

  while a minimum cache capacity does permit the lazy DFA to function

  where it otherwise couldn't, it's plausible that it may not function

  well if it's constantly running out of room. In that case, the speed

  advantages of the lazy DFA may be negated. On the other hand, the

  "minimum" cache capacity computed may not be completely accurate and

  could actually be bigger than what is really necessary. Therefore, it

  is plausible that using the minimum cache capacity could still result

  in very good performance.

  

  This is disabled by default.

  

  # Example

  

  This example shows what happens if the configured cache capacity is

  too small. In such cases, one could override the capacity explicitly.

  An alternative, demonstrated here, let's us force construction to use

  the minimum cache capacity if the configured capacity is otherwise

  too small.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

  

  let pattern = r"\p{L}{1000}";

  

  // The default cache capacity is likely too small to deal with regexes

  // that are very large. Large repetitions of large Unicode character

  // classes are a common way to make very large regexes.

  let _ = DFA::new(pattern).unwrap_err();

  // Configure construction such it automatically selects the minimum

  // cache capacity if it would otherwise be too small.

  let dfa = DFA::builder()

      .configure(DFA::config().skip_cache_capacity_check(true))

      .build(pattern)?;

  let mut cache = dfa.create_cache();

  

  let haystack = "ͰͲͶͿΆΈΉΊΌΎΏΑΒΓΔΕΖΗΘΙ".repeat(50);

  let expected = Some(HalfMatch::must(0, 2000));

  let got = dfa.try_search_fwd(&mut cache, &Input::new(&haystack))?;

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-minimum-cache-clear-count"></span>`fn minimum_cache_clear_count(self, min: Option<usize>) -> Config` — [`Config`](#config)

  Configure a lazy DFA search to quit after a certain number of cache

  clearings.

  

  When a minimum is set, then a lazy DFA search will *possibly* "give

  up" after the minimum number of cache clearings has occurred. This is

  typically useful in scenarios where callers want to detect whether the

  lazy DFA search is "efficient" or not. If the cache is cleared too many

  times, this is a good indicator that it is not efficient, and thus, the

  caller may wish to use some other regex engine.

  

  Note that the number of times a cache is cleared is a property of

  the cache itself. Thus, if a cache is used in a subsequent search

  with a similarly configured lazy DFA, then it could cause the

  search to "give up" if the cache needed to be cleared, depending

  on its internal count and configured minimum. The cache clear

  count can only be reset to `0` via `DFA::reset_cache` (or

  [`Regex::reset_cache`](crate::hybrid::regex::Regex::reset_cache) if

  you're using the `Regex` API).

  

  By default, no minimum is configured. Thus, a lazy DFA search will

  never give up due to cache clearings. If you do set this option, you

  might consider also setting `Config::minimum_bytes_per_state` in

  order for the lazy DFA to take efficiency into account before giving

  up.

  

  # Example

  

  This example uses a somewhat pathological configuration to demonstrate

  the _possible_ behavior of cache clearing and how it might result

  in a search that returns an error.

  

  It is important to note that the precise mechanics of how and when

  a cache gets cleared is an implementation detail.

  

  ```rust

  if cfg!(miri) { return Ok(()); } // miri takes too long

  use regex_automata::{hybrid::dfa::DFA, Input, MatchError, MatchErrorKind};

  

  // This is a carefully chosen regex. The idea is to pick one

  // that requires some decent number of states (hence the bounded

  // repetition). But we specifically choose to create a class with an

  // ASCII letter and a non-ASCII letter so that we can check that no new

  // states are created once the cache is full. Namely, if we fill up the

  // cache on a haystack of 'a's, then in order to match one 'β', a new

  // state will need to be created since a 'β' is encoded with multiple

  // bytes. Since there's no room for this state, the search should quit

  // at the very first position.

  let pattern = r"[aβ]{100}";

  let dfa = DFA::builder()

      .configure(

          // Configure it so that we have the minimum cache capacity

          // possible. And that if any clearings occur, the search quits.

          DFA::config()

              .skip_cache_capacity_check(true)

              .cache_capacity(0)

              .minimum_cache_clear_count(Some(0)),

      )

      .build(pattern)?;

  let mut cache = dfa.create_cache();

  

  // Our search will give up before reaching the end!

  let haystack = "a".repeat(101).into_bytes();

  let result = dfa.try_search_fwd(&mut cache, &Input::new(&haystack));

  assert!(matches!(

      *result.unwrap_err().kind(),

      MatchErrorKind::GaveUp { .. },

  ));

  

  // Now that we know the cache is full, if we search a haystack that we

  // know will require creating at least one new state, it should not

  // be able to make much progress.

  let haystack = "β".repeat(101).into_bytes();

  let result = dfa.try_search_fwd(&mut cache, &Input::new(&haystack));

  assert!(matches!(

      *result.unwrap_err().kind(),

      MatchErrorKind::GaveUp { .. },

  ));

  

  // If we reset the cache, then we should be able to create more states

  // and make more progress with searching for betas.

  cache.reset(&dfa);

  let haystack = "β".repeat(101).into_bytes();

  let result = dfa.try_search_fwd(&mut cache, &Input::new(&haystack));

  assert!(matches!(

      *result.unwrap_err().kind(),

      MatchErrorKind::GaveUp { .. },

  ));

  

  // ... switching back to ASCII still makes progress since it just needs

  // to set transitions on existing states!

  let haystack = "a".repeat(101).into_bytes();

  let result = dfa.try_search_fwd(&mut cache, &Input::new(&haystack));

  assert!(matches!(

      *result.unwrap_err().kind(),

      MatchErrorKind::GaveUp { .. },

  ));

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="config-minimum-bytes-per-state"></span>`fn minimum_bytes_per_state(self, min: Option<usize>) -> Config` — [`Config`](#config)

  Configure a lazy DFA search to quit only when its efficiency drops

  below the given minimum.

  

  The efficiency of the cache is determined by the number of DFA states

  compiled per byte of haystack searched. For example, if the efficiency

  is 2, then it means the lazy DFA is creating a new DFA state after

  searching approximately 2 bytes in a haystack. Generally speaking, 2

  is quite bad and it's likely that even a slower regex engine like the

  [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM) would be faster.

  

  This has no effect if `Config::minimum_cache_clear_count` is not set.

  Namely, this option only kicks in when the cache has been cleared more

  than the minimum number. If no minimum is set, then the cache is simply

  cleared whenever it fills up and it is impossible for the lazy DFA to

  quit due to ineffective use of the cache.

  

  In general, if one is setting `Config::minimum_cache_clear_count`,

  then one should probably also set this knob as well. The reason is

  that the absolute number of times the cache is cleared is generally

  not a great predictor of efficiency. For example, if a new DFA state

  is created for every 1,000 bytes searched, then it wouldn't be hard

  for the cache to get cleared more than `N` times and then cause the

  lazy DFA to quit. But a new DFA state every 1,000 bytes is likely quite

  good from a performance perspective, and it's likely that the lazy

  DFA should continue searching, even if it requires clearing the cache

  occasionally.

  

  Finally, note that if you're implementing your own lazy DFA search

  routine and also want this efficiency check to work correctly, then

  you'll need to use the following routines to record search progress:

  

  * Call `Cache::search_start` at the beginning of every search.

  * Call `Cache::search_update` whenever `DFA::next_state` is

  called.

  * Call `Cache::search_finish` before completing a search. (It is

  not strictly necessary to call this when an error is returned, as

  `Cache::search_start` will automatically finish the previous search

  for you. But calling it where possible before returning helps improve

  the accuracy of how many bytes have actually been searched.)

- <span id="config-get-match-kind"></span>`fn get_match_kind(&self) -> MatchKind` — [`MatchKind`](../../index.md#matchkind)

  Returns the match semantics set in this configuration.

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../util/prefilter/index.md#prefilter)

  Returns the prefilter set in this configuration, if one at all.

- <span id="config-get-starts-for-each-pattern"></span>`fn get_starts_for_each_pattern(&self) -> bool`

  Returns whether this configuration has enabled anchored starting states

  for every pattern in the DFA.

- <span id="config-get-byte-classes"></span>`fn get_byte_classes(&self) -> bool`

  Returns whether this configuration has enabled byte classes or not.

  This is typically a debugging oriented option, as disabling it confers

  no speed benefit.

- <span id="config-get-unicode-word-boundary"></span>`fn get_unicode_word_boundary(&self) -> bool`

  Returns whether this configuration has enabled heuristic Unicode word

  boundary support. When enabled, it is possible for a search to return

  an error.

- <span id="config-get-quit"></span>`fn get_quit(&self, byte: u8) -> bool`

  Returns whether this configuration will instruct the lazy DFA to enter

  a quit state whenever the given byte is seen during a search. When at

  least one byte has this enabled, it is possible for a search to return

  an error.

- <span id="config-get-specialize-start-states"></span>`fn get_specialize_start_states(&self) -> bool`

  Returns whether this configuration will instruct the lazy DFA to

  "specialize" start states. When enabled, the lazy DFA will tag start

  states so that search routines using the lazy DFA can detect when

  it's in a start state and do some kind of optimization (like run a

  prefilter).

- <span id="config-get-cache-capacity"></span>`fn get_cache_capacity(&self) -> usize`

  Returns the cache capacity set on this configuration.

- <span id="config-get-skip-cache-capacity-check"></span>`fn get_skip_cache_capacity_check(&self) -> bool`

  Returns whether the cache capacity check should be skipped.

- <span id="config-get-minimum-cache-clear-count"></span>`fn get_minimum_cache_clear_count(&self) -> Option<usize>`

  Returns, if set, the minimum number of times the cache must be cleared

  before a lazy DFA search can give up. When no minimum is set, then a

  search will never quit and will always clear the cache whenever it

  fills up.

- <span id="config-get-minimum-bytes-per-state"></span>`fn get_minimum_bytes_per_state(&self) -> Option<usize>`

  Returns, if set, the minimum number of bytes per state that need to be

  processed in order for the lazy DFA to keep going. If the minimum falls

  below this number (and the cache has been cleared a minimum number of

  times), then the lazy DFA will return a "gave up" error.

- <span id="config-get-minimum-cache-capacity"></span>`fn get_minimum_cache_capacity(&self, nfa: &thompson::NFA) -> Result<usize, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`BuildError`](../error/index.md#builderror)

  Returns the minimum lazy DFA cache capacity required for the given NFA.

  

  The cache capacity required for a particular NFA may change without

  notice. Callers should not rely on it being stable.

  

  This is useful for informational purposes, but can also be useful for

  other reasons. For example, if one wants to check the minimum cache

  capacity themselves or if one wants to set the capacity based on the

  minimum.

  

  This may return an error if this configuration does not support all of

  the instructions used in the given NFA. For example, if the NFA has a

  Unicode word boundary but this configuration does not enable heuristic

  support for Unicode word boundaries.

- <span id="config-byte-classes-from-nfa"></span>`fn byte_classes_from_nfa(&self, nfa: &thompson::NFA, quit: &ByteSet) -> ByteClasses` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`ByteSet`](../../util/alphabet/index.md#byteset), [`ByteClasses`](../../util/alphabet/index.md#byteclasses)

  Returns the byte class map used during search from the given NFA.

  

  If byte classes are disabled on this configuration, then a map is

  returned that puts each byte in its own equivalent class.

- <span id="config-quit-set-from-nfa"></span>`fn quit_set_from_nfa(&self, nfa: &thompson::NFA) -> Result<ByteSet, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`ByteSet`](../../util/alphabet/index.md#byteset), [`BuildError`](../error/index.md#builderror)

  Return the quit set for this configuration and the given NFA.

  

  This may return an error if the NFA is incompatible with this

  configuration's quit set. For example, if the NFA has a Unicode word

  boundary and the quit set doesn't include non-ASCII bytes.

- <span id="config-overwrite"></span>`fn overwrite(&self, o: Config) -> Config` — [`Config`](#config)

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

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl CloneToUninit for Config`

- <span id="config-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Config`

- <span id="config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

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

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:3991-3995`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L3991-L3995)*

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

  Create a new lazy DFA builder with the default configuration.

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Build a lazy DFA from the given pattern.

  

  If there was a problem parsing or compiling the pattern, then an error

  is returned.

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError>` — [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Build a lazy DFA from the given patterns.

  

  When matches are returned, the pattern ID corresponds to the index of

  the pattern in the slice given.

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: thompson::NFA) -> Result<DFA, BuildError>` — [`NFA`](../../nfa/thompson/nfa/index.md#nfa), [`DFA`](#dfa), [`BuildError`](../error/index.md#builderror)

  Build a DFA from the given NFA.

  

  Note that this requires owning a `thompson::NFA`. While this may force

  you to clone the NFA, such a clone is not a deep clone. Namely, NFAs

  are defined internally to support shared ownership such that cloning is

  very cheap.

  

  # Example

  

  This example shows how to build a lazy DFA if you already have an NFA

  in hand.

  

  ```rust

  use regex_automata::{

      hybrid::dfa::DFA,

      nfa::thompson,

      HalfMatch, Input,

  };

  

  let haystack = "foo123bar";

  

  // This shows how to set non-default options for building an NFA.

  let nfa = thompson::Compiler::new()

      .configure(thompson::Config::new().shrink(true))

      .build(r"[0-9]+")?;

  let dfa = DFA::builder().build_from_nfa(nfa)?;

  let mut cache = dfa.create_cache();

  let expected = Some(HalfMatch::must(0, 6));

  let got = dfa.try_search_fwd(&mut cache, &Input::new(haystack))?;

  assert_eq!(expected, got);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

  Apply the given lazy DFA configuration options to this builder.

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md#config), [`Builder`](#builder)

  Set the syntax configuration for this builder using

  [`syntax::Config`](crate::util::syntax::Config).

  

  This permits setting things like case insensitivity, Unicode and multi

  line mode.

  

  These settings only apply when constructing a lazy DFA directly from a

  pattern.

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md#config), [`Builder`](#builder)

  Set the Thompson NFA configuration for this builder using

  [`nfa::thompson::Config`](crate::nfa::thompson::Config).

  

  This permits setting things like whether the DFA should match the regex

  in reverse or if additional time should be spent shrinking the size of

  the NFA.

  

  These settings only apply when constructing a DFA directly from a

  pattern.

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4188-4227`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L4188-L4227)*

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

  Create a new overlapping state that begins at the start state of any

  automaton.

- <span id="overlappingstate-get-match"></span>`fn get_match(&self) -> Option<HalfMatch>` — [`HalfMatch`](../../index.md#halfmatch)

  Return the match result of the most recent search to execute with this

  state.

  

  A searches will clear this result automatically, such that if no

  match is found, this will correctly report `None`.

#### Trait Implementations

##### `impl Any for OverlappingState`

- <span id="overlappingstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for OverlappingState`

- <span id="overlappingstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for OverlappingState`

- <span id="overlappingstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for OverlappingState`

- <span id="overlappingstate-clone"></span>`fn clone(&self) -> OverlappingState` — [`OverlappingState`](#overlappingstate)

##### `impl CloneToUninit for OverlappingState`

- <span id="overlappingstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for OverlappingState`

- <span id="overlappingstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for OverlappingState`

##### `impl<T> From for OverlappingState`

- <span id="overlappingstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for OverlappingState`

- <span id="overlappingstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for OverlappingState`

- <span id="overlappingstate-partialeq-eq"></span>`fn eq(&self, other: &OverlappingState) -> bool` — [`OverlappingState`](#overlappingstate)

##### `impl StructuralPartialEq for OverlappingState`

##### `impl ToOwned for OverlappingState`

- <span id="overlappingstate-toowned-type-owned"></span>`type Owned = T`

- <span id="overlappingstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="overlappingstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for OverlappingState`

- <span id="overlappingstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="overlappingstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for OverlappingState`

- <span id="overlappingstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="overlappingstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2819-2832`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L2819-L2832)*

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

  Create an empty state saver.

- <span id="statesaver-take-to-save"></span>`fn take_to_save(&mut self) -> Option<(LazyStateID, State)>` — [`LazyStateID`](../id/index.md#lazystateid), [`State`](../../util/determinize/state/index.md#state)

  Replace this state saver with an empty saver, and if this saver is a

  request to save a state, return that request.

- <span id="statesaver-take-saved"></span>`fn take_saved(&mut self) -> Option<LazyStateID>` — [`LazyStateID`](../id/index.md#lazystateid)

  Replace this state saver with an empty saver, and if this saver is a

  saved state (or a request to save a state), return that state's ID.

  

  The idea here is that a request to save a state isn't necessarily

  honored because it might not be needed. e.g., Some higher level code

  might request a state to be saved on the off chance that the cache gets

  cleared when a new state is added at a lower level. But if that new

  state is never added, then the cache is never cleared and the state and

  its ID remain unchanged.

#### Trait Implementations

##### `impl Any for StateSaver`

- <span id="statesaver-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StateSaver`

- <span id="statesaver-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StateSaver`

- <span id="statesaver-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StateSaver`

- <span id="statesaver-clone"></span>`fn clone(&self) -> StateSaver` — [`StateSaver`](#statesaver)

##### `impl CloneToUninit for StateSaver`

- <span id="statesaver-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for StateSaver`

- <span id="statesaver-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StateSaver`

- <span id="statesaver-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StateSaver`

- <span id="statesaver-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for StateSaver`

- <span id="statesaver-toowned-type-owned"></span>`type Owned = T`

- <span id="statesaver-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="statesaver-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StateSaver`

- <span id="statesaver-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="statesaver-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StateSaver`

- <span id="statesaver-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="statesaver-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `skip_empty_utf8_splits_overlapping`

```rust
fn skip_empty_utf8_splits_overlapping<F>(input: &crate::util::search::Input<'_>, state: &mut OverlappingState, search: F) -> Result<(), crate::util::search::MatchError>
where
    F: FnMut(&crate::util::search::Input<'_>, &mut OverlappingState) -> Result<(), crate::util::search::MatchError>
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4261-4293`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L4261-L4293)*

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4301-4307`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L4301-L4307)*

Based on the minimum number of states required for a useful lazy DFA cache,
this returns the minimum lazy state ID that must be representable.

It's not likely for this to have any impact 32-bit systems (or higher), but
on 16-bit systems, the lazy state ID space is quite constrained and thus
may be insufficient if our MIN_STATES value is (for some reason) too high.

### `minimum_cache_capacity`

```rust
fn minimum_cache_capacity(nfa: &thompson::NFA, classes: &crate::util::alphabet::ByteClasses, starts_for_each_pattern: bool) -> usize
```

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:4335-4392`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L4335-L4392)*

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:2074`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L2074)*

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:42`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L42)*

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

*Defined in [`regex-automata-0.4.13/src/hybrid/dfa.rs:50`](../../../../.source_1765633015/regex-automata-0.4.13/src/hybrid/dfa.rs#L50)*

The number of "sentinel" states that get added to every lazy DFA.

These are special states indicating status conditions of a search: unknown,
dead and quit. These states in particular also use zero NFA states, so
their memory usage is quite small. This is relevant for computing the
minimum memory needed for a lazy DFA cache.

