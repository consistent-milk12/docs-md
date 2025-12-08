*[regex](../../index.md) / [regexset](../index.md) / [string](index.md)*

---

# Module `string`

## Structs

### `RegexSet`

```rust
struct RegexSet {
    meta: meta::Regex,
    patterns: alloc::sync::Arc<[alloc::string::String]>,
}
```

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

# Limitations

Regex sets are limited to answering the following two questions:

1. Does any regex in the set match?
2. If so, which regexes in the set match?

As with the main `Regex` type, it is cheaper to ask (1)
instead of (2) since the matching engines can stop after the first match
is found.

You cannot directly extract `Match` or
`Captures` objects from a regex set. If you need these
operations, the recommended approach is to compile each pattern in the set
independently and scan the exact same haystack a second time with those
independently compiled patterns:

```rust
use regex::{Regex, RegexSet};

let patterns = ["foo", "bar"];
// Both patterns will match different ranges of this string.
let hay = "barfoo";

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
let matches: Vec<&str> = set
    .matches(hay)
    .into_iter()
    // Dereference the match index to get the corresponding
    // compiled pattern.
    .map(|index| &regexes[index])
    // To get match locations or any other info, we then have to search the
    // exact same haystack again, using our separately-compiled pattern.
    .map(|re| re.find(hay).unwrap().as_str())
    .collect();

// Matches arrive in the order the constituent patterns were declared,
// not the order they appear in the haystack.
assert_eq!(vec!["foo", "bar"], matches);
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
use regex::RegexSet;

let set = RegexSet::new(&[
    r"[a-z]+@[a-z]+\.(com|org|net)",
    r"[a-z]+\.(com|org|net)",
]).unwrap();

// Ask whether any regexes in the set match.
assert!(set.is_match("foo@example.com"));

// Identify which regexes in the set match.
let matches: Vec<_> = set.matches("foo@example.com").into_iter().collect();
assert_eq!(vec![0, 1], matches);

// Try again, but with a haystack that only matches one of the regexes.
let matches: Vec<_> = set.matches("example.com").into_iter().collect();
assert_eq!(vec![1], matches);

// Try again, but with a haystack that doesn't match any regex in the set.
let matches: Vec<_> = set.matches("example").into_iter().collect();
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

- `fn new<I, S>(exprs: I) -> Result<RegexSet, Error>` — [`RegexSet`](../../index.md), [`Error`](../../error/index.md)

- `fn empty() -> RegexSet` — [`RegexSet`](../../index.md)

- `fn is_match(self: &Self, haystack: &str) -> bool`

- `fn is_match_at(self: &Self, haystack: &str, start: usize) -> bool`

- `fn matches(self: &Self, haystack: &str) -> SetMatches` — [`SetMatches`](../../index.md)

- `fn matches_at(self: &Self, haystack: &str, start: usize) -> SetMatches` — [`SetMatches`](../../index.md)

- `fn len(self: &Self) -> usize`

- `fn is_empty(self: &Self) -> bool`

- `fn patterns(self: &Self) -> &[String]`

#### Trait Implementations

##### `impl Clone for RegexSet`

- `fn clone(self: &Self) -> RegexSet` — [`RegexSet`](../../index.md)

##### `impl Debug for RegexSet`

- `fn fmt(self: &Self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for RegexSet`

- `fn default() -> Self`

### `SetMatches`

```rust
struct SetMatches(regex_automata::PatternSet);
```

A set of matches returned by a regex set.

Values of this type are constructed by `RegexSet::matches`.

#### Implementations

- `fn matched_any(self: &Self) -> bool`

- `fn matched_all(self: &Self) -> bool`

- `fn matched(self: &Self, index: usize) -> bool`

- `fn len(self: &Self) -> usize`

- `fn iter(self: &Self) -> SetMatchesIter<'_>` — [`SetMatchesIter`](../../index.md)

#### Trait Implementations

##### `impl Clone for SetMatches`

- `fn clone(self: &Self) -> SetMatches` — [`SetMatches`](../../index.md)

##### `impl Debug for SetMatches`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl IntoIterator for SetMatches`

- `type IntoIter = SetMatchesIntoIter`

- `type Item = usize`

- `fn into_iter(self: Self) -> <Self as >::IntoIter`

### `SetMatchesIntoIter`

```rust
struct SetMatchesIntoIter {
    patset: regex_automata::PatternSet,
    it: core::ops::Range<usize>,
}
```

An owned iterator over the set of matches from a regex set.

This will always produces matches in ascending order of index, where the
index corresponds to the index of the regex that matched with respect to
its position when initially building the set.

This iterator is created by calling `SetMatches::into_iter` via the
`IntoIterator` trait. This is automatically done in `for` loops.

# Example

```rust
use regex::RegexSet;

let set = RegexSet::new([
    r"[0-9]",
    r"[a-z]",
    r"[A-Z]",
    r"\p{Greek}",
]).unwrap();
let hay = "βa1";
let mut matches = vec![];
for index in set.matches(hay) {
    matches.push(index);
}
assert_eq!(matches, vec![0, 1, 3]);
```

#### Trait Implementations

##### `impl Debug for SetMatchesIntoIter`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl DoubleEndedIterator for SetMatchesIntoIter`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl FusedIterator for SetMatchesIntoIter`

##### `impl<I> IntoIterator for SetMatchesIntoIter`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl Iterator for SetMatchesIntoIter`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

### `SetMatchesIter<'a>`

```rust
struct SetMatchesIter<'a>(regex_automata::PatternSetIter<'a>);
```

A borrowed iterator over the set of matches from a regex set.

The lifetime `'a` refers to the lifetime of the [`SetMatches`](../../index.md) value that
created this iterator.

This will always produces matches in ascending order, where the index
corresponds to the index of the regex that matched with respect to its
position when initially building the set.

This iterator is created by the `SetMatches::iter` method.

#### Trait Implementations

##### `impl<'a> Clone for SetMatchesIter<'a>`

- `fn clone(self: &Self) -> SetMatchesIter<'a>` — [`SetMatchesIter`](../../index.md)

##### `impl<'a> Debug for SetMatchesIter<'a>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'a> DoubleEndedIterator for SetMatchesIter<'a>`

- `fn next_back(self: &mut Self) -> Option<usize>`

##### `impl<'a> FusedIterator for SetMatchesIter<'a>`

##### `impl<I> IntoIterator for SetMatchesIter<'a>`

- `type Item = <I as Iterator>::Item`

- `type IntoIter = I`

- `fn into_iter(self: Self) -> I`

##### `impl<'a> Iterator for SetMatchesIter<'a>`

- `type Item = usize`

- `fn next(self: &mut Self) -> Option<usize>`

- `fn size_hint(self: &Self) -> (usize, Option<usize>)`

