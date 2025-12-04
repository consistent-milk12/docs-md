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

* [`Searcher`](../../regex_automata/regex_automata/util/iter/index.md) executes the actual search algorithm to report matches in a
haystack.
* [`Builder`](aho_corasick/dfa/index.md) accumulates patterns incrementally and can construct a
`Searcher`.
* [`Config`](../../base64/base64/engine/index.md) permits tuning the searcher, and itself will produce a `Builder`
(which can then be used to build a `Searcher`). Currently, the only tuneable
knob are the match semantics, but this may be expanded in the future.

# Examples

This example shows how to create a searcher from an iterator of patterns.
By default, leftmost-first match semantics are used. (See the top-level
[`MatchKind`](#matchkind) type for more details about match semantics, which apply
similarly to packed substring search.)

```
use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

# fn example() -> Option<()> {
let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

This example shows how to use [`Config`](../../base64/base64/engine/index.md) to change the match semantics to
leftmost-longest:

```
use aho_corasick::{packed::{Config, MatchKind}, PatternID};

# fn example() -> Option<()> {
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
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
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

## Structs

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

A builder for constructing a packed searcher from a collection of patterns.

# Example

This example shows how to use a builder to construct a searcher. By
default, leftmost-first match semantics are used.

```
use aho_corasick::{packed::{Builder, MatchKind}, PatternID};

# fn example() -> Option<()> {
let searcher = Builder::new()
    .add("foobar")
    .add("foo")
    .build()?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

#### Implementations

- `fn new() -> Builder`
  Create a new builder for constructing a multi-pattern searcher. This

- `fn build(self: &Self) -> Option<Searcher>`
  Build a searcher from the patterns added to this builder so far.

- `fn add<P: AsRef<[u8]>>(self: &mut Self, pattern: P) -> &mut Builder`
  Add the given pattern to this set to match.

- `fn extend<I, P>(self: &mut Self, patterns: I) -> &mut Builder`
  Add the given iterator of patterns to this set to match.

- `fn len(self: &Self) -> usize`
  Returns the number of patterns added to this builder.

- `fn minimum_len(self: &Self) -> usize`
  Returns the length, in bytes, of the shortest pattern added.

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

### `Config`

```rust
struct Config {
    // [REDACTED: Private Fields]
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

```
use aho_corasick::{packed::{Config, MatchKind}, PatternID};

# fn example() -> Option<()> {
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
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

#### Implementations

- `fn new() -> Config`
  Create a new default configuration. A default configuration uses

- `fn builder(self: &Self) -> Builder`
  Create a packed builder from this configuration. The builder can be

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut Config`
  Set the match semantics for this configuration.

- `fn heuristic_pattern_limits(self: &mut Self, yes: bool) -> &mut Config`
  Request that heuristic limitations on the number of patterns be

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

### `FindIter<'s, 'h>`

```rust
struct FindIter<'s, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over non-overlapping matches from a packed searcher.

The lifetime `'s` refers to the lifetime of the underlying [`Searcher`](../../regex_automata/regex_automata/util/iter/index.md),
while the lifetime `'h` refers to the lifetime of the haystack being
searched.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl IntoIterator<I>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Iterator<'s, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'s, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Searcher`

```rust
struct Searcher {
    // [REDACTED: Private Fields]
}
```

A packed searcher for quickly finding occurrences of multiple patterns.

If callers need more flexible construction, or if one wants to change the
match semantics (either leftmost-first or leftmost-longest), then one can
use the [`Config`](../../base64/base64/engine/index.md) and/or [`Builder`](aho_corasick/dfa/index.md) types for more fine grained control.

# Example

This example shows how to create a searcher from an iterator of patterns.
By default, leftmost-first match semantics are used.

```
use aho_corasick::{packed::{MatchKind, Searcher}, PatternID};

# fn example() -> Option<()> {
let searcher = Searcher::new(["foobar", "foo"].iter().cloned())?;
let matches: Vec<PatternID> = searcher
    .find_iter("foobar")
    .map(|mat| mat.pattern())
    .collect();
assert_eq!(vec![PatternID::ZERO], matches);
# Some(()) }
# if cfg!(all(feature = "std", any(
#     target_arch = "x86_64", target_arch = "aarch64",
# ))) {
#     example().unwrap()
# } else {
#     assert!(example().is_none());
# }
```

#### Implementations

- `fn new<I, P>(patterns: I) -> Option<Searcher>`
  A convenience function for constructing a searcher from an iterator

- `fn config() -> Config`
  A convenience function for calling `Config::new()`.

- `fn builder() -> Builder`
  A convenience function for calling `Builder::new()`.

- `fn find<B: AsRef<[u8]>>(self: &Self, haystack: B) -> Option<Match>`
  Return the first occurrence of any of the patterns in this searcher,

- `fn find_in<B: AsRef<[u8]>>(self: &Self, haystack: B, span: Span) -> Option<Match>`
  Return the first occurrence of any of the patterns in this searcher,

- `fn find_iter<'a, 'b, B: ?Sized + AsRef<[u8]>>(self: &'a Self, haystack: &'b B) -> FindIter<'a, 'b>`
  Return an iterator of non-overlapping occurrences of the patterns in

- `fn match_kind(self: &Self) -> &MatchKind`
  Returns the match kind used by this packed searcher.

- `fn minimum_len(self: &Self) -> usize`
  Returns the minimum length of a haystack that is required in order for

- `fn memory_usage(self: &Self) -> usize`
  Returns the approximate total amount of heap used by this searcher, in

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

- `fn clone(self: &Self) -> Searcher`

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

- `fn clone(self: &Self) -> MatchKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MatchKind) -> bool`

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

##### `impl Default`

- `fn default() -> MatchKind`

