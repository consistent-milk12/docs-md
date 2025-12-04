*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [pikevm](index.md)*

---

# Module `pikevm`

An NFA backed Pike VM for executing regex searches with capturing groups.

This module provides a [`PikeVM`](regex_automata/nfa/thompson/pikevm/index.md) that works by simulating an NFA and
resolving all spans of capturing groups that participate in a match.

## Structs

### `Config`

```rust
struct Config {
    // [REDACTED: Private Fields]
}
```

The configuration used for building a [`PikeVM`](regex_automata/nfa/thompson/pikevm/index.md).

A PikeVM configuration is a simple data object that is typically used with
[`Builder::configure`](#configure). It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with [`PikeVM::config`](#config).

#### Implementations

- `fn new() -> Config`
  Return a new default PikeVM configuration.

- `fn match_kind(self: Self, kind: MatchKind) -> Config`
  Set the desired match semantics.

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config`
  Set a prefilter to be used whenever a start state is entered.

- `fn get_match_kind(self: &Self) -> MatchKind`
  Returns the match semantics set in this configuration.

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>`
  Returns the prefilter set in this configuration, if one at all.

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

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

A builder for a `PikeVM`.

This builder permits configuring options for the syntax of a pattern,
the NFA construction and the `PikeVM` construction. This builder is
different from a general purpose regex builder in that it permits fine
grain configuration of the construction process. The trade off for this is
complexity, and the possibility of setting a configuration that might not
make sense. For example, there are two different UTF-8 modes:

* [`util::syntax::Config::utf8`](crate::util::syntax::Config::utf8)
controls whether the pattern itself can contain sub-expressions that match
invalid UTF-8.
* [`thompson::Config::utf8`](#utf8) controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```
use regex_automata::{
    nfa::thompson::{self, pikevm::PikeVM},
    util::syntax,
    Match,
};

let re = PikeVM::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find_iter(&mut cache, haystack).next();
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
//
// N.B. This example does not show the impact of
// disabling UTF-8 mode on a PikeVM Config, since that
// only impacts regexes that can produce matches of
// length 0.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder`
  Create a new PikeVM builder with its default configuration.

- `fn build(self: &Self, pattern: &str) -> Result<PikeVM, BuildError>`
  Build a `PikeVM` from the given pattern.

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<PikeVM, BuildError>`
  Build a `PikeVM` from the given patterns.

- `fn build_from_nfa(self: &Self, nfa: NFA) -> Result<PikeVM, BuildError>`
  Build a `PikeVM` directly from its NFA.

- `fn configure(self: &mut Self, config: Config) -> &mut Builder`
  Apply the given `PikeVM` configuration options to this builder.

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder`
  Set the syntax configuration for this builder using

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder`
  Set the Thompson NFA configuration for this builder using

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

### `PikeVM`

```rust
struct PikeVM {
    // [REDACTED: Private Fields]
}
```

A virtual machine for executing regex searches with capturing groups.

# Infallible APIs

Unlike most other regex engines in this crate, a `PikeVM` never returns an
error at search time. It supports all [`Anchored`](#anchored) configurations, never
quits and works on haystacks of arbitrary length.

There are two caveats to mention though:

* If an invalid pattern ID is given to a search via [`Anchored::Pattern`](#pattern),
then the PikeVM will report "no match." This is consistent with all other
regex engines in this crate.
* When using [`PikeVM::which_overlapping_matches`](#which-overlapping-matches) with a [`PatternSet`](#patternset)
that has insufficient capacity to store all valid pattern IDs, then if a
match occurs for a `PatternID` that cannot be inserted, it is silently
dropped as if it did not match.

# Advice

The `PikeVM` is generally the most "powerful" regex engine in this crate.
"Powerful" in this context means that it can handle any regular expression
that is parseable by `regex-syntax` and any size haystack. Regrettably,
the `PikeVM` is also simultaneously often the _slowest_ regex engine in
practice. This results in an annoying situation where one generally tries
to pick any other regex engine (or perhaps none at all) before being
forced to fall back to a `PikeVM`.

For example, a common strategy for dealing with capturing groups is to
actually look for the overall match of the regex using a faster regex
engine, like a [lazy DFA](crate::hybrid::regex::Regex). Once the overall
match is found, one can then run the `PikeVM` on just the match span to
find the spans of the capturing groups. In this way, the faster regex
engine does the majority of the work, while the `PikeVM` only lends its
power in a more limited role.

Unfortunately, this isn't always possible because the faster regex engines
don't support all of the regex features in `regex-syntax`. This notably
includes (and is currently limited to) Unicode word boundaries. So if
your pattern has Unicode word boundaries, you typically can't use a
DFA-based regex engine at all (unless you [enable heuristic support for
it](crate::hybrid::dfa::Config::unicode_word_boundary)). (The [one-pass
DFA](crate::dfa::onepass::DFA) can handle Unicode word boundaries for
anchored searches only, but in a cruel sort of joke, many Unicode features
tend to result in making the regex _not_ one-pass.)

# Example

This example shows that the `PikeVM` implements Unicode word boundaries
correctly by default.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Match::must(0, 0..12)), it.next());
assert_eq!(Some(Match::must(0, 13..23)), it.next());
assert_eq!(None, it.next());
# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn is_match<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> bool`
  Returns true if and only if this `PikeVM` matches the given haystack.

- `fn find<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> Option<Match>`
  Executes a leftmost forward search and returns a `Match` if one exists.

- `fn captures<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I, caps: &mut Captures)`
  Executes a leftmost forward search and writes the spans of capturing

- `fn find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> FindMatches<'r, 'c, 'h>`
  Returns an iterator over all non-overlapping leftmost matches in the

- `fn captures_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> CapturesMatches<'r, 'c, 'h>`
  Returns an iterator over all non-overlapping `Captures` values. If no

- `fn search(self: &Self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures)`
  Executes a leftmost forward search and writes the spans of capturing

- `fn search_slots(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>`
  Executes a leftmost forward search and writes the spans of capturing

- `fn which_overlapping_matches(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)`
  Writes the set of patterns that match anywhere in the given search

- `fn new(pattern: &str) -> Result<PikeVM, BuildError>`
  Parse the given regular expression using the default configuration and

- `fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<PikeVM, BuildError>`
  Like `new`, but parses multiple patterns into a single "multi regex."

- `fn new_from_nfa(nfa: NFA) -> Result<PikeVM, BuildError>`
  Like `new`, but builds a PikeVM directly from an NFA. This is useful

- `fn always_match() -> Result<PikeVM, BuildError>`
  Create a new `PikeVM` that matches every input.

- `fn never_match() -> Result<PikeVM, BuildError>`
  Create a new `PikeVM` that never matches any input.

- `fn config() -> Config`
  Return a default configuration for a `PikeVM`.

- `fn builder() -> Builder`
  Return a builder for configuring the construction of a `PikeVM`.

- `fn create_captures(self: &Self) -> Captures`
  Create a new empty set of capturing groups that is guaranteed to be

- `fn create_cache(self: &Self) -> Cache`
  Create a new cache for this `PikeVM`.

- `fn reset_cache(self: &Self, cache: &mut Cache)`
  Reset the given cache such that it can be used for searching with the

- `fn pattern_len(self: &Self) -> usize`
  Returns the total number of patterns compiled into this `PikeVM`.

- `fn get_config(self: &Self) -> &Config`
  Return the config for this `PikeVM`.

- `fn get_nfa(self: &Self) -> &NFA`
  Returns a reference to the underlying NFA.

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

- `fn clone(self: &Self) -> PikeVM`

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

### `FindMatches<'r, 'c, 'h>`

```rust
struct FindMatches<'r, 'c, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over all non-overlapping matches for a particular search.

The iterator yields a [`Match`](../../../../syn/syn/token/index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`PikeVM::find_iter`](#find-iter) method.

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

##### `impl Iterator<'r, 'c, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'c, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `CapturesMatches<'r, 'c, 'h>`

```rust
struct CapturesMatches<'r, 'c, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a particular search.

The iterator yields a [`Captures`](regex_automata/util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the [`PikeVM::captures_iter`](#captures-iter) method.

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

##### `impl Iterator<'r, 'c, 'h>`

- `type Item = Captures`

- `fn next(self: &mut Self) -> Option<Captures>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'c, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Cache`

```rust
struct Cache {
    // [REDACTED: Private Fields]
}
```

A cache represents mutable state that a [`PikeVM`](regex_automata/nfa/thompson/pikevm/index.md) requires during a
search.

For a given [`PikeVM`](regex_automata/nfa/thompson/pikevm/index.md), its corresponding cache may be created either via
[`PikeVM::create_cache`](#create-cache), or via [`Cache::new`](#new). They are equivalent in
every way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the [`PikeVM`](regex_automata/nfa/thompson/pikevm/index.md) from which it
was created. It may only be used with that `PikeVM`. A cache and its
allocations may be re-purposed via [`Cache::reset`](#reset), in which case, it can
only be used with the new `PikeVM` (and not the old one).

#### Implementations

- `fn new(re: &PikeVM) -> Cache`
  Create a new [`PikeVM`] cache.

- `fn reset(self: &mut Self, re: &PikeVM)`
  Reset this cache such that it can be used for searching with a

- `fn memory_usage(self: &Self) -> usize`
  Returns the heap memory usage, in bytes, of this cache.

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

- `fn clone(self: &Self) -> Cache`

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

