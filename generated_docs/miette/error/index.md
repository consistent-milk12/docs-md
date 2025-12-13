*[miette](../index.md) / [error](index.md)*

---

# Module `error`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MietteError`](#mietteerror) | enum | Error enum for miette. |

## Enums

### `MietteError`

```rust
enum MietteError {
    IoError(io::Error),
    OutOfBounds,
}
```

*Defined in [`miette-7.6.0/src/error.rs:13-21`](../../../.source_1765521767/miette-7.6.0/src/error.rs#L13-L21)*

Error enum for miette. Used by certain operations in the protocol.

#### Variants

- **`IoError`**

  Wrapper around [`std::io::Error`](../../addr2line/index.md). This is returned when something went
  wrong while reading a [`SourceCode`](crate::SourceCode).

- **`OutOfBounds`**

  Returned when a [`SourceSpan`](crate::SourceSpan) extends beyond the
  bounds of a given [`SourceCode`](crate::SourceCode).

#### Trait Implementations

##### `impl Any for MietteError`

- <span id="mietteerror-any-type-id"></span>`fn type_id(&self) -> TypeId`

##### `impl<T> Borrow for MietteError`

- <span id="mietteerror-borrow"></span>`fn borrow(&self) -> &T`

##### `impl<T> BorrowMut for MietteError`

- <span id="mietteerror-borrowmut-borrow-mut"></span>`fn borrow_mut(&mut self) -> &mut T`

##### `impl Debug for MietteError`

- <span id="mietteerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Diag for MietteError`

- <span id="mietteerror-diag-ext-report"></span>`fn ext_report<D>(self, msg: D) -> Report` — [`Report`](../index.md#report)

##### `impl Diagnostic for MietteError`

- <span id="mietteerror-diagnostic-code"></span>`fn code<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- <span id="mietteerror-diagnostic-help"></span>`fn help<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

- <span id="mietteerror-diagnostic-url"></span>`fn url<'a>(self: &'a Self) -> Option<Box<dyn fmt::Display>>`

##### `impl Display for MietteError`

- <span id="mietteerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Error for MietteError`

- <span id="mietteerror-error-source"></span>`fn source(&self) -> Option<&dyn Error>`

##### `impl<T> From for MietteError`

- <span id="mietteerror-from"></span>`fn from(t: T) -> T`

  Returns the argument unchanged.

##### `impl<U> Into for MietteError`

- <span id="mietteerror-into"></span>`fn into(self) -> U`

  Calls `U::from(self)`.

  

  That is, this conversion is whatever the implementation of

  <code>[From]&lt;T&gt; for U</code> chooses to do.

##### `impl OwoColorize for MietteError`

##### `impl ToString for MietteError`

- <span id="mietteerror-tostring-to-string"></span>`fn to_string(&self) -> String`

##### `impl TraitKind for MietteError`

##### `impl<U> TryFrom for MietteError`

- <span id="mietteerror-tryfrom-type-error"></span>`type Error = Infallible`

- <span id="mietteerror-tryfrom-try-from"></span>`fn try_from(value: U) -> Result<T, <T as TryFrom>::Error>`

##### `impl<U> TryInto for MietteError`

- <span id="mietteerror-tryinto-type-error"></span>`type Error = <U as TryFrom>::Error`

- <span id="mietteerror-tryinto-try-into"></span>`fn try_into(self) -> Result<U, <U as TryFrom>::Error>`

