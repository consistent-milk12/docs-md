*[regex_automata](../../index.md) / [util](../index.md) / [search](index.md)*

---

# Module `search`

Types and routines that support the search APIs of most regex engines.

This sub-module isn't exposed directly, but rather, its contents are exported
at the crate root due to the universality of most of the types and routines in
this module.

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

- `fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h>` — [`Input`](../../index.md)

- `fn span<S: Into<Span>>(self: Self, span: S) -> Input<'h>` — [`Input`](../../index.md)

- `fn range<R: RangeBounds<usize>>(self: Self, range: R) -> Input<'h>` — [`Input`](../../index.md)

- `fn anchored(self: Self, mode: Anchored) -> Input<'h>` — [`Anchored`](../../index.md), [`Input`](../../index.md)

- `fn earliest(self: Self, yes: bool) -> Input<'h>` — [`Input`](../../index.md)

- `fn set_span<S: Into<Span>>(self: &mut Self, span: S)`

- `fn set_range<R: RangeBounds<usize>>(self: &mut Self, range: R)`

- `fn set_start(self: &mut Self, start: usize)`

- `fn set_end(self: &mut Self, end: usize)`

- `fn set_anchored(self: &mut Self, mode: Anchored)` — [`Anchored`](../../index.md)

- `fn set_earliest(self: &mut Self, yes: bool)`

- `fn haystack(self: &Self) -> &'h [u8]`

- `fn start(self: &Self) -> usize`

- `fn end(self: &Self) -> usize`

- `fn get_span(self: &Self) -> Span` — [`Span`](../../index.md)

- `fn get_range(self: &Self) -> Range<usize>`

- `fn get_anchored(self: &Self) -> Anchored` — [`Anchored`](../../index.md)

- `fn get_earliest(self: &Self) -> bool`

- `fn is_done(self: &Self) -> bool`

- `fn is_char_boundary(self: &Self, offset: usize) -> bool`

#### Trait Implementations

##### `impl<'h> Clone for Input<'h>`

- `fn clone(self: &Self) -> Input<'h>` — [`Input`](../../index.md)

##### `impl<'h> Debug for Input<'h>`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

### `Span`

```rust
struct Span {
    pub start: usize,
    pub end: usize,
}
```

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

- `fn range(self: &Self) -> Range<usize>`

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn contains(self: &Self, offset: usize) -> bool`

- `fn offset(self: &Self, offset: usize) -> Span` — [`Span`](../../index.md)

#### Trait Implementations

##### `impl Clone for Span`

- `fn clone(self: &Self) -> Span` — [`Span`](../../index.md)

##### `impl Copy for Span`

##### `impl Debug for Span`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Span`

##### `impl Hash for Span`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Span`

- `fn eq(self: &Self, other: &Span) -> bool` — [`Span`](../../index.md)

##### `impl StructuralPartialEq for Span`

### `HalfMatch`

```rust
struct HalfMatch {
    pattern: crate::util::primitives::PatternID,
    offset: usize,
}
```

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

- `fn new(pattern: PatternID, offset: usize) -> HalfMatch` — [`PatternID`](../../index.md), [`HalfMatch`](../../index.md)

- `fn must(pattern: usize, offset: usize) -> HalfMatch` — [`HalfMatch`](../../index.md)

- `fn pattern(self: &Self) -> PatternID` — [`PatternID`](../../index.md)

- `fn offset(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for HalfMatch`

- `fn clone(self: &Self) -> HalfMatch` — [`HalfMatch`](../../index.md)

##### `impl Copy for HalfMatch`

##### `impl Debug for HalfMatch`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for HalfMatch`

##### `impl Hash for HalfMatch`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for HalfMatch`

- `fn eq(self: &Self, other: &HalfMatch) -> bool` — [`HalfMatch`](../../index.md)

##### `impl StructuralPartialEq for HalfMatch`

### `Match`

```rust
struct Match {
    pattern: crate::util::primitives::PatternID,
    span: Span,
}
```

A representation of a match reported by a regex engine.

A match has two essential pieces of information: the [`PatternID`](../../index.md) that
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

- `fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match` — [`PatternID`](../../index.md), [`Match`](../../index.md)

- `fn must<S: Into<Span>>(pattern: usize, span: S) -> Match` — [`Match`](../../index.md)

- `fn pattern(self: &Self) -> PatternID` — [`PatternID`](../../index.md)

- `fn start(self: &Self) -> usize`

- `fn end(self: &Self) -> usize`

- `fn range(self: &Self) -> core::ops::Range<usize>`

