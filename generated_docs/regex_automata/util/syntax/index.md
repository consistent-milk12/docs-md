*[regex_automata](../../index.md) / [util](../index.md) / [syntax](index.md)*

---

# Module `syntax`

Utilities for dealing with the syntax of a regular expression.

This module currently only exposes a [`Config`](#config) type that
itself represents a wrapper around the configuration for a
[`regex-syntax::ParserBuilder`](regex_syntax::ParserBuilder). The purpose of
this wrapper is to make configuring syntax options very similar to how other
configuration is done throughout this crate. Namely, instead of duplicating
syntax options across every builder (of which there are many), we instead
create small config objects like this one that can be passed around and
composed.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Config`](#config) | struct | A common set of configuration options that apply to the syntax of a regex. |
| [`parse`](#parse) | fn | A convenience routine for parsing a pattern into an HIR value with the default configuration. |
| [`parse_many`](#parse-many) | fn | A convenience routine for parsing many patterns into HIR value with the default configuration. |
| [`parse_with`](#parse-with) | fn | A convenience routine for parsing a pattern into an HIR value using a `Config`. |
| [`parse_many_with`](#parse-many-with) | fn | A convenience routine for parsing many patterns into HIR values using a `Config`. |

## Structs

### `Config`

```rust
struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}
```

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:145-157`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/syntax.rs#L145-L157)*

A common set of configuration options that apply to the syntax of a regex.

