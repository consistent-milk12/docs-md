*[regex](../../index.md) / [regex](../index.md) / [string](index.md)*

---

# Module `string`

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
  - [`no_expansion`](#no-expansion)

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
| [`no_expansion`](#no-expansion) | fn | Quickly checks the given replacement string for whether interpolation should be done on it. |

## Structs

### `Regex`

```rust
struct Regex {
    meta: meta::Regex,
    pattern: alloc::sync::Arc<str>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:101-104`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L101-L104)*

A compiled regular expression for searching Unicode haystacks.

A `Regex` can be used to search haystacks, split haystacks into substrings
or replace substrings in a haystack with a different substring. All
searching is done with an implicit `(?s:.)*?` at the beginning and end of
an pattern. To force an expression to match the whole string (or a prefix
or a suffix), you must use an anchor like `^` or `$` (or `\A` and `\z`).

While this crate will handle Unicode strings (whether in the regular
expression or in the haystack), all positions returned are **byte
offsets**. Every byte offset is guaranteed to be at a Unicode code point
boundary. That is, all offsets returned by the `Regex` API are guaranteed
to be ranges that can slice a `&str` without panicking. If you want to
relax this requirement, then you must search `&[u8]` haystacks with a
[`bytes::Regex`](crate::bytes::Regex).

The only methods that allocate new strings are the string replacement
methods. All other methods (searching and splitting) return borrowed
references into the haystack given.

# Example

Find the offsets of a US phone number:

```rust
use regex::Regex;

let re = Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap();
let m = re.find("phone: 111-222-3333").unwrap();
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
use regex::Regex;

let hay = "
rabbit         54 true
groundhog 2 true
does not match
fox   109    false
";
let re = Regex::new(r"(?m)^\s*(\S+)\s+([0-9]+)\s+(true|false)\s*$").unwrap();
let mut fields: Vec<(&str, i64, bool)> = vec![];
for (_, [f1, f2, f3]) in re.captures_iter(hay).map(|caps| caps.extract()) {
    fields.push((f1, f2.parse()?, f3.parse()?));
}
assert_eq!(fields, vec![
    ("rabbit", 54, true),
    ("groundhog", 2, true),
    ("fox", 109, false),
]);

Ok::<(), Box<dyn std::error::Error>>(())
```

# Example: searching with the `Pattern` trait

**Note**: This section requires that this crate is compiled with the
`pattern` Cargo feature enabled, which **requires nightly Rust**.

Since `Regex` implements `Pattern` from the standard library, one can
use regexes with methods defined on `&str`. For example, `is_match`,
`find`, `find_iter` and `split` can, in some cases, be replaced with
`str::contains`, `str::find`, `str::match_indices` and `str::split`.

Here are some examples:

```ignore
use regex::Regex;

let re = Regex::new(r"\d+").unwrap();
let hay = "a111b222c";

assert!(hay.contains(&re));
assert_eq!(hay.find(&re), Some(1));
assert_eq!(hay.match_indices(&re).collect::<Vec<_>>(), vec![
    (1, "111"),
    (5, "222"),
]);
assert_eq!(hay.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);
```

#### Implementations

- <span id="regex-new"></span>`fn new(re: &str) -> Result<Regex, Error>` — [`Regex`](../../index.md#regex), [`Error`](../../error/index.md#error)

  Compiles a regular expression. Once compiled, it can be used repeatedly

  to search, split or replace substrings in a haystack.

  

  Note that regex compilation tends to be a somewhat expensive process,

  and unlike higher level environments, compilation is not automatically

  cached for you. One should endeavor to compile a regex once and then

  reuse it. For example, it's a bad idea to compile the same regex

  repeatedly in a loop.

  

  # Errors

  

  If an invalid pattern is given, then an error is returned.

  An error is also returned if the pattern is valid, but would

  produce a regex that is bigger than the configured size limit via

  `RegexBuilder::size_limit`. (A reasonable size limit is enabled by

  default.)

  

  # Example

  

  ```rust

  use regex::Regex;

  

  // An Invalid pattern because of an unclosed parenthesis

  assert!(Regex::new(r"foo(bar").is_err());

  // An invalid pattern because the regex would be too big

  // because Unicode tends to inflate things.

  assert!(Regex::new(r"\w{1000}").is_err());

  // Disabling Unicode can make the regex much smaller,

  // potentially by up to or more than an order of magnitude.

  assert!(Regex::new(r"(?-u:\w){1000}").is_ok());

  ```

- <span id="regex-is-match"></span>`fn is_match(&self, haystack: &str) -> bool`

  Returns true if and only if there is a match for the regex anywhere

  in the haystack given.

  

  It is recommended to use this method if all you need to do is test

  whether a match exists, since the underlying matching engine may be

  able to do less work.

  

  # Example

  

  Test if some haystack contains at least one word with exactly 13

  Unicode word characters:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = "I categorically deny having triskaidekaphobia.";

  assert!(re.is_match(hay));

  ```

- <span id="regex-find"></span>`fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>>` — [`Match`](../../index.md#match)

  This routine searches for the first match of this regex in the

  haystack given, and if found, returns a [`Match`](../../index.md). The `Match`

  provides access to both the byte offsets of the match and the actual

  substring that matched.

  

  Note that this should only be used if you want to find the entire

  match. If instead you just want to test the existence of a match,

  it's potentially faster to use `Regex::is_match(hay)` instead of

  `Regex::find(hay).is_some()`.

  

  # Example

  

  Find the first word with exactly 13 Unicode word characters:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = "I categorically deny having triskaidekaphobia.";

  let mat = re.find(hay).unwrap();

  assert_eq!(2..15, mat.range());

  assert_eq!("categorically", mat.as_str());

  ```

- <span id="regex-find-iter"></span>`fn find_iter<'r, 'h>(self: &'r Self, haystack: &'h str) -> Matches<'r, 'h>` — [`Matches`](../../index.md#matches)

  Returns an iterator that yields successive non-overlapping matches in

  the given haystack. The iterator yields values of type [`Match`](../../index.md).

  

  # Time complexity

  

  Note that since `find_iter` runs potentially many searches on the

  haystack and since each search has worst case `O(m * n)` time

  complexity, the overall worst case time complexity for iteration is

  `O(m * n^2)`.

  

  # Example

  

  Find every word with exactly 13 Unicode word characters:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = "Retroactively relinquishing remunerations is reprehensible.";

  let matches: Vec<_> = re.find_iter(hay).map(|m| m.as_str()).collect();

  assert_eq!(matches, vec![

      "Retroactively",

      "relinquishing",

      "remunerations",

      "reprehensible",

  ]);

  ```

- <span id="regex-captures"></span>`fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>>` — [`Captures`](../../index.md#captures)

  This routine searches for the first match of this regex in the haystack

  given, and if found, returns not only the overall match but also the

  matches of each capture group in the regex. If no match is found, then

  `None` is returned.

  

  Capture group `0` always corresponds to an implicit unnamed group that

  includes the entire match. If a match is found, this group is always

  present. Subsequent groups may be named and are numbered, starting

  at 1, by the order in which the opening parenthesis appears in the

  pattern. For example, in the pattern `(?<a>.(?<b>.))(?<c>.)`, `a`,

  `b` and `c` correspond to capture group indices `1`, `2` and `3`,

  respectively.

  

  You should only use `captures` if you need access to the capture group

  matches. Otherwise, `Regex::find` is generally faster for discovering

  just the overall match.

  

  # Example

  

  Say you have some haystack with movie names and their release years,

  like "'Citizen Kane' (1941)". It'd be nice if we could search for

  substrings looking like that, while also extracting the movie name and

  its release year separately. The example below shows how to do that.

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();

  let hay = "Not my favorite movie: 'Citizen Kane' (1941).";

  let caps = re.captures(hay).unwrap();

  assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");

  assert_eq!(caps.get(1).unwrap().as_str(), "Citizen Kane");

  assert_eq!(caps.get(2).unwrap().as_str(), "1941");

  // You can also access the groups by index using the Index notation.

  // Note that this will panic on an invalid index. In this case, these

  // accesses are always correct because the overall regex will only

  // match when these capture groups match.

  assert_eq!(&caps[0], "'Citizen Kane' (1941)");

  assert_eq!(&caps[1], "Citizen Kane");

  assert_eq!(&caps[2], "1941");

  ```

  

  Note that the full match is at capture group `0`. Each subsequent

  capture group is indexed by the order of its opening `(`.

  

  We can make this example a bit clearer by using *named* capture groups:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>\d{4})\)").unwrap();

  let hay = "Not my favorite movie: 'Citizen Kane' (1941).";

  let caps = re.captures(hay).unwrap();

  assert_eq!(caps.get(0).unwrap().as_str(), "'Citizen Kane' (1941)");

  assert_eq!(caps.name("title").unwrap().as_str(), "Citizen Kane");

  assert_eq!(caps.name("year").unwrap().as_str(), "1941");

  // You can also access the groups by name using the Index notation.

  // Note that this will panic on an invalid group name. In this case,

  // these accesses are always correct because the overall regex will

  // only match when these capture groups match.

  assert_eq!(&caps[0], "'Citizen Kane' (1941)");

  assert_eq!(&caps["title"], "Citizen Kane");

  assert_eq!(&caps["year"], "1941");

  ```

  

  Here we name the capture groups, which we can access with the `name`

  method or the `Index` notation with a `&str`. Note that the named

  capture groups are still accessible with `get` or the `Index` notation

  with a `usize`.

  

  The `0`th capture group is always unnamed, so it must always be

  accessed with `get(0)` or `[0]`.

  

  Finally, one other way to get the matched substrings is with the

  `Captures::extract` API:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();

  let hay = "Not my favorite movie: 'Citizen Kane' (1941).";

  let (full, [title, year]) = re.captures(hay).unwrap().extract();

  assert_eq!(full, "'Citizen Kane' (1941)");

  assert_eq!(title, "Citizen Kane");

  assert_eq!(year, "1941");

  ```

- <span id="regex-captures-iter"></span>`fn captures_iter<'r, 'h>(self: &'r Self, haystack: &'h str) -> CaptureMatches<'r, 'h>` — [`CaptureMatches`](../../index.md#capturematches)

  Returns an iterator that yields successive non-overlapping matches in

  the given haystack. The iterator yields values of type [`Captures`](../../index.md).

  

  This is the same as `Regex::find_iter`, but instead of only providing

  access to the overall match, each value yield includes access to the

  matches of all capture groups in the regex. Reporting this extra match

  data is potentially costly, so callers should only use `captures_iter`

  over `find_iter` when they actually need access to the capture group

  matches.

  

  # Time complexity

  

  Note that since `captures_iter` runs potentially many searches on the

  haystack and since each search has worst case `O(m * n)` time

  complexity, the overall worst case time complexity for iteration is

  `O(m * n^2)`.

  

  # Example

  

  We can use this to find all movie titles and their release years in

  some haystack, where the movie is formatted like "'Title' (xxxx)":

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();

  let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

  let mut movies = vec![];

  for (_, [title, year]) in re.captures_iter(hay).map(|c| c.extract()) {

      movies.push((title, year.parse::<i64>()?));

  }

  assert_eq!(movies, vec![

      ("Citizen Kane", 1941),

      ("The Wizard of Oz", 1939),

      ("M", 1931),

  ]);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Or with named groups:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>[0-9]{4})\)").unwrap();

  let hay = "'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

  let mut it = re.captures_iter(hay);

  

  let caps = it.next().unwrap();

  assert_eq!(&caps["title"], "Citizen Kane");

  assert_eq!(&caps["year"], "1941");

  

  let caps = it.next().unwrap();

  assert_eq!(&caps["title"], "The Wizard of Oz");

  assert_eq!(&caps["year"], "1939");

  

  let caps = it.next().unwrap();

  assert_eq!(&caps["title"], "M");

  assert_eq!(&caps["year"], "1931");

  ```

- <span id="regex-split"></span>`fn split<'r, 'h>(self: &'r Self, haystack: &'h str) -> Split<'r, 'h>` — [`Split`](../../index.md#split)

  Returns an iterator of substrings of the haystack given, delimited by a

  match of the regex. Namely, each element of the iterator corresponds to

  a part of the haystack that *isn't* matched by the regular expression.

  

  # Time complexity

  

  Since iterators over all matches requires running potentially many

  searches on the haystack, and since each search has worst case

  `O(m * n)` time complexity, the overall worst case time complexity for

  this routine is `O(m * n^2)`.

  

  # Example

  

  To split a string delimited by arbitrary amounts of spaces or tabs:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"[ \t]+").unwrap();

  let hay = "a b \t  c\td    e";

  let fields: Vec<&str> = re.split(hay).collect();

  assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);

  ```

  

  # Example: more cases

  

  Basic usage:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r" ").unwrap();

  let hay = "Mary had a little lamb";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["Mary", "had", "a", "little", "lamb"]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec![""]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "lionXXtigerXleopard";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["lion", "", "tiger", "leopard"]);

  

  let re = Regex::new(r"::").unwrap();

  let hay = "lion::tiger::leopard";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["lion", "tiger", "leopard"]);

  ```

  

  If a haystack contains multiple contiguous matches, you will end up

  with empty spans yielded by the iterator:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"X").unwrap();

  let hay = "XXXXaXXbXc";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["", "", "", "", "a", "", "b", "c"]);

  

  let re = Regex::new(r"/").unwrap();

  let hay = "(///)";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["(", "", "", ")"]);

  ```

  

  Separators at the start or end of a haystack are neighbored by empty

  substring.

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"0").unwrap();

  let hay = "010";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["", "1", ""]);

  ```

  

  When the empty string is used as a regex, it splits at every valid

  UTF-8 boundary by default (which includes the beginning and end of the

  haystack):

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"").unwrap();

  let hay = "rust";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["", "r", "u", "s", "t", ""]);

  

  // Splitting by an empty string is UTF-8 aware by default!

  let re = Regex::new(r"").unwrap();

  let hay = "☃";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["", "☃", ""]);

  ```

  

  Contiguous separators (commonly shows up with whitespace), can lead to

  possibly surprising behavior. For example, this code is correct:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r" ").unwrap();

  let hay = "    a  b c";

  let got: Vec<&str> = re.split(hay).collect();

  assert_eq!(got, vec!["", "", "", "", "a", "", "b", "c"]);

  ```

  

  It does *not* give you `["a", "b", "c"]`. For that behavior, you'd want

  to match contiguous space characters:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r" +").unwrap();

  let hay = "    a  b c";

  let got: Vec<&str> = re.split(hay).collect();

  // N.B. This does still include a leading empty span because ' +'

  // matches at the beginning of the haystack.

  assert_eq!(got, vec!["", "a", "b", "c"]);

  ```

