*[regex_automata](../../index.md) / [dfa](../index.md) / [regex](index.md)*

---

# Module `regex`

A DFA-backed `Regex`.

This module provides [`Regex`](#regex), which is defined generically over the
[`Automaton`](../automaton/index.md) trait. A `Regex` implements convenience routines you might have
come to expect, such as finding the start/end of a match and iterating over
all non-overlapping matches. This `Regex` type is limited in its capabilities
to what a DFA can provide. Therefore, APIs involving capturing groups, for
example, are not provided.

Internally, a `Regex` is composed of two DFAs. One is a "forward" DFA that
finds the end offset of a match, where as the other is a "reverse" DFA that
find the start offset of a match.

See the [parent module](crate::dfa) for examples.

## Structs

### `Regex<A>`

```rust
struct Regex<A> {
    forward: A,
    reverse: A,
}
```

A regular expression that uses deterministic finite automata for fast
searching.

A regular expression is comprised of two DFAs, a "forward" DFA and a
"reverse" DFA. The forward DFA is responsible for detecting the end of
a match while the reverse DFA is responsible for detecting the start
of a match. Thus, in order to find the bounds of any given match, a
forward search must first be run followed by a reverse search. A match
found by the forward DFA guarantees that the reverse DFA will also find
a match.

The type of the DFA used by a `Regex` corresponds to the `A` type
parameter, which must satisfy the [`Automaton`](../automaton/index.md) trait. Typically, `A`
is either a `dense::DFA` or a `sparse::DFA`, where dense DFAs use
more memory but search faster, while sparse DFAs use less memory but
search more slowly.

# Crate features

Note that despite what the documentation auto-generates, the _only_
crate feature needed to use this type is `dfa-search`. You do _not_
need to enable the `alloc` feature.

By default, a regex's automaton type parameter is set to
`dense::DFA<Vec<u32>>` when the `alloc` feature is enabled. For most
in-memory work loads, this is the most convenient type that gives the
best search performance. When the `alloc` feature is disabled, no
default type is used.

# When should I use this?

Generally speaking, if you can afford the overhead of building a full
DFA for your regex, and you don't need things like capturing groups,
then this is a good choice if you're looking to optimize for matching
speed. Note however that its speed may be worse than a general purpose
regex engine if you don't provide a `dense::Config::prefilter` to the
underlying DFA.

# Sparse DFAs

Since a `Regex` is generic over the [`Automaton`](../automaton/index.md) trait, it can be
used with any kind of DFA. While this crate constructs dense DFAs by
default, it is easy enough to build corresponding sparse DFAs, and then
build a regex from them:

```rust
use regex_automata::dfa::regex::Regex;

// First, build a regex that uses dense DFAs.
let dense_re = Regex::new("foo[0-9]+")?;

// Second, build sparse DFAs from the forward and reverse dense DFAs.
let fwd = dense_re.forward().to_sparse()?;
let rev = dense_re.reverse().to_sparse()?;

// Third, build a new regex from the constituent sparse DFAs.
let sparse_re = Regex::builder().build_from_dfas(fwd, rev);

// A regex that uses sparse DFAs can be used just like with dense DFAs.
assert_eq!(true, sparse_re.is_match(b"foo123"));

Ok::<(), Box<dyn std::error::Error>>(())
```

Alternatively, one can use a [`Builder`](../../meta/regex/index.md) to construct a sparse DFA
more succinctly. (Note though that dense DFAs are still constructed
first internally, and then converted to sparse DFAs, as in the example
above.)

```rust
use regex_automata::dfa::regex::Regex;

let sparse_re = Regex::builder().build_sparse(r"foo[0-9]+")?;
// A regex that uses sparse DFAs can be used just like with dense DFAs.
assert!(sparse_re.is_match(b"foo123"));

Ok::<(), Box<dyn std::error::Error>>(())
```

# Fallibility

Most of the search routines defined on this type will _panic_ when the
underlying search fails. This might be because the DFA gave up because
it saw a quit byte, whether configured explicitly or via heuristic
Unicode word boundary support, although neither are enabled by default.
Or it might fail because an invalid `Input` configuration is given,
for example, with an unsupported [`Anchored`](../../index.md) mode.

If you need to handle these error cases instead of allowing them to
trigger a panic, then the lower level `Regex::try_search` provides
a fallible API that never panics.

# Example

This example shows how to cause a search to terminate if it sees a
`\n` byte, and handle the error returned. This could be useful if, for
example, you wanted to prevent a user supplied pattern from matching
across a line boundary.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{dfa::{self, regex::Regex}, Input, MatchError};

let re = Regex::builder()
    .dense(dfa::dense::Config::new().quit(b'\n', true))
    .build(r"foo\p{any}+bar")?;

let input = Input::new("foo\nbar");
// Normally this would produce a match, since \p{any} contains '\n'.
// But since we instructed the automaton to enter a quit state if a
// '\n' is observed, this produces a match error instead.
let expected = MatchError::quit(b'\n', 3);
let got = re.try_search(&input).unwrap_err();
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn try_search(self: &Self, input: &Input<'_>) -> Result<Option<Match>, MatchError>` — [`Input`](../../index.md), [`Match`](../../index.md), [`MatchError`](../../index.md)

- `fn is_anchored(self: &Self, input: &Input<'_>) -> bool` — [`Input`](../../index.md)

#### Trait Implementations

##### `impl<A: $crate::clone::Clone> Clone for Regex<A>`

- `fn clone(self: &Self) -> Regex<A>` — [`Regex`](#regex)

##### `impl<A: $crate::fmt::Debug> Debug for Regex<A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindMatches<'r, 'h, A>`

```rust
struct FindMatches<'r, 'h, A> {
    re: &'r Regex<A>,
    it: iter::Searcher<'h>,
}
```

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.
If the underlying regex engine returns an error, then a panic occurs.

The type parameters are as follows:

* `A` represents the type of the underlying DFA that implements the
[`Automaton`](../automaton/index.md) trait.

The lifetime parameters are as follows:

* `'h` represents the lifetime of the haystack being searched.
* `'r` represents the lifetime of the regex object itself.

This iterator can be created with the `Regex::find_iter` method.

#### Trait Implementations

##### `impl<'r, 'h, A: $crate::fmt::Debug> Debug for FindMatches<'r, 'h, A>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindMatches<'r, 'h, A>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'r, 'h, A: Automaton> Iterator for FindMatches<'r, 'h, A>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>` — [`Match`](../../index.md)

### `Builder`

```rust
struct Builder {
    dfa: dense::Builder,
}
```

A builder for a regex based on deterministic finite automatons.

This builder permits configuring options for the syntax of a pattern, the
NFA construction, the DFA construction and finally the regex searching
itself. This builder is different from a general purpose regex builder in
that it permits fine grain configuration of the construction process. The
trade off for this is complexity, and the possibility of setting a
configuration that might not make sense. For example, there are two
different UTF-8 modes:

* [`syntax::Config::utf8`](crate::util::syntax::Config::utf8) controls
whether the pattern itself can contain sub-expressions that match invalid
UTF-8.
* [`thompson::Config::utf8`](crate::nfa::thompson::Config::utf8) controls
how the regex iterators themselves advance the starting position of the
next search when a match with zero length is found.

Generally speaking, callers will want to either enable all of these or
disable all of these.

Internally, building a regex requires building two DFAs, where one is
responsible for finding the end of a match and the other is responsible
for finding the start of a match. If you only need to detect whether
something matched, or only the end of a match, then you should use a
`dense::Builder` to construct a single DFA, which is cheaper than
building two DFAs.

# Build methods

This builder has a few "build" methods. In general, it's the result of
combining the following parameters:

* Building one or many regexes.
* Building a regex with dense or sparse DFAs.

The simplest "build" method is `Builder::build`. It accepts a single
pattern and builds a dense DFA using `usize` for the state identifier
representation.

The most general "build" method is `Builder::build_many`, which permits
building a regex that searches for multiple patterns simultaneously while
using a specific state identifier representation.

The most flexible "build" method, but hardest to use, is
`Builder::build_from_dfas`. This exposes the fact that a [`Regex`](#regex) is
just a pair of DFAs, and this method allows you to specify those DFAs
exactly.

# Example

This example shows how to disable UTF-8 mode in the syntax and the regex
itself. This is generally what you want for matching on arbitrary bytes.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    dfa::regex::Regex, nfa::thompson, util::syntax, Match,
};

let re = Regex::builder()
    .syntax(syntax::Config::new().utf8(false))
    .thompson(thompson::Config::new().utf8(false))
    .build(r"foo(?-u:[^b])ar.*")?;
let haystack = b"\xFEfoo\xFFarzz\xE2\x98\xFF\n";
let expected = Some(Match::must(0, 1..9));
let got = re.find(haystack);
assert_eq!(expected, got);
// Notice that `(?-u:[^b])` matches invalid UTF-8,
// but the subsequent `.*` does not! Disabling UTF-8
// on the syntax permits this.
assert_eq!(b"foo\xFFarzz", &haystack[got.unwrap().range()]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder` — [`Builder`](#builder)

- `fn build(self: &Self, pattern: &str) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../dense/index.md)

- `fn build_sparse(self: &Self, pattern: &str) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError>` — [`Regex`](#regex), [`DFA`](../sparse/index.md), [`BuildError`](../dense/index.md)

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<Regex, BuildError>` — [`Regex`](#regex), [`BuildError`](../dense/index.md)

- `fn build_many_sparse<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<Regex<sparse::DFA<Vec<u8>>>, BuildError>` — [`Regex`](#regex), [`DFA`](../sparse/index.md), [`BuildError`](../dense/index.md)

- `fn build_from_dfas<A: Automaton>(self: &Self, forward: A, reverse: A) -> Regex<A>` — [`Regex`](#regex)

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder` — [`Config`](../../util/syntax/index.md), [`Builder`](#builder)

- `fn thompson(self: &mut Self, config: crate::nfa::thompson::Config) -> &mut Builder` — [`Config`](../../nfa/thompson/compiler/index.md), [`Builder`](#builder)

- `fn dense(self: &mut Self, config: dense::Config) -> &mut Builder` — [`Config`](../dense/index.md), [`Builder`](#builder)

#### Trait Implementations

##### `impl Clone for Builder`

- `fn clone(self: &Self) -> Builder` — [`Builder`](#builder)

##### `impl Debug for Builder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Builder`

- `fn default() -> Builder` — [`Builder`](#builder)

