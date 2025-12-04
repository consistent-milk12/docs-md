*[regex_automata](../index.md) / [meta](index.md)*

---

# Module `meta`

Provides a regex matcher that composes several other regex matchers
automatically.

This module is home to a meta [`Regex`](../hybrid/regex/index.md), which provides a convenient high
level API for executing regular expressions in linear time.

# Comparison with the `regex` crate

A meta `Regex` is the implementation used directly by the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over a meta
`Regex`. This means that if you need the full flexibility offered by this
API, then you should be able to switch to using this API directly without
any changes in match semantics or syntax. However, there are some API level
differences:

* The `regex` crate API returns match objects that include references to the
haystack itself, which in turn makes it easy to access the matching strings
without having to slice the haystack yourself. In contrast, a meta `Regex`
returns match objects that only have offsets in them.
* At time of writing, a meta `Regex` doesn't have some of the convenience
routines that the `regex` crate has, such as replacements. Note though that
[`Captures::interpolate_string`](crate::util::captures::Captures::interpolate_string)
will handle the replacement string interpolation for you.
* A meta `Regex` supports the [`Input`](crate::Input) abstraction, which
provides a way to configure a search in more ways than is supported by the
`regex` crate. For example, [`Input::anchored`](crate::Input::anchored) can
be used to run an anchored search, regardless of whether the pattern is itself
anchored with a `^`.
* A meta `Regex` supports multi-pattern searching everywhere.
Indeed, every [`Match`](crate::Match) returned by the search APIs
include a [`PatternID`](crate::PatternID) indicating which pattern
matched. In the single pattern case, all matches correspond to
[`PatternID::ZERO`](crate::PatternID::ZERO). In contrast, the `regex` crate
has distinct `Regex` and a `RegexSet` APIs. The former only supports a single
pattern, while the latter supports multiple patterns but cannot report the
offsets of a match.
* A meta `Regex` provides the explicit capability of bypassing its internal
memory pool for automatically acquiring mutable scratch space required by its
internal regex engines. Namely, a [`Cache`](../hybrid/dfa/index.md) can be explicitly provided to lower
level routines such as `Regex::search_with`.

## Structs

### `BuildError`

```rust
struct BuildError {
    // [REDACTED: Private Fields]
}
```

An error that occurs when construction of a `Regex` fails.

A build error is generally a result of one of two possible failure
modes. First is a parse or syntax error in the concrete syntax of a
pattern. Second is that the construction of the underlying regex matcher
fails, usually because it gets too big with respect to limits like
[`Config::nfa_size_limit`](crate::meta::Config::nfa_size_limit).

This error provides very little introspection capabilities. You can:

* Ask for the [`PatternID`](../util/primitives/index.md) of the pattern that caused an error, if one
is available. This is available for things like syntax errors, but not for
cases where build limits are exceeded.
* Ask for the underlying syntax error, but only if the error is a syntax
error.
* Ask for a human readable message corresponding to the underlying error.
* The `BuildError::source` method (from the `std::error::Error`
trait implementation) may be used to query for an underlying error if one
exists. There are no API guarantees about which error is returned.

When the `std` feature is enabled, this implements `std::error::Error`.

#### Implementations

- `fn pattern(self: &Self) -> Option<PatternID>`
  If it is known which pattern ID caused this build error to occur, then

- `fn size_limit(self: &Self) -> Option<usize>`
  If this error occurred because the regex exceeded the configured size

- `fn syntax_error(self: &Self) -> Option<&regex_syntax::Error>`
  If this error corresponds to a syntax error, then a reference to it is

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

- `fn clone(self: &Self) -> BuildError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error`