- <span id="regex-splitn"></span>`fn splitn<'r, 'h>(self: &'r Self, haystack: &'h str, limit: usize) -> SplitN<'r, 'h>` — [`SplitN`](../../index.md#splitn)

  Returns an iterator of at most `limit` substrings of the haystack

  given, delimited by a match of the regex. (A `limit` of `0` will return

  no substrings.) Namely, each element of the iterator corresponds to a

  part of the haystack that *isn't* matched by the regular expression.

  The remainder of the haystack that is not split will be the last

  element in the iterator.

  

  # Time complexity

  

  Since iterators over all matches requires running potentially many

  searches on the haystack, and since each search has worst case

  `O(m * n)` time complexity, the overall worst case time complexity for

  this routine is `O(m * n^2)`.

  

  Although note that the worst case time here has an upper bound given

  by the `limit` parameter.

  

  # Example

  

  Get the first two words in some haystack:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"\W+").unwrap();

  let hay = "Hey! How are you?";

  let fields: Vec<&str> = re.splitn(hay, 3).collect();

  assert_eq!(fields, vec!["Hey", "How", "are you?"]);

  ```

  

  # Examples: more cases

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r" ").unwrap();

  let hay = "Mary had a little lamb";

  let got: Vec<&str> = re.splitn(hay, 3).collect();

  assert_eq!(got, vec!["Mary", "had", "a little lamb"]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "";

  let got: Vec<&str> = re.splitn(hay, 3).collect();

  assert_eq!(got, vec![""]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "lionXXtigerXleopard";

  let got: Vec<&str> = re.splitn(hay, 3).collect();

  assert_eq!(got, vec!["lion", "", "tigerXleopard"]);

  

  let re = Regex::new(r"::").unwrap();

  let hay = "lion::tiger::leopard";

  let got: Vec<&str> = re.splitn(hay, 2).collect();

  assert_eq!(got, vec!["lion", "tiger::leopard"]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "abcXdef";

  let got: Vec<&str> = re.splitn(hay, 1).collect();

  assert_eq!(got, vec!["abcXdef"]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "abcdef";

  let got: Vec<&str> = re.splitn(hay, 2).collect();

  assert_eq!(got, vec!["abcdef"]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = "abcXdef";

  let got: Vec<&str> = re.splitn(hay, 0).collect();

  assert!(got.is_empty());

  ```

