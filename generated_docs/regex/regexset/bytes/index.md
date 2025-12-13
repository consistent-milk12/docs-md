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

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:136-139`](../../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L136-L139)*

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

- <span id="regexset-new"></span>`fn new<I, S>(exprs: I) -> Result<RegexSet, Error>` — [`RegexSet`](#regexset), [`Error`](../../error/index.md#error)

  Create a new regex set with the given regular expressions.

  

  This takes an iterator of `S`, where `S` is something that can produce

  a `&str`. If any of the strings in the iterator are not valid regular

  expressions, then an error is returned.

  

  # Example

  

  Create a new regex set from an iterator of strings:

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();

  assert!(set.is_match(b"foo"));

  ```

- <span id="regexset-empty"></span>`fn empty() -> RegexSet` — [`RegexSet`](#regexset)

  Create a new empty regex set.

  

  An empty regex never matches anything.

  

  This is a convenience function for `RegexSet::new([])`, but doesn't

  require one to specify the type of the input.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::empty();

  assert!(set.is_empty());

  // an empty set matches nothing

  assert!(!set.is_match(b""));

  ```

- <span id="regexset-is-match"></span>`fn is_match(&self, haystack: &[u8]) -> bool`

  Returns true if and only if one of the regexes in this set matches

  the haystack given.

  

  This method should be preferred if you only need to test whether any

  of the regexes in the set should match, but don't care about *which*

  regexes matched. This is because the underlying matching engine will

  quit immediately after seeing the first match instead of continuing to

  find all matches.

  

  Note that as with searches using [`Regex`](crate::bytes::Regex), the

  expression is unanchored by default. That is, if the regex does not

  start with `^` or `\A`, or end with `$` or `\z`, then it is permitted

  to match anywhere in the haystack.

  

  # Example

  

  Tests whether a set matches somewhere in a haystack:

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();

  assert!(set.is_match(b"foo"));

  assert!(!set.is_match("☃".as_bytes()));

  ```

- <span id="regexset-is-match-at"></span>`fn is_match_at(&self, haystack: &[u8], start: usize) -> bool`

  Returns true if and only if one of the regexes in this set matches the

  haystack given, with the search starting at the offset given.

  

  The significance of the starting point is that it takes the surrounding

  context into consideration. For example, the `\A` anchor can only

  match when `start == 0`.

  

  # Panics

  

  This panics when `start >= haystack.len() + 1`.

  

  # Example

  

  This example shows the significance of `start`. Namely, consider a

  haystack `foobar` and a desire to execute a search starting at offset

  `3`. You could search a substring explicitly, but then the look-around

  assertions won't work correctly. Instead, you can use this method to

  specify the start position of a search.

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([r"\bbar\b", r"(?m)^bar$"]).unwrap();

  let hay = b"foobar";

  // We get a match here, but it's probably not intended.

  assert!(set.is_match(&hay[3..]));

  // No match because the  assertions take the context into account.

  assert!(!set.is_match_at(hay, 3));

  ```

- <span id="regexset-matches"></span>`fn matches(&self, haystack: &[u8]) -> SetMatches` — [`SetMatches`](#setmatches)

  Returns the set of regexes that match in the given haystack.

  

  The set returned contains the index of each regex that matches in

  the given haystack. The index is in correspondence with the order of

  regular expressions given to `RegexSet`'s constructor.

  

  The set can also be used to iterate over the matched indices. The order

  of iteration is always ascending with respect to the matching indices.

  

  Note that as with searches using [`Regex`](crate::bytes::Regex), the

  expression is unanchored by default. That is, if the regex does not

  start with `^` or `\A`, or end with `$` or `\z`, then it is permitted

  to match anywhere in the haystack.

  

  # Example

  

  Tests which regular expressions match the given haystack:

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([

      r"\w+",

      r"\d+",

      r"\pL+",

      r"foo",

      r"bar",

      r"barfoo",

      r"foobar",

  ]).unwrap();

  let matches: Vec<_> = set.matches(b"foobar").into_iter().collect();

  assert_eq!(matches, vec![0, 2, 3, 4, 6]);

  

  // You can also test whether a particular regex matched:

  let matches = set.matches(b"foobar");

  assert!(!matches.matched(5));

  assert!(matches.matched(6));

  ```

- <span id="regexset-matches-at"></span>`fn matches_at(&self, haystack: &[u8], start: usize) -> SetMatches` — [`SetMatches`](#setmatches)

  Returns the set of regexes that match in the given haystack.

  

  The set returned contains the index of each regex that matches in

  the given haystack. The index is in correspondence with the order of

  regular expressions given to `RegexSet`'s constructor.

  

  The set can also be used to iterate over the matched indices. The order

  of iteration is always ascending with respect to the matching indices.

  

  The significance of the starting point is that it takes the surrounding

  context into consideration. For example, the `\A` anchor can only

  match when `start == 0`.

  

  # Panics

  

  This panics when `start >= haystack.len() + 1`.

  

  # Example

  

  Tests which regular expressions match the given haystack:

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([r"\bbar\b", r"(?m)^bar$"]).unwrap();

  let hay = b"foobar";

  // We get matches here, but it's probably not intended.

  let matches: Vec<_> = set.matches(&hay[3..]).into_iter().collect();

  assert_eq!(matches, vec![0, 1]);

  // No matches because the  assertions take the context into account.

  let matches: Vec<_> = set.matches_at(hay, 3).into_iter().collect();

  assert_eq!(matches, vec![]);

  ```

- <span id="regexset-len"></span>`fn len(&self) -> usize`

  Returns the total number of regexes in this set.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  assert_eq!(0, RegexSet::empty().len());

  assert_eq!(1, RegexSet::new([r"[0-9]"]).unwrap().len());

  assert_eq!(2, RegexSet::new([r"[0-9]", r"[a-z]"]).unwrap().len());

  ```

- <span id="regexset-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if this set contains no regexes.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  assert!(RegexSet::empty().is_empty());

  assert!(!RegexSet::new([r"[0-9]"]).unwrap().is_empty());

  ```

- <span id="regexset-patterns"></span>`fn patterns(&self) -> &[String]`

  Returns the regex patterns that this regex set was constructed from.

  

  This function can be used to determine the pattern for a match. The

  slice returned has exactly as many patterns givens to this regex set,

  and the order of the slice is the same as the order of the patterns

  provided to the set.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new(&[

      r"\w+",

      r"\d+",

      r"\pL+",

      r"foo",

      r"bar",

      r"barfoo",

      r"foobar",

  ]).unwrap();

  let matches: Vec<_> = set

      .matches(b"foobar")

      .into_iter()

      .map(|index| &set.patterns()[index])

      .collect();

  assert_eq!(matches, vec![r"\w+", r"\pL+", r"foo", r"bar", r"foobar"]);

  ```

#### Trait Implementations

##### `impl Any for RegexSet`

- <span id="regexset-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegexSet`

- <span id="regexset-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegexSet`

- <span id="regexset-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RegexSet`

- <span id="regexset-clone"></span>`fn clone(&self) -> RegexSet` — [`RegexSet`](#regexset)

##### `impl CloneToUninit for RegexSet`

- <span id="regexset-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RegexSet`

- <span id="regexset-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for RegexSet`

- <span id="regexset-default"></span>`fn default() -> Self`

##### `impl<T> From for RegexSet`

- <span id="regexset-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegexSet`

- <span id="regexset-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RegexSet`

- <span id="regexset-toowned-type-owned"></span>`type Owned = T`

- <span id="regexset-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regexset-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RegexSet`

- <span id="regexset-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regexset-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegexSet`

- <span id="regexset-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regexset-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SetMatches`

```rust
struct SetMatches(regex_automata::PatternSet);
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:463`](../../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L463)*

A set of matches returned by a regex set.

Values of this type are constructed by `RegexSet::matches`.

#### Implementations

- <span id="setmatches-matched-any"></span>`fn matched_any(&self) -> bool`

  Whether this set contains any matches.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new(&[

      r"[a-z]+@[a-z]+\.(com|org|net)",

      r"[a-z]+\.(com|org|net)",

  ]).unwrap();

  let matches = set.matches(b"foo@example.com");

  assert!(matches.matched_any());

  ```

- <span id="setmatches-matched-all"></span>`fn matched_all(&self) -> bool`

  Whether all patterns in this set matched.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new(&[

      r"^foo",

      r"[a-z]+\.com",

  ]).unwrap();

  let matches = set.matches(b"foo.example.com");

  assert!(matches.matched_all());

  ```

- <span id="setmatches-matched"></span>`fn matched(&self, index: usize) -> bool`

  Whether the regex at the given index matched.

  

  The index for a regex is determined by its insertion order upon the

  initial construction of a `RegexSet`, starting at `0`.

  

  # Panics

  

  If `index` is greater than or equal to the number of regexes in the

  original set that produced these matches. Equivalently, when `index`

  is greater than or equal to `SetMatches::len`.

  

  # Example

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([

      r"[a-z]+@[a-z]+\.(com|org|net)",

      r"[a-z]+\.(com|org|net)",

  ]).unwrap();

  let matches = set.matches(b"example.com");

  assert!(!matches.matched(0));

  assert!(matches.matched(1));

  ```

- <span id="setmatches-len"></span>`fn len(&self) -> usize`

  The total number of regexes in the set that created these matches.

  

  **WARNING:** This always returns the same value as `RegexSet::len`.

  In particular, it does *not* return the number of elements yielded by

  `SetMatches::iter`. The only way to determine the total number of

  matched regexes is to iterate over them.

  

  # Example

  

  Notice that this method returns the total number of regexes in the

  original set, and *not* the total number of regexes that matched.

  

  ```rust

  use regex::bytes::RegexSet;

  

  let set = RegexSet::new([

      r"[a-z]+@[a-z]+\.(com|org|net)",

      r"[a-z]+\.(com|org|net)",

  ]).unwrap();

  let matches = set.matches(b"example.com");

  // Total number of patterns that matched.

  assert_eq!(1, matches.iter().count());

  // Total number of patterns in the set.

  assert_eq!(2, matches.len());

  ```

- <span id="setmatches-iter"></span>`fn iter(&self) -> SetMatchesIter<'_>` — [`SetMatchesIter`](#setmatchesiter)

  Returns an iterator over the indices of the regexes that matched.

  

  This will always produces matches in ascending order, where the index

  yielded corresponds to the index of the regex that matched with respect

  to its position when initially building the set.

  

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

  let matches: Vec<_> = set.matches(hay).iter().collect();

  assert_eq!(matches, vec![0, 1, 3]);

  ```

  

  Note that `SetMatches` also implements the `IntoIterator` trait, so

  this method is not always needed. For example:

  

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

##### `impl Any for SetMatches`

- <span id="setmatches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SetMatches`

- <span id="setmatches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SetMatches`

- <span id="setmatches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SetMatches`

- <span id="setmatches-clone"></span>`fn clone(&self) -> SetMatches` — [`SetMatches`](#setmatches)

##### `impl CloneToUninit for SetMatches`

- <span id="setmatches-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SetMatches`

- <span id="setmatches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SetMatches`

- <span id="setmatches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for SetMatches`

- <span id="setmatches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SetMatches`

- <span id="setmatches-intoiterator-type-intoiter"></span>`type IntoIter = SetMatchesIntoIter`

- <span id="setmatches-intoiterator-type-item"></span>`type Item = usize`

- <span id="setmatches-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl ToOwned for SetMatches`

- <span id="setmatches-toowned-type-owned"></span>`type Owned = T`

- <span id="setmatches-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="setmatches-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SetMatches`

- <span id="setmatches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setmatches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SetMatches`

- <span id="setmatches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setmatches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SetMatchesIntoIter`

```rust
struct SetMatchesIntoIter {
    patset: regex_automata::PatternSet,
    it: core::ops::Range<usize>,
}
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:656-659`](../../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L656-L659)*

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

##### `impl Any for SetMatchesIntoIter`

- <span id="setmatchesintoiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SetMatchesIntoIter`

- <span id="setmatchesintoiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SetMatchesIntoIter`

- <span id="setmatchesintoiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SetMatchesIntoIter`

- <span id="setmatchesintoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for SetMatchesIntoIter`

- <span id="setmatchesintoiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for SetMatchesIntoIter`

##### `impl<U> Into for SetMatchesIntoIter`

- <span id="setmatchesintoiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="setmatchesintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="setmatchesintoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-iterator-type-item"></span>`type Item = usize`

- <span id="setmatchesintoiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="setmatchesintoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for SetMatchesIntoIter`

- <span id="setmatchesintoiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setmatchesintoiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SetMatchesIntoIter`

- <span id="setmatchesintoiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setmatchesintoiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SetMatchesIter<'a>`

```rust
struct SetMatchesIter<'a>(regex_automata::PatternSetIter<'a>);
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:702`](../../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L702)*

A borrowed iterator over the set of matches from a regex set.

The lifetime `'a` refers to the lifetime of the [`SetMatches`](#setmatches) value that
created this iterator.

This will always produces matches in ascending order, where the index
corresponds to the index of the regex that matched with respect to its
position when initially building the set.

This iterator is created by the `SetMatches::iter` method.

#### Trait Implementations

##### `impl Any for SetMatchesIter<'a>`

- <span id="setmatchesiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SetMatchesIter<'a>`

- <span id="setmatchesiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SetMatchesIter<'a>`

- <span id="setmatchesiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SetMatchesIter<'a>`

- <span id="setmatchesiter-clone"></span>`fn clone(&self) -> SetMatchesIter<'a>` — [`SetMatchesIter`](#setmatchesiter)

##### `impl CloneToUninit for SetMatchesIter<'a>`

- <span id="setmatchesiter-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SetMatchesIter<'a>`

- <span id="setmatchesiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl<T> From for SetMatchesIter<'a>`

- <span id="setmatchesiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for SetMatchesIter<'a>`

##### `impl<U> Into for SetMatchesIter<'a>`

- <span id="setmatchesiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="setmatchesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="setmatchesiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-iterator-type-item"></span>`type Item = usize`

- <span id="setmatchesiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="setmatchesiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl ToOwned for SetMatchesIter<'a>`

- <span id="setmatchesiter-toowned-type-owned"></span>`type Owned = T`

- <span id="setmatchesiter-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="setmatchesiter-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SetMatchesIter<'a>`

- <span id="setmatchesiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="setmatchesiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SetMatchesIter<'a>`

- <span id="setmatchesiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="setmatchesiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

