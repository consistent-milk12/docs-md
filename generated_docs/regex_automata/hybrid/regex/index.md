*[regex_automata](../../index.md) / [hybrid](../index.md) / [regex](index.md)*

---

# Module `regex`

A lazy DFA backed `Regex`.

This module provides a [`Regex`](#regex) backed by a lazy DFA. A `Regex` implements
convenience routines you might have come to expect, such as finding a match
and iterating over all non-overlapping matches. This `Regex` type is limited
in its capabilities to what a lazy DFA can provide. Therefore, APIs involving
capturing groups, for example, are not provided.

Internally, a `Regex` is composed of two DFAs. One is a "forward" DFA that
finds the end offset of a match, where as the other is a "reverse" DFA that
find the start offset of a match.

See the [parent module](crate::hybrid) for examples.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Regex`](#regex) | struct | A regular expression that uses hybrid NFA/DFAs (also called "lazy DFAs") for searching. |
| [`FindMatches`](#findmatches) | struct | An iterator over all non-overlapping matches for an infallible search. |
| [`Cache`](#cache) | struct | A cache represents a partially computed forward and reverse DFA. |
| [`Builder`](#builder) | struct | A builder for a regex based on a hybrid NFA/DFA. |

## Structs

### `Regex`

```rust
struct Regex {
    forward: crate::hybrid::dfa::DFA,
    reverse: crate::hybrid::dfa::DFA,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/regex.rs:82-96`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/regex.rs#L82-L96)*

A regular expression that uses hybrid NFA/DFAs (also called "lazy DFAs")
for searching.

A regular expression is comprised of two lazy DFAs, a "forward" DFA and a
"reverse" DFA. The forward DFA is responsible for detecting the end of
a match while the reverse DFA is responsible for detecting the start
of a match. Thus, in order to find the bounds of any given match, a
forward search must first be run followed by a reverse search. A match
found by the forward DFA guarantees that the reverse DFA will also find
a match.

# Fallibility

Most of the search routines defined on this type will _panic_ when the
underlying search fails. This might be because the DFA gave up because it
saw a quit byte, whether configured explicitly or via heuristic Unicode
word boundary support, although neither are enabled by default. It might
also fail if the underlying DFA determines it isn't making effective use of
the cache (which also never happens by default). Or it might fail because
an invalid `Input` configuration is given, for example, with an unsupported
[`Anchored`](../../index.md) mode.

If you need to handle these error cases instead of allowing them to trigger
a panic, then the lower level `Regex::try_search` provides a fallible API
that never panics.

# Example

This example shows how to cause a search to terminate if it sees a
`\n` byte, and handle the error returned. This could be useful if, for
example, you wanted to prevent a user supplied pattern from matching
across a line boundary.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{hybrid::{dfa, regex::Regex}, Input, MatchError};

let re = Regex::builder()
    .dfa(dfa::Config::new().quit(b'\n', true))
    .build(r"foo\p{any}+bar")?;
let mut cache = re.create_cache();

let input = Input::new("foo\nbar");
// Normally this would produce a match, since \p{any} contains '\n'.
// But since we instructed the automaton to enter a quit state if a
// '\n' is observed, this produces a match error instead.
let expected = MatchError::quit(b'\n', 3);
let got = re.try_search(&mut cache, &input).unwrap_err();
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`forward`**: `crate::hybrid::dfa::DFA`

  The forward lazy DFA. This can only find the end of a match.

- **`reverse`**: `crate::hybrid::dfa::DFA`

  The reverse lazy DFA. This can only find the start of a match.
  
  This is built with 'all' match semantics (instead of leftmost-first)
  so that it always finds the longest possible match (which corresponds
  to the leftmost starting position). It is also compiled as an anchored
  matcher and has 'starts_for_each_pattern' enabled. Including starting
  states for each pattern is necessary to ensure that we only look for
  matches of a pattern that matched in the forward direction. Otherwise,
  we might wind up finding the "leftmost" starting position of a totally
  different pattern!

#### Implementations

- <span id="regex-new"></span>`fn new(pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

- <span id="regex-new-many"></span>`fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

- <span id="regex-builder"></span>`fn builder() -> Builder` — [`Builder`](#builder)

- <span id="regex-create-cache"></span>`fn create_cache(&self) -> Cache` — [`Cache`](#cache)

- <span id="regex-reset-cache"></span>`fn reset_cache(&self, cache: &mut Cache)` — [`Cache`](#cache)

#### Trait Implementations

##### `impl Debug for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FindMatches<'r, 'c, 'h>`

```rust
struct FindMatches<'r, 'c, 'h> {
    re: &'r Regex,
    cache: &'c mut Cache,
    it: iter::Searcher<'h>,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/regex.rs:569-573`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/regex.rs#L569-L573)*

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.
If the underlying regex engine returns an error, then a panic occurs.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the regex object.
* `'h` represents the lifetime of the haystack being searched.
* `'c` represents the lifetime of the regex cache.

This iterator can be created with the `Regex::find_iter` method.

#### Trait Implementations

##### `impl Debug for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findmatches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findmatches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindMatches<'r, 'c, 'h>`

- <span id="findmatches-iterator-type-item"></span>`type Item = Match`

- <span id="findmatches-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../index.md#match)

### `Cache`

```rust
struct Cache {
    forward: dfa::Cache,
    reverse: dfa::Cache,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/regex.rs:601-604`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/regex.rs#L601-L604)*

A cache represents a partially computed forward and reverse DFA.

A cache is the key component that differentiates a classical DFA and a
hybrid NFA/DFA (also called a "lazy DFA"). Where a classical DFA builds a
complete transition table that can handle all possible inputs, a hybrid
NFA/DFA starts with an empty transition table and builds only the parts
required during search. The parts that are built are stored in a cache. For
this reason, a cache is a required parameter for nearly every operation on
a [`Regex`](#regex).

Caches can be created from their corresponding `Regex` via
`Regex::create_cache`. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with `Cache::reset`. Using a cache with any other `Regex` may result in
panics or incorrect results.

#### Implementations

- <span id="cache-new"></span>`fn new(re: &Regex) -> Cache` — [`Regex`](#regex), [`Cache`](#cache)

- <span id="cache-reset"></span>`fn reset(&mut self, re: &Regex)` — [`Regex`](#regex)

- <span id="cache-forward"></span>`fn forward(&mut self) -> &dfa::Cache` — [`Cache`](../dfa/index.md#cache)

- <span id="cache-reverse"></span>`fn reverse(&mut self) -> &dfa::Cache` — [`Cache`](../dfa/index.md#cache)

- <span id="cache-forward-mut"></span>`fn forward_mut(&mut self) -> &mut dfa::Cache` — [`Cache`](../dfa/index.md#cache)

- <span id="cache-reverse-mut"></span>`fn reverse_mut(&mut self) -> &mut dfa::Cache` — [`Cache`](../dfa/index.md#cache)

- <span id="cache-as-parts"></span>`fn as_parts(&self) -> (&dfa::Cache, &dfa::Cache)` — [`Cache`](../dfa/index.md#cache)

- <span id="cache-as-parts-mut"></span>`fn as_parts_mut(&mut self) -> (&mut dfa::Cache, &mut dfa::Cache)` — [`Cache`](../dfa/index.md#cache)

- <span id="cache-memory-usage"></span>`fn memory_usage(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Cache`

- <span id="cache-clone"></span>`fn clone(&self) -> Cache` — [`Cache`](#cache)

##### `impl Debug for Cache`

- <span id="cache-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Builder`

```rust
struct Builder {
    dfa: dfa::Builder,
}
```

*Defined in [`regex-automata-0.4.13/src/hybrid/regex.rs:767-769`](../../../../.source_1765210505/regex-automata-0.4.13/src/hybrid/regex.rs#L767-L769)*

A builder for a regex based on a hybrid NFA/DFA.

This builder permits configuring options for the syntax of a pattern, the
NFA construction, the lazy DFA construction and finally the regex searching
itself. This builder is different from a general purpose regex builder
in that it permits fine grain configuration of the construction process.
The trade off for this is complexity, and the possibility of setting a
configuration that might not make sense. For example, there are two
different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* `thompson::Config::utf8` controls how the regex iterators themselves
advance the starting position of the next search when a match with zero
length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

Internally, building a regex requires building two hybrid NFA/DFAs,
where one is responsible for finding the end of a match and the other is
responsible for finding the start of a match. If you only need to detect
whether something matched, or only the end of a match, then you should use
a [`dfa::Builder`](../dfa/index.md) to construct a single hybrid NFA/DFA, which is cheaper
than building two of them.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    hybrid::regex::Regex, nfa::thompson, util::syntax, Match,
};

let re = Regex::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let mut cache = re.create_cache();

let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find(&mut cache, haystack);
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- <span id="builder-new"></span>`fn new() -> Builder` — [`Builder`](#builder)

- <span id="builder-build"></span>`fn build(&self, pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

- <span id="builder-build-many"></span>`fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../error/index.md#builderror)

- <span id="builder-build-from-dfas"></span>`fn build_from_dfas(&self, forward: DFA, reverse: DFA) -> Regex` — [`DFA`](../dfa/index.md#dfa), [`Regex`](#regex)

- <span id="builder-syntax"></span>`fn syntax(&mut self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md#config), [`Builder`](#builder)

- <span id="builder-thompson"></span>`fn thompson(&mut self, config: thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md#config), [`Builder`](#builder)

- <span id="builder-dfa"></span>`fn dfa(&mut self, config: dfa::Config) -> &mut Builder` — [`Config`](../dfa/index.md#config), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- <span id="builder-clone"></span>`fn clone(&self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- <span id="builder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Builder`

- <span id="builder-default"></span>`fn default() -> Builder` — [`Builder`](#builder)