- `fn span(self: &Self) -> Span` — [`Span`](../../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

#### Trait Implementations

##### `impl Clone for Match`

- `fn clone(self: &Self) -> Match` — [`Match`](../../index.md)

##### `impl Copy for Match`

##### `impl Debug for Match`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Match`

##### `impl Hash for Match`

- `fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H)`

##### `impl PartialEq for Match`

- `fn eq(self: &Self, other: &Match) -> bool` — [`Match`](../../index.md)

##### `impl StructuralPartialEq for Match`

### `PatternSet`

```rust
struct PatternSet {
    len: usize,
    which: alloc::boxed::Box<[bool]>,
}
```

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

- `fn new(capacity: usize) -> PatternSet` — [`PatternSet`](../../index.md)

- `fn clear(self: &mut Self)`

- `fn contains(self: &Self, pid: PatternID) -> bool` — [`PatternID`](../../index.md)

- `fn insert(self: &mut Self, pid: PatternID) -> bool` — [`PatternID`](../../index.md)

- `fn try_insert(self: &mut Self, pid: PatternID) -> Result<bool, PatternSetInsertError>` — [`PatternID`](../../index.md), [`PatternSetInsertError`](../../index.md)

- `fn is_empty(self: &Self) -> bool`

- `fn is_full(self: &Self) -> bool`

- `fn len(self: &Self) -> usize`

- `fn capacity(self: &Self) -> usize`

- `fn iter(self: &Self) -> PatternSetIter<'_>` — [`PatternSetIter`](../../index.md)

#### Trait Implementations

##### `impl Clone for PatternSet`

- `fn clone(self: &Self) -> PatternSet` — [`PatternSet`](../../index.md)

##### `impl Debug for PatternSet`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for PatternSet`

##### `impl PartialEq for PatternSet`

- `fn eq(self: &Self, other: &PatternSet) -> bool` — [`PatternSet`](../../index.md)

##### `impl StructuralPartialEq for PatternSet`

### `PatternSetInsertError`

```rust
struct PatternSetInsertError {
    attempted: crate::util::primitives::PatternID,
    capacity: usize,
}
```

An error that occurs when a `PatternID` failed to insert into a
`PatternSet`.

An insert fails when the given `PatternID` exceeds the configured capacity
of the `PatternSet`.

This error is created by the `PatternSet::try_insert` routine.

#### Trait Implementations

##### `impl Clone for PatternSetInsertError`

- `fn clone(self: &Self) -> PatternSetInsertError` — [`PatternSetInsertError`](../../index.md)

##### `impl Debug for PatternSetInsertError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for PatternSetInsertError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for PatternSetInsertError`

##### `impl<T> ToString for PatternSetInsertError`

- `fn to_string(self: &Self) -> String`

### `PatternSetIter<'a>`

```rust
struct PatternSetIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
}
```

An iterator over all pattern identifiers in a [`PatternSet`](../../index.md).

The lifetime parameter `'a` refers to the lifetime of the pattern set being
iterated over.

This iterator is created by the `PatternSet::iter` method.

#### Trait Implementations

##### `impl<'a> Clone for PatternSetIter<'a>`

- `fn clone(self: &Self) -> PatternSetIter<'a>` — [`PatternSetIter`](../../index.md)

##### `impl<'a> Debug for PatternSetIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for PatternSetIter<'a>`

- `fn next_back(self: &mut Self) -> Option<PatternID>` — [`PatternID`](../../index.md)

##### `impl<I> IntoIterator for PatternSetIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for PatternSetIter<'a>`

- `type Item = PatternID`

- `fn next(self: &mut Self) -> Option<PatternID>` — [`PatternID`](../../index.md)

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `MatchError`

```rust
struct MatchError(alloc::boxed::Box<MatchErrorKind>);
```

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

- `fn new(kind: MatchErrorKind) -> MatchError` — [`MatchErrorKind`](../../index.md), [`MatchError`](../../index.md)

- `fn kind(self: &Self) -> &MatchErrorKind` — [`MatchErrorKind`](../../index.md)

- `fn quit(byte: u8, offset: usize) -> MatchError` — [`MatchError`](../../index.md)

- `fn gave_up(offset: usize) -> MatchError` — [`MatchError`](../../index.md)

- `fn haystack_too_long(len: usize) -> MatchError` — [`MatchError`](../../index.md)

- `fn unsupported_anchored(mode: Anchored) -> MatchError` — [`Anchored`](../../index.md), [`MatchError`](../../index.md)

#### Trait Implementations

##### `impl Clone for MatchError`

- `fn clone(self: &Self) -> MatchError` — [`MatchError`](../../index.md)

##### `impl Debug for MatchError`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Display for MatchError`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for MatchError`

##### `impl Error for MatchError`

##### `impl PartialEq for MatchError`

- `fn eq(self: &Self, other: &MatchError) -> bool` — [`MatchError`](../../index.md)

##### `impl StructuralPartialEq for MatchError`

##### `impl<T> ToString for MatchError`

- `fn to_string(self: &Self) -> String`

## Enums

### `Anchored`

```rust
enum Anchored {
    No,
    Yes,
    Pattern(crate::util::primitives::PatternID),
}
```

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

- `fn is_anchored(self: &Self) -> bool`

- `fn pattern(self: &Self) -> Option<PatternID>` — [`PatternID`](../../index.md)

#### Trait Implementations

##### `impl Clone for Anchored`

- `fn clone(self: &Self) -> Anchored` — [`Anchored`](../../index.md)

##### `impl Copy for Anchored`

##### `impl Debug for Anchored`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Anchored`

##### `impl PartialEq for Anchored`

- `fn eq(self: &Self, other: &Anchored) -> bool` — [`Anchored`](../../index.md)

##### `impl StructuralPartialEq for Anchored`

### `MatchKind`

```rust
enum MatchKind {
    All,
    LeftmostFirst,
}
```

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

- `fn continue_past_first_match(self: &Self) -> bool`

#### Trait Implementations

##### `impl Clone for MatchKind`

- `fn clone(self: &Self) -> MatchKind` — [`MatchKind`](../../index.md)

##### `impl Copy for MatchKind`

##### `impl Debug for MatchKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for MatchKind`

- `fn default() -> MatchKind` — [`MatchKind`](../../index.md)

##### `impl Eq for MatchKind`

##### `impl PartialEq for MatchKind`

- `fn eq(self: &Self, other: &MatchKind) -> bool` — [`MatchKind`](../../index.md)

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

- `fn clone(self: &Self) -> MatchErrorKind` — [`MatchErrorKind`](../../index.md)

##### `impl Debug for MatchErrorKind`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for MatchErrorKind`

##### `impl PartialEq for MatchErrorKind`

- `fn eq(self: &Self, other: &MatchErrorKind) -> bool` — [`MatchErrorKind`](../../index.md)

##### `impl StructuralPartialEq for MatchErrorKind`

