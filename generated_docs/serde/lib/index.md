*[serde](../index.md) / [lib](index.md)*

---

# Module `lib`

A facade around all the types we need from the `std`, `core`, and `alloc`
crates. This avoids elaborate import wrangling having to happen in every
module.

## Contents

- [Modules](#modules)
  - [`core`](#core)
- [Structs](#structs)
  - [`ptr`](#ptr)
  - [`default`](#default)
- [Functions](#functions)
  - [`convert`](#convert)
  - [`fmt`](#fmt)
  - [`FmtWrite`](#fmtwrite)
  - [`PhantomData`](#phantomdata)
  - [`result`](#result)
  - [`String`](#string)
  - [`ToString`](#tostring)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`core`](#core) | mod |  |
| [`ptr`](#ptr) | struct |  |
| [`default`](#default) | struct |  |
| [`convert`](#convert) | fn |  |
| [`fmt`](#fmt) | fn |  |
| [`FmtWrite`](#fmtwrite) | fn |  |
| [`PhantomData`](#phantomdata) | fn |  |
| [`result`](#result) | fn |  |
| [`String`](#string) | fn |  |
| [`ToString`](#tostring) | fn |  |

## Modules

- [`core`](core/index.md)

## Structs

### `ptr<'a>`

```rust
struct ptr<'a> {
    pub file: Option<&'a str>,
    pub line: Option<u32>,
    pub column: Option<u32>,
}
```

*Defined in [`addr2line-0.25.1/src/frame.rs:8-17`](../../../.source_1765521767/addr2line-0.25.1/src/frame.rs#L8-L17)*

*Re-exported from `addr2line`*

A source location.

#### Fields

- **`file`**: `Option<&'a str>`

  The file name.

- **`line`**: `Option<u32>`

  The line number.

- **`column`**: `Option<u32>`

  The column number.
  
  A value of `Some(0)` indicates the left edge.

### `default`

```rust
struct default {
    trans: alloc::vec::Vec<crate::util::primitives::StateID>,
    matches: alloc::vec::Vec<alloc::vec::Vec<crate::util::primitives::PatternID>>,
    matches_memory_usage: usize,
    pattern_lens: alloc::vec::Vec<crate::util::primitives::SmallIndex>,
    prefilter: Option<crate::util::prefilter::Prefilter>,
    match_kind: crate::util::search::MatchKind,
    state_len: usize,
    alphabet_len: usize,
    stride2: usize,
    byte_classes: crate::util::alphabet::ByteClasses,
    min_pattern_len: usize,
    max_pattern_len: usize,
    special: crate::util::special::Special,
}
```

*Defined in [`aho-corasick-1.1.4/src/dfa.rs:91-132`](../../../.source_1765521767/aho-corasick-1.1.4/src/dfa.rs#L91-L132)*

*Re-exported from `aho_corasick`*

A DFA implementation of Aho-Corasick.

When possible, prefer using [`AhoCorasick`](crate::AhoCorasick) instead of
this type directly. Using a `DFA` directly is typically only necessary when
one needs access to the `Automaton` trait implementation.

This DFA can only be built by first constructing a [`noncontiguous::NFA`](#noncontiguousnfa).
Both [`DFA::new`](../../addr2line/index.md) and `Builder::build` do this for you automatically, but
[`Builder::build_from_noncontiguous`](../../clap_builder/index.md) permits doing it explicitly.

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
via [`Builder::start_kind`](../../aho_corasick/util/error/index.md). By default, a DFA only supports unanchored
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
```rust

It is also possible to implement your own version of `try_find`. See the
`Automaton` documentation for an example.

#### Fields

- **`trans`**: `alloc::vec::Vec<crate::util::primitives::StateID>`

  The DFA transition table. IDs in this table are pre-multiplied. So
  instead of the IDs being 0, 1, 2, 3, ..., they are 0*stride, 1*stride,
  2*stride, 3*stride, ...

- **`matches`**: `alloc::vec::Vec<alloc::vec::Vec<crate::util::primitives::PatternID>>`

  The matches for every match state in this DFA. This is first indexed by
  state index (so that's `sid >> stride2`) and then by order in which the
  matches are meant to occur.

- **`matches_memory_usage`**: `usize`

  The amount of heap memory used, in bytes, by the inner Vecs of
  'matches'.

- **`pattern_lens`**: `alloc::vec::Vec<crate::util::primitives::SmallIndex>`

  The length of each pattern. This is used to compute the start offset
  of a match.

- **`prefilter`**: `Option<crate::util::prefilter::Prefilter>`

  A prefilter for accelerating searches, if one exists.

- **`match_kind`**: `crate::util::search::MatchKind`

  The match semantics built into this DFA.

- **`state_len`**: `usize`

  The total number of states in this DFA.

- **`alphabet_len`**: `usize`

  The alphabet size, or total number of equivalence classes, for this
  DFA. Note that the actual number of transitions in each state is
  stride=2^stride2, where stride is the smallest power of 2 greater than
  or equal to alphabet_len. We do things this way so that we can use
  bitshifting to go from a state ID to an index into 'matches'.

- **`stride2`**: `usize`

  The exponent with a base 2, such that stride=2^stride2. Given a state
  index 'i', its state identifier is 'i << stride2'. Given a state
  identifier 'sid', its state index is 'sid >> stride2'.

- **`byte_classes`**: `crate::util::alphabet::ByteClasses`

  The equivalence classes for this DFA. All transitions are defined on
  equivalence classes and not on the 256 distinct byte values.

- **`min_pattern_len`**: `usize`

  The length of the shortest pattern in this automaton.

- **`max_pattern_len`**: `usize`

  The length of the longest pattern in this automaton.

- **`special`**: `crate::util::special::Special`

  The information required to deduce which states are "special" in this
  DFA.

#### Implementations

- <span id="dfa-const-dead"></span>`const DEAD: StateID`

- <span id="dfa-set-matches"></span>`fn set_matches(&mut self, sid: StateID, pids: impl Iterator<Item = PatternID>)`

- <span id="dfa-new"></span>`fn new<I, P>(patterns: I) -> Result<DFA, BuildError>` — [`FmtWrite`](#fmtwrite), [`default`](#default), [`FmtWrite`](#fmtwrite)

- <span id="dfa-builder"></span>`fn builder() -> Builder`

#### Trait Implementations

##### `impl Automaton for DFA`

- <span id="dfa-start-state"></span>`fn start_state(&self, anchored: Anchored) -> Result<StateID, MatchError>` — [`FmtWrite`](#fmtwrite)

- <span id="dfa-next-state"></span>`fn next_state(&self, _anchored: Anchored, sid: StateID, byte: u8) -> StateID`

- <span id="dfa-is-special"></span>`fn is_special(&self, sid: StateID) -> bool`

- <span id="dfa-is-dead"></span>`fn is_dead(&self, sid: StateID) -> bool`

- <span id="dfa-is-match"></span>`fn is_match(&self, sid: StateID) -> bool`

- <span id="dfa-is-start"></span>`fn is_start(&self, sid: StateID) -> bool`

- <span id="dfa-match-kind"></span>`fn match_kind(&self) -> MatchKind`

- <span id="dfa-patterns-len"></span>`fn patterns_len(&self) -> usize`

- <span id="dfa-pattern-len"></span>`fn pattern_len(&self, pid: PatternID) -> usize`

- <span id="dfa-min-pattern-len"></span>`fn min_pattern_len(&self) -> usize`

- <span id="dfa-max-pattern-len"></span>`fn max_pattern_len(&self) -> usize`

- <span id="dfa-match-len"></span>`fn match_len(&self, sid: StateID) -> usize`

- <span id="dfa-match-pattern"></span>`fn match_pattern(&self, sid: StateID, index: usize) -> PatternID`

- <span id="dfa-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="dfa-prefilter"></span>`fn prefilter(&self) -> Option<&Prefilter>` — [`Cow`](#cow)

##### `impl Clone for DFA`

- <span id="dfa-clone"></span>`fn clone(&self) -> DFA` — [`default`](#default)

##### `impl Debug for DFA`

- <span id="dfa-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Sealed for crate::dfa::DFA`

## Functions

### `convert`

```rust
fn convert(&self) -> &T
```

### `fmt`

```rust
fn fmt(&mut self) -> &mut T
```

### `FmtWrite`

```rust
fn FmtWrite(self) -> U
```

Calls `U::from(self)`.

That is, this conversion is whatever the implementation of
<code>[From]&lt;T&gt; for U</code> chooses to do.

### `PhantomData`

```rust
fn PhantomData(t: T) -> T
```

Returns the argument unchanged.

### `result`

```rust
fn result(self) -> Result<U, <U as TryFrom>::Error>
```

### `String`

```rust
fn String(value: U) -> Result<T, <T as TryFrom>::Error>
```

### `ToString`

```rust
fn ToString(&self) -> TypeId
```

