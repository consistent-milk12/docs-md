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
  - [`PATTERN_LIMIT`](#pattern-limit)

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
| [`PATTERN_LIMIT`](#pattern-limit) | const | This is a limit placed on the total number of patterns we're willing to try and match at once. |

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

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:87-93`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L87-L93)*

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

  Create a new default configuration. A default configuration uses

  leftmost-first match semantics.

- <span id="config-builder"></span>`fn builder(&self) -> Builder` — [`Builder`](#builder)

  Create a packed builder from this configuration. The builder can be

  used to accumulate patterns and create a [`Searcher`](#searcher) from them.

- <span id="config-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut Config` — [`MatchKind`](#matchkind), [`Config`](#config)

  Set the match semantics for this configuration.

- <span id="config-heuristic-pattern-limits"></span>`fn heuristic_pattern_limits(&mut self, yes: bool) -> &mut Config` — [`Config`](#config)

  Request that heuristic limitations on the number of patterns be

  employed. This useful to disable for benchmarking where one wants to

  explore how Teddy performs on large number of patterns even if the

  heuristics would otherwise refuse construction.

  

  This is enabled by default.

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
    inert: bool,
    patterns: crate::packed::pattern::Patterns,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:232-239`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L232-L239)*

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

  Create a new builder for constructing a multi-pattern searcher. This

  constructor uses the default configuration.

- <span id="builder-from-config"></span>`fn from_config(config: Config) -> Builder` — [`Config`](#config), [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self) -> Option<Searcher>` — [`Searcher`](#searcher)

  Build a searcher from the patterns added to this builder so far.

- <span id="builder-build-teddy"></span>`fn build_teddy(&self, patterns: Arc<Patterns>) -> Option<self::builder::Searcher>` — [`Patterns`](../pattern/index.md#patterns), [`Searcher`](../teddy/builder/index.md#searcher)

- <span id="builder-add"></span>`fn add<P: AsRef<[u8]>>(&mut self, pattern: P) -> &mut Builder` — [`Builder`](#builder)

  Add the given pattern to this set to match.

  

  The order in which patterns are added is significant. Namely, when

  using leftmost-first match semantics, then when multiple patterns can

  match at a particular location, the pattern that was added first is

  used as the match.

  

  If the number of patterns added exceeds the amount supported by packed

  searchers, then the builder will stop accumulating patterns and render

  itself inert. At this point, constructing a searcher will always return

  `None`.

- <span id="builder-extend"></span>`fn extend<I, P>(&mut self, patterns: I) -> &mut Builder` — [`Builder`](#builder)

  Add the given iterator of patterns to this set to match.

  

  The iterator must yield elements that can be converted into a `&[u8]`.

  

  The order in which patterns are added is significant. Namely, when

  using leftmost-first match semantics, then when multiple patterns can

  match at a particular location, the pattern that was added first is

  used as the match.

  

  If the number of patterns added exceeds the amount supported by packed

  searchers, then the builder will stop accumulating patterns and render

  itself inert. At this point, constructing a searcher will always return

  `None`.

- <span id="builder-len"></span>`fn len(&self) -> usize`

  Returns the number of patterns added to this builder.

- <span id="builder-minimum-len"></span>`fn minimum_len(&self) -> usize`

  Returns the length, in bytes, of the shortest pattern added.

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

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

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

### `Searcher`

```rust
struct Searcher {
    patterns: alloc::sync::Arc<crate::packed::pattern::Patterns>,
    rabinkarp: crate::packed::rabinkarp::RabinKarp,
    search_kind: SearchKind,
    minimum_len: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:396-401`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L396-L401)*

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

  A convenience function for constructing a searcher from an iterator

  of things that can be converted to a `&[u8]`.

  

  If a searcher could not be constructed (either because of an

  unsupported CPU or because there are too many patterns), then `None`

  is returned.

  

  # Example

  

  Basic usage:

  

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

- <span id="searcher-config"></span>`fn config() -> Config` — [`Config`](#config)

  A convenience function for calling `Config::new()`.

  

  This is useful for avoiding an additional import.

- <span id="searcher-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

  A convenience function for calling `Builder::new()`.

  

  This is useful for avoiding an additional import.

- <span id="searcher-find"></span>`fn find<B: AsRef<[u8]>>(&self, haystack: B) -> Option<Match>` — [`Match`](../../util/search/index.md#match)

  Return the first occurrence of any of the patterns in this searcher,

  according to its match semantics, in the given haystack. The `Match`

  returned will include the identifier of the pattern that matched, which

  corresponds to the index of the pattern (starting from `0`) in which it

  was added.

  

  # Example

  

  Basic usage:

  

  ```rust

  use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

  

  fn example() -> Option<()> {

  let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;

  let mat = searcher.find("foobar")?;

  assert_eq!(PatternID::ZERO, mat.pattern());

  assert_eq!(0, mat.start());

  assert_eq!(6, mat.end());

  Some(()) }

  if cfg!(all(feature = "std", any(

      target_arch = "x86_64", target_arch = "aarch64",

  ))) {

      example().unwrap()

  } else {

      assert!(example().is_none());

  }

  ```

- <span id="searcher-find-in"></span>`fn find_in<B: AsRef<[u8]>>(&self, haystack: B, span: Span) -> Option<Match>` — [`Span`](../../util/search/index.md#span), [`Match`](../../util/search/index.md#match)

  Return the first occurrence of any of the patterns in this searcher,

  according to its match semantics, in the given haystack starting from

  the given position.

  

  The `Match` returned will include the identifier of the pattern that

  matched, which corresponds to the index of the pattern (starting from

  `0`) in which it was added. The offsets in the `Match` will be relative

  to the start of `haystack` (and not `at`).

  

  # Example

  

  Basic usage:

  

  ```rust

  use aho_corasick::{packed::{MatchKind, Searcher}, PatternID, Span};

  

  fn example() -> Option<()> {

  let haystack = "foofoobar";

  let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;

  let mat = searcher.find_in(haystack, Span::from(3..haystack.len()))?;

  assert_eq!(PatternID::ZERO, mat.pattern());

  assert_eq!(3, mat.start());

  assert_eq!(9, mat.end());

  Some(()) }

  if cfg!(all(feature = "std", any(

      target_arch = "x86_64", target_arch = "aarch64",

  ))) {

      example().unwrap()

  } else {

      assert!(example().is_none());

  }

  ```

- <span id="searcher-find-iter"></span>`fn find_iter<'a, 'b, B: ?Sized + AsRef<[u8]>>(self: &'a Self, haystack: &'b B) -> FindIter<'a, 'b>` — [`FindIter`](#finditer)

  Return an iterator of non-overlapping occurrences of the patterns in

  this searcher, according to its match semantics, in the given haystack.

  

  # Example

  

  Basic usage:

  

  ```rust

  use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

  

  fn example() -> Option<()> {

  let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;

  let matches: Vec<PatternID> = searcher

      .find_iter("foobar fooba foofoo")

      .map(|mat| mat.pattern())

      .collect();

  assert_eq!(vec![

      PatternID::must(0),

      PatternID::must(1),

      PatternID::must(1),

      PatternID::must(1),

  ], matches);

  Some(()) }

  if cfg!(all(feature = "std", any(

      target_arch = "x86_64", target_arch = "aarch64",

  ))) {

      example().unwrap()

  } else {

      assert!(example().is_none());

  }

  ```

- <span id="searcher-match-kind"></span>`fn match_kind(&self) -> &MatchKind` — [`MatchKind`](#matchkind)

  Returns the match kind used by this packed searcher.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use aho_corasick::packed::{MatchKind, Searcher};

  

  fn example() -> Option<()> {

  let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;

  // leftmost-first is the default.

  assert_eq!(&MatchKind::LeftmostFirst, searcher.match_kind());

  Some(()) }

  if cfg!(all(feature = "std", any(

      target_arch = "x86_64", target_arch = "aarch64",

  ))) {

      example().unwrap()

  } else {

      assert!(example().is_none());

  }

  ```

- <span id="searcher-minimum-len"></span>`fn minimum_len(&self) -> usize`

  Returns the minimum length of a haystack that is required in order for

  packed searching to be effective.

  

  In some cases, the underlying packed searcher may not be able to search

  very short haystacks. When that occurs, the implementation will defer

  to a slower non-packed searcher (which is still generally faster than

  Aho-Corasick for a small number of patterns). However, callers may

  want to avoid ever using the slower variant, which one can do by

  never passing a haystack shorter than the minimum length returned by

  this method.

- <span id="searcher-memory-usage"></span>`fn memory_usage(&self) -> usize`

  Returns the approximate total amount of heap used by this searcher, in

  units of bytes.

- <span id="searcher-find-in-slow"></span>`fn find_in_slow(&self, haystack: &[u8], span: Span) -> Option<Match>` — [`Span`](../../util/search/index.md#span), [`Match`](../../util/search/index.md#match)

  Use a slow (non-packed) searcher.

  

  This is useful when a packed searcher could be constructed, but could

  not be used to search a specific haystack. For example, if Teddy was

  built but the haystack is smaller than ~34 bytes, then Teddy might not

  be able to run.

#### Trait Implementations

##### `impl Any for Searcher`

- <span id="searcher-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Searcher`

- <span id="searcher-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Searcher`

- <span id="searcher-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Searcher`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher` — [`Searcher`](#searcher)

##### `impl CloneToUninit for Searcher`

- <span id="searcher-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Searcher`

- <span id="searcher-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Searcher`

- <span id="searcher-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Searcher`

- <span id="searcher-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Searcher`

- <span id="searcher-toowned-type-owned"></span>`type Owned = T`

- <span id="searcher-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searcher-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Searcher`

- <span id="searcher-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searcher-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Searcher`

- <span id="searcher-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searcher-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindIter<'s, 'h>`

```rust
struct FindIter<'s, 'h> {
    searcher: &'s Searcher,
    haystack: &'h [u8],
    span: crate::util::search::Span,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:666-670`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L666-L670)*

An iterator over non-overlapping matches from a packed searcher.

The lifetime `'s` refers to the lifetime of the underlying [`Searcher`](#searcher),
while the lifetime `'h` refers to the lifetime of the haystack being
searched.

#### Trait Implementations

##### `impl Any for FindIter<'s, 'h>`

- <span id="finditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindIter<'s, 'h>`

- <span id="finditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindIter<'s, 'h>`

- <span id="finditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FindIter<'s, 'h>`

- <span id="finditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FindIter<'s, 'h>`

- <span id="finditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindIter<'s, 'h>`

- <span id="finditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for FindIter<'s, 'h>`

- <span id="finditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="finditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="finditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindIter<'s, 'h>`

- <span id="finditer-iterator-type-item"></span>`type Item = Match`

- <span id="finditer-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../util/search/index.md#match)

##### `impl<U> TryFrom for FindIter<'s, 'h>`

- <span id="finditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="finditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindIter<'s, 'h>`

- <span id="finditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="finditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `MatchKind`

```rust
enum MatchKind {
    LeftmostFirst,
    LeftmostLongest,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:28-40`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L28-L40)*

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

##### `impl Any for MatchKind`

- <span id="matchkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchKind`

- <span id="matchkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchKind`

- <span id="matchkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchKind`

- <span id="matchkind-clone"></span>`fn clone(&self) -> MatchKind` — [`MatchKind`](#matchkind)

##### `impl CloneToUninit for MatchKind`

- <span id="matchkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- <span id="matchkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MatchKind`

- <span id="matchkind-default"></span>`fn default() -> MatchKind` — [`MatchKind`](#matchkind)

##### `impl Eq for MatchKind`

##### `impl<T> From for MatchKind`

- <span id="matchkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchKind`

- <span id="matchkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MatchKind`

- <span id="matchkind-partialeq-eq"></span>`fn eq(&self, other: &MatchKind) -> bool` — [`MatchKind`](#matchkind)

##### `impl StructuralPartialEq for MatchKind`

##### `impl ToOwned for MatchKind`

- <span id="matchkind-toowned-type-owned"></span>`type Owned = T`

- <span id="matchkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matchkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MatchKind`

- <span id="matchkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matchkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchKind`

- <span id="matchkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matchkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ForceAlgorithm`

```rust
enum ForceAlgorithm {
    Teddy,
    RabinKarp,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:101-104`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L101-L104)*

An internal option for forcing the use of a particular packed algorithm.

When an algorithm is forced, if a searcher could not be constructed for it,
then no searcher will be returned even if an alternative algorithm would
work.

#### Trait Implementations

##### `impl Any for ForceAlgorithm`

- <span id="forcealgorithm-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ForceAlgorithm`

- <span id="forcealgorithm-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ForceAlgorithm`

- <span id="forcealgorithm-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ForceAlgorithm`

- <span id="forcealgorithm-clone"></span>`fn clone(&self) -> ForceAlgorithm` — [`ForceAlgorithm`](#forcealgorithm)

##### `impl CloneToUninit for ForceAlgorithm`

- <span id="forcealgorithm-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ForceAlgorithm`

- <span id="forcealgorithm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ForceAlgorithm`

- <span id="forcealgorithm-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ForceAlgorithm`

- <span id="forcealgorithm-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ForceAlgorithm`

- <span id="forcealgorithm-toowned-type-owned"></span>`type Owned = T`

- <span id="forcealgorithm-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="forcealgorithm-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ForceAlgorithm`

- <span id="forcealgorithm-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="forcealgorithm-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ForceAlgorithm`

- <span id="forcealgorithm-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="forcealgorithm-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SearchKind`

```rust
enum SearchKind {
    Teddy(self::builder::Searcher),
    RabinKarp,
}
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:404-407`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L404-L407)*

#### Implementations

- <span id="searchkind-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Any for SearchKind`

- <span id="searchkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SearchKind`

- <span id="searchkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SearchKind`

- <span id="searchkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SearchKind`

- <span id="searchkind-clone"></span>`fn clone(&self) -> SearchKind` — [`SearchKind`](#searchkind)

##### `impl CloneToUninit for SearchKind`

- <span id="searchkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SearchKind`

- <span id="searchkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SearchKind`

- <span id="searchkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SearchKind`

- <span id="searchkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for SearchKind`

- <span id="searchkind-toowned-type-owned"></span>`type Owned = T`

- <span id="searchkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="searchkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SearchKind`

- <span id="searchkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="searchkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SearchKind`

- <span id="searchkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="searchkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Constants

### `PATTERN_LIMIT`
```rust
const PATTERN_LIMIT: usize = 128usize;
```

*Defined in [`aho-corasick-1.1.4/src/packed/api.rs:11`](../../../../.source_1765633015/aho-corasick-1.1.4/src/packed/api.rs#L11)*

This is a limit placed on the total number of patterns we're willing to try
and match at once. As more sophisticated algorithms are added, this number
may be increased.

