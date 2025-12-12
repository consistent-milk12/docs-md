*[regex](../index.md) / [bytes](index.md)*

---

# Module `bytes`

Search for regex matches in `&[u8]` haystacks.

This module provides a nearly identical API via [`Regex`](../regex/bytes/index.md) to the one found in
the top-level of this crate. There are two important differences:

1. Matching is done on `&[u8]` instead of `&str`. Additionally, `Vec<u8>`
is used where `String` would have been used in the top-level API.
2. Unicode support can be disabled even when disabling it would result in
matching invalid UTF-8 bytes.

# Example: match null terminated string

This shows how to find all null-terminated strings in a slice of bytes. This
works even if a C string contains invalid UTF-8.

```rust
use regex::bytes::Regex;

let re = Regex::new(r"(?-u)(?<cstr>[^\x00]+)\x00").unwrap();
let hay = b"foo\x00qu\xFFux\x00baz\x00";

// Extract all of the strings without the NUL terminator from each match.
// The unwrap is OK here since a match requires the `cstr` capture to match.
let cstrs: Vec<&[u8]> =
    re.captures_iter(hay)
      .map(|c| c.name("cstr").unwrap().as_bytes())
      .collect();
assert_eq!(cstrs, vec![&b"foo"[..], &b"qu\xFFux"[..], &b"baz"[..]]);
```

# Example: selectively enable Unicode support

This shows how to match an arbitrary byte pattern followed by a UTF-8 encoded
string (e.g., to extract a title from a Matroska file):

```rust
use regex::bytes::Regex;

let re = Regex::new(
    r"(?-u)\x7b\xa9(?:[\x80-\xfe]|[\x40-\xff].)(?u:(.*))"
).unwrap();
let hay = b"\x12\xd0\x3b\x5f\x7b\xa9\x85\xe2\x98\x83\x80\x98\x54\x76\x68\x65";

// Notice that despite the `.*` at the end, it will only match valid UTF-8
// because Unicode mode was enabled with the `u` flag. Without the `u` flag,
// the `.*` would match the rest of the bytes regardless of whether they were
// valid UTF-8.
let (_, [title]) = re.captures(hay).unwrap().extract();
assert_eq!(title, b"\xE2\x98\x83");
// We can UTF-8 decode the title now. And the unwrap here
// is correct because the existence of a match guarantees
// that `title` is valid UTF-8.
let title = std::str::from_utf8(title).unwrap();
assert_eq!(title, "☃");
```

In general, if the Unicode flag is enabled in a capture group and that capture
is part of the overall match, then the capture is *guaranteed* to be valid
UTF-8.

# Syntax

The supported syntax is pretty much the same as the syntax for Unicode
regular expressions with a few changes that make sense for matching arbitrary
bytes:

1. The `u` flag can be disabled even when disabling it might cause the regex to
match invalid UTF-8. When the `u` flag is disabled, the regex is said to be in
"ASCII compatible" mode.
2. In ASCII compatible mode, Unicode character classes are not allowed. Literal
Unicode scalar values outside of character classes are allowed.
3. In ASCII compatible mode, Perl character classes (`\w`, `\d` and `\s`)
revert to their typical ASCII definition. `\w` maps to `[[:word:]]`, `\d` maps
to `[[:digit:]]` and `\s` maps to `[[:space:]]`.
4. In ASCII compatible mode, word boundaries use the ASCII compatible `\w` to
determine whether a byte is a word byte or not.
5. Hexadecimal notation can be used to specify arbitrary bytes instead of
Unicode codepoints. For example, in ASCII compatible mode, `\xFF` matches the
literal byte `\xFF`, while in Unicode mode, `\xFF` is the Unicode codepoint
`U+00FF` that matches its UTF-8 encoding of `\xC3\xBF`. Similarly for octal
notation when enabled.
6. In ASCII compatible mode, `.` matches any *byte* except for `\n`. When the
`s` flag is additionally enabled, `.` matches any byte.

# Performance

In general, one should expect performance on `&[u8]` to be roughly similar to
performance on `&str`.

