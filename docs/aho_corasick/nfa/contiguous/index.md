*[aho_corasick](../../index.md) / [nfa](../index.md) / [contiguous](index.md)*

---

# Module `contiguous`

Provides a contiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a contiguous NFA directly. Using an `NFA` directly is typically only
necessary when one needs access to the [`Automaton`](aho_corasick/automaton/index.md) trait implementation.

## Structs

### `NFA`

```rust
struct NFA {
    // [REDACTED: Private Fields]
}
```

A contiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`](aho_corasick/automaton/index.md) trait implementation.

This NFA can only be built by first constructing a [`noncontiguous::NFA`](#nfa).
Both [`NFA::new`](#new) and [`Builder::build`](#build) do this for you automatically, but
[`Builder::build_from_noncontiguous`](#build-from-noncontiguous) permits doing it explicitly.

The main difference between a noncontiguous NFA and a contiguous NFA is
that the latter represents all of its states and transitions in a single
allocation, where as the former uses a separate allocation for each state.
Doing this at construction time while keeping a low memory footprint isn't
feasible, which is primarily why there are two different NFA types: one
that does the least amount of work possible to build itself, and another
that does a little extra work to compact itself and make state transitions
faster by making some states use a dense representation.

Because a contiguous NFA uses a single allocation, there is a lot more
opportunity for compression tricks to reduce the heap memory used. Indeed,
it is not uncommon for a contiguous NFA to use an order of magnitude less
heap memory than a noncontiguous NFA. Since building a contiguous NFA
usually only takes a fraction of the time it takes to build a noncontiguous
NFA, the overall build time is not much slower. Thus, in most cases, a
contiguous NFA is the best choice.

Since a contiguous NFA uses various tricks for compression and to achieve
faster state transitions, currently, its limit on the number of states
is somewhat smaller than what a noncontiguous NFA can achieve. Generally
speaking, you shouldn't expect to run into this limit if the number of
patterns is under 1 million. It is plausible that this limit will be
increased in the future. If the limit is reached, building a contiguous NFA
will return an error. Often, since building a contiguous NFA is relatively
cheap, it can make sense to always try it even if you aren't sure if it
will fail or not. If it does, you can always fall back to a noncontiguous
NFA. (Indeed, the main [`AhoCorasick`](crate::AhoCorasick) type employs a
strategy similar to this at construction time.)

# Example

This example shows how to build an `NFA` directly and use it to execute

use aho_corasick::{
    automaton::Automaton,
    nfa::contiguous::NFA,
    Input, Match,
};

let patterns = &["b", "abc", "abcd"];
let haystack = "abcd";

let nfa = NFA::new(patterns).unwrap();
assert_eq!(
    Some(Match::must(0, 1..2)),
    nfa.try_find(&Input::new(haystack))?,
);
# Ok::<(), Box<dyn std::error::Error>>(())
```

It is also possible to implement your own version of `try_find`. See the
[`Automaton`](aho_corasick/automaton/index.md) documentation for an example.

#### Implementations

- `fn new<I, P>(patterns: I) -> Result<NFA, BuildError>`
  Create a new Aho-Corasick contiguous NFA using the default

- `fn builder() -> Builder`
  A convenience method for returning a new Aho-Corasick contiguous NFA

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

- `fn next_state(self: &Self, anchored: Anchored, sid: StateID, byte: u8) -> StateID`

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

- `fn clone(self: &Self) -> NFA`

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

A builder for configuring an Aho-Corasick contiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- `fn new() -> Builder`
  Create a new builder for configuring an Aho-Corasick contiguous NFA.

- `fn build<I, P>(self: &Self, patterns: I) -> Result<NFA, BuildError>`
  Build an Aho-Corasick contiguous NFA from the given iterator of

- `fn build_from_noncontiguous(self: &Self, nnfa: &noncontiguous::NFA) -> Result<NFA, BuildError>`
  Build an Aho-Corasick contiguous NFA from the given noncontiguous NFA.

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder`
  Set the desired match semantics.

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder`
  Enable ASCII-aware case insensitive matching.

- `fn prefilter(self: &mut Self, yes: bool) -> &mut Builder`
  Enable heuristic prefilter optimizations.

- `fn dense_depth(self: &mut Self, depth: usize) -> &mut Builder`
  Set the limit on how many states use a dense representation for their

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

