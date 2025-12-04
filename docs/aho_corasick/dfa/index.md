*[aho_corasick](../index.md) / [dfa](index.md)*

---

# Module `dfa`

Provides direct access to a DFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a DFA directly. Using an `DFA` directly is typically only necessary
when one needs access to the [`Automaton`](aho_corasick/automaton/index.md) trait implementation.

## Structs

### `DFA`

```rust
struct DFA {
    // [REDACTED: Private Fields]
}
```

A DFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using a `DFA` directly is typically only necessary when
one needs access to the [`Automaton`](aho_corasick/automaton/index.md) trait implementation.

This DFA can only be built by first constructing a [`noncontiguous::NFA`](#nfa).
Both [`DFA::new`](#new) and [`Builder::build`](#build) do this for you automatically, but
[`Builder::build_from_noncontiguous`](#build-from-noncontiguous) permits doing it explicitly.

A DFA provides the best possible search performance (in this crate) via two
mechanisms:

* All states use a dense representation for their transitions.
* All failure transitions are pre-computed such that they are never
explicitly handled at search time.

These two facts combined mean that every state transition is performed
using a constant number of instructions. However, this comes at
great cost. The memory usage of a DFA can be quite exorbitant.
It is potentially multiple orders of magnitude greater than a
[`contiguous::NFA`](crate::nfa::contiguous::NFA) for example. In exchange,
a DFA will typically have better search speed than a `contiguous::NFA`, but
not by orders of magnitude.

Unless you have a small number of patterns or memory usage is not a concern
and search performance is critical, a DFA is usually not the best choice.

Moreover, unlike the NFAs in this crate, it is costly for a DFA to
support for anchored and unanchored search configurations. Namely,
since failure transitions are pre-computed, supporting both anchored
and unanchored searches requires a duplication of the transition table,
making the memory usage of such a DFA ever bigger. (The NFAs in this crate
unconditionally support both anchored and unanchored searches because there
is essentially no added cost for doing so.) It is for this reason that
a DFA's support for anchored and unanchored searches can be configured
via [`Builder::start_kind`](#start-kind). By default, a DFA only supports unanchored
searches.

# Example

This example shows how to build an `DFA` directly and use it to execute

use aho_corasick::{
    automaton::Automaton,
    dfa::DFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = DFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`](aho_corasick/automaton/index.md) documentation for an example.

#### Implementations

- `fn new<I, P>(patterns: I) -> Result<DFA, BuildError>`
  Create a new Aho-Corasick DFA using the default configuration.

- `fn builder() -> Builder`
  A convenience method for returning a new Aho-Corasick DFA builder.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Automaton`

- `fn start_state(self: &Self, anchored: Anchored) -> Result<StateID, MatchError>`

- `fn next_state(self: &Self, _anchored: Anchored, sid: StateID, byte: u8) -> StateID`

- `fn is_special(self: &Self, sid: StateID) -> bool`

- `fn is_dead(self: &Self, sid: StateID) -> bool`

- `fn is_match(self: &Self, sid: StateID) -> bool`

- `fn is_start(self: &Self, sid: StateID) -> bool`

- `fn match_kind(self: &Self) -> MatchKind`

- `fn patterns_len(self: &Self) -> usize`

- `fn pattern_len(self: &Self, pid: PatternID) -> usize`

- `fn min_pattern_len(self: &Self) -> usize`

- `fn max_pattern_len(self: &Self) -> usize`

- `fn match_len(self: &Self, sid: StateID) -> usize`

- `fn match_pattern(self: &Self, sid: StateID, index: usize) -> PatternID`

- `fn memory_usage(self: &Self) -> usize`

- `fn prefilter(self: &Self) -> Option<&Prefilter>`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

A builder for configuring an Aho-Corasick DFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- `fn new() -> Builder`
  Create a new builder for configuring an Aho-Corasick DFA.

- `fn build<I, P>(self: &Self, patterns: I) -> Result<DFA, BuildError>`
  Build an Aho-Corasick DFA from the given iterator of patterns.

- `fn build_from_noncontiguous(self: &Self, nnfa: &noncontiguous::NFA) -> Result<DFA, BuildError>`
  Build an Aho-Corasick DFA from the given noncontiguous NFA.

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder`
  Set the desired match semantics.

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder`
  Enable ASCII-aware case insensitive matching.

- `fn prefilter(self: &mut Self, yes: bool) -> &mut Builder`
  Enable heuristic prefilter optimizations.

- `fn start_kind(self: &mut Self, kind: StartKind) -> &mut Builder`
  Sets the starting state configuration for the automaton.

- `fn byte_classes(self: &mut Self, yes: bool) -> &mut Builder`
  A debug setting for whether to attempt to shrink the size of the

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

##### `impl Default`

- `fn default() -> Builder`

