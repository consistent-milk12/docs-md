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
match semantics means, see the [`MatchKind`](util/search/index.md) type.

# Overview

This section gives a brief overview of the primary types in this crate:

* [`AhoCorasick`](ahocorasick/index.md) is the primary type and represents an Aho-Corasick automaton.
This is the type you use to execute searches.
* [`AhoCorasickBuilder`](ahocorasick/index.md) can be used to build an Aho-Corasick automaton, and
supports configuring a number of options.
* [`Match`](util/search/index.md) represents a single match reported by an Aho-Corasick automaton.
Each match has two pieces of information: the pattern that matched and the
start and end byte offsets corresponding to the position in the haystack at
which it matched.

# Example: basic searching

This example shows how to search for occurrences of multiple patterns
simultaneously. Each match includes the pattern that matched along with the
byte offsets of the match.

```rust
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

```rust
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

```rust
#[cfg(feature = "std")] {
use aho_corasick::AhoCorasick;

fn example() -> Result<(), std::io::Error> {
let patterns = &["fox", "brown", "quick"];
let replace_with = &["sloth", "grey", "slow"];

// In a real example, these might be `std::fs::File`s instead. All you need to
// do is supply a pair of `std::io::Read` and `std::io::Write` implementations.
let rdr = "The quick brown fox.";
let mut wtr = vec![];

let ac = AhoCorasick::new(patterns).unwrap();
ac.try_stream_replace_all(rdr.as_bytes(), &mut wtr, replace_with)?;
assert_eq!(b"The slow grey sloth.".to_vec(), wtr);
Ok(()) }; example().unwrap()
}
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

```rust
use aho_corasick::AhoCorasick;

let patterns = &["Samwise", "Sam"];
let haystack = "Samwise";

let ac = AhoCorasick::new(patterns).unwrap();
let mat = ac.find(haystack).expect("should have a match");
assert_eq!("Sam", &haystack[mat.start()..mat.end()]);
```

And now here's the leftmost-first version, which matches how a Perl-like
regex will work:

```rust
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
expression alternation. See [`MatchKind`](util/search/index.md) for more details.

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
disabled via `AhoCorasickBuilder::prefilter`.

# Lower level APIs

This crate also provides several sub-modules that collectively expose many of
the implementation details of the main [`AhoCorasick`](ahocorasick/index.md) type. Most users of this
library can completely ignore the submodules and their contents, but if you
needed finer grained control, some parts of them may be useful to you. Here is
a brief overview of each and why you might want to use them:

* The [`packed`](packed/index.md) sub-module contains a lower level API for using fast
vectorized routines for finding a small number of patterns in a haystack.
You might want to use this API when you want to completely side-step using
Aho-Corasick automata. Otherwise, the fast vectorized routines are used
automatically as prefilters for `AhoCorasick` searches whenever possible.
* The [`automaton`](automaton/index.md) sub-module provides a lower level finite state
machine interface that the various Aho-Corasick implementations in
this crate implement. This sub-module's main contribution is the
[`Automaton`](automaton::Automaton) trait, which permits manually walking the
state transitions of an Aho-Corasick automaton.
* The [`dfa`](dfa/index.md) and [`nfa`](nfa/index.md) sub-modules provide DFA and NFA implementations of
the aforementioned `Automaton` trait. The main reason one might want to use
these sub-modules is to get access to a type that implements the `Automaton`
trait. (The top-level `AhoCorasick` type does not implement the `Automaton`
trait.)

As mentioned above, if you aren't sure whether you need these sub-modules,
you should be able to safely ignore them and just focus on the [`AhoCorasick`](ahocorasick/index.md)
type.

# Crate features

This crate exposes a few features for controlling dependency usage and whether
this crate can be used without the standard library.

* **std** -
  Enables support for the standard library. This feature is enabled by
  default. When disabled, only `core` and `alloc` are used. At an API
  level, enabling `std` enables `std::error::Error` trait impls for the
  various error types, and higher level stream search routines such as
  `AhoCorasick::try_stream_find_iter`. But the `std` feature is also required
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

## Contents

