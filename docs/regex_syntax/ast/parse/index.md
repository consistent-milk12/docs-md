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

- `fn new() -> ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

- `fn build(self: &Self) -> Parser` — [`Parser`](../../../ast/parse/index.md)

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

- `fn octal(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

- `fn empty_min_range(self: &mut Self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ParserBuilder` — [`ParserBuilder`](../../../ast/parse/index.md)

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

- `fn new() -> Parser` — [`Parser`](../../../ast/parse/index.md)

- `fn parse(self: &mut Self, pattern: &str) -> core::result::Result<Ast, ast::Error>` — [`Ast`](../../../ast/index.md), [`Error`](../../../ast/index.md)

- `fn parse_with_comments(self: &mut Self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error>` — [`WithComments`](../../../ast/index.md), [`Error`](../../../ast/index.md)

- `fn reset(self: &Self)`

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Parser` — [`Parser`](../../../ast/parse/index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

