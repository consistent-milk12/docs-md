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

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

This type combines the builder options for both the [AST
`ParserBuilder`](ast::parse::ParserBuilder) and the [HIR
`TranslatorBuilder`](hir::translate::TranslatorBuilder).

#### Implementations

- <span id="parserbuilder-new"></span>`fn new() -> ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-build"></span>`fn build(&self) -> Parser` — [`Parser`](../index.md)

- <span id="parserbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-utf8"></span>`fn utf8(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-case-insensitive"></span>`fn case_insensitive(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-multi-line"></span>`fn multi_line(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-dot-matches-new-line"></span>`fn dot_matches_new_line(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-crlf"></span>`fn crlf(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-line-terminator"></span>`fn line_terminator(&mut self, byte: u8) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-swap-greed"></span>`fn swap_greed(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

- <span id="parserbuilder-unicode"></span>`fn unicode(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../index.md)

#### Trait Implementations

##### `impl Clone for ParserBuilder`

- <span id="parserbuilder-clone"></span>`fn clone(&self) -> ParserBuilder` — [`ParserBuilder`](../index.md)

##### `impl Debug for ParserBuilder`

- <span id="parserbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParserBuilder`

- <span id="parserbuilder-default"></span>`fn default() -> ParserBuilder` — [`ParserBuilder`](../index.md)

### `Parser`

```rust
struct Parser {
    ast: ast::parse::Parser,
    hir: hir::translate::Translator,
}
```

A convenience parser for regular expressions.

This parser takes as input a regular expression pattern string (the
"concrete syntax") and returns a high-level intermediate representation
(the HIR) suitable for most types of analysis. In particular, this parser
hides the intermediate state of producing an AST (the "abstract syntax").
The AST is itself far more complex than the HIR, so this parser serves as a
convenience for never having to deal with it at all.

If callers have more fine grained use cases that need an AST, then please
see the [`ast::parse`](../ast/parse/index.md) module.

A `Parser` can be configured in more detail via a [`ParserBuilder`](../index.md).

#### Implementations

- <span id="parser-new"></span>`fn new() -> Parser` — [`Parser`](../index.md)

- <span id="parser-parse"></span>`fn parse(&mut self, pattern: &str) -> Result<hir::Hir, Error>` — [`Hir`](../hir/index.md), [`Error`](../index.md)

#### Trait Implementations

##### `impl Clone for Parser`

- <span id="parser-clone"></span>`fn clone(&self) -> Parser` — [`Parser`](../index.md)

##### `impl Debug for Parser`

- <span id="parser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<hir::Hir, crate::Error>
```

A convenience routine for parsing a regex using default options.

This is equivalent to `Parser::new().parse(pattern)`.

If you need to set non-default options, then use a [`ParserBuilder`](../index.md).

This routine returns an [`Hir`](hir::Hir) value. Namely, it automatically
parses the pattern as an [`Ast`](ast::Ast) and then invokes the translator
to convert the `Ast` into an `Hir`. If you need access to the `Ast`, then
you should use a [`ast::parse::Parser`](../ast/parse/index.md).

