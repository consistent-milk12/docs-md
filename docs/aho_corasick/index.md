# Crate `aho_corasick`

A library for finding occurrences of many patterns at once. This library
provides multiple pattern search principally through an implementation of the
[Aho-Corasick algorithm](https://en.wikipedia.org/wiki/Aho%E2%80%93Corasick_algorithm),
which builds a fast finite state machine for executing searches in linear time.

Additionally, this library provides a number of configuration options for
building the automaton that permit controlling the space versus time trade
off. Other features include simple ASCII case insensitive matching, finding
overlapping matches, replacements, searching streams and even searching and
replacing text in streams.

Finally, unlike most other Aho-Corasick implementations, this one
supports enabling [leftmost-first](MatchKind::LeftmostFirst) or
[leftmost-longest](MatchKind::LeftmostLongest) match semantics, using a
(seemingly) novel alternative construction algorithm. For more details on what
match semantics means, see the [`MatchKind`](#matchkind) type.

# Overview

This section gives a brief overview of the primary types in this crate:

* [`AhoCorasick`](#ahocorasick) is the primary type and represents an Aho-Corasick automaton.
This is the type you use to execute searches.
* [`AhoCorasickBuilder`](#ahocorasickbuilder) can be used to build an Aho-Corasick automaton, and
supports configuring a number of options.
* [`Match`](../syn/syn/token/index.md) represents a single match reported by an Aho-Corasick automaton.
Each match has two pieces of information: the pattern that matched and the
start and end byte offsets corresponding to the position in the haystack at
which it matched.

# Example: basic searching

This example shows how to search for occurrences of multiple patterns
simultaneously. Each match includes the pattern that matched along with the
byte offsets of the match.

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "Snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::new(patterns).unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

# Example: case insensitivity

This is like the previous example, but matches `Snapple` case insensitively
using `AhoCorasickBuilder`:

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::builder()
    .ascii_case_insensitive(true)
    .build(patterns)
    .unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

# Example: replacing matches in a stream

This example shows how to execute a search and replace on a stream without
loading the entire stream into memory first.

```
# #[cfg(feature = "std")] {
use aho_corasick::AhoCorasick;

# fn example() -> Result<(), std::io::Error> {
let patterns = &["fox", "brown", "quick"];
let replace_with = &["sloth", "grey", "slow"];

// In a real example, these might be `std::fs::File`s instead. All you need to
// do is supply a pair of `std::io::Read` and `std::io::Write` implementations.
let rdr = "The quick brown fox.";
let mut wtr = vec![];

let ac = AhoCorasick::new(patterns).unwrap();
ac.try_stream_replace_all(rdr.as_bytes(), &mut wtr, replace_with)?;
assert_eq!(b"The slow grey sloth.".to_vec(), wtr);
# Ok(()) }; example().unwrap()
# }
```

# Example: finding the leftmost first match

In the textbook description of Aho-Corasick, its formulation is typically
structured such that it reports all possible matches, even when they overlap
with another. In many cases, overlapping matches may not be desired, such as
the case of finding all successive non-overlapping matches like you might with
a standard regular expression.

Unfortunately the "obvious" way to modify the Aho-Corasick algorithm to do
this doesn't always work in the expected way, since it will report matches as
soon as they are seen. For example, consider matching the regex `Samwise|Sam`
against the text `Samwise`. Most regex engines (that are Perl-like, or
non-POSIX) will report `Samwise` as a match, but the standard Aho-Corasick
algorithm modified for reporting non-overlapping matches will report `Sam`.

A novel contribution of this library is the ability to change the match
semantics of Aho-Corasick (without additional search time overhead) such that
`Samwise` is reported instead. For example, here's the standard approach:

```
use aho_corasick::AhoCorasick;

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::new(patterns).unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Sam", &haystack[mat.start()..mat.end()]);
```

And now here's the leftmost-first version, which matches how a Perl-like
regex will work:

```
use aho_corasick::{AhoCorasick, MatchKind};

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::builder()
    .match_kind(MatchKind::LeftmostFirst)
    .build(patterns)
    .unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Samwise", &haystack[mat.start()..mat.end()]);
```

In addition to leftmost-first semantics, this library also supports
leftmost-longest semantics, which match the POSIX behavior of a regular
expression alternation. See [`MatchKind`](#matchkind) for more details.

# Prefilters

While an Aho-Corasick automaton can perform admirably when compared to more
naive solutions, it is generally slower than more specialized algorithms that
are accelerated using vector instructions such as SIMD.

For that reason, this library will internally use a "prefilter" to attempt
to accelerate searches when possible. Currently, this library has several
different algorithms it might use depending on the patterns provided. Once the
number of patterns gets too big, prefilters are no longer used.

While a prefilter is generally good to have on by default since it works
well in the common case, it can lead to less predictable or even sub-optimal
performance in some cases. For that reason, prefilters can be explicitly
disabled via [`AhoCorasickBuilder::prefilter`](#prefilter).

# Lower level APIs

This crate also provides several sub-modules that collectively expose many of
the implementation details of the main [`AhoCorasick`](#ahocorasick) type. Most users of this
library can completely ignore the submodules and their contents, but if you
needed finer grained control, some parts of them may be useful to you. Here is
a brief overview of each and why you might want to use them:

* The [`packed`](aho_corasick/packed/index.md) sub-module contains a lower level API for using fast
vectorized routines for finding a small number of patterns in a haystack.
You might want to use this API when you want to completely side-step using
Aho-Corasick automata. Otherwise, the fast vectorized routines are used
automatically as prefilters for `AhoCorasick` searches whenever possible.
* The [`automaton`](aho_corasick/automaton/index.md) sub-module provides a lower level finite state
machine interface that the various Aho-Corasick implementations in
this crate implement. This sub-module's main contribution is the
[`Automaton`](automaton::Automaton) trait, which permits manually walking the
state transitions of an Aho-Corasick automaton.
* The [`dfa`](aho_corasick/dfa/index.md) and [`nfa`](aho_corasick/nfa/index.md) sub-modules provide DFA and NFA implementations of
the aforementioned `Automaton` trait. The main reason one might want to use
these sub-modules is to get access to a type that implements the `Automaton`
trait. (The top-level `AhoCorasick` type does not implement the `Automaton`
trait.)

As mentioned above, if you aren't sure whether you need these sub-modules,
you should be able to safely ignore them and just focus on the [`AhoCorasick`](#ahocorasick)
type.

# Crate features

This crate exposes a few features for controlling dependency usage and whether
this crate can be used without the standard library.

* **std** -
  Enables support for the standard library. This feature is enabled by
  default. When disabled, only `core` and `alloc` are used. At an API
  level, enabling `std` enables `std::error::Error` trait impls for the
  various error types, and higher level stream search routines such as
  [`AhoCorasick::try_stream_find_iter`](#try-stream-find-iter). But the `std` feature is also required
  to enable vectorized prefilters. Prefilters can greatly accelerate searches,
  but generally only apply when the number of patterns is small (less than
  ~100).
* **perf-literal** -
  Enables support for literal prefilters that use vectorized routines from
  external crates. This feature is enabled by default. If you're only using
  Aho-Corasick for large numbers of patterns or otherwise can abide lower
  throughput when searching with a small number of patterns, then it is
  reasonable to disable this feature.
* **logging** -
  Enables a dependency on the `log` crate and emits messages to aide in
  diagnostics. This feature is disabled by default.

## Modules

- [`automaton`](automaton/index.md) - Provides [`Automaton`] trait for abstracting over Aho-Corasick automata.
- [`dfa`](dfa/index.md) - Provides direct access to a DFA implementation of Aho-Corasick.
- [`nfa`](nfa/index.md) - Provides direct access to NFA implementations of Aho-Corasick.
- [`packed`](packed/index.md) - Provides packed multiple substring search, principally for a small number of

## Structs

### `StreamFindIter<'a, R>`

```rust
struct StreamFindIter<'a, R>();
```

An iterator that reports Aho-Corasick matches in a stream.

This iterator yields elements of type `Result<Match, std::io::Error>`,
where an error is reported if there was a problem reading from the
underlying stream. The iterator terminates only when the underlying stream
reaches `EOF`.

This iterator is constructed via the [`AhoCorasick::stream_find_iter`](#stream-find-iter) and
[`AhoCorasick::try_stream_find_iter`](#try-stream-find-iter) methods.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the corresponding
[`AhoCorasick`](#ahocorasick) searcher.

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

##### `impl Iterator<'a, R: std::io::Read>`

- `type Item = Result<Match, Error>`

- `fn next(self: &mut Self) -> Option<Result<Match, std::io::Error>>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, R: $crate::fmt::Debug>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `AhoCorasick`

```rust
struct AhoCorasick {
}
```

An automaton for searching multiple strings in linear time.

The `AhoCorasick` type supports a few basic ways of constructing an
automaton, with the default being [`AhoCorasick::new`](#new). However, there
are a fair number of configurable options that can be set by using
[`AhoCorasickBuilder`](#ahocorasickbuilder) instead. Such options include, but are not limited
to, how matches are determined, simple case insensitivity, whether to use a
DFA or not and various knobs for controlling the space-vs-time trade offs
taken when building the automaton.

# Resource usage

Aho-Corasick automatons are always constructed in `O(p)` time, where
`p` is the combined length of all patterns being searched. With that
said, building an automaton can be fairly costly because of high constant
factors, particularly when enabling the [DFA](AhoCorasickKind::DFA) option
with [`AhoCorasickBuilder::kind`](#kind). For this reason, it's generally a good
idea to build an automaton once and reuse it as much as possible.

Aho-Corasick automatons can also use a fair bit of memory. To get
a concrete idea of how much memory is being used, try using the
[`AhoCorasick::memory_usage`](#memory-usage) method.

To give a quick idea of the differences between Aho-Corasick
implementations and their resource usage, here's a sample of construction
times and heap memory used after building an automaton from 100,000
randomly selected titles from Wikipedia:

* 99MB for a [`noncontiguous::NFA`](#nfa) in 240ms.
* 21MB for a [`contiguous::NFA`](#nfa) in 275ms.
* 1.6GB for a [`dfa::DFA`](#dfa) in 1.88s.

(Note that the memory usage above reflects the size of each automaton and
not peak memory usage. For example, building a contiguous NFA requires
first building a noncontiguous NFA. Once the contiguous NFA is built, the
noncontiguous NFA is freed.)

This experiment very strongly argues that a contiguous NFA is often the
best balance in terms of resource usage. It takes a little longer to build,
but its memory usage is quite small. Its search speed (not listed) is
also often faster than a noncontiguous NFA, but a little slower than a
DFA. Indeed, when no specific [`AhoCorasickKind`](#ahocorasickkind) is used (which is the
default), a contiguous NFA is used in most cases.

The only "catch" to using a contiguous NFA is that, because of its variety
of compression tricks, it may not be able to support automatons as large as
what the noncontiguous NFA supports. In which case, building a contiguous
NFA will fail and (by default) `AhoCorasick` will automatically fall
back to a noncontiguous NFA. (This typically only happens when building
automatons from millions of patterns.) Otherwise, the small additional time
for building a contiguous NFA is almost certainly worth it.

# Cloning

The `AhoCorasick` type uses thread safe reference counting internally. It
is guaranteed that it is cheap to clone.

# Search configuration

Most of the search routines accept anything that can be cheaply converted
to an [`Input`](#input). This includes `&[u8](#u8)`, `&str` and `Input` itself.

# Construction failure

It is generally possible for building an Aho-Corasick automaton to fail.
Construction can fail in generally one way: when the inputs provided are
too big. Whether that's a pattern that is too long, too many patterns
or some combination of both. A first approximation for the scale at which
construction can fail is somewhere around "millions of patterns."

For that reason, if you're building an Aho-Corasick automaton from
untrusted input (or input that doesn't have any reasonable bounds on its
size), then it is strongly recommended to handle the possibility of an
error.

If you're constructing an Aho-Corasick automaton from static or trusted
data, then it is likely acceptable to panic (by calling `unwrap()` or
`expect()`) if construction fails.

# Fallibility

The `AhoCorasick` type provides a number of methods for searching, as one
might expect. Depending on how the Aho-Corasick automaton was built and
depending on the search configuration, it is possible for a search to
return an error. Since an error is _never_ dependent on the actual contents
of the haystack, this type provides both infallible and fallible methods
for searching. The infallible methods panic if an error occurs, and can be
used for convenience and when you know the search will never return an
error.

For example, the [`AhoCorasick::find_iter`](#find-iter) method is the infallible
version of the [`AhoCorasick::try_find_iter`](#try-find-iter) method.

Examples of errors that can occur:

* Running a search that requires [`MatchKind::Standard`](#standard) semantics (such
as a stream or overlapping search) with an automaton that was built with
[`MatchKind::LeftmostFirst`](#leftmostfirst) or [`MatchKind::LeftmostLongest`](#leftmostlongest) semantics.
* Running an anchored search with an automaton that only supports
unanchored searches. (By default, `AhoCorasick` only supports unanchored
searches. But this can be toggled with [`AhoCorasickBuilder::start_kind`](#start-kind).)
* Running an unanchored search with an automaton that only supports
anchored searches.

The common thread between the different types of errors is that they are
all rooted in the automaton construction and search configurations. If
those configurations are a static property of your program, then it is
reasonable to call infallible routines since you know an error will never
occur. And if one _does_ occur, then it's a bug in your program.

To re-iterate, if the patterns, build or search configuration come from
user or untrusted data, then you should handle errors at build or search
time. If only the haystack comes from user or untrusted data, then there
should be no need to handle errors anywhere and it is generally encouraged
to `unwrap()` (or `expect()`) both build and search time calls.

# Examples

This example shows how to search for occurrences of multiple patterns
simultaneously in a case insensitive fashion. Each match includes the
pattern that matched along with the byte offsets of the match.

```
use aho_corasick::{AhoCorasick, PatternID};

let patterns = &["apple", "maple", "snapple"];
let haystack = "Nobody likes maple in their apple flavored Snapple.";

let ac = AhoCorasick::builder()
    .ascii_case_insensitive(true)
    .build(patterns)
    .unwrap();
let mut matches = vec![];
for mat in ac.find_iter(haystack) {
    matches.push((mat.pattern(), mat.start(), mat.end()));
}
assert_eq!(matches, vec![
    (PatternID::must(1), 13, 18),
    (PatternID::must(0), 28, 33),
    (PatternID::must(2), 43, 50),
]);
```

This example shows how to replace matches with some other string:

```
use aho_corasick::AhoCorasick;

let patterns = &["fox", "brown", "quick"];
let haystack = "The quick brown fox.";
let replace_with = &["sloth", "grey", "slow"];

let ac = AhoCorasick::new(patterns).unwrap();
let result = ac.replace_all(haystack, replace_with);
assert_eq!(result, "The slow grey sloth.");
```

#### Implementations

- `fn try_find<'h, I: Into<Input<'h>>>(self: &Self, input: I) -> Result<Option<Match>, MatchError>`
  Returns the location of the first match according to the match

- `fn try_find_overlapping<'h, I: Into<Input<'h>>>(self: &Self, input: I, state: &mut OverlappingState) -> Result<(), MatchError>`
  Returns the location of the first overlapping match in the given

- `fn try_find_iter<'a, 'h, I: Into<Input<'h>>>(self: &'a Self, input: I) -> Result<FindIter<'a, 'h>, MatchError>`
  Returns an iterator of non-overlapping matches, using the match

- `fn try_find_overlapping_iter<'a, 'h, I: Into<Input<'h>>>(self: &'a Self, input: I) -> Result<FindOverlappingIter<'a, 'h>, MatchError>`
  Returns an iterator of overlapping matches.

- `fn try_replace_all<B>(self: &Self, haystack: &str, replace_with: &[B]) -> Result<String, MatchError>`
  Replace all matches with a corresponding value in the `replace_with`

- `fn try_replace_all_bytes<B>(self: &Self, haystack: &[u8], replace_with: &[B]) -> Result<Vec<u8>, MatchError>`
  Replace all matches using raw bytes with a corresponding value in the

- `fn try_replace_all_with<F>(self: &Self, haystack: &str, dst: &mut String, replace_with: F) -> Result<(), MatchError>`
  Replace all matches using a closure called on each match.

- `fn try_replace_all_with_bytes<F>(self: &Self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F) -> Result<(), MatchError>`
  Replace all matches using raw bytes with a closure called on each

- `fn try_stream_find_iter<'a, R: std::io::Read>(self: &'a Self, rdr: R) -> Result<StreamFindIter<'a, R>, MatchError>`
  Returns an iterator of non-overlapping matches in the given

- `fn try_stream_replace_all<R, W, B>(self: &Self, rdr: R, wtr: W, replace_with: &[B]) -> Result<(), std::io::Error>`
  Search for and replace all matches of this automaton in

- `fn try_stream_replace_all_with<R, W, F>(self: &Self, rdr: R, wtr: W, replace_with: F) -> Result<(), std::io::Error>`
  Search the given reader and replace all matches of this automaton

- `fn is_match<'h, I: Into<Input<'h>>>(self: &Self, input: I) -> bool`
  Returns true if and only if this automaton matches the haystack at any

- `fn find<'h, I: Into<Input<'h>>>(self: &Self, input: I) -> Option<Match>`
  Returns the location of the first match according to the match

- `fn find_overlapping<'h, I: Into<Input<'h>>>(self: &Self, input: I, state: &mut OverlappingState)`
  Returns the location of the first overlapping match in the given

- `fn find_iter<'a, 'h, I: Into<Input<'h>>>(self: &'a Self, input: I) -> FindIter<'a, 'h>`
  Returns an iterator of non-overlapping matches, using the match

- `fn find_overlapping_iter<'a, 'h, I: Into<Input<'h>>>(self: &'a Self, input: I) -> FindOverlappingIter<'a, 'h>`
  Returns an iterator of overlapping matches. Stated differently, this

- `fn replace_all<B>(self: &Self, haystack: &str, replace_with: &[B]) -> String`
  Replace all matches with a corresponding value in the `replace_with`

- `fn replace_all_bytes<B>(self: &Self, haystack: &[u8], replace_with: &[B]) -> Vec<u8>`
  Replace all matches using raw bytes with a corresponding value in the

- `fn replace_all_with<F>(self: &Self, haystack: &str, dst: &mut String, replace_with: F)`
  Replace all matches using a closure called on each match.

- `fn replace_all_with_bytes<F>(self: &Self, haystack: &[u8], dst: &mut Vec<u8>, replace_with: F)`
  Replace all matches using raw bytes with a closure called on each

- `fn stream_find_iter<'a, R: std::io::Read>(self: &'a Self, rdr: R) -> StreamFindIter<'a, R>`
  Returns an iterator of non-overlapping matches in the given

- `fn new<I, P>(patterns: I) -> Result<AhoCorasick, BuildError>`
  Create a new Aho-Corasick automaton using the default configuration.

- `fn builder() -> AhoCorasickBuilder`
  A convenience method for returning a new Aho-Corasick builder.

- `fn kind(self: &Self) -> AhoCorasickKind`
  Returns the kind of the Aho-Corasick automaton used by this searcher.

- `fn start_kind(self: &Self) -> StartKind`
  Returns the type of starting search configuration supported by this

- `fn match_kind(self: &Self) -> MatchKind`
  Returns the match kind used by this automaton.

- `fn min_pattern_len(self: &Self) -> usize`
  Returns the length of the shortest pattern matched by this automaton.

- `fn max_pattern_len(self: &Self) -> usize`
  Returns the length of the longest pattern matched by this automaton.

- `fn patterns_len(self: &Self) -> usize`
  Return the total number of patterns matched by this automaton.

- `fn memory_usage(self: &Self) -> usize`
  Returns the approximate total amount of heap used by this automaton, in

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

- `fn clone(self: &Self) -> AhoCorasick`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `AhoCorasickBuilder`

```rust
struct AhoCorasickBuilder {
}
```

A builder for configuring an Aho-Corasick automaton.

# Quick advice

* Use [`AhoCorasickBuilder::match_kind`](#match-kind) to configure your searcher
with [`MatchKind::LeftmostFirst`](#leftmostfirst) if you want to match how backtracking
regex engines execute searches for `pat1|pat2|..|patN`. Use
[`MatchKind::LeftmostLongest`](#leftmostlongest) if you want to match how POSIX regex engines
do it.
* If you need an anchored search, use [`AhoCorasickBuilder::start_kind`](#start-kind) to
set the [`StartKind::Anchored`](#anchored) mode since [`StartKind::Unanchored`](#unanchored) is the
default. Or just use [`StartKind::Both`](#both) to support both types of searches.
* You might want to use [`AhoCorasickBuilder::kind`](#kind) to set your searcher
to always use a [`AhoCorasickKind::DFA`](#dfa) if search speed is critical and
memory usage isn't a concern. Otherwise, not setting a kind will probably
make the right choice for you. Beware that if you use [`StartKind::Both`](#both)
to build a searcher that supports both unanchored and anchored searches
_and_ you set [`AhoCorasickKind::DFA`](#dfa), then the DFA will essentially be
duplicated to support both simultaneously. This results in very high memory
usage.
* For all other options, their defaults are almost certainly what you want.

#### Implementations

- `fn new() -> AhoCorasickBuilder`
  Create a new builder for configuring an Aho-Corasick automaton.

- `fn build<I, P>(self: &Self, patterns: I) -> Result<AhoCorasick, BuildError>`
  Build an Aho-Corasick automaton using the configuration set on this

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut AhoCorasickBuilder`
  Set the desired match semantics.

- `fn start_kind(self: &mut Self, kind: StartKind) -> &mut AhoCorasickBuilder`
  Sets the starting state configuration for the automaton.

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut AhoCorasickBuilder`
  Enable ASCII-aware case insensitive matching.

- `fn kind(self: &mut Self, kind: Option<AhoCorasickKind>) -> &mut AhoCorasickBuilder`
  Choose the type of underlying automaton to use.

- `fn prefilter(self: &mut Self, yes: bool) -> &mut AhoCorasickBuilder`
  Enable heuristic prefilter optimizations.

- `fn dense_depth(self: &mut Self, depth: usize) -> &mut AhoCorasickBuilder`
  Set the limit on how many states use a dense representation for their

- `fn byte_classes(self: &mut Self, yes: bool) -> &mut AhoCorasickBuilder`
  A debug settting for whether to attempt to shrink the size of the

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

- `fn clone(self: &Self) -> AhoCorasickBuilder`

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

- `fn default() -> AhoCorasickBuilder`

### `FindIter<'a, 'h>`

```rust
struct FindIter<'a, 'h>();
```

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`](#matchkind) used by this
automaton.

This iterator is constructed via the [`AhoCorasick::find_iter`](#find-iter) and
[`AhoCorasick::try_find_iter`](#try-find-iter) methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

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

##### `impl Iterator<'a, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `FindOverlappingIter<'a, 'h>`

```rust
struct FindOverlappingIter<'a, 'h>();
```

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the [`AhoCorasick::find_overlapping_iter`](#find-overlapping-iter)
and [`AhoCorasick::try_find_overlapping_iter`](#try-find-overlapping-iter) methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

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

##### `impl Iterator<'a, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

### `BuildError`

```rust
struct BuildError {
}
```

An error that occurred during the construction of an Aho-Corasick
automaton.

Build errors occur when some kind of limit has been exceeded, either in the
number of states, the number of patterns of the length of a pattern. These
limits aren't part of the public API, but they should generally be large
enough to handle most use cases.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

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

### `MatchError`

```rust
struct MatchError();
```

An error that occurred during an Aho-Corasick search.

An error that occurs during a search is limited to some kind of
misconfiguration that resulted in an illegal call. Stated differently,
whether an error occurs is not dependent on the specific bytes in the
haystack.

Examples of misconfiguration:

* Executing a stream or overlapping search on a searcher that was built was
something other than [`MatchKind::Standard`](crate::MatchKind::Standard)
semantics.
* Requested an anchored or an unanchored search on a searcher that doesn't
support unanchored or anchored searches, respectively.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- `fn new(kind: MatchErrorKind) -> MatchError`
  Create a new error value with the given kind.

- `fn kind(self: &Self) -> &MatchErrorKind`
  Returns a reference to the underlying error kind.

- `fn invalid_input_anchored() -> MatchError`
  Create a new "invalid anchored search" error. This occurs when the

- `fn invalid_input_unanchored() -> MatchError`
  Create a new "invalid unanchored search" error. This occurs when the

- `fn unsupported_stream(got: MatchKind) -> MatchError`
  Create a new "unsupported stream search" error. This occurs when the

- `fn unsupported_overlapping(got: MatchKind) -> MatchError`
  Create a new "unsupported overlapping search" error. This occurs when

- `fn unsupported_empty() -> MatchError`
  Create a new "unsupported empty pattern" error. This occurs when the

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

- `fn clone(self: &Self) -> MatchError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MatchError) -> bool`

##### `impl StructuralPartialEq`

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

### `PatternID`

```rust
struct PatternID();
```

The identifier of a pattern in an Aho-Corasick automaton.

It is represented by a `u32` even on 64-bit systems in order to conserve
space. Namely, on all targets, this type guarantees that its value will
fit in a `u32`, `i32`, `usize` and an `isize`. This means that on 16-bit
targets, for example, this type's maximum value will never overflow an
`isize`, which means it will never overflow a `i16` even though its
internal representation is still a `u32`.

# Safety

While a `PatternID` is meant to guarantee that its value fits into `usize`
without using as much space as a `usize` on all targets, callers must
not rely on this property for safety. Callers may choose to rely on this
property for correctness however. For example, creating a `StateID` with an
invalid value can be done in entirely safe code. This may in turn result in
panics or silent logical errors.

#### Implementations

- `const MAX: PatternID`

- `const LIMIT: usize`

- `const ZERO: PatternID`

- `const SIZE: usize`

- `fn new(value: usize) -> Result<PatternID, PatternIDError>`
  Create a new value that is represented by a "small index."

- `const fn new_unchecked(value: usize) -> PatternID`
  Create a new value without checking whether the given argument

- `const fn from_u32_unchecked(index: u32) -> PatternID`
  Create a new value from a `u32` without checking whether the

- `fn must(value: usize) -> PatternID`
  Like `new`, but panics if the given value is not valid.

- `const fn as_usize(self: &Self) -> usize`
  Return the internal value as a `usize`. This is guaranteed to

- `const fn as_u64(self: &Self) -> u64`
  Return the internal value as a `u64`. This is guaranteed to

- `const fn as_u32(self: &Self) -> u32`
  Return the internal value as a `u32`. This is guaranteed to

- `const fn as_i32(self: &Self) -> i32`
  Return the internal value as a `i32`. This is guaranteed to

- `fn one_more(self: &Self) -> usize`
  Returns one more than this value as a usize.

- `fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>`
  Decode this value from the bytes given using the native endian

- `fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID`
  Decode this value from the bytes given using the native endian

- `fn to_ne_bytes(self: &Self) -> [u8; 4]`
  Return the underlying integer as raw bytes in native endian

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From`

- `fn from(value: u8) -> PatternID`

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

- `fn clone(self: &Self) -> PatternID`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl Ord`

- `fn cmp(self: &Self, other: &PatternID) -> $crate::cmp::Ordering`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PatternID) -> bool`

##### `impl PartialOrd`

- `fn partial_cmp(self: &Self, other: &PatternID) -> $crate::option::Option<$crate::cmp::Ordering>`

##### `impl StructuralPartialEq`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: u16) -> Result<PatternID, PatternIDError>`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: u32) -> Result<PatternID, PatternIDError>`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: u64) -> Result<PatternID, PatternIDError>`

##### `impl TryFrom`

- `type Error = PatternIDError`

- `fn try_from(value: usize) -> Result<PatternID, PatternIDError>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default`

- `fn default() -> PatternID`

### `PatternIDError`

```rust
struct PatternIDError();
```

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- `fn attempted(self: &Self) -> u64`
  Returns the value that could not be converted to an ID.

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

- `fn clone(self: &Self) -> PatternIDError`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Display`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq`

##### `impl Error`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &PatternIDError) -> bool`

##### `impl StructuralPartialEq`

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

### `Input<'h>`

```rust
struct Input<'h> {
}
```

The configuration and the haystack to use for an Aho-Corasick search.

When executing a search, there are a few parameters one might want to
configure:

* The haystack to search, provided to the [`Input::new`](#new) constructor. This
is the only required parameter.
* The span _within_ the haystack to limit a search to. (The default
is the entire haystack.) This is configured via [`Input::span`](#span) or
[`Input::range`](#range).
* Whether to run an unanchored (matches can occur anywhere after the
start of the search) or anchored (matches can only occur beginning at
the start of the search) search. Unanchored search is the default. This is
configured via [`Input::anchored`](#anchored).
* Whether to quit the search as soon as a match has been found, regardless
of the [`MatchKind`](#matchkind) that the searcher was built with. This is configured
via [`Input::earliest`](#earliest).

For most cases, the defaults for all optional parameters are appropriate.
The utility of this type is that it keeps the default or common case simple
while permitting tweaking parameters in more niche use cases while reusing
the same search APIs.

# Valid bounds and search termination

An `Input` permits setting the bounds of a search via either
[`Input::span`](#span) or [`Input::range`](#range). The bounds set must be valid, or
else a panic will occur. Bounds are valid if and only if:

* The bounds represent a valid range into the input's haystack.
* **or** the end bound is a valid ending bound for the haystack *and*
the start bound is exactly one greater than the end bound.

In the latter case, [`Input::is_done`](#is-done) will return true and indicates any
search receiving such an input should immediately return with no match.

Other than representing "search is complete," the `Input::span` and
`Input::range` APIs are never necessary. Instead, callers can slice the
haystack instead, e.g., with `&haystack[start..end]`. With that said, they
can be more convenient than slicing because the match positions reported
when using `Input::span` or `Input::range` are in terms of the original
haystack. If you instead use `&haystack[start..end]`, then you'll need to
add `start` to any match position returned in order for it to be a correct
index into `haystack`.

# Example: `&str` and `&[u8](#u8)` automatically convert to an `Input`

There is a `From<&T> for Input` implementation for all `T: AsRef<[u8](#u8)>`.
Additionally, the [`AhoCorasick`](crate::AhoCorasick) search APIs accept
a `Into<Input>`. These two things combined together mean you can provide
things like `&str` and `&[u8](#u8)` to search APIs when the defaults are
suitable, but also an `Input` when they're not. For example:

```
use aho_corasick::{AhoCorasick, Anchored, Input, Match, StartKind};

// Build a searcher that supports both unanchored and anchored modes.
let ac = AhoCorasick::builder()
    .start_kind(StartKind::Both)
    .build(&["abcd", "b"])
    .unwrap();
let haystack = "abcd";

// A search using default parameters is unanchored. With standard
// semantics, this finds `b` first.
assert_eq!(
    Some(Match::must(1, 1..2)),
    ac.find(haystack),
);
// Using the same 'find' routine, we can provide an 'Input' explicitly
// that is configured to do an anchored search. Since 'b' doesn't start
// at the beginning of the search, it is not reported as a match.
assert_eq!(
    Some(Match::must(0, 0..4)),
    ac.find(Input::new(haystack).anchored(Anchored::Yes)),
);
```

#### Implementations

- `fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h>`
  Create a new search configuration for the given haystack.

- `fn span<S: Into<Span>>(self: Self, span: S) -> Input<'h>`
  Set the span for this search.

- `fn range<R: RangeBounds<usize>>(self: Self, range: R) -> Input<'h>`
  Like `Input::span`, but accepts any range instead.

- `fn anchored(self: Self, mode: Anchored) -> Input<'h>`
  Sets the anchor mode of a search.

- `fn earliest(self: Self, yes: bool) -> Input<'h>`
  Whether to execute an "earliest" search or not.

- `fn set_span<S: Into<Span>>(self: &mut Self, span: S)`
  Set the span for this search configuration.

- `fn set_range<R: RangeBounds<usize>>(self: &mut Self, range: R)`
  Set the span for this search configuration given any range.

- `fn set_start(self: &mut Self, start: usize)`
  Set the starting offset for the span for this search configuration.

- `fn set_end(self: &mut Self, end: usize)`
  Set the ending offset for the span for this search configuration.

- `fn set_anchored(self: &mut Self, mode: Anchored)`
  Set the anchor mode of a search.

- `fn set_earliest(self: &mut Self, yes: bool)`
  Set whether the search should execute in "earliest" mode or not.

- `fn haystack(self: &Self) -> &[u8]`
  Return a borrow of the underlying haystack as a slice of bytes.

- `fn start(self: &Self) -> usize`
  Return the start position of this search.

- `fn end(self: &Self) -> usize`
  Return the end position of this search.

- `fn get_span(self: &Self) -> Span`
  Return the span for this search configuration.

- `fn get_range(self: &Self) -> Range<usize>`
  Return the span as a range for this search configuration.

- `fn get_anchored(self: &Self) -> Anchored`
  Return the anchored mode for this search configuration.

- `fn get_earliest(self: &Self) -> bool`
  Return whether this search should execute in "earliest" mode.

- `fn is_done(self: &Self) -> bool`
  Return true if this input has been exhausted, which in turn means all

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl From<'h, H: ?Sized + AsRef<[u8]>>`

- `fn from(haystack: &'h H) -> Input<'h>`

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone<'h>`

- `fn clone(self: &Self) -> Input<'h>`

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

##### `impl Debug<'h>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Match`

```rust
struct Match {
}
```

A representation of a match reported by an Aho-Corasick searcher.

A match has two essential pieces of information: the [`PatternID`](../regex_automata/regex_automata/util/primitives/index.md) that
matches, and the [`Span`](../proc_macro2/proc_macro2/index.md) of the match in a haystack.

The pattern is identified by an ID, which corresponds to its position
(starting from `0`) relative to other patterns used to construct the
corresponding searcher. If only a single pattern is provided, then all
matches are guaranteed to have a pattern ID of `0`.

Every match reported by a searcher guarantees that its span has its start
offset as less than or equal to its end offset.

#### Implementations

- `fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match`
  Create a new match from a pattern ID and a span.

- `fn must<S: Into<Span>>(pattern: usize, span: S) -> Match`
  Create a new match from a pattern ID and a byte offset span.

- `fn pattern(self: &Self) -> PatternID`
  Returns the ID of the pattern that matched.

- `fn start(self: &Self) -> usize`
  The starting position of the match.

- `fn end(self: &Self) -> usize`
  The ending position of the match.

- `fn range(self: &Self) -> core::ops::Range<usize>`
  Returns the match span as a range.

- `fn span(self: &Self) -> Span`
  Returns the span for this match.

- `fn is_empty(self: &Self) -> bool`
  Returns true when the span in this match is empty.

- `fn len(self: &Self) -> usize`
  Returns the length of this match.

- `fn offset(self: &Self, offset: usize) -> Match`
  Returns a new match with `offset` added to its span's `start` and `end`

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

- `fn clone(self: &Self) -> Match`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Match) -> bool`

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

### `Span`

```rust
struct Span {
    pub start: usize,
    pub end: usize,
}
```

A representation of a range in a haystack.

A span corresponds to the starting and ending _byte offsets_ of a
contiguous region of bytes. The starting offset is inclusive while the
ending offset is exclusive. That is, a span is a half-open interval.

A span is used to report the offsets of a match, but it is also used to
convey which region of a haystack should be searched via routines like
[`Input::span`](#span).

This is basically equivalent to a `std::ops::Range<usize>`, except this
type implements `Copy` which makes it more ergonomic to use in the context
of this crate. Indeed, `Span` exists only because `Range<usize>` does
not implement `Copy`. Like a range, this implements `Index` for `[u8](#u8)`
and `str`, and `IndexMut` for `[u8](#u8)`. For convenience, this also impls
`From<Range>`, which means things like `Span::from(5..10)` work.

There are no constraints on the values of a span. It is, for example, legal
to create a span where `start > end`.

#### Fields

- **`start`**: `usize`

  The start offset of the span, inclusive.

- **`end`**: `usize`

  The end offset of the span, exclusive.

#### Implementations

- `fn range(self: &Self) -> Range<usize>`
  Returns this span as a range.

- `fn is_empty(self: &Self) -> bool`
  Returns true when this span is empty. That is, when `start >= end`.

- `fn len(self: &Self) -> usize`
  Returns the length of this span.

- `fn contains(self: &Self, offset: usize) -> bool`
  Returns true when the given offset is contained within this span.

- `fn offset(self: &Self, offset: usize) -> Span`
  Returns a new span with `offset` added to this span's `start` and `end`

#### Trait Implementations

##### `impl From`

- `fn from(range: Range<usize>) -> Span`

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

- `fn clone(self: &Self) -> Span`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl Hash`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Span) -> bool`

##### `impl PartialEq`

- `fn eq(self: &Self, range: &Range<usize>) -> bool`

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

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

## Enums

### `AhoCorasickKind`

```rust
enum AhoCorasickKind {
    NoncontiguousNFA,
    ContiguousNFA,
    DFA,
}
```

The type of Aho-Corasick implementation to use in an [`AhoCorasick`](#ahocorasick)
searcher.

This is principally used as an input to the
[`AhoCorasickBuilder::start_kind`](#start-kind) method. Its documentation goes into more
detail about each choice.

#### Variants

- **`NoncontiguousNFA`**

  Use a noncontiguous NFA.

- **`ContiguousNFA`**

  Use a contiguous NFA.

- **`DFA`**

  Use a DFA. Warning: DFAs typically use a large amount of memory.

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

- `fn clone(self: &Self) -> AhoCorasickKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &AhoCorasickKind) -> bool`

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

### `MatchErrorKind`

```rust
enum MatchErrorKind {
    InvalidInputAnchored,
    InvalidInputUnanchored,
    UnsupportedStream {
        got: crate::util::search::MatchKind,
    },
    UnsupportedOverlapping {
        got: crate::util::search::MatchKind,
    },
    UnsupportedEmpty,
}
```

The underlying kind of a [`MatchError`](#matcherror).

This is a **non-exhaustive** enum. That means new variants may be added in
a semver-compatible release.

#### Variants

- **`InvalidInputAnchored`**

  An error indicating that an anchored search was requested, but from a
  searcher that was built without anchored support.

- **`InvalidInputUnanchored`**

  An error indicating that an unanchored search was requested, but from a
  searcher that was built without unanchored support.

- **`UnsupportedStream`**

  An error indicating that a stream search was attempted on an
  Aho-Corasick automaton with an unsupported `MatchKind`.

- **`UnsupportedOverlapping`**

  An error indicating that an overlapping search was attempted on an
  Aho-Corasick automaton with an unsupported `MatchKind`.

- **`UnsupportedEmpty`**

  An error indicating that the operation requested doesn't support
  automatons that contain an empty pattern string.

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

- `fn clone(self: &Self) -> MatchErrorKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &MatchErrorKind) -> bool`

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

### `Anchored`

```rust
enum Anchored {
    No,
    Yes,
}
```

The type of anchored search to perform.

If an Aho-Corasick searcher does not support the anchored mode selected,
then the search will return an error or panic, depending on whether a
fallible or an infallible routine was called.

#### Variants

- **`No`**

  Run an unanchored search. This means a match may occur anywhere at or
  after the start position of the search up until the end position of the
  search.

- **`Yes`**

  Run an anchored search. This means that a match must begin at the start
  position of the search and end before the end position of the search.

#### Implementations

- `fn is_anchored(self: &Self) -> bool`
  Returns true if and only if this anchor mode corresponds to an anchored

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

- `fn clone(self: &Self) -> Anchored`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Anchored) -> bool`

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

### `MatchKind`

```rust
enum MatchKind {
    Standard,
    LeftmostFirst,
    LeftmostLongest,
}
```

A knob for controlling the match semantics of an Aho-Corasick automaton.

There are two generally different ways that Aho-Corasick automatons can
report matches. The first way is the "standard" approach that results from
implementing most textbook explanations of Aho-Corasick. The second way is
to report only the leftmost non-overlapping matches. The leftmost approach
is in turn split into two different ways of resolving ambiguous matches:
leftmost-first and leftmost-longest.

The `Standard` match kind is the default and is the only one that supports
overlapping matches and stream searching. (Trying to find overlapping or
streaming matches using leftmost match semantics will result in an error in
fallible APIs and a panic when using infallibe APIs.) The `Standard` match
kind will report matches as they are seen. When searching for overlapping
matches, then all possible matches are reported. When searching for
non-overlapping matches, the first match seen is reported. For example, for
non-overlapping matches, given the patterns `abcd` and `b` and the haystack
`abcdef`, only a match for `b` is reported since it is detected first. The
`abcd` match is never reported since it overlaps with the `b` match.

In contrast, the leftmost match kind always prefers the leftmost match
among all possible matches. Given the same example as above with `abcd` and
`b` as patterns and `abcdef` as the haystack, the leftmost match is `abcd`
since it begins before the `b` match, even though the `b` match is detected
before the `abcd` match. In this case, the `b` match is not reported at all
since it overlaps with the `abcd` match.

The difference between leftmost-first and leftmost-longest is in how they
resolve ambiguous matches when there are multiple leftmost matches to
choose from. Leftmost-first always chooses the pattern that was provided
earliest, where as leftmost-longest always chooses the longest matching
pattern. For example, given the patterns `a` and `ab` and the subject
string `ab`, the leftmost-first match is `a` but the leftmost-longest match
is `ab`. Conversely, if the patterns were given in reverse order, i.e.,
`ab` and `a`, then both the leftmost-first and leftmost-longest matches
would be `ab`. Stated differently, the leftmost-first match depends on the
order in which the patterns were given to the Aho-Corasick automaton.
Because of that, when leftmost-first matching is used, if a pattern `A`
that appears before a pattern `B` is a prefix of `B`, then it is impossible
to ever observe a match of `B`.

If you're not sure which match kind to pick, then stick with the standard
kind, which is the default. In particular, if you need overlapping or
streaming matches, then you _must_ use the standard kind. The leftmost
kinds are useful in specific circumstances. For example, leftmost-first can
be very useful as a way to implement match priority based on the order of
patterns given and leftmost-longest can be useful for dictionary searching
such that only the longest matching words are reported.

# Relationship with regular expression alternations

Understanding match semantics can be a little tricky, and one easy way
to conceptualize non-overlapping matches from an Aho-Corasick automaton
is to think about them as a simple alternation of literals in a regular
expression. For example, let's say we wanted to match the strings
`Sam` and `Samwise`, which would turn into the regex `Sam|Samwise`. It
turns out that regular expression engines have two different ways of
matching this alternation. The first way, leftmost-longest, is commonly
found in POSIX compatible implementations of regular expressions (such as
`grep`). The second way, leftmost-first, is commonly found in backtracking
implementations such as Perl. (Some regex engines, such as RE2 and Rust's
regex engine do not use backtracking, but still implement leftmost-first
semantics in an effort to match the behavior of dominant backtracking
regex engines such as those found in Perl, Ruby, Python, Javascript and
PHP.)

That is, when matching `Sam|Samwise` against `Samwise`, a POSIX regex
will match `Samwise` because it is the longest possible match, but a
Perl-like regex will match `Sam` since it appears earlier in the
alternation. Indeed, the regex `Sam|Samwise` in a Perl-like regex engine
will never match `Samwise` since `Sam` will always have higher priority.
Conversely, matching the regex `Samwise|Sam` against `Samwise` will lead to
a match of `Samwise` in both POSIX and Perl-like regexes since `Samwise` is
still longest match, but it also appears earlier than `Sam`.

The "standard" match semantics of Aho-Corasick generally don't correspond
to the match semantics of any large group of regex implementations, so
there's no direct analogy that can be made here. Standard match semantics
are generally useful for overlapping matches, or if you just want to see
matches as they are detected.

The main conclusion to draw from this section is that the match semantics
can be tweaked to precisely match either Perl-like regex alternations or
POSIX regex alternations.

#### Variants

- **`Standard`**

  Use standard match semantics, which support overlapping matches. When
  used with non-overlapping matches, matches are reported as they are
  seen.

- **`LeftmostFirst`**

  Use leftmost-first match semantics, which reports leftmost matches.
  When there are multiple possible leftmost matches, the match
  corresponding to the pattern that appeared earlier when constructing
  the automaton is reported.
  
  This does **not** support overlapping matches or stream searching. If
  this match kind is used, attempting to find overlapping matches or
  stream matches will fail.

- **`LeftmostLongest`**

  Use leftmost-longest match semantics, which reports leftmost matches.
  When there are multiple possible leftmost matches, the longest match
  is chosen.
  
  This does **not** support overlapping matches or stream searching. If
  this match kind is used, attempting to find overlapping matches or
  stream matches will fail.

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

### `StartKind`

```rust
enum StartKind {
    Both,
    Unanchored,
    Anchored,
}
```

The kind of anchored starting configurations to support in an Aho-Corasick
searcher.

Depending on which searcher is used internally by
[`AhoCorasick`](crate::AhoCorasick), supporting both unanchored
and anchored searches can be quite costly. For this reason,
[`AhoCorasickBuilder::start_kind`](crate::AhoCorasickBuilder::start_kind)
can be used to configure whether your searcher supports unanchored,
anchored or both kinds of searches.

This searcher configuration knob works in concert with the search time
configuration [`Input::anchored`](#anchored). Namely, if one requests an unsupported
anchored mode, then the search will either panic or return an error,
depending on whether you're using infallible or fallibe APIs, respectively.

`AhoCorasick` by default only supports unanchored searches.

#### Variants

- **`Both`**

  Support both anchored and unanchored searches.

- **`Unanchored`**

  Support only unanchored searches. Requesting an anchored search will
  return an error in fallible APIs and panic in infallible APIs.

- **`Anchored`**

  Support only anchored searches. Requesting an unanchored search will
  return an error in fallible APIs and panic in infallible APIs.

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

- `fn clone(self: &Self) -> StartKind`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl Copy`

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &StartKind) -> bool`

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

- `fn default() -> StartKind`