- <span id="regex-replace"></span>`fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str>`

  Replaces the leftmost-first match in the given haystack with the

  replacement provided. The replacement can be a regular string (where

  `$N` and `$name` are expanded to match capture groups) or a function

  that takes a [`Captures`](../../index.md) and returns the replaced string.

  

  If no match is found, then the haystack is returned unchanged. In that

  case, this implementation will likely return a `Cow::Borrowed` value

  such that no allocation is performed.

  

  When a `Cow::Borrowed` is returned, the value returned is guaranteed

  to be equivalent to the `haystack` given.

  

  # Replacement string syntax

  

  All instances of `$ref` in the replacement string are replaced with

  the substring corresponding to the capture group identified by `ref`.

  

  `ref` may be an integer corresponding to the index of the capture group

  (counted by order of opening parenthesis where `0` is the entire match)

  or it can be a name (consisting of letters, digits or underscores)

  corresponding to a named capture group.

  

  If `ref` isn't a valid capture group (whether the name doesn't exist or

  isn't a valid index), then it is replaced with the empty string.

  

  The longest possible name is used. For example, `$1a` looks up the

  capture group named `1a` and not the capture group at index `1`. To

  exert more precise control over the name, use braces, e.g., `${1}a`.

  

  To write a literal `$` use `$$`.

  

  # Example

  

  Note that this function is polymorphic with respect to the replacement.

  In typical usage, this can just be a normal string:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"[^01]+").unwrap();

  assert_eq!(re.replace("1078910", ""), "1010");

  ```

  

  But anything satisfying the [`Replacer`](../../index.md) trait will work. For example,

  a closure of type `|&Captures| -> String` provides direct access to the

  captures corresponding to a match. This allows one to access capturing

  group matches easily:

  

  ```rust

  use regex::{Captures, Regex};

  

  let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();

  let result = re.replace("Springsteen, Bruce", |caps: &Captures| {

      format!("{} {}", &caps[2], &caps[1])

  });

  assert_eq!(result, "Bruce Springsteen");

  ```

  

  But this is a bit cumbersome to use all the time. Instead, a simple

  syntax is supported (as described above) that expands `$name` into the

  corresponding capture group. Here's the last example, but using this

  expansion technique with named capture groups:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();

  let result = re.replace("Springsteen, Bruce", "$first $last");

  assert_eq!(result, "Bruce Springsteen");

  ```

  

  Note that using `$2` instead of `$first` or `$1` instead of `$last`

  would produce the same result. To write a literal `$` use `$$`.

  

  Sometimes the replacement string requires use of curly braces to

  delineate a capture group replacement when it is adjacent to some other

  literal text. For example, if we wanted to join two words together with

  an underscore:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(?<first>\w+)\s+(?<second>\w+)").unwrap();

  let result = re.replace("deep fried", "${first}_$second");

  assert_eq!(result, "deep_fried");

  ```

  

  Without the curly braces, the capture group name `first_` would be

  used, and since it doesn't exist, it would be replaced with the empty

  string.

  

  Finally, sometimes you just want to replace a literal string with no

  regard for capturing group expansion. This can be done by wrapping a

  string with [`NoExpand`](../../index.md):

  

  ```rust

  use regex::{NoExpand, Regex};

  

  let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();

  let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));

  assert_eq!(result, "$2 $last");

  ```

  

  Using `NoExpand` may also be faster, since the replacement string won't

  need to be parsed for the `$` syntax.

- <span id="regex-replace-all"></span>`fn replace_all<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str>`

  Replaces all non-overlapping matches in the haystack with the

  replacement provided. This is the same as calling `replacen` with

  `limit` set to `0`.

  

  If no match is found, then the haystack is returned unchanged. In that

  case, this implementation will likely return a `Cow::Borrowed` value

  such that no allocation is performed.

  

  When a `Cow::Borrowed` is returned, the value returned is guaranteed

  to be equivalent to the `haystack` given.

  

  The documentation for `Regex::replace` goes into more detail about

  what kinds of replacement strings are supported.

  

  # Time complexity

  

  Since iterators over all matches requires running potentially many

  searches on the haystack, and since each search has worst case

  `O(m * n)` time complexity, the overall worst case time complexity for

  this routine is `O(m * n^2)`.

  

  # Fallibility

  

  If you need to write a replacement routine where any individual

  replacement might "fail," doing so with this API isn't really feasible

  because there's no way to stop the search process if a replacement

  fails. Instead, if you need this functionality, you should consider

  implementing your own replacement routine:

  

  ```rust

  use regex::{Captures, Regex};

  

  fn replace_all<E>(

      re: &Regex,

      haystack: &str,

      replacement: impl Fn(&Captures) -> Result<String, E>,

  ) -> Result<String, E> {

      let mut new = String::with_capacity(haystack.len());

      let mut last_match = 0;

      for caps in re.captures_iter(haystack) {

          let m = caps.get(0).unwrap();

          new.push_str(&haystack[last_match..m.start()]);

          new.push_str(&replacement(&caps)?);

          last_match = m.end();

      }

      new.push_str(&haystack[last_match..]);

      Ok(new)

  }

  

  // Let's replace each word with the number of bytes in that word.

  // But if we see a word that is "too long," we'll give up.

  let re = Regex::new(r"\w+").unwrap();

  let replacement = |caps: &Captures| -> Result<String, &'static str> {

      if caps[0].len() >= 5 {

          return Err("word too long");

      }

      Ok(caps[0].len().to_string())

  };

  assert_eq!(

      Ok("2 3 3 3?".to_string()),

      replace_all(&re, "hi how are you?", &replacement),

  );

  assert!(replace_all(&re, "hi there", &replacement).is_err());

  ```

  

  # Example

  

  This example shows how to flip the order of whitespace (excluding line

  terminators) delimited fields, and normalizes the whitespace that

  delimits the fields:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();

  let hay = "

  Greetings  1973

  Wild\t1973

  BornToRun\t\t\t\t1975

  Darkness                    1978

  TheRiver 1980

  ";

  let new = re.replace_all(hay, "$2 $1");

  assert_eq!(new, "

  1973 Greetings

  1973 Wild

  1975 BornToRun

  1978 Darkness

  1980 TheRiver

  ");

  ```

