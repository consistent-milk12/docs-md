*[regex_automata](../../index.md) / [util](../index.md) / [iter](index.md)*

---

# Module `iter`

Generic helpers for iteration of matches from a regex engine in a haystack.

The principle type in this module is a [`Searcher`](#searcher). A `Searcher` provides
its own lower level iterator-like API in addition to methods for constructing
types that implement `Iterator`. The documentation for `Searcher` explains a
bit more about why these different APIs exist.

Currently, this module supports iteration over any regex engine that works
with the [`HalfMatch`](../../index.md), [`Match`](../../index.md) or [`Captures`](../captures/index.md) types.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Searcher`](#searcher) | struct | A searcher for creating iterators and performing lower level iteration. |
| [`TryHalfMatchesIter`](#tryhalfmatchesiter) | struct | An iterator over all non-overlapping half matches for a fallible search. |
| [`HalfMatchesIter`](#halfmatchesiter) | struct | An iterator over all non-overlapping half matches for an infallible search. |
| [`TryMatchesIter`](#trymatchesiter) | struct | An iterator over all non-overlapping matches for a fallible search. |
| [`MatchesIter`](#matchesiter) | struct | An iterator over all non-overlapping matches for an infallible search. |
| [`TryCapturesIter`](#trycapturesiter) | struct | An iterator over all non-overlapping captures for a fallible search. |
| [`CapturesIter`](#capturesiter) | struct | An iterator over all non-overlapping captures for an infallible search. |

## Structs

### `Searcher<'h>`

```rust
struct Searcher<'h> {
    input: crate::util::search::Input<'h>,
    last_match_end: Option<usize>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:147-156`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L147-L156)*

A searcher for creating iterators and performing lower level iteration.

This searcher encapsulates the logic required for finding all successive
non-overlapping matches in a haystack. In theory, iteration would look
something like this:

1. Setting the start position to `0`.
2. Execute a regex search. If no match, end iteration.
3. Report the match and set the start position to the end of the match.
4. Go back to (2).

And if this were indeed the case, it's likely that `Searcher` wouldn't
exist. Unfortunately, because a regex may match the empty string, the above
logic won't work for all possible regexes. Namely, if an empty match is
found, then step (3) would set the start position of the search to the
position it was at. Thus, iteration would never end.

Instead, a `Searcher` knows how to detect these cases and forcefully
advance iteration in the case of an empty match that overlaps with a
previous match.

If you know that your regex cannot match any empty string, then the simple
algorithm described above will work correctly.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

In particular, a `Searcher` is not itself an iterator. Instead, it provides
`advance` routines that permit moving the search along explicitly. It also
provides various routines, like `Searcher::into_matches_iter`, that
accept a closure (representing how a regex engine executes a search) and
returns a conventional iterator.

The lifetime parameters come from the [`Input`](../../index.md) type passed to
`Searcher::new`:

* `'h` is the lifetime of the underlying haystack.

# Searcher vs Iterator

Why does a search type with "advance" APIs exist at all when we also have
iterators? Unfortunately, the reasoning behind this split is a complex
combination of the following things:

