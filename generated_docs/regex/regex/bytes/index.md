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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:99-102`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L99-L102)*

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

  use regex::bytes::Regex;

  

  // An Invalid pattern because of an unclosed parenthesis

  assert!(Regex::new(r"foo(bar").is_err());

  // An invalid pattern because the regex would be too big

  // because Unicode tends to inflate things.

  assert!(Regex::new(r"\w{1000}").is_err());

  // Disabling Unicode can make the regex much smaller,

  // potentially by up to or more than an order of magnitude.

  assert!(Regex::new(r"(?-u:\w){1000}").is_ok());

  ```

- <span id="regex-is-match"></span>`fn is_match(&self, haystack: &[u8]) -> bool`

  Returns true if and only if there is a match for the regex anywhere

  in the haystack given.

  

  It is recommended to use this method if all you need to do is test

  whether a match exists, since the underlying matching engine may be

  able to do less work.

  

  # Example

  

  Test if some haystack contains at least one word with exactly 13

  Unicode word characters:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = b"I categorically deny having triskaidekaphobia.";

  assert!(re.is_match(hay));

  ```

- <span id="regex-find"></span>`fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>>` — [`Match`](#match)

  This routine searches for the first match of this regex in the

  haystack given, and if found, returns a [`Match`](#match). The `Match`

  provides access to both the byte offsets of the match and the actual

  substring that matched.

  

  Note that this should only be used if you want to find the entire

  match. If instead you just want to test the existence of a match,

  it's potentially faster to use `Regex::is_match(hay)` instead of

  `Regex::find(hay).is_some()`.

  

  # Example

  

  Find the first word with exactly 13 Unicode word characters:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = b"I categorically deny having triskaidekaphobia.";

  let mat = re.find(hay).unwrap();

  assert_eq!(2..15, mat.range());

  assert_eq!(b"categorically", mat.as_bytes());

  ```