- [Modules](#modules)
  - [`macros`](#macros)
  - [`ahocorasick`](#ahocorasick)
  - [`automaton`](#automaton)
  - [`dfa`](#dfa)
  - [`nfa`](#nfa)
  - [`packed`](#packed)
  - [`util`](#util)
- [Structs](#structs)
  - [`StreamFindIter`](#streamfinditer)
  - [`AhoCorasick`](#ahocorasick)
  - [`AhoCorasickBuilder`](#ahocorasickbuilder)
  - [`FindIter`](#finditer)
  - [`FindOverlappingIter`](#findoverlappingiter)
  - [`BuildError`](#builderror)
  - [`MatchError`](#matcherror)
  - [`PatternID`](#patternid)
  - [`PatternIDError`](#patterniderror)
  - [`Input`](#input)
  - [`Match`](#match)
  - [`Span`](#span)
- [Enums](#enums)
  - [`AhoCorasickKind`](#ahocorasickkind)
  - [`MatchErrorKind`](#matcherrorkind)
  - [`Anchored`](#anchored)
  - [`MatchKind`](#matchkind)
  - [`StartKind`](#startkind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`macros`](#macros) | mod |  |
| [`ahocorasick`](#ahocorasick) | mod |  |
| [`automaton`](#automaton) | mod | Provides [`Automaton`] trait for abstracting over Aho-Corasick automata. |
| [`dfa`](#dfa) | mod | Provides direct access to a DFA implementation of Aho-Corasick. |
| [`nfa`](#nfa) | mod | Provides direct access to NFA implementations of Aho-Corasick. |
| [`packed`](#packed) | mod | Provides packed multiple substring search, principally for a small number of patterns. |
| [`util`](#util) | mod |  |
| [`StreamFindIter`](#streamfinditer) | struct |  |
| [`AhoCorasick`](#ahocorasick) | struct |  |
| [`AhoCorasickBuilder`](#ahocorasickbuilder) | struct |  |
| [`FindIter`](#finditer) | struct |  |
| [`FindOverlappingIter`](#findoverlappingiter) | struct |  |
| [`BuildError`](#builderror) | struct |  |
| [`MatchError`](#matcherror) | struct |  |
| [`PatternID`](#patternid) | struct |  |
| [`PatternIDError`](#patterniderror) | struct |  |
| [`Input`](#input) | struct |  |
| [`Match`](#match) | struct |  |
| [`Span`](#span) | struct |  |
| [`AhoCorasickKind`](#ahocorasickkind) | enum |  |
| [`MatchErrorKind`](#matcherrorkind) | enum |  |
| [`Anchored`](#anchored) | enum |  |
| [`MatchKind`](#matchkind) | enum |  |
| [`StartKind`](#startkind) | enum |  |

## Modules

- [`macros`](macros/index.md)
- [`ahocorasick`](ahocorasick/index.md)
- [`automaton`](automaton/index.md) — Provides [`Automaton`] trait for abstracting over Aho-Corasick automata.
- [`dfa`](dfa/index.md) — Provides direct access to a DFA implementation of Aho-Corasick.
- [`nfa`](nfa/index.md) — Provides direct access to NFA implementations of Aho-Corasick.
- [`packed`](packed/index.md) — Provides packed multiple substring search, principally for a small number of
- [`util`](util/index.md)

## Structs

### `StreamFindIter<'a, R>`

```rust
struct StreamFindIter<'a, R>(automaton::StreamFindIter<'a, alloc::sync::Arc<dyn AcAutomaton>, R>);
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2100-2102`](../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2100-L2102)*

An iterator that reports Aho-Corasick matches in a stream.

This iterator yields elements of type `Result<Match, std::io::Error>`,
where an error is reported if there was a problem reading from the
underlying stream. The iterator terminates only when the underlying stream
reaches `EOF`.

This iterator is constructed via the `AhoCorasick::stream_find_iter` and
`AhoCorasick::try_stream_find_iter` methods.

The type variable `R` refers to the `io::Read` stream that is being read
from.

The lifetime `'a` refers to the lifetime of the corresponding
[`AhoCorasick`](ahocorasick/index.md) searcher.

#### Trait Implementations

##### `impl Any for StreamFindIter<'a, R>`

- <span id="streamfinditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StreamFindIter<'a, R>`

- <span id="streamfinditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StreamFindIter<'a, R>`

- <span id="streamfinditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug> Debug for StreamFindIter<'a, R>`

- <span id="streamfinditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for StreamFindIter<'a, R>`

- <span id="streamfinditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StreamFindIter<'a, R>`

- <span id="streamfinditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for StreamFindIter<'a, R>`

- <span id="streamfinditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamfinditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamfinditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: std::io::Read> Iterator for StreamFindIter<'a, R>`

- <span id="streamfinditer-iterator-type-item"></span>`type Item = Result<Match, Error>`

- <span id="streamfinditer-iterator-next"></span>`fn next(&mut self) -> Option<Result<Match, std::io::Error>>` — [`Match`](util/search/index.md#match)

##### `impl<U> TryFrom for StreamFindIter<'a, R>`

- <span id="streamfinditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamfinditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StreamFindIter<'a, R>`

- <span id="streamfinditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamfinditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AhoCorasick`

```rust
struct AhoCorasick {
    aut: alloc::sync::Arc<dyn AcAutomaton>,
    kind: AhoCorasickKind,
    start_kind: crate::util::search::StartKind,
}
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:177-214`](../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L177-L214)*

An automaton for searching multiple strings in linear time.

The `AhoCorasick` type supports a few basic ways of constructing an
automaton, with the default being `AhoCorasick::new`. However, there
are a fair number of configurable options that can be set by using
[`AhoCorasickBuilder`](ahocorasick/index.md) instead. Such options include, but are not limited
to, how matches are determined, simple case insensitivity, whether to use a
DFA or not and various knobs for controlling the space-vs-time trade offs
taken when building the automaton.

# Resource usage

Aho-Corasick automatons are always constructed in `O(p)` time, where
`p` is the combined length of all patterns being searched. With that
said, building an automaton can be fairly costly because of high constant
factors, particularly when enabling the [DFA](AhoCorasickKind::DFA) option
with `AhoCorasickBuilder::kind`. For this reason, it's generally a good
idea to build an automaton once and reuse it as much as possible.

Aho-Corasick automatons can also use a fair bit of memory. To get
a concrete idea of how much memory is being used, try using the
`AhoCorasick::memory_usage` method.

To give a quick idea of the differences between Aho-Corasick
implementations and their resource usage, here's a sample of construction
times and heap memory used after building an automaton from 100,000
randomly selected titles from Wikipedia:

* 99MB for a [`noncontiguous::NFA`](nfa/noncontiguous/index.md) in 240ms.
* 21MB for a [`contiguous::NFA`](nfa/contiguous/index.md) in 275ms.
* 1.6GB for a [`dfa::DFA`](dfa/index.md) in 1.88s.

(Note that the memory usage above reflects the size of each automaton and
not peak memory usage. For example, building a contiguous NFA requires
first building a noncontiguous NFA. Once the contiguous NFA is built, the
noncontiguous NFA is freed.)

This experiment very strongly argues that a contiguous NFA is often the
best balance in terms of resource usage. It takes a little longer to build,
but its memory usage is quite small. Its search speed (not listed) is
also often faster than a noncontiguous NFA, but a little slower than a
DFA. Indeed, when no specific [`AhoCorasickKind`](ahocorasick/index.md) is used (which is the
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
to an [`Input`](util/search/index.md). This includes `&[u8]`, `&str` and `Input` itself.

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

For example, the `AhoCorasick::find_iter` method is the infallible
version of the `AhoCorasick::try_find_iter` method.

Examples of errors that can occur:

* Running a search that requires [`MatchKind::Standard`](#matchkindstandard) semantics (such
as a stream or overlapping search) with an automaton that was built with
[`MatchKind::LeftmostFirst`](#matchkindleftmostfirst) or [`MatchKind::LeftmostLongest`](#matchkindleftmostlongest) semantics.
* Running an anchored search with an automaton that only supports
unanchored searches. (By default, `AhoCorasick` only supports unanchored
searches. But this can be toggled with `AhoCorasickBuilder::start_kind`.)
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

```rust
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

```rust
use aho_corasick::AhoCorasick;

let patterns = &["fox", "brown", "quick"];
let haystack = "The quick brown fox.";
let replace_with = &["sloth", "grey", "slow"];

let ac = AhoCorasick::new(patterns).unwrap();
let result = ac.replace_all(haystack, replace_with);
assert_eq!(result, "The slow grey sloth.");
```

#### Fields

- **`aut`**: `alloc::sync::Arc<dyn AcAutomaton>`

  The underlying Aho-Corasick automaton. It's one of
  nfa::noncontiguous::NFA, nfa::contiguous::NFA or dfa::DFA.

- **`kind`**: `AhoCorasickKind`

  The specific Aho-Corasick kind chosen. This makes it possible to
  inspect any `AhoCorasick` and know what kind of search strategy it
  uses.

- **`start_kind`**: `crate::util::search::StartKind`

  The start kind of this automaton as configured by the caller.
  
  We don't really *need* to put this here, since the underlying automaton
  will correctly return errors if the caller requests an unsupported
  search type. But we do keep this here for API behavior consistency.
  Namely, the NFAs in this crate support both unanchored and anchored
  searches unconditionally. There's no way to disable one or the other.
  They always both work. But the DFA in this crate specifically only
  supports both unanchored and anchored searches if it's configured to
  do so. Why? Because for the DFA, supporting both essentially requires
  two copies of the transition table: one generated by following failure
  transitions from the original NFA and one generated by not following
  those failure transitions.
  
  So why record the start kind here? Well, consider what happens
  when no specific 'AhoCorasickKind' is selected by the caller and
  'StartKind::Unanchored' is used (both are the default). It *might*
  result in using a DFA or it might pick an NFA. If it picks an NFA, the
  caller would then be able to run anchored searches, even though the
  caller only asked for support for unanchored searches. Maybe that's
  fine, but what if the DFA was chosen instead? Oops, the caller would
  get an error.
  
  Basically, it seems bad to return an error or not based on some
  internal implementation choice. So we smooth things out and ensure
  anchored searches *always* report an error when only unanchored support
  was asked for (and vice versa), even if the underlying automaton
  supports it.

#### Implementations

- <span id="ahocorasick-new"></span>`fn new<I, P>(patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](ahocorasick/index.md#ahocorasick), [`BuildError`](util/error/index.md#builderror)

  Create a new Aho-Corasick automaton using the default configuration.

  

  The default configuration optimizes for less space usage, but at the

  expense of longer search times. To change the configuration, use

  [`AhoCorasickBuilder`](ahocorasick/index.md).

  

  This uses the default [`MatchKind::Standard`](#matchkindstandard) match semantics, which

  reports a match as soon as it is found. This corresponds to the

  standard match semantics supported by textbook descriptions of the

  Aho-Corasick algorithm.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use aho_corasick::{AhoCorasick, PatternID};

  

  let ac = AhoCorasick::new(&["foo", "bar", "baz"]).unwrap();

  assert_eq!(

      Some(PatternID::must(1)),

      ac.find("xxx bar xxx").map(|m| m.pattern()),

  );

  ```

- <span id="ahocorasick-builder"></span>`fn builder() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  A convenience method for returning a new Aho-Corasick builder.

  

  This usually permits one to just import the `AhoCorasick` type.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use aho_corasick::{AhoCorasick, Match, MatchKind};

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostFirst)

      .build(&["samwise", "sam"])

      .unwrap();

  assert_eq!(Some(Match::must(0, 0..7)), ac.find("samwise"));

  ```

#### Trait Implementations

##### `impl Any for AhoCorasick`

- <span id="ahocorasick-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AhoCorasick`

- <span id="ahocorasick-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AhoCorasick`

- <span id="ahocorasick-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AhoCorasick`

- <span id="ahocorasick-clone"></span>`fn clone(&self) -> AhoCorasick` — [`AhoCorasick`](ahocorasick/index.md#ahocorasick)

##### `impl CloneToUninit for AhoCorasick`

- <span id="ahocorasick-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AhoCorasick`

- <span id="ahocorasick-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for AhoCorasick`

- <span id="ahocorasick-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AhoCorasick`

- <span id="ahocorasick-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AhoCorasick`

- <span id="ahocorasick-toowned-type-owned"></span>`type Owned = T`

- <span id="ahocorasick-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ahocorasick-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AhoCorasick`

- <span id="ahocorasick-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ahocorasick-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AhoCorasick`

- <span id="ahocorasick-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ahocorasick-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `AhoCorasickBuilder`

```rust
struct AhoCorasickBuilder {
    nfa_noncontiguous: noncontiguous::Builder,
    nfa_contiguous: contiguous::Builder,
    dfa: dfa::Builder,
    kind: Option<AhoCorasickKind>,
    start_kind: crate::util::search::StartKind,
}
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2135-2141`](../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2135-L2141)*

A builder for configuring an Aho-Corasick automaton.

# Quick advice

* Use `AhoCorasickBuilder::match_kind` to configure your searcher
with [`MatchKind::LeftmostFirst`](#matchkindleftmostfirst) if you want to match how backtracking
regex engines execute searches for `pat1|pat2|..|patN`. Use
[`MatchKind::LeftmostLongest`](#matchkindleftmostlongest) if you want to match how POSIX regex engines
do it.
* If you need an anchored search, use `AhoCorasickBuilder::start_kind` to
set the [`StartKind::Anchored`](#startkindanchored) mode since [`StartKind::Unanchored`](#startkindunanchored) is the
default. Or just use [`StartKind::Both`](#startkindboth) to support both types of searches.
* You might want to use `AhoCorasickBuilder::kind` to set your searcher
to always use a [`AhoCorasickKind::DFA`](#ahocorasickkinddfa) if search speed is critical and
memory usage isn't a concern. Otherwise, not setting a kind will probably
make the right choice for you. Beware that if you use [`StartKind::Both`](#startkindboth)
to build a searcher that supports both unanchored and anchored searches
_and_ you set [`AhoCorasickKind::DFA`](#ahocorasickkinddfa), then the DFA will essentially be
duplicated to support both simultaneously. This results in very high memory
usage.
* For all other options, their defaults are almost certainly what you want.

#### Implementations

- <span id="ahocorasickbuilder-new"></span>`fn new() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Create a new builder for configuring an Aho-Corasick automaton.

  

  The builder provides a way to configure a number of things, including

  ASCII case insensitivity and what kind of match semantics are used.

- <span id="ahocorasickbuilder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](ahocorasick/index.md#ahocorasick), [`BuildError`](util/error/index.md#builderror)

  Build an Aho-Corasick automaton using the configuration set on this

  builder.

  

  A builder may be reused to create more automatons.

  

  # Examples

  

  Basic usage:

  

  ```rust

  use aho_corasick::{AhoCorasickBuilder, PatternID};

  

  let patterns = &["foo", "bar", "baz"];

  let ac = AhoCorasickBuilder::new().build(patterns).unwrap();

  assert_eq!(

      Some(PatternID::must(1)),

      ac.find("xxx bar xxx").map(|m| m.pattern()),

  );

  ```

- <span id="ahocorasickbuilder-build-auto"></span>`fn build_auto(&self, nfa: noncontiguous::NFA) -> (Arc<dyn AcAutomaton>, AhoCorasickKind)` — [`NFA`](nfa/noncontiguous/index.md#nfa), [`AcAutomaton`](ahocorasick/index.md#acautomaton), [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind)

  Implements the automatic selection logic for the Aho-Corasick

  implementation to use. Since all Aho-Corasick automatons are built

  from a non-contiguous NFA, the caller is responsible for building

  that first.

- <span id="ahocorasickbuilder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut AhoCorasickBuilder` — [`MatchKind`](util/search/index.md#matchkind), [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Set the desired match semantics.

  

  The default is [`MatchKind::Standard`](#matchkindstandard), which corresponds to the match

  semantics supported by the standard textbook description of the

  Aho-Corasick algorithm. Namely, matches are reported as soon as they

  are found. Moreover, this is the only way to get overlapping matches

  or do stream searching.

  

  The other kinds of match semantics that are supported are

  [`MatchKind::LeftmostFirst`](#matchkindleftmostfirst) and [`MatchKind::LeftmostLongest`](#matchkindleftmostlongest). The

  former corresponds to the match you would get if you were to try to

  match each pattern at each position in the haystack in the same order

  that you give to the automaton. That is, it returns the leftmost match

  corresponding to the earliest pattern given to the automaton. The

  latter corresponds to finding the longest possible match among all

  leftmost matches.

  

  For more details on match semantics, see the [documentation for

  `MatchKind`](MatchKind).

  

  Note that setting this to [`MatchKind::LeftmostFirst`](#matchkindleftmostfirst) or

  [`MatchKind::LeftmostLongest`](#matchkindleftmostlongest) will cause some search routines on

  [`AhoCorasick`](ahocorasick/index.md) to return an error (or panic if you're using the

  infallible API). Notably, this includes stream and overlapping

  searches.

  

  # Examples

  

  In these examples, we demonstrate the differences between match

  semantics for a particular set of patterns in a specific order:

  `b`, `abc`, `abcd`.

  

  Standard semantics:

  

  ```rust

  use aho_corasick::{AhoCorasick, MatchKind};

  

  let patterns = &["b", "abc", "abcd"];

  let haystack = "abcd";

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::Standard) // default, not necessary

      .build(patterns)

      .unwrap();

  let mat = ac.find(haystack).expect("should have a match");

  assert_eq!("b", &haystack[mat.start()..mat.end()]);

  ```

  

  Leftmost-first semantics:

  

  ```rust

  use aho_corasick::{AhoCorasick, MatchKind};

  

  let patterns = &["b", "abc", "abcd"];

  let haystack = "abcd";

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostFirst)

      .build(patterns)

      .unwrap();

  let mat = ac.find(haystack).expect("should have a match");

  assert_eq!("abc", &haystack[mat.start()..mat.end()]);

  ```

  

  Leftmost-longest semantics:

  

  ```rust

  use aho_corasick::{AhoCorasick, MatchKind};

  

  let patterns = &["b", "abc", "abcd"];

  let haystack = "abcd";

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostLongest)

      .build(patterns)

      .unwrap();

  let mat = ac.find(haystack).expect("should have a match");

  assert_eq!("abcd", &haystack[mat.start()..mat.end()]);

  ```

- <span id="ahocorasickbuilder-start-kind"></span>`fn start_kind(&mut self, kind: StartKind) -> &mut AhoCorasickBuilder` — [`StartKind`](util/search/index.md#startkind), [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Sets the starting state configuration for the automaton.

  

  Every Aho-Corasick automaton is capable of having two start states: one

  that is used for unanchored searches and one that is used for anchored

  searches. Some automatons, like the NFAs, support this with almost zero

  additional cost. Other automatons, like the DFA, require two copies of

  the underlying transition table to support both simultaneously.

  

  Because there may be an added non-trivial cost to supporting both, it

  is possible to configure which starting state configuration is needed.

  

  Indeed, since anchored searches tend to be somewhat more rare,

  _only_ unanchored searches are supported by default. Thus,

  [`StartKind::Unanchored`](#startkindunanchored) is the default.

  

  Note that when this is set to [`StartKind::Unanchored`](#startkindunanchored), then

  running an anchored search will result in an error (or a panic

  if using the infallible APIs). Similarly, when this is set to

  [`StartKind::Anchored`](#startkindanchored), then running an unanchored search will

  result in an error (or a panic if using the infallible APIs). When

  [`StartKind::Both`](#startkindboth) is used, then both unanchored and anchored searches

  are always supported.

  

  Also note that even if an `AhoCorasick` searcher is using an NFA

  internally (which always supports both unanchored and anchored

  searches), an error will still be reported for a search that isn't

  supported by the configuration set via this method. This means,

  for example, that an error is never dependent on which internal

  implementation of Aho-Corasick is used.

  

  # Example: anchored search

  

  This shows how to build a searcher that only supports anchored

  searches:

  

  ```rust

  use aho_corasick::{

      AhoCorasick, Anchored, Input, Match, MatchKind, StartKind,

  };

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostFirst)

      .start_kind(StartKind::Anchored)

      .build(&["b", "abc", "abcd"])

      .unwrap();

  

  // An unanchored search is not supported! An error here is guaranteed

  // given the configuration above regardless of which kind of

  // Aho-Corasick implementation ends up being used internally.

  let input = Input::new("foo abcd").anchored(Anchored::No);

  assert!(ac.try_find(input).is_err());

  

  let input = Input::new("foo abcd").anchored(Anchored::Yes);

  assert_eq!(None, ac.try_find(input)?);

  

  let input = Input::new("abcd").anchored(Anchored::Yes);

  assert_eq!(Some(Match::must(1, 0..3)), ac.try_find(input)?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  # Example: unanchored and anchored searches

  

  This shows how to build a searcher that supports both unanchored and

  anchored searches:

  

  ```rust

  use aho_corasick::{

      AhoCorasick, Anchored, Input, Match, MatchKind, StartKind,

  };

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostFirst)

      .start_kind(StartKind::Both)

      .build(&["b", "abc", "abcd"])

      .unwrap();

  

  let input = Input::new("foo abcd").anchored(Anchored::No);

  assert_eq!(Some(Match::must(1, 4..7)), ac.try_find(input)?);

  

  let input = Input::new("foo abcd").anchored(Anchored::Yes);

  assert_eq!(None, ac.try_find(input)?);

  

  let input = Input::new("abcd").anchored(Anchored::Yes);

  assert_eq!(Some(Match::must(1, 0..3)), ac.try_find(input)?);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="ahocorasickbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Enable ASCII-aware case insensitive matching.

  

  When this option is enabled, searching will be performed without

  respect to case for ASCII letters (`a-z` and `A-Z`) only.

  

  Enabling this option does not change the search algorithm, but it may

  increase the size of the automaton.

  

  **NOTE:** It is unlikely that support for Unicode case folding will

  be added in the future. The ASCII case works via a simple hack to the

  underlying automaton, but full Unicode handling requires a fair bit of

  sophistication. If you do need Unicode handling, you might consider

  using the [`regex` crate](https://docs.rs/regex) or the lower level

  [`regex-automata` crate](https://docs.rs/regex-automata).

  

  # Examples

  

  Basic usage:

  

  ```rust

  use aho_corasick::AhoCorasick;

  

  let patterns = &["FOO", "bAr", "BaZ"];

  let haystack = "foo bar baz";

  

  let ac = AhoCorasick::builder()

      .ascii_case_insensitive(true)

      .build(patterns)

      .unwrap();

  assert_eq!(3, ac.find_iter(haystack).count());

  ```

- <span id="ahocorasickbuilder-kind"></span>`fn kind(&mut self, kind: Option<AhoCorasickKind>) -> &mut AhoCorasickBuilder` — [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind), [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Choose the type of underlying automaton to use.

  

  Currently, there are four choices:

  

  * [`AhoCorasickKind::NoncontiguousNFA`](#ahocorasickkindnoncontiguousnfa) instructs the searcher to

  use a [`noncontiguous::NFA`](nfa/noncontiguous/index.md). A noncontiguous NFA is the fastest to

  be built, has moderate memory usage and is typically the slowest to

  execute a search.

  * [`AhoCorasickKind::ContiguousNFA`](#ahocorasickkindcontiguousnfa) instructs the searcher to use a

  [`contiguous::NFA`](nfa/contiguous/index.md). A contiguous NFA is a little slower to build than

  a noncontiguous NFA, has excellent memory usage and is typically a

  little slower than a DFA for a search.

  * [`AhoCorasickKind::DFA`](#ahocorasickkinddfa) instructs the searcher to use a

  [`dfa::DFA`](dfa/index.md). A DFA is very slow to build, uses exorbitant amounts of

  memory, but will typically execute searches the fastest.

  * `None` (the default) instructs the searcher to choose the "best"

  Aho-Corasick implementation. This choice is typically based primarily

  on the number of patterns.

  

  Setting this configuration does not change the time complexity for

  constructing the Aho-Corasick automaton (which is `O(p)` where `p`

  is the total number of patterns being compiled). Setting this to

  [`AhoCorasickKind::DFA`](#ahocorasickkinddfa) does however reduce the time complexity of

  non-overlapping searches from `O(n + p)` to `O(n)`, where `n` is the

  length of the haystack.

  

  In general, you should probably stick to the default unless you have

  some kind of reason to use a specific Aho-Corasick implementation. For

  example, you might choose `AhoCorasickKind::DFA` if you don't care

  about memory usage and want the fastest possible search times.

  

  Setting this guarantees that the searcher returned uses the chosen

  implementation. If that implementation could not be constructed, then

  an error will be returned. In contrast, when `None` is used, it is

  possible for it to attempt to construct, for example, a contiguous

  NFA and have it fail. In which case, it will fall back to using a

  noncontiguous NFA.

  

  If `None` is given, then one may use `AhoCorasick::kind` to determine

  which Aho-Corasick implementation was chosen.

  

  Note that the heuristics used for choosing which `AhoCorasickKind`

  may be changed in a semver compatible release.

- <span id="ahocorasickbuilder-prefilter"></span>`fn prefilter(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Enable heuristic prefilter optimizations.

  

  When enabled, searching will attempt to quickly skip to match

  candidates using specialized literal search routines. A prefilter

  cannot always be used, and is generally treated as a heuristic. It

  can be useful to disable this if the prefilter is observed to be

  sub-optimal for a particular workload.

  

  Currently, prefilters are typically only active when building searchers

  with a small (less than 100) number of patterns.

  

  This is enabled by default.

- <span id="ahocorasickbuilder-dense-depth"></span>`fn dense_depth(&mut self, depth: usize) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  Set the limit on how many states use a dense representation for their

  transitions. Other states will generally use a sparse representation.

  

  A dense representation uses more memory but is generally faster, since

  the next transition in a dense representation can be computed in a

  constant number of instructions. A sparse representation uses less

  memory but is generally slower, since the next transition in a sparse

  representation requires executing a variable number of instructions.

  

  This setting is only used when an Aho-Corasick implementation is used

  that supports the dense versus sparse representation trade off. Not all

  do.

  

  This limit is expressed in terms of the depth of a state, i.e., the

  number of transitions from the starting state of the automaton. The

  idea is that most of the time searching will be spent near the starting

  state of the automaton, so states near the start state should use a

  dense representation. States further away from the start state would

  then use a sparse representation.

  

  By default, this is set to a low but non-zero number. Setting this to

  `0` is almost never what you want, since it is likely to make searches

  very slow due to the start state itself being forced to use a sparse

  representation. However, it is unlikely that increasing this number

  will help things much, since the most active states have a small depth.

  More to the point, the memory usage increases superlinearly as this

  number increases.

- <span id="ahocorasickbuilder-byte-classes"></span>`fn byte_classes(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

  A debug settting for whether to attempt to shrink the size of the

  automaton's alphabet or not.

  

  This option is enabled by default and should never be disabled unless

  one is debugging the underlying automaton.

  

  When enabled, some (but not all) Aho-Corasick automatons will use a map

  from all possible bytes to their corresponding equivalence class. Each

  equivalence class represents a set of bytes that does not discriminate

  between a match and a non-match in the automaton.

  

  The advantage of this map is that the size of the transition table can

  be reduced drastically from `#states * 256 * sizeof(u32)` to

  `#states * k * sizeof(u32)` where `k` is the number of equivalence

  classes (rounded up to the nearest power of 2). As a result, total

  space usage can decrease substantially. Moreover, since a smaller

  alphabet is used, automaton compilation becomes faster as well.

  

  **WARNING:** This is only useful for debugging automatons. Disabling

  this does not yield any speed advantages. Namely, even when this is

  disabled, a byte class map is still used while searching. The only

  difference is that every byte will be forced into its own distinct

  equivalence class. This is useful for debugging the actual generated

  transitions because it lets one see the transitions defined on actual

  bytes instead of the equivalence classes.

#### Trait Implementations

##### `impl Any for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-clone"></span>`fn clone(&self) -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

##### `impl CloneToUninit for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-default"></span>`fn default() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

##### `impl<T> From for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="ahocorasickbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ahocorasickbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ahocorasickbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ahocorasickbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindIter<'a, 'h>`

```rust
struct FindIter<'a, 'h>(automaton::FindIter<'a, 'h, alloc::sync::Arc<dyn AcAutomaton>>);
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2047`](../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2047)*

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`](util/search/index.md) used by this
automaton.

This iterator is constructed via the `AhoCorasick::find_iter` and
`AhoCorasick::try_find_iter` methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Any for FindIter<'a, 'h>`

- <span id="finditer-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindIter<'a, 'h>`

- <span id="finditer-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindIter<'a, 'h>`

- <span id="finditer-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FindIter<'a, 'h>`

- <span id="finditer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FindIter<'a, 'h>`

- <span id="finditer-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindIter<'a, 'h>`

- <span id="finditer-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for FindIter<'a, 'h>`

- <span id="finditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="finditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="finditer-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindIter<'a, 'h>`

- <span id="finditer-iterator-type-item"></span>`type Item = Match`

- <span id="finditer-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](util/search/index.md#match)

##### `impl<U> TryFrom for FindIter<'a, 'h>`

- <span id="finditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="finditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindIter<'a, 'h>`

- <span id="finditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="finditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `FindOverlappingIter<'a, 'h>`

```rust
struct FindOverlappingIter<'a, 'h>(automaton::FindOverlappingIter<'a, 'h, alloc::sync::Arc<dyn AcAutomaton>>);
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2070-2072`](../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2070-L2072)*

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the `AhoCorasick::find_overlapping_iter`
and `AhoCorasick::try_find_overlapping_iter` methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Any for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findoverlappingiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findoverlappingiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-iterator-type-item"></span>`type Item = Match`

- <span id="findoverlappingiter-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](util/search/index.md#match)

##### `impl<U> TryFrom for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findoverlappingiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findoverlappingiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `BuildError`

```rust
struct BuildError {
    kind: ErrorKind,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:17-19`](../../.source_1765521767/aho-corasick-1.1.4/src/util/error.rs#L17-L19)*

An error that occurred during the construction of an Aho-Corasick
automaton.

Build errors occur when some kind of limit has been exceeded, either in the
number of states, the number of patterns of the length of a pattern. These
limits aren't part of the public API, but they should generally be large
enough to handle most use cases.

When the `std` feature is enabled, this implements the `std::error::Error`
trait.

#### Implementations

- <span id="builderror-state-id-overflow"></span>`fn state_id_overflow(max: u64, requested_max: u64) -> BuildError` — [`BuildError`](util/error/index.md#builderror)

- <span id="builderror-pattern-id-overflow"></span>`fn pattern_id_overflow(max: u64, requested_max: u64) -> BuildError` — [`BuildError`](util/error/index.md#builderror)

- <span id="builderror-pattern-too-long"></span>`fn pattern_too_long(pattern: PatternID, len: usize) -> BuildError` — [`PatternID`](util/primitives/index.md#patternid), [`BuildError`](util/error/index.md#builderror)

#### Trait Implementations

##### `impl Any for BuildError`

- <span id="builderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for BuildError`

- <span id="builderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for BuildError`

- <span id="builderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](util/error/index.md#builderror)

##### `impl CloneToUninit for BuildError`

- <span id="builderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for BuildError`

- <span id="builderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

##### `impl<T> From for BuildError`

- <span id="builderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for BuildError`

- <span id="builderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for BuildError`

- <span id="builderror-toowned-type-owned"></span>`type Owned = T`

- <span id="builderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="builderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for BuildError`

- <span id="builderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for BuildError`

- <span id="builderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="builderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for BuildError`

- <span id="builderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="builderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:130`](../../.source_1765521767/aho-corasick-1.1.4/src/util/error.rs#L130)*

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

- <span id="matcherror-new"></span>`fn new(kind: MatchErrorKind) -> MatchError` — [`MatchErrorKind`](util/error/index.md#matcherrorkind), [`MatchError`](util/error/index.md#matcherror)

  Create a new error value with the given kind.

  

  This is a more verbose version of the kind-specific constructors, e.g.,

  `MatchError::unsupported_stream`.

- <span id="matcherror-kind"></span>`fn kind(&self) -> &MatchErrorKind` — [`MatchErrorKind`](util/error/index.md#matcherrorkind)

  Returns a reference to the underlying error kind.

- <span id="matcherror-invalid-input-anchored"></span>`fn invalid_input_anchored() -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

  Create a new "invalid anchored search" error. This occurs when the

  caller requests an anchored search but where anchored searches aren't

  supported.

  

  This is the same as calling `MatchError::new` with a

  [`MatchErrorKind::InvalidInputAnchored`](#matcherrorkindinvalidinputanchored) kind.

- <span id="matcherror-invalid-input-unanchored"></span>`fn invalid_input_unanchored() -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

  Create a new "invalid unanchored search" error. This occurs when the

  caller requests an unanchored search but where unanchored searches

  aren't supported.

  

  This is the same as calling `MatchError::new` with a

  [`MatchErrorKind::InvalidInputUnanchored`](#matcherrorkindinvalidinputunanchored) kind.

- <span id="matcherror-unsupported-stream"></span>`fn unsupported_stream(got: MatchKind) -> MatchError` — [`MatchKind`](util/search/index.md#matchkind), [`MatchError`](util/error/index.md#matcherror)

  Create a new "unsupported stream search" error. This occurs when the

  caller requests a stream search while using an Aho-Corasick automaton

  with a match kind other than [`MatchKind::Standard`](#matchkindstandard).

  

  The match kind given should be the match kind of the automaton. It

  should never be `MatchKind::Standard`.

- <span id="matcherror-unsupported-overlapping"></span>`fn unsupported_overlapping(got: MatchKind) -> MatchError` — [`MatchKind`](util/search/index.md#matchkind), [`MatchError`](util/error/index.md#matcherror)

  Create a new "unsupported overlapping search" error. This occurs when

  the caller requests an overlapping search while using an Aho-Corasick

  automaton with a match kind other than [`MatchKind::Standard`](#matchkindstandard).

  

  The match kind given should be the match kind of the automaton. It

  should never be `MatchKind::Standard`.

- <span id="matcherror-unsupported-empty"></span>`fn unsupported_empty() -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

  Create a new "unsupported empty pattern" error. This occurs when the

  caller requests a search for which matching an automaton that contains

  an empty pattern string is not supported.

#### Trait Implementations

##### `impl Any for MatchError`

- <span id="matcherror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchError`

- <span id="matcherror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchError`

- <span id="matcherror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchError`

- <span id="matcherror-clone"></span>`fn clone(&self) -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

##### `impl CloneToUninit for MatchError`

- <span id="matcherror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MatchError`

- <span id="matcherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchError`

- <span id="matcherror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl<T> From for MatchError`

- <span id="matcherror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchError`

- <span id="matcherror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MatchError`

- <span id="matcherror-partialeq-eq"></span>`fn eq(&self, other: &MatchError) -> bool` — [`MatchError`](util/error/index.md#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl ToOwned for MatchError`

- <span id="matcherror-toowned-type-owned"></span>`type Owned = T`

- <span id="matcherror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matcherror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for MatchError`

- <span id="matcherror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for MatchError`

- <span id="matcherror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matcherror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchError`

- <span id="matcherror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matcherror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternID`

```rust
struct PatternID(SmallIndex);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:713`](../../.source_1765521767/aho-corasick-1.1.4/src/util/primitives.rs#L713)*

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

- <span id="patternid-const-max"></span>`const MAX: PatternID`

- <span id="patternid-const-limit"></span>`const LIMIT: usize`

- <span id="patternid-const-zero"></span>`const ZERO: PatternID`

- <span id="patternid-const-size"></span>`const SIZE: usize`

- <span id="patternid-new"></span>`fn new(value: usize) -> Result<PatternID, PatternIDError>` — [`PatternID`](util/primitives/index.md#patternid), [`PatternIDError`](util/primitives/index.md#patterniderror)

  Create a new value that is represented by a "small index."

  

  If the given index exceeds the maximum allowed value, then this

  returns an error.

- <span id="patternid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

  Create a new value without checking whether the given argument

  exceeds the maximum.

  

  Using this routine with an invalid value will result in

  unspecified behavior, but *not* undefined behavior. In

  particular, an invalid ID value is likely to cause panics or

  possibly even silent logical errors.

  

  Callers must never rely on this type to be within a certain

  range for memory safety.

- <span id="patternid-from-u32-unchecked"></span>`const fn from_u32_unchecked(index: u32) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

  Create a new value from a `u32` without checking whether the

  given value exceeds the maximum.

  

  Using this routine with an invalid value will result in

  unspecified behavior, but *not* undefined behavior. In

  particular, an invalid ID value is likely to cause panics or

  possibly even silent logical errors.

  

  Callers must never rely on this type to be within a certain

  range for memory safety.

- <span id="patternid-must"></span>`fn must(value: usize) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

  Like `new`, but panics if the given value is not valid.

- <span id="patternid-as-usize"></span>`const fn as_usize(&self) -> usize`

  Return the internal value as a `usize`. This is guaranteed to

  never overflow `usize`.

- <span id="patternid-as-u64"></span>`const fn as_u64(&self) -> u64`

  Return the internal value as a `u64`. This is guaranteed to

  never overflow.

- <span id="patternid-as-u32"></span>`const fn as_u32(&self) -> u32`

  Return the internal value as a `u32`. This is guaranteed to

  never overflow `u32`.

- <span id="patternid-as-i32"></span>`const fn as_i32(&self) -> i32`

  Return the internal value as a `i32`. This is guaranteed to

  never overflow an `i32`.

- <span id="patternid-one-more"></span>`fn one_more(&self) -> usize`

  Returns one more than this value as a usize.

  

  Since values represented by a "small index" have constraints

  on their maximum value, adding `1` to it will always fit in a

  `usize`, `u32` and a `i32`.

- <span id="patternid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](util/primitives/index.md#patternid), [`PatternIDError`](util/primitives/index.md#patterniderror)

  Decode this value from the bytes given using the native endian

  byte order for the current target.

  

  If the decoded integer is not representable as a small index

  for the current target, then this returns an error.

- <span id="patternid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

  Decode this value from the bytes given using the native endian

  byte order for the current target.

  

  This is analogous to `new_unchecked` in that is does not check

  whether the decoded integer is representable as a small index.

- <span id="patternid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

  Return the underlying integer as raw bytes in native endian

  format.

- <span id="patternid-iter"></span>`fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](util/primitives/index.md#patterniditer)

  Returns an iterator over all values from 0 up to and not

  including the given length.

  

  If the given length exceeds this type's limit, then this

  panics.

#### Trait Implementations

##### `impl Any for PatternID`

- <span id="patternid-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternID`

- <span id="patternid-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternID`

- <span id="patternid-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PatternID`

- <span id="patternid-clone"></span>`fn clone(&self) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl CloneToUninit for PatternID`

- <span id="patternid-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- <span id="patternid-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- <span id="patternid-default"></span>`fn default() -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl Eq for PatternID`

##### `impl<T> From for PatternID`

- <span id="patternid-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for PatternID`

- <span id="patternid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: PatternID) -> &T` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl<T> IndexMut for [T]`

- <span id="t-indexmut-index-mut"></span>`fn index_mut(&mut self, index: PatternID) -> &mut T` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl<U> Into for PatternID`

- <span id="patternid-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Ord for PatternID`

- <span id="patternid-ord-cmp"></span>`fn cmp(&self, other: &PatternID) -> cmp::Ordering` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl PartialEq for PatternID`

- <span id="patternid-partialeq-eq"></span>`fn eq(&self, other: &PatternID) -> bool` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl PartialOrd for PatternID`

- <span id="patternid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &PatternID) -> option::Option<cmp::Ordering>` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl StructuralPartialEq for PatternID`

##### `impl ToOwned for PatternID`

- <span id="patternid-toowned-type-owned"></span>`type Owned = T`

- <span id="patternid-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patternid-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for PatternID`

- <span id="patternid-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patternid-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternID`

- <span id="patternid-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patternid-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `PatternIDError`

```rust
struct PatternIDError(SmallIndexError);
```

*Defined in [`aho-corasick-1.1.4/src/util/primitives.rs:736`](../../.source_1765521767/aho-corasick-1.1.4/src/util/primitives.rs#L736)*

This error occurs when an ID could not be constructed.

This occurs when given an integer exceeding the maximum allowed
value.

When the `std` feature is enabled, this implements the `Error`
trait.

#### Implementations

- <span id="patterniderror-attempted"></span>`fn attempted(&self) -> u64`

  Returns the value that could not be converted to an ID.

#### Trait Implementations

##### `impl Any for PatternIDError`

- <span id="patterniderror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for PatternIDError`

- <span id="patterniderror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for PatternIDError`

- <span id="patterniderror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for PatternIDError`

- <span id="patterniderror-clone"></span>`fn clone(&self) -> PatternIDError` — [`PatternIDError`](util/primitives/index.md#patterniderror)

##### `impl CloneToUninit for PatternIDError`

- <span id="patterniderror-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for PatternIDError`

- <span id="patterniderror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for PatternIDError`

- <span id="patterniderror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PatternIDError`

##### `impl Error for PatternIDError`

##### `impl<T> From for PatternIDError`

- <span id="patterniderror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for PatternIDError`

- <span id="patterniderror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for PatternIDError`

- <span id="patterniderror-partialeq-eq"></span>`fn eq(&self, other: &PatternIDError) -> bool` — [`PatternIDError`](util/primitives/index.md#patterniderror)

##### `impl StructuralPartialEq for PatternIDError`

##### `impl ToOwned for PatternIDError`

- <span id="patterniderror-toowned-type-owned"></span>`type Owned = T`

- <span id="patterniderror-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="patterniderror-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for PatternIDError`

- <span id="patterniderror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for PatternIDError`

- <span id="patterniderror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="patterniderror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for PatternIDError`

- <span id="patterniderror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="patterniderror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Input<'h>`

```rust
struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/search.rs:83-88`](../../.source_1765521767/aho-corasick-1.1.4/src/util/search.rs#L83-L88)*

The configuration and the haystack to use for an Aho-Corasick search.

When executing a search, there are a few parameters one might want to
configure:

* The haystack to search, provided to the `Input::new` constructor. This
is the only required parameter.
* The span _within_ the haystack to limit a search to. (The default
is the entire haystack.) This is configured via `Input::span` or
`Input::range`.
* Whether to run an unanchored (matches can occur anywhere after the
start of the search) or anchored (matches can only occur beginning at
the start of the search) search. Unanchored search is the default. This is
configured via `Input::anchored`.
* Whether to quit the search as soon as a match has been found, regardless
of the [`MatchKind`](util/search/index.md) that the searcher was built with. This is configured
via `Input::earliest`.

For most cases, the defaults for all optional parameters are appropriate.
The utility of this type is that it keeps the default or common case simple
while permitting tweaking parameters in more niche use cases while reusing
the same search APIs.

# Valid bounds and search termination

An `Input` permits setting the bounds of a search via either
`Input::span` or `Input::range`. The bounds set must be valid, or
else a panic will occur. Bounds are valid if and only if:

* The bounds represent a valid range into the input's haystack.
* **or** the end bound is a valid ending bound for the haystack *and*
the start bound is exactly one greater than the end bound.

In the latter case, `Input::is_done` will return true and indicates any
search receiving such an input should immediately return with no match.

Other than representing "search is complete," the `Input::span` and
`Input::range` APIs are never necessary. Instead, callers can slice the
haystack instead, e.g., with `&haystack[start..end]`. With that said, they
can be more convenient than slicing because the match positions reported
when using `Input::span` or `Input::range` are in terms of the original
haystack. If you instead use `&haystack[start..end]`, then you'll need to
add `start` to any match position returned in order for it to be a correct
index into `haystack`.

# Example: `&str` and `&[u8]` automatically convert to an `Input`

There is a `From<&T> for Input` implementation for all `T: AsRef<[u8]>`.
Additionally, the [`AhoCorasick`](crate::AhoCorasick) search APIs accept
a `Into<Input>`. These two things combined together mean you can provide
things like `&str` and `&[u8]` to search APIs when the defaults are
suitable, but also an `Input` when they're not. For example:

```rust
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

- <span id="input-new"></span>`fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h>` — [`Input`](util/search/index.md#input)

  Create a new search configuration for the given haystack.

- <span id="input-span"></span>`fn span<S: Into<Span>>(self, span: S) -> Input<'h>` — [`Input`](util/search/index.md#input)

  Set the span for this search.

  

  This routine is generic over how a span is provided. While

  a [`Span`](util/search/index.md) may be given directly, one may also provide a

  `std::ops::Range<usize>`. To provide anything supported by range

  syntax, use the `Input::range` method.

  

  The default span is the entire haystack.

  

  Note that `Input::range` overrides this method and vice versa.

  

  # Panics

  

  This panics if the given span does not correspond to valid bounds in

  the haystack or the termination of a search.

  

  # Example

  

  This example shows how the span of the search can impact whether a

  match is reported or not.

  

  ```rust

  use aho_corasick::{AhoCorasick, Input, MatchKind};

  

  let patterns = &["b", "abcd", "abc"];

  let haystack = "abcd";

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostFirst)

      .build(patterns)

      .unwrap();

  let input = Input::new(haystack).span(0..3);

  let mat = ac.try_find(input)?.expect("should have a match");

  // Without the span stopping the search early, 'abcd' would be reported

  // because it is the correct leftmost-first match.

  assert_eq!("abc", &haystack[mat.span()]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="input-range"></span>`fn range<R: RangeBounds<usize>>(self, range: R) -> Input<'h>` — [`Input`](util/search/index.md#input)

  Like `Input::span`, but accepts any range instead.

  

  The default range is the entire haystack.

  

  Note that `Input::span` overrides this method and vice versa.

  

  # Panics

  

  This routine will panic if the given range could not be converted

  to a valid [`Range`](../gimli/read/index.md). For example, this would panic when given

  `0..=usize::MAX` since it cannot be represented using a half-open

  interval in terms of `usize`.

  

  This routine also panics if the given range does not correspond to

  valid bounds in the haystack or the termination of a search.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let input = Input::new("foobar");

  assert_eq!(0..6, input.get_range());

  

  let input = Input::new("foobar").range(2..=4);

  assert_eq!(2..5, input.get_range());

  ```

- <span id="input-anchored"></span>`fn anchored(self, mode: Anchored) -> Input<'h>` — [`Anchored`](util/search/index.md#anchored), [`Input`](util/search/index.md#input)

  Sets the anchor mode of a search.

  

  When a search is anchored (via [`Anchored::Yes`](#anchoredyes)), a match must begin

  at the start of a search. When a search is not anchored (that's

  [`Anchored::No`](#anchoredno)), searchers will look for a match anywhere in the

  haystack.

  

  By default, the anchored mode is [`Anchored::No`](#anchoredno).

  

  # Support for anchored searches

  

  Anchored or unanchored searches might not always be available,

  depending on the type of searcher used and its configuration:

  

  * [`noncontiguous::NFA`](crate::nfa::noncontiguous::NFA) always

  supports both unanchored and anchored searches.

  * [`contiguous::NFA`](crate::nfa::contiguous::NFA) always supports both

  unanchored and anchored searches.

  * [`dfa::DFA`](crate::dfa::DFA) supports only unanchored

  searches by default.

  [`dfa::Builder::start_kind`](crate::dfa::Builder::start_kind) can

  be used to change the default to supporting both kinds of searches

  or even just anchored searches.

  * [`AhoCorasick`](crate::AhoCorasick) inherits the same setup as a

  `DFA`. Namely, it only supports unanchored searches by default, but

  [`AhoCorasickBuilder::start_kind`](crate::AhoCorasickBuilder::start_kind)

  can change this.

  

  If you try to execute a search using a `try_` ("fallible") method with

  an unsupported anchor mode, then an error will be returned. For calls

  to infallible search methods, a panic will result.

  

  # Example

  

  This demonstrates the differences between an anchored search and

  an unanchored search. Notice that we build our `AhoCorasick` searcher

  with [`StartKind::Both`](#startkindboth) so that it supports both unanchored and

  anchored searches simultaneously.

  

  ```rust

  use aho_corasick::{

      AhoCorasick, Anchored, Input, MatchKind, StartKind,

  };

  

  let patterns = &["bcd"];

  let haystack = "abcd";

  

  let ac = AhoCorasick::builder()

      .start_kind(StartKind::Both)

      .build(patterns)

      .unwrap();

  

  // Note that 'Anchored::No' is the default, so it doesn't need to

  // be explicitly specified here.

  let input = Input::new(haystack);

  let mat = ac.try_find(input)?.expect("should have a match");

  assert_eq!("bcd", &haystack[mat.span()]);

  

  // While 'bcd' occurs in the haystack, it does not begin where our

  // search begins, so no match is found.

  let input = Input::new(haystack).anchored(Anchored::Yes);

  assert_eq!(None, ac.try_find(input)?);

  

  // However, if we start our search where 'bcd' starts, then we will

  // find a match.

  let input = Input::new(haystack).range(1..).anchored(Anchored::Yes);

  let mat = ac.try_find(input)?.expect("should have a match");

  assert_eq!("bcd", &haystack[mat.span()]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="input-earliest"></span>`fn earliest(self, yes: bool) -> Input<'h>` — [`Input`](util/search/index.md#input)

  Whether to execute an "earliest" search or not.

  

  When running a non-overlapping search, an "earliest" search will

  return the match location as early as possible. For example, given

  the patterns `abc` and `b`, and a haystack of `abc`, a normal

  leftmost-first search will return `abc` as a match. But an "earliest"

  search will return as soon as it is known that a match occurs, which

  happens once `b` is seen.

  

  Note that when using [`MatchKind::Standard`](#matchkindstandard), the "earliest" option

  has no effect since standard semantics are already "earliest." Note

  also that this has no effect in overlapping searches, since overlapping

  searches also use standard semantics and report all possible matches.

  

  This is disabled by default.

  

  # Example

  

  This example shows the difference between "earliest" searching and

  normal leftmost searching.

  

  ```rust

  use aho_corasick::{AhoCorasick, Anchored, Input, MatchKind, StartKind};

  

  let patterns = &["abc", "b"];

  let haystack = "abc";

  

  let ac = AhoCorasick::builder()

      .match_kind(MatchKind::LeftmostFirst)

      .build(patterns)

      .unwrap();

  

  // The normal leftmost-first match.

  let input = Input::new(haystack);

  let mat = ac.try_find(input)?.expect("should have a match");

  assert_eq!("abc", &haystack[mat.span()]);

  

  // The "earliest" possible match, even if it isn't leftmost-first.

  let input = Input::new(haystack).earliest(true);

  let mat = ac.try_find(input)?.expect("should have a match");

  assert_eq!("b", &haystack[mat.span()]);

  

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

- <span id="input-set-span"></span>`fn set_span<S: Into<Span>>(&mut self, span: S)`

  Set the span for this search configuration.

  

  This is like the `Input::span` method, except this mutates the

  span in place.

  

  This routine is generic over how a span is provided. While

  a [`Span`](util/search/index.md) may be given directly, one may also provide a

  `std::ops::Range<usize>`.

  

  # Panics

  

  This panics if the given span does not correspond to valid bounds in

  the haystack or the termination of a search.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let mut input = Input::new("foobar");

  assert_eq!(0..6, input.get_range());

  input.set_span(2..4);

  assert_eq!(2..4, input.get_range());

  ```

- <span id="input-set-range"></span>`fn set_range<R: RangeBounds<usize>>(&mut self, range: R)`

  Set the span for this search configuration given any range.

  

  This is like the `Input::range` method, except this mutates the

  span in place.

  

  # Panics

  

  This routine will panic if the given range could not be converted

  to a valid [`Range`](../gimli/read/index.md). For example, this would panic when given

  `0..=usize::MAX` since it cannot be represented using a half-open

  interval in terms of `usize`.

  

  This routine also panics if the given range does not correspond to

  valid bounds in the haystack or the termination of a search.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let mut input = Input::new("foobar");

  assert_eq!(0..6, input.get_range());

  input.set_range(2..=4);

  assert_eq!(2..5, input.get_range());

  ```

- <span id="input-set-start"></span>`fn set_start(&mut self, start: usize)`

  Set the starting offset for the span for this search configuration.

  

  This is a convenience routine for only mutating the start of a span

  without having to set the entire span.

  

  # Panics

  

  This panics if the given span does not correspond to valid bounds in

  the haystack or the termination of a search.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let mut input = Input::new("foobar");

  assert_eq!(0..6, input.get_range());

  input.set_start(5);

  assert_eq!(5..6, input.get_range());

  ```

- <span id="input-set-end"></span>`fn set_end(&mut self, end: usize)`

  Set the ending offset for the span for this search configuration.

  

  This is a convenience routine for only mutating the end of a span

  without having to set the entire span.

  

  # Panics

  

  This panics if the given span does not correspond to valid bounds in

  the haystack or the termination of a search.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let mut input = Input::new("foobar");

  assert_eq!(0..6, input.get_range());

  input.set_end(5);

  assert_eq!(0..5, input.get_range());

  ```

- <span id="input-set-anchored"></span>`fn set_anchored(&mut self, mode: Anchored)` — [`Anchored`](util/search/index.md#anchored)

  Set the anchor mode of a search.

  

  This is like `Input::anchored`, except it mutates the search

  configuration in place.

  

  # Example

  

  ```rust

  use aho_corasick::{Anchored, Input};

  

  let mut input = Input::new("foobar");

  assert_eq!(Anchored::No, input.get_anchored());

  

  input.set_anchored(Anchored::Yes);

  assert_eq!(Anchored::Yes, input.get_anchored());

  ```

- <span id="input-set-earliest"></span>`fn set_earliest(&mut self, yes: bool)`

  Set whether the search should execute in "earliest" mode or not.

  

  This is like `Input::earliest`, except it mutates the search

  configuration in place.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let mut input = Input::new("foobar");

  assert!(!input.get_earliest());

  input.set_earliest(true);

  assert!(input.get_earliest());

  ```

- <span id="input-haystack"></span>`fn haystack(&self) -> &[u8]`

  Return a borrow of the underlying haystack as a slice of bytes.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let input = Input::new("foobar");

  assert_eq!(b"foobar", input.haystack());

  ```

- <span id="input-start"></span>`fn start(&self) -> usize`

  Return the start position of this search.

  

  This is a convenience routine for `search.get_span().start()`.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let input = Input::new("foobar");

  assert_eq!(0, input.start());

  

  let input = Input::new("foobar").span(2..4);

  assert_eq!(2, input.start());

  ```

- <span id="input-end"></span>`fn end(&self) -> usize`

  Return the end position of this search.

  

  This is a convenience routine for `search.get_span().end()`.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let input = Input::new("foobar");

  assert_eq!(6, input.end());

  

  let input = Input::new("foobar").span(2..4);

  assert_eq!(4, input.end());

  ```

- <span id="input-get-span"></span>`fn get_span(&self) -> Span` — [`Span`](util/search/index.md#span)

  Return the span for this search configuration.

  

  If one was not explicitly set, then the span corresponds to the entire

  range of the haystack.

  

  # Example

  

  ```rust

  use aho_corasick::{Input, Span};

  

  let input = Input::new("foobar");

  assert_eq!(Span { start: 0, end: 6 }, input.get_span());

  ```

- <span id="input-get-range"></span>`fn get_range(&self) -> Range<usize>`

  Return the span as a range for this search configuration.

  

  If one was not explicitly set, then the span corresponds to the entire

  range of the haystack.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let input = Input::new("foobar");

  assert_eq!(0..6, input.get_range());

  ```

- <span id="input-get-anchored"></span>`fn get_anchored(&self) -> Anchored` — [`Anchored`](util/search/index.md#anchored)

  Return the anchored mode for this search configuration.

  

  If no anchored mode was set, then it defaults to [`Anchored::No`](#anchoredno).

  

  # Example

  

  ```rust

  use aho_corasick::{Anchored, Input};

  

  let mut input = Input::new("foobar");

  assert_eq!(Anchored::No, input.get_anchored());

  

  input.set_anchored(Anchored::Yes);

  assert_eq!(Anchored::Yes, input.get_anchored());

  ```

- <span id="input-get-earliest"></span>`fn get_earliest(&self) -> bool`

  Return whether this search should execute in "earliest" mode.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let input = Input::new("foobar");

  assert!(!input.get_earliest());

  ```

- <span id="input-is-done"></span>`fn is_done(&self) -> bool`

  Return true if this input has been exhausted, which in turn means all

  subsequent searches will return no matches.

  

  This occurs precisely when the start position of this search is greater

  than the end position of the search.

  

  # Example

  

  ```rust

  use aho_corasick::Input;

  

  let mut input = Input::new("foobar");

  assert!(!input.is_done());

  input.set_start(6);

  assert!(!input.is_done());

  input.set_start(7);

  assert!(input.is_done());

  ```

#### Trait Implementations

##### `impl Any for Input<'h>`

- <span id="input-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Input<'h>`

- <span id="input-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Input<'h>`

- <span id="input-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Input<'h>`

- <span id="input-clone"></span>`fn clone(&self) -> Input<'h>` — [`Input`](util/search/index.md#input)

##### `impl CloneToUninit for Input<'h>`

- <span id="input-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Input<'h>`

- <span id="input-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Input<'h>`

- <span id="input-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Input<'h>`

- <span id="input-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Input<'h>`

- <span id="input-toowned-type-owned"></span>`type Owned = T`

- <span id="input-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="input-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Input<'h>`

- <span id="input-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="input-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Input<'h>`

- <span id="input-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="input-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Match`

```rust
struct Match {
    pattern: crate::util::primitives::PatternID,
    span: Span,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/search.rs:825-830`](../../.source_1765521767/aho-corasick-1.1.4/src/util/search.rs#L825-L830)*

A representation of a match reported by an Aho-Corasick searcher.

A match has two essential pieces of information: the [`PatternID`](util/primitives/index.md) that
matches, and the [`Span`](util/search/index.md) of the match in a haystack.

The pattern is identified by an ID, which corresponds to its position
(starting from `0`) relative to other patterns used to construct the
corresponding searcher. If only a single pattern is provided, then all
matches are guaranteed to have a pattern ID of `0`.

Every match reported by a searcher guarantees that its span has its start
offset as less than or equal to its end offset.

#### Fields

- **`pattern`**: `crate::util::primitives::PatternID`

  The pattern ID.

- **`span`**: `Span`

  The underlying match span.

#### Implementations

- <span id="match-new"></span>`fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match` — [`PatternID`](util/primitives/index.md#patternid), [`Match`](util/search/index.md#match)

  Create a new match from a pattern ID and a span.

  

  This constructor is generic over how a span is provided. While

  a [`Span`](util/search/index.md) may be given directly, one may also provide a

  `std::ops::Range<usize>`.

  

  # Panics

  

  This panics if `end < start`.

  

  # Example

  

  This shows how to create a match for the first pattern in an

  Aho-Corasick searcher using convenient range syntax.

  

  ```rust

  use aho_corasick::{Match, PatternID};

  

  let m = Match::new(PatternID::ZERO, 5..10);

  assert_eq!(0, m.pattern().as_usize());

  assert_eq!(5, m.start());

  assert_eq!(10, m.end());

  ```

- <span id="match-must"></span>`fn must<S: Into<Span>>(pattern: usize, span: S) -> Match` — [`Match`](util/search/index.md#match)

  Create a new match from a pattern ID and a byte offset span.

  

  This constructor is generic over how a span is provided. While

  a [`Span`](util/search/index.md) may be given directly, one may also provide a

  `std::ops::Range<usize>`.

  

  This is like `Match::new`, but accepts a `usize` instead of a

  [`PatternID`](util/primitives/index.md). This panics if the given `usize` is not representable

  as a `PatternID`.

  

  # Panics

  

  This panics if `end < start` or if `pattern > PatternID::MAX`.

  

  # Example

  

  This shows how to create a match for the third pattern in an

  Aho-Corasick searcher using convenient range syntax.

  

  ```rust

  use aho_corasick::Match;

  

  let m = Match::must(3, 5..10);

  assert_eq!(3, m.pattern().as_usize());

  assert_eq!(5, m.start());

  assert_eq!(10, m.end());

  ```

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

  Returns the ID of the pattern that matched.

  

  The ID of a pattern is derived from the position in which it was

  originally inserted into the corresponding searcher. The first pattern

  has identifier `0`, and each subsequent pattern is `1`, `2` and so on.

- <span id="match-start"></span>`fn start(&self) -> usize`

  The starting position of the match.

  

  This is a convenience routine for `Match::span().start`.

- <span id="match-end"></span>`fn end(&self) -> usize`

  The ending position of the match.

  

  This is a convenience routine for `Match::span().end`.

- <span id="match-range"></span>`fn range(&self) -> core::ops::Range<usize>`

  Returns the match span as a range.

  

  This is a convenience routine for `Match::span().range()`.

- <span id="match-span"></span>`fn span(&self) -> Span` — [`Span`](util/search/index.md#span)

  Returns the span for this match.

- <span id="match-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true when the span in this match is empty.

  

  An empty match can only be returned when empty pattern is in the

  Aho-Corasick searcher.

- <span id="match-len"></span>`fn len(&self) -> usize`

  Returns the length of this match.

  

  This returns `0` in precisely the cases that `is_empty` returns `true`.

- <span id="match-offset"></span>`fn offset(&self, offset: usize) -> Match` — [`Match`](util/search/index.md#match)

  Returns a new match with `offset` added to its span's `start` and `end`

  values.

#### Trait Implementations

##### `impl Any for Match`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](util/search/index.md#match)

##### `impl CloneToUninit for Match`

- <span id="match-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Match`

##### `impl<T> From for Match`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Match`

- <span id="match-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<U> Into for Match`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Match`

- <span id="match-partialeq-eq"></span>`fn eq(&self, other: &Match) -> bool` — [`Match`](util/search/index.md#match)

##### `impl StructuralPartialEq for Match`

##### `impl ToOwned for Match`

- <span id="match-toowned-type-owned"></span>`type Owned = T`

- <span id="match-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="match-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Match`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Span`

```rust
struct Span {
    pub start: usize,
    pub end: usize,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/search.rs:673-678`](../../.source_1765521767/aho-corasick-1.1.4/src/util/search.rs#L673-L678)*

A representation of a range in a haystack.

A span corresponds to the starting and ending _byte offsets_ of a
contiguous region of bytes. The starting offset is inclusive while the
ending offset is exclusive. That is, a span is a half-open interval.

A span is used to report the offsets of a match, but it is also used to
convey which region of a haystack should be searched via routines like
`Input::span`.

This is basically equivalent to a `std::ops::Range<usize>`, except this
type implements `Copy` which makes it more ergonomic to use in the context
of this crate. Indeed, `Span` exists only because `Range<usize>` does
not implement `Copy`. Like a range, this implements `Index` for `[u8]`
and `str`, and `IndexMut` for `[u8]`. For convenience, this also impls
`From<Range>`, which means things like `Span::from(5..10)` work.

There are no constraints on the values of a span. It is, for example, legal
to create a span where `start > end`.

#### Fields

- **`start`**: `usize`

  The start offset of the span, inclusive.

- **`end`**: `usize`

  The end offset of the span, exclusive.

#### Implementations

- <span id="span-range"></span>`fn range(&self) -> Range<usize>`

  Returns this span as a range.

- <span id="span-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true when this span is empty. That is, when `start >= end`.

- <span id="span-len"></span>`fn len(&self) -> usize`

  Returns the length of this span.

  

  This returns `0` in precisely the cases that `is_empty` returns `true`.

- <span id="span-contains"></span>`fn contains(&self, offset: usize) -> bool`

  Returns true when the given offset is contained within this span.

  

  Note that an empty span contains no offsets and will always return

  false.

- <span id="span-offset"></span>`fn offset(&self, offset: usize) -> Span` — [`Span`](util/search/index.md#span)

  Returns a new span with `offset` added to this span's `start` and `end`

  values.

#### Trait Implementations

##### `impl Any for Span`

- <span id="span-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Span`

- <span id="span-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Span`

- <span id="span-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](util/search/index.md#span)

##### `impl CloneToUninit for Span`

- <span id="span-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl<T> From for Span`

- <span id="span-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for [u8]`

- <span id="u8-index-type-output"></span>`type Output = [u8]`

- <span id="u8-index"></span>`fn index(&self, index: Span) -> &[u8]` — [`Span`](util/search/index.md#span)

##### `impl IndexMut for [u8]`

- <span id="u8-indexmut-index-mut"></span>`fn index_mut(&mut self, index: Span) -> &mut [u8]` — [`Span`](util/search/index.md#span)

##### `impl<U> Into for Span`

- <span id="span-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Span`

- <span id="span-partialeq-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](util/search/index.md#span)

##### `impl StructuralPartialEq for Span`

##### `impl ToOwned for Span`

- <span id="span-toowned-type-owned"></span>`type Owned = T`

- <span id="span-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="span-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Span`

- <span id="span-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="span-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Span`

- <span id="span-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="span-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `AhoCorasickKind`

```rust
enum AhoCorasickKind {
    NoncontiguousNFA,
    ContiguousNFA,
    DFA,
}
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2627-2634`](../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2627-L2634)*

The type of Aho-Corasick implementation to use in an [`AhoCorasick`](ahocorasick/index.md)
searcher.

This is principally used as an input to the
`AhoCorasickBuilder::start_kind` method. Its documentation goes into more
detail about each choice.

#### Variants

- **`NoncontiguousNFA`**

  Use a noncontiguous NFA.

- **`ContiguousNFA`**

  Use a contiguous NFA.

- **`DFA`**

  Use a DFA. Warning: DFAs typically use a large amount of memory.

#### Trait Implementations

##### `impl Any for AhoCorasickKind`

- <span id="ahocorasickkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for AhoCorasickKind`

- <span id="ahocorasickkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for AhoCorasickKind`

- <span id="ahocorasickkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for AhoCorasickKind`

- <span id="ahocorasickkind-clone"></span>`fn clone(&self) -> AhoCorasickKind` — [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind)

##### `impl CloneToUninit for AhoCorasickKind`

- <span id="ahocorasickkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for AhoCorasickKind`

##### `impl Debug for AhoCorasickKind`

- <span id="ahocorasickkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AhoCorasickKind`

##### `impl<T> From for AhoCorasickKind`

- <span id="ahocorasickkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for AhoCorasickKind`

- <span id="ahocorasickkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for AhoCorasickKind`

- <span id="ahocorasickkind-partialeq-eq"></span>`fn eq(&self, other: &AhoCorasickKind) -> bool` — [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind)

##### `impl StructuralPartialEq for AhoCorasickKind`

##### `impl ToOwned for AhoCorasickKind`

- <span id="ahocorasickkind-toowned-type-owned"></span>`type Owned = T`

- <span id="ahocorasickkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="ahocorasickkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for AhoCorasickKind`

- <span id="ahocorasickkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="ahocorasickkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for AhoCorasickKind`

- <span id="ahocorasickkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="ahocorasickkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`aho-corasick-1.1.4/src/util/error.rs:200-222`](../../.source_1765521767/aho-corasick-1.1.4/src/util/error.rs#L200-L222)*

The underlying kind of a [`MatchError`](util/error/index.md).

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

##### `impl Any for MatchErrorKind`

- <span id="matcherrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchErrorKind`

- <span id="matcherrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchErrorKind`

- <span id="matcherrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchErrorKind`

- <span id="matcherrorkind-clone"></span>`fn clone(&self) -> MatchErrorKind` — [`MatchErrorKind`](util/error/index.md#matcherrorkind)

##### `impl CloneToUninit for MatchErrorKind`

- <span id="matcherrorkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for MatchErrorKind`

- <span id="matcherrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl<T> From for MatchErrorKind`

- <span id="matcherrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MatchErrorKind`

- <span id="matcherrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for MatchErrorKind`

- <span id="matcherrorkind-partialeq-eq"></span>`fn eq(&self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](util/error/index.md#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

##### `impl ToOwned for MatchErrorKind`

- <span id="matcherrorkind-toowned-type-owned"></span>`type Owned = T`

- <span id="matcherrorkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="matcherrorkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for MatchErrorKind`

- <span id="matcherrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matcherrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MatchErrorKind`

- <span id="matcherrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matcherrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Anchored`

```rust
enum Anchored {
    No,
    Yes,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/search.rs:784-792`](../../.source_1765521767/aho-corasick-1.1.4/src/util/search.rs#L784-L792)*

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

- <span id="anchored-is-anchored"></span>`fn is_anchored(&self) -> bool`

  Returns true if and only if this anchor mode corresponds to an anchored

  search.

  

  # Example

  

  ```rust

  use aho_corasick::Anchored;

  

  assert!(!Anchored::No.is_anchored());

  assert!(Anchored::Yes.is_anchored());

  ```

#### Trait Implementations

##### `impl Any for Anchored`

- <span id="anchored-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Anchored`

- <span id="anchored-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Anchored`

- <span id="anchored-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Anchored`

- <span id="anchored-clone"></span>`fn clone(&self) -> Anchored` — [`Anchored`](util/search/index.md#anchored)

##### `impl CloneToUninit for Anchored`

- <span id="anchored-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Anchored`

##### `impl Debug for Anchored`

- <span id="anchored-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Anchored`

##### `impl<T> From for Anchored`

- <span id="anchored-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Anchored`

- <span id="anchored-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Anchored`

- <span id="anchored-partialeq-eq"></span>`fn eq(&self, other: &Anchored) -> bool` — [`Anchored`](util/search/index.md#anchored)

##### `impl StructuralPartialEq for Anchored`

##### `impl ToOwned for Anchored`

- <span id="anchored-toowned-type-owned"></span>`type Owned = T`

- <span id="anchored-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="anchored-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Anchored`

- <span id="anchored-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="anchored-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Anchored`

- <span id="anchored-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="anchored-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `MatchKind`

```rust
enum MatchKind {
    Standard,
    LeftmostFirst,
    LeftmostLongest,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/search.rs:1052-1074`](../../.source_1765521767/aho-corasick-1.1.4/src/util/search.rs#L1052-L1074)*

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

#### Implementations

- <span id="matchkind-is-standard"></span>`fn is_standard(&self) -> bool`

- <span id="matchkind-is-leftmost"></span>`fn is_leftmost(&self) -> bool`

- <span id="matchkind-is-leftmost-first"></span>`fn is_leftmost_first(&self) -> bool`

- <span id="matchkind-as-packed"></span>`fn as_packed(&self) -> Option<crate::packed::MatchKind>` — [`MatchKind`](packed/api/index.md#matchkind)

  Convert this match kind into a packed match kind. If this match kind

  corresponds to standard semantics, then this returns None, since

  packed searching does not support standard semantics.

#### Trait Implementations

##### `impl Any for MatchKind`

- <span id="matchkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MatchKind`

- <span id="matchkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MatchKind`

- <span id="matchkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for MatchKind`

- <span id="matchkind-clone"></span>`fn clone(&self) -> MatchKind` — [`MatchKind`](util/search/index.md#matchkind)

##### `impl CloneToUninit for MatchKind`

- <span id="matchkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- <span id="matchkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MatchKind`

- <span id="matchkind-default"></span>`fn default() -> MatchKind` — [`MatchKind`](util/search/index.md#matchkind)

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

- <span id="matchkind-partialeq-eq"></span>`fn eq(&self, other: &MatchKind) -> bool` — [`MatchKind`](util/search/index.md#matchkind)

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

### `StartKind`

```rust
enum StartKind {
    Both,
    Unanchored,
    Anchored,
}
```

*Defined in [`aho-corasick-1.1.4/src/util/search.rs:1133-1142`](../../.source_1765521767/aho-corasick-1.1.4/src/util/search.rs#L1133-L1142)*

The kind of anchored starting configurations to support in an Aho-Corasick
searcher.

Depending on which searcher is used internally by
[`AhoCorasick`](crate::AhoCorasick), supporting both unanchored
and anchored searches can be quite costly. For this reason,
[`AhoCorasickBuilder::start_kind`](crate::AhoCorasickBuilder::start_kind)
can be used to configure whether your searcher supports unanchored,
anchored or both kinds of searches.

This searcher configuration knob works in concert with the search time
configuration `Input::anchored`. Namely, if one requests an unsupported
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

##### `impl Any for StartKind`

- <span id="startkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for StartKind`

- <span id="startkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for StartKind`

- <span id="startkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for StartKind`

- <span id="startkind-clone"></span>`fn clone(&self) -> StartKind` — [`StartKind`](util/search/index.md#startkind)

##### `impl CloneToUninit for StartKind`

- <span id="startkind-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for StartKind`

##### `impl Debug for StartKind`

- <span id="startkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StartKind`

- <span id="startkind-default"></span>`fn default() -> StartKind` — [`StartKind`](util/search/index.md#startkind)

##### `impl Eq for StartKind`

##### `impl<T> From for StartKind`

- <span id="startkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for StartKind`

- <span id="startkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for StartKind`

- <span id="startkind-partialeq-eq"></span>`fn eq(&self, other: &StartKind) -> bool` — [`StartKind`](util/search/index.md#startkind)

##### `impl StructuralPartialEq for StartKind`

##### `impl ToOwned for StartKind`

- <span id="startkind-toowned-type-owned"></span>`type Owned = T`

- <span id="startkind-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="startkind-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for StartKind`

- <span id="startkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="startkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StartKind`

- <span id="startkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="startkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

