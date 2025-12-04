*[regex_syntax](../../index.md) / [hir](../index.md) / [print](index.md)*

---

# Module `print`

This module provides a regular expression printer for `Hir`.

## Structs

### `Printer`

```rust
struct Printer {
}
```

A printer for a regular expression's high-level intermediate
representation.

A printer converts a high-level intermediate representation (HIR) to a
regular expression pattern string. This particular printer uses constant
stack space and heap space proportional to the size of the HIR.

Since this printer is only using the HIR, the pattern it prints will likely
not resemble the original pattern at all. For example, a pattern like
`\pL` will have its entire class written out.

The purpose of this printer is to provide a means to mutate an HIR and then
build a regular expression from the result of that mutation. (A regex
library could provide a constructor from this HIR explicitly, but that
creates an unnecessary public coupling between the regex library and this
specific HIR representation.)

#### Implementations

- `fn new() -> Printer`
  Create a new printer.

- `fn print<W: fmt::Write>(self: &mut Self, hir: &Hir, wtr: W) -> fmt::Result`
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