- <span id="regex-find-iter"></span>`fn find_iter<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> Matches<'r, 'h>` — [`Matches`](#matches)

  Returns an iterator that yields successive non-overlapping matches in

  the given haystack. The iterator yields values of type [`Match`](#match).

  

  # Time complexity

  

  Note that since `find_iter` runs potentially many searches on the

  haystack and since each search has worst case `O(m * n)` time

  complexity, the overall worst case time complexity for iteration is

  `O(m * n^2)`.

  

  # Example

  

  Find every word with exactly 13 Unicode word characters:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = b"Retroactively relinquishing remunerations is reprehensible.";

  let matches: Vec<_> = re.find_iter(hay).map(|m| m.as_bytes()).collect();

  assert_eq!(matches, vec![

      &b"Retroactively"[..],

      &b"relinquishing"[..],

      &b"remunerations"[..],

      &b"reprehensible"[..],

  ]);

  ```

- <span id="regex-captures"></span>`fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>>` — [`Captures`](#captures)

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

  strings looking like that, while also extracting the movie name and its

  release year separately. The example below shows how to do that.

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();

  let hay = b"Not my favorite movie: 'Citizen Kane' (1941).";

  let caps = re.captures(hay).unwrap();

  assert_eq!(caps.get(0).unwrap().as_bytes(), b"'Citizen Kane' (1941)");

  assert_eq!(caps.get(1).unwrap().as_bytes(), b"Citizen Kane");

  assert_eq!(caps.get(2).unwrap().as_bytes(), b"1941");

  // You can also access the groups by index using the Index notation.

  // Note that this will panic on an invalid index. In this case, these

  // accesses are always correct because the overall regex will only

  // match when these capture groups match.

  assert_eq!(&caps[0], b"'Citizen Kane' (1941)");

  assert_eq!(&caps[1], b"Citizen Kane");

  assert_eq!(&caps[2], b"1941");

  ```

  

  Note that the full match is at capture group `0`. Each subsequent

  capture group is indexed by the order of its opening `(`.

  

  We can make this example a bit clearer by using *named* capture groups:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>\d{4})\)").unwrap();

  let hay = b"Not my favorite movie: 'Citizen Kane' (1941).";

  let caps = re.captures(hay).unwrap();

  assert_eq!(caps.get(0).unwrap().as_bytes(), b"'Citizen Kane' (1941)");

  assert_eq!(caps.name("title").unwrap().as_bytes(), b"Citizen Kane");

  assert_eq!(caps.name("year").unwrap().as_bytes(), b"1941");

  // You can also access the groups by name using the Index notation.

  // Note that this will panic on an invalid group name. In this case,

  // these accesses are always correct because the overall regex will

  // only match when these capture groups match.

  assert_eq!(&caps[0], b"'Citizen Kane' (1941)");

  assert_eq!(&caps["title"], b"Citizen Kane");

  assert_eq!(&caps["year"], b"1941");

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();

  let hay = b"Not my favorite movie: 'Citizen Kane' (1941).";

  let (full, [title, year]) = re.captures(hay).unwrap().extract();

  assert_eq!(full, b"'Citizen Kane' (1941)");

  assert_eq!(title, b"Citizen Kane");

  assert_eq!(year, b"1941");

  ```

- <span id="regex-captures-iter"></span>`fn captures_iter<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> CaptureMatches<'r, 'h>` — [`CaptureMatches`](#capturematches)

  Returns an iterator that yields successive non-overlapping matches in

  the given haystack. The iterator yields values of type [`Captures`](#captures).

  

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"'([^']+)'\s+\(([0-9]{4})\)").unwrap();

  let hay = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

  let mut movies = vec![];

  for (_, [title, year]) in re.captures_iter(hay).map(|c| c.extract()) {

      // OK because [0-9]{4} can only match valid UTF-8.

      let year = std::str::from_utf8(year).unwrap();

      movies.push((title, year.parse::<i64>()?));

  }

  assert_eq!(movies, vec![

      (&b"Citizen Kane"[..], 1941),

      (&b"The Wizard of Oz"[..], 1939),

      (&b"M"[..], 1931),

  ]);

  Ok::<(), Box<dyn std::error::Error>>(())

  ```

  

  Or with named groups:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"'(?<title>[^']+)'\s+\((?<year>[0-9]{4})\)").unwrap();

  let hay = b"'Citizen Kane' (1941), 'The Wizard of Oz' (1939), 'M' (1931).";

  let mut it = re.captures_iter(hay);

  

  let caps = it.next().unwrap();

  assert_eq!(&caps["title"], b"Citizen Kane");

  assert_eq!(&caps["year"], b"1941");

  

  let caps = it.next().unwrap();

  assert_eq!(&caps["title"], b"The Wizard of Oz");

  assert_eq!(&caps["year"], b"1939");

  

  let caps = it.next().unwrap();

  assert_eq!(&caps["title"], b"M");

  assert_eq!(&caps["year"], b"1931");

  ```

- <span id="regex-split"></span>`fn split<'r, 'h>(self: &'r Self, haystack: &'h [u8]) -> Split<'r, 'h>` — [`Split`](#split)

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"[ \t]+").unwrap();

  let hay = b"a b \t  c\td    e";

  let fields: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(fields, vec![

      &b"a"[..], &b"b"[..], &b"c"[..], &b"d"[..], &b"e"[..],

  ]);

  ```

  

  # Example: more cases

  

  Basic usage:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r" ").unwrap();

  let hay = b"Mary had a little lamb";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![

      &b"Mary"[..], &b"had"[..], &b"a"[..], &b"little"[..], &b"lamb"[..],

  ]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![&b""[..]]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"lionXXtigerXleopard";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![

      &b"lion"[..], &b""[..], &b"tiger"[..], &b"leopard"[..],

  ]);

  

  let re = Regex::new(r"::").unwrap();

  let hay = b"lion::tiger::leopard";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![&b"lion"[..], &b"tiger"[..], &b"leopard"[..]]);

  ```

  

  If a haystack contains multiple contiguous matches, you will end up

  with empty spans yielded by the iterator:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"XXXXaXXbXc";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![

      &b""[..], &b""[..], &b""[..], &b""[..],

      &b"a"[..], &b""[..], &b"b"[..], &b"c"[..],

  ]);

  

  let re = Regex::new(r"/").unwrap();

  let hay = b"(///)";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![&b"("[..], &b""[..], &b""[..], &b")"[..]]);

  ```

  

  Separators at the start or end of a haystack are neighbored by empty

  substring.

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"0").unwrap();

  let hay = b"010";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![&b""[..], &b"1"[..], &b""[..]]);

  ```

  

  When the regex can match the empty string, it splits at every byte

  position in the haystack. This includes between all UTF-8 code units.

  (The top-level [`Regex::split`](crate::Regex::split) will only split

  at valid UTF-8 boundaries.)

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"").unwrap();

  let hay = "☃".as_bytes();

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![

      &[][..], &[b'\xE2'][..], &[b'\x98'][..], &[b'\x83'][..], &[][..],

  ]);

  ```

  

  Contiguous separators (commonly shows up with whitespace), can lead to

  possibly surprising behavior. For example, this code is correct:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r" ").unwrap();

  let hay = b"    a  b c";

  let got: Vec<&[u8]> = re.split(hay).collect();

  assert_eq!(got, vec![

      &b""[..], &b""[..], &b""[..], &b""[..],

      &b"a"[..], &b""[..], &b"b"[..], &b"c"[..],

  ]);

  ```

  

  It does *not* give you `["a", "b", "c"]`. For that behavior, you'd want

  to match contiguous space characters:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r" +").unwrap();

  let hay = b"    a  b c";

  let got: Vec<&[u8]> = re.split(hay).collect();

  // N.B. This does still include a leading empty span because ' +'

  // matches at the beginning of the haystack.

  assert_eq!(got, vec![&b""[..], &b"a"[..], &b"b"[..], &b"c"[..]]);

  ```