- `fn source(self: &Self) -> Option<&dyn std::error::Error>`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl ToString<T>`

- `fn to_string(self: &Self) -> String`

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

A builder for configuring and constructing a `Regex`.

The builder permits configuring two different aspects of a `Regex`:

* `Builder::configure` will set high-level configuration options as
described by a [`Config`](../dfa/onepass/index.md).
* `Builder::syntax` will set the syntax level configuration options
as described by a [`util::syntax::Config`](crate::util::syntax::Config).
This only applies when building a `Regex` from pattern strings.

Once configured, the builder can then be used to construct a `Regex` from
one of 4 different inputs:

* `Builder::build` creates a regex from a single pattern string.
* `Builder::build_many` creates a regex from many pattern strings.
* `Builder::build_from_hir` creates a regex from a
[`regex-syntax::Hir`](Hir) expression.
* `Builder::build_many_from_hir` creates a regex from many
[`regex-syntax::Hir`](Hir) expressions.

The latter two methods in particular provide a way to construct a fully
feature regular expression matcher directly from an `Hir` expression
without having to first convert it to a string. (This is in contrast to the
top-level `regex` crate which intentionally provides no such API in order
to avoid making `regex-syntax` a public dependency.)

As a convenience, this builder may be created via `Regex::builder`, which
may help avoid an extra import.

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change the
line terminator to the NUL byte:

```
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: disable UTF-8 requirement

By default, regex patterns are required to match UTF-8. This includes
regex patterns that can produce matches of length zero. In the case of an
empty match, by default, matches will not appear between the code units of
a UTF-8 encoded codepoint.

However, it can be useful to disable this requirement, particularly if
you're searching things like `&[u8](#u8)
` that are not known to be valid UTF-8.

```
use regex_automata::{meta::Regex, util::syntax, Match};

let mut builder = Regex::builder();
// Disables the requirement that non-empty matches match UTF-8.
builder.syntax(syntax::Config::new().utf8(false));
// Disables the requirement that empty matches match UTF-8 boundaries.
builder.configure(Regex::config().utf8_empty(false));

// We can match raw bytes via \xZZ syntax, but we need to disable
// Unicode mode to do that. We could disable it everywhere, or just
// selectively, as shown here.
let re = builder.build(r"(?-u:\xFF)foo(?-u:\xFF)")?;
let hay = b"\xFFfoo\xFF";
assert_eq!(Some(Match::must(0, 0..5)), re.find(hay));

// We can also match between code units.
let re = builder.build(r"")?;
let hay = "☃";
assert_eq!(re.find_iter(hay).collect::<Vec<Match>>(), vec![
    Match::must(0, 0..0),
    Match::must(0, 1..1),
    Match::must(0, 2..2),
    Match::must(0, 3..3),
]);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Builder`
  Creates a new builder for configuring and constructing a [`Regex`].

- `fn build(self: &Self, pattern: &str) -> Result<Regex, BuildError>`
  Builds a `Regex` from a single pattern string.

- `fn build_many<P: AsRef<str>>(self: &Self, patterns: &[P]) -> Result<Regex, BuildError>`
  Builds a `Regex` from many pattern strings.

- `fn build_from_hir(self: &Self, hir: &Hir) -> Result<Regex, BuildError>`
  Builds a `Regex` directly from an `Hir` expression.

- `fn build_many_from_hir<H: Borrow<Hir>>(self: &Self, hirs: &[H]) -> Result<Regex, BuildError>`
  Builds a `Regex` directly from many `Hir` expressions.

- `fn configure(self: &mut Self, config: Config) -> &mut Builder`
  Configure the behavior of a `Regex`.

- `fn syntax(self: &mut Self, config: crate::util::syntax::Config) -> &mut Builder`
  Configure the syntax options when parsing a pattern string while

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

### `Cache`

```rust
struct Cache {
    // [REDACTED: Private Fields]
}
```

Represents mutable scratch space used by regex engines during a search.

Most of the regex engines in this crate require some kind of
mutable state in order to execute a search. This mutable state is
explicitly separated from the core regex object (such as a
[`thompson::NFA`](crate::nfa::thompson::NFA)) so that the read-only regex
object can be shared across multiple threads simultaneously without any
synchronization. Conversely, a `Cache` must either be duplicated if using
the same `Regex` from multiple threads, or else there must be some kind of
synchronization that guarantees exclusive access while it's in use by one
thread.

A `Regex` attempts to do this synchronization for you by using a thread
pool internally. Its size scales roughly with the number of simultaneous
regex searches.

For cases where one does not want to rely on a `Regex`'s internal thread
pool, lower level routines such as `Regex::search_with` are provided
that permit callers to pass a `Cache` into the search routine explicitly.

General advice is that the thread pool is often more than good enough.
However, it may be possible to observe the effects of its latency,
especially when searching many small haystacks from many threads
simultaneously.

Caches can be created from their corresponding `Regex` via
`Regex::create_cache`. A cache can only be used with either the `Regex`
that created it, or the `Regex` that was most recently used to reset it
with `Cache::reset`. Using a cache with any other `Regex` may result in
panics or incorrect results.

# Example

```
use regex_automata::{meta::Regex, Input, Match};

