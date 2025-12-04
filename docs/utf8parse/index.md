# Crate `utf8parse`

A table-driven UTF-8 Parser

This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only branch is
on the `Action` returned from unpacking a transition.

## Structs

### `Parser`

```rust
struct Parser {
}
```

A parser for Utf8 Characters

Repeatedly call `advance` with bytes to emit Utf8 characters

#### Implementations

- `fn new() -> Parser`
  Create a new Parser

- `fn advance<R>(self: &mut Self, receiver: &mut R, byte: u8)`
  Advance the parser

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

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Parser) -> bool`

##### `impl StructuralPartialEq`

##### `impl TryFrom<T, U>`

- `type Error = Infallible`

- `fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl TryInto<T, U>`

- `type Error = <U as TryFrom>::Error`

- `fn try_into(self: Self) -> Result<U, <U as TryFrom>::Error>`

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Parser`

## Traits

### `Receiver`

```rust
trait Receiver { ... }
```

Handles codepoint and invalid sequence events from the parser.

#### Required Methods

- `fn codepoint(self: &mut Self, _: char)`

  Called whenever a codepoint is parsed successfully

- `fn invalid_sequence(self: &mut Self)`

  Called when an invalid_sequence is detected

