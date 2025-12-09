*[regex](../../index.md) / [regexset](../index.md) / [bytes](index.md)*

---

# Module `bytes`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RegexSet`](#regexset) | struct | Match multiple, possibly overlapping, regexes in a single search. |
| [`SetMatches`](#setmatches) | struct | A set of matches returned by a regex set. |
| [`SetMatchesIntoIter`](#setmatchesintoiter) | struct | An owned iterator over the set of matches from a regex set. |
| [`SetMatchesIter`](#setmatchesiter) | struct | A borrowed iterator over the set of matches from a regex set. |

## Structs

### `RegexSet`

```rust
struct RegexSet {
    meta: meta::Regex,
    patterns: alloc::sync::Arc<[alloc::string::String]>,
}
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:136-139`](../../../../.source_1765210505/regex-1.12.2/src/regexset/bytes.rs#L136-L139)*

Match multiple, possibly overlapping, regexes in a single search.

A regex set corresponds to the union of zero or more regular expressions.
That is, a regex set will match a haystack when at least one of its
constituent regexes matches. A regex set as its formulated here provides a
touch more power: it will also report *which* regular expressions in the
set match. Indeed, this is the key difference between regex sets and a
single `Regex` with many alternates, since only one alternate can match at
a time.

For example, consider regular expressions to match email addresses and
domains: `[a-z]+@[a-z]+\.(com|org|net)` and `[a-z]+\.(com|org|net)`. If a
regex set is constructed from those regexes, then searching the haystack
`foo@example.com` will report both regexes as matching. Of course, one
could accomplish this by compiling each regex on its own and doing two
searches over the haystack. The key advantage of using a regex set is
that it will report the matching regexes using a *single pass through the
haystack*. If one has hundreds or thousands of regexes to match repeatedly
(like a URL router for a complex web application or a user agent matcher),
then a regex set *can* realize huge performance gains.

Unlike the top-level [`RegexSet`](crate::RegexSet), this `RegexSet`
searches haystacks with type `&[u8]` instead of `&str`. Consequently, this
`RegexSet` is permitted to match invalid UTF-8.

# Limitations

Regex sets are limited to answering the following two questions:

1. Does any regex in the set match?
2. If so, which regexes in the set match?

As with the main `Regex` type, it is cheaper to ask
(1) instead of (2) since the matching engines can stop after the first
match is found.

You cannot directly extract `Match` or
`Captures` objects from a regex set. If you need
these operations, the recommended approach is to compile each pattern in
the set independently and scan the exact same haystack a second time with
those independently compiled patterns:

```rust
use regex::bytes::{Regex, RegexSet};

let patterns = ["foo", "bar"];
// Both patterns will match different ranges of this string.
let hay = b"barfoo";

// Compile a set matching any of our patterns.
let set = RegexSet::new(patterns).unwrap();
// Compile each pattern independently.
let regexes: Vec<_> = set
    .patterns()
    .iter()
    .map(|pat| Regex::new(pat).unwrap())
    .collect();

// Match against the whole set first and identify the individual
// matching patterns.
let matches: Vec<&[u8]> = set
    .matches(hay)
    .into_iter()
    // Dereference the match index to get the corresponding
    // compiled pattern.
    .map(|index| &regexes[index])
    // To get match locations or any other info, we then have to search the
    // exact same haystack again, using our separately-compiled pattern.
    .map(|re| re.find(hay).unwrap().as_bytes())
    .collect();

// Matches arrive in the order the constituent patterns were declared,
// not the order they appear in the haystack.
assert_eq!(vec![&b"foo"[..], &b"bar"[..]], matches);
```

# Performance

A `RegexSet` has the same performance characteristics as `Regex`. Namely,
search takes `O(m * n)` time, where `m` is proportional to the size of the
regex set and `n` is proportional to the length of the haystack.

# Trait implementations

The `Default` trait is implemented for `RegexSet`. The default value
is an empty set. An empty set can also be explicitly constructed via
`RegexSet::empty`.

# Example

This shows how the above two regexes (for matching email addresses and
domains) might work:

```rust
use regex::bytes::RegexSet;

let set = RegexSet::new(&[
    r"[a-z]+@[a-z]+\.(com|org|net)",
    r"[a-z]+\.(com|org|net)",
]).unwrap();

// Ask whether any regexes in the set match.
assert!(set.is_match(b"foo@example.com"));

// Identify which regexes in the set match.
let matches: Vec<_> = set.matches(b"foo@example.com").into_iter().collect();
assert_eq!(vec![0, 1], matches);

// Try again, but with a haystack that only matches one of the regexes.
let matches: Vec<_> = set.matches(b"example.com").into_iter().collect();
assert_eq!(vec![1], matches);

// Try again, but with a haystack that doesn't match any regex in the set.
let matches: Vec<_> = set.matches(b"example").into_iter().collect();
assert!(matches.is_empty());
```

Note that it would be possible to adapt the above example to using `Regex`
with an expression like:

```text
(?P<email>[a-z]+@(?P<email_domain>[a-z]+[.](com|org|net)))|(?P<domain>[a-z]+[.](com|org|net))
```

After a match, one could then inspect the capture groups to figure out
which alternates matched. The problem is that it is hard to make this
approach scale when there are many regexes since the overlap between each
alternate isn't always obvious to reason about.

#### Implementations

- <span id="regexset-new"></span>`fn new<I, S>(exprs: I) -> Result<RegexSet, Error>` — [`RegexSet`](#regexset), [`Error`](../../error/index.md)

- <span id="regexset-empty"></span>`fn empty() -> RegexSet` — [`RegexSet`](#regexset)

- <span id="regexset-is-match"></span>`fn is_match(&self, haystack: &[u8]) -> bool`

- <span id="regexset-is-match-at"></span>`fn is_match_at(&self, haystack: &[u8], start: usize) -> bool`

- <span id="regexset-matches"></span>`fn matches(&self, haystack: &[u8]) -> SetMatches` — [`SetMatches`](#setmatches)

- <span id="regexset-matches-at"></span>`fn matches_at(&self, haystack: &[u8], start: usize) -> SetMatches` — [`SetMatches`](#setmatches)

- <span id="regexset-len"></span>`fn len(&self) -> usize`

- <span id="regexset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="regexset-patterns"></span>`fn patterns(&self) -> &[String]`

#### Trait Implementations

##### `impl Clone for RegexSet`

- <span id="regexset-clone"></span>`fn clone(&self) -> RegexSet` — [`RegexSet`](#regexset)

##### `impl Debug for RegexSet`

- <span id="regexset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for RegexSet`

- <span id="regexset-default"></span>`fn default() -> Self`

### `SetMatches`

```rust
struct SetMatches(regex_automata::PatternSet);
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:463`](../../../../.source_1765210505/regex-1.12.2/src/regexset/bytes.rs#L463)*

A set of matches returned by a regex set.

Values of this type are constructed by `RegexSet::matches`.

#### Implementations

- <span id="setmatches-matched-any"></span>`fn matched_any(&self) -> bool`

- <span id="setmatches-matched-all"></span>`fn matched_all(&self) -> bool`

- <span id="setmatches-matched"></span>`fn matched(&self, index: usize) -> bool`

- <span id="setmatches-len"></span>`fn len(&self) -> usize`

- <span id="setmatches-iter"></span>`fn iter(&self) -> SetMatchesIter<'_>` — [`SetMatchesIter`](#setmatchesiter)

#### Trait Implementations

##### `impl Clone for SetMatches`

- <span id="setmatches-clone"></span>`fn clone(&self) -> SetMatches` — [`SetMatches`](#setmatches)

##### `impl Debug for SetMatches`

- <span id="setmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SetMatches`

- <span id="setmatches-type-intoiter"></span>`type IntoIter = SetMatchesIntoIter`

- <span id="setmatches-type-item"></span>`type Item = usize`

- <span id="setmatches-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `SetMatchesIntoIter`

```rust
struct SetMatchesIntoIter {
    patset: regex_automata::PatternSet,
    it: core::ops::Range<usize>,
}
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:656-659`](../../../../.source_1765210505/regex-1.12.2/src/regexset/bytes.rs#L656-L659)*

An owned iterator over the set of matches from a regex set.

This will always produces matches in ascending order of index, where the
index corresponds to the index of the regex that matched with respect to
its position when initially building the set.

This iterator is created by calling `SetMatches::into_iter` via the
`IntoIterator` trait. This is automatically done in `for` loops.

# Example

```rust
use regex::bytes::RegexSet;

let set = RegexSet::new([
    r"[0-9]",
    r"[a-z]",
    r"[A-Z]",
    r"\p{Greek}",
]).unwrap();
let hay = "βa1".as_bytes();
let mut matches = vec![];
for index in set.matches(hay) {
    matches.push(index);
}
assert_eq!(matches, vec![0, 1, 3]);
```

#### Trait Implementations

##### `impl Debug for SetMatchesIntoIter`

- <span id="setmatchesintoiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for SetMatchesIntoIter`

##### `impl IntoIterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="setmatchesintoiter-type-intoiter"></span>`type IntoIter = I`

- <span id="setmatchesintoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-type-item"></span>`type Item = usize`

- <span id="setmatchesintoiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="setmatchesintoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `SetMatchesIter<'a>`

```rust
struct SetMatchesIter<'a>(regex_automata::PatternSetIter<'a>);
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:702`](../../../../.source_1765210505/regex-1.12.2/src/regexset/bytes.rs#L702)*

A borrowed iterator over the set of matches from a regex set.

The lifetime `'a` refers to the lifetime of the [`SetMatches`](#setmatches) value that
created this iterator.

This will always produces matches in ascending order, where the index
corresponds to the index of the regex that matched with respect to its
position when initially building the set.

This iterator is created by the `SetMatches::iter` method.

#### Trait Implementations

##### `impl Clone for SetMatchesIter<'a>`

- <span id="setmatchesiter-clone"></span>`fn clone(&self) -> SetMatchesIter<'a>` — [`SetMatchesIter`](#setmatchesiter)

##### `impl Debug for SetMatchesIter<'a>`

- <span id="setmatchesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for SetMatchesIter<'a>`

##### `impl IntoIterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="setmatchesiter-type-intoiter"></span>`type IntoIter = I`

- <span id="setmatchesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-type-item"></span>`type Item = usize`

- <span id="setmatchesiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="setmatchesiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

