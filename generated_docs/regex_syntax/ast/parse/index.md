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
  - [`is_hex`](#is-hex)
  - [`is_capture_char`](#is-capture-char)
  - [`specialize_err`](#specialize-err)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParserBuilder`](#parserbuilder) | struct | A builder for a regular expression parser. |
| [`Parser`](#parser) | struct | A regular expression parser. |
| [`ParserI`](#parseri) | struct | ParserI is the internal parser implementation. |
| [`NestLimiter`](#nestlimiter) | struct | A type that traverses a fully parsed Ast and checks whether its depth exceeds the specified nesting limit. |
| [`Primitive`](#primitive) | enum | A primitive is an expression with no sub-expressions. |
| [`GroupState`](#groupstate) | enum | GroupState represents a single stack frame while parsing nested groups and alternations. |
| [`ClassState`](#classstate) | enum | ClassState represents a single stack frame while parsing character classes. |
| [`is_hex`](#is-hex) | fn | Returns true if the given character is a hexadecimal digit. |
| [`is_capture_char`](#is-capture-char) | fn | Returns true if the given character is a valid in a capture group name. |
| [`specialize_err`](#specialize-err) | fn | When the result is an error, transforms the ast::ErrorKind from the source Result into another one. |
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

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:123-128`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L123-L128)*

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

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

  

  This limit is not checked until the entire AST is parsed. Therefore,

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

- <span id="parserbuilder-ignore-whitespace"></span>`fn ignore_whitespace(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Enable verbose mode in the regular expression.

  

  When enabled, verbose mode permits insignificant whitespace in many

  places in the regular expression, as well as comments. Comments are

  started using `#` and continue until the end of the line.

  

  By default, this is disabled. It may be selectively enabled in the

  regular expression by using the `x` flag regardless of this setting.

- <span id="parserbuilder-empty-min-range"></span>`fn empty_min_range(&mut self, yes: bool) -> &mut ParserBuilder` — [`ParserBuilder`](#parserbuilder)

  Allow using `{,n}` as an equivalent to `{0,n}`.

  

  When enabled, the parser accepts `{,n}` as valid syntax for `{0,n}`.

  Most regular expression engines don't support the `{,n}` syntax, but

  some others do it, namely Python's `re` library.

  

  This is disabled by default.

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

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:249-283`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L249-L283)*

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

  Create a new parser with a default configuration.

  

  The parser can be run with either the `parse` or `parse_with_comments`

  methods. The parse methods return an abstract syntax tree.

  

  To set configuration options on the parser, use [`ParserBuilder`](#parserbuilder).

- <span id="parser-parse"></span>`fn parse(&mut self, pattern: &str) -> core::result::Result<Ast, ast::Error>` — [`Ast`](../index.md#ast), [`Error`](../index.md#error)

  Parse the regular expression into an abstract syntax tree.

- <span id="parser-parse-with-comments"></span>`fn parse_with_comments(&mut self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error>` — [`WithComments`](../index.md#withcomments), [`Error`](../index.md#error)

  Parse the regular expression and return an abstract syntax tree with

  all of the comments found in the pattern.

- <span id="parser-reset"></span>`fn reset(&self)`

  Reset the internal state of a parser.

  

  This is called at the beginning of every parse. This prevents the

  parser from running with inconsistent state (say, if a previous

  invocation returned an error and the parser is reused).

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

### `ParserI<'s, P>`

```rust
struct ParserI<'s, P> {
    parser: P,
    pattern: &'s str,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:295-300`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L295-L300)*

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

- <span id="parseri-new"></span>`fn new(parser: P, pattern: &'s str) -> ParserI<'s, P>` — [`ParserI`](#parseri)

  Build an internal parser from a parser configuration and a pattern.

- <span id="parseri-parser"></span>`fn parser(&self) -> &Parser` — [`Parser`](#parser)

  Return a reference to the parser state.

- <span id="parseri-pattern"></span>`fn pattern(&self) -> &str`

  Return a reference to the pattern being parsed.

- <span id="parseri-error"></span>`fn error(&self, span: Span, kind: ast::ErrorKind) -> ast::Error` — [`Span`](../index.md#span), [`ErrorKind`](../index.md#errorkind), [`Error`](../index.md#error)

  Create a new error with the given span and error type.

- <span id="parseri-offset"></span>`fn offset(&self) -> usize`

  Return the current offset of the parser.

  

  The offset starts at `0` from the beginning of the regular expression

  pattern string.

- <span id="parseri-line"></span>`fn line(&self) -> usize`

  Return the current line number of the parser.

  

  The line number starts at `1`.

- <span id="parseri-column"></span>`fn column(&self) -> usize`

  Return the current column of the parser.

  

  The column number starts at `1` and is reset whenever a `\n` is seen.

- <span id="parseri-next-capture-index"></span>`fn next_capture_index(&self, span: Span) -> core::result::Result<u32, ast::Error>` — [`Span`](../index.md#span), [`Error`](../index.md#error)

  Return the next capturing index. Each subsequent call increments the

  internal index.

  

  The span given should correspond to the location of the opening

  parenthesis.

  

  If the capture limit is exceeded, then an error is returned.

- <span id="parseri-add-capture-name"></span>`fn add_capture_name(&self, cap: &ast::CaptureName) -> core::result::Result<(), ast::Error>` — [`CaptureName`](../index.md#capturename), [`Error`](../index.md#error)

  Adds the given capture name to this parser. If this capture name has

  already been used, then an error is returned.

- <span id="parseri-ignore-whitespace"></span>`fn ignore_whitespace(&self) -> bool`

  Return whether the parser should ignore whitespace or not.

- <span id="parseri-char"></span>`fn char(&self) -> char`

  Return the character at the current position of the parser.

  

  This panics if the current position does not point to a valid char.

- <span id="parseri-char-at"></span>`fn char_at(&self, i: usize) -> char`

  Return the character at the given position.

  

  This panics if the given position does not point to a valid char.

- <span id="parseri-bump"></span>`fn bump(&self) -> bool`

  Bump the parser to the next Unicode scalar value.

  

  If the end of the input has been reached, then `false` is returned.

- <span id="parseri-bump-if"></span>`fn bump_if(&self, prefix: &str) -> bool`

  If the substring starting at the current position of the parser has

  the given prefix, then bump the parser to the character immediately

  following the prefix and return true. Otherwise, don't bump the parser

  and return false.

- <span id="parseri-is-lookaround-prefix"></span>`fn is_lookaround_prefix(&self) -> bool`

  Returns true if and only if the parser is positioned at a look-around

  prefix. The conditions under which this returns true must always

  correspond to a regular expression that would otherwise be consider

  invalid.

  

  This should only be called immediately after parsing the opening of

  a group or a set of flags.

- <span id="parseri-bump-and-bump-space"></span>`fn bump_and_bump_space(&self) -> bool`

  Bump the parser, and if the `x` flag is enabled, bump through any

  subsequent spaces. Return true if and only if the parser is not at

  EOF.

- <span id="parseri-bump-space"></span>`fn bump_space(&self)`

  If the `x` flag is enabled (i.e., whitespace insensitivity with

  comments), then this will advance the parser through all whitespace

  and comments to the next non-whitespace non-comment byte.

  

  If the `x` flag is disabled, then this is a no-op.

  

  This should be used selectively throughout the parser where

  arbitrary whitespace is permitted when the `x` flag is enabled. For

  example, `{   5  , 6}` is equivalent to `{5,6}`.

- <span id="parseri-peek"></span>`fn peek(&self) -> Option<char>`

  Peek at the next character in the input without advancing the parser.

  

  If the input has been exhausted, then this returns `None`.

- <span id="parseri-peek-space"></span>`fn peek_space(&self) -> Option<char>`

  Like peek, but will ignore spaces when the parser is in whitespace

  insensitive mode.

- <span id="parseri-is-eof"></span>`fn is_eof(&self) -> bool`

  Returns true if the next call to `bump` would return false.

- <span id="parseri-pos"></span>`fn pos(&self) -> Position` — [`Position`](../index.md#position)

  Return the current position of the parser, which includes the offset,

  line and column.

- <span id="parseri-span"></span>`fn span(&self) -> Span` — [`Span`](../index.md#span)

  Create a span at the current position of the parser. Both the start

  and end of the span are set.

- <span id="parseri-span-char"></span>`fn span_char(&self) -> Span` — [`Span`](../index.md#span)

  Create a span that covers the current character.

- <span id="parseri-push-alternate"></span>`fn push_alternate(&self, concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md#concat), [`Error`](../index.md#error)

  Parse and push a single alternation on to the parser's internal stack.

  If the top of the stack already has an alternation, then add to that

  instead of pushing a new one.

  

  The concatenation given corresponds to a single alternation branch.

  The concatenation returned starts the next branch and is empty.

  

  This assumes the parser is currently positioned at `|` and will advance

  the parser to the character following `|`.

- <span id="parseri-push-or-add-alternation"></span>`fn push_or_add_alternation(&self, concat: ast::Concat)` — [`Concat`](../index.md#concat)

  Pushes or adds the given branch of an alternation to the parser's

  internal stack of state.

- <span id="parseri-push-group"></span>`fn push_group(&self, concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md#concat), [`Error`](../index.md#error)

  Parse and push a group AST (and its parent concatenation) on to the

  parser's internal stack. Return a fresh concatenation corresponding

  to the group's sub-AST.

  

  If a set of flags was found (with no group), then the concatenation

  is returned with that set of flags added.

  

  This assumes that the parser is currently positioned on the opening

  parenthesis. It advances the parser to the character at the start

  of the sub-expression (or adjoining expression).

  

  If there was a problem parsing the start of the group, then an error

  is returned.

- <span id="parseri-pop-group"></span>`fn pop_group(&self, group_concat: ast::Concat) -> core::result::Result<ast::Concat, ast::Error>` — [`Concat`](../index.md#concat), [`Error`](../index.md#error)

  Pop a group AST from the parser's internal stack and set the group's

  AST to the given concatenation. Return the concatenation containing

  the group.

  

  This assumes that the parser is currently positioned on the closing

  parenthesis and advances the parser to the character following the `)`.

  

  If no such group could be popped, then an unopened group error is

  returned.

- <span id="parseri-pop-group-end"></span>`fn pop_group_end(&self, concat: ast::Concat) -> core::result::Result<Ast, ast::Error>` — [`Concat`](../index.md#concat), [`Ast`](../index.md#ast), [`Error`](../index.md#error)

  Pop the last state from the parser's internal stack, if it exists, and

  add the given concatenation to it. There either must be no state or a

  single alternation item on the stack. Any other scenario produces an

  error.

  

  This assumes that the parser has advanced to the end.

- <span id="parseri-push-class-open"></span>`fn push_class_open(&self, parent_union: ast::ClassSetUnion) -> core::result::Result<ast::ClassSetUnion, ast::Error>` — [`ClassSetUnion`](../index.md#classsetunion), [`Error`](../index.md#error)

  Parse the opening of a character class and push the current class

  parsing context onto the parser's stack. This assumes that the parser

  is positioned at an opening `[`. The given union should correspond to

  the union of set items built up before seeing the ``.

  

  If there was a problem parsing the opening of the class, then an error

  is returned. Otherwise, a new union of set items for the class is

  returned (which may be populated with either a `` or a `-`).

- <span id="parseri-pop-class"></span>`fn pop_class(&self, nested_union: ast::ClassSetUnion) -> core::result::Result<Either<ast::ClassSetUnion, ast::ClassBracketed>, ast::Error>` — [`ClassSetUnion`](../index.md#classsetunion), [`Either`](../../either/index.md#either), [`ClassBracketed`](../index.md#classbracketed), [`Error`](../index.md#error)

  Parse the end of a character class set and pop the character class

  parser stack. The union given corresponds to the last union built

  before seeing the closing `]`. The union returned corresponds to the

  parent character class set with the nested class added to it.

  

  This assumes that the parser is positioned at a `]` and will advance

  the parser to the byte immediately following the `]`.

  

  If the stack is empty after popping, then this returns the final

  "top-level" character class AST (where a "top-level" character class

  is one that is not nested inside any other character class).

  

  If there is no corresponding opening bracket on the parser's stack,

  then an error is returned.

- <span id="parseri-unclosed-class-error"></span>`fn unclosed_class_error(&self) -> ast::Error` — [`Error`](../index.md#error)

  Return an "unclosed class" error whose span points to the most

  recently opened class.

  

  This should only be called while parsing a character class.

- <span id="parseri-push-class-op"></span>`fn push_class_op(&self, next_kind: ast::ClassSetBinaryOpKind, next_union: ast::ClassSetUnion) -> ast::ClassSetUnion` — [`ClassSetBinaryOpKind`](../index.md#classsetbinaryopkind), [`ClassSetUnion`](../index.md#classsetunion)

  Push the current set of class items on to the class parser's stack as

  the left hand side of the given operator.

  

  A fresh set union is returned, which should be used to build the right

  hand side of this operator.

- <span id="parseri-pop-class-op"></span>`fn pop_class_op(&self, rhs: ast::ClassSet) -> ast::ClassSet` — [`ClassSet`](../index.md#classset)

  Pop a character class set from the character class parser stack. If the

  top of the stack is just an item (not an operation), then return the

  given set unchanged. If the top of the stack is an operation, then the

  given set will be used as the rhs of the operation on the top of the

  stack. In that case, the binary operation is returned as a set.

#### Trait Implementations

##### `impl Any for ParserI<'s, P>`

- <span id="parseri-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParserI<'s, P>`

- <span id="parseri-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParserI<'s, P>`

- <span id="parseri-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: clone::Clone> Clone for ParserI<'s, P>`

- <span id="parseri-clone"></span>`fn clone(&self) -> ParserI<'s, P>` — [`ParserI`](#parseri)

##### `impl CloneToUninit for ParserI<'s, P>`

- <span id="parseri-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl<P: fmt::Debug> Debug for ParserI<'s, P>`

- <span id="parseri-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParserI<'s, P>`

- <span id="parseri-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParserI<'s, P>`

- <span id="parseri-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ParserI<'s, P>`

- <span id="parseri-toowned-type-owned"></span>`type Owned = T`

- <span id="parseri-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="parseri-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ParserI<'s, P>`

- <span id="parseri-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseri-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParserI<'s, P>`

- <span id="parseri-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseri-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

### `NestLimiter<'p, 's, P>`

```rust
struct NestLimiter<'p, 's, P> {
    p: &'p ParserI<'s, P>,
    depth: u32,
}
```

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:2266-2271`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L2266-L2271)*

A type that traverses a fully parsed Ast and checks whether its depth
exceeds the specified nesting limit. If it does, then an error is returned.

#### Fields

- **`p`**: `&'p ParserI<'s, P>`

  The parser that is checking the nest limit.

- **`depth`**: `u32`

  The current depth while walking an Ast.

#### Implementations

- <span id="nestlimiter-new"></span>`fn new(p: &'p ParserI<'s, P>) -> NestLimiter<'p, 's, P>` — [`ParserI`](#parseri), [`NestLimiter`](#nestlimiter)

- <span id="nestlimiter-check"></span>`fn check(self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md#ast), [`Error`](../index.md#error)

- <span id="nestlimiter-increment-depth"></span>`fn increment_depth(&mut self, span: &Span) -> core::result::Result<(), ast::Error>` — [`Span`](../index.md#span), [`Error`](../index.md#error)

- <span id="nestlimiter-decrement-depth"></span>`fn decrement_depth(&mut self)`

#### Trait Implementations

##### `impl Any for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl<P: fmt::Debug> Debug for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl<U> TryFrom for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="nestlimiter-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="nestlimiter-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

##### `impl<P: Borrow<Parser>> Visitor for NestLimiter<'p, 's, P>`

- <span id="nestlimiter-visitor-type-output"></span>`type Output = ()`

- <span id="nestlimiter-visitor-type-err"></span>`type Err = Error`

- <span id="nestlimiter-visitor-finish"></span>`fn finish(self) -> core::result::Result<(), ast::Error>` — [`Error`](../index.md#error)

- <span id="nestlimiter-visitor-visit-pre"></span>`fn visit_pre(&mut self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md#ast), [`Error`](../index.md#error)

- <span id="nestlimiter-visitor-visit-post"></span>`fn visit_post(&mut self, ast: &Ast) -> core::result::Result<(), ast::Error>` — [`Ast`](../index.md#ast), [`Error`](../index.md#error)

- <span id="nestlimiter-visitor-visit-class-set-item-pre"></span>`fn visit_class_set_item_pre(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), ast::Error>` — [`ClassSetItem`](../index.md#classsetitem), [`Error`](../index.md#error)

- <span id="nestlimiter-visitor-visit-class-set-item-post"></span>`fn visit_class_set_item_post(&mut self, ast: &ast::ClassSetItem) -> core::result::Result<(), ast::Error>` — [`ClassSetItem`](../index.md#classsetitem), [`Error`](../index.md#error)

- <span id="nestlimiter-visitor-visit-class-set-binary-op-pre"></span>`fn visit_class_set_binary_op_pre(&mut self, ast: &ast::ClassSetBinaryOp) -> core::result::Result<(), ast::Error>` — [`ClassSetBinaryOp`](../index.md#classsetbinaryop), [`Error`](../index.md#error)

- <span id="nestlimiter-visitor-visit-class-set-binary-op-post"></span>`fn visit_class_set_binary_op_post(&mut self, _ast: &ast::ClassSetBinaryOp) -> core::result::Result<(), ast::Error>` — [`ClassSetBinaryOp`](../index.md#classsetbinaryop), [`Error`](../index.md#error)

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

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:33-39`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L33-L39)*

A primitive is an expression with no sub-expressions. This includes
literals, assertions and non-set character classes. This representation
is used as intermediate state in the parser.

This does not include ASCII character classes, since they can only appear
within a set character class.

#### Implementations

- <span id="primitive-span"></span>`fn span(&self) -> &Span` — [`Span`](../index.md#span)

  Return the span of this primitive.

- <span id="primitive-into-ast"></span>`fn into_ast(self) -> Ast` — [`Ast`](../index.md#ast)

  Convert this primitive into a proper AST.

- <span id="primitive-into-class-set-item"></span>`fn into_class_set_item<P: Borrow<Parser>>(self, p: &ParserI<'_, P>) -> core::result::Result<ast::ClassSetItem, ast::Error>` — [`ParserI`](#parseri), [`ClassSetItem`](../index.md#classsetitem), [`Error`](../index.md#error)

  Convert this primitive into an item in a character class.

  

  If this primitive is not a legal item (i.e., an assertion or a dot),

  then return an error.

- <span id="primitive-into-class-literal"></span>`fn into_class_literal<P: Borrow<Parser>>(self, p: &ParserI<'_, P>) -> core::result::Result<ast::Literal, ast::Error>` — [`ParserI`](#parseri), [`Literal`](../index.md#literal), [`Error`](../index.md#error)

  Convert this primitive into a literal in a character class. In

  particular, literals are the only valid items that can appear in

  ranges.

  

  If this primitive is not a legal item (i.e., a class, assertion or a

  dot), then return an error.

#### Trait Implementations

##### `impl Any for Primitive`

- <span id="primitive-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for Primitive`

- <span id="primitive-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for Primitive`

- <span id="primitive-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for Primitive`

- <span id="primitive-clone"></span>`fn clone(&self) -> Primitive` — [`Primitive`](#primitive)

##### `impl CloneToUninit for Primitive`

- <span id="primitive-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for Primitive`

- <span id="primitive-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Primitive`

##### `impl<T> From for Primitive`

- <span id="primitive-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Primitive`

- <span id="primitive-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Primitive`

- <span id="primitive-partialeq-eq"></span>`fn eq(&self, other: &Primitive) -> bool` — [`Primitive`](#primitive)

##### `impl StructuralPartialEq for Primitive`

##### `impl ToOwned for Primitive`

- <span id="primitive-toowned-type-owned"></span>`type Owned = T`

- <span id="primitive-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="primitive-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for Primitive`

- <span id="primitive-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="primitive-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Primitive`

- <span id="primitive-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="primitive-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:306-321`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L306-L321)*

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

##### `impl Any for GroupState`

- <span id="groupstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for GroupState`

- <span id="groupstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for GroupState`

- <span id="groupstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for GroupState`

- <span id="groupstate-clone"></span>`fn clone(&self) -> GroupState` — [`GroupState`](#groupstate)

##### `impl CloneToUninit for GroupState`

- <span id="groupstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for GroupState`

- <span id="groupstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for GroupState`

- <span id="groupstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for GroupState`

- <span id="groupstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for GroupState`

- <span id="groupstate-toowned-type-owned"></span>`type Owned = T`

- <span id="groupstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="groupstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for GroupState`

- <span id="groupstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="groupstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for GroupState`

- <span id="groupstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="groupstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

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

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:330-348`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L330-L348)*

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

##### `impl Any for ClassState`

- <span id="classstate-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ClassState`

- <span id="classstate-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ClassState`

- <span id="classstate-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Clone for ClassState`

- <span id="classstate-clone"></span>`fn clone(&self) -> ClassState` — [`ClassState`](#classstate)

##### `impl CloneToUninit for ClassState`

- <span id="classstate-clonetouninit-clone-to-uninit"></span>`unsafe fn clone_to_uninit(&self, dest: *mut u8)`

##### `impl Debug for ClassState`

- <span id="classstate-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ClassState`

- <span id="classstate-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ClassState`

- <span id="classstate-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToOwned for ClassState`

- <span id="classstate-toowned-type-owned"></span>`type Owned = T`

- <span id="classstate-toowned-to-owned"></span>`fn to_owned(&self) -> T`

- <span id="classstate-toowned-clone-into"></span>`fn clone_into(&self, target: &mut T)`

##### `impl<U> TryFrom for ClassState`

- <span id="classstate-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="classstate-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ClassState`

- <span id="classstate-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="classstate-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Functions

### `is_hex`

```rust
fn is_hex(c: char) -> bool
```

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:103-105`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L103-L105)*

Returns true if the given character is a hexadecimal digit.

### `is_capture_char`

```rust
fn is_capture_char(c: char, first: bool) -> bool
```

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:111-117`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L111-L117)*

Returns true if the given character is a valid in a capture group name.

If `first` is true, then `c` is treated as the first character in the
group name (which must be alphabetic or underscore).

### `specialize_err`

```rust
fn specialize_err<T>(result: core::result::Result<T, ast::Error>, from: ast::ErrorKind, to: ast::ErrorKind) -> core::result::Result<T, ast::Error>
```

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:2420-2434`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L2420-L2434)*

When the result is an error, transforms the ast::ErrorKind from the source
Result into another one. This function is used to return clearer error
messages when possible.

## Type Aliases

### `Result<T>`

```rust
type Result<T> = core::result::Result<T, ast::Error>;
```

*Defined in [`regex-syntax-0.8.8/src/ast/parse.rs:24`](../../../../.source_1765633015/regex-syntax-0.8.8/src/ast/parse.rs#L24)*

