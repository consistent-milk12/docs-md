*[aho_corasick](../../index.md) / [packed](../index.md) / [api](index.md)*

---

# Module `api`

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

- `fn new() -> Config` — [`Config`](../index.md)

- `fn builder(self: &Self) -> Builder` — [`Builder`](../index.md)

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Config` — [`MatchKind`](../index.md), [`Config`](../index.md)

- `fn heuristic_pattern_limits(self: &mut Self, yes: bool) -> &mut Config` — [`Config`](../index.md)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](../index.md)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](../index.md)

### `Builder`

```rust
struct Builder {
    config: Config,
    inert: bool,
    patterns: crate::packed::pattern::Patterns,
}
```

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

- `fn new() -> Builder` — [`Builder`](../index.md)

- `fn from_config(config: Config) -> Builder` — [`Config`](../index.md), [`Builder`](../index.md)

- `fn build(self: &Self) -> Option<Searcher>` — [`Searcher`](../index.md)

- `fn build_teddy(self: &Self, patterns: Arc<Patterns>) -> Option<self::builder::Searcher>` — [`Patterns`](../pattern/index.md), [`Searcher`](../teddy/builder/index.md)

- `fn add<P: AsRef<[u8]>>(self: &mut Self, pattern: P) -> &mut Builder` — [`Builder`](../index.md)

- `fn extend<I, P>(self: &mut Self, patterns: I) -> &mut Builder` — [`Builder`](../index.md)

- `fn len(self: &Self) -> usize`

- `fn minimum_len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](../index.md)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](../index.md)

### `Searcher`

```rust
struct Searcher {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    rabinkarp: crate::packed::rabinkarp::RabinKarp,
    search_kind: SearchKind,
    minimum_len: usize,
}
```

A packed searcher for quickly finding occurrences of multiple patterns.

If callers need more flexible construction, or if one wants to change the
match semantics (either leftmost-first or leftmost-longest), then one can
use the [`Config`](../index.md) and/or [`Builder`](../index.md) types for more fine grained control.

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

- `fn new<I, P>(patterns: I) -> Option<Searcher>` — [`Searcher`](../index.md)

- `fn config() -> Config` — [`Config`](../index.md)

- `fn builder() -> Builder` — [`Builder`](../index.md)

- `fn find<B: AsRef<[u8]>>(self: &Self, haystack: B) -> Option<Match>` — [`Match`](../../index.md)

- `fn find_in<B: AsRef<[u8]>>(self: &Self, haystack: B, span: Span) -> Option<Match>` — [`Span`](../../index.md), [`Match`](../../index.md)

- `fn find_iter<'a, 'b, B: ?Sized + AsRef<[u8]>>(self: &'a Self, haystack: &'b B) -> FindIter<'a, 'b>` — [`FindIter`](../index.md)

- `fn match_kind(self: &Self) -> &MatchKind` — [`MatchKind`](../index.md)

- `fn minimum_len(self: &Self) -> usize`

- `fn memory_usage(self: &Self) -> usize`

- `fn find_in_slow(self: &Self, haystack: &[u8], span: Span) -> Option<Match>` — [`Span`](../../index.md), [`Match`](../../index.md)

#### Trait Implementations

##### `impl Clone for Searcher`

- `fn clone(self: &Self) -> Searcher` — [`Searcher`](../index.md)

##### `impl Debug for Searcher`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindIter<'s, 'h>`

```rust
struct FindIter<'s, 'h> {
    searcher: &'s Searcher,
    haystack: &'h [u8],
    span: crate::util::search::Span,
}
```

An iterator over non-overlapping matches from a packed searcher.

The lifetime `'s` refers to the lifetime of the underlying [`Searcher`](../index.md),
while the lifetime `'h` refers to the lifetime of the haystack being
searched.

#### Trait Implementations

##### `impl<'s, 'h> Debug for FindIter<'s, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindIter<'s, 'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'s, 'h> Iterator for FindIter<'s, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>` — [`Match`](../../index.md)

## Enums

### `MatchKind`

```rust
enum MatchKind {
    LeftmostFirst,
    LeftmostLongest,
}
```

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

- `fn clone(self: &Self) -> MatchKind` — [`MatchKind`](../index.md)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MatchKind`

- `fn default() -> MatchKind` — [`MatchKind`](../index.md)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- `fn eq(self: &Self, other: &MatchKind) -> bool` — [`MatchKind`](../index.md)

##### `impl StructuralPartialEq for MatchKind`

### `ForceAlgorithm`

```rust
enum ForceAlgorithm {
    Teddy,
    RabinKarp,
}
```

An internal option for forcing the use of a particular packed algorithm.

When an algorithm is forced, if a searcher could not be constructed for it,
then no searcher will be returned even if an alternative algorithm would
work.

#### Trait Implementations

##### `impl Clone for ForceAlgorithm`

- `fn clone(self: &Self) -> ForceAlgorithm` — [`ForceAlgorithm`](#forcealgorithm)

##### `impl Debug for ForceAlgorithm`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SearchKind`

```rust
enum SearchKind {
    Teddy(self::builder::Searcher),
    RabinKarp,
}
```

#### Implementations

- `fn memory_usage(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for SearchKind`

- `fn clone(self: &Self) -> SearchKind` — [`SearchKind`](#searchkind)

##### `impl Debug for SearchKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Constants

### `PATTERN_LIMIT`

```rust
const PATTERN_LIMIT: usize = 128usize;
```

This is a limit placed on the total number of patterns we're willing to try
and match at once. As more sophisticated algorithms are added, this number
may be increased.

