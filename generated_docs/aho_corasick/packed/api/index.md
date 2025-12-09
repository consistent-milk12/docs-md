*[aho_corasick](../../index.md) / [packed](../index.md) / [api](index.md)*

---

# Module `api`

## Contents

- [Structs](#structs)
  - [`Config`](#config)
  - [`Builder`](#builder)
  - [`Searcher`](#searcher)
  - [`FindIter`](#finditer)
- [Enums](#enums)
  - [`MatchKind`](#matchkind)
  - [`ForceAlgorithm`](#forcealgorithm)
  - [`SearchKind`](#searchkind)
- [Constants](#constants)
  - [`PATTERN_LIMIT`](#pattern_limit)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | The configuration for a packed multiple pattern searcher. |
| [`Builder`](#builder) | struct | A builder for constructing a packed searcher from a collection of patterns. |
| [`Searcher`](#searcher) | struct | A packed searcher for quickly finding occurrences of multiple patterns. |
| [`FindIter`](#finditer) | struct | An iterator over non-overlapping matches from a packed searcher. |
| [`MatchKind`](#matchkind) | enum | A knob for controlling the match semantics of a packed multiple string searcher. |
| [`ForceAlgorithm`](#forcealgorithm) | enum | An internal option for forcing the use of a particular packed algorithm. |
| [`SearchKind`](#searchkind) | enum |  |
| [`PATTERN_LIMIT`](#pattern_limit) | const | This is a limit placed on the total number of patterns we're willing to try and match at once. |

## Structs

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

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:87-93`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L87-L93)*

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

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

- <span id="config-builder"></span>`fn builder(&self) -> Builder` — [`Builder`](#builder)

- <span id="config-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Config` — [`MatchKind`](#matchkind), [`Config`](#config)

- <span id="config-heuristic-pattern-limits"></span>`fn heuristic_pattern_limits(&mut self, yes: bool) -> &mut Config` — [`Config`](#config)

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
    inert: bool,
    patterns: crate::packed::pattern::Patterns,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:232-239`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L232-L239)*

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

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

- <span id="builder-from-config"></span>`fn from_config(config: Config) -> Builder` — [`Config`](#config), [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self) -> Option<Searcher>` — [`Searcher`](#searcher)

- <span id="builder-build-teddy"></span>`fn build_teddy(&self, patterns: Arc<Patterns>) -> Option<self::builder::Searcher>` — [`Patterns`](../pattern/index.md), [`Searcher`](../teddy/builder/index.md)

- <span id="builder-add"></span>`fn add<P: AsRef<[u8]>>(&mut self, pattern: P) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-extend"></span>`fn extend<I, P>(&mut self, patterns: I) -> &mut Builder` — [`Builder`](#builder)

- <span id="builder-len"></span>`fn len(&self) -> usize`

- <span id="builder-minimum-len"></span>`fn minimum_len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

### `Searcher`

```rust
struct Searcher {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    rabinkarp: crate::packed::rabinkarp::RabinKarp,
    search_kind: SearchKind,
    minimum_len: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:396-401`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L396-L401)*

A packed searcher for quickly finding occurrences of multiple patterns.

If callers need more flexible construction, or if one wants to change the
match semantics (either leftmost-first or leftmost-longest), then one can
use the [`Config`](#config) and/or [`Builder`](#builder) types for more fine grained control.

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

- <span id="searcher-new"></span>`fn new<I, P>(patterns: I) -> Option<Searcher>` — [`Searcher`](#searcher)

- <span id="searcher-config"></span>`fn config() -> Config` — [`Config`](#config)

- <span id="searcher-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

- <span id="searcher-find"></span>`fn find<B: AsRef<[u8]>>(&self, haystack: B) -> Option<Match>` — [`Match`](../../util/search/index.md)

- <span id="searcher-find-in"></span>`fn find_in<B: AsRef<[u8]>>(&self, haystack: B, span: Span) -> Option<Match>` — [`Span`](../../util/search/index.md), [`Match`](../../util/search/index.md)

- <span id="searcher-find-iter"></span>`fn find_iter<'a, 'b, B: ?Sized + AsRef<[u8]>>(self: &'a Self, haystack: &'b B) -> FindIter<'a, 'b>` — [`FindIter`](#finditer)

- <span id="searcher-match-kind"></span>`fn match_kind(&self) -> &MatchKind` — [`MatchKind`](#matchkind)

- <span id="searcher-minimum-len"></span>`fn minimum_len(&self) -> usize`

- <span id="searcher-memory-usage"></span>`fn memory_usage(&self) -> usize`

- <span id="searcher-find-in-slow"></span>`fn find_in_slow(&self, haystack: &[u8], span: Span) -> Option<Match>` — [`Span`](../../util/search/index.md), [`Match`](../../util/search/index.md)

#### Trait Implementations

##### `impl Clone for Searcher`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher` — [`Searcher`](#searcher)

##### `impl Debug for Searcher`

- <span id="searcher-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FindIter<'s, 'h>`

```rust
struct FindIter<'s, 'h> {
    searcher: &'s Searcher,
    haystack: &'h [u8],
    span: crate::util::search::Span,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:666-670`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L666-L670)*

An iterator over non-overlapping matches from a packed searcher.

The lifetime `'s` refers to the lifetime of the underlying [`Searcher`](#searcher),
while the lifetime `'h` refers to the lifetime of the haystack being
searched.

#### Trait Implementations

##### `impl Debug for FindIter<'s, 'h>`

- <span id="finditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindIter<'s, 'h>`

- <span id="finditer-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="finditer-type-intoiter"></span>`type IntoIter = I`

- <span id="finditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindIter<'s, 'h>`

- <span id="finditer-type-item"></span>`type Item = Match`

- <span id="finditer-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../util/search/index.md)

## Enums

### `MatchKind`

```rust
enum MatchKind {
    LeftmostFirst,
    LeftmostLongest,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:28-40`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L28-L40)*

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

- <span id="matchkind-clone"></span>`fn clone(&self) -> MatchKind` — [`MatchKind`](#matchkind)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- <span id="matchkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MatchKind`

- <span id="matchkind-default"></span>`fn default() -> MatchKind` — [`MatchKind`](#matchkind)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- <span id="matchkind-eq"></span>`fn eq(&self, other: &MatchKind) -> bool` — [`MatchKind`](#matchkind)

##### `impl StructuralPartialEq for MatchKind`

### `ForceAlgorithm`

```rust
enum ForceAlgorithm {
    Teddy,
    RabinKarp,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:101-104`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L101-L104)*

An internal option for forcing the use of a particular packed algorithm.

When an algorithm is forced, if a searcher could not be constructed for it,
then no searcher will be returned even if an alternative algorithm would
work.

#### Trait Implementations

##### `impl Clone for ForceAlgorithm`

- <span id="forcealgorithm-clone"></span>`fn clone(&self) -> ForceAlgorithm` — [`ForceAlgorithm`](#forcealgorithm)

##### `impl Debug for ForceAlgorithm`

- <span id="forcealgorithm-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SearchKind`

```rust
enum SearchKind {
    Teddy(self::builder::Searcher),
    RabinKarp,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:404-407`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L404-L407)*

#### Implementations

- <span id="searchkind-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for SearchKind`

- <span id="searchkind-clone"></span>`fn clone(&self) -> SearchKind` — [`SearchKind`](#searchkind)

##### `impl Debug for SearchKind`

- <span id="searchkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Constants

### `PATTERN_LIMIT`
```rust
const PATTERN_LIMIT: usize = 128usize;
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:11`](../../../../.source_1765210505/aho-corasick-1.1.4/src/packed/api.rs#L11)*

This is a limit placed on the total number of patterns we're willing to try
and match at once. As more sophisticated algorithms are added, this number
may be increased.