## Contents

- [Structs](#structs)
  - [`RegexBuilder`](#regexbuilder)
  - [`RegexSetBuilder`](#regexsetbuilder)
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
  - [`RegexSet`](#regexset)
  - [`SetMatches`](#setmatches)
  - [`SetMatchesIntoIter`](#setmatchesintoiter)
  - [`SetMatchesIter`](#setmatchesiter)
- [Traits](#traits)
  - [`Replacer`](#replacer)
- [Functions](#functions)
  - [`no_expansion`](#no-expansion)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RegexBuilder`](#regexbuilder) | struct | A configurable builder for a [`Regex`]. |
| [`RegexSetBuilder`](#regexsetbuilder) | struct | A configurable builder for a [`RegexSet`]. |
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
| [`RegexSet`](#regexset) | struct | Match multiple, possibly overlapping, regexes in a single search. |
| [`SetMatches`](#setmatches) | struct | A set of matches returned by a regex set. |
| [`SetMatchesIntoIter`](#setmatchesintoiter) | struct | An owned iterator over the set of matches from a regex set. |
| [`SetMatchesIter`](#setmatchesiter) | struct | A borrowed iterator over the set of matches from a regex set. |
| [`Replacer`](#replacer) | trait | A trait for types that can be used to replace matches in a haystack. |
| [`no_expansion`](#no-expansion) | fn | Quickly checks the given replacement string for whether interpolation should be done on it. |

## Structs

### `RegexBuilder`

```rust
struct RegexBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:1372-1374`](../../../.source_1765521767/regex-1.12.2/src/builders.rs#L1372-L1374)*

A configurable builder for a [`Regex`](../regex/bytes/index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- <span id="regexbuilder-new"></span>`fn new(pattern: &str) -> RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-build"></span>`fn build(&self) -> Result<Regex, Error>` — [`Regex`](../regex/bytes/index.md#regex), [`Error`](../error/index.md#error)

- <span id="regexbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

- <span id="regexbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

#### Trait Implementations

##### `impl Clone for RegexBuilder`

- <span id="regexbuilder-clone"></span>`fn clone(&self) -> RegexBuilder` — [`RegexBuilder`](#regexbuilder)

##### `impl Debug for RegexBuilder`

- <span id="regexbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RegexSetBuilder`

```rust
struct RegexSetBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:1966-1968`](../../../.source_1765521767/regex-1.12.2/src/builders.rs#L1966-L1968)*

A configurable builder for a [`RegexSet`](../regexset/bytes/index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- <span id="regexsetbuilder-new"></span>`fn new<I, S>(patterns: I) -> RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-build"></span>`fn build(&self) -> Result<RegexSet, Error>` — [`RegexSet`](../regexset/bytes/index.md#regexset), [`Error`](../error/index.md#error)

- <span id="regexsetbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

- <span id="regexsetbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

#### Trait Implementations

##### `impl Clone for RegexSetBuilder`

- <span id="regexsetbuilder-clone"></span>`fn clone(&self) -> RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

##### `impl Debug for RegexSetBuilder`

- <span id="regexsetbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Regex`

```rust
struct Regex {
    meta: meta::Regex,
    pattern: alloc::sync::Arc<str>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:99-102`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L99-L102)*

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

- <span id="regex-new"></span>`fn new(re: &str) -> Result<Regex, Error>` — [`Regex`](../regex/bytes/index.md#regex), [`Error`](../error/index.md#error)

- <span id="regex-is-match"></span>`fn is_match(&self, haystack: &[u8]) -> bool`

- <span id="regex-find"></span>`fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>>` — [`Match`](../regex/bytes/index.md#match)

- <span id="regex-find-iter"></span>`fn find_iter<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> Matches<'r, 'h>` — [`Matches`](../regex/bytes/index.md#matches)

- <span id="regex-captures"></span>`fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>>` — [`Captures`](../regex/bytes/index.md#captures)

- <span id="regex-captures-iter"></span>`fn captures_iter<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> CaptureMatches<'r, 'h>` — [`CaptureMatches`](../regex/bytes/index.md#capturematches)

- <span id="regex-split"></span>`fn split<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> Split<'r, 'h>` — [`Split`](../regex/bytes/index.md#split)

- <span id="regex-splitn"></span>`fn splitn<'r, 'h>(self: &'r Self, haystack: &'h [u8], limit: usize) -> SplitN<'r, 'h>` — [`SplitN`](../regex/bytes/index.md#splitn)

- <span id="regex-replace"></span>`fn replace<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>`

- <span id="regex-replace-all"></span>`fn replace_all<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>`

- <span id="regex-replacen"></span>`fn replacen<'h, R: Replacer>(&self, haystack: &'h [u8], limit: usize, rep: R) -> Cow<'h, [u8]>`

#### Trait Implementations

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](../regex/bytes/index.md#regex)

##### `impl Debug for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Regex`

- <span id="regex-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl FromStr for Regex`

- <span id="regex-fromstr-type-err"></span>`type Err = Error`

- <span id="regex-from-str"></span>`fn from_str(s: &str) -> Result<Regex, Error>` — [`Regex`](../regex/bytes/index.md#regex), [`Error`](../error/index.md#error)

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:1483-1487`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L1483-L1487)*

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
[Dijkstra's note on a related topic][`note`](../../object/read/elf/note/index.md).


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

- <span id="match-new"></span>`fn new(haystack: &'h [u8], start: usize, end: usize) -> Match<'h>` — [`Match`](../regex/bytes/index.md#match)

#### Trait Implementations

##### `impl Clone for Match<'h>`

- <span id="match-clone"></span>`fn clone(&self) -> Match<'h>` — [`Match`](../regex/bytes/index.md#match)

##### `impl Copy for Match<'h>`

##### `impl Debug for Match<'h>`

- <span id="match-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Match<'h>`

##### `impl PartialEq for Match<'h>`

- <span id="match-eq"></span>`fn eq(&self, other: &Match<'h>) -> bool` — [`Match`](../regex/bytes/index.md#match)

##### `impl StructuralPartialEq for Match<'h>`

### `Captures<'h>`

```rust
struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:1632-1636`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L1632-L1636)*

Represents the capture groups for a single match.

Capture groups refer to parts of a regex enclosed in parentheses. They
can be optionally named. The purpose of capture groups is to be able to
reference different parts of a match based on the original pattern. In
essence, a `Captures` is a container of [`Match`](../regex/bytes/index.md) values for each group
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

- <span id="captures-get"></span>`fn get(&self, i: usize) -> Option<Match<'h>>` — [`Match`](../regex/bytes/index.md#match)

- <span id="captures-get-match"></span>`fn get_match(&self) -> Match<'h>` — [`Match`](../regex/bytes/index.md#match)

- <span id="captures-name"></span>`fn name(&self, name: &str) -> Option<Match<'h>>` — [`Match`](../regex/bytes/index.md#match)

- <span id="captures-extract"></span>`fn extract<const N: usize>(&self) -> (&'h [u8], [&'h [u8]; N])`

- <span id="captures-expand"></span>`fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>)`

- <span id="captures-iter"></span>`fn iter<'c>(self: &'c Self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](../regex/bytes/index.md#subcapturematches)

- <span id="captures-len"></span>`fn len(&self) -> usize`

#### Trait Implementations

##### `impl Debug for Captures<'h>`

- <span id="captures-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Index for Captures<'h>`

- <span id="captures-index-type-output"></span>`type Output = [u8]`

- <span id="captures-index"></span>`fn index<'a>(self: &'a Self, i: usize) -> &'a [u8]`

### `CaptureLocations`

```rust
struct CaptureLocations(captures::Captures);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2084`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2084)*

A low level representation of the byte offsets of each capture group.

You can think of this as a lower level [`Captures`](../regex/bytes/index.md), where this type does
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

- <span id="capturelocations-clone"></span>`fn clone(&self) -> CaptureLocations` — [`CaptureLocations`](../regex/bytes/index.md#capturelocations)

##### `impl Debug for CaptureLocations`

- <span id="capturelocations-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Matches<'r, 'h>`

```rust
struct Matches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::FindMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2184-2187`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2184-L2187)*

An iterator over all non-overlapping matches in a haystack.

This iterator yields [`Match`](../regex/bytes/index.md) values. The iterator stops when no more
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

- <span id="matches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="matches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Matches<'r, 'h>`

- <span id="matches-iterator-type-item"></span>`type Item = Match<'h>`

- <span id="matches-next"></span>`fn next(&mut self) -> Option<Match<'h>>` — [`Match`](../regex/bytes/index.md#match)

- <span id="matches-count"></span>`fn count(self) -> usize`

### `CaptureMatches<'r, 'h>`

```rust
struct CaptureMatches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::CapturesMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2228-2231`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2228-L2231)*

An iterator over all non-overlapping capture matches in a haystack.

This iterator yields [`Captures`](../regex/bytes/index.md) values. The iterator stops when no more
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

- <span id="capturematches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturematches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturematches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CaptureMatches<'r, 'h>`

- <span id="capturematches-iterator-type-item"></span>`type Item = Captures<'h>`

- <span id="capturematches-next"></span>`fn next(&mut self) -> Option<Captures<'h>>` — [`Captures`](../regex/bytes/index.md#captures)

- <span id="capturematches-count"></span>`fn count(self) -> usize`

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    haystack: &'h [u8],
    it: meta::Split<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2272-2275`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2272-L2275)*

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

- <span id="split-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="split-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'r, 'h>`

- <span id="split-iterator-type-item"></span>`type Item = &'h [u8]`

- <span id="split-next"></span>`fn next(&mut self) -> Option<&'h [u8]>`

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    haystack: &'h [u8],
    it: meta::SplitN<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2307-2310`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2307-L2310)*

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

- <span id="splitn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splitn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splitn-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SplitN<'r, 'h>`

- <span id="splitn-iterator-type-item"></span>`type Item = &'h [u8]`

- <span id="splitn-next"></span>`fn next(&mut self) -> Option<&'h [u8]>`

- <span id="splitn-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `CaptureNames<'r>`

```rust
struct CaptureNames<'r>(captures::GroupInfoPatternNames<'r>);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2339`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2339)*

An iterator over the names of all capture groups in a regex.

This iterator yields values of type `Option<&str>` in order of the opening
capture group parenthesis in the regex pattern. `None` is yielded for
groups with no name. The first element always corresponds to the implicit
and unnamed group for the overall match.

`'r` is the lifetime of the compiled regular expression.

This iterator is created by `Regex::capture_names`.

#### Trait Implementations

##### `impl Clone for CaptureNames<'r>`

- <span id="capturenames-clone"></span>`fn clone(&self) -> CaptureNames<'r>` — [`CaptureNames`](../regex/bytes/index.md#capturenames)

##### `impl Debug for CaptureNames<'r>`

- <span id="capturenames-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for CaptureNames<'r>`

##### `impl FusedIterator for CaptureNames<'r>`

##### `impl IntoIterator for CaptureNames<'r>`

- <span id="capturenames-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturenames-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturenames-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CaptureNames<'r>`

- <span id="capturenames-iterator-type-item"></span>`type Item = Option<&'r str>`

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2381-2384`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2381-L2384)*

An iterator over all group matches in a [`Captures`](../regex/bytes/index.md) value.

This iterator yields values of type `Option<Match<'h>>`, where `'h` is the
lifetime of the haystack that the matches are for. The order of elements
yielded corresponds to the order of the opening parenthesis for the group
in the regex pattern. `None` is yielded for groups that did not participate
in the match.

The first element always corresponds to the implicit group for the overall
match. Since this iterator is created by a [`Captures`](../regex/bytes/index.md) value, and a
`Captures` value is only created when a match occurs, it follows that the
first element yielded by this iterator is guaranteed to be non-`None`.

The lifetime `'c` corresponds to the lifetime of the `Captures` value that
created this iterator, and the lifetime `'h` corresponds to the originally
matched haystack.

#### Trait Implementations

##### `impl Clone for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-clone"></span>`fn clone(&self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](../regex/bytes/index.md#subcapturematches)

##### `impl Debug for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for SubCaptureMatches<'c, 'h>`

##### `impl FusedIterator for SubCaptureMatches<'c, 'h>`

##### `impl IntoIterator for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="subcapturematches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="subcapturematches-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-iterator-type-item"></span>`type Item = Option<Match<'h>>`

- <span id="subcapturematches-next"></span>`fn next(&mut self) -> Option<Option<Match<'h>>>` — [`Match`](../regex/bytes/index.md#match)

- <span id="subcapturematches-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="subcapturematches-count"></span>`fn count(self) -> usize`

### `ReplacerRef<'a, R: ?Sized>`

```rust
struct ReplacerRef<'a, R: ?Sized>(&'a mut R);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2579`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2579)*

A by-reference adaptor for a [`Replacer`](../regex/bytes/index.md).

This permits reusing the same `Replacer` value in multiple calls to a
replacement routine like `Regex::replace_all`.

This type is created by `Replacer::by_ref`.

#### Trait Implementations

##### `impl<R: fmt::Debug + ?Sized> Debug for ReplacerRef<'a, R>`

- <span id="replacerref-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R>`

- <span id="replacerref-replace-append"></span>`fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>)` — [`Captures`](../regex/bytes/index.md#captures)

- <span id="replacerref-no-expansion"></span>`fn no_expansion<'r>(self: &'r mut Self) -> Option<Cow<'r, [u8]>>`

### `NoExpand<'s>`

```rust
struct NoExpand<'s>(&'s [u8]);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2611`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2611)*

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

- <span id="noexpand-clone"></span>`fn clone(&self) -> NoExpand<'s>` — [`NoExpand`](../regex/bytes/index.md#noexpand)

##### `impl Debug for NoExpand<'s>`

- <span id="noexpand-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Replacer for NoExpand<'s>`

- <span id="noexpand-replace-append"></span>`fn replace_append(&mut self, _: &Captures<'_>, dst: &mut Vec<u8>)` — [`Captures`](../regex/bytes/index.md#captures)

- <span id="noexpand-no-expansion"></span>`fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>>`

### `RegexSet`

```rust
struct RegexSet {
    meta: meta::Regex,
    patterns: alloc::sync::Arc<[alloc::string::String]>,
}
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:136-139`](../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L136-L139)*

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

- <span id="regexset-new"></span>`fn new<I, S>(exprs: I) -> Result<RegexSet, Error>` — [`RegexSet`](../regexset/bytes/index.md#regexset), [`Error`](../error/index.md#error)

- <span id="regexset-empty"></span>`fn empty() -> RegexSet` — [`RegexSet`](../regexset/bytes/index.md#regexset)

- <span id="regexset-is-match"></span>`fn is_match(&self, haystack: &[u8]) -> bool`

- <span id="regexset-is-match-at"></span>`fn is_match_at(&self, haystack: &[u8], start: usize) -> bool`

- <span id="regexset-matches"></span>`fn matches(&self, haystack: &[u8]) -> SetMatches` — [`SetMatches`](../regexset/bytes/index.md#setmatches)

- <span id="regexset-matches-at"></span>`fn matches_at(&self, haystack: &[u8], start: usize) -> SetMatches` — [`SetMatches`](../regexset/bytes/index.md#setmatches)

- <span id="regexset-len"></span>`fn len(&self) -> usize`

- <span id="regexset-is-empty"></span>`fn is_empty(&self) -> bool`

- <span id="regexset-patterns"></span>`fn patterns(&self) -> &[String]`

#### Trait Implementations

##### `impl Clone for RegexSet`

- <span id="regexset-clone"></span>`fn clone(&self) -> RegexSet` — [`RegexSet`](../regexset/bytes/index.md#regexset)

##### `impl Debug for RegexSet`

- <span id="regexset-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Default for RegexSet`

- <span id="regexset-default"></span>`fn default() -> Self`

### `SetMatches`

```rust
struct SetMatches(regex_automata::PatternSet);
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:463`](../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L463)*

A set of matches returned by a regex set.

Values of this type are constructed by `RegexSet::matches`.

#### Implementations

- <span id="setmatches-matched-any"></span>`fn matched_any(&self) -> bool`

- <span id="setmatches-matched-all"></span>`fn matched_all(&self) -> bool`

- <span id="setmatches-matched"></span>`fn matched(&self, index: usize) -> bool`

- <span id="setmatches-len"></span>`fn len(&self) -> usize`

- <span id="setmatches-iter"></span>`fn iter(&self) -> SetMatchesIter<'_>` — [`SetMatchesIter`](../regexset/bytes/index.md#setmatchesiter)

#### Trait Implementations

##### `impl Clone for SetMatches`

- <span id="setmatches-clone"></span>`fn clone(&self) -> SetMatches` — [`SetMatches`](../regexset/bytes/index.md#setmatches)

##### `impl Debug for SetMatches`

- <span id="setmatches-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for SetMatches`

- <span id="setmatches-intoiterator-type-intoiter"></span>`type IntoIter = SetMatchesIntoIter`

- <span id="setmatches-intoiterator-type-item"></span>`type Item = usize`

- <span id="setmatches-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

### `SetMatchesIntoIter`

```rust
struct SetMatchesIntoIter {
    patset: regex_automata::PatternSet,
    it: core::ops::Range<usize>,
}
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:656-659`](../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L656-L659)*

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

- <span id="setmatchesintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="setmatchesintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="setmatchesintoiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SetMatchesIntoIter`

- <span id="setmatchesintoiter-iterator-type-item"></span>`type Item = usize`

- <span id="setmatchesintoiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="setmatchesintoiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `SetMatchesIter<'a>`

```rust
struct SetMatchesIter<'a>(regex_automata::PatternSetIter<'a>);
```

*Defined in [`regex-1.12.2/src/regexset/bytes.rs:702`](../../../.source_1765521767/regex-1.12.2/src/regexset/bytes.rs#L702)*

A borrowed iterator over the set of matches from a regex set.

The lifetime `'a` refers to the lifetime of the [`SetMatches`](../regexset/bytes/index.md) value that
created this iterator.

This will always produces matches in ascending order, where the index
corresponds to the index of the regex that matched with respect to its
position when initially building the set.

This iterator is created by the `SetMatches::iter` method.

#### Trait Implementations

##### `impl Clone for SetMatchesIter<'a>`

- <span id="setmatchesiter-clone"></span>`fn clone(&self) -> SetMatchesIter<'a>` — [`SetMatchesIter`](../regexset/bytes/index.md#setmatchesiter)

##### `impl Debug for SetMatchesIter<'a>`

- <span id="setmatchesiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for SetMatchesIter<'a>`

##### `impl IntoIterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="setmatchesiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="setmatchesiter-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SetMatchesIter<'a>`

- <span id="setmatchesiter-iterator-type-item"></span>`type Item = usize`

- <span id="setmatchesiter-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="setmatchesiter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Traits

### `Replacer`

```rust
trait Replacer { ... }
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2443-2490`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2443-L2490)*

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

- [`NoExpand`](../regex/bytes/index.md#noexpand)
- [`ReplacerRef`](../regex/bytes/index.md#replacerref)
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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2631-2637`](../../../.source_1765521767/regex-1.12.2/src/regex/bytes.rs#L2631-L2637)*

Quickly checks the given replacement string for whether interpolation
should be done on it. It returns `None` if a `$` was found anywhere in the
given string, which suggests interpolation needs to be done. But if there's
no `$` anywhere, then interpolation definitely does not need to be done. In
that case, the given string is returned as a borrowed `Cow`.

This is meant to be used to implement the `Replacer::no_expansion` method
in its various trait impls.

