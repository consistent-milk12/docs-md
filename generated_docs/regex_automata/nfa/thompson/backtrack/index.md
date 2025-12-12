*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [backtrack](index.md)*

---

# Module `backtrack`

An NFA backed bounded backtracker for executing regex searches with capturing
groups.

This module provides a [`BoundedBacktracker`](#boundedbacktracker) that works by simulating an NFA
using the classical backtracking algorithm with a twist: it avoids redoing
work that it has done before and thereby avoids worst case exponential time.
In exchange, it can only be used on "short" haystacks. Its advantage is that
is can be faster than the [`PikeVM`](thompson::pikevm::PikeVM) in many cases
because it does less book-keeping.

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`BoundedBacktracker`](#boundedbacktracker)
  - [`TryFindMatches`](#tryfindmatches)
  - [`TryCapturesMatches`](#trycapturesmatches)
  - [`Cache`](#cache)
  - [`Visited`](#visited)
- [Enums](#enums)
  - [`Frame`](#frame)
- [Functions](#functions)
  - [`min_visited_capacity`](#min-visited-capacity)
  - [`div_ceil`](#div-ceil)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration used for building a bounded backtracker. |
| [`Builder`](#builder) | struct | A builder for a bounded backtracker. |
| [`BoundedBacktracker`](#boundedbacktracker) | struct | A backtracking regex engine that bounds its execution to avoid exponential blow-up. |
| [`TryFindMatches`](#tryfindmatches) | struct | An iterator over all non-overlapping matches for a fallible search. |
| [`TryCapturesMatches`](#trycapturesmatches) | struct | An iterator over all non-overlapping leftmost matches, with their capturing groups, for a fallible search. |
| [`Cache`](#cache) | struct | A cache represents mutable state that a [`BoundedBacktracker`] requires during a search. |
| [`Visited`](#visited) | struct | A bitset that keeps track of whether a particular (StateID, offset) has been considered during backtracking. |
| [`Frame`](#frame) | enum | Represents a stack frame on the heap while doing backtracking. |
| [`min_visited_capacity`](#min-visited-capacity) | fn | Returns the minimum visited capacity for the given haystack. |
| [`div_ceil`](#div-ceil) | fn | Integer division, but rounds up instead of down. |

## Structs

### `Config`

```rust
struct Config {
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
    visited_capacity: Option<usize>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:50-53`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L50-L53)*

The configuration used for building a bounded backtracker.

A bounded backtracker configuration is a simple data object that is
typically used with `Builder::configure`.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-prefilter"></span>`fn prefilter(self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../../util/prefilter/index.md#prefilter), [`Config`](#config)

- <span id="config-visited-capacity"></span>`fn visited_capacity(self, capacity: usize) -> Config` — [`Config`](#config)

- <span id="config-get-prefilter"></span>`fn get_prefilter(&self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md#prefilter)

- <span id="config-get-visited-capacity"></span>`fn get_visited_capacity(&self) -> usize`

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

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:256-260`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L256-L260)*

A builder for a bounded backtracker.

This builder permits configuring options for the syntax of a pattern, the
NFA construction and the `BoundedBacktracker` construction. This builder
is different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* `thompson::Config::utf8` controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```rust
use regex_automata::{
    nfa::thompson::{self, backtrack::BoundedBacktracker},
    util::syntax,
    Match,
};

let re = BoundedBacktracker::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Ok(Match::must(0, 1..9)));
let got = re.try_find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a BoundedBacktracker Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap()?.range()]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="builder-build-from-nfa"></span>`fn build_from_nfa(&self, nfa: NFA) -> Result<BoundedBacktracker, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="builder-configure"></span>`fn configure(&mut self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../../util/syntax/index.md#config), [`Builder`](#builder)

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../compiler/index.md#config), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `BoundedBacktracker`

```rust
struct BoundedBacktracker {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:427-430`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L427-L430)*

A backtracking regex engine that bounds its execution to avoid exponential
blow-up.

This regex engine only implements leftmost-first match semantics and
only supports leftmost searches. It effectively does the same thing as a
[`PikeVM`](thompson::pikevm::PikeVM), but typically does it faster because
it doesn't have to worry about copying capturing group spans for most NFA
states. Instead, the backtracker can maintain one set of captures (provided
by the caller) and never needs to copy them. In exchange, the backtracker
bounds itself to ensure it doesn't exhibit worst case exponential time.
This results in the backtracker only being able to handle short haystacks
given reasonable memory usage.

# Searches may return an error!

By design, this backtracking regex engine is bounded. This bound is
implemented by not visiting any combination of NFA state ID and position
in a haystack more than once. Thus, the total memory required to bound
backtracking is proportional to `haystack.len() * nfa.states().len()`.
This can obviously get quite large, since large haystacks aren't terribly
uncommon. To avoid using exorbitant memory, the capacity is bounded by
a fixed limit set via `Config::visited_capacity`. Thus, if the total
capacity required for a particular regex and a haystack exceeds this
capacity, then the search routine will return an error.

Unlike other regex engines that may return an error at search time (like
the DFA or the hybrid NFA/DFA), there is no way to guarantee that a bounded
backtracker will work for every haystack. Therefore, this regex engine
_only_ exposes fallible search routines to avoid the footgun of panicking
when running a search on a haystack that is too big.

If one wants to use the fallible search APIs without handling the
error, the only way to guarantee an error won't occur from the
haystack length is to ensure the haystack length does not exceed
`BoundedBacktracker::max_haystack_len`.

# Example: Unicode word boundaries

This example shows that the bounded backtracker implements Unicode word
boundaries correctly by default.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Ok(Match::must(0, 0..12))), it.next());
assert_eq!(Some(Ok(Match::must(0, 13..23))), it.next());
assert_eq!(None, it.next());
Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: multiple regex patterns

The bounded backtracker supports searching for multiple patterns
simultaneously, just like other regex engines. Note though that because it
uses a backtracking strategy, this regex engine is unlikely to scale well
as more patterns are added. But then again, as more patterns are added, the
maximum haystack length allowed will also shorten (assuming the visited
capacity remains invariant).

```rust
use regex_automata::{nfa::thompson::backtrack::BoundedBacktracker, Match};

let re = BoundedBacktracker::new_many(&["[a-z]+", "[0-9]+"])?;
let mut cache = re.create_cache();

let mut it = re.try_find_iter(&mut cache, "abc 1 foo 4567 0 quux");
assert_eq!(Some(Ok(Match::must(0, 0..3))), it.next());
assert_eq!(Some(Ok(Match::must(1, 4..5))), it.next());
assert_eq!(Some(Ok(Match::must(0, 6..9))), it.next());
assert_eq!(Some(Ok(Match::must(1, 10..14))), it.next());
assert_eq!(Some(Ok(Match::must(1, 15..16))), it.next());
assert_eq!(Some(Ok(Match::must(0, 17..21))), it.next());
assert_eq!(None, it.next());
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="boundedbacktracker-new"></span>`fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktracker-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktracker-new-from-nfa"></span>`fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError>` — [`NFA`](../nfa/index.md#nfa), [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktracker-always-match"></span>`fn always_match() -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktracker-never-match"></span>`fn never_match() -> Result<BoundedBacktracker, BuildError>` — [`BoundedBacktracker`](#boundedbacktracker), [`BuildError`](../error/index.md#builderror)

- <span id="boundedbacktracker-config"></span>`fn config() -> Config` — [`Config`](#config)

- <span id="boundedbacktracker-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

- <span id="boundedbacktracker-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

- <span id="boundedbacktracker-create-captures"></span>`fn create_captures(&self) -> Captures` — [`Captures`](../../../util/captures/index.md#captures)

- <span id="boundedbacktracker-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

- <span id="boundedbacktracker-pattern-len"></span>`fn pattern_len(&self) -> usize`

- <span id="boundedbacktracker-get-config"></span>`fn get_config(&self) -> &Config` — [`Config`](#config)

- <span id="boundedbacktracker-get-nfa"></span>`fn get_nfa(&self) -> &NFA` — [`NFA`](../nfa/index.md#nfa)

- <span id="boundedbacktracker-max-haystack-len"></span>`fn max_haystack_len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for BoundedBacktracker`

- <span id="boundedbacktracker-clone"></span>`fn clone(&self) -> BoundedBacktracker` — [`BoundedBacktracker`](#boundedbacktracker)

##### `impl Debug for BoundedBacktracker`

- <span id="boundedbacktracker-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TryFindMatches<'r, 'c, 'h>`

```rust
struct TryFindMatches<'r, 'c, 'h> {
    re: &'r BoundedBacktracker,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1572-1577`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1572-L1577)*

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `BoundedBacktracker::try_find_iter`
method.

#### Trait Implementations

##### `impl Debug for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tryfindmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tryfindmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TryFindMatches<'r, 'c, 'h>`

- <span id="tryfindmatches-iterator-type-item"></span>`type Item = Result<Match, MatchError>`

- <span id="tryfindmatches-next"></span>`fn next(&mut self) -> Option<Result<Match, MatchError>>` — [`Match`](../../../index.md#match), [`MatchError`](../../../index.md#matcherror)

### `TryCapturesMatches<'r, 'c, 'h>`

```rust
struct TryCapturesMatches<'r, 'c, 'h> {
    re: &'r BoundedBacktracker,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1610-1615`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1610-L1615)*

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the BoundedBacktracker.
* `'c` represents the lifetime of the BoundedBacktracker's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the
`BoundedBacktracker::try_captures_iter` method.

#### Trait Implementations

##### `impl Debug for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="trycapturesmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="trycapturesmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TryCapturesMatches<'r, 'c, 'h>`

- <span id="trycapturesmatches-iterator-type-item"></span>`type Item = Result<Captures, MatchError>`

- <span id="trycapturesmatches-next"></span>`fn next(&mut self) -> Option<Result<Captures, MatchError>>` — [`Captures`](../../../util/captures/index.md#captures), [`MatchError`](../../../index.md#matcherror)

### `Cache`

```rust
struct Cache {
    stack: alloc::vec::Vec<Frame>,
    visited: Visited,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1653-1664`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1653-L1664)*

A cache represents mutable state that a [`BoundedBacktracker`](#boundedbacktracker) requires
during a search.

For a given [`BoundedBacktracker`](#boundedbacktracker), its corresponding cache may be created
either via `BoundedBacktracker::create_cache`, or via `Cache::new`.
They are equivalent in every way, except the former does not require
explicitly importing `Cache`.

A particular `Cache` is coupled with the [`BoundedBacktracker`](#boundedbacktracker) from which
it was created. It may only be used with that `BoundedBacktracker`. A cache
and its allocations may be re-purposed via `Cache::reset`, in which case,
it can only be used with the new `BoundedBacktracker` (and not the old
one).

#### Fields

- **`stack`**: `alloc::vec::Vec<Frame>`

  Stack used on the heap for doing backtracking instead of the
  traditional recursive approach. We don't want recursion because then
  we're likely to hit a stack overflow for bigger regexes.

- **`visited`**: `Visited`

  The set of (StateID, HaystackOffset) pairs that have been visited
  by the backtracker within a single search. If such a pair has been
  visited, then we avoid doing the work for that pair again. This is
  what "bounds" the backtracking and prevents it from having worst case
  exponential time.

#### Implementations

- <span id="cache-new"></span>`fn new(re: &BoundedBacktracker) -> Cache` — [`BoundedBacktracker`](#boundedbacktracker), [`Cache`](#cache)

- <span id="cache-reset"></span>`fn reset(&mut self, re: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="cache-setup-search"></span>`fn setup_search(&mut self, re: &BoundedBacktracker, input: &Input<'_>) -> Result<(), MatchError>` — [`BoundedBacktracker`](#boundedbacktracker), [`Input`](../../../index.md#input), [`MatchError`](../../../index.md#matcherror)

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Visited`

```rust
struct Visited {
    bitset: alloc::vec::Vec<usize>,
    stride: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1779-1801`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1779-L1801)*

A bitset that keeps track of whether a particular (StateID, offset) has
been considered during backtracking. If it has already been visited, then
backtracking skips it. This is what gives backtracking its "bound."

#### Fields

- **`bitset`**: `alloc::vec::Vec<usize>`

  The actual underlying bitset. Each element in the bitset corresponds
  to a particular (StateID, offset) pair. States correspond to the rows
  and the offsets correspond to the columns.
  
  If our underlying NFA has N states and the haystack we're searching
  has M bytes, then we have N*(M+1) entries in our bitset table. The
  M+1 occurs because our matches are delayed by one byte (to support
  look-around), and so we need to handle the end position itself rather
  than stopping just before the end. (If there is no end position, then
  it's treated as "end-of-input," which is matched by things like '$'.)
  
  Given BITS=N*(M+1), we wind up with div_ceil(BITS, sizeof(usize))
  blocks.
  
  We use 'usize' to represent our blocks because it makes some of the
  arithmetic in 'insert' a bit nicer. For example, if we used 'u32' for
  our block, we'd either need to cast u32s to usizes or usizes to u32s.

- **`stride`**: `usize`

  The stride represents one plus length of the haystack we're searching
  (as described above). The stride must be initialized for each search.

#### Implementations

- <span id="visited-const-block-size"></span>`const BLOCK_SIZE: usize`

- <span id="visited-new"></span>`fn new(re: &BoundedBacktracker) -> Visited` — [`BoundedBacktracker`](#boundedbacktracker), [`Visited`](#visited)

- <span id="visited-insert"></span>`fn insert(&mut self, sid: StateID, at: usize) -> bool` — [`StateID`](../../../util/primitives/index.md#stateid)

- <span id="visited-reset"></span>`fn reset(&mut self, _: &BoundedBacktracker)` — [`BoundedBacktracker`](#boundedbacktracker)

- <span id="visited-setup-search"></span>`fn setup_search(&mut self, re: &BoundedBacktracker, input: &Input<'_>) -> Result<(), MatchError>` — [`BoundedBacktracker`](#boundedbacktracker), [`Input`](../../../index.md#input), [`MatchError`](../../../index.md#matcherror)

- <span id="visited-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Visited`

- <span id="visited-clone"></span>`fn clone(&self) -> Visited` — [`Visited`](#visited)

##### `impl Debug for Visited`

- <span id="visited-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Frame`

```rust
enum Frame {
    Step {
        sid: crate::util::primitives::StateID,
        at: usize,
    },
    RestoreCapture {
        slot: crate::util::primitives::SmallIndex,
        offset: Option<crate::util::primitives::NonMaxUsize>,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1761-1773`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1761-L1773)*

Represents a stack frame on the heap while doing backtracking.

Instead of using explicit recursion for backtracking, we use a stack on
the heap to keep track of things that we want to explore if the current
backtracking branch turns out to not lead to a match.

#### Variants

- **`Step`**

  Look for a match starting at `sid` and the given position in the
  haystack.

- **`RestoreCapture`**

  Reset the given `slot` to the given `offset` (which might be `None`).
  This effectively gives a "scope" to capturing groups, such that an
  offset for a particular group only gets returned if the match goes
  through that capturing group. If backtracking ends up going down a
  different branch that results in a different offset (or perhaps none at
  all), then this "restore capture" frame will cause the offset to get
  reset.

#### Trait Implementations

##### `impl Clone for Frame`

- <span id="frame-clone"></span>`fn clone(&self) -> Frame` — [`Frame`](#frame)

##### `impl Debug for Frame`

- <span id="frame-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `min_visited_capacity`

```rust
fn min_visited_capacity(nfa: &crate::nfa::thompson::NFA, input: &crate::util::search::Input<'_>) -> usize
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:41-43`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L41-L43)*

Returns the minimum visited capacity for the given haystack.

This function can be used as the argument to `Config::visited_capacity`
in order to guarantee that a backtracking search for the given `input`
won't return an error when using a [`BoundedBacktracker`](#boundedbacktracker) built from the
given `NFA`.

This routine exists primarily as a way to test that the bounded backtracker
works correctly when its capacity is set to the smallest possible amount.
Still, it may be useful in cases where you know you want to use the bounded
backtracker for a specific input, and just need to know what visited
capacity to provide to make it work.

Be warned that this number could be quite large as it is multiplicative in
the size the given NFA and haystack.

### `div_ceil`

```rust
fn div_ceil(lhs: usize, rhs: usize) -> usize
```

*Defined in [`regex-automata-0.4.13/src/nfa/thompson/backtrack.rs:1881-1887`](../../../../../.source_1765521767/regex-automata-0.4.13/src/nfa/thompson/backtrack.rs#L1881-L1887)*

Integer division, but rounds up instead of down.

