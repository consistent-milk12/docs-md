*[regex_syntax](../../index.md) / [ast](../index.md) / [parse](index.md)*

---

# Module `parse`

This module provides a regular expression parser.

## Structs

### `ParserBuilder`

```rust
struct ParserBuilder {
    // [REDACTED: Private Fields]
}
```

A builder for a regular expression parser.

This builder permits modifying configuration options for the parser.

#### Implementations

- `fn new() -> ParserBuilder`
  Create a new parser builder with a default configuration.

- `fn build(self: &Self) -> Parser`
  Build a parser from this configuration with the given pattern.

- `fn nest_limit(self: &mut Self, limit: u32) -> &mut ParserBuilder`
  Set the nesting limit for this parser.

- `fn octal(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Whether to support octal syntax or not.

- `fn ignore_whitespace(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Enable verbose mode in the regular expression.

- `fn empty_min_range(self: &mut Self, yes: bool) -> &mut ParserBuilder`
  Allow using `{,n}` as an equivalent to `{0,n}`.

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> ParserBuilder`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> ParserBuilder`

### `Parser`

```rust
struct Parser {
    // [REDACTED: Private Fields]
}
```

A regular expression parser.

This parses a string representation of a regular expression into an
abstract syntax tree. The size of the tree is proportional to the length
of the regular expression pattern.

A `Parser` can be configured in more detail via a [`ParserBuilder`](#parserbuilder).

#### Implementations

- `fn new() -> Parser`
  Create a new parser with a default configuration.

- `fn parse(self: &mut Self, pattern: &str) -> core::result::Result<Ast, ast::Error>`
  Parse the regular expression into an abstract syntax tree.

- `fn parse_with_comments(self: &mut Self, pattern: &str) -> core::result::Result<ast::WithComments, ast::Error>`
  Parse the regular expression and return an abstract syntax tree with

#### Trait Implementations

##### `impl From<T>`

- `fn from(t: T) -> T`
  Returns the argument unchanged.

##### `impl Into<T, U>`

- `fn into(self: Self) -> U`
  Calls `U::from(self)`.

##### `impl Any<T>`

- `fn type_id(self: &Self) -> TypeId`

##### `impl Borrow<T>`

- `fn borrow(self: &Self) -> &T`

##### `impl BorrowMut<T>`

- `fn borrow_mut(self: &mut Self) -> &mut T`

##### `impl Clone`

- `fn clone(self: &Self) -> Parser`

##### `impl CloneToUninit<T>`

- `unsafe fn clone_to_uninit(self: &Self, dest: *mut u8)`

##### `impl ToOwned<T>`

- `type Owned = T`

- `fn to_owned(self: &Self) -> T`

- `fn clone_into(self: &Self, target: &mut T)`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

