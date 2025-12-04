*[aho_corasick](../../index.md) / [nfa](../index.md) / [noncontiguous](index.md)*

---

# Module `noncontiguous`

Provides a noncontiguous NFA implementation of Aho-Corasick.

This is a low-level API that generally only needs to be used in niche
circumstances. When possible, prefer using [`AhoCorasick`](crate::AhoCorasick)
instead of a noncontiguous NFA directly. Using an `NFA` directly is typically
only necessary when one needs access to the [`Automaton`](automaton/index.md) trait implementation.

## Structs

### `NFA`

```rust
struct NFA {
    // [REDACTED: Private Fields]
}
```

A noncontiguous NFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using an `NFA` directly is typically only necessary
when one needs access to the [`Automaton`](automaton/index.md) trait implementation.

This NFA represents the "core" implementation of Aho-Corasick in this
crate. Namely, constructing this NFA involving building a trie and then
filling in the failure transitions between states, similar to what is
described in any standard textbook description of Aho-Corasick.

In order to minimize heap usage and to avoid additional construction costs,
this implementation represents the transitions of all states as distinct
sparse memory allocations. This is where it gets its name from. That is,
this NFA has no contiguous memory allocation for its transition table. Each
state gets its own allocation.

While the sparse representation keeps memory usage to somewhat reasonable
levels, it is still quite large and also results in somewhat mediocre
search performance. For this reason, it is almost always a good idea to
use a [`contiguous::NFA`](crate::nfa::contiguous::NFA) instead. It is
marginally slower to build, but has higher throughput and can sometimes use
an order of magnitude less memory. The main reason to use a noncontiguous
NFA is when you need the fastest possible construction time, or when a
contiguous NFA does not have the desired capacity. (The total number of NFA
states it can have is fewer than a noncontiguous NFA.)

# Example

This example shows how to build an `NFA` directly and use it to execute

use aho_corasick::{
    automaton::Automaton,
    nfa::noncontiguous::NFA,
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
[`Automaton`](automaton/index.md) documentation for an example.

#### Implementations

- `fn new<I, P>(patterns: I) -> Result<NFA, BuildError>`
  Create a new Aho-Corasick noncontiguous NFA using the default

- `fn builder() -> Builder`
  A convenience method for returning a new Aho-Corasick noncontiguous NFA

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

A builder for configuring an Aho-Corasick noncontiguous NFA.

This builder has a subset of the options available to a
[`AhoCorasickBuilder`](crate::AhoCorasickBuilder). Of the shared options,
their behavior is identical.

#### Implementations

- `fn new() -> Builder`
  Create a new builder for configuring an Aho-Corasick noncontiguous NFA.

- `fn build<I, P>(self: &Self, patterns: I) -> Result<NFA, BuildError>`
  Build an Aho-Corasick noncontiguous NFA from the given iterator of

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Builder`
  Set the desired match semantics.

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut Builder`
  Enable ASCII-aware case insensitive matching.

- `fn dense_depth(self: &mut Self, depth: usize) -> &mut Builder`
  Set the limit on how many states use a dense representation for their

- `fn prefilter(self: &mut Self, yes: bool) -> &mut Builder`
  Enable heuristic prefilter optimizations.

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

