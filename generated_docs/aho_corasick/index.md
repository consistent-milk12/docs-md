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

##### `impl<R: fmt::Debug> Debug for StreamFindIter<'a, R>`

- <span id="streamfinditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for StreamFindIter<'a, R>`

- <span id="streamfinditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="streamfinditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="streamfinditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: std::io::Read> Iterator for StreamFindIter<'a, R>`

- <span id="streamfinditer-iterator-type-item"></span>`type Item = Result<Match, Error>`

- <span id="streamfinditer-next"></span>`fn next(&mut self) -> Option<Result<Match, std::io::Error>>` — [`Match`](util/search/index.md#match)

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

- <span id="ahocorasick-builder"></span>`fn builder() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

#### Trait Implementations

##### `impl Clone for AhoCorasick`

- <span id="ahocorasick-clone"></span>`fn clone(&self) -> AhoCorasick` — [`AhoCorasick`](ahocorasick/index.md#ahocorasick)

##### `impl Debug for AhoCorasick`

- <span id="ahocorasick-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

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

- <span id="ahocorasickbuilder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](ahocorasick/index.md#ahocorasick), [`BuildError`](util/error/index.md#builderror)

- <span id="ahocorasickbuilder-build-auto"></span>`fn build_auto(&self, nfa: noncontiguous::NFA) -> (Arc<dyn AcAutomaton>, AhoCorasickKind)` — [`NFA`](nfa/noncontiguous/index.md#nfa), [`AcAutomaton`](ahocorasick/index.md#acautomaton), [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind)

- <span id="ahocorasickbuilder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut AhoCorasickBuilder` — [`MatchKind`](util/search/index.md#matchkind), [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

- <span id="ahocorasickbuilder-start-kind"></span>`fn start_kind(&mut self, kind: StartKind) -> &mut AhoCorasickBuilder` — [`StartKind`](util/search/index.md#startkind), [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

- <span id="ahocorasickbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

- <span id="ahocorasickbuilder-kind"></span>`fn kind(&mut self, kind: Option<AhoCorasickKind>) -> &mut AhoCorasickBuilder` — [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind), [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

- <span id="ahocorasickbuilder-prefilter"></span>`fn prefilter(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

- <span id="ahocorasickbuilder-dense-depth"></span>`fn dense_depth(&mut self, depth: usize) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

- <span id="ahocorasickbuilder-byte-classes"></span>`fn byte_classes(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

#### Trait Implementations

##### `impl Clone for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-clone"></span>`fn clone(&self) -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

##### `impl Debug for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-default"></span>`fn default() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](ahocorasick/index.md#ahocorasickbuilder)

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

##### `impl Debug for FindIter<'a, 'h>`

- <span id="finditer-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindIter<'a, 'h>`

- <span id="finditer-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="finditer-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="finditer-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindIter<'a, 'h>`

- <span id="finditer-iterator-type-item"></span>`type Item = Match`

- <span id="finditer-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](util/search/index.md#match)

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

##### `impl Debug for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="findoverlappingiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="findoverlappingiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-iterator-type-item"></span>`type Item = Match`

- <span id="findoverlappingiter-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](util/search/index.md#match)

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

##### `impl Clone for BuildError`

- <span id="builderror-clone"></span>`fn clone(&self) -> BuildError` — [`BuildError`](util/error/index.md#builderror)

##### `impl Debug for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for BuildError`

- <span id="builderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for BuildError`

##### `impl ToString for BuildError`

- <span id="builderror-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="matcherror-kind"></span>`fn kind(&self) -> &MatchErrorKind` — [`MatchErrorKind`](util/error/index.md#matcherrorkind)

- <span id="matcherror-invalid-input-anchored"></span>`fn invalid_input_anchored() -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

- <span id="matcherror-invalid-input-unanchored"></span>`fn invalid_input_unanchored() -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

- <span id="matcherror-unsupported-stream"></span>`fn unsupported_stream(got: MatchKind) -> MatchError` — [`MatchKind`](util/search/index.md#matchkind), [`MatchError`](util/error/index.md#matcherror)

- <span id="matcherror-unsupported-overlapping"></span>`fn unsupported_overlapping(got: MatchKind) -> MatchError` — [`MatchKind`](util/search/index.md#matchkind), [`MatchError`](util/error/index.md#matcherror)

- <span id="matcherror-unsupported-empty"></span>`fn unsupported_empty() -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

#### Trait Implementations

##### `impl Clone for MatchError`

- <span id="matcherror-clone"></span>`fn clone(&self) -> MatchError` — [`MatchError`](util/error/index.md#matcherror)

##### `impl Debug for MatchError`

- <span id="matcherror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchError`

- <span id="matcherror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl PartialEq for MatchError`

- <span id="matcherror-eq"></span>`fn eq(&self, other: &MatchError) -> bool` — [`MatchError`](util/error/index.md#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl ToString for MatchError`

- <span id="matcherror-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="patternid-new-unchecked"></span>`const fn new_unchecked(value: usize) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

- <span id="patternid-from-u32-unchecked"></span>`const fn from_u32_unchecked(index: u32) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

- <span id="patternid-must"></span>`fn must(value: usize) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

- <span id="patternid-as-usize"></span>`const fn as_usize(&self) -> usize`

- <span id="patternid-as-u64"></span>`const fn as_u64(&self) -> u64`

- <span id="patternid-as-u32"></span>`const fn as_u32(&self) -> u32`

- <span id="patternid-as-i32"></span>`const fn as_i32(&self) -> i32`

- <span id="patternid-one-more"></span>`fn one_more(&self) -> usize`

- <span id="patternid-from-ne-bytes"></span>`fn from_ne_bytes(bytes: [u8; 4]) -> Result<PatternID, PatternIDError>` — [`PatternID`](util/primitives/index.md#patternid), [`PatternIDError`](util/primitives/index.md#patterniderror)

- <span id="patternid-from-ne-bytes-unchecked"></span>`fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

- <span id="patternid-to-ne-bytes"></span>`fn to_ne_bytes(&self) -> [u8; 4]`

- <span id="patternid-iter"></span>`fn iter(len: usize) -> PatternIDIter` — [`PatternIDIter`](util/primitives/index.md#patterniditer)

#### Trait Implementations

##### `impl Clone for PatternID`

- <span id="patternid-clone"></span>`fn clone(&self) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl Copy for PatternID`

##### `impl Debug for PatternID`

- <span id="patternid-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for PatternID`

- <span id="patternid-default"></span>`fn default() -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl Eq for PatternID`

##### `impl Hash for PatternID`

- <span id="patternid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T> Index for [T]`

- <span id="t-index-type-output"></span>`type Output = T`

- <span id="t-index"></span>`fn index(&self, index: PatternID) -> &T` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl<T> IndexMut for [T]`

- <span id="t-index-mut"></span>`fn index_mut(&mut self, index: PatternID) -> &mut T` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl Ord for PatternID`

- <span id="patternid-cmp"></span>`fn cmp(&self, other: &PatternID) -> cmp::Ordering` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl PartialEq for PatternID`

- <span id="patternid-eq"></span>`fn eq(&self, other: &PatternID) -> bool` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl PartialOrd for PatternID`

- <span id="patternid-partial-cmp"></span>`fn partial_cmp(&self, other: &PatternID) -> option::Option<cmp::Ordering>` — [`PatternID`](util/primitives/index.md#patternid)

##### `impl StructuralPartialEq for PatternID`

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

#### Trait Implementations

##### `impl Clone for PatternIDError`

- <span id="patterniderror-clone"></span>`fn clone(&self) -> PatternIDError` — [`PatternIDError`](util/primitives/index.md#patterniderror)

##### `impl Debug for PatternIDError`

- <span id="patterniderror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for PatternIDError`

- <span id="patterniderror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for PatternIDError`

##### `impl Error for PatternIDError`

##### `impl PartialEq for PatternIDError`

- <span id="patterniderror-eq"></span>`fn eq(&self, other: &PatternIDError) -> bool` — [`PatternIDError`](util/primitives/index.md#patterniderror)

##### `impl StructuralPartialEq for PatternIDError`

##### `impl ToString for PatternIDError`

- <span id="patterniderror-to-string"></span>`fn to_string(&self) -> String`

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

- <span id="input-span"></span>`fn span<S: Into<Span>>(self, span: S) -> Input<'h>` — [`Input`](util/search/index.md#input)

- <span id="input-range"></span>`fn range<R: RangeBounds<usize>>(self, range: R) -> Input<'h>` — [`Input`](util/search/index.md#input)

- <span id="input-anchored"></span>`fn anchored(self, mode: Anchored) -> Input<'h>` — [`Anchored`](util/search/index.md#anchored), [`Input`](util/search/index.md#input)

- <span id="input-earliest"></span>`fn earliest(self, yes: bool) -> Input<'h>` — [`Input`](util/search/index.md#input)

- <span id="input-set-span"></span>`fn set_span<S: Into<Span>>(&mut self, span: S)`

- <span id="input-set-range"></span>`fn set_range<R: RangeBounds<usize>>(&mut self, range: R)`

- <span id="input-set-start"></span>`fn set_start(&mut self, start: usize)`

- <span id="input-set-end"></span>`fn set_end(&mut self, end: usize)`

- <span id="input-set-anchored"></span>`fn set_anchored(&mut self, mode: Anchored)` — [`Anchored`](util/search/index.md#anchored)

- <span id="input-set-earliest"></span>`fn set_earliest(&mut self, yes: bool)`

- <span id="input-haystack"></span>`fn haystack(&self) -> &[u8]`

- <span id="input-start"></span>`fn start(&self) -> usize`

- <span id="input-end"></span>`fn end(&self) -> usize`

- <span id="input-get-span"></span>`fn get_span(&self) -> Span` — [`Span`](util/search/index.md#span)

- <span id="input-get-range"></span>`fn get_range(&self) -> Range<usize>`

- <span id="input-get-anchored"></span>`fn get_anchored(&self) -> Anchored` — [`Anchored`](util/search/index.md#anchored)

- <span id="input-get-earliest"></span>`fn get_earliest(&self) -> bool`

- <span id="input-is-done"></span>`fn is_done(&self) -> bool`

#### Trait Implementations

##### `impl Clone for Input<'h>`

- <span id="input-clone"></span>`fn clone(&self) -> Input<'h>` — [`Input`](util/search/index.md#input)

##### `impl Debug for Input<'h>`

- <span id="input-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

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

- <span id="match-must"></span>`fn must<S: Into<Span>>(pattern: usize, span: S) -> Match` — [`Match`](util/search/index.md#match)

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](util/primitives/index.md#patternid)

- <span id="match-start"></span>`fn start(&self) -> usize`

- <span id="match-end"></span>`fn end(&self) -> usize`

- <span id="match-range"></span>`fn range(&self) -> core::ops::Range<usize>`

- <span id="match-span"></span>`fn span(&self) -> Span` — [`Span`](util/search/index.md#span)

- <span id="match-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="match-len"></span>`fn len(&self) -> usize`

- <span id="match-offset"></span>`fn offset(&self, offset: usize) -> Match` — [`Match`](util/search/index.md#match)

#### Trait Implementations

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](util/search/index.md#match)

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Match`

##### `impl Hash for Match`

- <span id="match-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Match`

- <span id="match-eq"></span>`fn eq(&self, other: &Match) -> bool` — [`Match`](util/search/index.md#match)

##### `impl StructuralPartialEq for Match`

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

- <span id="span-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="span-len"></span>`fn len(&self) -> usize`

- <span id="span-contains"></span>`fn contains(&self, offset: usize) -> bool`

- <span id="span-offset"></span>`fn offset(&self, offset: usize) -> Span` — [`Span`](util/search/index.md#span)

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](util/search/index.md#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for [u8]`

- <span id="u8-index-type-output"></span>`type Output = [u8]`

- <span id="u8-index"></span>`fn index(&self, index: Span) -> &[u8]` — [`Span`](util/search/index.md#span)

##### `impl IndexMut for [u8]`

- <span id="u8-index-mut"></span>`fn index_mut(&mut self, index: Span) -> &mut [u8]` — [`Span`](util/search/index.md#span)

##### `impl PartialEq for Span`

- <span id="span-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](util/search/index.md#span)

##### `impl StructuralPartialEq for Span`

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

##### `impl Clone for AhoCorasickKind`

- <span id="ahocorasickkind-clone"></span>`fn clone(&self) -> AhoCorasickKind` — [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind)

##### `impl Copy for AhoCorasickKind`

##### `impl Debug for AhoCorasickKind`

- <span id="ahocorasickkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AhoCorasickKind`

##### `impl PartialEq for AhoCorasickKind`

- <span id="ahocorasickkind-eq"></span>`fn eq(&self, other: &AhoCorasickKind) -> bool` — [`AhoCorasickKind`](ahocorasick/index.md#ahocorasickkind)

##### `impl StructuralPartialEq for AhoCorasickKind`

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

##### `impl Clone for MatchErrorKind`

- <span id="matcherrorkind-clone"></span>`fn clone(&self) -> MatchErrorKind` — [`MatchErrorKind`](util/error/index.md#matcherrorkind)

##### `impl Debug for MatchErrorKind`

- <span id="matcherrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl PartialEq for MatchErrorKind`

- <span id="matcherrorkind-eq"></span>`fn eq(&self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](util/error/index.md#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

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

#### Trait Implementations

##### `impl Clone for Anchored`

- <span id="anchored-clone"></span>`fn clone(&self) -> Anchored` — [`Anchored`](util/search/index.md#anchored)

##### `impl Copy for Anchored`

##### `impl Debug for Anchored`

- <span id="anchored-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Anchored`

##### `impl PartialEq for Anchored`

- <span id="anchored-eq"></span>`fn eq(&self, other: &Anchored) -> bool` — [`Anchored`](util/search/index.md#anchored)

##### `impl StructuralPartialEq for Anchored`

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

#### Trait Implementations

##### `impl Clone for MatchKind`

- <span id="matchkind-clone"></span>`fn clone(&self) -> MatchKind` — [`MatchKind`](util/search/index.md#matchkind)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- <span id="matchkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MatchKind`

- <span id="matchkind-default"></span>`fn default() -> MatchKind` — [`MatchKind`](util/search/index.md#matchkind)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- <span id="matchkind-eq"></span>`fn eq(&self, other: &MatchKind) -> bool` — [`MatchKind`](util/search/index.md#matchkind)

##### `impl StructuralPartialEq for MatchKind`

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

##### `impl Clone for StartKind`

- <span id="startkind-clone"></span>`fn clone(&self) -> StartKind` — [`StartKind`](util/search/index.md#startkind)

##### `impl Copy for StartKind`

##### `impl Debug for StartKind`

- <span id="startkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StartKind`

- <span id="startkind-default"></span>`fn default() -> StartKind` — [`StartKind`](util/search/index.md#startkind)

##### `impl Eq for StartKind`

##### `impl PartialEq for StartKind`

- <span id="startkind-eq"></span>`fn eq(&self, other: &StartKind) -> bool` — [`StartKind`](util/search/index.md#startkind)

##### `impl StructuralPartialEq for StartKind`