- <span id="regex-replacen"></span>`fn replacen<'h, R: Replacer>(&self, haystack: &'h str, limit: usize, rep: R) -> Cow<'h, str>`

  Replaces at most `limit` non-overlapping matches in the haystack with

  the replacement provided. If `limit` is `0`, then all non-overlapping

  matches are replaced. That is, `Regex::replace_all(hay, rep)` is

  equivalent to `Regex::replacen(hay, 0, rep)`.

  

  If no match is found, then the haystack is returned unchanged. In that

  case, this implementation will likely return a `Cow::Borrowed` value

  such that no allocation is performed.

  

  When a `Cow::Borrowed` is returned, the value returned is guaranteed

  to be equivalent to the `haystack` given.

  

  The documentation for `Regex::replace` goes into more detail about

  what kinds of replacement strings are supported.

  

  # Time complexity

  

  Since iterators over all matches requires running potentially many

  searches on the haystack, and since each search has worst case

  `O(m * n)` time complexity, the overall worst case time complexity for

  this routine is `O(m * n^2)`.

  

  Although note that the worst case time here has an upper bound given

  by the `limit` parameter.

  

  # Fallibility

  

  See the corresponding section in the docs for `Regex::replace_all`

  for tips on how to deal with a replacement routine that can fail.

  

  # Example

  

  This example shows how to flip the order of whitespace (excluding line

  terminators) delimited fields, and normalizes the whitespace that

  delimits the fields. But we only do it for the first two matches.

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();

  let hay = "

  Greetings  1973

  Wild\t1973

  BornToRun\t\t\t\t1975

  Darkness                    1978

  TheRiver 1980

  ";

  let new = re.replacen(hay, 2, "$2 $1");

  assert_eq!(new, "

  1973 Greetings

  1973 Wild

  BornToRun\t\t\t\t1975

  Darkness                    1978

  TheRiver 1980

  ");

  ```

#### Trait Implementations

##### `impl Any for Regex`

- <span id="regex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Regex`

- <span id="regex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Regex`

- <span id="regex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](../../index.md#regex)

##### `impl CloneToUninit for Regex`

- <span id="regex-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Regex`

- <span id="regex-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Shows the original regular expression.

##### `impl Display for Regex`

- <span id="regex-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

  Shows the original regular expression.

##### `impl<T> From for Regex`

- <span id="regex-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FromStr for Regex`

- <span id="regex-fromstr-type-err"></span>`type Err = Error`

- <span id="regex-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Regex, Error>` — [`Regex`](../../index.md#regex), [`Error`](../../error/index.md#error)

  Attempts to parse a string into a regular expression

##### `impl<U> Into for Regex`

- <span id="regex-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Regex`

- <span id="regex-toowned-type-owned"></span>`type Owned = T`

- <span id="regex-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regex-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Regex`

- <span id="regex-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Regex`

- <span id="regex-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regex-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Regex`

- <span id="regex-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regex-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Match<'h>`

```rust
struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:1490-1494`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L1490-L1494)*

Represents a single match of a regex in a haystack.

A `Match` contains both the start and end byte offsets of the match and the
actual substring corresponding to the range of those byte offsets. It is
guaranteed that `start <= end`. When `start == end`, the match is empty.

Since this `Match` can only be produced by the top-level `Regex` APIs
that only support searching UTF-8 encoded strings, the byte offsets for a
`Match` are guaranteed to fall on valid UTF-8 codepoint boundaries. That
is, slicing a `&str` with `Match::range` is guaranteed to never panic.

Values with this type are created by `Regex::find` or
`Regex::find_iter`. Other APIs can create `Match` values too. For
example, `Captures::get`.

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
use regex::Regex;

let re = Regex::new(r"\p{Greek}+").unwrap();
let hay = "Greek: αβγδ";
let m = re.find(hay).unwrap();
assert_eq!(7, m.start());
assert_eq!(15, m.end());
assert!(!m.is_empty());
assert_eq!(8, m.len());
assert_eq!(7..15, m.range());
assert_eq!("αβγδ", m.as_str());
```

#### Implementations

- <span id="match-start"></span>`fn start(&self) -> usize`

  Returns the byte offset of the start of the match in the haystack. The

  start of the match corresponds to the position where the match begins

  and includes the first byte in the match.

  

  It is guaranteed that `Match::start() <= Match::end()`.

  

  This is guaranteed to fall on a valid UTF-8 codepoint boundary. That

  is, it will never be an offset that appears between the UTF-8 code

  units of a UTF-8 encoded Unicode scalar value. Consequently, it is

  always safe to slice the corresponding haystack using this offset.

- <span id="match-end"></span>`fn end(&self) -> usize`

  Returns the byte offset of the end of the match in the haystack. The

  end of the match corresponds to the byte immediately following the last

  byte in the match. This means that `&slice[start..end]` works as one

  would expect.

  

  It is guaranteed that `Match::start() <= Match::end()`.

  

  This is guaranteed to fall on a valid UTF-8 codepoint boundary. That

  is, it will never be an offset that appears between the UTF-8 code

  units of a UTF-8 encoded Unicode scalar value. Consequently, it is

  always safe to slice the corresponding haystack using this offset.

- <span id="match-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns true if and only if this match has a length of zero.

  

  Note that an empty match can only occur when the regex itself can

  match the empty string. Here are some examples of regexes that can

  all match the empty string: `^`, `^$`, `\b`, `a?`, `a*`, `a{0}`,

  `(foo|\d+|quux)?`.

- <span id="match-len"></span>`fn len(&self) -> usize`

  Returns the length, in bytes, of this match.

- <span id="match-range"></span>`fn range(&self) -> core::ops::Range<usize>`

  Returns the range over the starting and ending byte offsets of the

  match in the haystack.

  

  It is always correct to slice the original haystack searched with this

  range. That is, because the offsets are guaranteed to fall on valid

  UTF-8 boundaries, the range returned is always valid.

- <span id="match-as-str"></span>`fn as_str(&self) -> &'h str`

  Returns the substring of the haystack that matched.

