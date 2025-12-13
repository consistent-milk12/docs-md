# Crate `utf8parse`

A table-driven UTF-8 Parser

This module implements a table-driven UTF-8 parser which should
theoretically contain the minimal number of branches (1). The only branch is
on the `Action` returned from unpacking a transition.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`types`](#types) | mod | Types supporting the UTF-8 parser |
| [`Parser`](#parser) | struct | A parser for Utf8 Characters |
| [`Receiver`](#receiver) | trait | Handles codepoint and invalid sequence events from the parser. |
| [`CONTINUATION_MASK`](#continuation-mask) | const | Continuation bytes are masked with this value. |

## Modules

- [`types`](types/index.md) — Types supporting the UTF-8 parser

## Structs

### `Parser`

```rust
struct Parser {
    point: u32,
    state: types::State,
}
```

*Defined in [`utf8parse-0.2.2/src/lib.rs:29-32`](../../.source_1765633015/utf8parse-0.2.2/src/lib.rs#L29-L32)*

A parser for Utf8 Characters

Repeatedly call `advance` with bytes to emit Utf8 characters

#### Implementations

- <span id="parser-new"></span>`fn new() -> Parser` — [`Parser`](#parser)

  Create a new Parser

- <span id="parser-advance"></span>`fn advance<R>(&mut self, receiver: &mut R, byte: u8)`

  Advance the parser

  

  The provider receiver will be called whenever a codepoint is completed or an invalid

  sequence is detected.

- <span id="parser-perform-action"></span>`fn perform_action<R>(&mut self, receiver: &mut R, byte: u8, action: Action)` — [`Action`](types/index.md#action)

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

##### `impl Default for Parser`

- <span id="parser-default"></span>`fn default() -> Parser` — [`Parser`](#parser)

##### `impl Eq for Parser`

##### `impl<T> From for Parser`

- <span id="parser-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for Parser`

- <span id="parser-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl PartialEq for Parser`

- <span id="parser-partialeq-eq"></span>`fn eq(&self, other: &Parser) -> bool` — [`Parser`](#parser)

##### `impl StructuralPartialEq for Parser`

##### `impl<U> TryFrom for Parser`

- <span id="parser-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parser-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for Parser`

- <span id="parser-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parser-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Traits

### `Receiver`

```rust
trait Receiver { ... }
```

*Defined in [`utf8parse-0.2.2/src/lib.rs:17-23`](../../.source_1765633015/utf8parse-0.2.2/src/lib.rs#L17-L23)*

Handles codepoint and invalid sequence events from the parser.

#### Required Methods

- `fn codepoint(&mut self, _: char)`

  Called whenever a codepoint is parsed successfully

- `fn invalid_sequence(&mut self)`

  Called when an invalid_sequence is detected

## Constants

### `CONTINUATION_MASK`
```rust
const CONTINUATION_MASK: u8 = 63u8;
```

*Defined in [`utf8parse-0.2.2/src/lib.rs:35`](../../.source_1765633015/utf8parse-0.2.2/src/lib.rs#L35)*

Continuation bytes are masked with this value.

