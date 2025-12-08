# Crate `utf8parse`

A table-driven UTF-8 Parser

 
This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only branch is
on the `Action` returned from unpacking a transition.

## Modules

- [`types`](types/index.md) - Types supporting the UTF-8 parser

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

- `fn new() -> Parser` — [`Parser`](#parser)

- `fn advance<R>(self: &mut Self, receiver: &mut R, byte: u8)`

- `fn perform_action<R>(self: &mut Self, receiver: &mut R, byte: u8, action: Action)` — [`Action`](types/index.md)

#### Trait Implementations

##### `impl Clone for Parser`

- `fn clone(self: &Self) -> Parser` — [`Parser`](#parser)

##### `impl Debug for Parser`

- `fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<'_>) -> $crate::fmt::Result`

##### `impl Default for Parser`

- `fn default() -> Parser` — [`Parser`](#parser)

##### `impl Eq for Parser`

##### `impl PartialEq for Parser`

- `fn eq(self: &Self, other: &Parser) -> bool` — [`Parser`](#parser)

##### `impl StructuralPartialEq for Parser`

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

## Constants

### `CONTINUATION_MASK`

```rust
const CONTINUATION_MASK: u8 = 63u8;
```

Continuation bytes are masked with this value.

