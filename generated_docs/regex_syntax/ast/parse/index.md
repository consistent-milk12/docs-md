*[regex_syntax](../../index.md) / [ast](../index.md) / [parse](index.md)*

---

# Module `parse`

This module provides a regular expression parser.

## Contents

- [Structs](#structs)
  - [`ParserBuilder`](#parserbuilder)
  - [`Parser`](#parser)
  - [`ParserI`](#parseri)
  - [`NestLimiter`](#nestlimiter)
- [Enums](#enums)
  - [`Primitive`](#primitive)
  - [`GroupState`](#groupstate)
  - [`ClassState`](#classstate)
- [Functions](#functions)
  - [`is_hex`](#is_hex)
  - [`is_capture_char`](#is_capture_char)
  - [`specialize_err`](#specialize_err)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParserBuilder`](#parserbuilder) | struct | A builder for a regular expression parser. |
| [`Parser`](#parser) | struct | A regular expression parser. |
| [`ParserI`](#parseri) | struct | ParserI is the internal parser implementation. |
| [`NestLimiter`](#nestlimiter) | struct | A type that traverses a fully parsed Ast and checks whether its depth |
| [`Primitive`](#primitive) | enum | A primitive is an expression with no sub-expressions. |
| [`GroupState`](#groupstate) | enum | GroupState represents a single stack frame while parsing nested groups |
| [`ClassState`](#classstate) | enum | ClassState represents a single stack frame while parsing character classes. |
| [`is_hex`](#is_hex) | fn | Returns true if the given character is a hexadecimal digit. |
| [`is_capture_char`](#is_capture_char) | fn | Returns true if the given character is a valid in a capture group name. |
| [`specialize_err`](#specialize_err) | fn | When the result is an error, transforms the ast::ErrorKind from the source |
| [`Result`](#result) | type |  |

## Structs

### `ParserBuilder`

```rust
struct ParserBuilder {
    ignore_whitespace: bool,
    nest_limit: u32,
    octal: bool,
    empty_min_range: bool,
}
```

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

#### Implementations

- <span id="parserbuilder-new"></span>`fn new() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- <span id="parserbuilder-build"></span>`fn build(&self) -> Parser` — [`Parser`](#parser)

- <span id="parserbuilder-nest-limit"></span>`fn nest_limit(&mut self, limit: u32) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- <span id="parserbuilder-octal"></span>`fn octal(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- <span id="parserbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- <span id="parserbuilder-empty-min-range"></span>`fn empty_min_range(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

#### Trait Implementations

##### `impl Clone for ParserBuilder`

- <span id="parserbuilder-clone"></span>`fn clone(&self) -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

##### `impl Debug for ParserBuilder`

- <span id="parserbuilder-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ParserBuilder`

- <span id="parserbuilder-default"></span>`fn default() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

### `Parser`

```rust
struct Parser {
    pos: core::cell::Cell<crate::ast::Position>,
    capture_index: core::cell::Cell<u32>,
    nest_limit: u32,
    octal: bool,
    initial_ignore_whitespace: bool,
    empty_min_range: bool,
    ignore_whitespace: core::cell::Cell<bool>,
    comments: core::cell::RefCell<alloc::vec::Vec<ast::Comment>>,
    stack_group: core::cell::RefCell<alloc::vec::Vec<GroupState>>,
    stack_class: core::cell::RefCell<alloc::vec::Vec<ClassState>>,
    capture_names: core::cell::RefCell<alloc::vec::Vec<ast::CaptureName>>,
    scratch: core::cell::RefCell<alloc::string::String>,
}
```

A regular expression parser.

This parses a string representation of a regular expression into an
abstract syntax tree. The size of the tree is proportional to the length
of the regular expression pattern.

A `Parser` can be configured in more detail via a [`ParserBuilder`](#parserbuilder).

#### Fields

- **`pos`**: `core::cell::Cell<crate::ast::Position>`

  The current position of the parser.

- **`capture_index`**: `core::cell::Cell<u32>`

  The current capture index.

- **`nest_limit`**: `u32`

  The maximum number of open parens/brackets allowed. If the parser
  exceeds this number, then an error is returned.

- **`octal`**: `bool`

  Whether to support octal syntax or not. When `false`, the parser will
  return an error helpfully pointing out that backreferences are not
  supported.

- **`initial_ignore_whitespace`**: `bool`

  The initial setting for `ignore_whitespace` as provided by
  `ParserBuilder`. It is used when resetting the parser's state.

- **`empty_min_range`**: `bool`

  Whether the parser supports `{,n}` repetitions as an equivalent to
  `{0,n}.`

- **`ignore_whitespace`**: `core::cell::Cell<bool>`

  Whether whitespace should be ignored. When enabled, comments are
  also permitted.

- **`comments`**: `core::cell::RefCell<alloc::vec::Vec<ast::Comment>>`

  A list of comments, in order of appearance.

- **`stack_group`**: `core::cell::RefCell<alloc::vec::Vec<GroupState>>`

  A stack of grouped sub-expressions, including alternations.

- **`stack_class`**: `core::cell::RefCell<alloc::vec::Vec<ClassState>>`

  A stack of nested character classes. This is only non-empty when
  parsing a class.

- **`capture_names`**: `core::cell::RefCell<alloc::vec::Vec<ast::CaptureName>>`

  A sorted sequence of capture names. This is used to detect duplicate
  capture names and report an error if one is detected.

- **`scratch`**: `core::cell::RefCell<alloc::string::String>`

  A scratch buffer used in various places. Mostly this is used to
  accumulate relevant characters from parts of a pattern.

#### Implementations

- <span id="parser-new"></span>`fn new() -> Parser` — [`Parser`](#parser)

- <span id="parser-parse"></span>`fn parse(&mut self, pattern: &str) -> core::result::Result<Ast, ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- <span id="parser-parse-with-comments"></span>`fn parse_with_comments(&mut self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error>` — [`WithComments`](../index.md), [`Error`](../index.md)

- <span id="parser-reset"></span>`fn reset(&self)`

#### Trait Implementations

##### `impl Clone for Parser`

- <span id="parser-clone"></span>`fn clone(&self) -> Parser` — [`Parser`](#parser)

##### `impl Debug for Parser`

- <span id="parser-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ParserI<'s, P>`

```rust
struct ParserI<'s, P> {
    parser: P,
    pattern: &'s str,
}
```

ParserI is the internal parser implementation.

We use this separate type so that we can carry the provided pattern string
along with us. In particular, a `Parser` internal state is not tied to any
one pattern, but `ParserI` is.

This type also lets us use `ParserI<&Parser>` in production code while
retaining the convenience of `ParserI<Parser>` for tests, which sometimes
work against the internal interface of the parser.

#### Fields

- **`parser`**: `P`

  The parser state/configuration.

- **`pattern`**: `&'s str`

  The full regular expression provided by the user.

#### Implementations

- <span id="parseri-parse"></span>`fn parse(&self) -> core::result::Result<Ast, ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-with-comments"></span>`fn parse_with_comments(&self) -> core::result::Result<ast::WithComments, ast::Error>` — [`WithComments`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-uncounted-repetition"></span>`fn parse_uncounted_repetition(&self, concat: ast::Concat, kind: ast::RepetitionKind) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md), [`RepetitionKind`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-counted-repetition"></span>`fn parse_counted_repetition(&self, concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-group"></span>`fn parse_group(&self) -> core::result::Result<Either<ast::SetFlags, ast::Group>, ast::Error>` — [`Either`](../../either/index.md), [`SetFlags`](../index.md), [`Group`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-capture-name"></span>`fn parse_capture_name(&self, capture_index: u32) -> core::result::Result<ast::CaptureName, ast::Error>` — [`CaptureName`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-flags"></span>`fn parse_flags(&self) -> core::result::Result<ast::Flags, ast::Error>` — [`Flags`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-flag"></span>`fn parse_flag(&self) -> core::result::Result<ast::Flag, ast::Error>` — [`Flag`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-primitive"></span>`fn parse_primitive(&self) -> core::result::Result<Primitive, ast::Error>` — [`Primitive`](#primitive), [`Error`](../index.md)

- <span id="parseri-parse-escape"></span>`fn parse_escape(&self) -> core::result::Result<Primitive, ast::Error>` — [`Primitive`](#primitive), [`Error`](../index.md)

- <span id="parseri-maybe-parse-special-word-boundary"></span>`fn maybe_parse_special_word_boundary(&self, wb_start: Position) -> core::result::Result<Option<ast::AssertionKind>, ast::Error>` — [`Position`](../index.md), [`AssertionKind`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-octal"></span>`fn parse_octal(&self) -> ast::Literal` — [`Literal`](../index.md)

- <span id="parseri-parse-hex"></span>`fn parse_hex(&self) -> core::result::Result<ast::Literal, ast::Error>` — [`Literal`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-hex-digits"></span>`fn parse_hex_digits(&self, kind: ast::HexLiteralKind) -> core::result::Result<ast::Literal, ast::Error>` — [`HexLiteralKind`](../index.md), [`Literal`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-hex-brace"></span>`fn parse_hex_brace(&self, kind: ast::HexLiteralKind) -> core::result::Result<ast::Literal, ast::Error>` — [`HexLiteralKind`](../index.md), [`Literal`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-decimal"></span>`fn parse_decimal(&self) -> core::result::Result<u32, ast::Error>` — [`Error`](../index.md)

- <span id="parseri-parse-set-class"></span>`fn parse_set_class(&self) -> core::result::Result<ast::ClassBracketed, ast::Error>` — [`ClassBracketed`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-set-class-range"></span>`fn parse_set_class_range(&self) -> core::result::Result<ast::ClassSetItem, ast::Error>` — [`ClassSetItem`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-set-class-item"></span>`fn parse_set_class_item(&self) -> core::result::Result<Primitive, ast::Error>` — [`Primitive`](#primitive), [`Error`](../index.md)

- <span id="parseri-parse-set-class-open"></span>`fn parse_set_class_open(&self) -> core::result::Result<(ast::ClassBracketed, ast::ClassSetUnion), ast::Error>` — [`ClassBracketed`](../index.md), [`ClassSetUnion`](../index.md), [`Error`](../index.md)

- <span id="parseri-maybe-parse-ascii-class"></span>`fn maybe_parse_ascii_class(&self) -> Option<ast::ClassAscii>` — [`ClassAscii`](../index.md)

- <span id="parseri-parse-unicode-class"></span>`fn parse_unicode_class(&self) -> core::result::Result<ast::ClassUnicode, ast::Error>` — [`ClassUnicode`](../index.md), [`Error`](../index.md)

- <span id="parseri-parse-perl-class"></span>`fn parse_perl_class(&self) -> ast::ClassPerl` — [`ClassPerl`](../index.md)

#### Trait Implementations

##### `impl<'s, P: clone::Clone> Clone for ParserI<'s, P>`

- <span id="parseri-clone"></span>`fn clone(&self) -> ParserI<'s, P>` — [`ParserI`](#parseri)

##### `impl<'s, P: fmt::Debug> Debug for ParserI<'s, P>`

- <span id="parseri-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NestLimiter<'p, 's, P>`

```rust
struct NestLimiter<'p, 's, P> {
    p: &'p ParserI<'s, P>,
    depth: u32,
}
```

A type that traverses a fully parsed Ast and checks whether its depth
exceeds the specified nesting limit. If it does, then an error is returned.

#### Fields

- **`p`**: `&'p ParserI<'s, P>`

  The parser that is checking the nest limit.

- **`depth`**: `u32`

  The current depth while walking an Ast.

#### Implementations

- <span id="nestlimiter-new"></span>`fn new(p: &'p ParserI<'s, P>) -> NestLimiter<'p, 's, P>` — [`ParserI`](#parseri), [`NestLimiter`](#nestlimiter)

- <span id="nestlimiter-check"></span>`fn check(self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-increment-depth"></span>`fn increment_depth(&mut self, span: &Span) -> core::result::Result<(), ast::Error>` — [`Span`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-decrement-depth"></span>`fn decrement_depth(&mut self)`

#### Trait Implementations

##### `impl<'p, 's, P: fmt::Debug> Debug for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<'p, 's, P: Borrow<Parser>> Visitor for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-output"></span>`type Output = ()`

- <span id="nestlimiter-err"></span>`type Err = Error`

- <span id="nestlimiter-finish"></span>`fn finish(self) -> core::result::Result<(), ast::Error>` — [`Error`](../index.md)

- <span id="nestlimiter-visit-pre"></span>`fn visit_pre(&mut self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-visit-post"></span>`fn visit_post(&mut self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-visit-class-set-item-pre"></span>`fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), ast::Error>` — [`ClassSetItem`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-visit-class-set-item-post"></span>`fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), ast::Error>` — [`ClassSetItem`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-visit-class-set-binary-op-pre"></span>`fn visit_class_set_binary_op_pre(&mut self, ast: &ast::ClassSetBinaryOp) -> core::result::Result<(), ast::Error>` — [`ClassSetBinaryOp`](../index.md), [`Error`](../index.md)

- <span id="nestlimiter-visit-class-set-binary-op-post"></span>`fn visit_class_set_binary_op_post(&mut self, _ast: &ast::ClassSetBinaryOp) -> core::result::Result<(), ast::Error>` — [`ClassSetBinaryOp`](../index.md), [`Error`](../index.md)

## Enums

### `Primitive`

```rust
enum Primitive {
    Literal(ast::Literal),
    Assertion(ast::Assertion),
    Dot(crate::ast::Span),
    Perl(ast::ClassPerl),
    Unicode(ast::ClassUnicode),
}
```

A primitive is an expression with no sub-expressions. This includes
literals, assertions and non-set character classes. This representation
is used as intermediate state in the parser.

This does not include ASCII character classes, since they can only appear
within a set character class.

#### Implementations

- <span id="primitive-span"></span>`fn span(&self) -> &Span` — [`Span`](../index.md)

- <span id="primitive-into-ast"></span>`fn into_ast(self) -> Ast` — [`Ast`](../index.md)

- <span id="primitive-into-class-set-item"></span>`fn into_class_set_item<P: Borrow<Parser>>(self, p: &ParserI<'_, P>) -> core::result::Result<ast::ClassSetItem, ast::Error>` — [`ParserI`](#parseri), [`ClassSetItem`](../index.md), [`Error`](../index.md)

- <span id="primitive-into-class-literal"></span>`fn into_class_literal<P: Borrow<Parser>>(self, p: &ParserI<'_, P>) -> core::result::Result<ast::Literal, ast::Error>` — [`ParserI`](#parseri), [`Literal`](../index.md), [`Error`](../index.md)

#### Trait Implementations

##### `impl Clone for Primitive`

- <span id="primitive-clone"></span>`fn clone(&self) -> Primitive` — [`Primitive`](#primitive)

##### `impl Debug for Primitive`

- <span id="primitive-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Primitive`

##### `impl PartialEq for Primitive`

- <span id="primitive-eq"></span>`fn eq(&self, other: &Primitive) -> bool` — [`Primitive`](#primitive)

##### `impl StructuralPartialEq for Primitive`

### `GroupState`

```rust
enum GroupState {
    Group {
        concat: ast::Concat,
        group: ast::Group,
        ignore_whitespace: bool,
    },
    Alternation(ast::Alternation),
}
```

GroupState represents a single stack frame while parsing nested groups
and alternations. Each frame records the state up to an opening parenthesis
or a alternating bracket `|`.

#### Variants

- **`Group`**

  This state is pushed whenever an opening group is found.

- **`Alternation`**

  This state is pushed whenever a new alternation branch is found. If
  an alternation branch is found and this state is at the top of the
  stack, then this state should be modified to include the new
  alternation.

#### Trait Implementations

##### `impl Clone for GroupState`

- <span id="groupstate-clone"></span>`fn clone(&self) -> GroupState` — [`GroupState`](#groupstate)

##### `impl Debug for GroupState`

- <span id="groupstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ClassState`

```rust
enum ClassState {
    Open {
        union: ast::ClassSetUnion,
        set: ast::ClassBracketed,
    },
    Op {
        kind: ast::ClassSetBinaryOpKind,
        lhs: ast::ClassSet,
    },
}
```

ClassState represents a single stack frame while parsing character classes.
Each frame records the state up to an intersection, difference, symmetric
difference or nested class.

Note that a parser's character class stack is only non-empty when parsing
a character class. In all other cases, it is empty.

#### Variants

- **`Open`**

  This state is pushed whenever an opening bracket is found.

- **`Op`**

  This state is pushed when a operator is seen. When popped, the stored
  set becomes the left hand side of the operator.

#### Trait Implementations

##### `impl Clone for ClassState`

- <span id="classstate-clone"></span>`fn clone(&self) -> ClassState` — [`ClassState`](#classstate)

##### `impl Debug for ClassState`

- <span id="classstate-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `is_hex`

```rust
fn is_hex(c: char) -> bool
```

Returns true if the given character is a hexadecimal digit.

### `is_capture_char`

```rust
fn is_capture_char(c: char, first: bool) -> bool
```

Returns true if the given character is a valid in a capture group name.

If `first` is true, then `c` is treated as the first character in the
group name (which must be alphabetic or underscore).

### `specialize_err`

```rust
fn specialize_err<T>(result: core::result::Result<T, ast::Error>, from: ast::ErrorKind, to: ast::ErrorKind) -> core::result::Result<T, ast::Error>
```

When the result is an error, transforms the ast::ErrorKind from the source
Result into another one. This function is used to return clearer error
messages when possible.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, ast::Error>;
```

