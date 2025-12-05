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

## Structs

### `Regex`

```rust
struct Regex {
    // [REDACTED: Private Fields]
}
```

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

#### Implementations

- `fn new(pattern: &str) -> Result<Regex, BuildError>`
  Parse the given regular expression using the default configuration and

- `fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>`
  Like `new`, but parses multiple patterns into a single "multi regex."

- `fn builder() -> Builder`
  Return a builder for configuring the construction of a `Regex`.

- `fn create_cache(self: &Self) -> Cache`
  Create a new cache for this `Regex`.

- `fn reset_cache(self: &Self, cache: &mut Cache)`
  Reset the given cache such that it can be used for searching with the

- `fn forward(self: &Self) -> &DFA`
  Return the underlying lazy DFA responsible for forward matching.

- `fn reverse(self: &Self) -> &DFA`
  Return the underlying lazy DFA responsible for reverse matching.

- `fn pattern_len(self: &Self) -> usize`
  Returns the total number of patterns matched by this regex.

- `fn try_search(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Result<Option<Match>, MatchError>`
  Returns the start and end offset of the leftmost match. If no match

- `fn is_match<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> bool`
  Returns true if and only if this regex matches the given haystack.

- `fn find<'h, I: Into<Input<'h>>>(self: &Self, cache: &mut Cache, input: I) -> Option<Match>`
  Returns the start and end offset of the leftmost match. If no match

- `fn find_iter<'r, 'c, 'h, I: Into<Input<'h>>>(self: &'r Self, cache: &'c mut Cache, input: I) -> FindMatches<'r, 'c, 'h>`
  Returns an iterator over all non-overlapping leftmost matches in the

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

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.
If the underlying regex engine returns an error, then a panic occurs.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the regex object.
* `'h` represents the lifetime of the haystack being searched.
* `'c` represents the lifetime of the regex cache.

This iterator can be created with the `Regex::find_iter` method.

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

### `Cache`

```rust
struct Cache {
    // [REDACTED: Private Fields]
}
```

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

- `fn new(re: &Regex) -> Cache`
  Create a new cache for the given `Regex`.

- `fn reset(self: &mut Self, re: &Regex)`
  Reset this cache such that it can be used for searching with the given

- `fn forward(self: &mut Self) -> &dfa::Cache`
  Return a reference to the forward cache.

- `fn reverse(self: &mut Self) -> &dfa::Cache`
  Return a reference to the reverse cache.

- `fn forward_mut(self: &mut Self) -> &mut dfa::Cache`
  Return a mutable reference to the forward cache.

- `fn reverse_mut(self: &mut Self) -> &mut dfa::Cache`
  Return a mutable reference to the reverse cache.

- `fn as_parts(self: &Self) -> (&dfa::Cache, &dfa::Cache)`
  Return references to the forward and reverse caches, respectively.

- `fn as_parts_mut(self: &mut Self) -> (&mut dfa::Cache, &mut dfa::Cache)`
  Return mutable references to the forward and reverse caches,

- `fn memory_usage(self: &Self) -> usize`
  Returns the heap memory usage, in bytes, as a sum of the forward and

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

### `Builder`

```rust
struct Builder {
    // [REDACTED: Private Fields]
}
```

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
a `dfa::Builder` to construct a single hybrid NFA/DFA, which is cheaper
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

- `fn new() -> Builder`
  Create a new regex builder with the default configuration.

- `fn build(self: &Self, pattern: &str) -> Result<Regex, BuildError>`
  Build a regex from the given pattern.

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<Regex, BuildError>`
  Build a regex from the given patterns.

- `fn build_from_dfas(self: &Self, forward: DFA, reverse: DFA) -> Regex`
  Build a regex from its component forward and reverse hybrid NFA/DFAs.

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder`
  Set the syntax configuration for this builder using

- `fn thompson(self: &mut Self, config: thompson::Config) -> &mut Builder`
  Set the Thompson NFA configuration for this builder using

- `fn dfa(self: &mut Self, config: dfa::Config) -> &mut Builder`
  Set the lazy DFA compilation configuration for this builder using

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

