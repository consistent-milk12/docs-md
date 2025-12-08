*[aho_corasick](../index.md) / [ahocorasick](index.md)*

---

# Module `ahocorasick`

## Structs

### `AhoCorasick`

```rust
struct AhoCorasick {
    aut: alloc::sync::Arc<dyn AcAutomaton>,
    kind: AhoCorasickKind,
    start_kind: crate::util::search::StartKind,
}
```

An automaton for searching multiple strings in linear time.

The `AhoCorasick` type supports a few basic ways of constructing an
automaton, with the default being `AhoCorasick::new`. However, there
are a fair number of configurable options that can be set by using
[`AhoCorasickBuilder`](../index.md) instead. Such options include, but are not limited
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

* 99MB for a [`noncontiguous::NFA`](../nfa/noncontiguous/index.md) in 240ms.
* 21MB for a [`contiguous::NFA`](../nfa/contiguous/index.md) in 275ms.
* 1.6GB for a [`dfa::DFA`](../dfa/index.md) in 1.88s.

(Note that the memory usage above reflects the size of each automaton and
not peak memory usage. For example, building a contiguous NFA requires
first building a noncontiguous NFA. Once the contiguous NFA is built, the
noncontiguous NFA is freed.)

This experiment very strongly argues that a contiguous NFA is often the
best balance in terms of resource usage. It takes a little longer to build,
but its memory usage is quite small. Its search speed (not listed) is
also often faster than a noncontiguous NFA, but a little slower than a
DFA. Indeed, when no specific [`AhoCorasickKind`](../index.md) is used (which is the
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
to an [`Input`](../index.md). This includes `&[u8]`, `&str` and `Input` itself.

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

* Running a search that requires [`MatchKind::Standard`](../index.md) semantics (such
as a stream or overlapping search) with an automaton that was built with
[`MatchKind::LeftmostFirst`](../index.md) or [`MatchKind::LeftmostLongest`](../index.md) semantics.
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

- `fn new<I, P>(patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](../index.md), [`BuildError`](../index.md)

- `fn builder() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

#### Trait Implementations

##### `impl Clone for AhoCorasick`

- `fn clone(self: &Self) -> AhoCorasick` — [`AhoCorasick`](../index.md)

##### `impl Debug for AhoCorasick`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `FindIter<'a, 'h>`

```rust
struct FindIter<'a, 'h>(automaton::FindIter<'a, 'h, alloc::sync::Arc<dyn AcAutomaton>>);
```

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`](../index.md) used by this
automaton.

This iterator is constructed via the `AhoCorasick::find_iter` and
`AhoCorasick::try_find_iter` methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl<'a, 'h> Debug for FindIter<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindIter<'a, 'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, 'h> Iterator for FindIter<'a, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>` — [`Match`](../index.md)

### `FindOverlappingIter<'a, 'h>`

```rust
struct FindOverlappingIter<'a, 'h>(automaton::FindOverlappingIter<'a, 'h, alloc::sync::Arc<dyn AcAutomaton>>);
```

An iterator of overlapping matches in a particular haystack.

This iterator will report all possible matches in a particular haystack,
even when the matches overlap.

This iterator is constructed via the `AhoCorasick::find_overlapping_iter`
and `AhoCorasick::try_find_overlapping_iter` methods.

The lifetime `'a` refers to the lifetime of the `AhoCorasick` automaton.

The lifetime `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl<'a, 'h> Debug for FindOverlappingIter<'a, 'h>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for FindOverlappingIter<'a, 'h>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, 'h> Iterator for FindOverlappingIter<'a, 'h>`

- `type Item = Match`

- `fn next(self: &mut Self) -> Option<Match>` — [`Match`](../index.md)

### `StreamFindIter<'a, R>`

```rust
struct StreamFindIter<'a, R>(automaton::StreamFindIter<'a, alloc::sync::Arc<dyn AcAutomaton>, R>);
```

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
[`AhoCorasick`](../index.md) searcher.

#### Trait Implementations

##### `impl<'a, R: $crate::fmt::Debug> Debug for StreamFindIter<'a, R>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<I> IntoIterator for StreamFindIter<'a, R>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a, R: std::io::Read> Iterator for StreamFindIter<'a, R>`

- `type Item = Result<Match, Error>`

- `fn next(self: &mut Self) -> Option<Result<Match, std::io::Error>>` — [`Match`](../index.md)

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

A builder for configuring an Aho-Corasick automaton.

# Quick advice

* Use `AhoCorasickBuilder::match_kind` to configure your searcher
with [`MatchKind::LeftmostFirst`](../index.md) if you want to match how backtracking
regex engines execute searches for `pat1|pat2|..|patN`. Use
[`MatchKind::LeftmostLongest`](../index.md) if you want to match how POSIX regex engines
do it.
* If you need an anchored search, use `AhoCorasickBuilder::start_kind` to
set the [`StartKind::Anchored`](../index.md) mode since [`StartKind::Unanchored`](../index.md) is the
default. Or just use [`StartKind::Both`](../index.md) to support both types of searches.
* You might want to use `AhoCorasickBuilder::kind` to set your searcher
to always use a [`AhoCorasickKind::DFA`](../index.md) if search speed is critical and
memory usage isn't a concern. Otherwise, not setting a kind will probably
make the right choice for you. Beware that if you use [`StartKind::Both`](../index.md)
to build a searcher that supports both unanchored and anchored searches
_and_ you set [`AhoCorasickKind::DFA`](../index.md), then the DFA will essentially be
duplicated to support both simultaneously. This results in very high memory
usage.
* For all other options, their defaults are almost certainly what you want.

#### Implementations

- `fn new() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

- `fn build<I, P>(self: &Self, patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](../index.md), [`BuildError`](../index.md)

- `fn build_auto(self: &Self, nfa: noncontiguous::NFA) -> (Arc<dyn AcAutomaton>, AhoCorasickKind)` — [`NFA`](../nfa/noncontiguous/index.md), [`AcAutomaton`](#acautomaton), [`AhoCorasickKind`](../index.md)

- `fn match_kind(self: &mut Self, kind: MatchKind) -> &mut AhoCorasickBuilder` — [`MatchKind`](../index.md), [`AhoCorasickBuilder`](../index.md)

- `fn start_kind(self: &mut Self, kind: StartKind) -> &mut AhoCorasickBuilder` — [`StartKind`](../index.md), [`AhoCorasickBuilder`](../index.md)

- `fn ascii_case_insensitive(self: &mut Self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

- `fn kind(self: &mut Self, kind: Option<AhoCorasickKind>) -> &mut AhoCorasickBuilder` — [`AhoCorasickKind`](../index.md), [`AhoCorasickBuilder`](../index.md)

- `fn prefilter(self: &mut Self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

- `fn dense_depth(self: &mut Self, depth: usize) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

- `fn byte_classes(self: &mut Self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

#### Trait Implementations

##### `impl Clone for AhoCorasickBuilder`

- `fn clone(self: &Self) -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

##### `impl Debug for AhoCorasickBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for AhoCorasickBuilder`

- `fn default() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](../index.md)

## Enums

### `AhoCorasickKind`

```rust
enum AhoCorasickKind {
    NoncontiguousNFA,
    ContiguousNFA,
    DFA,
}
```

The type of Aho-Corasick implementation to use in an [`AhoCorasick`](../index.md)
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

- `fn clone(self: &Self) -> AhoCorasickKind` — [`AhoCorasickKind`](../index.md)

##### `impl Copy for AhoCorasickKind`

##### `impl Debug for AhoCorasickKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for AhoCorasickKind`

##### `impl PartialEq for AhoCorasickKind`

- `fn eq(self: &Self, other: &AhoCorasickKind) -> bool` — [`AhoCorasickKind`](../index.md)

##### `impl StructuralPartialEq for AhoCorasickKind`

## Traits

### `AcAutomaton`

```rust
trait AcAutomaton: Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

A trait that effectively gives us practical dynamic dispatch over anything
that impls `Automaton`, but without needing to add a bunch of bounds to
the core `Automaton` trait. Basically, we provide all of the marker traits
that our automatons have, in addition to `Debug` impls and requiring that
there is no borrowed data. Without these, the main `AhoCorasick` type would
not be able to meaningfully impl `Debug` or the marker traits without also
requiring that all impls of `Automaton` do so, which would be not great.

## Functions

### `enforce_anchored_consistency`

```rust
fn enforce_anchored_consistency(have: crate::util::search::StartKind, want: crate::util::search::Anchored) -> Result<(), crate::util::error::MatchError>
```

Returns an error if the start state configuration does not support the
desired search configuration. See the internal 'AhoCorasick::start_kind'
field docs for more details.

