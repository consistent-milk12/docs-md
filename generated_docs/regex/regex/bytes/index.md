*[regex](../../index.md) / [regex](../index.md) / [bytes](index.md)*

---

# Module `bytes`

## Contents

- [Structs](#structs)
  - [`Regex`](#regex)
  - [`Match`](#match)
  - [`Captures`](#captures)
  - [`CaptureLocations`](#capturelocations)
  - [`Matches`](#matches)
  - [`CaptureMatches`](#capturematches)
  - [`Split`](#split)
  - [`SplitN`](#splitn)
  - [`CaptureNames`](#capturenames)
  - [`SubCaptureMatches`](#subcapturematches)
  - [`ReplacerRef`](#replacerref)
  - [`NoExpand`](#noexpand)
- [Traits](#traits)
  - [`Replacer`](#replacer)
- [Functions](#functions)
  - [`no_expansion`](#no_expansion)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Regex`](#regex) | struct | A compiled regular expression for searching Unicode haystacks. |
| [`Match`](#match) | struct | Represents a single match of a regex in a haystack. |
| [`Captures`](#captures) | struct | Represents the capture groups for a single match. |
| [`CaptureLocations`](#capturelocations) | struct | A low level representation of the byte offsets of each capture group. |
| [`Matches`](#matches) | struct | An iterator over all non-overlapping matches in a haystack. |
| [`CaptureMatches`](#capturematches) | struct | An iterator over all non-overlapping capture matches in a haystack. |
| [`Split`](#split) | struct | An iterator over all substrings delimited by a regex match. |
| [`SplitN`](#splitn) | struct | An iterator over at most `N` substrings delimited by a regex match. |
| [`CaptureNames`](#capturenames) | struct | An iterator over the names of all capture groups in a regex. |
| [`SubCaptureMatches`](#subcapturematches) | struct | An iterator over all group matches in a [`Captures`] value. |
| [`ReplacerRef`](#replacerref) | struct | A by-reference adaptor for a [`Replacer`]. |
| [`NoExpand`](#noexpand) | struct | A helper type for forcing literal string replacement. |
| [`Replacer`](#replacer) | trait | A trait for types that can be used to replace matches in a haystack. |
| [`no_expansion`](#no_expansion) | fn | Quickly checks the given replacement string for whether interpolation should be done on it. |

## Structs

### `Regex`

```rust
struct Regex {
    meta: meta::Regex,
    pattern: alloc::sync::Arc<str>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:99-102`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L99-L102)*

A compiled regular expression for searching Unicode haystacks.

A `Regex` can be used to search haystacks, split haystacks into substrings
or replace substrings in a haystack with a different substring. All
searching is done with an implicit `(?s:.)*?` at the beginning and end of
an pattern. To force an expression to match the whole string (or a prefix
or a suffix), you must use an anchor like `^` or `$` (or `\A` and `\z`).

Like the `Regex` type in the parent module, matches with this regex return
byte offsets into the haystack. **Unlike** the parent `Regex` type, these
byte offsets may not correspond to UTF-8 sequence boundaries since the
regexes in this module can match arbitrary bytes.

The only methods that allocate new byte strings are the string replacement
methods. All other methods (searching and splitting) return borrowed
references into the haystack given.

# Example

Find the offsets of a US phone number:

```rust
use regex::bytes::Regex;

let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
let m = re.find(b"phone: 111-222-3333").unwrap();
assert_eq!(7..19, m.range());
```

# Example: extracting capture groups

A common way to use regexes is with capture groups. That is, instead of
just looking for matches of an entire regex, parentheses are used to create
groups that represent part of the match.

For example, consider a haystack with multiple lines, and each line has
three whitespace delimited fields where the second field is expected to be
a number and the third field a boolean. To make this convenient, we use
the `Captures::extract` API to put the strings that match each group
into a fixed size array:

```rust
use regex::bytes::Regex;

let hay = b"
rabbit         54 true
groundhog 2 true
does not match
fox   109    false
";
let re = Regex::new(r"(?m)^\s*(\S+)\s+([0-9]+)\s+(true|false)\s*$").unwrap();
let mut fields: Vec<(&[u8], i64, bool)> = vec![];
for (_, [f1, f2, f3]) in re.captures_iter(hay).map(|caps| caps.extract()) {
    // These unwraps are OK because our pattern is written in a way where
    // all matches for f2 and f3 will be valid UTF-8.
    let f2 = std::str::from_utf8(f2).unwrap();
    let f3 = std::str::from_utf8(f3).unwrap();
    fields.push((f1, f2.parse()?, f3.parse()?));
}
assert_eq!(fields, vec![
    (&b"rabbit"[..], 54, true),
    (&b"groundhog"[..], 2, true),
    (&b"fox"[..], 109, false),
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: matching invalid UTF-8

One of the reasons for searching `&[u8]` haystacks is that the `&[u8]`
might not be valid UTF-8. Indeed, with a `bytes::Regex`, patterns that
match invalid UTF-8 are explicitly allowed. Here's one example that looks
for valid UTF-8 fields that might be separated by invalid UTF-8. In this
case, we use `(?s-u:.)`, which matches any byte. Attempting to use it in a
top-level `Regex` will result in the regex failing to compile. Notice also
that we use `.` with Unicode mode enabled, in which case, only valid UTF-8
is matched. In this way, we can build one pattern where some parts only
match valid UTF-8 while other parts are more permissive.

```rust
use regex::bytes::Regex;

// F0 9F 92 A9 is the UTF-8 encoding for a Pile of Poo.
let hay = b"\xFF\xFFfoo\xFF\xFF\xFF\xF0\x9F\x92\xA9\xFF";
// An equivalent to '(?s-u:.)' is '(?-u:[\x00-\xFF])'.
let re = Regex::new(r"(?s)(?-u:.)*?(?<f1>.+)(?-u:.)*?(?<f2>.+)").unwrap();
let caps = re.captures(hay).unwrap();
assert_eq!(&caps["f1"], &b"foo"[..]);
assert_eq!(&caps["f2"], "💩".as_bytes());
```

#### Implementations

- <span id="regex-new"></span>`fn new(re: &str) -> Result<Regex, Error>` — [`Regex`](#regex), [`Error`](../../error/index.md#error)

- <span id="regex-is-match"></span>`fn is_match(&self, haystack: &[u8]) -> bool`

- <span id="regex-find"></span>`fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>>` — [`Match`](#match)

- <span id="regex-find-iter"></span>`fn find_iter<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> Matches<'r, 'h>` — [`Matches`](#matches)

- <span id="regex-captures"></span>`fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>>` — [`Captures`](#captures)

- <span id="regex-captures-iter"></span>`fn captures_iter<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> CaptureMatches<'r, 'h>` — [`CaptureMatches`](#capturematches)

- <span id="regex-split"></span>`fn split<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> Split<'r, 'h>` — [`Split`](#split)

- <span id="regex-splitn"></span>`fn splitn<'r, 'h>(self: &'r Self, haystack: &'h [u8], limit: usize) -> SplitN<'r, 'h>` — [`SplitN`](#splitn)

- <span id="regex-replace"></span>`fn replace<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>`

- <span id="regex-replace-all"></span>`fn replace_all<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>`

- <span id="regex-replacen"></span>`fn replacen<'h, R: Replacer>(&self, haystack: &'h [u8], limit: usize, rep: R) -> Cow<'h, [u8]>`

#### Trait Implementations

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](#regex)

##### `impl Debug for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl FromStr for Regex`

- <span id="regex-type-err"></span>`type Err = Error`

- <span id="regex-from-str"></span>`fn from_str(s: &str) -> Result<Regex, Error>` — [`Regex`](#regex), [`Error`](../../error/index.md#error)

##### `impl ToString for Regex`

- <span id="regex-to-string"></span>`fn to_string(&self) -> String`

### `Match<'h>`

```rust
struct Match<'h> {
    haystack: &'h [u8],
    start: usize,
    end: usize,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:1483-1487`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L1483-L1487)*

Represents a single match of a regex in a haystack.

A `Match` contains both the start and end byte offsets of the match and the
actual substring corresponding to the range of those byte offsets. It is
guaranteed that `start <= end`. When `start == end`, the match is empty.

Unlike the top-level `Match` type, this `Match` type is produced by APIs
that search `&[u8]` haystacks. This means that the offsets in a `Match` can
point to anywhere in the haystack, including in a place that splits the
UTF-8 encoding of a Unicode scalar value.

The lifetime parameter `'h` refers to the lifetime of the matched of the
haystack that this match was produced from.

# Numbering

The byte offsets in a `Match` form a half-open interval. That is, the
start of the range is inclusive and the end of the range is exclusive.
For example, given a haystack `abcFOOxyz` and a match of `FOO`, its byte
offset range starts at `3` and ends at `6`. `3` corresponds to `F` and
`6` corresponds to `x`, which is one past the end of the match. This
corresponds to the same kind of slicing that Rust uses.

For more on why this was chosen over other schemes (aside from being
consistent with how Rust the language works), see [this discussion] and
[Dijkstra's note on a related topic][`note`](../../../object/read/elf/note/index.md).


# Example

This example shows the value of each of the methods on `Match` for a
particular search.

```rust
use regex::bytes::Regex;

let re = Regex::new(r"\p{Greek}+").unwrap();
let hay = "Greek: αβγδ".as_bytes();
let m = re.find(hay).unwrap();
assert_eq!(7, m.start());
assert_eq!(15, m.end());
assert!(!m.is_empty());
assert_eq!(8, m.len());
assert_eq!(7..15, m.range());
assert_eq!("αβγδ".as_bytes(), m.as_bytes());
```

#### Implementations

- <span id="match-start"></span>`fn start(&self) -> usize`

- <span id="match-end"></span>`fn end(&self) -> usize`

- <span id="match-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="match-len"></span>`fn len(&self) -> usize`

- <span id="match-range"></span>`fn range(&self) -> core::ops::Range<usize>`

- <span id="match-as-bytes"></span>`fn as_bytes(&self) -> &'h [u8]`

- <span id="match-new"></span>`fn new(haystack: &'h [u8], start: usize, end: usize) -> Match<'h>` — [`Match`](#match)

#### Trait Implementations

##### `impl Clone for Match<'h>`

- <span id="match-clone"></span>`fn clone(&self) -> Match<'h>` — [`Match`](#match)

##### `impl Copy for Match<'h>`

##### `impl Debug for Match<'h>`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Match<'h>`

##### `impl PartialEq for Match<'h>`

- <span id="match-eq"></span>`fn eq(&self, other: &Match<'h>) -> bool` — [`Match`](#match)

##### `impl StructuralPartialEq for Match<'h>`

### `Captures<'h>`

```rust
struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:1632-1636`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L1632-L1636)*

Represents the capture groups for a single match.

Capture groups refer to parts of a regex enclosed in parentheses. They
can be optionally named. The purpose of capture groups is to be able to
reference different parts of a match based on the original pattern. In
essence, a `Captures` is a container of [`Match`](#match) values for each group
that participated in a regex match. Each `Match` can be looked up by either
its capture group index or name (if it has one).

For example, say you want to match the individual letters in a 5-letter
word:

```text
(?<first>\w)(\w)(?:\w)\w(?<last>\w)
```

This regex has 4 capture groups:

* The group at index `0` corresponds to the overall match. It is always
present in every match and never has a name.
* The group at index `1` with name `first` corresponding to the first
letter.
* The group at index `2` with no name corresponding to the second letter.
* The group at index `3` with name `last` corresponding to the fifth and
last letter.

Notice that `(?:\w)` was not listed above as a capture group despite it
being enclosed in parentheses. That's because `(?:pattern)` is a special
syntax that permits grouping but *without* capturing. The reason for not
treating it as a capture is that tracking and reporting capture groups
requires additional state that may lead to slower searches. So using as few
capture groups as possible can help performance. (Although the difference
in performance of a couple of capture groups is likely immaterial.)

Values with this type are created by `Regex::captures` or
`Regex::captures_iter`.

`'h` is the lifetime of the haystack that these captures were matched from.

# Example

```rust
use regex::bytes::Regex;

let re = Regex::new(r"(?<first>\w)(\w)(?:\w)\w(?<last>\w)").unwrap();
let caps = re.captures(b"toady").unwrap();
assert_eq!(b"toady", &caps[0]);
assert_eq!(b"t", &caps["first"]);
assert_eq!(b"o", &caps[2]);
assert_eq!(b"y", &caps["last"]);
```

#### Implementations

- <span id="captures-get"></span>`fn get(&self, i: usize) -> Option<Match<'h>>` — [`Match`](#match)

- <span id="captures-get-match"></span>`fn get_match(&self) -> Match<'h>` — [`Match`](#match)

- <span id="captures-name"></span>`fn name(&self, name: &str) -> Option<Match<'h>>` — [`Match`](#match)

- <span id="captures-extract"></span>`fn extract<const N: usize>(&self) -> (&'h [u8], [&'h [u8]; N])`

- <span id="captures-expand"></span>`fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>)`

- <span id="captures-iter"></span>`fn iter<'c>(self: &'c Self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](#subcapturematches)

- <span id="captures-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Debug for Captures<'h>`

- <span id="captures-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Index for Captures<'h>`

- <span id="captures-type-output"></span>`type Output = [u8]`

- <span id="captures-index"></span>`fn index<'a>(self: &'a Self, i: usize) -> &'a [u8]`

### `CaptureLocations`

```rust
struct CaptureLocations(captures::Captures);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2084`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2084)*

A low level representation of the byte offsets of each capture group.

You can think of this as a lower level [`Captures`](#captures), where this type does
not support named capturing groups directly and it does not borrow the
haystack that these offsets were matched on.

Primarily, this type is useful when using the lower level `Regex` APIs such
as `Regex::captures_read`, which permits amortizing the allocation in
which capture match offsets are stored.

In order to build a value of this type, you'll need to call the
`Regex::capture_locations` method. The value returned can then be reused
in subsequent searches for that regex. Using it for other regexes may
result in a panic or otherwise incorrect results.

# Example

This example shows how to create and use `CaptureLocations` in a search.

```rust
use regex::bytes::Regex;

let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
let mut locs = re.capture_locations();
let m = re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();
assert_eq!(0..17, m.range());
assert_eq!(Some((0, 17)), locs.get(0));
assert_eq!(Some((0, 5)), locs.get(1));
assert_eq!(Some((6, 17)), locs.get(2));

// Asking for an invalid capture group always returns None.
assert_eq!(None, locs.get(3));
// literals are too big for 32-bit usize: #1041
#[cfg(target_pointer_width = "64")]
assert_eq!(None, locs.get(34973498648));
#[cfg(target_pointer_width = "64")]
assert_eq!(None, locs.get(9944060567225171988));
```

#### Implementations

- <span id="capturelocations-get"></span>`fn get(&self, i: usize) -> Option<(usize, usize)>`

- <span id="capturelocations-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Clone for CaptureLocations`

- <span id="capturelocations-clone"></span>`fn clone(&self) -> CaptureLocations` — [`CaptureLocations`](#capturelocations)

##### `impl Debug for CaptureLocations`

- <span id="capturelocations-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Matches<'r, 'h>`

```rust
struct Matches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::FindMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2184-2187`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2184-L2187)*

An iterator over all non-overlapping matches in a haystack.

This iterator yields [`Match`](#match) values. The iterator stops when no more
matches can be found.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the haystack.

This iterator is created by `Regex::find_iter`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

#### Trait Implementations

##### `impl Debug for Matches<'r, 'h>`

- <span id="matches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for Matches<'r, 'h>`

##### `impl IntoIterator for Matches<'r, 'h>`

- <span id="matches-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matches-type-intoiter"></span>`type IntoIter = I`

- <span id="matches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Matches<'r, 'h>`

- <span id="matches-type-item"></span>`type Item = Match<'h>`

- <span id="matches-next"></span>`fn next(&mut self) -> Option<Match<'h>>` — [`Match`](#match)

- <span id="matches-count"></span>`fn count(self) -> usize`

### `CaptureMatches<'r, 'h>`

```rust
struct CaptureMatches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::CapturesMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2228-2231`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2228-L2231)*

An iterator over all non-overlapping capture matches in a haystack.

This iterator yields [`Captures`](#captures) values. The iterator stops when no more
matches can be found.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the matched string.

This iterator is created by `Regex::captures_iter`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

#### Trait Implementations

##### `impl Debug for CaptureMatches<'r, 'h>`

- <span id="capturematches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for CaptureMatches<'r, 'h>`

##### `impl IntoIterator for CaptureMatches<'r, 'h>`

- <span id="capturematches-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturematches-type-intoiter"></span>`type IntoIter = I`

- <span id="capturematches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CaptureMatches<'r, 'h>`

- <span id="capturematches-type-item"></span>`type Item = Captures<'h>`

- <span id="capturematches-next"></span>`fn next(&mut self) -> Option<Captures<'h>>` — [`Captures`](#captures)

- <span id="capturematches-count"></span>`fn count(self) -> usize`

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    haystack: &'h [u8],
    it: meta::Split<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2272-2275`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2272-L2275)*

An iterator over all substrings delimited by a regex match.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the byte string being split.

This iterator is created by `Regex::split`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

#### Trait Implementations

##### `impl Debug for Split<'r, 'h>`

- <span id="split-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for Split<'r, 'h>`

##### `impl IntoIterator for Split<'r, 'h>`

- <span id="split-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-type-intoiter"></span>`type IntoIter = I`

- <span id="split-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'r, 'h>`

- <span id="split-type-item"></span>`type Item = &'h [u8]`

- <span id="split-next"></span>`fn next(&mut self) -> Option<&'h [u8]>`

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    haystack: &'h [u8],
    it: meta::SplitN<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2307-2310`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2307-L2310)*

An iterator over at most `N` substrings delimited by a regex match.

The last substring yielded by this iterator will be whatever remains after
`N-1` splits.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the byte string being split.

This iterator is created by `Regex::splitn`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

Although note that the worst case time here has an upper bound given
by the `limit` parameter to `Regex::splitn`.

#### Trait Implementations

##### `impl Debug for SplitN<'r, 'h>`

- <span id="splitn-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FusedIterator for SplitN<'r, 'h>`

##### `impl IntoIterator for SplitN<'r, 'h>`

- <span id="splitn-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splitn-type-intoiter"></span>`type IntoIter = I`

- <span id="splitn-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SplitN<'r, 'h>`

- <span id="splitn-type-item"></span>`type Item = &'h [u8]`

- <span id="splitn-next"></span>`fn next(&mut self) -> Option<&'h [u8]>`

- <span id="splitn-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `CaptureNames<'r>`

```rust
struct CaptureNames<'r>(captures::GroupInfoPatternNames<'r>);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2339`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2339)*

An iterator over the names of all capture groups in a regex.

This iterator yields values of type `Option<&str>` in order of the opening
capture group parenthesis in the regex pattern. `None` is yielded for
groups with no name. The first element always corresponds to the implicit
and unnamed group for the overall match.

`'r` is the lifetime of the compiled regular expression.

This iterator is created by `Regex::capture_names`.

#### Trait Implementations

##### `impl Clone for CaptureNames<'r>`

- <span id="capturenames-clone"></span>`fn clone(&self) -> CaptureNames<'r>` — [`CaptureNames`](#capturenames)

##### `impl Debug for CaptureNames<'r>`

- <span id="capturenames-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for CaptureNames<'r>`

##### `impl FusedIterator for CaptureNames<'r>`

##### `impl IntoIterator for CaptureNames<'r>`

- <span id="capturenames-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturenames-type-intoiter"></span>`type IntoIter = I`

- <span id="capturenames-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CaptureNames<'r>`

- <span id="capturenames-type-item"></span>`type Item = Option<&'r str>`

- <span id="capturenames-next"></span>`fn next(&mut self) -> Option<Option<&'r str>>`

- <span id="capturenames-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="capturenames-count"></span>`fn count(self) -> usize`

### `SubCaptureMatches<'c, 'h>`

```rust
struct SubCaptureMatches<'c, 'h> {
    haystack: &'h [u8],
    it: captures::CapturesPatternIter<'c>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2381-2384`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2381-L2384)*

An iterator over all group matches in a [`Captures`](#captures) value.

This iterator yields values of type `Option<Match<'h>>`, where `'h` is the
lifetime of the haystack that the matches are for. The order of elements
yielded corresponds to the order of the opening parenthesis for the group
in the regex pattern. `None` is yielded for groups that did not participate
in the match.

The first element always corresponds to the implicit group for the overall
match. Since this iterator is created by a [`Captures`](#captures) value, and a
`Captures` value is only created when a match occurs, it follows that the
first element yielded by this iterator is guaranteed to be non-`None`.

The lifetime `'c` corresponds to the lifetime of the `Captures` value that
created this iterator, and the lifetime `'h` corresponds to the originally
matched haystack.

#### Trait Implementations

##### `impl Clone for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-clone"></span>`fn clone(&self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](#subcapturematches)

##### `impl Debug for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for SubCaptureMatches<'c, 'h>`

##### `impl FusedIterator for SubCaptureMatches<'c, 'h>`

##### `impl IntoIterator for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="subcapturematches-type-intoiter"></span>`type IntoIter = I`

- <span id="subcapturematches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-type-item"></span>`type Item = Option<Match<'h>>`

- <span id="subcapturematches-next"></span>`fn next(&mut self) -> Option<Option<Match<'h>>>` — [`Match`](#match)

- <span id="subcapturematches-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="subcapturematches-count"></span>`fn count(self) -> usize`

### `ReplacerRef<'a, R: ?Sized>`

```rust
struct ReplacerRef<'a, R: ?Sized>(&'a mut R);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2579`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2579)*

A by-reference adaptor for a [`Replacer`](#replacer).

This permits reusing the same `Replacer` value in multiple calls to a
replacement routine like `Regex::replace_all`.

This type is created by `Replacer::by_ref`.

#### Trait Implementations

##### `impl<'a, R: fmt::Debug + ?Sized> Debug for ReplacerRef<'a, R>`

- <span id="replacerref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'a, R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R>`

- <span id="replacerref-replace-append"></span>`fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>)` — [`Captures`](#captures)

- <span id="replacerref-no-expansion"></span>`fn no_expansion<'r>(self: &'r mut Self) -> Option<Cow<'r, [u8]>>`

### `NoExpand<'s>`

```rust
struct NoExpand<'s>(&'s [u8]);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2611`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2611)*

A helper type for forcing literal string replacement.

It can be used with routines like `Regex::replace` and
`Regex::replace_all` to do a literal string replacement without expanding
`$name` to their corresponding capture groups. This can be both convenient
(to avoid escaping `$`, for example) and faster (since capture groups
don't need to be found).

`'s` is the lifetime of the literal string to use.

# Example

```rust
use regex::bytes::{NoExpand, Regex};

let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();
let result = re.replace(b"Springsteen, Bruce", NoExpand(b"$2 $last"));
assert_eq!(result, &b"$2 $last"[..]);
```

#### Trait Implementations

##### `impl Clone for NoExpand<'s>`

- <span id="noexpand-clone"></span>`fn clone(&self) -> NoExpand<'s>` — [`NoExpand`](#noexpand)

##### `impl Debug for NoExpand<'s>`

- <span id="noexpand-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Replacer for NoExpand<'s>`

- <span id="noexpand-replace-append"></span>`fn replace_append(&mut self, _: &Captures<'_>, dst: &mut Vec<u8>)` — [`Captures`](#captures)

- <span id="noexpand-no-expansion"></span>`fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>>`

## Traits

### `Replacer`

```rust
trait Replacer { ... }
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2443-2490`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2443-L2490)*

A trait for types that can be used to replace matches in a haystack.

In general, users of this crate shouldn't need to implement this trait,
since implementations are already provided for `&[u8]` along with other
variants of byte string types, as well as `FnMut(&Captures) -> Vec<u8>` (or
any `FnMut(&Captures) -> T` where `T: AsRef<[u8]>`). Those cover most use
cases, but callers can implement this trait directly if necessary.

# Example

This example shows a basic implementation of the `Replacer` trait. This can
be done much more simply using the replacement byte string interpolation
support (e.g., `$first $last`), but this approach avoids needing to parse
the replacement byte string at all.

```rust
use regex::bytes::{Captures, Regex, Replacer};

struct NameSwapper;

impl Replacer for NameSwapper {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>) {
        dst.extend_from_slice(&caps["first"]);
        dst.extend_from_slice(b" ");
        dst.extend_from_slice(&caps["last"]);
    }
}

let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();
let result = re.replace(b"Springsteen, Bruce", NameSwapper);
assert_eq!(result, &b"Bruce Springsteen"[..]);
```

#### Required Methods

- `fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>)`

  Appends possibly empty data to `dst` to replace the current match.

#### Provided Methods

- `fn no_expansion<'r>(self: &'r mut Self) -> Option<Cow<'r, [u8]>>`

  Return a fixed unchanging replacement byte string.

- `fn by_ref<'r>(self: &'r mut Self) -> ReplacerRef<'r, Self>`

  Returns a type that implements `Replacer`, but that borrows and wraps

#### Implementors

- [`NoExpand`](#noexpand)
- [`ReplacerRef`](#replacerref)
- `&'a [u8; N]`
- `&'a [u8]`
- `&'a alloc::borrow::Cow<'a, [u8]>`
- `&'a alloc::vec::Vec<u8>`
- `F`
- `[u8; N]`
- `alloc::borrow::Cow<'a, [u8]>`
- `alloc::vec::Vec<u8>`

## Functions

### `no_expansion`

```rust
fn no_expansion<T: AsRef<[u8]>>(replacement: &T) -> Option<alloc::borrow::Cow<'_, [u8]>>
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2631-2637`](../../../../.source_1765210505/regex-1.12.2/src/regex/bytes.rs#L2631-L2637)*

Quickly checks the given replacement string for whether interpolation
should be done on it. It returns `None` if a `$` was found anywhere in the
given string, which suggests interpolation needs to be done. But if there's
no `$` anywhere, then interpolation definitely does not need to be done. In
that case, the given string is returned as a borrowed `Cow`.

This is meant to be used to implement the `Replacer::no_expansion` method
in its various trait impls.

