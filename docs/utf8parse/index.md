# Crate `utf8parse`

A table-driven UTF-8 Parser

This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only branch is
on the `Action` returned from unpacking a transition.

## Structs

### `Parser`

```rust
struct Parser {
    point: u32,
    state: types::State,
}
```

A parser for Utf8 Characters

Repeatedly call `advance` with bytes to emit Utf8 characters

#### Implementations

- `fn new() -> Parser` — [`Parser`](../index.md)

- `fn advance<R>(self: &mut Self, receiver: &mut R, byte: u8)`

- `fn perform_action<R>(self: &mut Self, receiver: &mut R, byte: u8, action: Action)` — [`Action`](../types/index.md)

#### Trait Implementations

##### `impl Clone`

- `fn clone(self: &Self) -> Parser` — [`Parser`](../index.md)

##### `impl Debug`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default`

- `fn default() -> Parser` — [`Parser`](../index.md)

##### `impl Eq`

##### `impl PartialEq`

- `fn eq(self: &Self, other: &Parser) -> bool` — [`Parser`](../index.md)

##### `impl StructuralPartialEq`

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