1. While many of the regex engines expose their own iterators, it is also
nice to expose this lower level iteration helper because it permits callers
to provide their own `Input` configuration. Moreover, a `Searcher` can work
with _any_ regex engine instead of only the ones defined in this crate.
This way, everyone benefits from a shared iteration implementation.
2. There are many different regex engines that, while they have the same
match semantics, they have slightly different APIs. Iteration is just
complex enough to want to share code, and so we need a way of abstracting
over those different regex engines. While we could define a new trait that
describes any regex engine search API, it would wind up looking very close
to a closure. While there may still be reasons for the more generic trait
to exist, for now and for the purposes of iteration, we use a closure.
Closures also provide a lot of easy flexibility at the call site, in that
they permit the caller to borrow any kind of state they want for use during
each search call.
3. As a result of using closures, and because closures are anonymous types
that cannot be named, it is difficult to encapsulate them without both
costs to speed and added complexity to the public API. For example, in
defining an iterator type like
[`dfa::regex::FindMatches`](crate::dfa::regex::FindMatches),
if we use a closure internally, it's not possible to name this type in the
return type of the iterator constructor. Thus, the only way around it is
to erase the type by boxing it and turning it into a `Box<dyn FnMut ...>`.
This boxed closure is unlikely to be inlined _and_ it infects the public
API in subtle ways. Namely, unless you declare the closure as implementing
`Send` and `Sync`, then the resulting iterator type won't implement it
either. But there are practical issues with requiring the closure to
implement `Send` and `Sync` that result in other API complexities that
are beyond the scope of this already long exposition.
4. Some regex engines expose more complex match information than just
"which pattern matched" and "at what offsets." For example, the PikeVM
exposes match spans for each capturing group that participated in the
match. In such cases, it can be quite beneficial to reuse the capturing
group allocation on subsequent searches. A proper iterator doesn't permit
this API due to its interface, so it's useful to have something a bit lower
level that permits callers to amortize allocations while also reusing a
shared implementation of iteration. (See the documentation for
`Searcher::advance` for an example of using the "advance" API with the
PikeVM.)

What this boils down to is that there are "advance" APIs which require
handing a closure to it for every call, and there are also APIs to create
iterators from a closure. The former are useful for _implementing_
iterators or when you need more flexibility, while the latter are useful
for conveniently writing custom iterators on-the-fly.

# Example: iterating with captures

Several regex engines in this crate over convenient iterator APIs over
[`Captures`](../captures/index.md) values. To do so, this requires allocating a new `Captures`
value for each iteration step. This can perhaps be more costly than you
might want. Instead of implementing your own iterator to avoid that
cost (which can be a little subtle if you want to handle empty matches
correctly), you can use this `Searcher` to do it for you:

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    util::iter::Searcher,
    Input, Span,
};

let re = PikeVM::new("foo(?P<numbers>[0-9]+)")?;
let haystack = "foo1 foo12 foo123";