- <span id="match-new"></span>`fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h>` — [`Match`](../../index.md#match)

  Creates a new match from the given haystack and byte offsets.

#### Trait Implementations

##### `impl Any for Match<'h>`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match<'h>`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match<'h>`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Match<'h>`

- <span id="match-clone"></span>`fn clone(&self) -> Match<'h>` — [`Match`](../../index.md#match)

##### `impl CloneToUninit for Match<'h>`

- <span id="match-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Match<'h>`

##### `impl Debug for Match<'h>`

- <span id="match-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Eq for Match<'h>`

##### `impl<T> From for Match<'h>`

- <span id="match-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Match<'h>`

- <span id="match-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Match<'h>`

- <span id="match-partialeq-eq"></span>`fn eq(&self, other: &Match<'h>) -> bool` — [`Match`](../../index.md#match)

##### `impl StructuralPartialEq for Match<'h>`

##### `impl ToOwned for Match<'h>`

- <span id="match-toowned-type-owned"></span>`type Owned = T`

- <span id="match-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="match-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Match<'h>`

- <span id="match-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="match-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Match<'h>`

- <span id="match-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="match-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Captures<'h>`

```rust
struct Captures<'h> {
    haystack: &'h str,
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:1642-1646`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L1642-L1646)*

Represents the capture groups for a single match.

Capture groups refer to parts of a regex enclosed in parentheses. They
can be optionally named. The purpose of capture groups is to be able to
reference different parts of a match based on the original pattern. In
essence, a `Captures` is a container of [`Match`](../../index.md) values for each group
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
use regex::Regex;

let re = Regex::new(r"(?<first>\w)(\w)(?:\w)\w(?<last>\w)").unwrap();
let caps = re.captures("toady").unwrap();
assert_eq!("toady", &caps[0]);
assert_eq!("t", &caps["first"]);
assert_eq!("o", &caps[2]);
assert_eq!("y", &caps["last"]);
```

#### Implementations

- <span id="captures-get"></span>`fn get(&self, i: usize) -> Option<Match<'h>>` — [`Match`](../../index.md#match)

  Returns the `Match` associated with the capture group at index `i`. If

  `i` does not correspond to a capture group, or if the capture group did

  not participate in the match, then `None` is returned.

  

  When `i == 0`, this is guaranteed to return a non-`None` value.

  

  # Examples

  

  Get the substring that matched with a default of an empty string if the

  group didn't participate in the match:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();

  let caps = re.captures("abc123").unwrap();

  

  let substr1 = caps.get(1).map_or("", |m| m.as_str());

  let substr2 = caps.get(2).map_or("", |m| m.as_str());

  assert_eq!(substr1, "123");

  assert_eq!(substr2, "");

  ```

- <span id="captures-get-match"></span>`fn get_match(&self) -> Match<'h>` — [`Match`](../../index.md#match)

  Return the overall match for the capture.

  

  This returns the match for index `0`. That is it is equivalent to

  `m.get(0).unwrap()`

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"[a-z]+([0-9]+)").unwrap();

  let caps = re.captures("   abc123-def").unwrap();

  

  assert_eq!(caps.get_match().as_str(), "abc123");

  

  ```

- <span id="captures-name"></span>`fn name(&self, name: &str) -> Option<Match<'h>>` — [`Match`](../../index.md#match)

  Returns the `Match` associated with the capture group named `name`. If

  `name` isn't a valid capture group or it refers to a group that didn't

  match, then `None` is returned.

  

  Note that unlike `caps["name"]`, this returns a `Match` whose lifetime

  matches the lifetime of the haystack in this `Captures` value.

  Conversely, the substring returned by `caps["name"]` has a lifetime

  of the `Captures` value, which is likely shorter than the lifetime of

  the haystack. In some cases, it may be necessary to use this method to

  access the matching substring instead of the `caps["name"]` notation.

  

  # Examples

  

  Get the substring that matched with a default of an empty string if the

  group didn't participate in the match:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(

      r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))",

  ).unwrap();

  let caps = re.captures("abc123").unwrap();

  

  let numbers = caps.name("numbers").map_or("", |m| m.as_str());

  let letters = caps.name("letters").map_or("", |m| m.as_str());

  assert_eq!(numbers, "123");

  assert_eq!(letters, "");

  ```

- <span id="captures-extract"></span>`fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N])`

  This is a convenience routine for extracting the substrings

  corresponding to matching capture groups.

  

  This returns a tuple where the first element corresponds to the full

  substring of the haystack that matched the regex. The second element is

  an array of substrings, with each corresponding to the substring that

  matched for a particular capture group.

  

  # Panics

  

  This panics if the number of possible matching groups in this

  `Captures` value is not fixed to `N` in all circumstances.

  More precisely, this routine only works when `N` is equivalent to

  `Regex::static_captures_len`.

  

  Stated more plainly, if the number of matching capture groups in a

  regex can vary from match to match, then this function always panics.

  

  For example, `(a)(b)|(c)` could produce two matching capture groups

  or one matching capture group for any given match. Therefore, one

  cannot use `extract` with such a pattern.

  

  But a pattern like `(a)(b)|(c)(d)` can be used with `extract` because

  the number of capture groups in every match is always equivalent,

  even if the capture _indices_ in each match are not.

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();

  let hay = "On 2010-03-14, I became a Tennessee lamb.";

  let Some((full, [year, month, day])) =

      re.captures(hay).map(|caps| caps.extract()) else { return };

  assert_eq!("2010-03-14", full);

  assert_eq!("2010", year);

  assert_eq!("03", month);

  assert_eq!("14", day);

  ```

  

  # Example: iteration

  

  This example shows how to use this method when iterating over all

  `Captures` matches in a haystack.

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();

  let hay = "1973-01-05, 1975-08-25 and 1980-10-18";

  

  let mut dates: Vec<(&str, &str, &str)> = vec![];

  for (_, [y, m, d]) in re.captures_iter(hay).map(|c| c.extract()) {

      dates.push((y, m, d));

  }

  assert_eq!(dates, vec![

      ("1973", "01", "05"),

      ("1975", "08", "25"),

      ("1980", "10", "18"),

  ]);

  ```

  

  # Example: parsing different formats

  

  This API is particularly useful when you need to extract a particular

  value that might occur in a different format. Consider, for example,

  an identifier that might be in double quotes or single quotes:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r#"id:(?:"([^"]+)"|'([^']+)')"#).unwrap();

  let hay = r#"The first is id:"foo" and the second is id:'bar'."#;

  let mut ids = vec![];

  for (_, [id]) in re.captures_iter(hay).map(|c| c.extract()) {

      ids.push(id);

  }

  assert_eq!(ids, vec!["foo", "bar"]);

  ```