let re = Regex::new(r"(?-u)m\w+\s+m\w+")?;
let mut cache = re.create_cache();
let input = Input::new("crazy janey and her mission man");
assert_eq!(
    Some(Match::must(0, 20..31)),
    re.search_with(&mut cache, &input),
);

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new(re: &Regex) -> Cache`
  Creates a new `Cache` for use with this regex.

- `fn reset(self: &mut Self, re: &Regex)`
  Reset this cache such that it can be used for searching with the given

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

### `CapturesMatches<'r, 'h>`

```rust
struct CapturesMatches<'r, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over all non-overlapping leftmost matches with their capturing
groups.

The iterator yields a [`Captures`](../util/captures/index.md) value until no more matches could be
found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::captures_iter` method.

#### Implementations

- `fn regex(self: &Self) -> &'r Regex`
  Returns the `Regex` value that created this iterator.

- `fn input<'s>(self: &'s Self) -> &'s Input<'h>`
  Returns the current `Input` associated with this iterator.

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

##### `impl FusedIterator<'r, 'h>`

##### `impl Iterator<'r, 'h>`

- `type Item = Captures`

- `fn next(self: &mut Self) -> Option<Captures>`

- `fn count(self: Self) -> usize`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Config`

```rust
struct Config {
    // [REDACTED: Private Fields]
}
```

An object describing the configuration of a `Regex`.

This configuration only includes options for the
non-syntax behavior of a `Regex`, and can be applied via the
`Builder::configure` method. For configuring the syntax options, see
[`util::syntax::Config`](crate::util::syntax::Config).

# Example: lower the NFA size limit

In some cases, the default size limit might be too big. The size limit can
be lowered, which will prevent large regex patterns from compiling.

```
# if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::meta::Regex;

let result = Regex::builder()
    .configure(Regex::config().nfa_size_limit(Some(20 * (1<<10))))
    // Not even 20KB is enough to build a single large Unicode class!
    .build(r"\pL");
