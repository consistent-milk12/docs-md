*[regex_syntax](../index.md) / [parser](index.md)*

---

# Module `parser`

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

- `fn new() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn build(self: &Self) -> Parser` — [`Parser`](#parser)

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn octal(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn utf8(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn case_insensitive(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn multi_line(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn dot_matches_new_line(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn crlf(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn line_terminator(self: &mut Self, byte: u8) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn swap_greed(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn unicode(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

#### Trait Implementations

##### `impl Clone for ParserBuilder`

- `fn clone(self: &Self) -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

##### `impl Debug for ParserBuilder`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for ParserBuilder`

- `fn default() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

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

A `Parser` can be configured in more detail via a [`ParserBuilder`](#parserbuilder).

#### Implementations

- `fn new() -> Parser` — [`Parser`](#parser)

- `fn parse(self: &mut Self, pattern: &str) -> Result<hir::Hir, Error>` — [`Hir`](../hir/index.md), [`Error`](../error/index.md)

#### Trait Implementations

##### `impl Clone for Parser`

- `fn clone(self: &Self) -> Parser` — [`Parser`](#parser)

##### `impl Debug for Parser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

## Functions

### `parse`

```rust
fn parse(pattern: &str) -> Result<hir::Hir, crate::Error>
```

A convenience routine for parsing a regex using default options.

This is equivalent to `Parser::new().parse(pattern)`.

If you need to set non-default options, then use a [`ParserBuilder`](#parserbuilder).

This routine returns an [`Hir`](hir::Hir) value. Namely, it automatically
parses the pattern as an [`Ast`](ast::Ast) and then invokes the translator
to convert the `Ast` into an `Hir`. If you need access to the `Ast`, then
you should use a [`ast::parse::Parser`](../ast/parse/index.md).

