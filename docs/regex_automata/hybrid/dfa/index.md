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

```
use regex_automata::{hybrid::dfa::DFA, HalfMatch, Input};

let dfa = DFA::new("foo[0-9]+")?;
let mut cache = dfa.create_cache();

let expected = Some(HalfMatch::must(0, 8));
assert_eq!(expected, dfa.try_search_fwd(
    &mut cache, &Input::new("foo12345"))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn next_state(self: &Self, cache: &mut Cache, current: LazyStateID, input: u8) -> Result<LazyStateID, CacheError>`
  Transitions from the current state to the next state, given the next

- `fn next_state_untagged(self: &Self, cache: &Cache, current: LazyStateID, input: u8) -> LazyStateID`
  Transitions from the current state to the next state, given the next

- `unsafe fn next_state_untagged_unchecked(self: &Self, cache: &Cache, current: LazyStateID, input: u8) -> LazyStateID`
  Transitions from the current state to the next state, eliding bounds

- `fn next_eoi_state(self: &Self, cache: &mut Cache, current: LazyStateID) -> Result<LazyStateID, CacheError>`
  Transitions from the current state to the next state for the special

- `fn start_state(self: &Self, cache: &mut Cache, config: &start::Config) -> Result<LazyStateID, StartError>`
  Return the ID of the start state for this lazy DFA for the given

- `fn start_state_forward(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<LazyStateID, MatchError>`
  Return the ID of the start state for this lazy DFA when executing a

- `fn start_state_reverse(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<LazyStateID, MatchError>`
  Return the ID of the start state for this lazy DFA when executing a

- `fn match_len(self: &Self, cache: &Cache, id: LazyStateID) -> usize`
  Returns the total number of patterns that match in this state.

- `fn match_pattern(self: &Self, cache: &Cache, id: LazyStateID, match_index: usize) -> PatternID`
  Returns the pattern ID corresponding to the given match index in the

- `fn try_search_fwd(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>`
  Executes a forward search and returns the end position of the leftmost

- `fn try_search_rev(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>`
  Executes a reverse search and returns the start of the position of the

- `fn try_search_overlapping_fwd(self: &Self, cache: &mut Cache, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>`
  Executes an overlapping forward search and returns the end position of

- `fn try_search_overlapping_rev(self: &Self, cache: &mut Cache, input: &Input<'_>, state: &mut OverlappingState) -> Result<(), MatchError>`
  Executes a reverse overlapping search and returns the start of the

- `fn try_which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet) -> Result<(), MatchError>`
  Writes the set of patterns that match anywhere in the given search

- `fn new(pattern: &str) -> Result<DFA, BuildError>`
  Parse the given regular expression using a default configuration and

- `fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError>`
  Parse the given regular expressions using a default configuration and

- `fn always_match() -> Result<DFA, BuildError>`
  Create a new lazy DFA that matches every input.

- `fn never_match() -> Result<DFA, BuildError>`
  Create a new lazy DFA that never matches any input.

- `fn config() -> Config`
  Return a default configuration for a `DFA`.

- `fn builder() -> Builder`
  Return a builder for configuring the construction of a `Regex`.

- `fn create_cache(self: &Self) -> Cache`
  Create a new cache for this lazy DFA.

- `fn reset_cache(self: &Self, cache: &mut Cache)`
  Reset the given cache such that it can be used for searching with the

- `fn pattern_len(self: &Self) -> usize`
  Returns the total number of patterns compiled into this lazy DFA.

- `fn byte_classes(self: &Self) -> &ByteClasses`
  Returns the equivalence classes that make up the alphabet for this DFA.

- `fn get_config(self: &Self) -> &Config`
  Returns this lazy DFA's configuration.

- `fn get_nfa(self: &Self) -> &thompson::NFA`
  Returns a reference to the underlying NFA.

- `fn memory_usage(self: &Self) -> usize`
  Returns the memory usage, in bytes, of this lazy DFA.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> DFA`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Cache`

```rust
struct Cache {
}
```

A cache represents a partially computed DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`DFA`](regex_automata/dfa/onepass/index.md).