- <span id="regex-splitn"></span>`fn splitn<'r, 'h>(self: &'r Self, haystack: &'h [u8], limit: usize) -> SplitN<'r, 'h>` — [`SplitN`](#splitn)

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"\W+").unwrap();

  let hay = b"Hey! How are you?";

  let fields: Vec<&[u8]> = re.splitn(hay, 3).collect();

  assert_eq!(fields, vec![&b"Hey"[..], &b"How"[..], &b"are you?"[..]]);

  ```

  

  # Examples: more cases

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r" ").unwrap();

  let hay = b"Mary had a little lamb";

  let got: Vec<&[u8]> = re.splitn(hay, 3).collect();

  assert_eq!(got, vec![&b"Mary"[..], &b"had"[..], &b"a little lamb"[..]]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"";

  let got: Vec<&[u8]> = re.splitn(hay, 3).collect();

  assert_eq!(got, vec![&b""[..]]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"lionXXtigerXleopard";

  let got: Vec<&[u8]> = re.splitn(hay, 3).collect();

  assert_eq!(got, vec![&b"lion"[..], &b""[..], &b"tigerXleopard"[..]]);

  

  let re = Regex::new(r"::").unwrap();

  let hay = b"lion::tiger::leopard";

  let got: Vec<&[u8]> = re.splitn(hay, 2).collect();

  assert_eq!(got, vec![&b"lion"[..], &b"tiger::leopard"[..]]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"abcXdef";

  let got: Vec<&[u8]> = re.splitn(hay, 1).collect();

  assert_eq!(got, vec![&b"abcXdef"[..]]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"abcdef";

  let got: Vec<&[u8]> = re.splitn(hay, 2).collect();

  assert_eq!(got, vec![&b"abcdef"[..]]);

  

  let re = Regex::new(r"X").unwrap();

  let hay = b"abcXdef";

  let got: Vec<&[u8]> = re.splitn(hay, 0).collect();

  assert!(got.is_empty());

  ```