- <span id="captures-expand"></span>`fn expand(&self, replacement: &str, dst: &mut String)`

  Expands all instances of `$ref` in `replacement` to the corresponding

  capture group, and writes them to the `dst` buffer given. A `ref` can

  be a capture group index or a name. If `ref` doesn't refer to a capture

  group that participated in the match, then it is replaced with the

  empty string.

  

  # Format

  

  The format of the replacement string supports two different kinds of

  capture references: unbraced and braced.

  

  For the unbraced format, the format supported is `$ref` where `name`

  can be any character in the class `[0-9A-Za-z_]`. `ref` is always

  the longest possible parse. So for example, `$1a` corresponds to the

  capture group named `1a` and not the capture group at index `1`. If

  `ref` matches `^[0-9]+$`, then it is treated as a capture group index

  itself and not a name.

  

  For the braced format, the format supported is `${ref}` where `ref` can

  be any sequence of bytes except for `}`. If no closing brace occurs,

  then it is not considered a capture reference. As with the unbraced

  format, if `ref` matches `^[0-9]+$`, then it is treated as a capture

  group index and not a name.

  

  The braced format is useful for exerting precise control over the name

  of the capture reference. For example, `${1}a` corresponds to the

  capture group reference `1` followed by the letter `a`, where as `$1a`

  (as mentioned above) corresponds to the capture group reference `1a`.

  The braced format is also useful for expressing capture group names

  that use characters not supported by the unbraced format. For example,

  `${foo[bar].baz}` refers to the capture group named `foo[bar].baz`.

  

  If a capture group reference is found and it does not refer to a valid

  capture group, then it will be replaced with the empty string.

  

  To write a literal `$`, use `$$`.

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(

      r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",

  ).unwrap();

  let hay = "On 14-03-2010, I became a Tennessee lamb.";

  let caps = re.captures(hay).unwrap();

  

  let mut dst = String::new();

  caps.expand("year=$year, month=$month, day=$day", &mut dst);

  assert_eq!(dst, "year=2010, month=03, day=14");

  ```

- <span id="captures-iter"></span>`fn iter<'c>(self: &'c Self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](../../index.md#subcapturematches)

  Returns an iterator over all capture groups. This includes both

  matching and non-matching groups.

  

  The iterator always yields at least one matching group: the first group

  (at index `0`) with no name. Subsequent groups are returned in the order

  of their opening parenthesis in the regex.

  

  The elements yielded have type `Option<Match<'h>>`, where a non-`None`

  value is present if the capture group matches.

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();

  let caps = re.captures("AZ").unwrap();

  

  let mut it = caps.iter();

  assert_eq!(it.next().unwrap().map(|m| m.as_str()), Some("AZ"));

  assert_eq!(it.next().unwrap().map(|m| m.as_str()), Some("A"));

  assert_eq!(it.next().unwrap().map(|m| m.as_str()), None);

  assert_eq!(it.next().unwrap().map(|m| m.as_str()), Some("Z"));

  assert_eq!(it.next(), None);

  ```

- <span id="captures-len"></span>`fn len(&self) -> usize`

  Returns the total number of capture groups. This includes both

  matching and non-matching groups.

  

  The length returned is always equivalent to the number of elements

  yielded by `Captures::iter`. Consequently, the length is always

  greater than zero since every `Captures` value always includes the

  match for the entire regex.

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();

  let caps = re.captures("AZ").unwrap();

  assert_eq!(caps.len(), 4);

  ```

#### Trait Implementations

##### `impl Any for Captures<'h>`

- <span id="captures-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Captures<'h>`

- <span id="captures-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Captures<'h>`

- <span id="captures-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Captures<'h>`

- <span id="captures-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl<T> From for Captures<'h>`

- <span id="captures-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl Index for Captures<'h>`

- <span id="captures-index-type-output"></span>`type Output = str`

- <span id="captures-index"></span>`fn index<'a>(self: &'a Self, i: usize) -> &'a str`

##### `impl<U> Into for Captures<'h>`

- <span id="captures-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for Captures<'h>`

- <span id="captures-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="captures-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Captures<'h>`

- <span id="captures-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="captures-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CaptureLocations`

```rust
struct CaptureLocations(captures::Captures);
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2093`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2093)*

A low level representation of the byte offsets of each capture group.

You can think of this as a lower level [`Captures`](../../index.md), where this type does
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
use regex::Regex;

let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();
let mut locs = re.capture_locations();
let m = re.captures_read(&mut locs, "Bruce Springsteen").unwrap();
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

  Returns the start and end byte offsets of the capture group at index

  `i`. This returns `None` if `i` is not a valid capture group or if the

  capture group did not match.

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();

  let mut locs = re.capture_locations();

  re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

  assert_eq!(Some((0, 17)), locs.get(0));

  assert_eq!(Some((0, 5)), locs.get(1));

  assert_eq!(Some((6, 17)), locs.get(2));

  ```

- <span id="capturelocations-len"></span>`fn len(&self) -> usize`

  Returns the total number of capture groups (even if they didn't match).

  That is, the length returned is unaffected by the result of a search.

  

  This is always at least `1` since every regex has at least `1`

  capturing group that corresponds to the entire match.

  

  # Example

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();

  let mut locs = re.capture_locations();

  assert_eq!(3, locs.len());

  re.captures_read(&mut locs, "Bruce Springsteen").unwrap();

  assert_eq!(3, locs.len());

  ```

  

  Notice that the length is always at least `1`, regardless of the regex:

  

  ```rust

  use regex::Regex;

  

  let re = Regex::new(r"").unwrap();

  let locs = re.capture_locations();

  assert_eq!(1, locs.len());

  

  // [a&&b] is a regex that never matches anything.

  let re = Regex::new(r"[a&&b]").unwrap();

  let locs = re.capture_locations();

  assert_eq!(1, locs.len());

  ```

#### Trait Implementations

##### `impl Any for CaptureLocations`

- <span id="capturelocations-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaptureLocations`

- <span id="capturelocations-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaptureLocations`

- <span id="capturelocations-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CaptureLocations`

- <span id="capturelocations-clone"></span>`fn clone(&self) -> CaptureLocations` — [`CaptureLocations`](../../index.md#capturelocations)

##### `impl CloneToUninit for CaptureLocations`

- <span id="capturelocations-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CaptureLocations`

- <span id="capturelocations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CaptureLocations`

- <span id="capturelocations-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for CaptureLocations`

- <span id="capturelocations-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for CaptureLocations`

- <span id="capturelocations-toowned-type-owned"></span>`type Owned = T`

- <span id="capturelocations-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capturelocations-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CaptureLocations`

- <span id="capturelocations-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturelocations-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaptureLocations`

- <span id="capturelocations-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturelocations-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Matches<'r, 'h>`

```rust
struct Matches<'r, 'h> {
    haystack: &'h str,
    it: meta::FindMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2193-2196`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2193-L2196)*

An iterator over all non-overlapping matches in a haystack.

This iterator yields [`Match`](../../index.md) values. The iterator stops when no more
matches can be found.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the haystack.

This iterator is created by `Regex::find_iter`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

#### Trait Implementations

##### `impl Any for Matches<'r, 'h>`

- <span id="matches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Matches<'r, 'h>`

- <span id="matches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Matches<'r, 'h>`

- <span id="matches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Matches<'r, 'h>`

- <span id="matches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Matches<'r, 'h>`

- <span id="matches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Matches<'r, 'h>`

##### `impl<U> Into for Matches<'r, 'h>`

- <span id="matches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Matches<'r, 'h>`

- <span id="matches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="matches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="matches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Matches<'r, 'h>`

- <span id="matches-iterator-type-item"></span>`type Item = Match<'h>`

- <span id="matches-iterator-next"></span>`fn next(&mut self) -> Option<Match<'h>>` — [`Match`](../../index.md#match)

- <span id="matches-iterator-count"></span>`fn count(self) -> usize`

##### `impl<U> TryFrom for Matches<'r, 'h>`

- <span id="matches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="matches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Matches<'r, 'h>`

- <span id="matches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="matches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CaptureMatches<'r, 'h>`

```rust
struct CaptureMatches<'r, 'h> {
    haystack: &'h str,
    it: meta::CapturesMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2237-2240`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2237-L2240)*

An iterator over all non-overlapping capture matches in a haystack.

This iterator yields [`Captures`](../../index.md) values. The iterator stops when no more
matches can be found.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the matched string.

This iterator is created by `Regex::captures_iter`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

#### Trait Implementations

##### `impl Any for CaptureMatches<'r, 'h>`

- <span id="capturematches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaptureMatches<'r, 'h>`

- <span id="capturematches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaptureMatches<'r, 'h>`

- <span id="capturematches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for CaptureMatches<'r, 'h>`

- <span id="capturematches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for CaptureMatches<'r, 'h>`

- <span id="capturematches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for CaptureMatches<'r, 'h>`

##### `impl<U> Into for CaptureMatches<'r, 'h>`

- <span id="capturematches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CaptureMatches<'r, 'h>`

- <span id="capturematches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturematches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturematches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CaptureMatches<'r, 'h>`

- <span id="capturematches-iterator-type-item"></span>`type Item = Captures<'h>`

- <span id="capturematches-iterator-next"></span>`fn next(&mut self) -> Option<Captures<'h>>` — [`Captures`](../../index.md#captures)

- <span id="capturematches-iterator-count"></span>`fn count(self) -> usize`

##### `impl<U> TryFrom for CaptureMatches<'r, 'h>`

- <span id="capturematches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturematches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaptureMatches<'r, 'h>`

- <span id="capturematches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturematches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Split<'r, 'h>`

```rust
struct Split<'r, 'h> {
    haystack: &'h str,
    it: meta::Split<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2281-2284`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2281-L2284)*

An iterator over all substrings delimited by a regex match.

`'r` is the lifetime of the compiled regular expression and `'h` is the
lifetime of the byte string being split.

This iterator is created by `Regex::split`.

# Time complexity

Note that since an iterator runs potentially many searches on the haystack
and since each search has worst case `O(m * n)` time complexity, the
overall worst case time complexity for iteration is `O(m * n^2)`.

#### Trait Implementations

##### `impl Any for Split<'r, 'h>`

- <span id="split-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Split<'r, 'h>`

- <span id="split-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Split<'r, 'h>`

- <span id="split-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for Split<'r, 'h>`

- <span id="split-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Split<'r, 'h>`

- <span id="split-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for Split<'r, 'h>`

##### `impl<U> Into for Split<'r, 'h>`

- <span id="split-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for Split<'r, 'h>`

- <span id="split-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="split-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="split-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for Split<'r, 'h>`

- <span id="split-iterator-type-item"></span>`type Item = &'h str`

- <span id="split-iterator-next"></span>`fn next(&mut self) -> Option<&'h str>`

##### `impl<U> TryFrom for Split<'r, 'h>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split<'r, 'h>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    haystack: &'h str,
    it: meta::SplitN<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2316-2319`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2316-L2319)*

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

##### `impl Any for SplitN<'r, 'h>`

- <span id="splitn-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SplitN<'r, 'h>`

- <span id="splitn-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SplitN<'r, 'h>`

- <span id="splitn-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for SplitN<'r, 'h>`

- <span id="splitn-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for SplitN<'r, 'h>`

- <span id="splitn-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for SplitN<'r, 'h>`

##### `impl<U> Into for SplitN<'r, 'h>`

- <span id="splitn-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SplitN<'r, 'h>`

- <span id="splitn-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="splitn-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="splitn-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SplitN<'r, 'h>`

- <span id="splitn-iterator-type-item"></span>`type Item = &'h str`

- <span id="splitn-iterator-next"></span>`fn next(&mut self) -> Option<&'h str>`

- <span id="splitn-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<U> TryFrom for SplitN<'r, 'h>`

- <span id="splitn-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="splitn-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SplitN<'r, 'h>`

- <span id="splitn-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="splitn-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `CaptureNames<'r>`

```rust
struct CaptureNames<'r>(captures::GroupInfoPatternNames<'r>);
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2348`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2348)*

An iterator over the names of all capture groups in a regex.

This iterator yields values of type `Option<&str>` in order of the opening
capture group parenthesis in the regex pattern. `None` is yielded for
groups with no name. The first element always corresponds to the implicit
and unnamed group for the overall match.

`'r` is the lifetime of the compiled regular expression.

This iterator is created by `Regex::capture_names`.

#### Trait Implementations

##### `impl Any for CaptureNames<'r>`

- <span id="capturenames-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for CaptureNames<'r>`

- <span id="capturenames-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for CaptureNames<'r>`

- <span id="capturenames-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for CaptureNames<'r>`

- <span id="capturenames-clone"></span>`fn clone(&self) -> CaptureNames<'r>` — [`CaptureNames`](../../index.md#capturenames)

##### `impl CloneToUninit for CaptureNames<'r>`

- <span id="capturenames-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for CaptureNames<'r>`

- <span id="capturenames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for CaptureNames<'r>`

##### `impl<T> From for CaptureNames<'r>`

- <span id="capturenames-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for CaptureNames<'r>`

##### `impl<U> Into for CaptureNames<'r>`

- <span id="capturenames-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for CaptureNames<'r>`

- <span id="capturenames-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="capturenames-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="capturenames-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for CaptureNames<'r>`

- <span id="capturenames-iterator-type-item"></span>`type Item = Option<&'r str>`

- <span id="capturenames-iterator-next"></span>`fn next(&mut self) -> Option<Option<&'r str>>`

- <span id="capturenames-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="capturenames-iterator-count"></span>`fn count(self) -> usize`

##### `impl ToOwned for CaptureNames<'r>`

- <span id="capturenames-toowned-type-owned"></span>`type Owned = T`

- <span id="capturenames-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="capturenames-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for CaptureNames<'r>`

- <span id="capturenames-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="capturenames-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for CaptureNames<'r>`

- <span id="capturenames-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="capturenames-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SubCaptureMatches<'c, 'h>`

```rust
struct SubCaptureMatches<'c, 'h> {
    haystack: &'h str,
    it: captures::CapturesPatternIter<'c>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2390-2393`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2390-L2393)*

An iterator over all group matches in a [`Captures`](../../index.md) value.

This iterator yields values of type `Option<Match<'h>>`, where `'h` is the
lifetime of the haystack that the matches are for. The order of elements
yielded corresponds to the order of the opening parenthesis for the group
in the regex pattern. `None` is yielded for groups that did not participate
in the match.

The first element always corresponds to the implicit group for the overall
match. Since this iterator is created by a [`Captures`](../../index.md) value, and a
`Captures` value is only created when a match occurs, it follows that the
first element yielded by this iterator is guaranteed to be non-`None`.

The lifetime `'c` corresponds to the lifetime of the `Captures` value that
created this iterator, and the lifetime `'h` corresponds to the originally
matched haystack.

#### Trait Implementations

##### `impl Any for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-clone"></span>`fn clone(&self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](../../index.md#subcapturematches)

##### `impl CloneToUninit for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ExactSizeIterator for SubCaptureMatches<'c, 'h>`

##### `impl<T> From for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl FusedIterator for SubCaptureMatches<'c, 'h>`

##### `impl<U> Into for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl IntoIterator for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="subcapturematches-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="subcapturematches-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-iterator-type-item"></span>`type Item = Option<Match<'h>>`

- <span id="subcapturematches-iterator-next"></span>`fn next(&mut self) -> Option<Option<Match<'h>>>` — [`Match`](../../index.md#match)

- <span id="subcapturematches-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="subcapturematches-iterator-count"></span>`fn count(self) -> usize`

##### `impl ToOwned for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-toowned-type-owned"></span>`type Owned = T`

- <span id="subcapturematches-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="subcapturematches-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="subcapturematches-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="subcapturematches-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `ReplacerRef<'a, R: ?Sized>`

```rust
struct ReplacerRef<'a, R: ?Sized>(&'a mut R);
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2567`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2567)*

A by-reference adaptor for a [`Replacer`](../../index.md).

This permits reusing the same `Replacer` value in multiple calls to a
replacement routine like `Regex::replace_all`.

This type is created by `Replacer::by_ref`.

#### Trait Implementations

##### `impl Any for ReplacerRef<'a, R>`

- <span id="replacerref-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ReplacerRef<'a, R>`

- <span id="replacerref-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ReplacerRef<'a, R>`

- <span id="replacerref-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<R: fmt::Debug + ?Sized> Debug for ReplacerRef<'a, R>`

- <span id="replacerref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ReplacerRef<'a, R>`

- <span id="replacerref-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ReplacerRef<'a, R>`

- <span id="replacerref-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<R: Replacer + ?Sized + 'a> Replacer for ReplacerRef<'a, R>`

- <span id="replacerref-replacer-replace-append"></span>`fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String)` — [`Captures`](../../index.md#captures)

- <span id="replacerref-replacer-no-expansion"></span>`fn no_expansion(&mut self) -> Option<Cow<'_, str>>`

##### `impl<U> TryFrom for ReplacerRef<'a, R>`

- <span id="replacerref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="replacerref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReplacerRef<'a, R>`

- <span id="replacerref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="replacerref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NoExpand<'s>`

```rust
struct NoExpand<'s>(&'s str);
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2599`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2599)*

A helper type for forcing literal string replacement.

It can be used with routines like `Regex::replace` and
`Regex::replace_all` to do a literal string replacement without expanding
`$name` to their corresponding capture groups. This can be both convenient
(to avoid escaping `$`, for example) and faster (since capture groups
don't need to be found).

`'s` is the lifetime of the literal string to use.

# Example

```rust
use regex::{NoExpand, Regex};

let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();
let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
assert_eq!(result, "$2 $last");
```

#### Trait Implementations

##### `impl Any for NoExpand<'s>`

- <span id="noexpand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoExpand<'s>`

- <span id="noexpand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoExpand<'s>`

- <span id="noexpand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NoExpand<'s>`

- <span id="noexpand-clone"></span>`fn clone(&self) -> NoExpand<'s>` — [`NoExpand`](../../index.md#noexpand)

##### `impl CloneToUninit for NoExpand<'s>`

- <span id="noexpand-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for NoExpand<'s>`

- <span id="noexpand-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NoExpand<'s>`

- <span id="noexpand-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NoExpand<'s>`

- <span id="noexpand-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl Replacer for NoExpand<'s>`

- <span id="noexpand-replacer-replace-append"></span>`fn replace_append(&mut self, _: &Captures<'_>, dst: &mut String)` — [`Captures`](../../index.md#captures)

- <span id="noexpand-replacer-no-expansion"></span>`fn no_expansion(&mut self) -> Option<Cow<'_, str>>`

##### `impl ToOwned for NoExpand<'s>`

- <span id="noexpand-toowned-type-owned"></span>`type Owned = T`

- <span id="noexpand-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="noexpand-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for NoExpand<'s>`

- <span id="noexpand-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="noexpand-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NoExpand<'s>`

- <span id="noexpand-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="noexpand-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Replacer`

```rust
trait Replacer { ... }
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2452-2498`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2452-L2498)*

A trait for types that can be used to replace matches in a haystack.

In general, users of this crate shouldn't need to implement this trait,
since implementations are already provided for `&str` along with other
variants of string types, as well as `FnMut(&Captures) -> String` (or any
`FnMut(&Captures) -> T` where `T: AsRef<str>`). Those cover most use cases,
but callers can implement this trait directly if necessary.

# Example

This example shows a basic implementation of  the `Replacer` trait. This
can be done much more simply using the replacement string interpolation
support (e.g., `$first $last`), but this approach avoids needing to parse
the replacement string at all.

```rust
use regex::{Captures, Regex, Replacer};

struct NameSwapper;

impl Replacer for NameSwapper {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        dst.push_str(&caps["first"]);
        dst.push_str(" ");
        dst.push_str(&caps["last"]);
    }
}

let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();
let result = re.replace("Springsteen, Bruce", NameSwapper);
assert_eq!(result, "Bruce Springsteen");
```

#### Required Methods

- `fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String)`

  Appends possibly empty data to `dst` to replace the current match.

#### Provided Methods

- `fn no_expansion<'r>(self: &'r mut Self) -> Option<Cow<'r, str>>`

  Return a fixed unchanging replacement string.

- `fn by_ref<'r>(self: &'r mut Self) -> ReplacerRef<'r, Self>`

  Returns a type that implements `Replacer`, but that borrows and wraps

#### Implementors

- [`NoExpand`](../../index.md#noexpand)
- [`ReplacerRef`](../../index.md#replacerref)
- `&'a alloc::borrow::Cow<'a, str>`
- `&'a alloc::string::String`
- `&'a str`
- `F`
- `alloc::borrow::Cow<'a, str>`
- `alloc::string::String`

## Functions

### `no_expansion`

```rust
fn no_expansion<T: AsRef<str>>(replacement: &T) -> Option<alloc::borrow::Cow<'_, str>>
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2619-2625`](../../../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2619-L2625)*

Quickly checks the given replacement string for whether interpolation
should be done on it. It returns `None` if a `$` was found anywhere in the
given string, which suggests interpolation needs to be done. But if there's
no `$` anywhere, then interpolation definitely does not need to be done. In
that case, the given string is returned as a borrowed `Cow`.

This is meant to be used to implement the `Replacer::no_expansion` method
in its various trait impls.

