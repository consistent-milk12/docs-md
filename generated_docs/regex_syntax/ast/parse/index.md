*[regex_syntax](../../index.md) / [ast](../index.md) / [parse](index.md)*

---

# Module `parse`

This module provides a regular expression parser.

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

- `fn new() -> ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn build(self: &Self) -> Parser` — [`Parser`](#parser)

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn octal(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

- `fn empty_min_range(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

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

- `fn new() -> Parser` — [`Parser`](#parser)

- `fn parse(self: &mut Self, pattern: &str) -> core::result::Result<Ast, ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- `fn parse_with_comments(self: &mut Self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error>` — [`WithComments`](../index.md), [`Error`](../index.md)

- `fn reset(self: &Self)`

#### Trait Implementations

##### `impl Clone for Parser`

- `fn clone(self: &Self) -> Parser` — [`Parser`](#parser)

##### `impl Debug for Parser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn new(parser: P, pattern: &'s str) -> ParserI<'s, P>` — [`ParserI`](#parseri)

- `fn parser(self: &Self) -> &Parser` — [`Parser`](#parser)

- `fn pattern(self: &Self) -> &str`

- `fn error(self: &Self, span: Span, kind: ast::ErrorKind) -> ast::Error` — [`Span`](../index.md), [`ErrorKind`](../index.md), [`Error`](../index.md)

- `fn offset(self: &Self) -> usize`

- `fn line(self: &Self) -> usize`

- `fn column(self: &Self) -> usize`

- `fn next_capture_index(self: &Self, span: Span) -> core::result::Result<u32, ast::Error>` — [`Span`](../index.md), [`Error`](../index.md)

- `fn add_capture_name(self: &Self, cap: &ast::CaptureName) -> core::result::Result<(), ast::Error>` — [`CaptureName`](../index.md), [`Error`](../index.md)

- `fn ignore_whitespace(self: &Self) -> bool`

- `fn char(self: &Self) -> char`

- `fn char_at(self: &Self, i: usize) -> char`

- `fn bump(self: &Self) -> bool`

- `fn bump_if(self: &Self, prefix: &str) -> bool`

- `fn is_lookaround_prefix(self: &Self) -> bool`

- `fn bump_and_bump_space(self: &Self) -> bool`

- `fn bump_space(self: &Self)`

- `fn peek(self: &Self) -> Option<char>`

- `fn peek_space(self: &Self) -> Option<char>`

- `fn is_eof(self: &Self) -> bool`

- `fn pos(self: &Self) -> Position` — [`Position`](../index.md)

- `fn span(self: &Self) -> Span` — [`Span`](../index.md)

- `fn span_char(self: &Self) -> Span` — [`Span`](../index.md)

- `fn push_alternate(self: &Self, concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md), [`Error`](../index.md)

- `fn push_or_add_alternation(self: &Self, concat: ast::Concat)` — [`Concat`](../index.md)

- `fn push_group(self: &Self, concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md), [`Error`](../index.md)

- `fn pop_group(self: &Self, group_concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md), [`Error`](../index.md)

- `fn pop_group_end(self: &Self, concat: ast::Concat) -> core::result::Result<Ast, ast::Error>` — [`Concat`](../index.md), [`Ast`](../index.md), [`Error`](../index.md)

- `fn push_class_open(self: &Self, parent_union: ast::ClassSetUnion) -> core::result::Result<ast::ClassSetUnion, ast::Error>` — [`ClassSetUnion`](../index.md), [`Error`](../index.md)

- `fn pop_class(self: &Self, nested_union: ast::ClassSetUnion) -> core::result::Result<Either<ast::ClassSetUnion, ast::ClassBracketed>, ast::Error>` — [`ClassSetUnion`](../index.md), [`Either`](../../either/index.md), [`ClassBracketed`](../index.md), [`Error`](../index.md)

- `fn unclosed_class_error(self: &Self) -> ast::Error` — [`Error`](../index.md)

- `fn push_class_op(self: &Self, next_kind: ast::ClassSetBinaryOpKind, next_union: ast::ClassSetUnion) -> ast::ClassSetUnion` — [`ClassSetBinaryOpKind`](../index.md), [`ClassSetUnion`](../index.md)

- `fn pop_class_op(self: &Self, rhs: ast::ClassSet) -> ast::ClassSet` — [`ClassSet`](../index.md)

#### Trait Implementations

##### `impl<'s, P: $crate::clone::Clone> Clone for ParserI<'s, P>`

- `fn clone(self: &Self) -> ParserI<'s, P>` — [`ParserI`](#parseri)

##### `impl<'s, P: $crate::fmt::Debug> Debug for ParserI<'s, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn new(p: &'p ParserI<'s, P>) -> NestLimiter<'p, 's, P>` — [`ParserI`](#parseri), [`NestLimiter`](#nestlimiter)

- `fn check(self: Self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- `fn increment_depth(self: &mut Self, span: &Span) -> core::result::Result<(), ast::Error>` — [`Span`](../index.md), [`Error`](../index.md)

- `fn decrement_depth(self: &mut Self)`

#### Trait Implementations

##### `impl<'p, 's, P: $crate::fmt::Debug> Debug for NestLimiter<'p, 's, P>`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl<'p, 's, P: Borrow<Parser>> Visitor for NestLimiter<'p, 's, P>`

- `type Output = ()`

- `type Err = Error`

- `fn finish(self: Self) -> core::result::Result<(), ast::Error>` — [`Error`](../index.md)

- `fn visit_pre(self: &mut Self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- `fn visit_post(self: &mut Self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md), [`Error`](../index.md)

- `fn visit_class_set_item_pre(self: &mut Self, ast: &ast::ClassSetItem) -> core::result::Result<(), ast::Error>` — [`ClassSetItem`](../index.md), [`Error`](../index.md)

- `fn visit_class_set_item_post(self: &mut Self, ast: &ast::ClassSetItem) -> core::result::Result<(), ast::Error>` — [`ClassSetItem`](../index.md), [`Error`](../index.md)

- `fn visit_class_set_binary_op_pre(self: &mut Self, ast: &ast::ClassSetBinaryOp) -> core::result::Result<(), ast::Error>` — [`ClassSetBinaryOp`](../index.md), [`Error`](../index.md)

- `fn visit_class_set_binary_op_post(self: &mut Self, _ast: &ast::ClassSetBinaryOp) -> core::result::Result<(), ast::Error>` — [`ClassSetBinaryOp`](../index.md), [`Error`](../index.md)

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

- `fn span(self: &Self) -> &Span` — [`Span`](../index.md)

- `fn into_ast(self: Self) -> Ast` — [`Ast`](../index.md)

- `fn into_class_set_item<P: Borrow<Parser>>(self: Self, p: &ParserI<'_, P>) -> core::result::Result<ast::ClassSetItem, ast::Error>` — [`ParserI`](#parseri), [`ClassSetItem`](../index.md), [`Error`](../index.md)

- `fn into_class_literal<P: Borrow<Parser>>(self: Self, p: &ParserI<'_, P>) -> core::result::Result<ast::Literal, ast::Error>` — [`ParserI`](#parseri), [`Literal`](../index.md), [`Error`](../index.md)

#### Trait Implementations

##### `impl Clone for Primitive`

- `fn clone(self: &Self) -> Primitive` — [`Primitive`](#primitive)

##### `impl Debug for Primitive`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Eq for Primitive`

##### `impl PartialEq for Primitive`

- `fn eq(self: &Self, other: &Primitive) -> bool` — [`Primitive`](#primitive)

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

- `fn clone(self: &Self) -> GroupState` — [`GroupState`](#groupstate)

##### `impl Debug for GroupState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

- `fn clone(self: &Self) -> ClassState` — [`ClassState`](#classstate)

##### `impl Debug for ClassState`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

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