assert!(result.is_err());

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn new() -> Config`
  Create a new configuration object for a `Regex`.

- `fn match_kind(self: Self, kind: MatchKind) -> Config`
  Set the match semantics for a `Regex`.

- `fn utf8_empty(self: Self, yes: bool) -> Config`
  Toggles whether empty matches are permitted to occur between the code

- `fn auto_prefilter(self: Self, yes: bool) -> Config`
  Toggles whether automatic prefilter support is enabled.

- `fn prefilter(self: Self, pre: Option<Prefilter>) -> Config`
  Overrides and sets the prefilter to use inside a `Regex`.

- `fn which_captures(self: Self, which_captures: WhichCaptures) -> Config`
  Configures what kinds of groups are compiled as "capturing" in the

- `fn nfa_size_limit(self: Self, limit: Option<usize>) -> Config`
  Sets the size limit, in bytes, to enforce on the construction of every

- `fn onepass_size_limit(self: Self, limit: Option<usize>) -> Config`
  Sets the size limit, in bytes, for the one-pass DFA.

- `fn hybrid_cache_capacity(self: Self, limit: usize) -> Config`
  Set the cache capacity, in bytes, for the lazy DFA.

- `fn dfa_size_limit(self: Self, limit: Option<usize>) -> Config`
  Sets the size limit, in bytes, for heap memory used for a fully

- `fn dfa_state_limit(self: Self, limit: Option<usize>) -> Config`
  Sets a limit on the total number of NFA states, beyond which, a full

- `fn byte_classes(self: Self, yes: bool) -> Config`
  Whether to attempt to shrink the size of the alphabet for the regex

- `fn line_terminator(self: Self, byte: u8) -> Config`
  Set the line terminator to be used by the `^` and `$` anchors in

- `fn hybrid(self: Self, yes: bool) -> Config`
  Toggle whether the hybrid NFA/DFA (also known as the "lazy DFA") should

- `fn dfa(self: Self, yes: bool) -> Config`
  Toggle whether a fully compiled DFA should be available for use by the

- `fn onepass(self: Self, yes: bool) -> Config`
  Toggle whether a one-pass DFA should be available for use by the meta

- `fn backtrack(self: Self, yes: bool) -> Config`
  Toggle whether a bounded backtracking regex engine should be available

- `fn get_match_kind(self: &Self) -> MatchKind`
  Returns the match kind on this configuration, as set by

- `fn get_utf8_empty(self: &Self) -> bool`
  Returns whether empty matches must fall on valid UTF-8 boundaries, as

- `fn get_auto_prefilter(self: &Self) -> bool`
  Returns whether automatic prefilters are enabled, as set by

- `fn get_prefilter(self: &Self) -> Option<&Prefilter>`
  Returns a manually set prefilter, if one was set by

- `fn get_which_captures(self: &Self) -> WhichCaptures`
  Returns the capture configuration, as set by

- `fn get_nfa_size_limit(self: &Self) -> Option<usize>`
  Returns NFA size limit, as set by [`Config::nfa_size_limit`].

- `fn get_onepass_size_limit(self: &Self) -> Option<usize>`
  Returns one-pass DFA size limit, as set by

- `fn get_hybrid_cache_capacity(self: &Self) -> usize`
  Returns hybrid NFA/DFA cache capacity, as set by

- `fn get_dfa_size_limit(self: &Self) -> Option<usize>`
  Returns DFA size limit, as set by [`Config::dfa_size_limit`].

- `fn get_dfa_state_limit(self: &Self) -> Option<usize>`
  Returns DFA size limit in terms of the number of states in the NFA, as

- `fn get_byte_classes(self: &Self) -> bool`
  Returns whether byte classes are enabled, as set by

- `fn get_line_terminator(self: &Self) -> u8`
  Returns the line terminator for this configuration, as set by

- `fn get_hybrid(self: &Self) -> bool`
  Returns whether the hybrid NFA/DFA regex engine may be used, as set by

- `fn get_dfa(self: &Self) -> bool`
  Returns whether the DFA regex engine may be used, as set by

- `fn get_onepass(self: &Self) -> bool`
  Returns whether the one-pass DFA regex engine may be used, as set by

- `fn get_backtrack(self: &Self) -> bool`
  Returns whether the bounded backtracking regex engine may be used, as

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

### `FindMatches<'r, 'h>`

```rust
struct FindMatches<'r, 'h> {
    // [REDACTED: Private Fields]
}
```

An iterator over all non-overlapping matches.

The iterator yields a [`Match`](../index.md) value until no more matches could be found.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::find_iter` method.

#### Implementations

- `fn regex(self: &Self) -> &'r Regex`
  Returns the `Regex` value that created this iterator.

- `fn input<'s>(self: &'s Self) -> &'s Input<'h>`
  Returns the current `Input` associated with this iterator.

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

##### `impl FusedIterator<'r, 'h>`

##### `impl Iterator<'r, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

- `fn count(self: Self) -> usize`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `Regex`

```rust
struct Regex {
    // [REDACTED: Private Fields]
}
```

A regex matcher that works by composing several other regex matchers
automatically.

In effect, a meta regex papers over a lot of the quirks or performance
problems in each of the regex engines in this crate. Its goal is to provide
an infallible and simple API that "just does the right thing" in the common
case.

