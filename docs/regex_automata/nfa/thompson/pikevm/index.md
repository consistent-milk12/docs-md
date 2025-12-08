*[regex_automata](../../../index.md) / [nfa](../../index.md) / [thompson](../index.md) / [pikevm](index.md)*

---

# Module `pikevm`

An NFA backed Pike VM for executing regex searches with capturing groups.

This module provides a [`PikeVM`](#pikevm) that works by simulating an NFA and
resolving all spans of capturing groups that participate in a match.

## Structs

### `Config`

```rust
struct Config {
    match_kind: Option<crate::util::search::MatchKind>,
    pre: Option<Option<crate::util::prefilter::Prefilter>>,
}
```

The configuration used for building a [`PikeVM`](#pikevm).

A PikeVM configuration is a simple data object that is typically used with
`Builder::configure`. It can be cheaply cloned.

A default configuration can be created either with `Config::new`, or
perhaps more conveniently, with `PikeVM::config`.

#### Implementations

- `fn new() -> Config` — [`Config`](#config)

- `fn match_kind(self: Self, kind: MatchKind) -> Config` — [`MatchKind`](../../../index.md), [`Config`](#config)

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config` — [`Prefilter`](../../../util/prefilter/index.md), [`Config`](#config)

- `fn get_match_kind(self: &Self) -> MatchKind` — [`MatchKind`](../../../index.md)

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>` — [`Prefilter`](../../../util/prefilter/index.md)

- `fn overwrite(self: &Self, o: Config) -> Config` — [`Config`](#config)

#### Trait Implementations

##### `impl Clone for Config`

- `fn clone(self: &Self) -> Config` — [`Config`](#config)

##### `impl Debug for Config`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Config`

- `fn default() -> Config` — [`Config`](#config)

### `Builder`

```rust
struct Builder {
    config: Config,
    thompson: thompson::Compiler,
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
* `thompson::Config::utf8` controls whether empty matches that split a
Unicode codepoint are reported or not.

Generally speaking, callers will want to either enable all of these or
disable all of these.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```rust
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

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn build(self: &Self, pattern: &str) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md)

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<PikeVM, BuildError>` — [`PikeVM`](#pikevm), [`BuildError`](../error/index.md)

- `fn build_from_nfa(self: &Self, nfa: NFA) -> Result<PikeVM, BuildError>` — [`NFA`](../nfa/index.md), [`PikeVM`](#pikevm), [`BuildError`](../error/index.md)

- `fn configure(self: &mut Self, config: Config) -> &mut Builder` — [`Config`](#config), [`Builder`](#builder)

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../../util/syntax/index.md), [`Builder`](#builder)

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder` — [`Config`](../compiler/index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `PikeVM`

```rust
struct PikeVM {
    config: Config,
    nfa: crate::nfa::thompson::NFA,
}
```

A virtual machine for executing regex searches with capturing groups.

# Infallible APIs

Unlike most other regex engines in this crate, a `PikeVM` never returns an
error at search time. It supports all [`Anchored`](../../../index.md) configurations, never
quits and works on haystacks of arbitrary length.

There are two caveats to mention though:

* If an invalid pattern ID is given to a search via `Anchored::Pattern`,
then the PikeVM will report "no match." This is consistent with all other
regex engines in this crate.
* When using `PikeVM::which_overlapping_matches` with a [`PatternSet`](../../../index.md)
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

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{nfa::thompson::pikevm::PikeVM, Match};

let re = PikeVM::new(r"\b\w+\b")?;
let mut cache = re.create_cache();

let mut it = re.find_iter(&mut cache, "Шерлок Холмс");
assert_eq!(Some(Match::must(0, 0..12)), it.next());
assert_eq!(Some(Match::must(0, 13..23)), it.next());
assert_eq!(None, it.next());
Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn is_match<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> bool` — [`Cache`](#cache)

- `fn find<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> Option<Match>` — [`Cache`](#cache), [`Match`](../../../index.md)

- `fn captures<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I, caps: &mut Captures)` — [`Cache`](#cache), [`Captures`](../../../util/captures/index.md)

- `fn find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> FindMatches<'r, 'c, 'h>` — [`Cache`](#cache), [`FindMatches`](#findmatches)

- `fn captures_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> CapturesMatches<'r, 'c, 'h>` — [`Cache`](#cache), [`CapturesMatches`](#capturesmatches)

#### Trait Implementations

##### `impl Clone for PikeVM`

- `fn clone(self: &Self) -> PikeVM` — [`PikeVM`](#pikevm)

##### `impl Debug for PikeVM`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindMatches<'r, 'c, 'h>`

```rust
struct FindMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

An iterator over all non-overlapping matches for a particular search.

The iterator yields a [`Match`](../../../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `PikeVM::find_iter` method.

#### Trait Implementations

##### `impl<'r, 'c, 'h> Debug for FindMatches<'r, 'c, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindMatches<'r, 'c, 'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'r, 'c, 'h> Iterator for FindMatches<'r, 'c, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>` — [`Match`](../../../index.md)

### `CapturesMatches<'r, 'c, 'h>`

```rust
struct CapturesMatches<'r, 'c, 'h> {
    re: &'r PikeVM,
    cache: &'c mut Cache,
    caps: crate::util::captures::Captures,
    it: iter::Searcher<'h>,
}
```

An iterator over all non-overlapping leftmost matches, with their capturing
groups, for a particular search.

The iterator yields a [`Captures`](../../../util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the PikeVM.
* `'c` represents the lifetime of the PikeVM's cache.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `PikeVM::captures_iter` method.

#### Trait Implementations

##### `impl<'r, 'c, 'h> Debug for CapturesMatches<'r, 'c, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for CapturesMatches<'r, 'c, 'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'r, 'c, 'h> Iterator for CapturesMatches<'r, 'c, 'h>`

- `type Item = Captures`

- `fn next(self: &mut Self) -> Option<Captures>` — [`Captures`](../../../util/captures/index.md)

### `Cache`

```rust
struct Cache {
    stack: alloc::vec::Vec<FollowEpsilon>,
    curr: ActiveStates,
    next: ActiveStates,
}
```

A cache represents mutable state that a [`PikeVM`](#pikevm) requires during a
search.

For a given [`PikeVM`](#pikevm), its corresponding cache may be created either via
`PikeVM::create_cache`, or via `Cache::new`. They are equivalent in
every way, except the former does not require explicitly importing `Cache`.

A particular `Cache` is coupled with the [`PikeVM`](#pikevm) from which it
was created. It may only be used with that `PikeVM`. A cache and its
allocations may be re-purposed via `Cache::reset`, in which case, it can
only be used with the new `PikeVM` (and not the old one).

#### Fields

- **`stack`**: `alloc::vec::Vec<FollowEpsilon>`

  Stack used while computing epsilon closure. This effectively lets us
  move what is more naturally expressed through recursion to a stack
  on the heap.

- **`curr`**: `ActiveStates`

  The current active states being explored for the current byte in the
  haystack.

- **`next`**: `ActiveStates`

  The next set of states we're building that will be explored for the
  next byte in the haystack.

#### Implementations

- `fn new(re: &PikeVM) -> Cache` — [`PikeVM`](#pikevm), [`Cache`](#cache)

- `fn reset(self: &mut Self, re: &PikeVM)` — [`PikeVM`](#pikevm)

- `fn memory_usage(self: &Self) -> usize`

- `fn setup_search(self: &mut Self, captures_slot_len: usize)`

#### Trait Implementations

##### `impl Clone for Cache`

- `fn clone(self: &Self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