Caches can be created from their corresponding DFA via
[`DFA::create_cache`](#create-cache). A cache can only be used with either the DFA that
created it, or the DFA that was most recently used to reset it with
[`Cache::reset`](#reset). Using a cache with any other DFA may result in panics
or incorrect results.

#### Implementations

- `fn new(dfa: &DFA) -> Cache`
  Create a new cache for the given lazy DFA.

- `fn reset(self: &mut Self, dfa: &DFA)`
  Reset this cache such that it can be used for searching with the given

- `fn search_start(self: &mut Self, at: usize)`
  Initializes a new search starting at the given position.

- `fn search_update(self: &mut Self, at: usize)`
  Updates the current search to indicate that it has search to the

- `fn search_finish(self: &mut Self, at: usize)`
  Indicates that a search has finished at the given position.

- `fn search_total_len(self: &Self) -> usize`
  Returns the total number of bytes that have been searched since this

- `fn clear_count(self: &Self) -> usize`
  Returns the total number of times this cache has been cleared since it

- `fn memory_usage(self: &Self) -> usize`
  Returns the heap memory usage, in bytes, of this cache.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Cache`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Config`

```rust
struct Config {
}
```

The configuration used for building a lazy DFA.

As a convenience, [`DFA::config`](#config) is an alias for [`Config::new`](#new). The
advantage of the former is that it often lets you avoid importing the
`Config` type directly.

A lazy DFA configuration is a simple data object that is typically used
with [`Builder::configure`](#configure).

The default configuration guarantees that a search will never return a
"gave up" or "quit" error, although it is possible for a search to fail
if [`Config::starts_for_each_pattern`](#starts-for-each-pattern) wasn't enabled (which it is not by
default) and an [`Anchored::Pattern`](#pattern) mode is requested via [`Input`](#input).

#### Implementations

- `fn new() -> Config`
  Return a new default lazy DFA builder configuration.

- `fn match_kind(self: Self, kind: MatchKind) -> Config`
  Set the desired match semantics.

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config`
  Set a prefilter to be used whenever a start state is entered.

- `fn starts_for_each_pattern(self: Self, yes: bool) -> Config`
  Whether to compile a separate start state for each pattern in the

- `fn byte_classes(self: Self, yes: bool) -> Config`
  Whether to attempt to shrink the size of the lazy DFA's alphabet or

- `fn unicode_word_boundary(self: Self, yes: bool) -> Config`
  Heuristically enable Unicode word boundaries.

- `fn quit(self: Self, byte: u8, yes: bool) -> Config`
  Add a "quit" byte to the lazy DFA.

- `fn specialize_start_states(self: Self, yes: bool) -> Config`
  Enable specializing start states in the lazy DFA.

- `fn cache_capacity(self: Self, bytes: usize) -> Config`
  Sets the maximum amount of heap memory, in bytes, to allocate to the

- `fn skip_cache_capacity_check(self: Self, yes: bool) -> Config`
  Configures construction of a lazy DFA to use the minimum cache capacity

- `fn minimum_cache_clear_count(self: Self, min: Option<usize>) -> Config`
  Configure a lazy DFA search to quit after a certain number of cache

- `fn minimum_bytes_per_state(self: Self, min: Option<usize>) -> Config`
  Configure a lazy DFA search to quit only when its efficiency drops

- `fn get_match_kind(self: &Self) -> MatchKind`
  Returns the match semantics set in this configuration.

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>`
  Returns the prefilter set in this configuration, if one at all.

- `fn get_starts_for_each_pattern(self: &Self) -> bool`
  Returns whether this configuration has enabled anchored starting states

- `fn get_byte_classes(self: &Self) -> bool`
  Returns whether this configuration has enabled byte classes or not.

- `fn get_unicode_word_boundary(self: &Self) -> bool`
  Returns whether this configuration has enabled heuristic Unicode word

- `fn get_quit(self: &Self, byte: u8) -> bool`
  Returns whether this configuration will instruct the lazy DFA to enter

- `fn get_specialize_start_states(self: &Self) -> bool`
  Returns whether this configuration will instruct the lazy DFA to

- `fn get_cache_capacity(self: &Self) -> usize`
  Returns the cache capacity set on this configuration.

- `fn get_skip_cache_capacity_check(self: &Self) -> bool`
  Returns whether the cache capacity check should be skipped.

- `fn get_minimum_cache_clear_count(self: &Self) -> Option<usize>`
  Returns, if set, the minimum number of times the cache must be cleared

- `fn get_minimum_bytes_per_state(self: &Self) -> Option<usize>`
  Returns, if set, the minimum number of bytes per state that need to be

- `fn get_minimum_cache_capacity(self: &Self, nfa: &thompson::NFA) -> Result<usize, BuildError>`
  Returns the minimum lazy DFA cache capacity required for the given NFA.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Config`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Config`

### `Builder`

```rust
struct Builder {
}
```

A builder for constructing a lazy deterministic finite automaton from
regular expressions.

As a convenience, [`DFA::builder`](#builder) is an alias for [`Builder::new`](#new). The
advantage of the former is that it often lets you avoid importing the
`Builder` type directly.

This builder provides two main things:

1. It provides a few different `build` routines for actually constructing
a DFA from different kinds of inputs. The most convenient is
[`Builder::build`](#build), which builds a DFA directly from a pattern string. The
most flexible is [`Builder::build_from_nfa`](#build-from-nfa), which builds a DFA straight
from an NFA.
2. The builder permits configuring a number of things.
[`Builder::configure`](#configure) is used with [`Config`](regex_automata/dfa/onepass/index.md) to configure aspects of
the DFA and the construction process itself. [`Builder::syntax`](#syntax) and
[`Builder::thompson`](#thompson) permit configuring the regex parser and Thompson NFA
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

```
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

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder`
  Create a new lazy DFA builder with the default configuration.

- `fn build(self: &Self, pattern: &str) -> Result<DFA, BuildError>`
  Build a lazy DFA from the given pattern.

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<DFA, BuildError>`
  Build a lazy DFA from the given patterns.

- `fn build_from_nfa(self: &Self, nfa: thompson::NFA) -> Result<DFA, BuildError>`
  Build a DFA from the given NFA.

- `fn configure(self: &mut Self, config: Config) -> &mut Builder`
  Apply the given lazy DFA configuration options to this builder.

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder`
  Set the syntax configuration for this builder using

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder`
  Set the Thompson NFA configuration for this builder using

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Builder`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `OverlappingState`

```rust
struct OverlappingState {
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
[`OverlappingState::start`](#start) when starting a new search. Reusing state from
a previous search may result in incorrect results.

#### Implementations

- `fn start() -> OverlappingState`
  Create a new overlapping state that begins at the start state of any

- `fn get_match(self: &Self) -> Option<HalfMatch>`
  Return the match result of the most recent search to execute with this

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> OverlappingState`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &OverlappingState) -> bool`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