A meta regex is the implementation of a `Regex` in the `regex` crate.
Indeed, the `regex` crate API is essentially just a light wrapper over
this type. This includes the `regex` crate's `RegexSet` API!

# Composition

This is called a "meta" matcher precisely because it uses other regex
matchers to provide a convenient high level regex API. Here are some
examples of how other regex matchers are composed:

* When calling `Regex::captures`, instead of immediately
running a slower but more capable regex engine like the
[`PikeVM`](crate::nfa::thompson::pikevm::PikeVM), the meta regex engine
will usually first look for the bounds of a match with a higher throughput
regex engine like a [lazy DFA](crate::hybrid). Only when a match is found
is a slower engine like `PikeVM` used to find the matching span for each
capture group.
* While higher throughout engines like the lazy DFA cannot handle
Unicode word boundaries in general, they can still be used on pure ASCII
haystacks by pretending that Unicode word boundaries are just plain ASCII
word boundaries. However, if a haystack is not ASCII, the meta regex engine
will automatically switch to a (possibly slower) regex engine that supports
Unicode word boundaries in general.
* In some cases where a regex pattern is just a simple literal or a small
set of literals, an actual regex engine won't be used at all. Instead,
substring or multi-substring search algorithms will be employed.

There are many other forms of composition happening too, but the above
should give a general idea. In particular, it may perhaps be surprising
that *multiple* regex engines might get executed for a single search. That
is, the decision of what regex engine to use is not _just_ based on the
pattern, but also based on the dynamic execution of the search itself.

The primary reason for this composition is performance. The fundamental
tension is that the faster engines tend to be less capable, and the more
capable engines tend to be slower.

Note that the forms of composition that are allowed are determined by
compile time crate features and configuration. For example, if the `hybrid`
feature isn't enabled, or if `Config::hybrid` has been disabled, then the
meta regex engine will never use a lazy DFA.

# Synchronization and cloning

Most of the regex engines in this crate require some kind of mutable
"scratch" space to read and write from while performing a search. Since
a meta regex composes these regex engines, a meta regex also requires
mutable scratch space. This scratch space is called a [`Cache`](../hybrid/dfa/index.md).

Most regex engines _also_ usually have a read-only component, typically
a [Thompson `NFA`](crate::nfa::thompson::NFA).

In order to make the `Regex` API convenient, most of the routines hide
the fact that a `Cache` is needed at all. To achieve this, a [memory
pool](crate::util::pool::Pool) is used internally to retrieve `Cache`
values in a thread safe way that also permits reuse. This in turn implies
that every such search call requires some form of synchronization. Usually
this synchronization is fast enough to not notice, but in some cases, it
can be a bottleneck. This typically occurs when all of the following are
true:

* The same `Regex` is shared across multiple threads simultaneously,
usually via a [`util::lazy::Lazy`](crate::util::lazy::Lazy) or something
similar from the `once_cell` or `lazy_static` crates.
* The primary unit of work in each thread is a regex search.
* Searches are run on very short haystacks.

This particular case can lead to high contention on the pool used by a
`Regex` internally, which can in turn increase latency to a noticeable
effect. This cost can be mitigated in one of the following ways:

* Use a distinct copy of a `Regex` in each thread, usually by cloning it.
Cloning a `Regex` _does not_ do a deep copy of its read-only component.
But it does lead to each `Regex` having its own memory pool, which in
turn eliminates the problem of contention. In general, this technique should
not result in any additional memory usage when compared to sharing the same
`Regex` across multiple threads simultaneously.
* Use lower level APIs, like `Regex::search_with`, which permit passing
a `Cache` explicitly. In this case, it is up to you to determine how best
to provide a `Cache`. For example, you might put a `Cache` in thread-local
storage if your use case allows for it.

Overall, this is an issue that happens rarely in practice, but it can
happen.

# Warning: spin-locks may be used in alloc-only mode

When this crate is built without the `std` feature and the high level APIs
on a `Regex` are used, then a spin-lock will be used to synchronize access
to an internal pool of `Cache` values. This may be undesirable because
a spin-lock is [effectively impossible to implement correctly in user
space][spinlocks-are-bad]. That is, more concretely, the spin-lock could
result in a deadlock.