let mut caps = re.create_captures();
let mut cache = re.create_cache();
let mut matches = vec![];
let mut searcher = Searcher::new(Input::new(haystack));
while let Some(_) = searcher.advance(|input| {
    re.search(&mut cache, input, &mut caps);
    Ok(caps.get_match())
}) {
    // The unwrap is OK since 'numbers' matches if the pattern matches.
    matches.push(caps.get_group_by_name("numbers").unwrap());
}
assert_eq!(matches, vec![
    Span::from(3..4),
    Span::from(8..10),
    Span::from(14..17),
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Fields

- **`input`**: `crate::util::search::Input<'h>`

  The input parameters to give to each regex engine call.
  
  The start position of the search is mutated during iteration.

- **`last_match_end`**: `Option<usize>`

  Records the end offset of the most recent match. This is necessary to
  handle a corner case for preventing empty matches from overlapping with
  the ending bounds of a prior match.

#### Implementations

- <span id="searcher-new"></span>`fn new(input: Input<'h>) -> Searcher<'h>` — [`Input`](../../index.md#input), [`Searcher`](#searcher)

- <span id="searcher-input"></span>`fn input<'s>(self: &'s Self) -> &'s Input<'h>` — [`Input`](../../index.md#input)

- <span id="searcher-advance-half"></span>`fn advance_half<F>(&mut self, finder: F) -> Option<HalfMatch>` — [`HalfMatch`](../../index.md#halfmatch)

- <span id="searcher-advance"></span>`fn advance<F>(&mut self, finder: F) -> Option<Match>` — [`Match`](../../index.md#match)

- <span id="searcher-try-advance-half"></span>`fn try_advance_half<F>(&mut self, finder: F) -> Result<Option<HalfMatch>, MatchError>` — [`HalfMatch`](../../index.md#halfmatch), [`MatchError`](../../index.md#matcherror)

- <span id="searcher-try-advance"></span>`fn try_advance<F>(&mut self, finder: F) -> Result<Option<Match>, MatchError>` — [`Match`](../../index.md#match), [`MatchError`](../../index.md#matcherror)

- <span id="searcher-into-half-matches-iter"></span>`fn into_half_matches_iter<F>(self, finder: F) -> TryHalfMatchesIter<'h, F>` — [`TryHalfMatchesIter`](#tryhalfmatchesiter)

- <span id="searcher-into-matches-iter"></span>`fn into_matches_iter<F>(self, finder: F) -> TryMatchesIter<'h, F>` — [`TryMatchesIter`](#trymatchesiter)

- <span id="searcher-into-captures-iter"></span>`fn into_captures_iter<F>(self, caps: Captures, finder: F) -> TryCapturesIter<'h, F>` — [`Captures`](../captures/index.md#captures), [`TryCapturesIter`](#trycapturesiter)

- <span id="searcher-handle-overlapping-empty-half-match"></span>`fn handle_overlapping_empty_half_match<F>(&mut self, _: HalfMatch, finder: F) -> Result<Option<HalfMatch>, MatchError>` — [`HalfMatch`](../../index.md#halfmatch), [`MatchError`](../../index.md#matcherror)

- <span id="searcher-handle-overlapping-empty-match"></span>`fn handle_overlapping_empty_match<F>(&mut self, m: Match, finder: F) -> Result<Option<Match>, MatchError>` — [`Match`](../../index.md#match), [`MatchError`](../../index.md#matcherror)

#### Trait Implementations

##### `impl Clone for Searcher<'h>`

- <span id="searcher-clone"></span>`fn clone(&self) -> Searcher<'h>` — [`Searcher`](#searcher)

##### `impl Debug for Searcher<'h>`

- <span id="searcher-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TryHalfMatchesIter<'h, F>`

```rust
struct TryHalfMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:699-702`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L699-L702)*

An iterator over all non-overlapping half matches for a fallible search.

The iterator yields a `Result<HalfMatch, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_half_matches_iter`.

#### Implementations

- <span id="tryhalfmatchesiter-infallible"></span>`fn infallible(self) -> HalfMatchesIter<'h, F>` — [`HalfMatchesIter`](#halfmatchesiter)

- <span id="tryhalfmatchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

#### Trait Implementations

##### `impl<'h, F> Debug for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<I> IntoIterator for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tryhalfmatchesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="tryhalfmatchesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'h, F> Iterator for TryHalfMatchesIter<'h, F>`

- <span id="tryhalfmatchesiter-type-item"></span>`type Item = Result<HalfMatch, MatchError>`

- <span id="tryhalfmatchesiter-next"></span>`fn next(&mut self) -> Option<Result<HalfMatch, MatchError>>` — [`HalfMatch`](../../index.md#halfmatch), [`MatchError`](../../index.md#matcherror)

### `HalfMatchesIter<'h, F>`

```rust
struct HalfMatchesIter<'h, F>(TryHalfMatchesIter<'h, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:765`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L765)*

An iterator over all non-overlapping half matches for an infallible search.

The iterator yields a [`HalfMatch`](../../index.md) value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_half_matches_iter` and
then calling `TryHalfMatchesIter::infallible`.

#### Implementations

- <span id="halfmatchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

#### Trait Implementations

##### `impl<'h, F: fmt::Debug> Debug for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="halfmatchesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="halfmatchesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'h, F> Iterator for HalfMatchesIter<'h, F>`

- <span id="halfmatchesiter-type-item"></span>`type Item = HalfMatch`

- <span id="halfmatchesiter-next"></span>`fn next(&mut self) -> Option<HalfMatch>` — [`HalfMatch`](../../index.md#halfmatch)

### `TryMatchesIter<'h, F>`

```rust
struct TryMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:814-817`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L814-L817)*

An iterator over all non-overlapping matches for a fallible search.

The iterator yields a `Result<Match, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_matches_iter`.

#### Implementations

- <span id="trymatchesiter-infallible"></span>`fn infallible(self) -> MatchesIter<'h, F>` — [`MatchesIter`](#matchesiter)

- <span id="trymatchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

#### Trait Implementations

##### `impl<'h, F> Debug for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<I> IntoIterator for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="trymatchesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="trymatchesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'h, F> Iterator for TryMatchesIter<'h, F>`

- <span id="trymatchesiter-type-item"></span>`type Item = Result<Match, MatchError>`

- <span id="trymatchesiter-next"></span>`fn next(&mut self) -> Option<Result<Match, MatchError>>` — [`Match`](../../index.md#match), [`MatchError`](../../index.md#matcherror)

### `MatchesIter<'h, F>`

```rust
struct MatchesIter<'h, F>(TryMatchesIter<'h, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:879`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L879)*

An iterator over all non-overlapping matches for an infallible search.

The iterator yields a [`Match`](../../index.md) value until no more matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_matches_iter` and
then calling `TryMatchesIter::infallible`.

#### Implementations

- <span id="matchesiter-input"></span>`fn input<'i>(self: &'i Self) -> &'i Input<'h>` — [`Input`](../../index.md#input)

#### Trait Implementations

##### `impl<'h, F: fmt::Debug> Debug for MatchesIter<'h, F>`

- <span id="matchesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for MatchesIter<'h, F>`

- <span id="matchesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matchesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="matchesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'h, F> Iterator for MatchesIter<'h, F>`

- <span id="matchesiter-type-item"></span>`type Item = Match`

- <span id="matchesiter-next"></span>`fn next(&mut self) -> Option<Match>` — [`Match`](../../index.md#match)

### `TryCapturesIter<'h, F>`

```rust
struct TryCapturesIter<'h, F> {
    it: Searcher<'h>,
    caps: crate::util::captures::Captures,
    finder: F,
}
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:929-933`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L929-L933)*

An iterator over all non-overlapping captures for a fallible search.

The iterator yields a `Result<Captures, MatchError>` value until no more
matches could be found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_captures_iter`.

#### Implementations

- <span id="trycapturesiter-infallible"></span>`fn infallible(self) -> CapturesIter<'h, F>` — [`CapturesIter`](#capturesiter)

#### Trait Implementations

##### `impl<'h, F> Debug for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<I> IntoIterator for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="trycapturesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="trycapturesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'h, F> Iterator for TryCapturesIter<'h, F>`

- <span id="trycapturesiter-type-item"></span>`type Item = Result<Captures, MatchError>`

- <span id="trycapturesiter-next"></span>`fn next(&mut self) -> Option<Result<Captures, MatchError>>` — [`Captures`](../captures/index.md#captures), [`MatchError`](../../index.md#matcherror)

### `CapturesIter<'h, F>`

```rust
struct CapturesIter<'h, F>(TryCapturesIter<'h, F>);
```

*Defined in [`regex-automata-0.4.13/src/util/iter.rs:1003`](../../../../.source_1765210505/regex-automata-0.4.13/src/util/iter.rs#L1003)*

An iterator over all non-overlapping captures for an infallible search.

The iterator yields a [`Captures`](../captures/index.md) value until no more matches could be
found.

The type parameters are as follows:

* `F` represents the type of a closure that executes the search.

The lifetime parameters come from the [`Input`](../../index.md) type:

* `'h` is the lifetime of the underlying haystack.

When possible, prefer the iterators defined on the regex engine you're
using. This tries to abstract over the regex engine and is thus a bit more
unwieldy to use.

This iterator is created by `Searcher::into_captures_iter` and then
calling `TryCapturesIter::infallible`.

#### Trait Implementations

##### `impl<'h, F: fmt::Debug> Debug for CapturesIter<'h, F>`

- <span id="capturesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<I> IntoIterator for CapturesIter<'h, F>`

- <span id="capturesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="capturesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<'h, F> Iterator for CapturesIter<'h, F>`

- <span id="capturesiter-type-item"></span>`type Item = Captures`

- <span id="capturesiter-next"></span>`fn next(&mut self) -> Option<Captures>` — [`Captures`](../captures/index.md#captures)

