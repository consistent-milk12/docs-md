*[aho_corasick](../index.md) / [ahocorasick](index.md)*

---

# Module `ahocorasick`

## Contents

- [Structs](#structs)
  - [`AhoCorasick`](#ahocorasick)
  - [`FindIter`](#finditer)
  - [`FindOverlappingIter`](#findoverlappingiter)
  - [`StreamFindIter`](#streamfinditer)
  - [`AhoCorasickBuilder`](#ahocorasickbuilder)
- [Enums](#enums)
  - [`AhoCorasickKind`](#ahocorasickkind)
- [Traits](#traits)
  - [`AcAutomaton`](#acautomaton)
- [Functions](#functions)
  - [`enforce_anchored_consistency`](#enforce-anchored-consistency)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`AhoCorasick`](#ahocorasick) | struct | An automaton for searching multiple strings in linear time. |
| [`FindIter`](#finditer) | struct | An iterator of non-overlapping matches in a particular haystack. |
| [`FindOverlappingIter`](#findoverlappingiter) | struct | An iterator of overlapping matches in a particular haystack. |
| [`StreamFindIter`](#streamfinditer) | struct | An iterator that reports Aho-Corasick matches in a stream. |
| [`AhoCorasickBuilder`](#ahocorasickbuilder) | struct | A builder for configuring an Aho-Corasick automaton. |
| [`AhoCorasickKind`](#ahocorasickkind) | enum | The type of Aho-Corasick implementation to use in an [`AhoCorasick`] searcher. |
| [`AcAutomaton`](#acautomaton) | trait | A trait that effectively gives us practical dynamic dispatch over anything that impls `Automaton`, but without needing to add a bunch of bounds to the core `Automaton` trait. |
| [`enforce_anchored_consistency`](#enforce-anchored-consistency) | fn | Returns an error if the start state configuration does not support the desired search configuration. |

## Structs

### `AhoCorasick`

```rust
struct AhoCorasick {
    aut: alloc::sync::Arc<dyn AcAutomaton>,
    kind: AhoCorasickKind,
    start_kind: crate::util::search::StartKind,
}
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:177-214`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L177-L214)*

An automaton for searching multiple strings in linear time.

The `AhoCorasick` type supports a few basic ways of constructing an
automaton, with the default being `AhoCorasick::new`. However, there
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
to an [`Input`](../util/search/index.md). This includes `&[u8]`, `&str` and `Input` itself.

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

- <span id="ahocorasick-new"></span>`fn new<I, P>(patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](#ahocorasick), [`BuildError`](../util/error/index.md#builderror)

  Create a new Aho-Corasick automaton using the default configuration.

  

  The default configuration optimizes for less space usage, but at the

  expense of longer search times. To change the configuration, use

  [`AhoCorasickBuilder`](#ahocorasickbuilder).

  

  This uses the default [`MatchKind::Standard`](../index.md) match semantics, which

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

- <span id="ahocorasick-builder"></span>`fn builder() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

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

- <span id="ahocorasick-clone"></span>`fn clone(&self) -> AhoCorasick` — [`AhoCorasick`](#ahocorasick)

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

### `FindIter<'a, 'h>`

```rust
struct FindIter<'a, 'h>(automaton::FindIter<'a, 'h, alloc::sync::Arc<dyn AcAutomaton>>);
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2047`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2047)*

An iterator of non-overlapping matches in a particular haystack.

This iterator yields matches according to the [`MatchKind`](../util/search/index.md) used by this
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

- <span id="finditer-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

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

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2070-2072`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2070-L2072)*

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

- <span id="findoverlappingiter-iterator-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../util/search/index.md#match)

##### `impl<U> TryFrom for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="findoverlappingiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for FindOverlappingIter<'a, 'h>`

- <span id="findoverlappingiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="findoverlappingiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `StreamFindIter<'a, R>`

```rust
struct StreamFindIter<'a, R>(automaton::StreamFindIter<'a, alloc::sync::Arc<dyn AcAutomaton>, R>);
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2100-2102`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2100-L2102)*

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
[`AhoCorasick`](#ahocorasick) searcher.

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

- <span id="streamfinditer-iterator-next"></span>`fn next(&mut self) -> Option<Result<Match, std::io::Error>>` — [`Match`](../util/search/index.md#match)

##### `impl<U> TryFrom for StreamFindIter<'a, R>`

- <span id="streamfinditer-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="streamfinditer-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for StreamFindIter<'a, R>`

- <span id="streamfinditer-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="streamfinditer-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2135-2141`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2135-L2141)*

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

- <span id="ahocorasickbuilder-new"></span>`fn new() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

  Create a new builder for configuring an Aho-Corasick automaton.

  

  The builder provides a way to configure a number of things, including

  ASCII case insensitivity and what kind of match semantics are used.

- <span id="ahocorasickbuilder-build"></span>`fn build<I, P>(&self, patterns: I) -> Result<AhoCorasick, BuildError>` — [`AhoCorasick`](#ahocorasick), [`BuildError`](../util/error/index.md#builderror)

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

- <span id="ahocorasickbuilder-build-auto"></span>`fn build_auto(&self, nfa: noncontiguous::NFA) -> (Arc<dyn AcAutomaton>, AhoCorasickKind)` — [`NFA`](../nfa/noncontiguous/index.md#nfa), [`AcAutomaton`](#acautomaton), [`AhoCorasickKind`](#ahocorasickkind)

  Implements the automatic selection logic for the Aho-Corasick

  implementation to use. Since all Aho-Corasick automatons are built

  from a non-contiguous NFA, the caller is responsible for building

  that first.

- <span id="ahocorasickbuilder-match-kind"></span>`fn match_kind(&mut self, kind: MatchKind) -> &mut AhoCorasickBuilder` — [`MatchKind`](../util/search/index.md#matchkind), [`AhoCorasickBuilder`](#ahocorasickbuilder)

  Set the desired match semantics.

  

  The default is [`MatchKind::Standard`](../index.md), which corresponds to the match

  semantics supported by the standard textbook description of the

  Aho-Corasick algorithm. Namely, matches are reported as soon as they

  are found. Moreover, this is the only way to get overlapping matches

  or do stream searching.

  

  The other kinds of match semantics that are supported are

  [`MatchKind::LeftmostFirst`](../index.md) and [`MatchKind::LeftmostLongest`](../index.md). The

  former corresponds to the match you would get if you were to try to

  match each pattern at each position in the haystack in the same order

  that you give to the automaton. That is, it returns the leftmost match

  corresponding to the earliest pattern given to the automaton. The

  latter corresponds to finding the longest possible match among all

  leftmost matches.

  

  For more details on match semantics, see the [documentation for

  `MatchKind`](MatchKind).

  

  Note that setting this to [`MatchKind::LeftmostFirst`](../index.md) or

  [`MatchKind::LeftmostLongest`](../index.md) will cause some search routines on

  [`AhoCorasick`](#ahocorasick) to return an error (or panic if you're using the

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

- <span id="ahocorasickbuilder-start-kind"></span>`fn start_kind(&mut self, kind: StartKind) -> &mut AhoCorasickBuilder` — [`StartKind`](../util/search/index.md#startkind), [`AhoCorasickBuilder`](#ahocorasickbuilder)

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

  [`StartKind::Unanchored`](../index.md) is the default.

  

  Note that when this is set to [`StartKind::Unanchored`](../index.md), then

  running an anchored search will result in an error (or a panic

  if using the infallible APIs). Similarly, when this is set to

  [`StartKind::Anchored`](../index.md), then running an unanchored search will

  result in an error (or a panic if using the infallible APIs). When

  [`StartKind::Both`](../index.md) is used, then both unanchored and anchored searches

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

- <span id="ahocorasickbuilder-ascii-case-insensitive"></span>`fn ascii_case_insensitive(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

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

- <span id="ahocorasickbuilder-kind"></span>`fn kind(&mut self, kind: Option<AhoCorasickKind>) -> &mut AhoCorasickBuilder` — [`AhoCorasickKind`](#ahocorasickkind), [`AhoCorasickBuilder`](#ahocorasickbuilder)

  Choose the type of underlying automaton to use.

  

  Currently, there are four choices:

  

  * [`AhoCorasickKind::NoncontiguousNFA`](../index.md) instructs the searcher to

  use a [`noncontiguous::NFA`](../nfa/noncontiguous/index.md). A noncontiguous NFA is the fastest to

  be built, has moderate memory usage and is typically the slowest to

  execute a search.

  * [`AhoCorasickKind::ContiguousNFA`](../index.md) instructs the searcher to use a

  [`contiguous::NFA`](../nfa/contiguous/index.md). A contiguous NFA is a little slower to build than

  a noncontiguous NFA, has excellent memory usage and is typically a

  little slower than a DFA for a search.

  * [`AhoCorasickKind::DFA`](../index.md) instructs the searcher to use a

  [`dfa::DFA`](../dfa/index.md). A DFA is very slow to build, uses exorbitant amounts of

  memory, but will typically execute searches the fastest.

  * `None` (the default) instructs the searcher to choose the "best"

  Aho-Corasick implementation. This choice is typically based primarily

  on the number of patterns.

  

  Setting this configuration does not change the time complexity for

  constructing the Aho-Corasick automaton (which is `O(p)` where `p`

  is the total number of patterns being compiled). Setting this to

  [`AhoCorasickKind::DFA`](../index.md) does however reduce the time complexity of

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

- <span id="ahocorasickbuilder-prefilter"></span>`fn prefilter(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

  Enable heuristic prefilter optimizations.

  

  When enabled, searching will attempt to quickly skip to match

  candidates using specialized literal search routines. A prefilter

  cannot always be used, and is generally treated as a heuristic. It

  can be useful to disable this if the prefilter is observed to be

  sub-optimal for a particular workload.

  

  Currently, prefilters are typically only active when building searchers

  with a small (less than 100) number of patterns.

  

  This is enabled by default.

- <span id="ahocorasickbuilder-dense-depth"></span>`fn dense_depth(&mut self, depth: usize) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

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

- <span id="ahocorasickbuilder-byte-classes"></span>`fn byte_classes(&mut self, yes: bool) -> &mut AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

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

- <span id="ahocorasickbuilder-clone"></span>`fn clone(&self) -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

##### `impl CloneToUninit for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for AhoCorasickBuilder`

- <span id="ahocorasickbuilder-default"></span>`fn default() -> AhoCorasickBuilder` — [`AhoCorasickBuilder`](#ahocorasickbuilder)

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

## Enums

### `AhoCorasickKind`

```rust
enum AhoCorasickKind {
    NoncontiguousNFA,
    ContiguousNFA,
    DFA,
}
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2627-2634`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2627-L2634)*

The type of Aho-Corasick implementation to use in an [`AhoCorasick`](#ahocorasick)
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

- <span id="ahocorasickkind-clone"></span>`fn clone(&self) -> AhoCorasickKind` — [`AhoCorasickKind`](#ahocorasickkind)

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

- <span id="ahocorasickkind-partialeq-eq"></span>`fn eq(&self, other: &AhoCorasickKind) -> bool` — [`AhoCorasickKind`](#ahocorasickkind)

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

## Traits

### `AcAutomaton`

```rust
trait AcAutomaton: Automaton + Debug + Send + Sync + UnwindSafe + RefUnwindSafe + 'static { ... }
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2643-2646`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2643-L2646)*

A trait that effectively gives us practical dynamic dispatch over anything
that impls `Automaton`, but without needing to add a bunch of bounds to
the core `Automaton` trait. Basically, we provide all of the marker traits
that our automatons have, in addition to `Debug` impls and requiring that
there is no borrowed data. Without these, the main `AhoCorasick` type would
not be able to meaningfully impl `Debug` or the marker traits without also
requiring that all impls of `Automaton` do so, which would be not great.

#### Implementors

- `A`

## Functions

### `enforce_anchored_consistency`

```rust
fn enforce_anchored_consistency(have: crate::util::search::StartKind, want: crate::util::search::Anchored) -> Result<(), crate::util::error::MatchError>
```

*Defined in [`aho-corasick-1.1.4/src/ahocorasick.rs:2778-2789`](../../../.source_1765521767/aho-corasick-1.1.4/src/ahocorasick.rs#L2778-L2789)*

Returns an error if the start state configuration does not support the
desired search configuration. See the internal 'AhoCorasick::start_kind'
field docs for more details.

