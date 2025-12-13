# Crate `regex`

This crate provides routines for searching strings for matches of a [regular
expression] (aka "regex"). The regex syntax supported by this crate is similar
to other regex engines, but it lacks several features that are not known how to
implement efficiently. This includes, but is not limited to, look-around and
backreferences. In exchange, all regex searches in this crate have worst case
`O(m * n)` time complexity, where `m` is proportional to the size of the regex
and `n` is proportional to the size of the string being searched.

If you just want API documentation, then skip to the [`Regex`](#regex) type. Otherwise,
here's a quick example showing one way of parsing the output of a grep-like
program:

```rust
use regex::Regex;

let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
let hay = "\
path/to/foo:54:Blue Harvest
path/to/bar:90:Something, Something, Something, Dark Side
path/to/baz:3:It's a Trap!
";

let mut results = vec![];
for (_, [path, lineno, line]) in re.captures_iter(hay).map(|c| c.extract()) {
    results.push((path, lineno.parse::<u64>()?, line));
}
assert_eq!(results, vec![
    ("path/to/foo", 54, "Blue Harvest"),
    ("path/to/bar", 90, "Something, Something, Something, Dark Side"),
    ("path/to/baz", 3, "It's a Trap!"),
]);
Ok::<(), Box<dyn std::error::Error>>(())
```

# Overview

The primary type in this crate is a [`Regex`](#regex). Its most important methods are
as follows:

* `Regex::new` compiles a regex using the default configuration. A
[`RegexBuilder`](#regexbuilder) permits setting a non-default configuration. (For example,
case insensitive matching, verbose mode and others.)
* `Regex::is_match` reports whether a match exists in a particular haystack.
* `Regex::find` reports the byte offsets of a match in a haystack, if one
exists. `Regex::find_iter` returns an iterator over all such matches.
* `Regex::captures` returns a [`Captures`](#captures), which reports both the byte
offsets of a match in a haystack and the byte offsets of each matching capture
group from the regex in the haystack.
`Regex::captures_iter` returns an iterator over all such matches.

There is also a [`RegexSet`](#regexset), which permits searching for multiple regex
patterns simultaneously in a single search. However, it currently only reports
which patterns match and *not* the byte offsets of a match.

Otherwise, this top-level crate documentation is organized as follows:

* [Usage](#usage) shows how to add the `regex` crate to your Rust project.
* [Examples](#examples) provides a limited selection of regex search examples.
* [Performance](#performance) provides a brief summary of how to optimize regex
searching speed.
* [Unicode](#unicode) discusses support for non-ASCII patterns.
* [Syntax](#syntax) enumerates the specific regex syntax supported by this
crate.
* [Untrusted input](#untrusted-input) discusses how this crate deals with regex
patterns or haystacks that are untrusted.
* [Crate features](#crate-features) documents the Cargo features that can be
enabled or disabled for this crate.
* [Other crates](#other-crates) links to other crates in the `regex` family.

# Usage

The `regex` crate is [on crates.io](https://crates.io/crates/regex) and can be
used by adding `regex` to your dependencies in your project's `Cargo.toml`.
Or more simply, just run `cargo add regex`.

Here is a complete example that creates a new Rust project, adds a dependency
on `regex`, creates the source code for a regex search and then runs the
program.

First, create the project in a new directory:

```text
$ mkdir regex-example
$ cd regex-example
$ cargo init
```

Second, add a dependency on `regex`:

```text
$ cargo add regex
```

Third, edit `src/main.rs`. Delete what's there and replace it with this:

```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"Hello (?<name>\w+)!").unwrap();
    let Some(caps) = re.captures("Hello Murphy!") else {
        println!("no match!");
        return;
    };
    println!("The name is: {}", &caps["name"]);
}
```

Fourth, run it with `cargo run`:

```text
$ cargo run
   Compiling memchr v2.5.0
   Compiling regex-syntax v0.7.1
   Compiling aho-corasick v1.0.1
   Compiling regex v1.8.1
   Compiling regex-example v0.1.0 (/tmp/regex-example)
    Finished dev [unoptimized + debuginfo] target(s) in 4.22s
     Running `target/debug/regex-example`
The name is: Murphy
```

The first time you run the program will show more output like above. But
subsequent runs shouldn't have to re-compile the dependencies.

# Examples

This section provides a few examples, in tutorial style, showing how to
search a haystack with a regex. There are more examples throughout the API
documentation.

Before starting though, it's worth defining a few terms:

* A **regex** is a Rust value whose type is `Regex`. We use `re` as a
variable name for a regex.
* A **pattern** is the string that is used to build a regex. We use `pat` as
a variable name for a pattern.
* A **haystack** is the string that is searched by a regex. We use `hay` as a
variable name for a haystack.

Sometimes the words "regex" and "pattern" are used interchangeably.

General use of regular expressions in this crate proceeds by compiling a
**pattern** into a **regex**, and then using that regex to search, split or
replace parts of a **haystack**.

### Example: find a middle initial

We'll start off with a very simple example: a regex that looks for a specific
name but uses a wildcard to match a middle initial. Our pattern serves as
something like a template that will match a particular name with *any* middle
initial.

```rust
use regex::Regex;

// We use 'unwrap()' here because it would be a bug in our program if the
// pattern failed to compile to a regex. Panicking in the presence of a bug
// is okay.
let re = Regex::new(r"Homer (.)\. Simpson").unwrap();
let hay = "Homer J. Simpson";
let Some(caps) = re.captures(hay) else { return };
assert_eq!("J", &caps[1]);
```

There are a few things worth noticing here in our first example:

* The `.` is a special pattern meta character that means "match any single
character except for new lines." (More precisely, in this crate, it means
"match any UTF-8 encoding of any Unicode scalar value other than `\n`.")
* We can match an actual `.` literally by escaping it, i.e., `\.`.
* We use Rust's [raw strings] to avoid needing to deal with escape sequences in
both the regex pattern syntax and in Rust's string literal syntax. If we didn't
use raw strings here, we would have had to use `\\.` to match a literal `.`
character. That is, `r"\."` and `"\\."` are equivalent patterns.
* We put our wildcard `.` instruction in parentheses. These parentheses have a
special meaning that says, "make whatever part of the haystack matches within
these parentheses available as a capturing group." After finding a match, we
access this capture group with `&caps[1]`.

Otherwise, we execute a search using `re.captures(hay)` and return from our
function if no match occurred. We then reference the middle initial by asking
for the part of the haystack that matched the capture group indexed at `1`.
(The capture group at index 0 is implicit and always corresponds to the entire
match. In this case, that's `Homer J. Simpson`.)

### Example: named capture groups

Continuing from our middle initial example above, we can tweak the pattern
slightly to give a name to the group that matches the middle initial:

```rust
use regex::Regex;

// Note that (?P<middle>.) is a different way to spell the same thing.
let re = Regex::new(r"Homer (?<middle>.)\. Simpson").unwrap();
let hay = "Homer J. Simpson";
let Some(caps) = re.captures(hay) else { return };
assert_eq!("J", &caps["middle"]);
```

Giving a name to a group can be useful when there are multiple groups in
a pattern. It makes the code referring to those groups a bit easier to
understand.

### Example: validating a particular date format

This examples shows how to confirm whether a haystack, in its entirety, matches
a particular date format:

```rust
use regex::Regex;

let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
assert!(re.is_match("2010-03-14"));
```

Notice the use of the `^` and `$` anchors. In this crate, every regex search is
run with an implicit `(?s:.)*?` at the beginning of its pattern, which allows
the regex to match anywhere in a haystack. Anchors, as above, can be used to
ensure that the full haystack matches a pattern.

This crate is also Unicode aware by default, which means that `\d` might match
more than you might expect it to. For example:

```rust
use regex::Regex;

let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
assert!(re.is_match("𝟚𝟘𝟙𝟘-𝟘𝟛-𝟙𝟜"));
```

To only match an ASCII decimal digit, all of the following are equivalent:

* `[0-9]`
* `(?-u:\d)`
* `[[:digit:]]`
* `[\d&&\p{ascii}]`

### Example: finding dates in a haystack

In the previous example, we showed how one might validate that a haystack,
in its entirety, corresponded to a particular date format. But what if we wanted
to extract all things that look like dates in a specific format from a haystack?
To do this, we can use an iterator API to find all matches (notice that we've
removed the anchors and switched to looking for ASCII-only digits):

```rust
use regex::Regex;

let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
// 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
let dates: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();
assert_eq!(dates, vec![
    "1865-04-14",
    "1881-07-02",
    "1901-09-06",
    "1963-11-22",
]);
```

We can also iterate over [`Captures`](#captures) values instead of [`Match`](#match) values, and
that in turn permits accessing each component of the date via capturing groups:

```rust
use regex::Regex;

let re = Regex::new(r"(?<y>[0-9]{4})-(?<m>[0-9]{2})-(?<d>[0-9]{2})").unwrap();
let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
// 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
let dates: Vec<(&str, &str, &str)> = re.captures_iter(hay).map(|caps| {
    // The unwraps are okay because every capture group must match if the whole
    // regex matches, and in this context, we know we have a match.
    //
    // Note that we use `caps.name("y").unwrap().as_str()` instead of
    // `&caps["y"]` because the lifetime of the former is the same as the
    // lifetime of `hay` above, but the lifetime of the latter is tied to the
    // lifetime of `caps` due to how the `Index` trait is defined.
    let year = caps.name("y").unwrap().as_str();
    let month = caps.name("m").unwrap().as_str();
    let day = caps.name("d").unwrap().as_str();
    (year, month, day)
}).collect();
assert_eq!(dates, vec![
    ("1865", "04", "14"),
    ("1881", "07", "02"),
    ("1901", "09", "06"),
    ("1963", "11", "22"),
]);
```

### Example: simpler capture group extraction

One can use `Captures::extract` to make the code from the previous example a
bit simpler in this case:

```rust
use regex::Regex;

let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
let dates: Vec<(&str, &str, &str)> = re.captures_iter(hay).map(|caps| {
    let (_, [year, month, day]) = caps.extract();
    (year, month, day)
}).collect();
assert_eq!(dates, vec![
    ("1865", "04", "14"),
    ("1881", "07", "02"),
    ("1901", "09", "06"),
    ("1963", "11", "22"),
]);
```

`Captures::extract` works by ensuring that the number of matching groups match
the number of groups requested via the `[year, month, day]` syntax. If they do,
then the substrings for each corresponding capture group are automatically
returned in an appropriately sized array. Rust's syntax for pattern matching
arrays does the rest.

### Example: replacement with named capture groups

Building on the previous example, perhaps we'd like to rearrange the date
formats. This can be done by finding each match and replacing it with
something different. The `Regex::replace_all` routine provides a convenient
way to do this, including by supporting references to named groups in the
replacement string:

```rust
use regex::Regex;

let re = Regex::new(r"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})").unwrap();
let before = "1973-01-05, 1975-08-25 and 1980-10-18";
let after = re.replace_all(before, "$m/$d/$y");
assert_eq!(after, "01/05/1973, 08/25/1975 and 10/18/1980");
```

The replace methods are actually polymorphic in the replacement, which
provides more flexibility than is seen here. (See the documentation for
`Regex::replace` for more details.)

### Example: verbose mode

When your regex gets complicated, you might consider using something other
than regex. But if you stick with regex, you can use the `x` flag to enable
insignificant whitespace mode or "verbose mode." In this mode, whitespace
is treated as insignificant and one may write comments. This may make your
patterns easier to comprehend.

```rust
use regex::Regex;

let re = Regex::new(r"(?x)
  (?P<y>\d{4}) # the year, including all Unicode digits
  -
  (?P<m>\d{2}) # the month, including all Unicode digits
  -
  (?P<d>\d{2}) # the day, including all Unicode digits
").unwrap();

let before = "1973-01-05, 1975-08-25 and 1980-10-18";
let after = re.replace_all(before, "$m/$d/$y");
assert_eq!(after, "01/05/1973, 08/25/1975 and 10/18/1980");
```

If you wish to match against whitespace in this mode, you can still use `\s`,
`\n`, `\t`, etc. For escaping a single space character, you can escape it
directly with `\ `, use its hex character code `\x20` or temporarily disable
the `x` flag, e.g., `(?-x: )`.

### Example: match multiple regular expressions simultaneously

This demonstrates how to use a [`RegexSet`](#regexset) to match multiple (possibly
overlapping) regexes in a single scan of a haystack:

```rust
use regex::RegexSet;

let set = RegexSet::new(&[
    r"\w+",
    r"\d+",
    r"\pL+",
    r"foo",
    r"bar",
    r"barfoo",
    r"foobar",
]).unwrap();

// Iterate over and collect all of the matches. Each match corresponds to the
// ID of the matching pattern.
let matches: Vec<_> = set.matches("foobar").into_iter().collect();
assert_eq!(matches, vec![0, 2, 3, 4, 6]);

// You can also test whether a particular regex matched:
let matches = set.matches("foobar");
assert!(!matches.matched(5));
assert!(matches.matched(6));
```

# Performance

This section briefly discusses a few concerns regarding the speed and resource
usage of regexes.

### Only ask for what you need

When running a search with a regex, there are generally three different types
of information one can ask for:

1. Does a regex match in a haystack?
2. Where does a regex match in a haystack?
3. Where do each of the capturing groups match in a haystack?

Generally speaking, this crate could provide a function to answer only #3,
which would subsume #1 and #2 automatically. However, it can be significantly
more expensive to compute the location of capturing group matches, so it's best
not to do it if you don't need to.

Therefore, only ask for what you need. For example, don't use `Regex::find`
if you only need to test if a regex matches a haystack. Use `Regex::is_match`
instead.

### Unicode can impact memory usage and search speed

This crate has first class support for Unicode and it is **enabled by default**.
In many cases, the extra memory required to support it will be negligible and
it typically won't impact search speed. But it can in some cases.

With respect to memory usage, the impact of Unicode principally manifests
through the use of Unicode character classes. Unicode character classes
tend to be quite large. For example, `\w` by default matches around 140,000
distinct codepoints. This requires additional memory, and tends to slow down
regex compilation. While a `\w` here and there is unlikely to be noticed,
writing `\w{100}` will for example result in quite a large regex by default.
Indeed, `\w` is considerably larger than its ASCII-only version, so if your
requirements are satisfied by ASCII, it's probably a good idea to stick to
ASCII classes. The ASCII-only version of `\w` can be spelled in a number of
ways. All of the following are equivalent:

* `[0-9A-Za-z_]`
* `(?-u:\w)`
* `[[:word:]]`
* `[\w&&\p{ascii}]`

With respect to search speed, Unicode tends to be handled pretty well, even when
using large Unicode character classes. However, some of the faster internal
regex engines cannot handle a Unicode aware word boundary assertion. So if you
don't need Unicode-aware word boundary assertions, you might consider using
`(?-u:\b)` instead of `\b`, where the former uses an ASCII-only definition of
a word character.

### Literals might accelerate searches

This crate tends to be quite good at recognizing literals in a regex pattern
and using them to accelerate a search. If it is at all possible to include
some kind of literal in your pattern, then it might make search substantially
faster. For example, in the regex `\w+@\w+`, the engine will look for
occurrences of `@` and then try a reverse match for `\w+` to find the start
position.

### Avoid re-compiling regexes, especially in a loop

It is an anti-pattern to compile the same pattern in a loop since regex
compilation is typically expensive. (It takes anywhere from a few microseconds
to a few **milliseconds** depending on the size of the pattern.) Not only is
compilation itself expensive, but this also prevents optimizations that reuse
allocations internally to the regex engine.

In Rust, it can sometimes be a pain to pass regular expressions around if
they're used from inside a helper function. Instead, we recommend using
`std::sync::LazyLock`, or the `once_cell` crate,
if you can't use the standard library.

This example shows how to use `std::sync::LazyLock`:

```rust
use std::sync::LazyLock;

use regex::Regex;

fn some_helper_function(haystack: &str) -> bool {
    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"...").unwrap());
    RE.is_match(haystack)
}

fn main() {
    assert!(some_helper_function("abc"));
    assert!(!some_helper_function("ac"));
}
```

Specifically, in this example, the regex will be compiled when it is used for
the first time. On subsequent uses, it will reuse the previously built `Regex`.
Notice how one can define the `Regex` locally to a specific function.


### Sharing a regex across threads can result in contention

While a single `Regex` can be freely used from multiple threads simultaneously,
there is a small synchronization cost that must be paid. Generally speaking,
one shouldn't expect to observe this unless the principal task in each thread
is searching with the regex *and* most searches are on short haystacks. In this
case, internal contention on shared resources can spike and increase latency,
which in turn may slow down each individual search.

One can work around this by cloning each `Regex` before sending it to another
thread. The cloned regexes will still share the same internal read-only portion
of its compiled state (it's reference counted), but each thread will get
optimized access to the mutable space that is used to run a search. In general,
there is no additional cost in memory to doing this. The only cost is the added
code complexity required to explicitly clone the regex. (If you share the same
`Regex` across multiple threads, each thread still gets its own mutable space,
but accessing that space is slower.)

# Unicode

This section discusses what kind of Unicode support this regex library has.
Before showing some examples, we'll summarize the relevant points:

* This crate almost fully implements "Basic Unicode Support" (Level 1) as
specified by the [Unicode Technical Standard #18][UTS18]. The full details
of what is supported are documented in [UNICODE.md] in the root of the regex
crate repository. There is virtually no support for "Extended Unicode Support"
(Level 2) from UTS#18.
* The top-level [`Regex`](#regex) runs searches *as if* iterating over each of the
codepoints in the haystack. That is, the fundamental atom of matching is a
single codepoint.
* [`bytes::Regex`](regex/bytes/index.md), in contrast, permits disabling Unicode mode for part of all
of your pattern in all cases. When Unicode mode is disabled, then a search is
run *as if* iterating over each byte in the haystack. That is, the fundamental
atom of matching is a single byte. (A top-level `Regex` also permits disabling
Unicode and thus matching *as if* it were one byte at a time, but only when
doing so wouldn't permit matching invalid UTF-8.)
* When Unicode mode is enabled (the default), `.` will match an entire Unicode
scalar value, even when it is encoded using multiple bytes. When Unicode mode
is disabled (e.g., `(?-u:.)`), then `.` will match a single byte in all cases.
* The character classes `\w`, `\d` and `\s` are all Unicode-aware by default.
Use `(?-u:\w)`, `(?-u:\d)` and `(?-u:\s)` to get their ASCII-only definitions.
* Similarly, `\b` and `\B` use a Unicode definition of a "word" character.
To get ASCII-only word boundaries, use `(?-u:\b)` and `(?-u:\B)`. This also
applies to the special word boundary assertions. (That is, `\b{start}`,
`\b{end}`, `\b{start-half}`, `\b{end-half}`.)
* `^` and `$` are **not** Unicode-aware in multi-line mode. Namely, they only
recognize `\n` (assuming CRLF mode is not enabled) and not any of the other
forms of line terminators defined by Unicode.
* Case insensitive searching is Unicode-aware and uses simple case folding.
* Unicode general categories, scripts and many boolean properties are available
by default via the `\p{property name}` syntax.
* In all cases, matches are reported using byte offsets. Or more precisely,
UTF-8 code unit offsets. This permits constant time indexing and slicing of the
haystack.


Patterns themselves are **only** interpreted as a sequence of Unicode scalar
values. This means you can use Unicode characters directly in your pattern:

```rust
use regex::Regex;

let re = Regex::new(r"(?i)Δ+").unwrap();
let m = re.find("ΔδΔ").unwrap();
assert_eq!((0, 6), (m.start(), m.end()));
// alternatively:
assert_eq!(0..6, m.range());
```

As noted above, Unicode general categories, scripts, script extensions, ages
and a smattering of boolean properties are available as character classes. For
example, you can match a sequence of numerals, Greek or Cherokee letters:

```rust
use regex::Regex;

let re = Regex::new(r"[\pN\p{Greek}\p{Cherokee}]+").unwrap();
let m = re.find("abcΔᎠβⅠᏴγδⅡxyz").unwrap();
assert_eq!(3..23, m.range());
```

While not specific to Unicode, this library also supports character class set
operations. Namely, one can nest character classes arbitrarily and perform set
operations on them. Those set operations are union (the default), intersection,
difference and symmetric difference. These set operations tend to be most
useful with Unicode character classes. For example, to match any codepoint
that is both in the `Greek` script and in the `Letter` general category:

```rust
use regex::Regex;

let re = Regex::new(r"[\p{Greek}&&\pL]+").unwrap();
let subs: Vec<&str> = re.find_iter("ΔδΔ𐅌ΔδΔ").map(|m| m.as_str()).collect();
assert_eq!(subs, vec!["ΔδΔ", "ΔδΔ"]);

// If we just matches on Greek, then all codepoints would match!
let re = Regex::new(r"\p{Greek}+").unwrap();
let subs: Vec<&str> = re.find_iter("ΔδΔ𐅌ΔδΔ").map(|m| m.as_str()).collect();
assert_eq!(subs, vec!["ΔδΔ𐅌ΔδΔ"]);
```

### Opt out of Unicode support

The [`bytes::Regex`](regex/bytes/index.md) type that can be used to search `&[u8]` haystacks. By
default, haystacks are conventionally treated as UTF-8 just like it is with the
main `Regex` type. However, this behavior can be disabled by turning off the
`u` flag, even if doing so could result in matching invalid UTF-8. For example,
when the `u` flag is disabled, `.` will match any byte instead of any Unicode
scalar value.

Disabling the `u` flag is also possible with the standard `&str`-based `Regex`
type, but it is only allowed where the UTF-8 invariant is maintained. For
example, `(?-u:\w)` is an ASCII-only `\w` character class and is legal in an
`&str`-based `Regex`, but `(?-u:\W)` will attempt to match *any byte* that
isn't in `(?-u:\w)`, which in turn includes bytes that are invalid UTF-8.
Similarly, `(?-u:\xFF)` will attempt to match the raw byte `\xFF` (instead of
`U+00FF`), which is invalid UTF-8 and therefore is illegal in `&str`-based
regexes.

Finally, since Unicode support requires bundling large Unicode data
tables, this crate exposes knobs to disable the compilation of those
data tables, which can be useful for shrinking binary size and reducing
compilation times. For details on how to do that, see the section on [crate
features](#crate-features).

# Syntax

The syntax supported in this crate is documented below.

Note that the regular expression parser and abstract syntax are exposed in
a separate crate, [`regex-syntax`](https://docs.rs/regex-syntax).

### Matching one character

<pre class="rust">
.             any character except new line (includes new line with s flag)
[0-9]         any ASCII digit
\d            digit (\p{Nd})
\D            not digit
\pX           Unicode character class identified by a one-letter name
\p{Greek}     Unicode character class (general category or script)
\PX           Negated Unicode character class identified by a one-letter name
\P{Greek}     negated Unicode character class (general category or script)
</pre>

### Character classes

<pre class="rust">
[xyz]         A character class matching either x, y or z (union).
[^xyz]        A character class matching any character except x, y and z.
[a-z]         A character class matching any character in range a-z.
[[:alpha:]]   ASCII character class ([A-Za-z])
[[:^alpha:]]  Negated ASCII character class ([^A-Za-z])
[x[^xyz]]     Nested/grouping character class (matching any character except y and z)
[a-y&&xyz]    Intersection (matching x or y)
[0-9&&[^4]]   Subtraction using intersection and negation (matching 0-9 except 4)
[0-9--4]      Direct subtraction (matching 0-9 except 4)
[a-g~~b-h]    Symmetric difference (matching `a` and `h` only)
[\[\]]        Escaping in character classes (matching [ or ])
[a&&b]        An empty character class matching nothing
</pre>

Any named character class may appear inside a bracketed `[...]` character
class. For example, `[\p{Greek}[:digit:]]` matches any ASCII digit or any
codepoint in the `Greek` script. `[\p{Greek}&&\pL]` matches Greek letters.

Precedence in character classes, from most binding to least:

1. Ranges: `[a-cd]` == `[[a-c]d]`
2. Union: `[ab&&bc]` == `[[ab]&&[bc]]`
3. Intersection, difference, symmetric difference. All three have equivalent
precedence, and are evaluated in left-to-right order. For example,
`[\pL--\p{Greek}&&\p{Uppercase}]` == `[[\pL--\p{Greek}]&&\p{Uppercase}]`.
4. Negation: `[^a-z&&b]` == `[^[a-z&&b]]`.

### Composites

<pre class="rust">
xy    concatenation (x followed by y)
x|y   alternation (x or y, prefer x)
</pre>

This example shows how an alternation works, and what it means to prefer a
branch in the alternation over subsequent branches.

```rust
use regex::Regex;

let haystack = "samwise";
// If 'samwise' comes first in our alternation, then it is
// preferred as a match, even if the regex engine could
// technically detect that 'sam' led to a match earlier.
let re = Regex::new(r"samwise|sam").unwrap();
assert_eq!("samwise", re.find(haystack).unwrap().as_str());
// But if 'sam' comes first, then it will match instead.
// In this case, it is impossible for 'samwise' to match
// because 'sam' is a prefix of it.
let re = Regex::new(r"sam|samwise").unwrap();
assert_eq!("sam", re.find(haystack).unwrap().as_str());
```

### Repetitions

<pre class="rust">
x*        zero or more of x (greedy)
x+        one or more of x (greedy)
x?        zero or one of x (greedy)
x*?       zero or more of x (ungreedy/lazy)
x+?       one or more of x (ungreedy/lazy)
x??       zero or one of x (ungreedy/lazy)
x{n,m}    at least n x and at most m x (greedy)
x{n,}     at least n x (greedy)
x{n}      exactly n x
x{n,m}?   at least n x and at most m x (ungreedy/lazy)
x{n,}?    at least n x (ungreedy/lazy)
x{n}?     exactly n x
</pre>

### Empty matches

<pre class="rust">
^               the beginning of a haystack (or start-of-line with multi-line mode)
$               the end of a haystack (or end-of-line with multi-line mode)
\A              only the beginning of a haystack (even with multi-line mode enabled)
\z              only the end of a haystack (even with multi-line mode enabled)
\b              a Unicode word boundary (\w on one side and \W, \A, or \z on other)
\B              not a Unicode word boundary
\b{start}, \<   a Unicode start-of-word boundary (\W|\A on the left, \w on the right)
\b{end}, \>     a Unicode end-of-word boundary (\w on the left, \W|\z on the right))
\b{start-half}  half of a Unicode start-of-word boundary (\W|\A on the left)
\b{end-half}    half of a Unicode end-of-word boundary (\W|\z on the right)
</pre>

The empty regex is valid and matches the empty string. For example, the
empty regex matches `abc` at positions `0`, `1`, `2` and `3`. When using the
top-level [`Regex`](#regex) on `&str` haystacks, an empty match that splits a codepoint
is guaranteed to never be returned. However, such matches are permitted when
using a [`bytes::Regex`](regex/bytes/index.md). For example:

```rust
let re = regex::Regex::new(r"").unwrap();
let ranges: Vec<_> = re.find_iter("💩").map(|m| m.range()).collect();
assert_eq!(ranges, vec![0..0, 4..4]);

let re = regex::bytes::Regex::new(r"").unwrap();
let ranges: Vec<_> = re.find_iter("💩".as_bytes()).map(|m| m.range()).collect();
assert_eq!(ranges, vec![0..0, 1..1, 2..2, 3..3, 4..4]);
```

Note that an empty regex is distinct from a regex that can never match.
For example, the regex `[a&&b]` is a character class that represents the
intersection of `a` and `b`. That intersection is empty, which means the
character class is empty. Since nothing is in the empty set, `[a&&b]` matches
nothing, not even the empty string.

### Grouping and flags

<pre class="rust">
(exp)          numbered capture group (indexed by opening parenthesis)
(?P&lt;name&gt;exp)  named (also numbered) capture group (names must be alpha-numeric)
(?&lt;name&gt;exp)   named (also numbered) capture group (names must be alpha-numeric)
(?:exp)        non-capturing group
(?flags)       set flags within current group
(?flags:exp)   set flags for exp (non-capturing)
</pre>

Capture group names must be any sequence of alpha-numeric Unicode codepoints,
in addition to `.`, `_`, `` and ``. Names must start with either an `_` or
an alphabetic codepoint. Alphabetic codepoints correspond to the `Alphabetic`
Unicode property, while numeric codepoints correspond to the union of the
`Decimal_Number`, `Letter_Number` and `Other_Number` general categories.

Flags are each a single character. For example, `(?x)` sets the flag `x`
and `(?-x)` clears the flag `x`. Multiple flags can be set or cleared at
the same time: `(?xy)` sets both the `x` and `y` flags and `(?x-y)` sets
the `x` flag and clears the `y` flag.

All flags are by default disabled unless stated otherwise. They are:

<pre class="rust">
i     case-insensitive: letters match both upper and lower case
m     multi-line mode: ^ and $ match begin/end of line
s     allow . to match \n
R     enables CRLF mode: when multi-line mode is enabled, \r\n is used
U     swap the meaning of x* and x*?
u     Unicode support (enabled by default)
x     verbose mode, ignores whitespace and allow line comments (starting with `#`)
</pre>

Note that in verbose mode, whitespace is ignored everywhere, including within
character classes. To insert whitespace, use its escaped form or a hex literal.
For example, `\ ` or `\x20` for an ASCII space.

Flags can be toggled within a pattern. Here's an example that matches
case-insensitively for the first part but case-sensitively for the second part:

```rust
use regex::Regex;

let re = Regex::new(r"(?i)a+(?-i)b+").unwrap();
let m = re.find("AaAaAbbBBBb").unwrap();
assert_eq!(m.as_str(), "AaAaAbb");
```

Notice that the `a+` matches either `a` or `A`, but the `b+` only matches
`b`.

Multi-line mode means `^` and `$` no longer match just at the beginning/end of
the input, but also at the beginning/end of lines:

```rust
use regex::Regex;

let re = Regex::new(r"(?m)^line \d+").unwrap();
let m = re.find("line one\nline 2\n").unwrap();
assert_eq!(m.as_str(), "line 2");
```

Note that `^` matches after new lines, even at the end of input:

```rust
use regex::Regex;

let re = Regex::new(r"(?m)^").unwrap();
let m = re.find_iter("test\n").last().unwrap();
assert_eq!((m.start(), m.end()), (5, 5));
```

When both CRLF mode and multi-line mode are enabled, then `^` and `$` will
match either `\r` or `\n`, but never in the middle of a `\r\n`:

```rust
use regex::Regex;

let re = Regex::new(r"(?mR)^foo$").unwrap();
let m = re.find("\r\nfoo\r\n").unwrap();
assert_eq!(m.as_str(), "foo");
```

Unicode mode can also be selectively disabled, although only when the result
*would not* match invalid UTF-8. One good example of this is using an ASCII
word boundary instead of a Unicode word boundary, which might make some regex
searches run faster:

```rust
use regex::Regex;

let re = Regex::new(r"(?-u:\b).+(?-u:\b)").unwrap();
let m = re.find("$$abc$$").unwrap();
assert_eq!(m.as_str(), "abc");
```

### Escape sequences

Note that this includes all possible escape sequences, even ones that are
documented elsewhere.

<pre class="rust">
\*              literal *, applies to all ASCII except [0-9A-Za-z<>]
\a              bell (\x07)
\f              form feed (\x0C)
\t              horizontal tab
\n              new line
\r              carriage return
\v              vertical tab (\x0B)
\A              matches at the beginning of a haystack
\z              matches at the end of a haystack
\b              word boundary assertion
\B              negated word boundary assertion
\b{start}, \<   start-of-word boundary assertion
\b{end}, \>     end-of-word boundary assertion
\b{start-half}  half of a start-of-word boundary assertion
\b{end-half}    half of a end-of-word boundary assertion
\123            octal character code, up to three digits (when enabled)
\x7F            hex character code (exactly two digits)
\x{10FFFF}      any hex character code corresponding to a Unicode code point
\u007F          hex character code (exactly four digits)
\u{7F}          any hex character code corresponding to a Unicode code point
\U0000007F      hex character code (exactly eight digits)
\U{7F}          any hex character code corresponding to a Unicode code point
\p{Letter}      Unicode character class
\P{Letter}      negated Unicode character class
\d, \s, \w      Perl character class
\D, \S, \W      negated Perl character class
</pre>

### Perl character classes (Unicode friendly)

These classes are based on the definitions provided in
[UTS#18](https://www.unicode.org/reports/tr18/#Compatibility_Properties):

<pre class="rust">
\d     digit (\p{Nd})
\D     not digit
\s     whitespace (\p{White_Space})
\S     not whitespace
\w     word character (\p{Alphabetic} + \p{M} + \d + \p{Pc} + \p{Join_Control})
\W     not word character
</pre>

### ASCII character classes

These classes are based on the definitions provided in
[UTS#18](https://www.unicode.org/reports/tr18/#Compatibility_Properties):

<pre class="rust">
[[:alnum:]]    alphanumeric ([0-9A-Za-z])
[[:alpha:]]    alphabetic ([A-Za-z])
[[:ascii:]]    ASCII ([\x00-\x7F])
[[:blank:]]    blank ([\t ])
[[:cntrl:]]    control ([\x00-\x1F\x7F])
[[:digit:]]    digits ([0-9])
[[:graph:]]    graphical ([!-~])
[[:lower:]]    lower case ([a-z])
[[:print:]]    printable ([ -~])
[[:punct:]]    punctuation ([!-/:-@\[-`{-~])
[[:space:]]    whitespace ([\t\n\v\f\r ])
[[:upper:]]    upper case ([A-Z])
[[:word:]]     word characters ([0-9A-Za-z_])
[[:xdigit:]]   hex digit ([0-9A-Fa-f])
</pre>

# Untrusted input

This crate is meant to be able to run regex searches on untrusted haystacks
without fear of [ReDoS]. This crate also, to a certain extent, supports
untrusted patterns.

This crate differs from most (but not all) other regex engines in that it
doesn't use unbounded backtracking to run a regex search. In those cases,
one generally cannot use untrusted patterns *or* untrusted haystacks because
it can be very difficult to know whether a particular pattern will result in
catastrophic backtracking or not.

We'll first discuss how this crate deals with untrusted inputs and then wrap
it up with a realistic discussion about what practice really looks like.

### Panics

Outside of clearly documented cases, most APIs in this crate are intended to
never panic regardless of the inputs given to them. For example, `Regex::new`,
`Regex::is_match`, `Regex::find` and `Regex::captures` should never panic. That
is, it is an API promise that those APIs will never panic no matter what inputs
are given to them. With that said, regex engines are complicated beasts, and
providing a rock solid guarantee that these APIs literally never panic is
essentially equivalent to saying, "there are no bugs in this library." That is
a bold claim, and not really one that can be feasibly made with a straight
face.

Don't get the wrong impression here. This crate is extensively tested, not just
with unit and integration tests, but also via fuzz testing. For example, this
crate is part of the [OSS-fuzz project]. Panics should be incredibly rare, but
it is possible for bugs to exist, and thus possible for a panic to occur. If
you need a rock solid guarantee against panics, then you should wrap calls into
this library with `std::panic::catch_unwind`.

It's also worth pointing out that this library will *generally* panic when
other regex engines would commit undefined behavior. When undefined behavior
occurs, your program might continue as if nothing bad has happened, but it also
might mean your program is open to the worst kinds of exploits. In contrast,
the worst thing a panic can do is a denial of service.


### Untrusted patterns

The principal way this crate deals with them is by limiting their size by
default. The size limit can be configured via `RegexBuilder::size_limit`. The
idea of a size limit is that compiling a pattern into a `Regex` will fail if it
becomes "too big." Namely, while *most* resources consumed by compiling a regex
are approximately proportional (albeit with some high constant factors in some
cases, such as with Unicode character classes) to the length of the pattern
itself, there is one particular exception to this: counted repetitions. Namely,
this pattern:

```text
a{5}{5}{5}{5}{5}{5}
```

Is equivalent to this pattern:

```text
a{15625}
```

In both of these cases, the actual pattern string is quite small, but the
resulting `Regex` value is quite large. Indeed, as the first pattern shows,
it isn't enough to locally limit the size of each repetition because they can
be stacked in a way that results in exponential growth.

To provide a bit more context, a simplified view of regex compilation looks
like this:

* The pattern string is parsed into a structured representation called an AST.
Counted repetitions are not expanded and Unicode character classes are not
looked up in this stage. That is, the size of the AST is proportional to the
size of the pattern with "reasonable" constant factors. In other words, one
can reasonably limit the memory used by an AST by limiting the length of the
pattern string.
* The AST is translated into an HIR. Counted repetitions are still *not*
expanded at this stage, but Unicode character classes are embedded into the
HIR. The memory usage of a HIR is still proportional to the length of the
original pattern string, but the constant factors---mostly as a result of
Unicode character classes---can be quite high. Still though, the memory used by
an HIR can be reasonably limited by limiting the length of the pattern string.
* The HIR is compiled into a [Thompson NFA]. This is the stage at which
something like `\w{5}` is rewritten to `\w\w\w\w\w`. Thus, this is the stage
at which `RegexBuilder::size_limit` is enforced. If the NFA exceeds the
configured size, then this stage will fail.

The size limit helps avoid two different kinds of exorbitant resource usage:

* It avoids permitting exponential memory usage based on the size of the
pattern string.
* It avoids long search times. This will be discussed in more detail in the
next section, but worst case search time *is* dependent on the size of the
regex. So keeping regexes limited to a reasonable size is also a way of keeping
search times reasonable.

Finally, it's worth pointing out that regex compilation is guaranteed to take
worst case `O(m)` time, where `m` is proportional to the size of regex. The
size of the regex here is *after* the counted repetitions have been expanded.

**Advice for those using untrusted regexes**: limit the pattern length to
something small and expand it as needed. Configure `RegexBuilder::size_limit`
to something small and then expand it as needed.

### Untrusted haystacks

The main way this crate guards against searches from taking a long time is by
using algorithms that guarantee a `O(m * n)` worst case time and space bound.
Namely:

* `m` is proportional to the size of the regex, where the size of the regex
includes the expansion of all counted repetitions. (See the previous section on
untrusted patterns.)
* `n` is proportional to the length, in bytes, of the haystack.

In other words, if you consider `m` to be a constant (for example, the regex
pattern is a literal in the source code), then the search can be said to run
in "linear time." Or equivalently, "linear time with respect to the size of the
haystack."

But the `m` factor here is important not to ignore. If a regex is
particularly big, the search times can get quite slow. This is why, in part,
`RegexBuilder::size_limit` exists.

**Advice for those searching untrusted haystacks**: As long as your regexes
are not enormous, you should expect to be able to search untrusted haystacks
without fear. If you aren't sure, you should benchmark it. Unlike backtracking
engines, if your regex is so big that it's likely to result in slow searches,
this is probably something you'll be able to observe regardless of what the
haystack is made up of.

### Iterating over matches

One thing that is perhaps easy to miss is that the worst case time
complexity bound of `O(m * n)` applies to methods like `Regex::is_match`,
`Regex::find` and `Regex::captures`. It does **not** apply to
`Regex::find_iter` or `Regex::captures_iter`. Namely, since iterating over
all matches can execute many searches, and each search can scan the entire
haystack, the worst case time complexity for iterators is `O(m * n^2)`.

One example of where this occurs is when a pattern consists of an alternation,
where an earlier branch of the alternation requires scanning the entire
haystack only to discover that there is no match. It also requires a later
branch of the alternation to have matched at the beginning of the search. For
example, consider the pattern `.*[^A-Z]|[A-Z]` and the haystack `AAAAA`. The
first search will scan to the end looking for matches of `.*[^A-Z]` even though
a finite automata engine (as in this crate) knows that `[A-Z]` has already
matched the first character of the haystack. This is due to the greedy nature
of regex searching. That first search will report a match at the first `A` only
after scanning to the end to discover that no other match exists. The next
search then begins at the second `A` and the behavior repeats.

There is no way to avoid this. This means that if both patterns and haystacks
are untrusted and you're iterating over all matches, you're susceptible to
worst case quadratic time complexity. One possible way to mitigate this
is to drop down to the lower level `regex-automata` crate and use its
`meta::Regex` iterator APIs. There, you can configure the search to operate
in "earliest" mode by passing a `Input::new(haystack).earliest(true)` to
`meta::Regex::find_iter` (for example). By enabling this mode, you give up
the normal greedy match semantics of regex searches and instead ask the regex
engine to immediately stop as soon as a match has been found. Enabling this
mode will thus restore the worst case `O(m * n)` time complexity bound, but at
the cost of different semantics.

### Untrusted inputs in practice

While providing a `O(m * n)` worst case time bound on all searches goes a long
way toward preventing [ReDoS], that doesn't mean every search you can possibly
run will complete without burning CPU time. In general, there are a few ways
for the `m * n` time bound to still bite you:

* You are searching an exceptionally long haystack. No matter how you slice
it, a longer haystack will take more time to search. This crate may often make
very quick work of even long haystacks because of its literal optimizations,
but those aren't available for all regexes.
* Unicode character classes can cause searches to be quite slow in some cases.
This is especially true when they are combined with counted repetitions. While
the regex size limit above will protect you from the most egregious cases,
the default size limit still permits pretty big regexes that can execute more
slowly than one might expect.
* While routines like `Regex::find` and `Regex::captures` guarantee
worst case `O(m * n)` search time, routines like `Regex::find_iter` and
`Regex::captures_iter` actually have worst case `O(m * n^2)` search time.
This is because `find_iter` runs many searches, and each search takes worst
case `O(m * n)` time. Thus, iteration of all matches in a haystack has
worst case `O(m * n^2)`. A good example of a pattern that exhibits this is
`(?:A+){1000}|` or even `.*[^A-Z]|[A-Z]`.

In general, untrusted haystacks are easier to stomach than untrusted patterns.
Untrusted patterns give a lot more control to the caller to impact the
performance of a search. In many cases, a regex search will actually execute in
average case `O(n)` time (i.e., not dependent on the size of the regex), but
this can't be guaranteed in general. Therefore, permitting untrusted patterns
means that your only line of defense is to put a limit on how big `m` (and
perhaps also `n`) can be in `O(m * n)`. `n` is limited by simply inspecting
the length of the haystack while `m` is limited by *both* applying a limit to
the length of the pattern *and* a limit on the compiled size of the regex via
`RegexBuilder::size_limit`.

It bears repeating: if you're accepting untrusted patterns, it would be a good
idea to start with conservative limits on `m` and `n`, and then carefully
increase them as needed.

# Crate features

By default, this crate tries pretty hard to make regex matching both as fast
as possible and as correct as it can be. This means that there is a lot of
code dedicated to performance, the handling of Unicode data and the Unicode
data itself. Overall, this leads to more dependencies, larger binaries and
longer compile times. This trade off may not be appropriate in all cases, and
indeed, even when all Unicode and performance features are disabled, one is
still left with a perfectly serviceable regex engine that will work well in
many cases. (Note that code is not arbitrarily reducible, and for this reason,
the [`regex-lite`](https://docs.rs/regex-lite) crate exists to provide an even
more minimal experience by cutting out Unicode and performance, but still
maintaining the linear search time bound.)

This crate exposes a number of features for controlling that trade off. Some
of these features are strictly performance oriented, such that disabling them
won't result in a loss of functionality, but may result in worse performance.
Other features, such as the ones controlling the presence or absence of Unicode
data, can result in a loss of functionality. For example, if one disables the
`unicode-case` feature (described below), then compiling the regex `(?i)a`
will fail since Unicode case insensitivity is enabled by default. Instead,
callers must use `(?i-u)a` to disable Unicode case folding. Stated differently,
enabling or disabling any of the features below can only add or subtract from
the total set of valid regular expressions. Enabling or disabling a feature
will never modify the match semantics of a regular expression.

Most features below are enabled by default. Features that aren't enabled by
default are noted.

### Ecosystem features

* **std** -
  When enabled, this will cause `regex` to use the standard library. In terms
  of APIs, `std` causes error types to implement the `std::error::Error`
  trait. Enabling `std` will also result in performance optimizations,
  including SIMD and faster synchronization primitives. Notably, **disabling
  the `std` feature will result in the use of spin locks**. To use a regex
  engine without `std` and without spin locks, you'll need to drop down to
  the [`regex-automata`](https://docs.rs/regex-automata) crate.
* **logging** -
  When enabled, the `log` crate is used to emit messages about regex
  compilation and search strategies. This is **disabled by default**. This is
  typically only useful to someone working on this crate's internals, but might
  be useful if you're doing some rabbit hole performance hacking. Or if you're
  just interested in the kinds of decisions being made by the regex engine.

### Performance features

**Note**:
  To get performance benefits offered by the SIMD, `std` must be enabled.
  None of the `perf-*` features will enable `std` implicitly.

* **perf** -
  Enables all performance related features except for `perf-dfa-full`. This
  feature is enabled by default is intended to cover all reasonable features
  that improve performance, even if more are added in the future.
* **perf-dfa** -
  Enables the use of a lazy DFA for matching. The lazy DFA is used to compile
  portions of a regex to a very fast DFA on an as-needed basis. This can
  result in substantial speedups, usually by an order of magnitude on large
  haystacks. The lazy DFA does not bring in any new dependencies, but it can
  make compile times longer.
* **perf-dfa-full** -
  Enables the use of a full DFA for matching. Full DFAs are problematic because
  they have worst case `O(2^n)` construction time. For this reason, when this
  feature is enabled, full DFAs are only used for very small regexes and a
  very small space bound is used during determinization to avoid the DFA
  from blowing up. This feature is not enabled by default, even as part of
  `perf`, because it results in fairly sizeable increases in binary size and
  compilation time. It can result in faster search times, but they tend to be
  more modest and limited to non-Unicode regexes.
* **perf-onepass** -
  Enables the use of a one-pass DFA for extracting the positions of capture
  groups. This optimization applies to a subset of certain types of NFAs and
  represents the fastest engine in this crate for dealing with capture groups.
* **perf-backtrack** -
  Enables the use of a bounded backtracking algorithm for extracting the
  positions of capture groups. This usually sits between the slowest engine
  (the PikeVM) and the fastest engine (one-pass DFA) for extracting capture
  groups. It's used whenever the regex is not one-pass and is small enough.
* **perf-inline** -
  Enables the use of aggressive inlining inside match routines. This reduces
  the overhead of each match. The aggressive inlining, however, increases
  compile times and binary size.
* **perf-literal** -
  Enables the use of literal optimizations for speeding up matches. In some
  cases, literal optimizations can result in speedups of _several_ orders of
  magnitude. Disabling this drops the `aho-corasick` and `memchr` dependencies.
* **perf-cache** -
  This feature used to enable a faster internal cache at the cost of using
  additional dependencies, but this is no longer an option. A fast internal
  cache is now used unconditionally with no additional dependencies. This may
  change in the future.

### Unicode features

* **unicode** -
  Enables all Unicode features. This feature is enabled by default, and will
  always cover all Unicode features, even if more are added in the future.
* **unicode-age** -
  Provide the data for the
  [Unicode `Age` property](https://www.unicode.org/reports/tr44/tr44-24.html#Character_Age).
  This makes it possible to use classes like `\p{Age:6.0}` to refer to all
  codepoints first introduced in Unicode 6.0
* **unicode-bool** -
  Provide the data for numerous Unicode boolean properties. The full list
  is not included here, but contains properties like `Alphabetic`, `Emoji`,
  `Lowercase`, `Math`, `Uppercase` and `White_Space`.
* **unicode-case** -
  Provide the data for case insensitive matching using
  [Unicode's "simple loose matches" specification](https://www.unicode.org/reports/tr18/#Simple_Loose_Matches).
* **unicode-gencat** -
  Provide the data for
  [Unicode general categories](https://www.unicode.org/reports/tr44/tr44-24.html#General_Category_Values).
  This includes, but is not limited to, `Decimal_Number`, `Letter`,
  `Math_Symbol`, `Number` and `Punctuation`.
* **unicode-perl** -
  Provide the data for supporting the Unicode-aware Perl character classes,
  corresponding to `\w`, `\s` and `\d`. This is also necessary for using
  Unicode-aware word boundary assertions. Note that if this feature is
  disabled, the `\s` and `\d` character classes are still available if the
  `unicode-bool` and `unicode-gencat` features are enabled, respectively.
* **unicode-script** -
  Provide the data for
  [Unicode scripts and script extensions](https://www.unicode.org/reports/tr24/).
  This includes, but is not limited to, `Arabic`, `Cyrillic`, `Hebrew`,
  `Latin` and `Thai`.
* **unicode-segment** -
  Provide the data necessary to provide the properties used to implement the
  [Unicode text segmentation algorithms](https://www.unicode.org/reports/tr29/).
  This enables using classes like `\p{gcb=Extend}`, `\p{wb=Katakana}` and
  `\p{sb=ATerm}`.

# Other crates

This crate has two required dependencies and several optional dependencies.
This section briefly describes them with the goal of raising awareness of how
different components of this crate may be used independently.

It is somewhat unusual for a regex engine to have dependencies, as most regex
libraries are self contained units with no dependencies other than a particular
environment's standard library. Indeed, for other similarly optimized regex
engines, most or all of the code in the dependencies of this crate would
normally just be inseparable or coupled parts of the crate itself. But since
Rust and its tooling ecosystem make the use of dependencies so easy, it made
sense to spend some effort de-coupling parts of this crate and making them
independently useful.

We only briefly describe each crate here.

* [`regex-lite`](https://docs.rs/regex-lite) is not a dependency of `regex`,
but rather, a standalone zero-dependency simpler version of `regex` that
prioritizes compile times and binary size. In exchange, it eschews Unicode
support and performance. Its match semantics are as identical as possible to
the `regex` crate, and for the things it supports, its APIs are identical to
the APIs in this crate. In other words, for a lot of use cases, it is a drop-in
replacement.
* [`regex-syntax`](https://docs.rs/regex-syntax) provides a regular expression
parser via `Ast` and `Hir` types. It also provides routines for extracting
literals from a pattern. Folks can use this crate to do analysis, or even to
build their own regex engine without having to worry about writing a parser.
* [`regex-automata`](https://docs.rs/regex-automata) provides the regex engines
themselves. One of the downsides of finite automata based regex engines is that
they often need multiple internal engines in order to have similar or better
performance than an unbounded backtracking engine in practice. `regex-automata`
in particular provides public APIs for a PikeVM, a bounded backtracker, a
one-pass DFA, a lazy DFA, a fully compiled DFA and a meta regex engine that
combines all them together. It also has native multi-pattern support and
provides a way to compile and serialize full DFAs such that they can be loaded
and searched in a no-std no-alloc environment. `regex-automata` itself doesn't
even have a required dependency on `regex-syntax`!
* [`memchr`](https://docs.rs/memchr) provides low level SIMD vectorized
routines for quickly finding the location of single bytes or even substrings
in a haystack. In other words, it provides fast `memchr` and `memmem` routines.
These are used by this crate in literal optimizations.
* [`aho-corasick`](https://docs.rs/aho-corasick) provides multi-substring
search. It also provides SIMD vectorized routines in the case where the number
of substrings to search for is relatively small. The `regex` crate also uses
this for literal optimizations.

## Contents

- [Modules](#modules)
  - [`builders`](#builders)
  - [`bytes`](#bytes)
  - [`error`](#error)
  - [`find_byte`](#find-byte)
  - [`regex`](#regex)
  - [`regexset`](#regexset)
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
- [Enums](#enums)
  - [`Error`](#error)
- [Traits](#traits)
  - [`Replacer`](#replacer)
- [Functions](#functions)
  - [`escape`](#escape)
  - [`no_expansion`](#no-expansion)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`builders`](#builders) | mod |  |
| [`bytes`](#bytes) | mod | Search for regex matches in `&[u8]` haystacks. |
| [`error`](#error) | mod |  |
| [`find_byte`](#find-byte) | mod |  |
| [`regex`](#regex) | mod |  |
| [`regexset`](#regexset) | mod |  |
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
| [`Error`](#error) | enum |  |
| [`Replacer`](#replacer) | trait | A trait for types that can be used to replace matches in a haystack. |
| [`escape`](#escape) | fn | Escapes all regular expression meta characters in `pattern`. |
| [`no_expansion`](#no-expansion) | fn | Quickly checks the given replacement string for whether interpolation should be done on it. |

## Modules

- [`builders`](builders/index.md)
- [`bytes`](bytes/index.md) — Search for regex matches in `&[u8]` haystacks.
- [`error`](error/index.md)
- [`find_byte`](find_byte/index.md)
- [`regex`](regex/index.md)
- [`regexset`](regexset/index.md)

## Structs

### `RegexBuilder`

```rust
struct RegexBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:212-214`](../../.source_1765633015/regex-1.12.2/src/builders.rs#L212-L214)*

A configurable builder for a [`Regex`](#regex).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- <span id="regexbuilder-new"></span>`fn new(pattern: &str) -> RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  Create a new builder with a default configuration for the given

  pattern.

  

  If the pattern is invalid or exceeds the configured size limits,

  then an error will be returned when `RegexBuilder::build` is

  called.

- <span id="regexbuilder-build"></span>`fn build(&self) -> Result<Regex, Error>` — [`Regex`](#regex), [`Error`](error/index.md#error)

  Compiles the pattern given to `RegexBuilder::new` with the

  configuration set on this builder.

  

  If the pattern isn't a valid regex or if a configured size limit

  was exceeded, then an error is returned.

- <span id="regexbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures Unicode mode for the entire pattern.

  

  Enabling Unicode mode does a number of things:

  

  * Most fundamentally, it causes the fundamental atom of matching

  to be a single codepoint. When Unicode mode is disabled, it's a

  single byte. For example, when Unicode mode is enabled, `.` will

  match `💩` once, where as it will match 4 times when Unicode mode

  is disabled. (Since the UTF-8 encoding of `💩` is 4 bytes long.)

  * Case insensitive matching uses Unicode simple case folding rules.

  * Unicode character classes like `\p{Letter}` and `\p{Greek}` are

  available.

  * Perl character classes are Unicode aware. That is, `\w`, `\s` and

  `\d`.

  * The word boundary assertions, `\b` and `\B`, use the Unicode

  definition of a word character.

  

  Note that if Unicode mode is disabled, then the regex will fail to

  compile if it could match invalid UTF-8. For example, when Unicode

  mode is disabled, then since `.` matches any byte (except for

  `\n`), then it can match invalid UTF-8 and thus building a regex

  from it will fail. Another example is `\w` and `\W`. Since `\w` can

  only match ASCII bytes when Unicode mode is disabled, it's allowed.

  But `\W` can match more than ASCII bytes, including invalid UTF-8,

  and so it is not allowed. This restriction can be lifted only by

  using a [`bytes::Regex`](crate::bytes::Regex).

  

  For more details on the Unicode support in this crate, see the

  [Unicode section](crate#unicode) in this crate's top-level

  documentation.

  

  The default for this is `true`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"\w")

      .unicode(false)

      .build()

      .unwrap();

  // Normally greek letters would be included in \w, but since

  // Unicode mode is disabled, it only matches ASCII letters.

  assert!(!re.is_match("δ"));

  

  let re = RegexBuilder::new(r"s")

      .case_insensitive(true)

      .unicode(false)

      .build()

      .unwrap();

  // Normally 'ſ' is included when searching for 's' case

  // insensitively due to Unicode's simple case folding rules. But

  // when Unicode mode is disabled, only ASCII case insensitive rules

  // are used.

  assert!(!re.is_match("ſ"));

  ```

- <span id="regexbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures whether to enable case insensitive matching for the

  entire pattern.

  

  This setting can also be configured using the inline flag `i`

  in the pattern. For example, `(?i:foo)` matches `foo` case

  insensitively while `(?-i:foo)` matches `foo` case sensitively.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"foo(?-i:bar)quux")

      .case_insensitive(true)

      .build()

      .unwrap();

  assert!(re.is_match("FoObarQuUx"));

  // Even though case insensitive matching is enabled in the builder,

  // it can be locally disabled within the pattern. In this case,

  // `bar` is matched case sensitively.

  assert!(!re.is_match("fooBARquux"));

  ```

- <span id="regexbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures multi-line mode for the entire pattern.

  

  Enabling multi-line mode changes the behavior of the `^` and `$`

  anchor assertions. Instead of only matching at the beginning and

  end of a haystack, respectively, multi-line mode causes them to

  match at the beginning and end of a line *in addition* to the

  beginning and end of a haystack. More precisely, `^` will match at

  the position immediately following a `\n` and `$` will match at the

  position immediately preceding a `\n`.

  

  The behavior of this option can be impacted by other settings too:

  

  * The `RegexBuilder::line_terminator` option changes `\n` above

  to any ASCII byte.

  * The `RegexBuilder::crlf` option changes the line terminator to

  be either `\r` or `\n`, but never at the position between a `\r`

  and `\n`.

  

  This setting can also be configured using the inline flag `m` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"^foo$")

      .multi_line(true)

      .build()

      .unwrap();

  assert_eq!(Some(1..4), re.find("\nfoo\n").map(|m| m.range()));

  ```

- <span id="regexbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures dot-matches-new-line mode for the entire pattern.

  

  Perhaps surprisingly, the default behavior for `.` is not to match

  any character, but rather, to match any character except for the

  line terminator (which is `\n` by default). When this mode is

  enabled, the behavior changes such that `.` truly matches any

  character.

  

  This setting can also be configured using the inline flag `s` in

  the pattern. For example, `(?s:.)` and `\p{any}` are equivalent

  regexes.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"foo.bar")

      .dot_matches_new_line(true)

      .build()

      .unwrap();

  let hay = "foo\nbar";

  assert_eq!(Some("foo\nbar"), re.find(hay).map(|m| m.as_str()));

  ```

- <span id="regexbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures CRLF mode for the entire pattern.

  

  When CRLF mode is enabled, both `\r` ("carriage return" or CR for

  short) and `\n` ("line feed" or LF for short) are treated as line

  terminators. This results in the following:

  

  * Unless dot-matches-new-line mode is enabled, `.` will now match

  any character except for `\n` and `\r`.

  * When multi-line mode is enabled, `^` will match immediately

  following a `\n` or a `\r`. Similarly, `$` will match immediately

  preceding a `\n` or a `\r`. Neither `^` nor `$` will ever match

  between `\r` and `\n`.

  

  This setting can also be configured using the inline flag `R` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"^foo$")

      .multi_line(true)

      .crlf(true)

      .build()

      .unwrap();

  let hay = "\r\nfoo\r\n";

  // If CRLF mode weren't enabled here, then '$' wouldn't match

  // immediately after 'foo', and thus no match would be found.

  assert_eq!(Some("foo"), re.find(hay).map(|m| m.as_str()));

  ```

  

  This example demonstrates that `^` will never match at a position

  between `\r` and `\n`. (`$` will similarly not match between a `\r`

  and a `\n`.)

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"^")

      .multi_line(true)

      .crlf(true)

      .build()

      .unwrap();

  let hay = "\r\n\r\n";

  let ranges: Vec<_> = re.find_iter(hay).map(|m| m.range()).collect();

  assert_eq!(ranges, vec![0..0, 2..2, 4..4]);

  ```

- <span id="regexbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  Configures the line terminator to be used by the regex.

  

  The line terminator is relevant in two ways for a particular regex:

  

  * When dot-matches-new-line mode is *not* enabled (the default),

  then `.` will match any character except for the configured line

  terminator.

  * When multi-line mode is enabled (not the default), then `^` and

  `$` will match immediately after and before, respectively, a line

  terminator.

  

  In both cases, if CRLF mode is enabled in a particular context,

  then it takes precedence over any configured line terminator.

  

  This option cannot be configured from within the pattern.

  

  The default line terminator is `\n`.

  

  # Example

  

  This shows how to treat the NUL byte as a line terminator. This can

  be a useful heuristic when searching binary data.

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"^foo$")

      .multi_line(true)

      .line_terminator(b'\x00')

      .build()

      .unwrap();

  let hay = "\x00foo\x00";

  assert_eq!(Some(1..4), re.find(hay).map(|m| m.range()));

  ```

  

  This example shows that the behavior of `.` is impacted by this

  setting as well:

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r".")

      .line_terminator(b'\x00')

      .build()

      .unwrap();

  assert!(re.is_match("\n"));

  assert!(!re.is_match("\x00"));

  ```

  

  This shows that building a regex will fail if the byte given

  is not ASCII and the pattern could result in matching invalid

  UTF-8. This is because any singular non-ASCII byte is not valid

  UTF-8, and it is not permitted for a [`Regex`](#regex) to match invalid

  UTF-8. (It is permissible to use a non-ASCII byte when building a

  [`bytes::Regex`](crate::bytes::Regex).)

  

  ```rust

  use regex::RegexBuilder;

  

  assert!(RegexBuilder::new(r".").line_terminator(0x80).build().is_err());

  // Note that using a non-ASCII byte isn't enough on its own to

  // cause regex compilation to fail. You actually have to make use

  // of it in the regex in a way that leads to matching invalid

  // UTF-8. If you don't, then regex compilation will succeed!

  assert!(RegexBuilder::new(r"a").line_terminator(0x80).build().is_ok());

  ```

- <span id="regexbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures swap-greed mode for the entire pattern.

  

  When swap-greed mode is enabled, patterns like `a+` will become

  non-greedy and patterns like `a+?` will become greedy. In other

  words, the meanings of `a+` and `a+?` are switched.

  

  This setting can also be configured using the inline flag `U` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let re = RegexBuilder::new(r"a+")

      .swap_greed(true)

      .build()

      .unwrap();

  assert_eq!(Some("a"), re.find("aaa").map(|m| m.as_str()));

  ```

- <span id="regexbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures verbose mode for the entire pattern.

  

  When enabled, whitespace will treated as insignificant in the

  pattern and `#` can be used to start a comment until the next new

  line.

  

  Normally, in most places in a pattern, whitespace is treated

  literally. For example ` +` will match one or more ASCII whitespace

  characters.

  

  When verbose mode is enabled, `\#` can be used to match a literal

  `#` and `\ ` can be used to match a literal ASCII whitespace

  character.

  

  Verbose mode is useful for permitting regexes to be formatted and

  broken up more nicely. This may make them more easily readable.

  

  This setting can also be configured using the inline flag `x` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  let pat = r"

      \b

      (?<first>\p{Uppercase}\w*)  # always start with uppercase letter

      [\s--\n]+                   # whitespace should separate names

      (?: # middle name can be an initial!

          (?:(?<initial>\p{Uppercase})\.|(?<middle>\p{Uppercase}\w*))

          [\s--\n]+

      )?

      (?<last>\p{Uppercase}\w*)

      \b

  ";

  let re = RegexBuilder::new(pat)

      .ignore_whitespace(true)

      .build()

      .unwrap();

  

  let caps = re.captures("Harry Potter").unwrap();

  assert_eq!("Harry", &caps["first"]);

  assert_eq!("Potter", &caps["last"]);

  

  let caps = re.captures("Harry J. Potter").unwrap();

  assert_eq!("Harry", &caps["first"]);

  // Since a middle name/initial isn't required for an overall match,

  // we can't assume that 'initial' or 'middle' will be populated!

  assert_eq!(Some("J"), caps.name("initial").map(|m| m.as_str()));

  assert_eq!(None, caps.name("middle").map(|m| m.as_str()));

  assert_eq!("Potter", &caps["last"]);

  

  let caps = re.captures("Harry James Potter").unwrap();

  assert_eq!("Harry", &caps["first"]);

  // Since a middle name/initial isn't required for an overall match,

  // we can't assume that 'initial' or 'middle' will be populated!

  assert_eq!(None, caps.name("initial").map(|m| m.as_str()));

  assert_eq!(Some("James"), caps.name("middle").map(|m| m.as_str()));

  assert_eq!("Potter", &caps["last"]);

  ```

- <span id="regexbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  This configures octal mode for the entire pattern.

  

  Octal syntax is a little-known way of uttering Unicode codepoints

  in a pattern. For example, `a`, `\x61`, `\u0061` and `\141` are all

  equivalent patterns, where the last example shows octal syntax.

  

  While supporting octal syntax isn't in and of itself a problem,

  it does make good error messages harder. That is, in PCRE based

  regex engines, syntax like `\1` invokes a backreference, which is

  explicitly unsupported this library. However, many users expect

  backreferences to be supported. Therefore, when octal support

  is disabled, the error message will explicitly mention that

  backreferences aren't supported.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  // Normally this pattern would not compile, with an error message

  // about backreferences not being supported. But with octal mode

  // enabled, octal escape sequences work.

  let re = RegexBuilder::new(r"\141")

      .octal(true)

      .build()

      .unwrap();

  assert!(re.is_match("a"));

  ```

- <span id="regexbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  Sets the approximate size limit, in bytes, of the compiled regex.

  

  This roughly corresponds to the number of heap memory, in

  bytes, occupied by a single regex. If the regex would otherwise

  approximately exceed this limit, then compiling that regex will

  fail.

  

  The main utility of a method like this is to avoid compiling

  regexes that use an unexpected amount of resources, such as

  time and memory. Even if the memory usage of a large regex is

  acceptable, its search time may not be. Namely, worst case time

  complexity for search is `O(m * n)`, where `m ~ len(pattern)` and

  `n ~ len(haystack)`. That is, search time depends, in part, on the

  size of the compiled regex. This means that putting a limit on the

  size of the regex limits how much a regex can impact search time.

  

  For more information about regex size limits, see the section on

  [untrusted inputs](crate#untrusted-input) in the top-level crate

  documentation.

  

  The default for this is some reasonable number that permits most

  patterns to compile successfully.

  

  # Example

  

  ```rust

  if !cfg!(target_pointer_width = "64") { return; } // see #1041

  use regex::RegexBuilder;

  

  // It may surprise you how big some seemingly small patterns can

  // be! Since \w is Unicode aware, this generates a regex that can

  // match approximately 140,000 distinct codepoints.

  assert!(RegexBuilder::new(r"\w").size_limit(45_000).build().is_err());

  ```

- <span id="regexbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  Set the approximate capacity, in bytes, of the cache of transitions

  used by the lazy DFA.

  

  While the lazy DFA isn't always used, in tends to be the most

  commonly use regex engine in default configurations. It tends to

  adopt the performance profile of a fully build DFA, but without the

  downside of taking worst case exponential time to build.

  

  The downside is that it needs to keep a cache of transitions and

  states that are built while running a search, and this cache

  can fill up. When it fills up, the cache will reset itself. Any

  previously generated states and transitions will then need to be

  re-generated. If this happens too many times, then this library

  will bail out of using the lazy DFA and switch to a different regex

  engine.

  

  If your regex provokes this particular downside of the lazy DFA,

  then it may be beneficial to increase its cache capacity. This will

  potentially reduce the frequency of cache resetting (ideally to

  `0`). While it won't fix all potential performance problems with

  the lazy DFA, increasing the cache capacity does fix some.

  

  There is no easy way to determine, a priori, whether increasing

  this cache capacity will help. In general, the larger your regex,

  the more cache it's likely to use. But that isn't an ironclad rule.

  For example, a regex like `[01]*1[01]{N}` would normally produce a

  fully build DFA that is exponential in size with respect to `N`.

  The lazy DFA will prevent exponential space blow-up, but it cache

  is likely to fill up, even when it's large and even for smallish

  values of `N`.

  

  If you aren't sure whether this helps or not, it is sensible to

  set this to some arbitrarily large number in testing, such as

  `usize::MAX`. Namely, this represents the amount of capacity that

  *may* be used. It's probably not a good idea to use `usize::MAX` in

  production though, since it implies there are no controls on heap

  memory used by this library during a search. In effect, set it to

  whatever you're willing to allocate for a single regex search.

- <span id="regexbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder` — [`RegexBuilder`](#regexbuilder)

  Set the nesting limit for this parser.

  

  The nesting limit controls how deep the abstract syntax tree is

  allowed to be. If the AST exceeds the given limit (e.g., with too

  many nested groups), then an error is returned by the parser.

  

  The purpose of this limit is to act as a heuristic to prevent stack

  overflow for consumers that do structural induction on an AST using

  explicit recursion. While this crate never does this (instead using

  constant stack space and moving the call stack to the heap), other

  crates may.

  

  This limit is not checked until the entire AST is parsed.

  Therefore, if callers want to put a limit on the amount of heap

  space used, then they should impose a limit on the length, in

  bytes, of the concrete pattern string. In particular, this is

  viable since this parser implementation will limit itself to heap

  space proportional to the length of the pattern string. See also

  the [untrusted inputs](crate#untrusted-input) section in the

  top-level crate documentation for more information about this.

  

  Note that a nest limit of `0` will return a nest limit error for

  most patterns but not all. For example, a nest limit of `0` permits

  `a` but not `ab`, since `ab` requires an explicit concatenation,

  which results in a nest depth of `1`. In general, a nest limit is

  not something that manifests in an obvious way in the concrete

  syntax, therefore, it should not be used in a granular way.

  

  # Example

  

  ```rust

  use regex::RegexBuilder;

  

  assert!(RegexBuilder::new(r"a").nest_limit(0).build().is_ok());

  assert!(RegexBuilder::new(r"ab").nest_limit(0).build().is_err());

  ```

#### Trait Implementations

##### `impl Any for RegexBuilder`

- <span id="regexbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegexBuilder`

- <span id="regexbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegexBuilder`

- <span id="regexbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RegexBuilder`

- <span id="regexbuilder-clone"></span>`fn clone(&self) -> RegexBuilder` — [`RegexBuilder`](#regexbuilder)

##### `impl CloneToUninit for RegexBuilder`

- <span id="regexbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RegexBuilder`

- <span id="regexbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RegexBuilder`

- <span id="regexbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegexBuilder`

- <span id="regexbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RegexBuilder`

- <span id="regexbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="regexbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regexbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RegexBuilder`

- <span id="regexbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regexbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegexBuilder`

- <span id="regexbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regexbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `RegexSetBuilder`

```rust
struct RegexSetBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:787-789`](../../.source_1765633015/regex-1.12.2/src/builders.rs#L787-L789)*

A configurable builder for a [`RegexSet`](#regexset).

This builder can be used to programmatically set flags such as
`i` (case insensitive) and `x` (for verbose mode). This builder
can also be used to configure things like the line terminator
and a size limit on the compiled regular expression.

#### Implementations

- <span id="regexsetbuilder-new"></span>`fn new<I, S>(patterns: I) -> RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  Create a new builder with a default configuration for the given

  patterns.

  

  If the patterns are invalid or exceed the configured size limits,

  then an error will be returned when `RegexSetBuilder::build` is

  called.

- <span id="regexsetbuilder-build"></span>`fn build(&self) -> Result<RegexSet, Error>` — [`RegexSet`](#regexset), [`Error`](error/index.md#error)

  Compiles the patterns given to `RegexSetBuilder::new` with the

  configuration set on this builder.

  

  If the patterns aren't valid regexes or if a configured size limit

  was exceeded, then an error is returned.

- <span id="regexsetbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures Unicode mode for the all of the patterns.

  

  Enabling Unicode mode does a number of things:

  

  * Most fundamentally, it causes the fundamental atom of matching

  to be a single codepoint. When Unicode mode is disabled, it's a

  single byte. For example, when Unicode mode is enabled, `.` will

  match `💩` once, where as it will match 4 times when Unicode mode

  is disabled. (Since the UTF-8 encoding of `💩` is 4 bytes long.)

  * Case insensitive matching uses Unicode simple case folding rules.

  * Unicode character classes like `\p{Letter}` and `\p{Greek}` are

  available.

  * Perl character classes are Unicode aware. That is, `\w`, `\s` and

  `\d`.

  * The word boundary assertions, `\b` and `\B`, use the Unicode

  definition of a word character.

  

  Note that if Unicode mode is disabled, then the regex will fail to

  compile if it could match invalid UTF-8. For example, when Unicode

  mode is disabled, then since `.` matches any byte (except for

  `\n`), then it can match invalid UTF-8 and thus building a regex

  from it will fail. Another example is `\w` and `\W`. Since `\w` can

  only match ASCII bytes when Unicode mode is disabled, it's allowed.

  But `\W` can match more than ASCII bytes, including invalid UTF-8,

  and so it is not allowed. This restriction can be lifted only by

  using a [`bytes::RegexSet`](crate::bytes::RegexSet).

  

  For more details on the Unicode support in this crate, see the

  [Unicode section](crate#unicode) in this crate's top-level

  documentation.

  

  The default for this is `true`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"\w"])

      .unicode(false)

      .build()

      .unwrap();

  // Normally greek letters would be included in \w, but since

  // Unicode mode is disabled, it only matches ASCII letters.

  assert!(!re.is_match("δ"));

  

  let re = RegexSetBuilder::new([r"s"])

      .case_insensitive(true)

      .unicode(false)

      .build()

      .unwrap();

  // Normally 'ſ' is included when searching for 's' case

  // insensitively due to Unicode's simple case folding rules. But

  // when Unicode mode is disabled, only ASCII case insensitive rules

  // are used.

  assert!(!re.is_match("ſ"));

  ```

- <span id="regexsetbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures whether to enable case insensitive matching for all

  of the patterns.

  

  This setting can also be configured using the inline flag `i`

  in the pattern. For example, `(?i:foo)` matches `foo` case

  insensitively while `(?-i:foo)` matches `foo` case sensitively.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"foo(?-i:bar)quux"])

      .case_insensitive(true)

      .build()

      .unwrap();

  assert!(re.is_match("FoObarQuUx"));

  // Even though case insensitive matching is enabled in the builder,

  // it can be locally disabled within the pattern. In this case,

  // `bar` is matched case sensitively.

  assert!(!re.is_match("fooBARquux"));

  ```

- <span id="regexsetbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures multi-line mode for all of the patterns.

  

  Enabling multi-line mode changes the behavior of the `^` and `$`

  anchor assertions. Instead of only matching at the beginning and

  end of a haystack, respectively, multi-line mode causes them to

  match at the beginning and end of a line *in addition* to the

  beginning and end of a haystack. More precisely, `^` will match at

  the position immediately following a `\n` and `$` will match at the

  position immediately preceding a `\n`.

  

  The behavior of this option can be impacted by other settings too:

  

  * The `RegexSetBuilder::line_terminator` option changes `\n`

  above to any ASCII byte.

  * The `RegexSetBuilder::crlf` option changes the line terminator

  to be either `\r` or `\n`, but never at the position between a `\r`

  and `\n`.

  

  This setting can also be configured using the inline flag `m` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"^foo$"])

      .multi_line(true)

      .build()

      .unwrap();

  assert!(re.is_match("\nfoo\n"));

  ```

- <span id="regexsetbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures dot-matches-new-line mode for the entire pattern.

  

  Perhaps surprisingly, the default behavior for `.` is not to match

  any character, but rather, to match any character except for the

  line terminator (which is `\n` by default). When this mode is

  enabled, the behavior changes such that `.` truly matches any

  character.

  

  This setting can also be configured using the inline flag `s` in

  the pattern. For example, `(?s:.)` and `\p{any}` are equivalent

  regexes.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"foo.bar"])

      .dot_matches_new_line(true)

      .build()

      .unwrap();

  let hay = "foo\nbar";

  assert!(re.is_match(hay));

  ```

- <span id="regexsetbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures CRLF mode for all of the patterns.

  

  When CRLF mode is enabled, both `\r` ("carriage return" or CR for

  short) and `\n` ("line feed" or LF for short) are treated as line

  terminators. This results in the following:

  

  * Unless dot-matches-new-line mode is enabled, `.` will now match

  any character except for `\n` and `\r`.

  * When multi-line mode is enabled, `^` will match immediately

  following a `\n` or a `\r`. Similarly, `$` will match immediately

  preceding a `\n` or a `\r`. Neither `^` nor `$` will ever match

  between `\r` and `\n`.

  

  This setting can also be configured using the inline flag `R` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"^foo$"])

      .multi_line(true)

      .crlf(true)

      .build()

      .unwrap();

  let hay = "\r\nfoo\r\n";

  // If CRLF mode weren't enabled here, then '$' wouldn't match

  // immediately after 'foo', and thus no match would be found.

  assert!(re.is_match(hay));

  ```

  

  This example demonstrates that `^` will never match at a position

  between `\r` and `\n`. (`$` will similarly not match between a `\r`

  and a `\n`.)

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"^\n"])

      .multi_line(true)

      .crlf(true)

      .build()

      .unwrap();

  assert!(!re.is_match("\r\n"));

  ```

- <span id="regexsetbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  Configures the line terminator to be used by the regex.

  

  The line terminator is relevant in two ways for a particular regex:

  

  * When dot-matches-new-line mode is *not* enabled (the default),

  then `.` will match any character except for the configured line

  terminator.

  * When multi-line mode is enabled (not the default), then `^` and

  `$` will match immediately after and before, respectively, a line

  terminator.

  

  In both cases, if CRLF mode is enabled in a particular context,

  then it takes precedence over any configured line terminator.

  

  This option cannot be configured from within the pattern.

  

  The default line terminator is `\n`.

  

  # Example

  

  This shows how to treat the NUL byte as a line terminator. This can

  be a useful heuristic when searching binary data.

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"^foo$"])

      .multi_line(true)

      .line_terminator(b'\x00')

      .build()

      .unwrap();

  let hay = "\x00foo\x00";

  assert!(re.is_match(hay));

  ```

  

  This example shows that the behavior of `.` is impacted by this

  setting as well:

  

  ```rust

  use regex::RegexSetBuilder;

  

  let re = RegexSetBuilder::new([r"."])

      .line_terminator(b'\x00')

      .build()

      .unwrap();

  assert!(re.is_match("\n"));

  assert!(!re.is_match("\x00"));

  ```

  

  This shows that building a regex will fail if the byte given

  is not ASCII and the pattern could result in matching invalid

  UTF-8. This is because any singular non-ASCII byte is not valid

  UTF-8, and it is not permitted for a [`RegexSet`](#regexset) to match invalid

  UTF-8. (It is permissible to use a non-ASCII byte when building a

  [`bytes::RegexSet`](crate::bytes::RegexSet).)

  

  ```rust

  use regex::RegexSetBuilder;

  

  assert!(

      RegexSetBuilder::new([r"."])

          .line_terminator(0x80)

          .build()

          .is_err()

  );

  // Note that using a non-ASCII byte isn't enough on its own to

  // cause regex compilation to fail. You actually have to make use

  // of it in the regex in a way that leads to matching invalid

  // UTF-8. If you don't, then regex compilation will succeed!

  assert!(

      RegexSetBuilder::new([r"a"])

          .line_terminator(0x80)

          .build()

          .is_ok()

  );

  ```

- <span id="regexsetbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures swap-greed mode for all of the patterns.

  

  When swap-greed mode is enabled, patterns like `a+` will become

  non-greedy and patterns like `a+?` will become greedy. In other

  words, the meanings of `a+` and `a+?` are switched.

  

  This setting can also be configured using the inline flag `U` in

  the pattern.

  

  Note that this is generally not useful for a `RegexSet` since a

  `RegexSet` can only report whether a pattern matches or not. Since

  greediness never impacts whether a match is found or not (only the

  offsets of the match), it follows that whether parts of a pattern

  are greedy or not doesn't matter for a `RegexSet`.

  

  The default for this is `false`.

- <span id="regexsetbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures verbose mode for all of the patterns.

  

  When enabled, whitespace will treated as insignificant in the

  pattern and `#` can be used to start a comment until the next new

  line.

  

  Normally, in most places in a pattern, whitespace is treated

  literally. For example ` +` will match one or more ASCII whitespace

  characters.

  

  When verbose mode is enabled, `\#` can be used to match a literal

  `#` and `\ ` can be used to match a literal ASCII whitespace

  character.

  

  Verbose mode is useful for permitting regexes to be formatted and

  broken up more nicely. This may make them more easily readable.

  

  This setting can also be configured using the inline flag `x` in

  the pattern.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  let pat = r"

      \b

      (?<first>\p{Uppercase}\w*)  # always start with uppercase letter

      [\s--\n]+                   # whitespace should separate names

      (?: # middle name can be an initial!

          (?:(?<initial>\p{Uppercase})\.|(?<middle>\p{Uppercase}\w*))

          [\s--\n]+

      )?

      (?<last>\p{Uppercase}\w*)

      \b

  ";

  let re = RegexSetBuilder::new([pat])

      .ignore_whitespace(true)

      .build()

      .unwrap();

  assert!(re.is_match("Harry Potter"));

  assert!(re.is_match("Harry J. Potter"));

  assert!(re.is_match("Harry James Potter"));

  assert!(!re.is_match("harry J. Potter"));

  ```

- <span id="regexsetbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  This configures octal mode for all of the patterns.

  

  Octal syntax is a little-known way of uttering Unicode codepoints

  in a pattern. For example, `a`, `\x61`, `\u0061` and `\141` are all

  equivalent patterns, where the last example shows octal syntax.

  

  While supporting octal syntax isn't in and of itself a problem,

  it does make good error messages harder. That is, in PCRE based

  regex engines, syntax like `\1` invokes a backreference, which is

  explicitly unsupported this library. However, many users expect

  backreferences to be supported. Therefore, when octal support

  is disabled, the error message will explicitly mention that

  backreferences aren't supported.

  

  The default for this is `false`.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  // Normally this pattern would not compile, with an error message

  // about backreferences not being supported. But with octal mode

  // enabled, octal escape sequences work.

  let re = RegexSetBuilder::new([r"\141"])

      .octal(true)

      .build()

      .unwrap();

  assert!(re.is_match("a"));

  ```

- <span id="regexsetbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  Sets the approximate size limit, in bytes, of the compiled regex.

  

  This roughly corresponds to the number of heap memory, in

  bytes, occupied by a single regex. If the regex would otherwise

  approximately exceed this limit, then compiling that regex will

  fail.

  

  The main utility of a method like this is to avoid compiling

  regexes that use an unexpected amount of resources, such as

  time and memory. Even if the memory usage of a large regex is

  acceptable, its search time may not be. Namely, worst case time

  complexity for search is `O(m * n)`, where `m ~ len(pattern)` and

  `n ~ len(haystack)`. That is, search time depends, in part, on the

  size of the compiled regex. This means that putting a limit on the

  size of the regex limits how much a regex can impact search time.

  

  For more information about regex size limits, see the section on

  [untrusted inputs](crate#untrusted-input) in the top-level crate

  documentation.

  

  The default for this is some reasonable number that permits most

  patterns to compile successfully.

  

  # Example

  

  ```rust

  if !cfg!(target_pointer_width = "64") { return; } // see #1041

  use regex::RegexSetBuilder;

  

  // It may surprise you how big some seemingly small patterns can

  // be! Since \w is Unicode aware, this generates a regex that can

  // match approximately 140,000 distinct codepoints.

  assert!(

      RegexSetBuilder::new([r"\w"])

          .size_limit(45_000)

          .build()

          .is_err()

  );

  ```

- <span id="regexsetbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  Set the approximate capacity, in bytes, of the cache of transitions

  used by the lazy DFA.

  

  While the lazy DFA isn't always used, in tends to be the most

  commonly use regex engine in default configurations. It tends to

  adopt the performance profile of a fully build DFA, but without the

  downside of taking worst case exponential time to build.

  

  The downside is that it needs to keep a cache of transitions and

  states that are built while running a search, and this cache

  can fill up. When it fills up, the cache will reset itself. Any

  previously generated states and transitions will then need to be

  re-generated. If this happens too many times, then this library

  will bail out of using the lazy DFA and switch to a different regex

  engine.

  

  If your regex provokes this particular downside of the lazy DFA,

  then it may be beneficial to increase its cache capacity. This will

  potentially reduce the frequency of cache resetting (ideally to

  `0`). While it won't fix all potential performance problems with

  the lazy DFA, increasing the cache capacity does fix some.

  

  There is no easy way to determine, a priori, whether increasing

  this cache capacity will help. In general, the larger your regex,

  the more cache it's likely to use. But that isn't an ironclad rule.

  For example, a regex like `[01]*1[01]{N}` would normally produce a

  fully build DFA that is exponential in size with respect to `N`.

  The lazy DFA will prevent exponential space blow-up, but it cache

  is likely to fill up, even when it's large and even for smallish

  values of `N`.

  

  If you aren't sure whether this helps or not, it is sensible to

  set this to some arbitrarily large number in testing, such as

  `usize::MAX`. Namely, this represents the amount of capacity that

  *may* be used. It's probably not a good idea to use `usize::MAX` in

  production though, since it implies there are no controls on heap

  memory used by this library during a search. In effect, set it to

  whatever you're willing to allocate for a single regex search.

- <span id="regexsetbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

  Set the nesting limit for this parser.

  

  The nesting limit controls how deep the abstract syntax tree is

  allowed to be. If the AST exceeds the given limit (e.g., with too

  many nested groups), then an error is returned by the parser.

  

  The purpose of this limit is to act as a heuristic to prevent stack

  overflow for consumers that do structural induction on an AST using

  explicit recursion. While this crate never does this (instead using

  constant stack space and moving the call stack to the heap), other

  crates may.

  

  This limit is not checked until the entire AST is parsed.

  Therefore, if callers want to put a limit on the amount of heap

  space used, then they should impose a limit on the length, in

  bytes, of the concrete pattern string. In particular, this is

  viable since this parser implementation will limit itself to heap

  space proportional to the length of the pattern string. See also

  the [untrusted inputs](crate#untrusted-input) section in the

  top-level crate documentation for more information about this.

  

  Note that a nest limit of `0` will return a nest limit error for

  most patterns but not all. For example, a nest limit of `0` permits

  `a` but not `ab`, since `ab` requires an explicit concatenation,

  which results in a nest depth of `1`. In general, a nest limit is

  not something that manifests in an obvious way in the concrete

  syntax, therefore, it should not be used in a granular way.

  

  # Example

  

  ```rust

  use regex::RegexSetBuilder;

  

  assert!(RegexSetBuilder::new([r"a"]).nest_limit(0).build().is_ok());

  assert!(RegexSetBuilder::new([r"ab"]).nest_limit(0).build().is_err());

  ```

#### Trait Implementations

##### `impl Any for RegexSetBuilder`

- <span id="regexsetbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for RegexSetBuilder`

- <span id="regexsetbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for RegexSetBuilder`

- <span id="regexsetbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for RegexSetBuilder`

- <span id="regexsetbuilder-clone"></span>`fn clone(&self) -> RegexSetBuilder` — [`RegexSetBuilder`](#regexsetbuilder)

##### `impl CloneToUninit for RegexSetBuilder`

- <span id="regexsetbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for RegexSetBuilder`

- <span id="regexsetbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for RegexSetBuilder`

- <span id="regexsetbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for RegexSetBuilder`

- <span id="regexsetbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for RegexSetBuilder`

- <span id="regexsetbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="regexsetbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="regexsetbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for RegexSetBuilder`

- <span id="regexsetbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="regexsetbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for RegexSetBuilder`

- <span id="regexsetbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="regexsetbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Regex`

```rust
struct Regex {
    meta: meta::Regex,
    pattern: alloc::sync::Arc<str>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:101-104`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L101-L104)*

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

- <span id="regex-new"></span>`fn new(re: &str) -> Result<Regex, Error>` — [`Regex`](#regex), [`Error`](error/index.md#error)

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

- <span id="regex-find"></span>`fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>>` — [`Match`](#match)

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

  use regex::Regex;

  

  let re = Regex::new(r"\b\w{13}\b").unwrap();

  let hay = "I categorically deny having triskaidekaphobia.";

  let mat = re.find(hay).unwrap();

  assert_eq!(2..15, mat.range());

  assert_eq!("categorically", mat.as_str());

  ```

- <span id="regex-find-iter"></span>`fn find_iter<'r, 'h>(self: &'r Self, haystack: &'h str) -> Matches<'r, 'h>` — [`Matches`](#matches)

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

- <span id="regex-captures"></span>`fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>>` — [`Captures`](#captures)

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

- <span id="regex-captures-iter"></span>`fn captures_iter<'r, 'h>(self: &'r Self, haystack: &'h str) -> CaptureMatches<'r, 'h>` — [`CaptureMatches`](#capturematches)

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

- <span id="regex-split"></span>`fn split<'r, 'h>(self: &'r Self, haystack: &'h str) -> Split<'r, 'h>` — [`Split`](#split)

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

- <span id="regex-splitn"></span>`fn splitn<'r, 'h>(self: &'r Self, haystack: &'h str, limit: usize) -> SplitN<'r, 'h>` — [`SplitN`](#splitn)

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

  use regex::Regex;

  

  let re = Regex::new(r"[^01]+").unwrap();

  assert_eq!(re.replace("1078910", ""), "1010");

  ```

  

  But anything satisfying the [`Replacer`](#replacer) trait will work. For example,

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

  string with [`NoExpand`](#noexpand):

  

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

- <span id="regex-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Regex, Error>` — [`Regex`](#regex), [`Error`](error/index.md#error)

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

*Defined in [`regex-1.12.2/src/regex/string.rs:1490-1494`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L1490-L1494)*

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
[Dijkstra's note on a related topic][`note`](../object/read/elf/note/index.md).


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

- <span id="match-new"></span>`fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h>` — [`Match`](#match)

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
    haystack: &'h str,
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:1642-1646`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L1642-L1646)*

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
use regex::Regex;

let re = Regex::new(r"(?<first>\w)(\w)(?:\w)\w(?<last>\w)").unwrap();
let caps = re.captures("toady").unwrap();
assert_eq!("toady", &caps[0]);
assert_eq!("t", &caps["first"]);
assert_eq!("o", &caps[2]);
assert_eq!("y", &caps["last"]);
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

  use regex::Regex;

  

  let re = Regex::new(r"[a-z]+(?:([0-9]+)|([A-Z]+))").unwrap();

  let caps = re.captures("abc123").unwrap();

  

  let substr1 = caps.get(1).map_or("", |m| m.as_str());

  let substr2 = caps.get(2).map_or("", |m| m.as_str());

  assert_eq!(substr1, "123");

  assert_eq!(substr2, "");

  ```

- <span id="captures-get-match"></span>`fn get_match(&self) -> Match<'h>` — [`Match`](#match)

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

*Defined in [`regex-1.12.2/src/regex/string.rs:2093`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2093)*

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
    haystack: &'h str,
    it: meta::FindMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2193-2196`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2193-L2196)*

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
    haystack: &'h str,
    it: meta::CapturesMatches<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2237-2240`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2237-L2240)*

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
    haystack: &'h str,
    it: meta::Split<'r, 'h>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2281-2284`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2281-L2284)*

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

*Defined in [`regex-1.12.2/src/regex/string.rs:2316-2319`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2316-L2319)*

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

*Defined in [`regex-1.12.2/src/regex/string.rs:2348`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2348)*

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
    haystack: &'h str,
    it: captures::CapturesPatternIter<'c>,
}
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2390-2393`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2390-L2393)*

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

*Defined in [`regex-1.12.2/src/regex/string.rs:2567`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2567)*

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

- <span id="replacerref-replacer-replace-append"></span>`fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String)` — [`Captures`](#captures)

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

*Defined in [`regex-1.12.2/src/regex/string.rs:2599`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2599)*

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

- <span id="noexpand-replacer-replace-append"></span>`fn replace_append(&mut self, _: &Captures<'_>, dst: &mut String)` — [`Captures`](#captures)

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

### `RegexSet`

```rust
struct RegexSet {
    meta: meta::Regex,
    patterns: alloc::sync::Arc<[alloc::string::String]>,
}
```

*Defined in [`regex-1.12.2/src/regexset/string.rs:132-135`](../../.source_1765633015/regex-1.12.2/src/regexset/string.rs#L132-L135)*

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

- <span id="regexset-new"></span>`fn new<I, S>(exprs: I) -> Result<RegexSet, Error>` — [`RegexSet`](#regexset), [`Error`](error/index.md#error)

  Create a new regex set with the given regular expressions.

  

  This takes an iterator of `S`, where `S` is something that can produce

  a `&str`. If any of the strings in the iterator are not valid regular

  expressions, then an error is returned.

  

  # Example

  

  Create a new regex set from an iterator of strings:

  

  ```rust

  use regex::RegexSet;

  

  let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();

  assert!(set.is_match("foo"));

  ```

- <span id="regexset-empty"></span>`fn empty() -> RegexSet` — [`RegexSet`](#regexset)

  Create a new empty regex set.

  

  An empty regex never matches anything.

  

  This is a convenience function for `RegexSet::new([])`, but doesn't

  require one to specify the type of the input.

  

  # Example

  

  ```rust

  use regex::RegexSet;

  

  let set = RegexSet::empty();

  assert!(set.is_empty());

  // an empty set matches nothing

  assert!(!set.is_match(""));

  ```

- <span id="regexset-is-match"></span>`fn is_match(&self, haystack: &str) -> bool`

  Returns true if and only if one of the regexes in this set matches

  the haystack given.

  

  This method should be preferred if you only need to test whether any

  of the regexes in the set should match, but don't care about *which*

  regexes matched. This is because the underlying matching engine will

  quit immediately after seeing the first match instead of continuing to

  find all matches.

  

  Note that as with searches using [`Regex`](crate::Regex), the

  expression is unanchored by default. That is, if the regex does not

  start with `^` or `\A`, or end with `$` or `\z`, then it is permitted

  to match anywhere in the haystack.

  

  # Example

  

  Tests whether a set matches somewhere in a haystack:

  

  ```rust

  use regex::RegexSet;

  

  let set = RegexSet::new([r"\w+", r"\d+"]).unwrap();

  assert!(set.is_match("foo"));

  assert!(!set.is_match("☃"));

  ```

- <span id="regexset-is-match-at"></span>`fn is_match_at(&self, haystack: &str, start: usize) -> bool`

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

  use regex::RegexSet;

  

  let set = RegexSet::new([r"\bbar\b", r"(?m)^bar$"]).unwrap();

  let hay = "foobar";

  // We get a match here, but it's probably not intended.

  assert!(set.is_match(&hay[3..]));

  // No match because the  assertions take the context into account.

  assert!(!set.is_match_at(hay, 3));

  ```

- <span id="regexset-matches"></span>`fn matches(&self, haystack: &str) -> SetMatches` — [`SetMatches`](#setmatches)

  Returns the set of regexes that match in the given haystack.

  

  The set returned contains the index of each regex that matches in

  the given haystack. The index is in correspondence with the order of

  regular expressions given to `RegexSet`'s constructor.

  

  The set can also be used to iterate over the matched indices. The order

  of iteration is always ascending with respect to the matching indices.

  

  Note that as with searches using [`Regex`](crate::Regex), the

  expression is unanchored by default. That is, if the regex does not

  start with `^` or `\A`, or end with `$` or `\z`, then it is permitted

  to match anywhere in the haystack.

  

  # Example

  

  Tests which regular expressions match the given haystack:

  

  ```rust

  use regex::RegexSet;

  

  let set = RegexSet::new([

      r"\w+",

      r"\d+",

      r"\pL+",

      r"foo",

      r"bar",

      r"barfoo",

      r"foobar",

  ]).unwrap();

  let matches: Vec<_> = set.matches("foobar").into_iter().collect();

  assert_eq!(matches, vec![0, 2, 3, 4, 6]);

  

  // You can also test whether a particular regex matched:

  let matches = set.matches("foobar");

  assert!(!matches.matched(5));

  assert!(matches.matched(6));

  ```

- <span id="regexset-matches-at"></span>`fn matches_at(&self, haystack: &str, start: usize) -> SetMatches` — [`SetMatches`](#setmatches)

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

  use regex::RegexSet;

  

  let set = RegexSet::new([r"\bbar\b", r"(?m)^bar$"]).unwrap();

  let hay = "foobar";

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

  use regex::RegexSet;

  

  assert_eq!(0, RegexSet::empty().len());

  assert_eq!(1, RegexSet::new([r"[0-9]"]).unwrap().len());

  assert_eq!(2, RegexSet::new([r"[0-9]", r"[a-z]"]).unwrap().len());

  ```

- <span id="regexset-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if this set contains no regexes.

  

  # Example

  

  ```rust

  use regex::RegexSet;

  

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

  use regex::RegexSet;

  

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

      .matches("foobar")

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

*Defined in [`regex-1.12.2/src/regexset/string.rs:459`](../../.source_1765633015/regex-1.12.2/src/regexset/string.rs#L459)*

A set of matches returned by a regex set.

Values of this type are constructed by `RegexSet::matches`.

#### Implementations

- <span id="setmatches-matched-any"></span>`fn matched_any(&self) -> bool`

  Whether this set contains any matches.

  

  # Example

  

  ```rust

  use regex::RegexSet;

  

  let set = RegexSet::new(&[

      r"[a-z]+@[a-z]+\.(com|org|net)",

      r"[a-z]+\.(com|org|net)",

  ]).unwrap();

  let matches = set.matches("foo@example.com");

  assert!(matches.matched_any());

  ```

- <span id="setmatches-matched-all"></span>`fn matched_all(&self) -> bool`

  Whether all patterns in this set matched.

  

  # Example

  

  ```rust

  use regex::RegexSet;

  

  let set = RegexSet::new(&[

      r"^foo",

      r"[a-z]+\.com",

  ]).unwrap();

  let matches = set.matches("foo.example.com");

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

  use regex::RegexSet;

  

  let set = RegexSet::new([

      r"[a-z]+@[a-z]+\.(com|org|net)",

      r"[a-z]+\.(com|org|net)",

  ]).unwrap();

  let matches = set.matches("example.com");

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

  use regex::RegexSet;

  

  let set = RegexSet::new([

      r"[a-z]+@[a-z]+\.(com|org|net)",

      r"[a-z]+\.(com|org|net)",

  ]).unwrap();

  let matches = set.matches("example.com");

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

  use regex::RegexSet;

  

  let set = RegexSet::new([

      r"[0-9]",

      r"[a-z]",

      r"[A-Z]",

      r"\p{Greek}",

  ]).unwrap();

  let hay = "βa1";

  let matches: Vec<_> = set.matches(hay).iter().collect();

  assert_eq!(matches, vec![0, 1, 3]);

  ```

  

  Note that `SetMatches` also implements the `IntoIterator` trait, so

  this method is not always needed. For example:

  

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

*Defined in [`regex-1.12.2/src/regexset/string.rs:652-655`](../../.source_1765633015/regex-1.12.2/src/regexset/string.rs#L652-L655)*

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

*Defined in [`regex-1.12.2/src/regexset/string.rs:698`](../../.source_1765633015/regex-1.12.2/src/regexset/string.rs#L698)*

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

## Enums

### `Error`

```rust
enum Error {
    Syntax(alloc::string::String),
    CompiledTooBig(usize),
}
```

*Defined in [`regex-1.12.2/src/error.rs:8-32`](../../.source_1765633015/regex-1.12.2/src/error.rs#L8-L32)*

An error that occurred during parsing or compiling a regular expression.

#### Variants

- **`Syntax`**

  A syntax error.

- **`CompiledTooBig`**

  The compiled program exceeded the set size
  limit. The argument is the size limit imposed by
  [`RegexBuilder::size_limit`](crate::RegexBuilder::size_limit). Even
  when not configured explicitly, it defaults to a reasonable limit.
  
  If you're getting this error, it occurred because your regex has been
  compiled to an intermediate state that is too big. It is important to
  note that exceeding this limit does _not_ mean the regex is too big to
  _work_, but rather, the regex is big enough that it may wind up being
  surprisingly slow when used in a search. In other words, this error is
  meant to be a practical heuristic for avoiding a performance footgun,
  and especially so for the case where the regex pattern is coming from
  an untrusted source.
  
  There are generally two ways to move forward if you hit this error.
  The first is to find some way to use a smaller regex. The second is to
  increase the size limit via `RegexBuilder::size_limit`. However, if
  your regex pattern is not from a trusted source, then neither of these
  approaches may be appropriate. Instead, you'll have to determine just
  how big of a regex you want to allow.

#### Implementations

- <span id="error-from-meta-build-error"></span>`fn from_meta_build_error(err: meta::BuildError) -> Error` — [`Error`](error/index.md#error)

#### Trait Implementations

##### `impl Any for Error`

- <span id="error-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Error`

- <span id="error-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Error`

- <span id="error-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](error/index.md#error)

##### `impl CloneToUninit for Error`

- <span id="error-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for Error`

- <span id="error-error-description"></span>`fn description(&self) -> &str`

##### `impl<T> From for Error`

- <span id="error-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Error`

- <span id="error-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](error/index.md#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToOwned for Error`

- <span id="error-toowned-type-owned"></span>`type Owned = T`

- <span id="error-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="error-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for Error`

- <span id="error-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="error-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Error`

- <span id="error-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="error-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Replacer`

```rust
trait Replacer { ... }
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2452-2498`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2452-L2498)*

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

- [`NoExpand`](#noexpand)
- [`ReplacerRef`](#replacerref)
- `&'a alloc::borrow::Cow<'a, str>`
- `&'a alloc::string::String`
- `&'a str`
- `F`
- `alloc::borrow::Cow<'a, str>`
- `alloc::string::String`

## Functions

### `escape`

```rust
fn escape(pattern: &str) -> alloc::string::String
```

*Defined in [`regex-1.12.2/src/lib.rs:1351-1353`](../../.source_1765633015/regex-1.12.2/src/lib.rs#L1351-L1353)*

Escapes all regular expression meta characters in `pattern`.

The string returned may be safely used as a literal in a regular
expression.

### `no_expansion`

```rust
fn no_expansion<T: AsRef<str>>(replacement: &T) -> Option<alloc::borrow::Cow<'_, str>>
```

*Defined in [`regex-1.12.2/src/regex/string.rs:2619-2625`](../../.source_1765633015/regex-1.12.2/src/regex/string.rs#L2619-L2625)*

Quickly checks the given replacement string for whether interpolation
should be done on it. It returns `None` if a `$` was found anywhere in the
given string, which suggests interpolation needs to be done. But if there's
no `$` anywhere, then interpolation definitely does not need to be done. In
that case, the given string is returned as a borrowed `Cow`.

This is meant to be used to implement the `Replacer::no_expansion` method
in its various trait impls.