- <span id="regex-replace"></span>`fn replace<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>`

  Replaces the leftmost-first match in the given haystack with the

  replacement provided. The replacement can be a regular string (where

  `$N` and `$name` are expanded to match capture groups) or a function

  that takes a [`Captures`](#captures) and returns the replaced string.

  

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"[^01]+").unwrap();

  assert_eq!(re.replace(b"1078910", b""), &b"1010"[..]);

  ```

  

  But anything satisfying the [`Replacer`](#replacer) trait will work. For example,

  a closure of type `|&Captures| -> String` provides direct access to the

  captures corresponding to a match. This allows one to access capturing

  group matches easily:

  

  ```rust

  use regex::bytes::{Captures, Regex};

  

  let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();

  let result = re.replace(b"Springsteen, Bruce", |caps: &Captures| {

      let mut buf = vec![];

      buf.extend_from_slice(&caps[2]);

      buf.push(b' ');

      buf.extend_from_slice(&caps[1]);

      buf

  });

  assert_eq!(result, &b"Bruce Springsteen"[..]);

  ```

  

  But this is a bit cumbersome to use all the time. Instead, a simple

  syntax is supported (as described above) that expands `$name` into the

  corresponding capture group. Here's the last example, but using this

  expansion technique with named capture groups:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(?<last>[^,\s]+),\s+(?<first>\S+)").unwrap();

  let result = re.replace(b"Springsteen, Bruce", b"$first $last");

  assert_eq!(result, &b"Bruce Springsteen"[..]);

  ```

  

  Note that using `$2` instead of `$first` or `$1` instead of `$last`

  would produce the same result. To write a literal `$` use `$$`.

  

  Sometimes the replacement string requires use of curly braces to

  delineate a capture group replacement when it is adjacent to some other

  literal text. For example, if we wanted to join two words together with

  an underscore:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(?<first>\w+)\s+(?<second>\w+)").unwrap();

  let result = re.replace(b"deep fried", b"${first}_$second");

  assert_eq!(result, &b"deep_fried"[..]);

  ```

  

  Without the curly braces, the capture group name `first_` would be

  used, and since it doesn't exist, it would be replaced with the empty

  string.

  

  Finally, sometimes you just want to replace a literal string with no

  regard for capturing group expansion. This can be done by wrapping a

  string with [`NoExpand`](#noexpand):

  

  ```rust

  use regex::bytes::{NoExpand, Regex};

  

  let re = Regex::new(r"(?<last>[^,\s]+),\s+(\S+)").unwrap();

  let result = re.replace(b"Springsteen, Bruce", NoExpand(b"$2 $last"));

  assert_eq!(result, &b"$2 $last"[..]);

  ```

  

  Using `NoExpand` may also be faster, since the replacement string won't

  need to be parsed for the `$` syntax.

- <span id="regex-replace-all"></span>`fn replace_all<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]>`

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

  use regex::bytes::{Captures, Regex};

  

  fn replace_all<E>(

      re: &Regex,

      haystack: &[u8],

      replacement: impl Fn(&Captures) -> Result<Vec<u8>, E>,

  ) -> Result<Vec<u8>, E> {

      let mut new = Vec::with_capacity(haystack.len());

      let mut last_match = 0;

      for caps in re.captures_iter(haystack) {

          let m = caps.get(0).unwrap();

          new.extend_from_slice(&haystack[last_match..m.start()]);

          new.extend_from_slice(&replacement(&caps)?);

          last_match = m.end();

      }

      new.extend_from_slice(&haystack[last_match..]);

      Ok(new)

  }

  

  // Let's replace each word with the number of bytes in that word.

  // But if we see a word that is "too long," we'll give up.

  let re = Regex::new(r"\w+").unwrap();

  let replacement = |caps: &Captures| -> Result<Vec<u8>, &'static str> {

      if caps[0].len() >= 5 {

          return Err("word too long");

      }

      Ok(caps[0].len().to_string().into_bytes())

  };

  assert_eq!(

      Ok(b"2 3 3 3?".to_vec()),

      replace_all(&re, b"hi how are you?", &replacement),

  );

  assert!(replace_all(&re, b"hi there", &replacement).is_err());

  ```

  

  # Example

  

  This example shows how to flip the order of whitespace (excluding line

  terminators) delimited fields, and normalizes the whitespace that

  delimits the fields:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();

  let hay = b"

  Greetings  1973

  Wild\t1973

  BornToRun\t\t\t\t1975

  Darkness                    1978

  TheRiver 1980

  ";

  let new = re.replace_all(hay, b"$2 $1");

  assert_eq!(new, &b"

  1973 Greetings

  1973 Wild

  1975 BornToRun

  1978 Darkness

  1980 TheRiver

  "[..]);

  ```

- <span id="regex-replacen"></span>`fn replacen<'h, R: Replacer>(&self, haystack: &'h [u8], limit: usize, rep: R) -> Cow<'h, [u8]>`

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(?m)^(\S+)[\s--\r\n]+(\S+)$").unwrap();

  let hay = b"

  Greetings  1973

  Wild\t1973

  BornToRun\t\t\t\t1975

  Darkness                    1978

  TheRiver 1980

  ";

  let new = re.replacen(hay, 2, b"$2 $1");

  assert_eq!(new, &b"

  1973 Greetings

  1973 Wild

  BornToRun\t\t\t\t1975

  Darkness                    1978

  TheRiver 1980

  "[..]);

  ```

#### Trait Implementations

##### `impl Any for Regex`

- <span id="regex-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Regex`

- <span id="regex-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Regex`

- <span id="regex-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Regex`

- <span id="regex-clone"></span>`fn clone(&self) -> Regex` — [`Regex`](#regex)

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

- <span id="regex-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Regex, Error>` — [`Regex`](#regex), [`Error`](../../error/index.md#error)

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
    haystack: &'h [u8],
    start: usize,
    end: usize,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:1483-1487`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L1483-L1487)*

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

  Returns the byte offset of the start of the match in the haystack. The

  start of the match corresponds to the position where the match begins

  and includes the first byte in the match.

  

  It is guaranteed that `Match::start() <= Match::end()`.

  

  Unlike the top-level `Match` type, the start offset may appear anywhere

  in the haystack. This includes between the code units of a UTF-8

  encoded Unicode scalar value.

- <span id="match-end"></span>`fn end(&self) -> usize`

  Returns the byte offset of the end of the match in the haystack. The

  end of the match corresponds to the byte immediately following the last

  byte in the match. This means that `&slice[start..end]` works as one

  would expect.

  

  It is guaranteed that `Match::start() <= Match::end()`.

  

  Unlike the top-level `Match` type, the start offset may appear anywhere

  in the haystack. This includes between the code units of a UTF-8

  encoded Unicode scalar value.

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

- <span id="match-as-bytes"></span>`fn as_bytes(&self) -> &'h [u8]`

  Returns the substring of the haystack that matched.

- <span id="match-new"></span>`fn new(haystack: &'h [u8], start: usize, end: usize) -> Match<'h>` — [`Match`](#match)

  Creates a new match from the given haystack and byte offsets.

#### Trait Implementations

##### `impl Any for Match<'h>`

- <span id="match-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Match<'h>`

- <span id="match-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Match<'h>`

- <span id="match-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Match<'h>`

- <span id="match-clone"></span>`fn clone(&self) -> Match<'h>` — [`Match`](#match)

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

- <span id="match-partialeq-eq"></span>`fn eq(&self, other: &Match<'h>) -> bool` — [`Match`](#match)

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
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:1632-1636`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L1632-L1636)*

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

  Returns the `Match` associated with the capture group at index `i`. If

  `i` does not correspond to a capture group, or if the capture group did

  not participate in the match, then `None` is returned.

  

  When `i == 0`, this is guaranteed to return a non-`None` value.

  

  # Examples

  

  Get the substring that matched with a default of an empty string if the

  group didn't participate in the match:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();

  let caps = re.captures(b"abc123").unwrap();

  

  let substr1 = caps.get(1).map_or(&b""[..], |m| m.as_bytes());

  let substr2 = caps.get(2).map_or(&b""[..], |m| m.as_bytes());

  assert_eq!(substr1, b"123");

  assert_eq!(substr2, b"");

  ```

- <span id="captures-get-match"></span>`fn get_match(&self) -> Match<'h>` — [`Match`](#match)

  Return the overall match for the capture.

  

  This returns the match for index `0`. That is it is equivalent to

  `m.get(0).unwrap()`

  

  # Example

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"[a-z]+([0-9]+)").unwrap();

  let caps = re.captures(b"   abc123-def").unwrap();

  

  assert_eq!(caps.get_match().as_bytes(), b"abc123");

  ```

- <span id="captures-name"></span>`fn name(&self, name: &str) -> Option<Match<'h>>` — [`Match`](#match)

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

  use regex::bytes::Regex;

  

  let re = Regex::new(

      r"[a-z]+(?:(?<numbers>[0-9]+)|(?<letters>[A-Z]+))",

  ).unwrap();

  let caps = re.captures(b"abc123").unwrap();

  

  let numbers = caps.name("numbers").map_or(&b""[..], |m| m.as_bytes());

  let letters = caps.name("letters").map_or(&b""[..], |m| m.as_bytes());

  assert_eq!(numbers, b"123");

  assert_eq!(letters, b"");

  ```

- <span id="captures-extract"></span>`fn extract<const N: usize>(&self) -> (&'h [u8], [&'h [u8]; N])`

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();

  let hay = b"On 2010-03-14, I became a Tennessee lamb.";

  let Some((full, [year, month, day])) =

      re.captures(hay).map(|caps| caps.extract()) else { return };

  assert_eq!(b"2010-03-14", full);

  assert_eq!(b"2010", year);

  assert_eq!(b"03", month);

  assert_eq!(b"14", day);

  ```

  

  # Example: iteration

  

  This example shows how to use this method when iterating over all

  `Captures` matches in a haystack.

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();

  let hay = b"1973-01-05, 1975-08-25 and 1980-10-18";

  

  let mut dates: Vec<(&[u8], &[u8], &[u8])> = vec![];

  for (_, [y, m, d]) in re.captures_iter(hay).map(|c| c.extract()) {

      dates.push((y, m, d));

  }

  assert_eq!(dates, vec![

      (&b"1973"[..], &b"01"[..], &b"05"[..]),

      (&b"1975"[..], &b"08"[..], &b"25"[..]),

      (&b"1980"[..], &b"10"[..], &b"18"[..]),

  ]);

  ```

  

  # Example: parsing different formats

  

  This API is particularly useful when you need to extract a particular

  value that might occur in a different format. Consider, for example,

  an identifier that might be in double quotes or single quotes:

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r#"id:(?:"([^"]+)"|'([^']+)')"#).unwrap();

  let hay = br#"The first is id:"foo" and the second is id:'bar'."#;

  let mut ids = vec![];

  for (_, [id]) in re.captures_iter(hay).map(|c| c.extract()) {

      ids.push(id);

  }

  assert_eq!(ids, vec![b"foo", b"bar"]);

  ```

- <span id="captures-expand"></span>`fn expand(&self, replacement: &[u8], dst: &mut Vec<u8>)`

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

  use regex::bytes::Regex;

  

  let re = Regex::new(

      r"(?<day>[0-9]{2})-(?<month>[0-9]{2})-(?<year>[0-9]{4})",

  ).unwrap();

  let hay = b"On 14-03-2010, I became a Tennessee lamb.";

  let caps = re.captures(hay).unwrap();

  

  let mut dst = vec![];

  caps.expand(b"year=$year, month=$month, day=$day", &mut dst);

  assert_eq!(dst, b"year=2010, month=03, day=14");

  ```

- <span id="captures-iter"></span>`fn iter<'c>(self: &'c Self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](#subcapturematches)

  Returns an iterator over all capture groups. This includes both

  matching and non-matching groups.

  

  The iterator always yields at least one matching group: the first group

  (at index `0`) with no name. Subsequent groups are returned in the order

  of their opening parenthesis in the regex.

  

  The elements yielded have type `Option<Match<'h>>`, where a non-`None`

  value is present if the capture group matches.

  

  # Example

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();

  let caps = re.captures(b"AZ").unwrap();

  

  let mut it = caps.iter();

  assert_eq!(it.next().unwrap().map(|m| m.as_bytes()), Some(&b"AZ"[..]));

  assert_eq!(it.next().unwrap().map(|m| m.as_bytes()), Some(&b"A"[..]));

  assert_eq!(it.next().unwrap().map(|m| m.as_bytes()), None);

  assert_eq!(it.next().unwrap().map(|m| m.as_bytes()), Some(&b"Z"[..]));

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(\w)(\d)?(\w)").unwrap();

  let caps = re.captures(b"AZ").unwrap();

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

- <span id="captures-index-type-output"></span>`type Output = [u8]`

- <span id="captures-index"></span>`fn index<'a>(self: &'a Self, i: usize) -> &'a [u8]`

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2084`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2084)*

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

  Returns the start and end byte offsets of the capture group at index

  `i`. This returns `None` if `i` is not a valid capture group or if the

  capture group did not match.

  

  # Example

  

  ```rust

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();

  let mut locs = re.capture_locations();

  re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();

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

  use regex::bytes::Regex;

  

  let re = Regex::new(r"(?<first>\w+)\s+(?<last>\w+)").unwrap();

  let mut locs = re.capture_locations();

  assert_eq!(3, locs.len());

  re.captures_read(&mut locs, b"Bruce Springsteen").unwrap();

  assert_eq!(3, locs.len());

  ```

  

  Notice that the length is always at least `1`, regardless of the regex:

  

  ```rust

  use regex::bytes::Regex;

  

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

- <span id="capturelocations-clone"></span>`fn clone(&self) -> CaptureLocations` — [`CaptureLocations`](#capturelocations)

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
    haystack: &'h [u8],
    it: meta::FindMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2184-2187`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2184-L2187)*

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

- <span id="matches-iterator-next"></span>`fn next(&mut self) -> Option<Match<'h>>` — [`Match`](#match)

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
    haystack: &'h [u8],
    it: meta::CapturesMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2228-2231`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2228-L2231)*

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

- <span id="capturematches-iterator-next"></span>`fn next(&mut self) -> Option<Captures<'h>>` — [`Captures`](#captures)

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
    haystack: &'h [u8],
    it: meta::Split<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2272-2275`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2272-L2275)*

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

- <span id="split-iterator-type-item"></span>`type Item = &'h [u8]`

- <span id="split-iterator-next"></span>`fn next(&mut self) -> Option<&'h [u8]>`

##### `impl<U> TryFrom for Split<'r, 'h>`

- <span id="split-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="split-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Split<'r, 'h>`

- <span id="split-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="split-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `SplitN<'r, 'h>`

```rust
struct SplitN<'r, 'h> {
    haystack: &'h [u8],
    it: meta::SplitN<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2307-2310`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2307-L2310)*

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

- <span id="splitn-iterator-type-item"></span>`type Item = &'h [u8]`

- <span id="splitn-iterator-next"></span>`fn next(&mut self) -> Option<&'h [u8]>`

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2339`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2339)*

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

- <span id="capturenames-clone"></span>`fn clone(&self) -> CaptureNames<'r>` — [`CaptureNames`](#capturenames)

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
    haystack: &'h [u8],
    it: captures::CapturesPatternIter<'c>,
}
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2381-2384`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2381-L2384)*

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

##### `impl Any for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for SubCaptureMatches<'c, 'h>`

- <span id="subcapturematches-clone"></span>`fn clone(&self) -> SubCaptureMatches<'c, 'h>` — [`SubCaptureMatches`](#subcapturematches)

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

- <span id="subcapturematches-iterator-next"></span>`fn next(&mut self) -> Option<Option<Match<'h>>>` — [`Match`](#match)

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2579`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2579)*

A by-reference adaptor for a [`Replacer`](#replacer).

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

- <span id="replacerref-replacer-replace-append"></span>`fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut Vec<u8>)` — [`Captures`](#captures)

- <span id="replacerref-replacer-no-expansion"></span>`fn no_expansion<'r>(self: &'r mut Self) -> Option<Cow<'r, [u8]>>`

##### `impl<U> TryFrom for ReplacerRef<'a, R>`

- <span id="replacerref-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="replacerref-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ReplacerRef<'a, R>`

- <span id="replacerref-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="replacerref-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NoExpand<'s>`

```rust
struct NoExpand<'s>(&'s [u8]);
```

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2611`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2611)*

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

##### `impl Any for NoExpand<'s>`

- <span id="noexpand-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NoExpand<'s>`

- <span id="noexpand-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NoExpand<'s>`

- <span id="noexpand-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for NoExpand<'s>`

- <span id="noexpand-clone"></span>`fn clone(&self) -> NoExpand<'s>` — [`NoExpand`](#noexpand)

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

- <span id="noexpand-replacer-replace-append"></span>`fn replace_append(&mut self, _: &Captures<'_>, dst: &mut Vec<u8>)` — [`Captures`](#captures)

- <span id="noexpand-replacer-no-expansion"></span>`fn no_expansion(&mut self) -> Option<Cow<'_, [u8]>>`

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2443-2490`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2443-L2490)*

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

*Defined in [`regex-1.12.2/src/regex/bytes.rs:2631-2637`](../../../../.source_1765633015/regex-1.12.2/src/regex/bytes.rs#L2631-L2637)*

Quickly checks the given replacement string for whether interpolation
should be done on it. It returns `None` if a `$` was found anywhere in the
given string, which suggests interpolation needs to be done. But if there's
no `$` anywhere, then interpolation definitely does not need to be done. In
that case, the given string is returned as a borrowed `Cow`.

This is meant to be used to implement the `Replacer::no_expansion` method
in its various trait impls.

