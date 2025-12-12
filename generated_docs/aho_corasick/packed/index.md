*[aho_corasick](../index.md) / [packed](index.md)*

---

# Module `packed`

Provides packed multiple substring search, principally for a small number of
patterns.

This sub-module provides vectorized routines for quickly finding
matches of a small number of patterns. In general, users of this crate
shouldn't need to interface with this module directly, as the primary
[`AhoCorasick`](crate::AhoCorasick) searcher will use these routines
automatically as a prefilter when applicable. However, in some cases, callers
may want to bypass the Aho-Corasick machinery entirely and use this vectorized
searcher directly.

# Overview

The primary types in this sub-module are:

* [`Searcher`](api/index.md) executes the actual search algorithm to report matches in a
haystack.
* [`Builder`](api/index.md) accumulates patterns incrementally and can construct a
`Searcher`.
* [`Config`](api/index.md) permits tuning the searcher, and itself will produce a `Builder`
(which can then be used to build a `Searcher`). Currently, the only tuneable
knob are the match semantics, but this may be expanded in the future.

# Examples

This example shows how to create a searcher from an iterator of patterns.
By default, leftmost-first match semantics are used. (See the top-level
[`MatchKind`](api/index.md) type for more details about match semantics, which apply
similarly to packed substring search.)

```rust
use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

fn example() -> Option<()> {
let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
Some(()) }
if cfg!(all(feature = "std", any(
    target_arch = "x86_64", target_arch = "aarch64",
))) {
    example().unwrap()
} else {
    assert!(example().is_none());
}
```

This example shows how to use [`Config`](api/index.md) to change the match semantics to
leftmost-longest:

```rust
use aho_corasick::{packed::{Config, MatchKind}, PatternID};

fn example() -> Option<()> {
let searcher = Config::new()
    .match_kind(MatchKind::LeftmostLongest)
    .builder()
    .add("foo")
    .add("foobar")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::must(1)], matches);
Some(()) }
if cfg!(all(feature = "std", any(
    target_arch = "x86_64", target_arch = "aarch64",
))) {
    example().unwrap()
} else {
    assert!(example().is_none());
}
```

# Packed substring searching

Packed substring searching refers to the use of SIMD (Single Instruction,
Multiple Data) to accelerate the detection of matches in a haystack. Unlike
conventional algorithms, such as Aho-Corasick, SIMD algorithms for substring
search tend to do better with a small number of patterns, where as Aho-Corasick
generally maintains reasonably consistent performance regardless of the number
of patterns you give it. Because of this, the vectorized searcher in this
sub-module cannot be used as a general purpose searcher, since building the
searcher may fail even when given a small number of patterns. However, in
exchange, when searching for a small number of patterns, searching can be quite
a bit faster than Aho-Corasick (sometimes by an order of magnitude).

The key take away here is that constructing a searcher from a list of patterns
is a fallible operation with no clear rules for when it will fail. While the
precise conditions under which building a searcher can fail is specifically an
implementation detail, here are some common reasons:

* Too many patterns were given. Typically, the limit is on the order of 100 or
  so, but this limit may fluctuate based on available CPU features.
* The available packed algorithms require CPU features that aren't available.
  For example, currently, this crate only provides packed algorithms for
  `x86_64` and `aarch64`. Therefore, constructing a packed searcher on any
  other target will always fail.
* Zero patterns were given, or one of the patterns given was empty. Packed
  searchers require at least one pattern and that all patterns are non-empty.
* Something else about the nature of the patterns (typically based on
  heuristics) suggests that a packed searcher would perform very poorly, so
  no searcher is built.

## Contents

