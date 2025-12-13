*[cargo_platform](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ParseError`](#parseerror) | struct |  |
| [`ParseErrorKind`](#parseerrorkind) | enum |  |

## Structs

### `ParseError`

```rust
struct ParseError {
    kind: ParseErrorKind,
    orig: String,
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:4-7`](../../../.source_1765633015/cargo-platform-0.3.1/src/error.rs#L4-L7)*

#### Implementations

- <span id="parseerror-new"></span>`fn new(orig: &str, kind: ParseErrorKind) -> ParseError` — [`ParseErrorKind`](#parseerrorkind), [`ParseError`](#parseerror)

#### Trait Implementations

##### `impl Any for ParseError`

- <span id="parseerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseError`

- <span id="parseerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseError`

- <span id="parseerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseError`

- <span id="parseerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseError`

- <span id="parseerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for ParseError`

##### `impl<T> From for ParseError`

- <span id="parseerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseError`

- <span id="parseerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ParseError`

- <span id="parseerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseError`

- <span id="parseerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseError`

- <span id="parseerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

## Enums

### `ParseErrorKind`

```rust
enum ParseErrorKind {
    UnterminatedString,
    UnexpectedChar(char),
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
    IncompleteExpr(&'static str),
    UnterminatedExpression(String),
    InvalidTarget(String),
}
```

*Defined in [`cargo-platform-0.3.1/src/error.rs:11-21`](../../../.source_1765633015/cargo-platform-0.3.1/src/error.rs#L11-L21)*

#### Trait Implementations

##### `impl Any for ParseErrorKind`

- <span id="parseerrorkind-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for ParseErrorKind`

- <span id="parseerrorkind-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for ParseErrorKind`

- <span id="parseerrorkind-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for ParseErrorKind`

- <span id="parseerrorkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseErrorKind`

- <span id="parseerrorkind-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> From for ParseErrorKind`

- <span id="parseerrorkind-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for ParseErrorKind`

- <span id="parseerrorkind-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl ToString for ParseErrorKind`

- <span id="parseerrorkind-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl<U> TryFrom for ParseErrorKind`

- <span id="parseerrorkind-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="parseerrorkind-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for ParseErrorKind`

- <span id="parseerrorkind-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="parseerrorkind-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

