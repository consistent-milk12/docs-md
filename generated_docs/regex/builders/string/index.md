*[regex](../../index.md) / [builders](../index.md) / [string](index.md)*

---

# Module `string`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RegexBuilder`](#regexbuilder) | struct | A configurable builder for a [`Regex`]. |
| [`RegexSetBuilder`](#regexsetbuilder) | struct | A configurable builder for a [`RegexSet`]. |

## Structs

### `RegexBuilder`

```rust
struct RegexBuilder {
    builder: super::Builder,
}
```

*Defined in [`regex-1.12.2/src/builders.rs:212-214`](../../../../.source_1765521767/regex-1.12.2/src/builders.rs#L212-L214)*

A configurable builder for a [`Regex`](../../index.md).

This builder can be used to programmatically set flags such as `i`
(case insensitive) and `x` (for verbose mode). This builder can also be
used to configure things like the line terminator and a size limit on
the compiled regular expression.

#### Implementations

- <span id="regexbuilder-new"></span>`fn new(pattern: &str) -> RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

  Create a new builder with a default configuration for the given

  pattern.

  

  If the pattern is invalid or exceeds the configured size limits,

  then an error will be returned when `RegexBuilder::build` is

  called.

- <span id="regexbuilder-build"></span>`fn build(&self) -> Result<Regex, Error>` — [`Regex`](../../index.md#regex), [`Error`](../../error/index.md#error)

  Compiles the pattern given to `RegexBuilder::new` with the

  configuration set on this builder.

  

  If the pattern isn't a valid regex or if a configured size limit

  was exceeded, then an error is returned.

- <span id="regexbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

  UTF-8, and it is not permitted for a [`Regex`](../../index.md) to match invalid

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

- <span id="regexbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

- <span id="regexbuilder-clone"></span>`fn clone(&self) -> RegexBuilder` — [`RegexBuilder`](../../index.md#regexbuilder)

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

*Defined in [`regex-1.12.2/src/builders.rs:787-789`](../../../../.source_1765521767/regex-1.12.2/src/builders.rs#L787-L789)*

A configurable builder for a [`RegexSet`](../../index.md).

This builder can be used to programmatically set flags such as
`i` (case insensitive) and `x` (for verbose mode). This builder
can also be used to configure things like the line terminator
and a size limit on the compiled regular expression.

#### Implementations

- <span id="regexsetbuilder-new"></span>`fn new<I, S>(patterns: I) -> RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

  Create a new builder with a default configuration for the given

  patterns.

  

  If the patterns are invalid or exceed the configured size limits,

  then an error will be returned when `RegexSetBuilder::build` is

  called.

- <span id="regexsetbuilder-build"></span>`fn build(&self) -> Result<RegexSet, Error>` — [`RegexSet`](../../index.md#regexset), [`Error`](../../error/index.md#error)

  Compiles the patterns given to `RegexSetBuilder::new` with the

  configuration set on this builder.

  

  If the patterns aren't valid regexes or if a configured size limit

  was exceeded, then an error is returned.

- <span id="regexsetbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

  UTF-8, and it is not permitted for a [`RegexSet`](../../index.md) to match invalid

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

- <span id="regexsetbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-size-limit"></span>`fn size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-dfa-size-limit"></span>`fn dfa_size_limit(&mut self, bytes: usize) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

- <span id="regexsetbuilder-clone"></span>`fn clone(&self) -> RegexSetBuilder` — [`RegexSetBuilder`](../../index.md#regexsetbuilder)

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