- [Modules](#modules)
  - [`api`](#api)
  - [`ext`](#ext)
  - [`pattern`](#pattern)
  - [`rabinkarp`](#rabinkarp)
  - [`teddy`](#teddy)
  - [`vector`](#vector)
- [Structs](#structs)
  - [`Builder`](#builder)
  - [`Config`](#config)
  - [`FindIter`](#finditer)
  - [`Searcher`](#searcher)
- [Enums](#enums)
  - [`MatchKind`](#matchkind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`api`](#api) | mod |  |
| [`ext`](#ext) | mod |  |
| [`pattern`](#pattern) | mod |  |
| [`rabinkarp`](#rabinkarp) | mod |  |
| [`teddy`](#teddy) | mod |  |
| [`vector`](#vector) | mod |  |
| [`Builder`](#builder) | struct |  |
| [`Config`](#config) | struct |  |
| [`FindIter`](#finditer) | struct |  |
| [`Searcher`](#searcher) | struct |  |
| [`MatchKind`](#matchkind) | enum |  |

## Modules

- [`api`](api/index.md)
- [`ext`](ext/index.md)
- [`pattern`](pattern/index.md)
- [`rabinkarp`](rabinkarp/index.md)
- [`teddy`](teddy/index.md)
- [`vector`](vector/index.md)

## Structs

### `Builder`

```rust
struct Builder {
    config: Config,
    inert: bool,
    patterns: crate::packed::pattern::Patterns,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:232-239`](../../../.source_1765521767/aho-corasick-1.1.4/src/packed/api.rs#L232-L239)*

A builder for constructing a packed searcher from a collection of patterns.

# Example

This example shows how to use a builder to construct a searcher. By
default, leftmost-first match semantics are used.

```rust
use aho_corasick::{packed::{Builder, MatchKind}, PatternID};

fn example() -> Option<()> {
let searcher = Builder::new()
    .add("foobar")
    .add("foo")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
Some(()) }
if cfg!(all(feature = "std", any(
    target_arch = "x86_64", target_arch = "aarch64",
))) {
    example().unwrap()
} else {
    assert!(example().is_none());
}
```

#### Fields

- **`config`**: `Config`

  The configuration of this builder and subsequent matcher.

- **`inert`**: `bool`

  Set to true if the builder detects that a matcher cannot be built.

- **`patterns`**: `crate::packed::pattern::Patterns`

  The patterns provided by the caller.

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](api/index.md#builder)

- <span id="builder-from-config"></span>`fn from_config(config: Config) -> Builder` — [`Config`](api/index.md#config), [`Builder`](api/index.md#builder)

- <span id="builder-build"></span>`fn build(&self) -> Option<Searcher>` — [`Searcher`](api/index.md#searcher)

- <span id="builder-build-teddy"></span>`fn build_teddy(&self, patterns: Arc<Patterns>) -> Option<self::builder::Searcher>` — [`Patterns`](pattern/index.md#patterns), [`Searcher`](teddy/builder/index.md#searcher)

- <span id="builder-add"></span>`fn add<P: AsRef<[u8]>>(&mut self, pattern: P) -> &mut Builder` — [`Builder`](api/index.md#builder)

- <span id="builder-extend"></span>`fn extend<I, P>(&mut self, patterns: I) -> &mut Builder` — [`Builder`](api/index.md#builder)

- <span id="builder-len"></span>`fn len(&self) -> usize`

- <span id="builder-minimum-len"></span>`fn minimum_len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](api/index.md#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](api/index.md#builder)

### `Config`

```rust
struct Config {
    kind: MatchKind,
    force: Option<ForceAlgorithm>,
    only_teddy_fat: Option<bool>,
    only_teddy_256bit: Option<bool>,
    heuristic_pattern_limits: bool,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:87-93`](../../../.source_1765521767/aho-corasick-1.1.4/src/packed/api.rs#L87-L93)*

The configuration for a packed multiple pattern searcher.

The configuration is currently limited only to being able to select the
match semantics (leftmost-first or leftmost-longest) of a searcher. In the
future, more knobs may be made available.

A configuration produces a [`packed::Builder`](Builder), which in turn can
be used to construct a [`packed::Searcher`](Searcher) for searching.

# Example

This example shows how to use leftmost-longest semantics instead of the
default (leftmost-first).

```rust
use aho_corasick::{packed::{Config, MatchKind}, PatternID};

fn example() -> Option<()> {
let searcher = Config::new()
    .match_kind(MatchKind::LeftmostLongest)
    .builder()
    .add("foo")
    .add("foobar")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::must(1)], matches);
Some(()) }
if cfg!(all(feature = "std", any(
    target_arch = "x86_64", target_arch = "aarch64",
))) {
    example().unwrap()
} else {
    assert!(example().is_none());
}
```

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](api/index.md#config)

- <span id="config-builder"></span>`fn builder(&self) -> Builder` — [`Builder`](api/index.md#builder)

- <span id="config-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Config` — [`MatchKind`](api/index.md#matchkind), [`Config`](api/index.md#config)

- <span id="config-heuristic-pattern-limits"></span>`fn heuristic_pattern_limits(&mut self, yes: bool) -> &mut Config` — [`Config`](api/index.md#config)

#### Trait Implementations

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](api/index.md#config)

##### `impl Debug for Config`

- <span id="config-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](api/index.md#config)

### `FindIter<'s, 'h>`

```rust
struct FindIter<'s, 'h> {
    searcher: &'s Searcher,
    haystack: &'h [u8],
    span: crate::util::search::Span,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:666-670`](../../../.source_1765521767/aho-corasick-1.1.4/src/packed/api.rs#L666-L670)*

An iterator over non-overlapping matches from a packed searcher.

The lifetime `'s` refers to the lifetime of the underlying [`Searcher`](api/index.md),
while the lifetime `'h` refers to the lifetime of the haystack being
searched.

#### Trait Implementations

##### `impl Debug for FindIter<'s, 'h>`

- <span id="finditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindIter<'s, 'h>`

- <span id="finditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="finditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="finditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindIter<'s, 'h>`

- <span id="finditer-iterator-type-item"></span>`type Item = Match`

- <span id="finditer-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

### `Searcher`

```rust
struct Searcher {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    rabinkarp: crate::packed::rabinkarp::RabinKarp,
    search_kind: SearchKind,
    minimum_len: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:396-401`](../../../.source_1765521767/aho-corasick-1.1.4/src/packed/api.rs#L396-L401)*

A packed searcher for quickly finding occurrences of multiple patterns.

If callers need more flexible construction, or if one wants to change the
match semantics (either leftmost-first or leftmost-longest), then one can
use the [`Config`](api/index.md) and/or [`Builder`](api/index.md) types for more fine grained control.

# Example

This example shows how to create a searcher from an iterator of patterns.
By default, leftmost-first match semantics are used.

```rust
use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

fn example() -> Option<()> {
let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
Some(()) }
if cfg!(all(feature = "std", any(
    target_arch = "x86_64", target_arch = "aarch64",
))) {
    example().unwrap()
} else {
    assert!(example().is_none());
}
```

#### Implementations

- <span id="searcher-new"></span>`fn new<I, P>(patterns: I) -> Option<Searcher>` — [`Searcher`](api/index.md#searcher)

- <span id="searcher-config"></span>`fn config() -> Config` — [`Config`](api/index.md#config)

- <span id="searcher-builder"></span>`fn builder() -> Builder` — [`Builder`](api/index.md#builder)

- <span id="searcher-find"></span>`fn find<B: AsRef<[u8]>>(&self, haystack: B) -> Option<Match>` — [`Match`](../util/search/index.md#match)

- <span id="searcher-find-in"></span>`fn find_in<B: AsRef<[u8]>>(&self, haystack: B, span: Span) -> Option<Match>` — [`Span`](../util/search/index.md#span), [`Match`](../util/search/index.md#match)

- <span id="searcher-find-iter"></span>`fn find_iter<'a, 'b, B: ?Sized + AsRef<[u8]>>(self: &'a Self, haystack: &'b B) -> FindIter<'a, 'b>` — [`FindIter`](api/index.md#finditer)

- <span id="searcher-match-kind"></span>`fn match_kind(&self) -> &MatchKind` — [`MatchKind`](api/index.md#matchkind)

- <span id="searcher-minimum-len"></span>`fn minimum_len(&self) -> usize`

- <span id="searcher-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="searcher-find-in-slow"></span>`fn find_in_slow(&self, haystack: &[u8], span: Span) -> Option<Match>` — [`Span`](../util/search/index.md#span), [`Match`](../util/search/index.md#match)

#### Trait Implementations

##### `impl Clone for Searcher`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher` — [`Searcher`](api/index.md#searcher)

##### `impl Debug for Searcher`

- <span id="searcher-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `MatchKind`

```rust
enum MatchKind {
    LeftmostFirst,
    LeftmostLongest,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:28-40`](../../../.source_1765521767/aho-corasick-1.1.4/src/packed/api.rs#L28-L40)*

A knob for controlling the match semantics of a packed multiple string
searcher.

This differs from the [`MatchKind`](crate::MatchKind) type in the top-level
crate module in that it doesn't support "standard" match semantics,
and instead only supports leftmost-first or leftmost-longest. Namely,
"standard" semantics cannot be easily supported by packed searchers.

For more information on the distinction between leftmost-first and
leftmost-longest, see the docs on the top-level `MatchKind` type.

Unlike the top-level `MatchKind` type, the default match semantics for this
type are leftmost-first.

#### Variants

- **`LeftmostFirst`**

  Use leftmost-first match semantics, which reports leftmost matches.
  When there are multiple possible leftmost matches, the match
  corresponding to the pattern that appeared earlier when constructing
  the automaton is reported.
  
  This is the default.

- **`LeftmostLongest`**

  Use leftmost-longest match semantics, which reports leftmost matches.
  When there are multiple possible leftmost matches, the longest match
  is chosen.

#### Trait Implementations

##### `impl Clone for MatchKind`

- <span id="matchkind-clone"></span>`fn clone(&self) -> MatchKind` — [`MatchKind`](api/index.md#matchkind)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- <span id="matchkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MatchKind`

- <span id="matchkind-default"></span>`fn default() -> MatchKind` — [`MatchKind`](api/index.md#matchkind)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- <span id="matchkind-eq"></span>`fn eq(&self, other: &MatchKind) -> bool` — [`MatchKind`](api/index.md#matchkind)

##### `impl StructuralPartialEq for MatchKind`