[spinlocks-are-bad]: https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html

If one wants to avoid the use of spin-locks when the `std` feature is
disabled, then you must use APIs that accept a `Cache` value explicitly.
For example, `Regex::search_with`.

# Example

```
use regex_automata::meta::Regex;

let re = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$")?;
assert!(re.is_match("2010-03-14"));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: anchored search

This example shows how to use `Input::anchored` to run an anchored
search, even when the regex pattern itself isn't anchored. An anchored
search guarantees that if a match is found, then the start offset of the
match corresponds to the offset at which the search was started.

```
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"\bfoo\b")?;
let input = Input::new("xx foo xx").range(3..).anchored(Anchored::Yes);
// The offsets are in terms of the original haystack.
assert_eq!(Some(Match::must(0, 3..6)), re.find(input));

// Notice that no match occurs here, because \b still takes the
// surrounding context into account, even if it means looking back
// before the start of your search.
let hay = "xxfoo xx";
let input = Input::new(hay).range(2..).anchored(Anchored::Yes);
assert_eq!(None, re.find(input));
// Indeed, you cannot achieve the above by simply slicing the
// haystack itself, since the regex engine can't see the
// surrounding context. This is why 'Input' permits setting
// the bounds of a search!
let input = Input::new(&hay[2..]).anchored(Anchored::Yes);
// WRONG!
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: earliest search

This example shows how to use `Input::earliest` to run a search that
might stop before finding the typical leftmost match.

```
use regex_automata::{meta::Regex, Anchored, Input, Match};

let re = Regex::new(r"[a-z]{3}|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 1..2)), re.find(input));

// Note that "earliest" isn't really a match semantic unto itself.
// Instead, it is merely an instruction to whatever regex engine
// gets used internally to quit as soon as it can. For example,
// this regex uses a different search technique, and winds up
// producing a different (but valid) match!
let re = Regex::new(r"abc|b")?;
let input = Input::new("abc").earliest(true);
assert_eq!(Some(Match::must(0, 0..3)), re.find(input));

# Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: change the line terminator

This example shows how to enable multi-line mode by default and change
the line terminator to the NUL byte:

```
use regex_automata::{meta::Regex, util::syntax, Match};

let re = Regex::builder()
    .syntax(syntax::Config::new().multi_line(true))
    .configure(Regex::config().line_terminator(b'\x00'))
    .build(r"^foo$")?;
let hay = "\x00foo\x00";
assert_eq!(Some(Match::must(0, 1..4)), re.find(hay));

