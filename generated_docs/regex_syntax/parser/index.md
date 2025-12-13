*[regex_syntax](../index.md) / [parser](index.md)*

---

# Module `parser`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParserBuilder`](#parserbuilder) | struct | A builder for a regular expression parser. |
| [`Parser`](#parser) | struct | A convenience parser for regular expressions. |
| [`parse`](#parse) | fn | A convenience routine for parsing a regex using default options. |

## Structs

### `ParserBuilder`

```rust
struct ParserBuilder {
    ast: ast::parse::ParserBuilder,
    hir: hir::translate::TranslatorBuilder,
}
```

*Defined in [`regex-syntax-0.8.8/src/parser.rs:25-28`](../../../.source_1765633015/regex-syntax-0.8.8/src/parser.rs#L25-L28)*

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

This type combines the builder options for both the [AST
`ParserBuilder`](ast::parse::ParserBuilder) and the [HIR
`TranslatorBuilder`](hir::translate::TranslatorBuilder).

#### Implementations

- <span id="parserbuilder-new"></span>`fn new() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Create a new parser builder with a default configuration.

- <span id="parserbuilder-build"></span>`fn build(&self) -> Parser` — [`Parser`](#parser)

  Build a parser from this configuration with the given pattern.

- <span id="parserbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Set the nesting limit for this parser.

  

  The nesting limit controls how deep the abstract syntax tree is allowed

  to be. If the AST exceeds the given limit (e.g., with too many nested

  groups), then an error is returned by the parser.

  

  The purpose of this limit is to act as a heuristic to prevent stack

  overflow for consumers that do structural induction on an `Ast` using

  explicit recursion. While this crate never does this (instead using

  constant stack space and moving the call stack to the heap), other

  crates may.

  

  This limit is not checked until the entire Ast is parsed. Therefore,

  if callers want to put a limit on the amount of heap space used, then

  they should impose a limit on the length, in bytes, of the concrete

  pattern string. In particular, this is viable since this parser

  implementation will limit itself to heap space proportional to the

  length of the pattern string.

  

  Note that a nest limit of `0` will return a nest limit error for most

  patterns but not all. For example, a nest limit of `0` permits `a` but

  not `ab`, since `ab` requires a concatenation, which results in a nest

  depth of `1`. In general, a nest limit is not something that manifests

  in an obvious way in the concrete syntax, therefore, it should not be

  used in a granular way.

- <span id="parserbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Whether to support octal syntax or not.

  

  Octal syntax is a little-known way of uttering Unicode codepoints in

  a regular expression. For example, `a`, `\x61`, `\u0061` and

  `\141` are all equivalent regular expressions, where the last example

  shows octal syntax.

  

  While supporting octal syntax isn't in and of itself a problem, it does

  make good error messages harder. That is, in PCRE based regex engines,

  syntax like `\0` invokes a backreference, which is explicitly

  unsupported in Rust's regex engine. However, many users expect it to

  be supported. Therefore, when octal support is disabled, the error

  message will explicitly mention that backreferences aren't supported.

  

  Octal syntax is disabled by default.

- <span id="parserbuilder-utf8"></span>`fn utf8(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  When disabled, translation will permit the construction of a regular

  expression that may match invalid UTF-8.

  

  When enabled (the default), the translator is guaranteed to produce an

  expression that, for non-empty matches, will only ever produce spans

  that are entirely valid UTF-8 (otherwise, the translator will return an

  error).

  

  Perhaps surprisingly, when UTF-8 is enabled, an empty regex or even

  a negated ASCII word boundary (uttered as `(?-u:\B)` in the concrete

  syntax) will be allowed even though they can produce matches that split

  a UTF-8 encoded codepoint. This only applies to zero-width or "empty"

  matches, and it is expected that the regex engine itself must handle

  these cases if necessary (perhaps by suppressing any zero-width matches

  that split a codepoint).

- <span id="parserbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable verbose mode in the regular expression.

  

  When enabled, verbose mode permits insignificant whitespace in many

  places in the regular expression, as well as comments. Comments are

  started using `#` and continue until the end of the line.

  

  By default, this is disabled. It may be selectively enabled in the

  regular expression by using the `x` flag regardless of this setting.

- <span id="parserbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable or disable the case insensitive flag by default.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `i` flag.

- <span id="parserbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable or disable the multi-line matching flag by default.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `m` flag.

- <span id="parserbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable or disable the "dot matches any character" flag by default.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `s` flag.

- <span id="parserbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable or disable the CRLF mode flag by default.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `R` flag.

  

  When CRLF mode is enabled, the following happens:

  

  * Unless `dot_matches_new_line` is enabled, `.` will match any character

  except for `\r` and `\n`.

  * When `multi_line` mode is enabled, `^` and `$` will treat `\r\n`,

  `\r` and `\n` as line terminators. And in particular, neither will

  match between a `\r` and a `\n`.

- <span id="parserbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

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

- <span id="parserbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable or disable the "swap greed" flag by default.

  

  By default this is disabled. It may alternatively be selectively

  enabled in the regular expression itself via the `U` flag.

- <span id="parserbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable or disable the Unicode flag (`u`) by default.

  

  By default this is **enabled**. It may alternatively be selectively

  disabled in the regular expression itself via the `u` flag.

  

  Note that unless `utf8` is disabled (it's enabled by default), a

  regular expression will fail to parse if Unicode mode is disabled and a

  sub-expression could possibly match invalid UTF-8.

#### Trait Implementations

##### `impl Any for ParserBuilder`

- <span id="parserbuilder-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParserBuilder`

- <span id="parserbuilder-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParserBuilder`

- <span id="parserbuilder-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ParserBuilder`

- <span id="parserbuilder-clone"></span>`fn clone(&self) -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

##### `impl CloneToUninit for ParserBuilder`

- <span id="parserbuilder-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ParserBuilder`

- <span id="parserbuilder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParserBuilder`

- <span id="parserbuilder-default"></span>`fn default() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

##### `impl<T> From for ParserBuilder`

- <span id="parserbuilder-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParserBuilder`

- <span id="parserbuilder-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ParserBuilder`

- <span id="parserbuilder-toowned-type-owned"></span>`type Owned = T`

- <span id="parserbuilder-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parserbuilder-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ParserBuilder`

- <span id="parserbuilder-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parserbuilder-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParserBuilder`

- <span id="parserbuilder-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parserbuilder-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `Parser`

```rust
struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
```

*Defined in [`regex-syntax-0.8.8/src/parser.rs:230-233`](../../../.source_1765633015/regex-syntax-0.8.8/src/parser.rs#L230-L233)*

A convenience parser for regular expressions.

This parser takes as input a regular expression pattern string (the
"concrete syntax") and returns a high-level intermediate representation
(the HIR) suitable for most types of analysis. In particular, this parser
hides the intermediate state of producing an AST (the "abstract syntax").
The AST is itself far more complex than the HIR, so this parser serves as a
convenience for never having to deal with it at all.

If callers have more fine grained use cases that need an AST, then please
see the [`ast::parse`](../ast/parse/index.md) module.

A `Parser` can be configured in more detail via a [`ParserBuilder`](#parserbuilder).

#### Implementations

- <span id="parser-new"></span>`fn new() -> Parser` — [`Parser`](#parser)

  Create a new parser with a default configuration.

  

  The parser can be run with `parse` method. The parse method returns

  a high level intermediate representation of the given regular

  expression.

  

  To set configuration options on the parser, use [`ParserBuilder`](#parserbuilder).

- <span id="parser-parse"></span>`fn parse(&mut self, pattern: &str) -> Result<hir::Hir, Error>` — [`Hir`](../hir/index.md#hir), [`Error`](../error/index.md#error)

  Parse the regular expression into a high level intermediate

  representation.

#### Trait Implementations

##### `impl Any for Parser`

- <span id="parser-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Parser`

- <span id="parser-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Parser`

- <span id="parser-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Parser`

- <span id="parser-clone"></span>`fn clone(&self) -> Parser` — [`Parser`](#parser)

##### `impl CloneToUninit for Parser`

- <span id="parser-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Parser`

- <span id="parser-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for Parser`

- <span id="parser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parser`

- <span id="parser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for Parser`

- <span id="parser-toowned-type-owned"></span>`type Owned = T`

- <span id="parser-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parser-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Parser`

- <span id="parser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parser`

- <span id="parser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<hir::Hir, crate::Error>
```

*Defined in [`regex-syntax-0.8.8/src/parser.rs:13-15`](../../../.source_1765633015/regex-syntax-0.8.8/src/parser.rs#L13-L15)*

A convenience routine for parsing a regex using default options.

This is equivalent to `Parser::new().parse(pattern)`.

If you need to set non-default options, then use a [`ParserBuilder`](#parserbuilder).

This routine returns an [`Hir`](hir::Hir) value. Namely, it automatically
parses the pattern as an [`Ast`](ast::Ast) and then invokes the translator
to convert the `Ast` into an `Hir`. If you need access to the `Ast`, then
you should use a [`ast::parse::Parser`](../ast/parse/index.md).

