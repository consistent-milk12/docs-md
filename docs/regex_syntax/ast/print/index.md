*[regex_syntax](../../index.md) / [ast](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Ast`.

## Structs

### `Printer`

```rust
struct Printer {
}
```

A printer for a regular expression abstract syntax tree.

A printer converts an abstract syntax tree (AST) to a regular expression
pattern string. This particular printer uses constant stack space and heap
space proportional to the size of the AST.

This printer will not necessarily preserve the original formatting of the
regular expression pattern string. For example, all whitespace and comments
are ignored.

#### Implementations

- `fn new() -> Printer`
  Create a new printer.

- `fn print<W: fmt::Write>(self: &mut Self, ast: &Ast, wtr: W) -> fmt::Result`
  Print the given `Ast` to the given writer. The writer must implement

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

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