This represents a group of configuration options that specifically apply
to how the concrete syntax of a regular expression is interpreted. In
particular, they are generally forwarded to the
[`ParserBuilder`](https://docs.rs/regex-syntax/*/regex_syntax/struct.ParserBuilder.html)
in the
[`regex-syntax`](https://docs.rs/regex-syntax)
crate when building a regex from its concrete syntax directly.

These options are defined as a group since they apply to every regex engine
in this crate. Instead of re-defining them on every engine's builder, they
are instead provided here as one cohesive unit.

#### Implementations

- <span id="config-new"></span>`fn new() -> Config` — [`Config`](#config)

  Return a new default syntax configuration.

- <span id="config-case-insensitive"></span>`fn case_insensitive(self, yes: bool) -> Config` — [`Config`](#config)

  Enable or disable the case insensitive flag by default.

  

  When Unicode mode is enabled, case insensitivity is Unicode-aware.

  Specifically, it will apply the "simple" case folding rules as

  specified by Unicode.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `i` flag.

- <span id="config-multi-line"></span>`fn multi_line(self, yes: bool) -> Config` — [`Config`](#config)

  Enable or disable the multi-line matching flag by default.

  

  When this is enabled, the `^` and `$` look-around assertions will

  match immediately after and immediately before a new line character,

  respectively. Note that the `\A` and `\z` look-around assertions are

  unaffected by this setting and always correspond to matching at the

  beginning and end of the input.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `m` flag.

- <span id="config-dot-matches-new-line"></span>`fn dot_matches_new_line(self, yes: bool) -> Config` — [`Config`](#config)

  Enable or disable the "dot matches any character" flag by default.

  

  When this is enabled, `.` will match any character. When it's disabled,

  then `.` will match any character except for a new line character.

  

  Note that `.` is impacted by whether the "unicode" setting is enabled

  or not. When Unicode is enabled (the default), `.` will match any UTF-8

  encoding of any Unicode scalar value (sans a new line, depending on

  whether this "dot matches new line" option is enabled). When Unicode

  mode is disabled, `.` will match any byte instead. Because of this,

  when Unicode mode is disabled, `.` can only be used when the "allow

  invalid UTF-8" option is enabled, since `.` could otherwise match

  invalid UTF-8.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `s` flag.

- <span id="config-crlf"></span>`fn crlf(self, yes: bool) -> Config` — [`Config`](#config)

  Enable or disable the "CRLF mode" flag by default.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `R` flag.

  

  When CRLF mode is enabled, the following happens:

  

  * Unless `dot_matches_new_line` is enabled, `.` will match any character

  except for `\r` and `\n`.

  * When `multi_line` mode is enabled, `^` and `$` will treat `\r\n`,

  `\r` and `\n` as line terminators. And in particular, neither will

  match between a `\r` and a `\n`.

- <span id="config-line-terminator"></span>`fn line_terminator(self, byte: u8) -> Config` — [`Config`](#config)

  Sets the line terminator for use with `(?u-s:.)` and `(?-us:.)`.

  

  Namely, instead of `.` (by default) matching everything except for `\n`,

  this will cause `.` to match everything except for the byte given.

  

  If `.` is used in a context where Unicode mode is enabled and this byte

  isn't ASCII, then an error will be returned. When Unicode mode is

  disabled, then any byte is permitted, but will return an error if UTF-8

  mode is enabled and it is a non-ASCII byte.

  

  In short, any ASCII value for a line terminator is always okay. But a

  non-ASCII byte might result in an error depending on whether Unicode

  mode or UTF-8 mode are enabled.

  

  Note that if `R` mode is enabled then it always takes precedence and

  the line terminator will be treated as `\r` and `\n` simultaneously.

  

  Note also that this *doesn't* impact the look-around assertions

  `(?m:^)` and `(?m:$)`. That's usually controlled by additional

  configuration in the regex engine itself.

- <span id="config-swap-greed"></span>`fn swap_greed(self, yes: bool) -> Config` — [`Config`](#config)

  Enable or disable the "swap greed" flag by default.

  

  When this is enabled, `.*` (for example) will become ungreedy and `.*?`

  will become greedy.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `U` flag.

- <span id="config-ignore-whitespace"></span>`fn ignore_whitespace(self, yes: bool) -> Config` — [`Config`](#config)

  Enable verbose mode in the regular expression.

  

  When enabled, verbose mode permits insignificant whitespace in many

  places in the regular expression, as well as comments. Comments are

  started using `#` and continue until the end of the line.

  

  By default, this is disabled. It may be selectively enabled in the

  regular expression by using the `x` flag regardless of this setting.

- <span id="config-unicode"></span>`fn unicode(self, yes: bool) -> Config` — [`Config`](#config)

  Enable or disable the Unicode flag (`u`) by default.

  

  By default this is **enabled**. It may alternatively be selectively

  disabled in the regular expression itself via the `u` flag.

  

  Note that unless "allow invalid UTF-8" is enabled (it's disabled by

  default), a regular expression will fail to parse if Unicode mode is

  disabled and a sub-expression could possibly match invalid UTF-8.

  

  **WARNING**: Unicode mode can greatly increase the size of the compiled

  DFA, which can noticeably impact both memory usage and compilation

  time. This is especially noticeable if your regex contains character

  classes like `\w` that are impacted by whether Unicode is enabled or

  not. If Unicode is not necessary, you are encouraged to disable it.

- <span id="config-utf8"></span>`fn utf8(self, yes: bool) -> Config` — [`Config`](#config)

  When disabled, the builder will permit the construction of a regular

  expression that may match invalid UTF-8.

  

  For example, when `Config::unicode` is disabled, then

  expressions like `[^a]` may match invalid UTF-8 since they can match

  any single byte that is not `a`. By default, these sub-expressions

  are disallowed to avoid returning offsets that split a UTF-8

  encoded codepoint. However, in cases where matching at arbitrary

  locations is desired, this option can be disabled to permit all such

  sub-expressions.

  

  When enabled (the default), the builder is guaranteed to produce a

  regex that will only ever match valid UTF-8 (otherwise, the builder

  will return an error).

- <span id="config-nest-limit"></span>`fn nest_limit(self, limit: u32) -> Config` — [`Config`](#config)

  Set the nesting limit used for the regular expression parser.

  

  The nesting limit controls how deep the abstract syntax tree is allowed

  to be. If the AST exceeds the given limit (e.g., with too many nested

  groups), then an error is returned by the parser.

  

  The purpose of this limit is to act as a heuristic to prevent stack

  overflow when building a finite automaton from a regular expression's

  abstract syntax tree. In particular, construction currently uses

  recursion. In the future, the implementation may stop using recursion

  and this option will no longer be necessary.

  

  This limit is not checked until the entire AST is parsed. Therefore,

  if callers want to put a limit on the amount of heap space used, then

  they should impose a limit on the length, in bytes, of the concrete

  pattern string. In particular, this is viable since the parser will

  limit itself to heap space proportional to the length of the pattern

  string.

  

  Note that a nest limit of `0` will return a nest limit error for most

  patterns but not all. For example, a nest limit of `0` permits `a` but

  not `ab`, since `ab` requires a concatenation AST item, which results

  in a nest depth of `1`. In general, a nest limit is not something that

  manifests in an obvious way in the concrete syntax, therefore, it

  should not be used in a granular way.

- <span id="config-octal"></span>`fn octal(self, yes: bool) -> Config` — [`Config`](#config)

  Whether to support octal syntax or not.

  

  Octal syntax is a little-known way of uttering Unicode codepoints in

  a regular expression. For example, `a`, `\x61`, `\u0061` and

  `\141` are all equivalent regular expressions, where the last example

  shows octal syntax.

  

  While supporting octal syntax isn't in and of itself a problem, it does

  make good error messages harder. That is, in PCRE based regex engines,

  syntax like `\1` invokes a backreference, which is explicitly

  unsupported in Rust's regex engine. However, many users expect it to

  be supported. Therefore, when octal support is disabled, the error

  message will explicitly mention that backreferences aren't supported.

  

  Octal syntax is disabled by default.

- <span id="config-get-unicode"></span>`fn get_unicode(&self) -> bool`

  Returns whether "unicode" mode is enabled.

- <span id="config-get-case-insensitive"></span>`fn get_case_insensitive(&self) -> bool`

  Returns whether "case insensitive" mode is enabled.

- <span id="config-get-multi-line"></span>`fn get_multi_line(&self) -> bool`

  Returns whether "multi line" mode is enabled.

- <span id="config-get-dot-matches-new-line"></span>`fn get_dot_matches_new_line(&self) -> bool`

  Returns whether "dot matches new line" mode is enabled.

- <span id="config-get-crlf"></span>`fn get_crlf(&self) -> bool`

  Returns whether "CRLF" mode is enabled.

- <span id="config-get-line-terminator"></span>`fn get_line_terminator(&self) -> u8`

  Returns the line terminator in this syntax configuration.

- <span id="config-get-swap-greed"></span>`fn get_swap_greed(&self) -> bool`

  Returns whether "swap greed" mode is enabled.

- <span id="config-get-ignore-whitespace"></span>`fn get_ignore_whitespace(&self) -> bool`

  Returns whether "ignore whitespace" mode is enabled.

- <span id="config-get-utf8"></span>`fn get_utf8(&self) -> bool`

  Returns whether UTF-8 mode is enabled.

- <span id="config-get-nest-limit"></span>`fn get_nest_limit(&self) -> u32`

  Returns the "nest limit" setting.

- <span id="config-get-octal"></span>`fn get_octal(&self) -> bool`

  Returns whether "octal" mode is enabled.

- <span id="config-apply"></span>`fn apply(&self, builder: &mut ParserBuilder)`

  Applies this configuration to the given parser.

- <span id="config-apply-ast"></span>`fn apply_ast(&self, builder: &mut ast::parse::ParserBuilder)`

  Applies this configuration to the given AST parser.

- <span id="config-apply-hir"></span>`fn apply_hir(&self, builder: &mut hir::translate::TranslatorBuilder)`

  Applies this configuration to the given AST-to-HIR translator.

#### Trait Implementations

##### `impl Any for Config`

- <span id="config-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Config`

- <span id="config-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Config`

- <span id="config-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Config`

- <span id="config-clone"></span>`fn clone(&self) -> Config` — [`Config`](#config)

##### `impl CloneToUninit for Config`

- <span id="config-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Copy for Config`

##### `impl Debug for Config`

- <span id="config-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Config`

- <span id="config-default"></span>`fn default() -> Config` — [`Config`](#config)

##### `impl<T> From for Config`

- <span id="config-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Config`

- <span id="config-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Config`

- <span id="config-toowned-type-owned"></span>`type Owned = T`

- <span id="config-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="config-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Config`

- <span id="config-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="config-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Config`

- <span id="config-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="config-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:37-39`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/syntax.rs#L37-L39)*

A convenience routine for parsing a pattern into an HIR value with the
default configuration.

# Example

This shows how to parse a pattern into an HIR value:

```rust
use regex_automata::util::syntax;

let hir = syntax::parse(r"([a-z]+)|([0-9]+)")?;
assert_eq!(Some(1), hir.properties().static_explicit_captures_len());

Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_many`

```rust
fn parse_many<P: AsRef<str>>(patterns: &[P]) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:63-65`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/syntax.rs#L63-L65)*

A convenience routine for parsing many patterns into HIR value with the
default configuration.

# Example

This shows how to parse many patterns into an corresponding HIR values:

```rust
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let hirs = syntax::parse_many(&[
    r"([a-z]+)|([0-9]+)",
    r"foo(A-Z]+)bar",
])?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert_eq!(Some(1), props.static_explicit_captures_len());

Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_with`

```rust
fn parse_with(pattern: &str, config: &Config) -> Result<regex_syntax::hir::Hir, regex_syntax::Error>
```

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:86-90`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/syntax.rs#L86-L90)*

A convenience routine for parsing a pattern into an HIR value using a
`Config`.

# Example

This shows how to parse a pattern into an HIR value with a non-default
configuration:

```rust
use regex_automata::util::syntax;

let hir = syntax::parse_with(
    r"^[a-z]+$",
    &syntax::Config::new().multi_line(true).crlf(true),
)?;
assert!(hir.properties().look_set().contains_anchor_crlf());

Ok::<(), Box<dyn std::error::Error>>(())
```

### `parse_many_with`

```rust
fn parse_many_with<P: AsRef<str>>(patterns: &[P], config: &Config) -> Result<alloc::vec::Vec<regex_syntax::hir::Hir>, regex_syntax::Error>
```

*Defined in [`regex-automata-0.4.13/src/util/syntax.rs:118-129`](../../../../.source_1765633015/regex-automata-0.4.13/src/util/syntax.rs#L118-L129)*

A convenience routine for parsing many patterns into HIR values using a
`Config`.

# Example

This shows how to parse many patterns into an corresponding HIR values
with a non-default configuration:

```rust
use {
    regex_automata::util::syntax,
    regex_syntax::hir::Properties,
};

let patterns = &[
    r"([a-z]+)|([0-9]+)",
    r"\W",
    r"foo(A-Z]+)bar",
];
let config = syntax::Config::new().unicode(false).utf8(false);
let hirs = syntax::parse_many_with(patterns, &config)?;
let props = Properties::union(hirs.iter().map(|h| h.properties()));
assert!(!props.is_utf8());

Ok::<(), Box<dyn std::error::Error>>(())
```

