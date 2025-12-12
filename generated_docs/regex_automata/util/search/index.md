*[regex_automata](../../index.md) / [util](../index.md) / [search](index.md)*

---

# Module `search`

Types and routines that support the search APIs of most regex engines.

This sub-module isn't exposed directly, but rather, its contents are exported
at the crate root due to the universality of most of the types and routines in
this module.

## Contents

- [Structs](#structs)
  - [`Input`](#input)
  - [`Span`](#span)
  - [`HalfMatch`](#halfmatch)
  - [`Match`](#match)
  - [`PatternSet`](#patternset)
  - [`PatternSetInsertError`](#patternsetinserterror)
  - [`PatternSetIter`](#patternsetiter)
  - [`MatchError`](#matcherror)
- [Enums](#enums)
  - [`Anchored`](#anchored)
  - [`MatchKind`](#matchkind)
  - [`MatchErrorKind`](#matcherrorkind)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Input`](#input) | struct | The parameters for a regex search including the haystack to search. |
| [`Span`](#span) | struct | A representation of a span reported by a regex engine. |
| [`HalfMatch`](#halfmatch) | struct | A representation of "half" of a match reported by a DFA. |
| [`Match`](#match) | struct | A representation of a match reported by a regex engine. |
| [`PatternSet`](#patternset) | struct | A set of `PatternID`s. |
| [`PatternSetInsertError`](#patternsetinserterror) | struct | An error that occurs when a `PatternID` failed to insert into a `PatternSet`. |
| [`PatternSetIter`](#patternsetiter) | struct | An iterator over all pattern identifiers in a [`PatternSet`]. |
| [`MatchError`](#matcherror) | struct | An error indicating that a search stopped before reporting whether a match exists or not. |
| [`Anchored`](#anchored) | enum | The type of anchored search to perform. |
| [`MatchKind`](#matchkind) | enum | The kind of match semantics to use for a regex pattern. |
| [`MatchErrorKind`](#matcherrorkind) | enum | The underlying kind of a [`MatchError`]. |

## Structs

### `Input<'h>`

```rust
struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:102-107`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L102-L107)*

The parameters for a regex search including the haystack to search.

It turns out that regex searches have a few parameters, and in most cases,
those parameters have defaults that work in the vast majority of cases.
This `Input` type exists to make that common case seamless while also
providing an avenue for changing the parameters of a search. In particular,
this type enables doing so without a combinatorial explosion of different
methods and/or superfluous parameters in the common cases.

An `Input` permits configuring the following things:

* Search only a substring of a haystack, while taking the broader context
into account for resolving look-around assertions.
* Indicating whether to search for all patterns in a regex, or to
only search for one pattern in particular.
* Whether to perform an anchored on unanchored search.
* Whether to report a match as early as possible.

All of these parameters, except for the haystack, have sensible default
values. This means that the minimal search configuration is simply a call
to `Input::new` with your haystack. Setting any other parameter is
optional.

Moreover, for any `H` that implements `AsRef<[u8]>`, there exists a
`From<H> for Input` implementation. This is useful because many of the
search APIs in this crate accept an `Into<Input>`. This means you can
provide string or byte strings to these routines directly, and they'll
automatically get converted into an `Input` for you.

The lifetime parameter `'h` refers to the lifetime of the haystack.

# Organization

The API of `Input` is split into a few different parts:

* A builder-like API that transforms a `Input` by value. Examples:
`Input::span` and `Input::anchored`.
* A setter API that permits mutating parameters in place. Examples:
`Input::set_span` and `Input::set_anchored`.
* A getter API that permits retrieving any of the search parameters.
Examples: `Input::get_span` and `Input::get_anchored`.
* A few convenience getter routines that don't conform to the above naming
pattern due to how common they are. Examples: `Input::haystack`,
`Input::start` and `Input::end`.
* Miscellaneous predicates and other helper routines that are useful
in some contexts. Examples: `Input::is_char_boundary`.

A `Input` exposes so much because it is meant to be used by both callers of
regex engines _and_ implementors of regex engines. A constraining factor is
that regex engines should accept a `&Input` as its lowest level API, which
means that implementors should only use the "getter" APIs of a `Input`.

# Valid bounds and search termination

An `Input` permits setting the bounds of a search via either
`Input::span` or `Input::range`. The bounds set must be valid, or
else a panic will occur. Bounds are valid if and only if:

* The bounds represent a valid range into the input's haystack.
* **or** the end bound is a valid ending bound for the haystack *and*
the start bound is exactly one greater than the start bound.

In the latter case, `Input::is_done` will return true and indicates any
search receiving such an input should immediately return with no match.

Note that while `Input` is used for reverse searches in this crate, the
`Input::is_done` predicate assumes a forward search. Because unsigned
offsets are used internally, there is no way to tell from only the offsets
whether a reverse search is done or not.

# Regex engine support

Any regex engine accepting an `Input` must support at least the following
things:

* Searching a `&[u8]` for matches.
* Searching a substring of `&[u8]` for a match, such that any match
reported must appear entirely within that substring.
* For a forwards search, a match should never be reported when
`Input::is_done` returns true. (For reverse searches, termination should
be handled outside of `Input`.)

Supporting other aspects of an `Input` are optional, but regex engines
should handle aspects they don't support gracefully. How this is done is
generally up to the regex engine. This crate generally treats unsupported
anchored modes as an error to report for example, but for simplicity, in
the meta regex engine, trying to search with an invalid pattern ID just
results in no match being reported.

#### Implementations

- <span id="input-new"></span>`fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h>` — [`Input`](../../index.md#input)

- <span id="input-span"></span>`fn span<S: Into<Span>>(self, span: S) -> Input<'h>` — [`Input`](../../index.md#input)

- <span id="input-range"></span>`fn range<R: RangeBounds<usize>>(self, range: R) -> Input<'h>` — [`Input`](../../index.md#input)

- <span id="input-anchored"></span>`fn anchored(self, mode: Anchored) -> Input<'h>` — [`Anchored`](../../index.md#anchored), [`Input`](../../index.md#input)

- <span id="input-earliest"></span>`fn earliest(self, yes: bool) -> Input<'h>` — [`Input`](../../index.md#input)

- <span id="input-set-span"></span>`fn set_span<S: Into<Span>>(&mut self, span: S)`

- <span id="input-set-range"></span>`fn set_range<R: RangeBounds<usize>>(&mut self, range: R)`

- <span id="input-set-start"></span>`fn set_start(&mut self, start: usize)`

- <span id="input-set-end"></span>`fn set_end(&mut self, end: usize)`

- <span id="input-set-anchored"></span>`fn set_anchored(&mut self, mode: Anchored)` — [`Anchored`](../../index.md#anchored)

- <span id="input-set-earliest"></span>`fn set_earliest(&mut self, yes: bool)`

- <span id="input-haystack"></span>`fn haystack(&self) -> &'h [u8]`

- <span id="input-start"></span>`fn start(&self) -> usize`

- <span id="input-end"></span>`fn end(&self) -> usize`

- <span id="input-get-span"></span>`fn get_span(&self) -> Span` — [`Span`](../../index.md#span)

- <span id="input-get-range"></span>`fn get_range(&self) -> Range<usize>`

- <span id="input-get-anchored"></span>`fn get_anchored(&self) -> Anchored` — [`Anchored`](../../index.md#anchored)

- <span id="input-get-earliest"></span>`fn get_earliest(&self) -> bool`

- <span id="input-is-done"></span>`fn is_done(&self) -> bool`

- <span id="input-is-char-boundary"></span>`fn is_char_boundary(&self, offset: usize) -> bool`

#### Trait Implementations

##### `impl Clone for Input<'h>`

- <span id="input-clone"></span>`fn clone(&self) -> Input<'h>` — [`Input`](../../index.md#input)

##### `impl Debug for Input<'h>`

- <span id="input-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Span`

```rust
struct Span {
    pub start: usize,
    pub end: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:807-812`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L807-L812)*

A representation of a span reported by a regex engine.

A span corresponds to the starting and ending _byte offsets_ of a
contiguous region of bytes. The starting offset is inclusive while the
ending offset is exclusive. That is, a span is a half-open interval.

A span is used to report the offsets of a match, but it is also used to
convey which region of a haystack should be searched via routines like
`Input::span`.

This is basically equivalent to a `std::ops::Range<usize>`, except this
type implements `Copy` which makes it more ergonomic to use in the context
of this crate. Like a range, this implements `Index` for `[u8]` and `str`,
and `IndexMut` for `[u8]`. For convenience, this also impls `From<Range>`,
which means things like `Span::from(5..10)` work.

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

- <span id="span-offset"></span>`fn offset(&self, offset: usize) -> Span` — [`Span`](../../index.md#span)

#### Trait Implementations

##### `impl Clone for Span`

- <span id="span-clone"></span>`fn clone(&self) -> Span` — [`Span`](../../index.md#span)

##### `impl Copy for Span`

##### `impl Debug for Span`

- <span id="span-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl Hash for Span`

- <span id="span-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Index for [u8]`

- <span id="u8-index-type-output"></span>`type Output = [u8]`

- <span id="u8-index"></span>`fn index(&self, index: Span) -> &[u8]` — [`Span`](../../index.md#span)

##### `impl IndexMut for [u8]`

- <span id="u8-index-mut"></span>`fn index_mut(&mut self, index: Span) -> &mut [u8]` — [`Span`](../../index.md#span)

##### `impl PartialEq for Span`

- <span id="span-eq"></span>`fn eq(&self, other: &Span) -> bool` — [`Span`](../../index.md#span)

##### `impl StructuralPartialEq for Span`

### `HalfMatch`

```rust
struct HalfMatch {
    pattern: crate::util::primitives::PatternID,
    offset: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:924-932`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L924-L932)*

A representation of "half" of a match reported by a DFA.

This is called a "half" match because it only includes the end location (or
start location for a reverse search) of a match. This corresponds to the
information that a single DFA scan can report. Getting the other half of
the match requires a second scan with a reversed DFA.

A half match also includes the pattern that matched. The pattern is
identified by an ID, which corresponds to its position (starting from `0`)
relative to other patterns used to construct the corresponding DFA. If only
a single pattern is provided to the DFA, then all matches are guaranteed to
have a pattern ID of `0`.

#### Fields

- **`pattern`**: `crate::util::primitives::PatternID`

  The pattern ID.

- **`offset`**: `usize`

  The offset of the match.
  
  For forward searches, the offset is exclusive. For reverse searches,
  the offset is inclusive.

#### Implementations

- <span id="halfmatch-new"></span>`fn new(pattern: PatternID, offset: usize) -> HalfMatch` — [`PatternID`](../primitives/index.md#patternid), [`HalfMatch`](../../index.md#halfmatch)

- <span id="halfmatch-must"></span>`fn must(pattern: usize, offset: usize) -> HalfMatch` — [`HalfMatch`](../../index.md#halfmatch)

- <span id="halfmatch-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](../primitives/index.md#patternid)

- <span id="halfmatch-offset"></span>`fn offset(&self) -> usize`

#### Trait Implementations

##### `impl Clone for HalfMatch`

- <span id="halfmatch-clone"></span>`fn clone(&self) -> HalfMatch` — [`HalfMatch`](../../index.md#halfmatch)

##### `impl Copy for HalfMatch`

##### `impl Debug for HalfMatch`

- <span id="halfmatch-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for HalfMatch`

##### `impl Hash for HalfMatch`

- <span id="halfmatch-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for HalfMatch`

- <span id="halfmatch-eq"></span>`fn eq(&self, other: &HalfMatch) -> bool` — [`HalfMatch`](../../index.md#halfmatch)

##### `impl StructuralPartialEq for HalfMatch`

### `Match`

```rust
struct Match {
    pattern: crate::util::primitives::PatternID,
    span: Span,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:985-990`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L985-L990)*

A representation of a match reported by a regex engine.

A match has two essential pieces of information: the [`PatternID`](../primitives/index.md) that
matches, and the [`Span`](../../index.md) of the match in a haystack.

The pattern is identified by an ID, which corresponds to its position
(starting from `0`) relative to other patterns used to construct the
corresponding regex engine. If only a single pattern is provided, then all
matches are guaranteed to have a pattern ID of `0`.

Every match reported by a regex engine guarantees that its span has its
start offset as less than or equal to its end offset.

#### Fields

- **`pattern`**: `crate::util::primitives::PatternID`

  The pattern ID.

- **`span`**: `Span`

  The underlying match span.

#### Implementations

- <span id="match-new"></span>`fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match` — [`PatternID`](../primitives/index.md#patternid), [`Match`](../../index.md#match)

- <span id="match-must"></span>`fn must<S: Into<Span>>(pattern: usize, span: S) -> Match` — [`Match`](../../index.md#match)

- <span id="match-pattern"></span>`fn pattern(&self) -> PatternID` — [`PatternID`](../primitives/index.md#patternid)

- <span id="match-start"></span>`fn start(&self) -> usize`

- <span id="match-end"></span>`fn end(&self) -> usize`

- <span id="match-range"></span>`fn range(&self) -> core::ops::Range<usize>`

- <span id="match-span"></span>`fn span(&self) -> Span` — [`Span`](../../index.md#span)

- <span id="match-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="match-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Match`

- <span id="match-clone"></span>`fn clone(&self) -> Match` — [`Match`](../../index.md#match)

##### `impl Copy for Match`

##### `impl Debug for Match`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Match`

##### `impl Hash for Match`

- <span id="match-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Match`

- <span id="match-eq"></span>`fn eq(&self, other: &Match) -> bool` — [`Match`](../../index.md#match)

##### `impl StructuralPartialEq for Match`

### `PatternSet`

```rust
struct PatternSet {
    len: usize,
    which: alloc::boxed::Box<[bool]>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1149-1172`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1149-L1172)*

A set of `PatternID`s.

A set of pattern identifiers is useful for recording which patterns have
matched a particular haystack. A pattern set _only_ includes pattern
identifiers. It does not include offset information.

# Example

This shows basic usage of a set.

```rust
use regex_automata::{PatternID, PatternSet};

let pid1 = PatternID::must(5);
let pid2 = PatternID::must(8);
// Create a new empty set.
let mut set = PatternSet::new(10);
// Insert pattern IDs.
set.insert(pid1);
set.insert(pid2);
// Test membership.
assert!(set.contains(pid1));
assert!(set.contains(pid2));
// Get all members.
assert_eq!(
    vec![5, 8],
    set.iter().map(|p| p.as_usize()).collect::<Vec<usize>>(),
);
// Clear the set.
set.clear();
// Test that it is indeed empty.
assert!(set.is_empty());
```

#### Fields

- **`len`**: `usize`

  The number of patterns set to 'true' in this set.

- **`which`**: `alloc::boxed::Box<[bool]>`

  A map from PatternID to boolean of whether a pattern matches or not.
  
  This should probably be a bitset, but it's probably unlikely to matter
  much in practice.
  
  The main downside of this representation (and similarly for a bitset)
  is that iteration scales with the capacity of the set instead of
  the length of the set. This doesn't seem likely to be a problem in
  practice.
  
  Another alternative is to just use a 'SparseSet' for this. It does use
  more memory (quite a bit more), but that seems fine I think compared
  to the memory being used by the regex engine. The real hiccup with
  it is that it yields pattern IDs in the order they were inserted.
  Which is actually kind of nice, but at the time of writing, pattern
  IDs are yielded in ascending order in the regex crate RegexSet API.
  If we did change to 'SparseSet', we could provide an additional
  'iter_match_order' iterator, but keep the ascending order one for
  compatibility.

#### Implementations

- <span id="patternset-new"></span>`fn new(capacity: usize) -> PatternSet` — [`PatternSet`](../../index.md#patternset)

- <span id="patternset-clear"></span>`fn clear(&mut self)`

- <span id="patternset-contains"></span>`fn contains(&self, pid: PatternID) -> bool` — [`PatternID`](../primitives/index.md#patternid)

- <span id="patternset-insert"></span>`fn insert(&mut self, pid: PatternID) -> bool` — [`PatternID`](../primitives/index.md#patternid)

- <span id="patternset-try-insert"></span>`fn try_insert(&mut self, pid: PatternID) -> Result<bool, PatternSetInsertError>` — [`PatternID`](../primitives/index.md#patternid), [`PatternSetInsertError`](../../index.md#patternsetinserterror)

- <span id="patternset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="patternset-is-full"></span>`fn is_full(&self) -> bool`

- <span id="patternset-len"></span>`fn len(&self) -> usize`

- <span id="patternset-capacity"></span>`fn capacity(&self) -> usize`

- <span id="patternset-iter"></span>`fn iter(&self) -> PatternSetIter<'_>` — [`PatternSetIter`](../../index.md#patternsetiter)

#### Trait Implementations

##### `impl Clone for PatternSet`

- <span id="patternset-clone"></span>`fn clone(&self) -> PatternSet` — [`PatternSet`](../../index.md#patternset)

##### `impl Debug for PatternSet`

- <span id="patternset-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for PatternSet`

##### `impl PartialEq for PatternSet`

- <span id="patternset-eq"></span>`fn eq(&self, other: &PatternSet) -> bool` — [`PatternSet`](../../index.md#patternset)

##### `impl StructuralPartialEq for PatternSet`

### `PatternSetInsertError`

```rust
struct PatternSetInsertError {
    attempted: crate::util::primitives::PatternID,
    capacity: usize,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1335-1338`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1335-L1338)*

An error that occurs when a `PatternID` failed to insert into a
`PatternSet`.

An insert fails when the given `PatternID` exceeds the configured capacity
of the `PatternSet`.

This error is created by the `PatternSet::try_insert` routine.

#### Trait Implementations

##### `impl Clone for PatternSetInsertError`

- <span id="patternsetinserterror-clone"></span>`fn clone(&self) -> PatternSetInsertError` — [`PatternSetInsertError`](../../index.md#patternsetinserterror)

##### `impl Debug for PatternSetInsertError`

- <span id="patternsetinserterror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for PatternSetInsertError`

- <span id="patternsetinserterror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for PatternSetInsertError`

##### `impl ToString for PatternSetInsertError`

- <span id="patternsetinserterror-to-string"></span>`fn to_string(&self) -> String`

### `PatternSetIter<'a>`

```rust
struct PatternSetIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1364-1366`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1364-L1366)*

An iterator over all pattern identifiers in a [`PatternSet`](../../index.md).

The lifetime parameter `'a` refers to the lifetime of the pattern set being
iterated over.

This iterator is created by the `PatternSet::iter` method.

#### Trait Implementations

##### `impl Clone for PatternSetIter<'a>`

- <span id="patternsetiter-clone"></span>`fn clone(&self) -> PatternSetIter<'a>` — [`PatternSetIter`](../../index.md#patternsetiter)

##### `impl Debug for PatternSetIter<'a>`

- <span id="patternsetiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for PatternSetIter<'a>`

- <span id="patternsetiter-next-back"></span>`fn next_back(&mut self) -> Option<PatternID>` — [`PatternID`](../primitives/index.md#patternid)

##### `impl IntoIterator for PatternSetIter<'a>`

- <span id="patternsetiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="patternsetiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="patternsetiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for PatternSetIter<'a>`

- <span id="patternsetiter-iterator-type-item"></span>`type Item = PatternID`

- <span id="patternsetiter-next"></span>`fn next(&mut self) -> Option<PatternID>` — [`PatternID`](../primitives/index.md#patternid)

- <span id="patternsetiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1778-1781`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1778-L1781)*

An error indicating that a search stopped before reporting whether a
match exists or not.

To be very clear, this error type implies that one cannot assume that no
matches occur, since the search stopped before completing. That is, if
you're looking for information about where a search determined that no
match can occur, then this error type does *not* give you that. (Indeed, at
the time of writing, if you need such a thing, you have to write your own
search routine.)

Normally, when one searches for something, the response is either an
affirmative "it was found at this location" or a negative "not found at
all." However, in some cases, a regex engine can be configured to stop its
search before concluding whether a match exists or not. When this happens,
it may be important for the caller to know why the regex engine gave up and
where in the input it gave up at. This error type exposes the 'why' and the
'where.'

For example, the DFAs provided by this library generally cannot correctly
implement Unicode word boundaries. Instead, they provide an option to
eagerly support them on ASCII text (since Unicode word boundaries are
equivalent to ASCII word boundaries when searching ASCII text), but will
"give up" if a non-ASCII byte is seen. In such cases, one is usually
required to either report the failure to the caller (unergonomic) or
otherwise fall back to some other regex engine (ergonomic, but potentially
costly).

More generally, some regex engines offer the ability for callers to specify
certain bytes that will trigger the regex engine to automatically quit if
they are seen.

Still yet, there may be other reasons for a failed match. For example,
the hybrid DFA provided by this crate can be configured to give up if it
believes that it is not efficient. This in turn permits callers to choose a
different regex engine.

(Note that DFAs are configured by default to never quit or give up in this
fashion. For example, by default, a DFA will fail to build if the regex
pattern contains a Unicode word boundary. One needs to opt into the "quit"
behavior via options, like
[`hybrid::dfa::Config::unicode_word_boundary`](crate::hybrid::dfa::Config::unicode_word_boundary).)

There are a couple other ways a search
can fail. For example, when using the
[`BoundedBacktracker`](crate::nfa::thompson::backtrack::BoundedBacktracker)
with a haystack that is too long, or trying to run an unanchored search
with a [one-pass DFA](crate::dfa::onepass).

#### Implementations

- <span id="matcherror-new"></span>`fn new(kind: MatchErrorKind) -> MatchError` — [`MatchErrorKind`](../../index.md#matcherrorkind), [`MatchError`](../../index.md#matcherror)

- <span id="matcherror-kind"></span>`fn kind(&self) -> &MatchErrorKind` — [`MatchErrorKind`](../../index.md#matcherrorkind)

- <span id="matcherror-quit"></span>`fn quit(byte: u8, offset: usize) -> MatchError` — [`MatchError`](../../index.md#matcherror)

- <span id="matcherror-gave-up"></span>`fn gave_up(offset: usize) -> MatchError` — [`MatchError`](../../index.md#matcherror)

- <span id="matcherror-haystack-too-long"></span>`fn haystack_too_long(len: usize) -> MatchError` — [`MatchError`](../../index.md#matcherror)

- <span id="matcherror-unsupported-anchored"></span>`fn unsupported_anchored(mode: Anchored) -> MatchError` — [`Anchored`](../../index.md#anchored), [`MatchError`](../../index.md#matcherror)

#### Trait Implementations

##### `impl Clone for MatchError`

- <span id="matcherror-clone"></span>`fn clone(&self) -> MatchError` — [`MatchError`](../../index.md#matcherror)

##### `impl Debug for MatchError`

- <span id="matcherror-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for MatchError`

- <span id="matcherror-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl PartialEq for MatchError`

- <span id="matcherror-eq"></span>`fn eq(&self, other: &MatchError) -> bool` — [`MatchError`](../../index.md#matcherror)

##### `impl StructuralPartialEq for MatchError`

##### `impl ToString for MatchError`

- <span id="matcherror-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Anchored`

```rust
enum Anchored {
    No,
    Yes,
    Pattern(crate::util::primitives::PatternID),
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1501-1516`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1501-L1516)*

The type of anchored search to perform.

This is *almost* a boolean option. That is, you can either do an unanchored
search for any pattern in a regex, or you can do an anchored search for any
pattern in a regex.

A third option exists that, assuming the regex engine supports it, permits
you to do an anchored search for a specific pattern.

Note that there is no way to run an unanchored search for a specific
pattern. If you need that, you'll need to build separate regexes for each
pattern.

# Errors

If a regex engine does not support the anchored mode selected, then the
regex engine will return an error. While any non-trivial regex engine
should support at least one of the available anchored modes, there is no
singular mode that is guaranteed to be universally supported. Some regex
engines might only support unanchored searches (DFAs compiled without
anchored starting states) and some regex engines might only support
anchored searches (like the one-pass DFA).

The specific error returned is a [`MatchError`](../../index.md) with a
[`MatchErrorKind::UnsupportedAnchored`](../../index.md) kind. The kind includes the
`Anchored` value given that is unsupported.

Note that regex engines should report "no match" if, for example, an
`Anchored::Pattern` is provided with an invalid pattern ID _but_ where
anchored searches for a specific pattern are supported. This is smooths out
behavior such that it's possible to guarantee that an error never occurs
based on how the regex engine is configured. All regex engines in this
crate report "no match" when searching for an invalid pattern ID, but where
searching for a valid pattern ID is otherwise supported.

# Example

This example shows how to use the various `Anchored` modes to run a
search. We use the [`PikeVM`](crate::nfa::thompson::pikevm::PikeVM)
because it supports all modes unconditionally. Some regex engines, like
the [`onepass::DFA`](crate::dfa::onepass::DFA) cannot support unanchored
searches.

```rust
if cfg!(miri) { return Ok(()); } // miri takes too long
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    Anchored, Input, Match, PatternID,
};

let re = PikeVM::new_many(&[
    r"Mrs. \w+",
    r"Miss \w+",
    r"Mr. \w+",
    r"Ms. \w+",
])?;
let mut cache = re.create_cache();
let hay = "Hello Mr. Springsteen!";

// The default is to do an unanchored search.
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, hay));
// Explicitly ask for an unanchored search. Same as above.
let input = Input::new(hay).anchored(Anchored::No);
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, hay));

// Now try an anchored search. Since the match doesn't start at the
// beginning of the haystack, no match is found!
let input = Input::new(hay).anchored(Anchored::Yes);
assert_eq!(None, re.find(&mut cache, input));

// We can try an anchored search again, but move the location of where
// we start the search. Note that the offsets reported are still in
// terms of the overall haystack and not relative to where we started
// the search.
let input = Input::new(hay).anchored(Anchored::Yes).range(6..);
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, input));

// Now try an anchored search for a specific pattern. We specifically
// choose a pattern that we know doesn't match to prove that the search
// only looks for the pattern we provide.
let input = Input::new(hay)
    .anchored(Anchored::Pattern(PatternID::must(1)))
    .range(6..);
assert_eq!(None, re.find(&mut cache, input));

// But if we switch it to the pattern that we know matches, then we find
// the match.
let input = Input::new(hay)
    .anchored(Anchored::Pattern(PatternID::must(2)))
    .range(6..);
assert_eq!(Some(Match::must(2, 6..21)), re.find(&mut cache, input));

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Variants

- **`No`**

  Run an unanchored search. This means a match may occur anywhere at or
  after the start position of the search.
  
  This search can return a match for any pattern in the regex.

- **`Yes`**

  Run an anchored search. This means that a match must begin at the
  start position of the search.
  
  This search can return a match for any pattern in the regex.

- **`Pattern`**

  Run an anchored search for a specific pattern. This means that a match
  must be for the given pattern and must begin at the start position of
  the search.

#### Implementations

- <span id="anchored-is-anchored"></span>`fn is_anchored(&self) -> bool`

- <span id="anchored-pattern"></span>`fn pattern(&self) -> Option<PatternID>` — [`PatternID`](../primitives/index.md#patternid)

#### Trait Implementations

##### `impl Clone for Anchored`

- <span id="anchored-clone"></span>`fn clone(&self) -> Anchored` — [`Anchored`](../../index.md#anchored)

##### `impl Copy for Anchored`

##### `impl Debug for Anchored`

- <span id="anchored-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Anchored`

##### `impl PartialEq for Anchored`

- <span id="anchored-eq"></span>`fn eq(&self, other: &Anchored) -> bool` — [`Anchored`](../../index.md#anchored)

##### `impl StructuralPartialEq for Anchored`

### `MatchKind`

```rust
enum MatchKind {
    All,
    LeftmostFirst,
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1698-1721`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1698-L1721)*

The kind of match semantics to use for a regex pattern.

The default match kind is `LeftmostFirst`, and this corresponds to the
match semantics used by most backtracking engines, such as Perl.

# Leftmost first or "preference order" match semantics

Leftmost-first semantics determine which match to report when there are
multiple paths through a regex that match at the same position. The tie is
essentially broken by how a backtracker would behave. For example, consider
running the regex `foofoofoo|foofoo|foo` on the haystack `foofoo`. In this
case, both the `foofoo` and `foo` branches match at position `0`. So should
the end of the match be `3` or `6`?

A backtracker will conceptually work by trying `foofoofoo` and failing.
Then it will try `foofoo`, find the match and stop there. Thus, the
leftmost-first match position is `6`. This is called "leftmost-first" or
"preference order" because the order of the branches as written in the
regex pattern is what determines how to break the tie.

(Note that leftmost-longest match semantics, which break ties by always
taking the longest matching string, are not currently supported by this
crate. These match semantics tend to be found in POSIX regex engines.)

This example shows how leftmost-first semantics work, and how it even
applies to multi-pattern regexes:

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    Match,
};

let re = PikeVM::new_many(&[
    r"foofoofoo",
    r"foofoo",
    r"foo",
])?;
let mut cache = re.create_cache();
let got: Vec<Match> = re.find_iter(&mut cache, "foofoo").collect();
let expected = vec![Match::must(1, 0..6)];
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

# All matches

The `All` match semantics report any and all matches, and generally will
attempt to match as much as possible. It doesn't respect any sort of match
priority at all, so things like non-greedy matching don't work in this
mode.

The fact that non-greedy matching doesn't work generally makes most forms
of unanchored non-overlapping searches have unintuitive behavior. Namely,
unanchored searches behave as if there is a `(?s-u:.)*?` prefix at the
beginning of the pattern, which is specifically non-greedy. Since it will
be treated as greedy in `All` match semantics, this generally means that
it will first attempt to consume all of the haystack and is likely to wind
up skipping matches.

Generally speaking, `All` should only be used in two circumstances:

* When running an anchored search and there is a desire to match as much as
possible. For example, when building a reverse regex matcher to find the
start of a match after finding the end. In this case, the reverse search
is anchored to the end of the match found by the forward search.
* When running overlapping searches. Since `All` encodes all possible
matches, this is generally what you want for an overlapping search. If you
try to use leftmost-first in an overlapping search, it is likely to produce
counter-intuitive results since leftmost-first specifically excludes some
matches from its underlying finite state machine.

This example demonstrates the counter-intuitive behavior of `All` semantics
when using a standard leftmost unanchored search:

```rust
use regex_automata::{
    nfa::thompson::pikevm::PikeVM,
    Match, MatchKind,
};

let re = PikeVM::builder()
    .configure(PikeVM::config().match_kind(MatchKind::All))
    .build("foo")?;
let hay = "first foo second foo wat";
let mut cache = re.create_cache();
let got: Vec<Match> = re.find_iter(&mut cache, hay).collect();
// Notice that it completely skips the first 'foo'!
let expected = vec![Match::must(0, 17..20)];
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

This second example shows how `All` semantics are useful for an overlapping
search. Note that we use lower level lazy DFA APIs here since the NFA
engines only currently support a very limited form of overlapping search.

```rust
use regex_automata::{
    hybrid::dfa::{DFA, OverlappingState},
    HalfMatch, Input, MatchKind,
};

let re = DFA::builder()
    // If we didn't set 'All' semantics here, then the regex would only
    // match 'foo' at offset 3 and nothing else. Why? Because the state
    // machine implements preference order and knows that the 'foofoo' and
    // 'foofoofoo' branches can never match since 'foo' will always match
    // when they match and take priority.
    .configure(DFA::config().match_kind(MatchKind::All))
    .build(r"foo|foofoo|foofoofoo")?;
let mut cache = re.create_cache();
let mut state = OverlappingState::start();
let input = Input::new("foofoofoo");
let mut got = vec![];
loop {
    re.try_search_overlapping_fwd(&mut cache, &input, &mut state)?;
    let m = match state.get_match() {
        None => break,
        Some(m) => m,
    };
    got.push(m);
}
let expected = vec![
    HalfMatch::must(0, 3),
    HalfMatch::must(0, 6),
    HalfMatch::must(0, 9),
];
assert_eq!(expected, got);

Ok::<(), Box<dyn std::error::Error>>(())
```

#### Variants

- **`All`**

  Report all possible matches.

- **`LeftmostFirst`**

  Report only the leftmost matches. When multiple leftmost matches exist,
  report the match corresponding to the part of the regex that appears
  first in the syntax.

#### Implementations

- <span id="matchkind-continue-past-first-match"></span>`fn continue_past_first_match(&self) -> bool`

#### Trait Implementations

##### `impl Clone for MatchKind`

- <span id="matchkind-clone"></span>`fn clone(&self) -> MatchKind` — [`MatchKind`](../../index.md#matchkind)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- <span id="matchkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MatchKind`

- <span id="matchkind-default"></span>`fn default() -> MatchKind` — [`MatchKind`](../../index.md#matchkind)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- <span id="matchkind-eq"></span>`fn eq(&self, other: &MatchKind) -> bool` — [`MatchKind`](../../index.md#matchkind)

##### `impl StructuralPartialEq for MatchKind`

### `MatchErrorKind`

```rust
enum MatchErrorKind {
    Quit {
        byte: u8,
        offset: usize,
    },
    GaveUp {
        offset: usize,
    },
    HaystackTooLong {
        len: usize,
    },
    UnsupportedAnchored {
        mode: Anchored,
    },
}
```

*Defined in [`regex-automata-0.4.13/src/util/search.rs:1849-1890`](../../../../.source_1765521767/regex-automata-0.4.13/src/util/search.rs#L1849-L1890)*

The underlying kind of a [`MatchError`](../../index.md).

This is a **non-exhaustive** enum. That means new variants may be added in
a semver-compatible release.

#### Variants

- **`Quit`**

  The search saw a "quit" byte at which it was instructed to stop
  searching.

- **`GaveUp`**

  The search, based on heuristics, determined that it would be better
  to stop, typically to provide the caller an opportunity to use an
  alternative regex engine.
  
  Currently, the only way for this to occur is via the lazy DFA and
  only when it is configured to do so (it will not return this error by
  default).

- **`HaystackTooLong`**

  This error occurs if the haystack given to the regex engine was too
  long to be searched. This occurs, for example, with regex engines
  like the bounded backtracker that have a configurable fixed amount of
  capacity that is tied to the length of the haystack. Anything beyond
  that configured limit will result in an error at search time.

- **`UnsupportedAnchored`**

  An error indicating that a particular type of anchored search was
  requested, but that the regex engine does not support it.
  
  Note that this error should not be returned by a regex engine simply
  because the pattern ID is invalid (i.e., equal to or exceeds the number
  of patterns in the regex). In that case, the regex engine should report
  a non-match.

#### Trait Implementations

##### `impl Clone for MatchErrorKind`

- <span id="matcherrorkind-clone"></span>`fn clone(&self) -> MatchErrorKind` — [`MatchErrorKind`](../../index.md#matcherrorkind)

##### `impl Debug for MatchErrorKind`

- <span id="matcherrorkind-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl PartialEq for MatchErrorKind`

- <span id="matcherrorkind-eq"></span>`fn eq(&self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](../../index.md#matcherrorkind)

##### `impl StructuralPartialEq for MatchErrorKind`