# Ok::<(), Box<dyn std::error::Error>>(())
```

#### Implementations

- `fn create_captures(self: &Self) -> Captures`
  Creates a new object for recording capture group offsets. This is used

- `fn create_cache(self: &Self) -> Cache`
  Creates a new cache for use with lower level search APIs like

- `fn pattern_len(self: &Self) -> usize`
  Returns the total number of patterns in this regex.

- `fn captures_len(self: &Self) -> usize`
  Returns the total number of capturing groups.

- `fn static_captures_len(self: &Self) -> Option<usize>`
  Returns the total number of capturing groups that appear in every

- `fn group_info(self: &Self) -> &GroupInfo`
  Return information about the capture groups in this `Regex`.

- `fn get_config(self: &Self) -> &Config`
  Returns the configuration object used to build this `Regex`.

- `fn is_accelerated(self: &Self) -> bool`
  Returns true if this regex has a high chance of being "accelerated."

- `fn memory_usage(self: &Self) -> usize`
  Return the total approximate heap memory, in bytes, used by this `Regex`.

- `fn search_with(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>`
  This is like [`Regex::search`], but requires the caller to

- `fn search_half_with(self: &Self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>`
  This is like [`Regex::search_half`], but requires the caller to

- `fn search_captures_with(self: &Self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures)`
  This is like [`Regex::search_captures`], but requires the caller to

- `fn search_slots_with(self: &Self, cache: &mut Cache, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>`
  This is like [`Regex::search_slots`], but requires the caller to

- `fn which_overlapping_matches_with(self: &Self, cache: &mut Cache, input: &Input<'_>, patset: &mut PatternSet)`
  This is like [`Regex::which_overlapping_matches`], but requires the

- `fn search(self: &Self, input: &Input<'_>) -> Option<Match>`
  Returns the start and end offset of the leftmost match. If no match

- `fn search_half(self: &Self, input: &Input<'_>) -> Option<HalfMatch>`
  Returns the end offset of the leftmost match. If no match exists, then

- `fn search_captures(self: &Self, input: &Input<'_>, caps: &mut Captures)`
  Executes a leftmost forward search and writes the spans of capturing

- `fn search_slots(self: &Self, input: &Input<'_>, slots: &mut [Option<NonMaxUsize>]) -> Option<PatternID>`
  Executes a leftmost forward search and writes the spans of capturing

- `fn which_overlapping_matches(self: &Self, input: &Input<'_>, patset: &mut PatternSet)`
  Writes the set of patterns that match anywhere in the given search

- `fn new(pattern: &str) -> Result<Regex, BuildError>`
  Builds a `Regex` from a single pattern string using the default

- `fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError>`
  Builds a `Regex` from many pattern strings using the default

- `fn config() -> Config`
  Return a default configuration for a `Regex`.

- `fn builder() -> Builder`
  Return a builder for configuring the construction of a `Regex`.

- `fn is_match<'h, I: Into<Input<'h>>>(self: &Self, input: I) -> bool`
  Returns true if and only if this regex matches the given haystack.

- `fn find<'h, I: Into<Input<'h>>>(self: &Self, input: I) -> Option<Match>`
  Executes a leftmost search and returns the first match that is found,

- `fn captures<'h, I: Into<Input<'h>>>(self: &Self, input: I, caps: &mut Captures)`
  Executes a leftmost forward search and writes the spans of capturing

- `fn find_iter<'r, 'h, I: Into<Input<'h>>>(self: &'r Self, input: I) -> FindMatches<'r, 'h>`
  Returns an iterator over all non-overlapping leftmost matches in

- `fn captures_iter<'r, 'h, I: Into<Input<'h>>>(self: &'r Self, input: I) -> CapturesMatches<'r, 'h>`
  Returns an iterator over all non-overlapping `Captures` values. If no

- `fn split<'r, 'h, I: Into<Input<'h>>>(self: &'r Self, input: I) -> Split<'r, 'h>`
  Returns an iterator of spans of the haystack given, delimited by a

- `fn splitn<'r, 'h, I: Into<Input<'h>>>(self: &'r Self, input: I, limit: usize) -> SplitN<'r, 'h>`
  Returns an iterator of at most `limit` spans of the haystack given,

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

- `fn clone(self: &Self) -> Regex`

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

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    // [REDACTED: Private Fields]
}
```

Yields all substrings delimited by a regular expression match.

The spans correspond to the offsets between matches.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::split` method.

#### Implementations

- `fn input<'s>(self: &'s Self) -> &'s Input<'h>`
  Returns the current `Input` associated with this iterator.

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

##### `impl FusedIterator<'r, 'h>`

##### `impl Iterator<'r, 'h>`

- `type Item = Span`

- `fn next(self: &mut Self) -> Option<Span>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    // [REDACTED: Private Fields]
}
```

Yields at most `N` spans delimited by a regular expression match.

The spans correspond to the offsets between matches. The last span will be
whatever remains after splitting.

The lifetime parameters are as follows:

* `'r` represents the lifetime of the `Regex` that produced this iterator.
* `'h` represents the lifetime of the haystack being searched.

This iterator can be created with the `Regex::splitn` method.

#### Implementations

- `fn input<'s>(self: &'s Self) -> &'s Input<'h>`
  Returns the current `Input` associated with this iterator.

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

##### `impl FusedIterator<'r, 'h>`

##### `impl Iterator<'r, 'h>`

- `type Item = Span`

- `fn next(self: &mut Self) -> Option<Span>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'r, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

